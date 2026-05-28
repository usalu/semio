/** @emoji 🧩 Generates nested-only `surface.construct` and `curve.construct` interaction specs for spatial.shape. */

const assignConst = (field: string, value: unknown) => ({
	op: "assign" as const,
	target: { root: "context" as const, segments: [{ kind: "field" as const, name: field }] },
	value: { kind: "const" as const, value },
});

const assignSurfaceMode = (mode: string) => assignConst("surfaceConstructMode", mode);

export const nestedCall = (interaction: string, outputs?: Record<string, string>, inputs?: Record<string, unknown>) => ({
	op: "interaction.call" as const,
	interaction,
	...(inputs ? { inputs } : {}),
	...(outputs
		? {
				outputs: Object.entries(outputs).map(([hostKey, childKey]) => ({
					target: { root: "context", segments: [{ kind: "field", name: hostKey }] },
					value: { kind: "path", root: "context", segments: [{ kind: "field", name: childKey }] },
				})),
			}
		: {}),
});

const promptLabel = (id: string, text: string) => ({
	kind: "label" as const,
	id,
	role: "prompt" as const,
	text,
	position: { kind: "const" as const, value: [0, 0.25, 1.55] as const },
});

const surfaceConstructOutputs = {
	faceId: "faceId",
	wireId: "wireId",
	wireIds: "wireIds",
	curves: "curves",
	points: "points",
	targets: "targets",
	surfaceConstructMode: "surfaceConstructMode",
};

const curveConstructOutputs = {
	wireId: "wireId",
	wireIds: "wireIds",
	curves: "curves",
	targets: "targets",
	points: "points",
	curveConstructMode: "curveConstructMode",
};

