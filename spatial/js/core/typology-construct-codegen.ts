/** @emoji 🏗️ Pure helpers to generate typology construct actions and interactions (no asset glob). */

/** @emoji 🏷️ PascalCase object name from a typology label (`External Wall` → `ExternalWall`). */
export function typologyObjectPascalFromLabel(label: string): string {
	return label
		.replace(/[^a-zA-Z0-9]+/g, " ")
		.trim()
		.split(/\s+/)
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join("");
}

/** @emoji 🧭 Stable ids for the three create actions plus one construct interaction/dispatch action. */
export function typologyConstructAssetIds(
	typology: string,
	label: string,
): {
	readonly typology: string;
	readonly createFrom2PointsAndHeight: string;
	readonly createFromCurveAndHeight: string;
	readonly createFromSurface: string;
	readonly construct: string;
} {
	const parts = typology.split(".");
	const prefix = parts.length > 1 ? `${parts.slice(0, -1).join(".")}.` : "";
	const pascal = typologyObjectPascalFromLabel(label);
	return {
		typology,
		createFrom2PointsAndHeight: `${prefix}create${pascal}From2PointsAndHeight`,
		createFromCurveAndHeight: `${prefix}create${pascal}FromCurveAndHeight`,
		createFromSurface: `${prefix}create${pascal}FromSurface`,
		construct: `${prefix}construct${pascal}`,
	};
}

/** @emoji 📄 Declarative capability action JSON for typology construction steps. */
export function capabilityActionSpecJson(id: string, label: string): Record<string, unknown> {
	return {
		schema: "spatial.action/v1",
		id,
		version: "1.0.0",
		label,
		steps: [
			{ op: "kernel.call", function: "spatial.action.capability", assignTo: "result" },
			{ op: "return", result: { kind: "var", name: "result" } },
		],
	};
}

export type TypologyConstructMode = "2PointsAndHeight" | "curveAndHeight" | "surface";

const ctxField = (name: string) => ({
	kind: "path" as const,
	root: "context" as const,
	segments: [{ kind: "field" as const, name }],
});

const promptLabel = (id: string, text: string, y = 0.25) => ({
	kind: "label" as const,
	id,
	role: "prompt" as const,
	text,
	position: { kind: "const" as const, value: [0, y, 1.55] as const },
});

const ctxPoint = (id: string, field: string, role: string) => ({
	kind: "point" as const,
	id,
	role,
	position: ctxField(field),
});

const rubberBoxPreview = (id: string, cornerAField: string, cornerBField: string) => ({
	kind: "box-preview" as const,
	id,
	role: "preview" as const,
	cornerA: ctxField(cornerAField),
	cornerB: ctxField(cornerBField),
	height: { kind: "const" as const, value: 0.01 },
});

const footprintBoxPreview = (id: string) => ({
	kind: "box-preview" as const,
	id,
	role: "preview" as const,
	cornerA: ctxField("pointA"),
	cornerB: ctxField("pointB"),
	height: ctxField("height"),
});

/** @emoji 🖼️ Display templates for typology construct workflow states. */
function buildTypologyConstructDisplay(label: string): { readonly states: readonly Record<string, unknown>[] } {
	return {
		states: [
			{
				state: "choose_mode",
				items: [
					promptLabel("choose-mode", `Construct ${label}: 1=2pts+height 2=curve+height 3=surface`),
				],
			},
			{
				state: "two_points_first",
				items: [
					promptLabel("hint-first", `Construct ${label}: click first footprint corner`),
					ctxPoint("cursor", "cursor", "cursor"),
				],
			},
			{
				state: "two_points_second",
				items: [
					promptLabel("hint-second", `Construct ${label}: click opposite footprint corner`),
					ctxPoint("anchor-a", "pointA", "anchor"),
					ctxPoint("cursor", "cursor", "cursor"),
					{
						kind: "segment",
						id: "footprint-rubber",
						role: "preview",
						from: ctxField("pointA"),
						to: ctxField("cursor"),
					},
					rubberBoxPreview("preview", "pointA", "cursor"),
				],
			},
			{
				state: "two_points_height",
				items: [
					promptLabel(
						"hint-height",
						`Construct ${label}: height (Z) — drag teal wall, Apply height, or Accept`,
					),
					footprintBoxPreview("preview"),
				],
			},
			{
				state: "curve_wire",
				items: [promptLabel("hint-wire", `Construct ${label}: pick path wire`)],
			},
			{
				state: "curve_height",
				items: [
					promptLabel("hint-curve-height", `Construct ${label}: set height (Apply height or Accept)`),
					{
						kind: "entity-highlight",
						id: "wire-highlight",
						role: "preview",
						geometryEntityKind: "wire",
						entityId: ctxField("wireId"),
					},
				],
			},
			{
				state: "committed",
				items: [
					promptLabel("hint-commit", `Construct ${label}: Accept to create`, 0.2),
					footprintBoxPreview("preview-final"),
				],
			},
		],
	};
}

