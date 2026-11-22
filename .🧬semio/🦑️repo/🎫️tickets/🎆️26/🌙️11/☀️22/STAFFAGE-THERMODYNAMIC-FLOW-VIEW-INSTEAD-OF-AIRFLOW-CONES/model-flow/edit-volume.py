"""One-shot editor of the volume module: replaces exact snippets, fails loudly when one is missing."""
import sys, pathlib

S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage")

def edit(rel, pairs):
    path = S / rel
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"{rel}: expected one match, found {text.count(old)} of: {old[:90]!r}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")

V = "🔨️modules/🧊️volume/🟦️.ts"
edit(V, [
    ("the `thermal` view (one material per thermal region and layer, painted from the solved field, animated per clip) and",
     "the `thermal` view (one material per thermal region and layer, painted from the solved field, animated per clip; with it the developed air flow of the static instant as stream tubes, arrow heads, vapour and fog puffs, see ./🌊️flow) and"),
    ("structural results and emitter states come from `evaluate`, `solveThermalAt`, `solveStructure` and `solveTransport`, and the colours from the shared `thermalPaint` and `memberPaint` of the 2D views;",
     "structural results and the flow field come from `evaluate`, `solveThermalAt`, `solveStructure` and `developTransport`, and the colours from the shared `thermalPaint`, `memberPaint` and `flowPaint` of the 2D views;"),
    ("every evaluated frame reports and checks, and the work yields to the event loop between frames.",
     "every evaluated frame and every slice of the developing air flow reports and checks, and the work yields to the event loop between them.\n * A clip carries the flow of its first instant as static geometry: the tubes do not follow the figure while the clip plays (the field of every frame would be a mesh per frame, which neither glTF nor FBX animates)."),
    ("import { MODES, VIEWS, VIEWS_OF, modeOf, sceneIdOf, type DisplayRange, type EvaluatedState, type EvaluationQuery, type ForceQuantity, type Mode, type ThermalQuantity, type View }",
     "import { MODES, VIEWS, VIEWS_OF, modeOf, sceneIdOf, type DisplayRange, type EvaluatedState, type EvaluationQuery, type FlowField, type ForceQuantity, type Mode, type ThermalQuantity, type View }"),
    ('import { solveTransport } from "../💨️transport/🟦️.ts";', 'import { developTransport, developedTimeS } from "../💨️transport/🟦️.ts";'),
    ("import { type Analytic, type FrameData, type FrameSetup, Slots, frameDataOf, glyphOf, glyphPaintOf } from \"./🧷️frames/🟦️.ts\";",
     "import { FLOW_MODEL_DEVELOP_S, FLOW_TAG, flowParts } from \"./🌊️flow/🟦️.ts\";\nimport { type Analytic, type FrameData, type FrameSetup, Slots, frameDataOf, glyphOf, glyphPaintOf } from \"./🧷️frames/🟦️.ts\";"),
    ("export { hexOf, linearOf, surfacesOf, type Surface, type Paint } from \"./🎨️looks/🟦️.ts\";",
     "export { hexOf, linearOf, surfacesOf, type Surface, type Paint } from \"./🎨️looks/🟦️.ts\";\nexport { FLOW_MODEL, FLOW_MODEL_DEVELOP_S, FLOW_TAG, excessBinOf, excessOfBin, flowParts, flowUnitOf, tubeStations, type FlowMaterial, type FlowParts } from \"./🌊️flow/🟦️.ts\";"),
    ("  /** 🔥️ Whether the thermal view carries a cone at every flowing emitter (default true); a viewer that draws the air flow itself asks for none. */\n  readonly emitterMarkers?: boolean;",
     "  /** 🌊️ Whether the thermal scenes carry the developed air flow of the static instant as geometry (default true); a viewer that animates the flow itself asks for none and saves developing it. */\n  readonly flowGeometry?: boolean;"),
    ("  readonly staticInstant: Instant;\n", "  readonly staticInstant: Instant;\n  readonly flow: FlowField | null;\n"),
    ("/** ⏱️ Evaluates and analyses every instant the models share (static state, poses, clip frames): the analyses are solved once whatever the number of scales and modes. */",
     "const FLOW_STEPS = 8;\n\n/** 💨️ The flow field of the static instant, developed as the drawings develop theirs: {@link FLOW_MODEL_DEVELOP_S} seconds of history before the instant (whole periods of a looping clip, so the pose stays), in the environment of the thermal colours, slice by slice with `report` and the cancellation of the operation. */\nasync function developedFlow(asset: Asset, at: EvaluationQuery, environment: EnvironmentInput, operation: Operation, report: (fraction: number) => void): Promise<FlowField | null> {\n  const clip = at.kind === \"clip\" ? asset.clips.find((c) => c.id === at.clip) : undefined;\n  const periodS = clip?.loop === true && clip.durationS > 0 ? clip.durationS : 0;\n  const query = { ...at, timeS: developedTimeS(Math.max(0, at.timeS ?? 0), FLOW_MODEL_DEVELOP_S, periodS) } as EvaluationQuery;\n  const pause = async (): Promise<void> => {\n    await yieldToLoop();\n    if (operation.cancelled()) throw new CancelledError();\n  };\n  return (await developTransport(asset, query, environment, { maxAgeS: FLOW_MODEL_DEVELOP_S, ...(periodS > 0 ? { periodS } : {}), flow: true, pathlines: false, streamlines: false, pause, progress: (done, total) => report(done / total) })).flow;\n}\n\n/** ⏱️ Evaluates and analyses every instant the models share (static state, poses, clip frames) and develops the air flow of the static instant: the analyses are solved once whatever the number of scales and modes. */"),
    ("  const total = 1 + request.poseIds.length + request.clipIds.reduce((n, id) => n + frameTimes(asset.clips.find((c) => c.id === id)!).length, 0);",
     "  const flowing = wants.thermal && options.flowGeometry !== false && asset.thermal.emitters.length > 0;\n  const total = (flowing ? FLOW_STEPS : 0) + 1 + request.poseIds.length + request.clipIds.reduce((n, id) => n + frameTimes(asset.clips.find((c) => c.id === id)!).length, 0);"),
    ("    if (wants.thermal && options.emitterMarkers !== false && asset.thermal.emitters.length > 0) analytic.emitters = solveTransport(asset, query, analytic.thermal!.environment, { maxAgeS: 0.5, pathlines: false, streamlines: false }).emitters;\n", ""),
    ("  const staticInstant = analyse(request.at, evaluate(asset, request.at));\n  const poses: { pose: Slug; state: EvaluatedState }[] = [];",
     "  const staticInstant = analyse(request.at, evaluate(asset, request.at));\n  const flow = flowing ? await developedFlow(asset, request.at, options.environment ?? asset.thermal.environment, operation, (fraction) => operation.report(Math.min(1, (done + FLOW_STEPS * fraction) / total))) : null;\n  if (flowing) done += FLOW_STEPS;\n  const poses: { pose: Slug; state: EvaluatedState }[] = [];"),
    ("  return { ...request, staticInstant, poses, clips };", "  return { ...request, staticInstant, flow, poses, clips };"),
    ("/** 🏭️ The fragment of a scale and mode: the list of solids, its surfaces and base materials, the looks of the mode's views bound as variants, the glyphs the mode's analyses need and their placements and colours at every instant. */",
     "/** 🏭️ The fragment of a scale and mode: the list of solids, its surfaces and base materials, the looks of the mode's views bound as variants, the glyphs the mode's analyses need and their placements at every instant, and in the thermal mode the flow parts of the scale. */"),
    ("    solids.push(...made.solids);\n  }\n\n  const edgeBase = \"edge\";",
     "    solids.push(...made.solids);\n  }\n  const flow = mode === \"thermal\" && instants.flow !== null ? flowParts(instants.flow, scale, Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM), theme, range) : null;\n  if (flow !== null) {\n    nodes.push(...flow.nodes);\n    solids.push(...flow.solids);\n    for (const m of flow.materials) materials.push(materialOf(m.id, m.title, hidden, true));\n  }\n\n  const edgeBase = \"edge\";"),
    ("      const mine = (view === \"forces\" && (s.kind === \"arrow\" || s.kind === \"mass\")) || (view === \"moments\" && s.kind === \"arc\") || (view === \"thermal\" && s.kind === \"emitter\");",
     "      const mine = (view === \"forces\" && (s.kind === \"arrow\" || s.kind === \"mass\")) || (view === \"moments\" && s.kind === \"arc\");"),
    ("glyphPaintOf(s, theme, staticFrame.glyphPaints)));\n      bindings[s.material] = id;\n      bindings[glyphEdgeOf(s.material)] = edgeId;\n    }\n",
     "glyphPaintOf(s, theme)));\n      bindings[s.material] = id;\n      bindings[glyphEdgeOf(s.material)] = edgeId;\n    }\n    if (view === \"thermal\" && flow !== null)\n      for (const m of flow.materials) {\n        materials.push(materialOf(`${view}-${m.id}`, m.title, m.paint, true));\n        bindings[m.id] = `${view}-${m.id}`;\n      }\n"),
    ("  const glyphIds = nodes.filter((n) => n.tags?.includes(\"glyph\")).map((n) => n.id);",
     "  const glyphIds = nodes.filter((n) => n.tags?.includes(\"glyph\") === true && !(n.tags.includes(FLOW_TAG) && n.parent !== null)).map((n) => n.id);"),
    ("      if (view === \"thermal\") {\n        for (const root of slots.order) {\n          const s = slots.map.get(root)!;\n          if (s.kind !== \"emitter\") continue;\n          const staticPaint = glyphPaintOf(s, theme, staticFrame.glyphPaints);\n          const channel = colourChannel(`thermal-${s.material}`, times, frames.map((f) => rgba(f.glyphPaints.get(s.material) ?? staticPaint)), rgba(staticPaint));\n          if (channel !== null) out.push(channel);\n        }\n      }\n", ""),
    ("/** 🌟️ The edge material of the feature-edge pass for the primitives of a procedural document: the base `edge` material of the primitive's scene (`<scene>-edge`; plain `edge` without scenes), except for the primitives of glyph solids, whose edges follow their own glyph material ({@link glyphEdgeOf}). */",
     "/** 🌟️ The edge material of the feature-edge pass for the primitives of a procedural document: the base `edge` material of the primitive's scene (`<scene>-edge`; plain `edge` without scenes), except for the primitives of glyph solids, whose edges follow their own glyph material ({@link glyphEdgeOf}), and of the flow solids, whose edges (the rims of tube ends and arrow heads) keep the material of their solid and so show and hide with it. */"),
    ("  return (source) => (glyphs.has(source.material) ? glyphEdgeOf(source.material) : `${prefixes.find((prefix) => source.material.startsWith(prefix)) ?? \"\"}edge`);",
     "  const flow = new Set(document.solids.filter((solid) => solid.tags?.includes(FLOW_TAG) === true).map((solid) => solid.material));\n  return (source) => (flow.has(source.material) ? source.material : glyphs.has(source.material) ? glyphEdgeOf(source.material) : `${prefixes.find((prefix) => source.material.startsWith(prefix)) ?? \"\"}edge`);"),
])

