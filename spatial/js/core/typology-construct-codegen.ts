/** @emoji 🏗️ Pure helpers to generate typology construct actions and interactions (no asset glob). */

const SURFACE_CONSTRUCT_OUTPUTS = {
	faceId: "faceId",
	wireId: "wireId",
	wireIds: "wireIds",
	curves: "curves",
	points: "points",
	targets: "targets",
	surfaceConstructMode: "surfaceConstructMode",
} as const;

function nestedCall(interaction: string, outputs: Readonly<Record<string, string>>): Record<string, unknown> {
	return {
		op: "interaction.call",
		interaction,
		outputs: Object.entries(outputs).map(([target, source]) => ({
			target: { root: "context", segments: [{ kind: "field", name: target }] },
			value: { kind: "path", root: "context", segments: [{ kind: "field", name: source }] },
		})),
	};
}

/** @emoji 🏷️ True when typology construct should expose surface-only workflow (e.g. base plate). */
export function typologyConstructIsSurfacePrimary(typologyId: string): boolean {
	return typologyId.endsWith(".baseplate");
}

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

export type TypologyConstructMode = "2PointsAndHeight" | "curveAndHeight" | "surface";

/** @emoji 🧭 Per-typology construct kit: three mode actions + one interaction id. */
export type TypologyConstructKit = {
	readonly typology: string;
	readonly interaction: string;
	readonly constructFrom2PointsAndHeight: string;
	readonly constructFromCurveAndHeight: string;
	readonly constructFromSurface: string;
};

/** @emoji 🧭 Stable ids: three `construct*From*` actions and one `construct*` interaction (same interaction id string as before). */
export function typologyConstructAssetIds(typology: string, label: string): TypologyConstructKit & { readonly construct: string } {
	const parts = typology.split(".");
	const prefix = parts.length > 1 ? `${parts.slice(0, -1).join(".")}.` : "";
	const pascal = typologyObjectPascalFromLabel(label);
	const interaction = `${prefix}construct${pascal}`;
	return {
		typology,
		interaction,
		construct: interaction,
		constructFrom2PointsAndHeight: `${prefix}construct${pascal}From2PointsAndHeight`,
		constructFromCurveAndHeight: `${prefix}construct${pascal}FromCurveAndHeight`,
		constructFromSurface: `${prefix}construct${pascal}FromSurface`,
	};
}

/** @emoji 🧭 `construct*` action ids declared on a typology (`surface`-primary typologies ship surface only). */
export function typologyConstructModeActionIds(typologyId: string, label: string): readonly string[] {
	const ids = typologyConstructAssetIds(typologyId, label);
	if (typologyConstructIsSurfacePrimary(typologyId)) return [ids.constructFromSurface];
	return [ids.constructFrom2PointsAndHeight, ids.constructFromCurveAndHeight, ids.constructFromSurface];
}

/** @emoji 🎯 Resolves the single mode action an interaction commit must run for `constructMode`. */
export function typologyConstructCommitActionForMode(
	kit: TypologyConstructKit,
	mode: string,
): string {
	switch (mode as TypologyConstructMode) {
		case "2PointsAndHeight":
			return kit.constructFrom2PointsAndHeight;
		case "curveAndHeight":
			return kit.constructFromCurveAndHeight;
		case "surface":
			return kit.constructFromSurface;
		default:
			throw new Error(`Unknown constructMode ${mode} for ${kit.interaction}`);
	}
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

function typologyConstructCommitParams(typology: string): Record<string, unknown> {
	const pathMode = (name: string) => ({
		kind: "path" as const,
		root: "context" as const,
		segments: [{ kind: "field" as const, name }],
	});
	return {
		typology: { kind: "const", value: typology },
		constructMode: pathMode("constructMode"),
		pointA: pathMode("pointA"),
		pointB: pathMode("pointB"),
		height: pathMode("height"),
		wireId: pathMode("wireId"),
		wireIds: pathMode("wireIds"),
		curves: pathMode("curves"),
		points: pathMode("points"),
		targets: pathMode("targets"),
		faceId: pathMode("faceId"),
		surfaceConstructMode: pathMode("surfaceConstructMode"),
	};
}

/** @emoji 🖼️ Display templates for typology construct workflow states. */
function buildTypologyConstructDisplay(
	label: string,
	surfacePrimary: boolean,
): { readonly states: readonly Record<string, unknown>[] } {
	const chooseText = surfacePrimary
		? `Construct ${label}: 1=surface (rectangle, pick, line+extrude, loft, …)`
		: `Construct ${label}: 1=2pts+height 2=curve+height 3=surface`;
	return {
		states: [
			{
				state: "choose_mode",
				items: [promptLabel("choose-mode", chooseText)],
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
				items: [promptLabel("hint-curve-height", `Construct ${label}: set height (Apply height or Accept)`)],
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

/** @emoji 🎮 Builds the single construct interaction for a typology (commit resolves to one mode action). */
export function buildTypologyConstructInteractionSpec(
	typology: string,
	label: string,
	interactionId: string,
): Record<string, unknown> {
	const pascal = typologyObjectPascalFromLabel(label);
	const key = pascal.replace(/[^a-zA-Z]/g, "").charAt(0).toLowerCase() || "c";
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
	const callSurfaceConstruct = nestedCall("surface.construct", { ...SURFACE_CONSTRUCT_OUTPUTS });
	const surfacePrimary = typologyConstructIsSurfacePrimary(typology);
	const chooseModeOn = surfacePrimary
		? [
				{
					event: "mode.surface",
					transitions: [{ target: "committed", key: "1", label: "Surface", effects: [assignMode("surface"), callSurfaceConstruct] }],
				},
			]
		: [
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
					transitions: [{ target: "committed", key: "3", label: "Surface", effects: [assignMode("surface"), callSurfaceConstruct] }],
				},
			];
	return {
		schema: "spatial.interaction/v1",
		id: interactionId,
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
					on: chooseModeOn,
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
		display: buildTypologyConstructDisplay(label, surfacePrimary),
		commit: {
			when: "hasConstructMode",
			fromStates: ["committed"],
			operation: {
				kind: "action",
				action: typologyConstructModeActionIds(typology, label)[0],
				params: typologyConstructCommitParams(typology),
			},
		},
		produces: { typology },
	};
}

/** @emoji 🗑️ Legacy action asset basenames to remove when migrating typologies to the strict kit. */
export function legacyTypologyConstructActionBasenames(label: string, interactionId: string): readonly string[] {
	const pascal = typologyObjectPascalFromLabel(label);
	const interactionBase = interactionId.split(".").pop() ?? interactionId;
	return [
		`create${pascal}From2PointsAndHeight.json`,
		`create${pascal}FromCurveAndHeight.json`,
		`create${pascal}FromSurface.json`,
		`${interactionBase}.json`,
	];
}