/** @emoji 🎮 Builds the shared multi-mode construct interaction for a typology. */
export function buildTypologyConstructInteractionSpec(
	typology: string,
	label: string,
	constructActionId: string,
): Record<string, unknown> {
	const pascal = typologyObjectPascalFromLabel(label);
	const key = pascal.replace(/[^a-zA-Z]/g, "").charAt(0).toLowerCase() || "c";
	const pathMode = (name: string) => ({
		kind: "path" as const,
		root: "context" as const,
		segments: [{ kind: "field" as const, name }],
	});
	const assignMode = (mode: TypologyConstructMode) => ({
		op: "assign" as const,
		target: { root: "context" as const, segments: [{ kind: "field" as const, name: "constructMode" }] },
		value: { kind: "const" as const, value: mode },
	});
	const assignPoint = (field: string) => ({
		op: "assign" as const,
		target: { root: "context" as const, segments: [{ kind: "field" as const, name: field }] },
		value: { kind: "path" as const, root: "event" as const, segments: [{ kind: "field" as const, name: "point" }] },
	});
	const assignConstHeight = (value: number) => ({
		op: "assign" as const,
		target: { root: "context" as const, segments: [{ kind: "field" as const, name: "height" }] },
		value: { kind: "const" as const, value },
	});
	const assignHeightFromPointerZ = (baseField: string) => ({
		op: "assign" as const,
		target: { root: "context" as const, segments: [{ kind: "field" as const, name: "height" }] },
		value: {
			kind: "let" as const,
			bindings: [
				{
					name: "z0",
					value: {
						kind: "path" as const,
						root: "context" as const,
						segments: [{ kind: "field" as const, name: baseField }, { kind: "index" as const, index: 2 }],
					},
				},
				{
					name: "z1",
					value: {
						kind: "path" as const,
						root: "event" as const,
						segments: [{ kind: "field" as const, name: "point" }, { kind: "index" as const, index: 2 }],
					},
				},
			],
			in: {
				kind: "binop" as const,
				op: "+" as const,
				left: {
					kind: "abs" as const,
					arg: {
						kind: "binop" as const,
						op: "-" as const,
						left: { kind: "var" as const, name: "z1" },
						right: { kind: "var" as const, name: "z0" },
					},
				},
				right: { kind: "const" as const, value: 0.01 },
			},
		},
	});
	const pointerMoveCursor = {
		event: "pointer.move",
		transitions: [{ transient: true, effects: [assignPoint("cursor")] }],
	};
	const pointerMoveHeight = {
		event: "pointer.move",
		transitions: [{ transient: true, effects: [assignHeightFromPointerZ("pointA")] }],
	};
	return {
		schema: "spatial.interaction/v1",
		id: constructActionId,
		version: "1.0.0",
		label: `Construct ${label}`,
		key,
		interaction: {
			spatialGroundPick: true,
			pickDisabledStates: ["choose_mode", "committed"],
			groundPointerMoveStates: ["two_points_first", "two_points_second"],
			heightDragStates: ["two_points_height", "curve_height"],
			verticalRodStates: [],
			heightConfirmState: "two_points_height",
		},
		guards: [
			{
				name: "hasConstructMode",
				expr: { kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "constructMode" }] } },
			},
		],
		machine: {
			initial: "choose_mode",
			states: [
				{
					name: "choose_mode",
					on: [
						{
							event: "mode.2points",
							transitions: [
								{ target: "two_points_first", key: "1", label: "2 points + height", effects: [assignMode("2PointsAndHeight")] },
							],
						},
						{
							event: "mode.curve",
							transitions: [{ target: "curve_wire", key: "2", label: "Curve + height", effects: [assignMode("curveAndHeight")] }],
						},
						{
							event: "mode.surface",
							transitions: [
								{
									target: "committed",
									key: "3",
									label: "Surface",
									effects: [
										assignMode("surface"),
										{
											op: "interaction.call",
											interaction: "pick.face",
											outputs: { faceId: "faceId" },
										},
									],
								},
							],
						},
					],
				},
				{
					name: "two_points_first",
					on: [
						pointerMoveCursor,
						{ event: "pointer.down", transitions: [{ target: "two_points_second", effects: [assignPoint("pointA")] }] },
					],
				},
				{
					name: "two_points_second",
					on: [
						pointerMoveCursor,
						{
							event: "pointer.down",
							transitions: [
								{
									target: "two_points_height",
									effects: [assignPoint("pointB"), assignConstHeight(0.25)],
								},
							],
						},
					],
				},
				{
					name: "two_points_height",
					on: [
						pointerMoveHeight,
						{
							event: "set.height",
							transitions: [
								{
									target: "committed",
									key: "n",
									label: "Height",
									effects: [
										{
											op: "assign",
											target: { root: "context", segments: [{ kind: "field", name: "height" }] },
											value: { kind: "path", root: "event", segments: [{ kind: "field", name: "value" }] },
										},
									],
								},
							],
						},
					],
				},
				{
					name: "curve_wire",
					selection: { accept: ["wire"], multiple: false, prompt: "Pick path wire" },
					on: [
						{
							event: "selection.changed",
							transitions: [
								{
									target: "curve_height",
									effects: [
										{
											op: "assign",
											target: { root: "context", segments: [{ kind: "field", name: "wireId" }] },
											value: {
												kind: "path",
												root: "event",
												segments: [
													{ kind: "field", name: "targets" },
													{ kind: "index", index: 0 },
													{ kind: "field", name: "id" },
												],
											},
										},
									],
								},
							],
						},
					],
				},
				{
					name: "curve_height",
					on: [
						{
							event: "set.height",
							transitions: [
								{
									target: "committed",
									key: "n",
									label: "Height",
									effects: [
										{
											op: "assign",
											target: { root: "context", segments: [{ kind: "field", name: "height" }] },
											value: { kind: "path", root: "event", segments: [{ kind: "field", name: "value" }] },
										},
									],
								},
							],
						},
					],
				},
				{ name: "committed", final: true },
			],
		},
		display: buildTypologyConstructDisplay(label),
		commit: {
			when: "hasConstructMode",
			fromStates: ["committed"],
			operation: {
				kind: "action",
				action: constructActionId,
				params: {
					typology: { kind: "const", value: typology },
					constructMode: pathMode("constructMode"),
					pointA: pathMode("pointA"),
					pointB: pathMode("pointB"),
					height: pathMode("height"),
					wireId: pathMode("wireId"),
					faceId: pathMode("faceId"),
				},
			},
		},
		produces: { typology },
	};
}