G = "🔨️modules/🧊️volume/🪧️glyphs/🟦️.ts"
edit(G, [
    ("a sphere for the centre of mass and cones for emitters.", "and a sphere for the centre of mass."),
    ("the node scale its radius; a cone lies along `+Y` from the origin. A glyph", "the node scale its radius. A glyph"),
    ("import type { ModelDocument, Node, Primitive, Solid, TransformPatch } from", "import type { Node, Solid, TransformPatch } from"),
    (", `massRadiusM` the radius of the centre-of-mass sphere and `plumeM` the length of an emitter cone. */", " and `massRadiusM` the radius of the centre-of-mass sphere. */"),
    ("readonly massRadiusM: number; readonly plumeM: number };", "readonly massRadiusM: number };"),
    ("massRadiusM: 1.3 * mm, plumeM: 5 * mm });", "massRadiusM: 1.3 * mm });"),
    ("at most `ARROW_SHARE` (arrows, plumes; moment arcs half of it as radius)", "at most `ARROW_SHARE` (arrows; moment arcs half of it as radius)"),
    ("massRadiusM: Math.min(1.3 * mm, MASS_SHARE * extentM), plumeM: Math.min(5 * mm, arrowCapM) };", "massRadiusM: Math.min(1.3 * mm, MASS_SHARE * extentM) };"),
    ("that an arrow or a plume may be long in a 3D model.", "that an arrow may be long in a 3D model."),
    ("/** 🌟️ The edge material of the feature-edge pass for the primitives of a procedural document: the shared `edge` material, except for the primitives of glyph solids, whose edges follow their own glyph material ({@link glyphEdgeOf}). */\nexport function edgeMaterialOf(document: ModelDocument): (source: Primitive) => Slug {\n  const glyphs = new Set(document.solids.filter((solid) => solid.tags?.includes(\"glyph\") === true).map((solid) => solid.material));\n  return (source) => (glyphs.has(source.material) ? glyphEdgeOf(source.material) : \"edge\");\n}\n\n", ""),
    ("\n\n/** 🔥️ The node and cone of one emitter marker glyph `id`: length `lengthM` (paper 5 mm at the scale unless capped) along the outflow, base radius 0.2 of the length. */\nexport function emitterGlyph(id: Slug, material: Slug, lengthM: number, at: TransformPatch = identity): { readonly nodes: Node[]; readonly solids: Solid[] } {\n  const root = `emitter-${id}`;\n  return { nodes: [glyphNode(root, null, at)], solids: [cone(root, root, material, lengthM, 0.2 * lengthM, 1)] };\n}\n\n/** 📤️ The patch of emitter marker `id` at `at` along `direction`; `null` (not flowing) hides it. */\nexport function emitterPatch(id: Slug, at: Triple | null, direction: Triple): Record<Slug, TransformPatch> {\n  const root = `emitter-${id}`;\n  if (at === null) return { [root]: { scale: HIDDEN } };\n  const n = portable.hypot(...direction) || 1;\n  return { [root]: { translation: at, rotation: quaternionOf(basisAlongY([direction[0] / n, direction[1] / n, direction[2] / n])), scale: [1, 1, 1] } };\n}", ""),
])

