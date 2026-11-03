import sys
p = sys.argv[1]
raw = open(p, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
s = raw.replace("\r\n", "\n")


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


a = s.index("/** 🧮️ The evaluated frames of the command")
b = s.index("type Canvas = ")
s = s[:a] + '''/** 🗓️ The queries of the command: one per scheduled time (a pose is one frame at t = 0), with the schedule that times them. */
export function frameQueries(asset: Asset, options: CliOptions): { readonly schedule: FrameSchedule; readonly queries: readonly EvaluationQuery[] } {
  if (options.pose !== undefined) {
    const schedule: FrameSchedule = { fps: options.fps ?? 1, durationS: 0, loop: false, count: 1, times: [0] };
    return { schedule, queries: [{ kind: "pose", pose: options.pose }] };
  }
  const clip = asset.clips.find((candidate) => candidate.id === options.clip);
  if (!clip) throw new Error(`asset ${asset.id} has no clip ${options.clip}; clips: ${asset.clips.map((candidate) => candidate.id).join(", ")}`);
  const schedule = options.time === undefined ? frameSchedule(clip, options.fps ?? clip.fps) : ({ fps: 1, durationS: clip.durationS, loop: clip.loop, count: 1, times: [options.time] } satisfies FrameSchedule);
  return { schedule, queries: schedule.times.map((timeS) => ({ kind: "clip", clip: clip.id, timeS }) as const) };
}

/** 🎯️ What the command draws: one viewpoint, or the sheet of all three in a projection; exactly one is set by {@link parseArguments}. */
export function targetOf(options: CliOptions): { readonly kind: "view"; readonly viewpoint: Viewpoint } | { readonly kind: "sheet"; readonly projection: Projection } {
  if (options.projection !== undefined) return { kind: "sheet", projection: options.projection };
  if (options.viewpoint !== undefined) return { kind: "view", viewpoint: options.viewpoint };
  throw new Error("name --viewpoint or --projection");
}

''' + s[b:]

a = s.index("  const { schedule, states } = evaluateFrames(asset, options, control);")
b = s.index("  const written: string[] = [];\n  const emit")
s = s[:a] + '''  const target = targetOf(options);
  const { schedule, queries } = frameQueries(asset, options);
  const presets = new Map(((options.view === "thermal" ? await loadEnvironments(options.catalog, control.signal) : null)?.presets ?? []).map((preset) => [preset.id, preset]));
  const environment = options.view === "thermal" ? resolveThermalEnvironment(asset, presets, options.environment) : undefined;
  const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...common } = defaultDepictionOptions(options.view, options.language, options.scale, "elevation");
  const drawing = { ...common, ...(environment === undefined ? {} : { environment }) };
  const source = { kind: "scale", denominator: options.scale } as const;
  const selector = { kind: "scale", scale: { denominator: options.scale } } as const;
  const transparentRaster = options.format === "png-sequence" || options.format === "webm";
  const frames = queries.map((query, index) => {
    control.signal?.throwIfAborted();
    control.progress?.(index + 1, queries.length, { en: "Evaluating frames", de: "Bilder werden ausgewertet" });
    if (target.kind === "sheet") {
      const resolved = resolveSheet(asset, query, source, options.view, drawing, target.projection, options.language);
      return { state: resolved.state, analysis: resolved.analysis, scene: resolved.sheet as DrawingScene };
    }
    const state = evaluate(asset, query, selector);
    const analysis: Analysis = options.view === "thermal" ? { thermal: solveThermalAt(state.asset, state.query, drawing.environmentTimeline ?? drawing.environment) } : options.view === "forces" || options.view === "moments" ? { structure: solveStructure(state) } : {};
    const depiction = { ...drawing, view: options.view, viewpoint: target.viewpoint, scale: { denominator: options.scale }, representation: selector, transparent: transparentRaster } as DepictionOptions;
    return { state, analysis, scene: depict(state, depiction, analysis) };
  });
  const scenes = frames.map((frame) => frame.scene);
  await mkdir(options.out, { recursive: true });
  const where = target.kind === "sheet" ? `sheet-${target.projection}` : target.viewpoint;
  const stem = `${options.asset}-${options.clip ?? options.pose}${options.time === undefined ? "" : `-t${options.time}`}-${options.view}-${where}-1to${options.scale}`;
  const diagrams = options.view === "forces" || options.view === "moments" ? (["axial", "shear", "moment"] as const).filter((kind) => common.diagrams[kind]) : [];
  const noteOf = (index: number): string => `staffage ${options.asset} ${options.clip ?? options.pose}${options.time === undefined ? "" : ` t=${options.time}`} ${options.view} ${where} 1:${options.scale} ${scenes[index]!.metadata.fidelity}`;
  const modelOf = (index: number) => {
    const { state, analysis } = frames[index]!;
    const model = target.kind === "sheet" ? modelSheet(state, { projection: target.projection, denominator: options.scale, language: options.language, analysis, diagrams }) : modelScene(state, { viewpoint: target.viewpoint, analysis, diagrams });
    return withNote(model, noteOf(index), options.scale);
  };
''' + s[b:]

rep("for (const [index, state] of states.entries()) await emit(`${stem}-${frameName(index).replace(\".png\", \".dxf\")}`, writeDxf(modelOf(state, index), note));",
    "for (const index of scenes.keys()) await emit(`${stem}-${frameName(index).replace(\".png\", \".dxf\")}`, writeDxf(modelOf(index), noteOf(index)));")
rep("for (const [index, state] of states.entries()) await emit(`${stem}-${frameName(index).replace(\".png\", \".dwg\")}`, await writeDwg(modelOf(state, index), control));",
    "for (const index of scenes.keys()) await emit(`${stem}-${frameName(index).replace(\".png\", \".dwg\")}`, await writeDwg(modelOf(index), control));")
rep("`comment=${note}`", "`comment=${noteOf(0)}`")
rep("import { PROJECTIONS, depictSheet, modelSheet, type Projection }", "import { PROJECTIONS, modelSheet, resolveSheet, withNote, type Projection }")
rep("import { VIEWS, type DrawingScene, type EvaluatedState, type JobControl, type View }", "import { VIEWS, type DepictionOptions, type DrawingScene, type EvaluationQuery, type JobControl, type View }")
if crlf:
    s = s.replace("\n", "\r\n")
open(p, "w", encoding="utf-8", newline="").write(s)