/** @emoji 🌐 Callable surface construction hub (rectangle plane, pick, line+extrude, loft, sweep, network). */
export function buildConstructSurfaceInteractionSpec(): Record<string, unknown> {
	const childMode = (key: string, label: string, mode: string, interaction: string, extraOutputs?: Record<string, string>) => ({
		event: `mode.${mode}`,
		transitions: [
			{
				target: "committed",
				key,
				label,
				effects: [assignSurfaceMode(mode), nestedCall(interaction, { ...surfaceConstructOutputs, ...extraOutputs })],
			},
		],
	});

	return {
		schema: "spatial.interaction/v1",
		id: "surface.construct",
		version: "1.0.0",
		label: "Construct Surface",
		key: "s",
		invocation: "callable",
		guards: [
			{
				name: "hasSurfaceResult",
				expr: {
					kind: "any",
					args: [
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "faceId" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "surfaceConstructMode" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "targets" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "points" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "wireIds" }] } },
					],
				},
			},
		],
		machine: {
			initial: "choose_mode",
			states: [
				{
					name: "choose_mode",
					on: [
						childMode("1", "Rectangle", "rectangle", "surface.plane"),
						{
							event: "mode.pick",
							transitions: [
								{
									target: "pick_face",
									key: "2",
									label: "Pick face",
									effects: [assignSurfaceMode("pick")],
								},
							],
						},
						{
							event: "mode.lineExtrude",
							transitions: [
								{
									target: "extrude_after_curve",
									key: "3",
									label: "Line + extrude",
									effects: [assignSurfaceMode("lineExtrude"), nestedCall("curve.line", curveConstructOutputs)],
								},
							],
						},
						{
							event: "mode.loft",
							transitions: [
								{
									target: "loft_run",
									key: "4",
									label: "Tween curves (loft)",
									effects: [assignSurfaceMode("loft"), nestedCall("curve.construct", curveConstructOutputs)],
								},
							],
						},
						childMode("5", "Sweep 1", "sweep1", "surface.sweep1"),
						childMode("6", "Sweep 2", "sweep2", "surface.sweep2"),
						childMode("7", "Network surface", "network", "surface.networkSrf"),
					],
				},
				{
					name: "pick_face",
					selection: { accept: ["face"], multiple: false, prompt: "Pick face" },
					on: [
						{
							event: "selection.changed",
							transitions: [
								{
									target: "committed",
									effects: [
										{
											op: "assign",
											target: { root: "context", segments: [{ kind: "field", name: "faceId" }] },
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
					name: "extrude_after_curve",
					on: [
						{
							event: "confirm",
							transitions: [
								{
									target: "committed",
									key: "e",
									label: "Extrude profile",
									effects: [nestedCall("surface.extrudeCrv", surfaceConstructOutputs)],
								},
							],
						},
					],
				},
				{
					name: "loft_run",
					on: [
						{
							event: "confirm",
							transitions: [
								{
									target: "committed",
									key: "l",
									label: "Loft curves",
									effects: [nestedCall("surface.loft", surfaceConstructOutputs)],
								},
							],
						},
					],
				},
				{ name: "committed", final: true },
			],
		},
		display: {
			states: [
				{
					state: "choose_mode",
					items: [
						promptLabel(
							"choose-surface",
							"Surface: 1=rectangle 2=pick 3=line+extrude 4=loft 5=sweep1 6=sweep2 7=network",
						),
					],
				},
				{ state: "pick_face", items: [promptLabel("pick-face-hint", "Pick a face")] },
				{ state: "extrude_after_curve", items: [promptLabel("extrude-hint", "Line drawn — confirm to extrude profile")] },
				{ state: "loft_run", items: [promptLabel("loft-hint", "Curves ready — confirm to loft")] },
			],
		},
		commit: {
			when: "hasSurfaceResult",
			fromStates: ["committed"],
			operation: { kind: "action", action: "command.finish", params: {} },
		},
	};
}

/** @emoji 📈 Callable curve construction hub (line, polyline, arc, circle, pick wires for loft). */
export function buildConstructCurveInteractionSpec(): Record<string, unknown> {
	const childMode = (key: string, label: string, mode: string, interaction: string) => ({
		event: `mode.${mode}`,
		transitions: [
			{
				target: "committed",
				key,
				label,
				effects: [assignConst("curveConstructMode", mode), nestedCall(interaction, curveConstructOutputs)],
			},
		],
	});

	return {
		schema: "spatial.interaction/v1",
		id: "curve.construct",
		version: "1.0.0",
		label: "Construct Curve",
		key: "c",
		invocation: "callable",
		guards: [
			{
				name: "hasCurveResult",
				expr: {
					kind: "any",
					args: [
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "wireId" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "wireIds" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "curves" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "targets" }] } },
						{ kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "points" }] } },
					],
				},
			},
		],
		machine: {
			initial: "choose_mode",
			states: [
				{
					name: "choose_mode",
					on: [
						childMode("1", "Line", "line", "curve.line"),
						childMode("2", "Polyline", "polyline", "curve.polyline"),
						childMode("3", "Arc", "arc", "curve.arc"),
						childMode("4", "Circle", "circle", "curve.circle"),
						childMode("5", "Interpolate", "interpolate", "curve.interpolateCurve"),
						{
							event: "mode.pickWires",
							transitions: [{ target: "pick_wires", key: "6", label: "Pick wires (loft)", effects: [assignConst("curveConstructMode", "pickWires")] }],
						},
					],
				},
				{
					name: "pick_wires",
					selection: { accept: ["wire", "edge"], multiple: true, prompt: "Pick curves for loft / tween" },
					on: [
						{
							event: "selection.changed",
							transitions: [
								{
									target: "pick_wires",
									key: "s",
									label: "Add",
									effects: [
										{
											op: "action",
											action: "command.addSelection",
											params: {
												field: { kind: "const", value: "curves" },
												targets: {
													kind: "path",
													root: "event",
													segments: [{ kind: "field", name: "targets" }],
												},
											},
										},
									],
								},
							],
						},
						{ event: "confirm", transitions: [{ target: "committed", key: "Enter", label: "Finish pick" }] },
					],
				},
				{ name: "committed", final: true },
			],
		},
		display: {
			states: [
				{
					state: "choose_mode",
					items: [
						promptLabel("choose-curve", "Curve: 1=line 2=polyline 3=arc 4=circle 5=interpolate 6=pick wires (loft)"),
					],
				},
				{ state: "pick_wires", items: [promptLabel("pick-wires-hint", "Select curves, then Finish pick")] },
			],
		},
		commit: {
			when: "hasCurveResult",
			fromStates: ["committed"],
			operation: { kind: "action", action: "command.finish", params: {} },
		},
	};
}