F = "🔨️modules/🧊️volume/🧷️frames/🟦️.ts"
edit(F, [
    ("the placements of the glyph nodes (force arrows, moment arcs, centre of mass, emitter cones) and the paint of every look and glyph.", "the placements of the glyph nodes (force arrows, moment arcs, centre of mass) and the paint of every look."),
    ("nothing about glyph placement, sizes or paints is computed twice.", "nothing about glyph placement or sizes is computed twice."),
    ("analytic values from the solved thermal field, structural result and emitters, colours", "analytic values from the solved thermal field and structural result, colours"),
    ("import type { EmitterState, EvaluatedState, StructuralResult, ThermalField, View }", "import type { EvaluatedState, StructuralResult, ThermalField, View }"),
    ("import { type Theme, THEMES } from", "import type { Theme } from"),
    ("arrowGlyph, arrowPatch, emitterGlyph, emitterPatch, massGlyph,", "arrowGlyph, arrowPatch, massGlyph,"),
    ("import { emitterPaint } from \"../🌡️thermal/🟦️.ts\";\n", ""),
    ("export type Analytic = { thermal?: ThermalField; structure?: StructuralResult; emitters?: readonly EmitterState[] };", "export type Analytic = { thermal?: ThermalField; structure?: StructuralResult };"),
    ("the paint of every analytic look by material key, the glyph node patches and the glyph paints by base material. */", "the paint of every analytic look by material key and the glyph node patches. */"),
    ("  readonly glyphs: Map<Slug, TransformPatch>;\n  readonly glyphPaints: Map<Slug, Paint>;\n", "  readonly glyphs: Map<Slug, TransformPatch>;\n"),
    ("its base material and a short detail (the load kind, the emitter id). */", "its base material and a short detail (the load or moment kind). */"),
    ("readonly kind: \"arrow\" | \"arc\" | \"mass\" | \"emitter\";", "readonly kind: \"arrow\" | \"arc\" | \"mass\";"),
    ("  const { asset, views, theme, mm, longestMm, looks, slots } = setup;", "  const { asset, views, mm, longestMm, looks, slots } = setup;"),
    ("  const glyphPaints = new Map<Slug, Paint>();\n", ""),
    ("  if (looks.thermal && analytic.thermal) {\n    paints.set(\"thermal\", looks.thermal.paint(analytic.thermal));\n    for (const emitter of asset.thermal.emitters) {\n      const root = `emitter-${slugOf(emitter.id)}`;\n      slots.add(root, { kind: \"emitter\", material: `glyph-emitter-${slugOf(emitter.id)}`, detail: emitter.id });\n      const e = analytic.emitters?.find((s) => s.emitter === emitter.id);\n      put(emitterPatch(slugOf(emitter.id), e !== undefined && e.active ? toNeutral([e.at.x, e.at.y, e.at.z]) : null, e === undefined ? [0, 1, 0] : toNeutral([e.direction.x, e.direction.y, e.direction.z])));\n      if (e !== undefined) glyphPaints.set(`glyph-emitter-${slugOf(emitter.id)}`, emitterPaint(e, analytic.thermal.environment.airTempC, theme));\n    }\n  }\n",
     "  if (looks.thermal && analytic.thermal) paints.set(\"thermal\", looks.thermal.paint(analytic.thermal));\n"),
    ("    else if (s.kind === \"arc\") put(arcPatch(id, null, [0, 1, 0], 0));\n    else if (s.kind === \"emitter\") put(emitterPatch(id, null, [0, 1, 0]));\n", "    else if (s.kind === \"arc\") put(arcPatch(id, null, [0, 1, 0], 0));\n"),
    ("  return { locals, paints, glyphs, glyphPaints };", "  return { locals, paints, glyphs };"),
    (": slot.kind === \"mass\" ? massGlyph(slot.material, sizing.massRadiusM, at) : emitterGlyph(id.replace(/^emitter-/u, \"\"), slot.material, sizing.plumeM, at);", ": massGlyph(slot.material, sizing.massRadiusM, at);"),
    ("/** 🎨️ The paint of the glyph of a slot: the arrow paint of its load kind, the arc paint of its moment kind, the ink of the centre of mass, or the temperature paint of an emitter at this instant (`glyphPaints` of its {@link FrameData}, else a 55 % opaque ink). */\nexport function glyphPaintOf(slot: Slot, theme: Theme, glyphPaints: ReadonlyMap<Slug, Paint>): Paint {\n  return slot.kind === \"arrow\" ? arrowPaint(theme, slot.detail as ArrowKind) : slot.kind === \"arc\" ? arcPaint(theme, slot.detail as ArcKind) : slot.kind === \"mass\" ? massPaint(theme) : (glyphPaints.get(slot.material) ?? { hex: THEMES[theme].ink, alpha: 0.55 });",
     "/** 🎨️ The paint of the glyph of a slot: the arrow paint of its load kind, the arc paint of its moment kind or the ink of the centre of mass. */\nexport function glyphPaintOf(slot: Slot, theme: Theme): Paint {\n  return slot.kind === \"arrow\" ? arrowPaint(theme, slot.detail as ArrowKind) : slot.kind === \"arc\" ? arcPaint(theme, slot.detail as ArcKind) : massPaint(theme);"),
])

