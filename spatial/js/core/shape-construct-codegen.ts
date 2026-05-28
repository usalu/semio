/** @emoji 🧩 Generates nested-only `surface.construct` and `curve.construct` interaction specs for spatial.shape. */

const ctxField = (name: string) => ({
	kind: "path" as const,
	root: "context" as const,
	segments: [{ kind: "field" as const, name }],
});

const assignConst = (field: string, value: unknown) => ({
	op: "assign" as const,
	target: { root: "context" as const, segments: [{ kind: "field" as const, name: field }] },
	value: { kind: "const" as const, value },
});

const assignMode = (mode: string) => assignConst("surfaceConstructMode", mode);

const callInteraction = (
	interaction: string,
	resumeTarget: string,
	outputs?: Record<string, string>,
	inputs?: Record<string, unknown>,
) => ({
	op: "interaction.call" as const,
	interaction,
	...(inputs ? { inputs } : {}),
	...(outputs ? { outputs } : {}),
	_resumeTarget: resumeTarget,
});

const promptLabel = (id: string, text: string) => ({
	kind: "label" as const,
	id,
	role: "prompt" as const,
	text,
	position: { kind: "const" as const, value: [0, 0.25, 1.55] as const },
});

/** @emoji 📞 Builds `interaction.call` effect; `resumeTarget` is applied by the host transition target. */
function nestedCall(
	interaction: string,
	resumeTarget: string,
	outputs?: Record<string, string>,
	inputs?: Record<string, unknown>,
): Record<string, unknown> {
	const row = callInteraction(interaction, resumeTarget, outputs, inputs);
	const { _resumeTarget, ...effect } = row;
	void _resumeTarget;
	return effect;
}

const surfaceConstructOutputs = {
	faceId: "faceId",
	wireId: "wireId",
	wireIds: "wireIds",
	curves: "curves",
	points: "points",
	targets: "targets",
	surfaceConstructMode: "surfaceConstructMode",
};

/** @emoji 🌐 Callable surface construction hub (rectangle plane, pick, line+extrude, loft, sweep, network). */
export function buildConstructSurfaceInteractionSpec(): Record<string, unknown> {
	const modeTransition = (
		key: string,
		label: string,
		mode: string,
		interaction: string,
		outputs?: Record<string, string>,
		inputs?: Record<string, unknown>,
	) => ({
		event: `mode.${mode}`,
		transitions: [
			{
				target: "committed",
				key,
				label,
				effects: [
					assignMode(mode),
					nestedCall(interaction, "committed", { ...surfaceConstructOutputs, ...outputs }, inputs),
				],
			},
		],
	});

	return {
		schema: "spatial.interaction/v1",
		id: "surface.construct",
		version: "1.0.0",
		label: "Construct Surface",
		key: "s",
		interaction: { callableOnly: true },
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
						modeTransition("1", "Rectangle", "rectangle", "surface.plane"),
						modeTransition("2", "Pick face", "pick", "pick.face", { faceId: "faceId" }),
						{
							event: "mode.lineExtrude",
							transitions: [
								{
									target: "extrude_after_curve",
									key: "3",
									label: "Line + extrude",
									effects: [
										assignMode("lineExtrude"),
										nestedCall("curve.construct", "extrude_after_curve", {
											wireId: "wireId",
											targets: "targets",
											curves: "curves",
											curveConstructMode: "curveConstructMode",
										}),
									],
								},
							],
						},
						{
							event: "mode.loft",
							transitions: [
								{
									target: "committed",
									key: "4",
									label: "Tween curves (loft)",
									effects: [
										assignMode("loft"),
										nestedCall("curve.construct", "committed", {
											wireIds: "wireIds",
											curves: "curves",
											targets: "targets",
											curveConstructMode: "curveConstructMode",
										}),
										nestedCall("surface.loft", "committed", surfaceConstructOutputs),
									],
								},
							],
						},
						modeTransition("5", "Sweep 1", "sweep1", "surface.sweep1"),
						modeTransition("6", "Sweep 2", "sweep2", "surface.sweep2"),
						modeTransition("7", "Network surface", "network", "surface.networkSrf"),
					],
				},
				{
					name: "extrude_after_curve",
					on: [
						{
							event: "start",
							transitions: [
								{
									target: "committed",
									effects: [
										nestedCall("surface.extrudeCrv", "committed", surfaceConstructOutputs, {
											seedTargets: ctxField("targets"),
										}),
									],
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
				{
					state: "extrude_after_curve",
					items: [promptLabel("extrude-hint", "Extrude profile — set distance or Accept")],
				},
			],
		},
		commit: {
			when: "hasSurfaceResult",
			fromStates: ["committed"],
			operation: { kind: "action", action: "command.finish", params: {} },
		},
	};
}

const curveConstructOutputs = {
	wireId: "wireId",
	wireIds: "wireIds",
	curves: "curves",
	targets: "targets",
	points: "points",
	curveConstructMode: "curveConstructMode",
};

/** @emoji 📈 Callable curve construction hub (line, polyline, arc, circle, pick wires for loft). */
export function buildConstructCurveInteractionSpec(): Record<string, unknown> {
	const modeTransition = (key: string, label: string, mode: string, interaction: string) => ({
		event: `mode.${mode}`,
		transitions: [
			{
				target: "committed",
				key,
				label,
				effects: [assignConst("curveConstructMode", mode), nestedCall(interaction, "committed", curveConstructOutputs)],
			},
		],
	});

	return {
		schema: "spatial.interaction/v1",
		id: "curve.construct",
		version: "1.0.0",
		label: "Construct Curve",
		key: "c",
		interaction: { callableOnly: true },
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
						modeTransition("1", "Line", "line", "curve.line"),
						modeTransition("2", "Polyline", "polyline", "curve.polyline"),
						modeTransition("3", "Arc", "arc", "curve.arc"),
						modeTransition("4", "Circle", "circle", "curve.circle"),
						modeTransition("5", "Interpolate", "interpolate", "curve.interpolateCurve"),
						{
							event: "mode.pickWires",
							transitions: [
								{
									target: "committed",
									key: "6",
									label: "Pick wires (loft)",
									effects: [
										assignConst("curveConstructMode", "pickWires"),
										nestedCall("surface.loft", "committed", {
											...curveConstructOutputs,
											wireIds: "wireIds",
										}),
									],
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
							"choose-curve",
							"Curve: 1=line 2=polyline 3=arc 4=circle 5=interpolate 6=pick wires (loft)",
						),
					],
				},
			],
		},
		commit: {
			when: "hasCurveResult",
			fromStates: ["committed"],
			operation: { kind: "action", action: "command.finish", params: {} },
		},
	};
}

/** @emoji 🏷️ True when typology construct should expose surface-only workflow (e.g. base plate). */
export function typologyConstructIsSurfacePrimary(typologyId: string): boolean {
	return typologyId.endsWith(".baseplate");
}