T = "🔨️modules/🧊️volume/🌡️thermal/🟦️.ts"
edit(T, [
    (",\n * and the emitters as translucent cone glyphs coloured by their temperature excess on the transport ramp (±20 K).", ".\n * The air around the body is not part of this look: the flow parts of ../🌊️flow carry it."),
    (" * @see ../../💨️transport/🟦️.ts — `solveTransport`\n", " * @see ../🌊️flow/🟦️.ts — the air flow of the thermal scenes\n"),
    ("import type { DisplayRange, EmitterState, ThermalField, ThermalQuantity }", "import type { DisplayRange, ThermalField, ThermalQuantity }"),
    ("import { type Theme, divergingColour, toneColour, tx }", "import { type Theme, toneColour, tx }"),
    ("import { SHARED_EXCESS_K } from \"../../🎨️depiction/💨️transport-view/🟦️.ts\";\n", ""),
    ("\n\n/** 🔥️ The paint of an emitter glyph: the temperature excess over the ambient air on the diverging ramp of the transport view (warm above), 55 % opaque. */\nexport function emitterPaint(state: EmitterState, airTempC: number, theme: Theme): Paint {\n  const excess = state.temperatureC - airTempC;\n  return { hex: divergingColour(excess, { min: -SHARED_EXCESS_K, centre: 0, max: SHARED_EXCESS_K }, true, theme), alpha: 0.55 };\n}", ""),
])

A = "🔨️modules/🎨️depiction/🧊️axonometric/🟦️.ts"
edit(A, [
    ("the arrows, arcs, centre of mass and emitter cones of the analytic views are the 3D glyph solids of the model.", "the arrows, arcs and centre of mass of the analytic views are the 3D glyph solids of the model; the air flow of the thermal view is not a solid here but the flow layers of `flowView`, projected through this camera."),
    ("import type { EmitterState, Fill, Legend, StructuralResult, ThermalField, LayerId, PathCommand }", "import type { Fill, Legend, StructuralResult, ThermalField, LayerId, PathCommand }"),
    ("/** 🧮️ The analyses the analytic views read (the emitters belong to the thermal view). */\nexport type Analysis3d = { readonly thermal?: ThermalField; readonly structure?: StructuralResult; readonly emitters?: readonly EmitterState[] };", "/** 🧮️ The analyses the analytic views read. */\nexport type Analysis3d = { readonly thermal?: ThermalField; readonly structure?: StructuralResult };"),
    ("readonly kind: \"arrow\" | \"arc\" | \"mass\" | \"emitter\"; readonly material: Slug;", "readonly kind: \"arrow\" | \"arc\" | \"mass\"; readonly material: Slug;"),
    ("glyphPaintOf(slot, theme, frame.glyphPaints);", "glyphPaintOf(slot, theme);"),
])
print("ok")
