"""🛬️ The engine change of lane CL-3D as one reproducible edit: the roundness levels of an asset (a model heavier than the triangle budget is tessellated on a lower level, and so is a scale whose lists would grow against the finer one) in `🧊️volume/⏱️job`, its language-agnostic rule and vectors in the fixture `🧊️model-roundness`, and the cases, the oracle and the scenario of the test `🧊️model-roundness`. `land.py --stage <dir>` writes the patched job module into <dir> and `--check` applies every edit in memory only with its imports pointed at the live tree (for a trial that touches nothing); A second change is the facets of the radial folds as limitless spherical joints in `🧊️volume/🦴️skeleton` with its case in `🧊️model-skeleton`. `land.py --live roundness|folds…` reads each live file right before it writes it and refuses when a text it replaces is not there exactly once (a second run changes nothing)."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 9))
S = os.path.join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage")
JOB = os.path.join(S, "🔨️modules/🧊️volume/⏱️job/🟦️.ts")
FIXTURE = os.path.join(S, "🧫️fixtures/🧊️model-roundness/🔣️.json")
TEST = os.path.join(S, "🧪️tests/🧊️model-roundness/🟦️.ts")
ORACLE = os.path.join(S, "🧪️tests/🧊️model-roundness/🐍️.py")
FEATURE = os.path.join(S, "🧪️tests/🧊️model-roundness/🥒️.feature")
SKELETON = os.path.join(S, "🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts")
SKELETON_TEST = os.path.join(S, "🧪️tests/🧊️model-skeleton/🟦️.ts")
SKELETON_FEATURE = os.path.join(S, "🧪️tests/🧊️model-skeleton/🥒️.feature")
MODEL_TRIANGLES = 300000


def swap(text, old, new, path):
    if new in text:
        return text
    if text.count(old) != 1:
        raise SystemExit(f"{path}: expected once, found {text.count(old)} times: {old[:90]!r}")
    return text.replace(old, new)


def job(text):
    text = swap(text, 'import type { Asset, Text } from "../../../🧬️schema/🟦️.ts";', 'import { VOLUME_LISTS, type Asset, type AuthoredVolume, type ScaleVolume, type Text, type VolumeSolid } from "../../../🧬️schema/🟦️.ts";', JOB)
    text = swap(text, 'import type { JobControl } from "../../🔗️contracts/🟦️.ts";', 'import { circleSegments } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/📏️segments/🟦️.ts";\nimport type { JobControl } from "../../🔗️contracts/🟦️.ts";\nimport { listOf } from "../🧊️solids/🟦️.ts";', JOB)
    text = swap(text, "the validators measure at a chord of 1/`referencePerChord` of the largest dimension, clamped to `referenceChordM`. */\nexport const ROUNDNESS = { paperFeatureM: 0.00015, chordPerFeature: 0.02, largestPerChord: 3200, roundSegments: 24, minSegments: 12, referencePerChord: 800, referenceChordM: [0.0005, 0.02] } as const;", "the validators measure at a chord of 1/`referencePerChord` of the largest dimension, clamped to `referenceChordM`. All of it holds on the full roundness level `fullLevel`; the solids of all authored scales of one asset hold at most `modelTriangles` nominal triangles ({@link nominalTriangles}), and a heavier model, or a scale whose lists would grow, is on a lower level ({@link roundnessLevels}). */\nexport const ROUNDNESS = { paperFeatureM: 0.00015, chordPerFeature: 0.02, largestPerChord: 3200, roundSegments: 24, minSegments: 12, referencePerChord: 800, referenceChordM: [0.0005, 0.02], fullLevel: 64, modelTriangles: %d } as const;" % MODEL_TRIANGLES, JOB)
    text = swap(text, "/** 🧢️ The most segments of one full circle at a scale: 96 up to 1:20, 64 up to 1:50, 48 up to 1:100, 40 up to 1:200 and 32 beyond, so that a coarser scale never has more triangles than a finer one and the triangle budgets hold. */", "/** 🧢️ The most segments of one full circle at a scale on the full roundness level: 96 up to 1:20, 64 up to 1:50, 48 up to 1:100, 40 up to 1:200 and 32 beyond, so that a coarser scale never has more triangles of the same solid than a finer one. */", JOB)
    text = swap(text, "/** 🎚️ The tessellation quality of an asset at a scale, the one rule of the viewer, every download and the budget oracle ({@link ROUNDNESS}): adaptive to curvature and size, the chord deviation is the smaller of 2 % of the smallest feature of the scale and 1/3200 of the largest dimension, a circle larger than one feature never has fewer than 24 segments, and the segments are capped by {@link maxSegmentsAt}.", """/** 🪜️ The most segments of one full circle at a scale on roundness level `level`: on the full level the cap of {@link maxSegmentsAt}; below it every cap falls in proportion but not under the round floor of 24; below 24 every cap is the level itself. */
export const levelSegmentsAt = (scale: number, level: number): number => (level >= ROUNDNESS.roundSegments ? Math.max(ROUNDNESS.roundSegments, Math.floor((maxSegmentsAt(scale) * level) / ROUNDNESS.fullLevel)) : Math.min(maxSegmentsAt(scale), level));

/** 🔢️ The nominal triangles of an authored solid at a quality, the measure of the triangle budget: the grid of its tessellator without seams, creases and end discs. With n the segments of its largest circle: an ellipsoid 2 n (⌈n/2⌉ − 1), a capsule 4 n ⌈n/4⌉, a loft n (2 (sections − 1) + caps), a revolve 2 columns per profile point (n columns for a full turn), a rounded box 12 (2 ⌈n/4⌉ + 1)², a box 12, an extrusion 4 per outline point, a polyhedron its triangles; times its copies. */
export function nominalTriangles(solid: VolumeSolid, quality: Quality): number {
  const copies = solid.repeat?.count ?? 1;
  const quarter = (n: number): number => Math.max(2, Math.ceil(n / 4));
  switch (solid.kind) {
    case "ellipsoid": {
      const n = circleSegments(Math.max(...solid.radii), quality);
      return copies * 2 * n * (Math.max(2, Math.ceil(n / 2)) - 1);
    }
    case "capsule": {
      const n = circleSegments(Math.max(solid.r0, solid.r1) * Math.max(1, solid.lateralScale ?? 1), quality);
      return copies * 4 * n * quarter(n);
    }
    case "loft": {
      const n = Math.max(4, circleSegments(Math.max(...solid.sections.map((section) => Math.max(section.rx, section.ry))), quality));
      return copies * n * (2 * (solid.sections.length - 1) + Number(solid.capStart) + Number(solid.capEnd));
    }
    case "revolve": {
      const n = circleSegments(Math.max(...solid.profile.map((point) => point[1])) * Math.max(1, solid.lateralScale ?? 1), quality);
      return copies * 2 * Math.max(1, Math.ceil((n * Math.min(solid.sweepRad ?? 2 * Math.PI, 2 * Math.PI)) / (2 * Math.PI))) * solid.profile.length;
    }
    case "box": {
      const radius = Math.min(solid.cornerRadius ?? 0, ...solid.size.map((edge) => edge / 2));
      return copies * (radius > 0 ? 12 * (2 * quarter(circleSegments(radius, quality)) + 1) ** 2 : 12);
    }
    case "extrude":
      return copies * 4 * (solid.profile.length + (solid.holes ?? []).reduce((sum, hole) => sum + hole.length, 0));
    case "polyhedron":
      return copies * solid.triangles.length;
  }
}

/** 🎛️ The quality of an asset at a scale on roundness level `level`: the rule on the full level; below it the chord deviation grows by (`fullLevel` / level)², so that a circle the chord decides has about level / `fullLevel` of its segments as a capped one has ({@link levelSegmentsAt}), while the floors of 24 and 12 segments stay. */
export function qualityOn(asset: Asset, scale: number, level: number): Quality {
  const featureM = ROUNDNESS.paperFeatureM * scale;
  const chordM = Math.min(Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM) / ROUNDNESS.largestPerChord, ROUNDNESS.chordPerFeature * featureM);
  return { chordM: chordM * (ROUNDNESS.fullLevel / level) ** 2, minSegments: ROUNDNESS.minSegments, maxSegments: levelSegmentsAt(scale, level), round: { radiusM: featureM, segments: ROUNDNESS.roundSegments } };
}

/** 🧮️ The nominal triangles of one authored scale on a roundness level: every solid of its body, thermal and structure lists once (a reference adds nothing to the solid it reuses). */
export function volumeTriangles(asset: Asset, volume: AuthoredVolume, level: number): number {
  const quality = qualityOn(asset, volume.denominator, level);
  return [...volume.body, ...volume.thermal, ...volume.structure].reduce((sum, solid) => sum + ("kind" in solid ? nominalTriangles(solid, quality) : 0), 0);
}

/** 🏋️ The roundness levels of an asset: `model` for a scale it has not authored, `scales` per authored denominator. */
export type RoundnessLevels = { readonly model: number; readonly scales: ReadonlyMap<number, number> };

const remembered = new WeakMap<Asset, RoundnessLevels>();

/** 🪂️ The roundness levels of an asset, remembered per asset. `model` is the full level unless the solids of all its authored scales hold more than `modelTriangles` nominal triangles ({@link volumeTriangles}), then the highest level on which they stay within it, never below `minSegments`: a tree of some hundred foliage masses or a figure with its tools trades rounder for lighter, so the triangle and file budgets of its 3D files hold, and every model within the budget is tessellated by the rule alone. A scale is on the model's level unless one of its lists (references resolved) would then hold more nominal triangles than the same list one scale finer, as happens where markers and rods at the smallest feature grow with the scale: then on the highest level on which no list grows. */
export function roundnessLevels(asset: Asset): RoundnessLevels {
  const known = remembered.get(asset);
  if (known !== undefined) return known;
  const volumes = Object.values((asset.volumes ?? {}) as Readonly<Record<string, ScaleVolume>>).filter((volume): volume is AuthoredVolume => volume.status === "authored").sort((a, b) => a.denominator - b.denominator);
  const lists = (volume: AuthoredVolume, level: number): number[] => VOLUME_LISTS.map((list) => listOf(volume, list).reduce((sum, solid) => sum + nominalTriangles(solid, qualityOn(asset, volume.denominator, level)), 0));
  let model: number = ROUNDNESS.fullLevel;
  while (model > ROUNDNESS.minSegments && volumes.reduce((sum, volume) => sum + volumeTriangles(asset, volume, model), 0) > ROUNDNESS.modelTriangles) model -= 1;
  const scales = new Map<number, number>();
  volumes.forEach((volume, index) => {
    let level = model;
    const finer = index === 0 ? undefined : lists(volumes[index - 1]!, scales.get(volumes[index - 1]!.denominator)!);
    while (finer !== undefined && level > ROUNDNESS.minSegments && lists(volume, level).some((triangles, list) => triangles > finer[list]!)) level -= 1;
    scales.set(volume.denominator, level);
  });
  const levels = { model, scales };
  remembered.set(asset, levels);
  return levels;
}

/** 🎚️ The tessellation quality of an asset at a scale, the one rule of the viewer, every download and the budget oracle ({@link ROUNDNESS}): adaptive to curvature and size, the chord deviation is the smaller of 2 % of the smallest feature of the scale and 1/3200 of the largest dimension, a circle larger than one feature never has fewer than 24 segments, and the segments are capped by {@link maxSegmentsAt}; a model heavier than the triangle budget, and a scale whose lists would grow, is tessellated on its lower roundness level ({@link roundnessLevels}, {@link qualityOn}).""", JOB)
    text = swap(text, """  const featureM = ROUNDNESS.paperFeatureM * scale;
  return { chordM: Math.min(largest / ROUNDNESS.largestPerChord, ROUNDNESS.chordPerFeature * featureM), minSegments: ROUNDNESS.minSegments, maxSegments: maxSegmentsAt(scale), round: { radiusM: featureM, segments: ROUNDNESS.roundSegments } };
}""", """  const levels = roundnessLevels(asset);
  return qualityOn(asset, scale, levels.scales.get(scale) ?? levels.model);
}""", JOB)
    return text


FIXTURE_COMMENT = 'feature(scale))). All of that is the full roundness level `fullLevel`. On level L the cap of a scale is max(roundSegments, floor(maxSegments[scale] * L / fullLevel)) for L >= roundSegments and min(maxSegments[scale], L) below, and the chord is chordM(asset, scale) * (fullLevel / L)^2; minSegments and the round floor stay. Nominal triangles of a solid whose largest circle has n segments by the rule on its level (ellipsoid: largest radius; capsule: max(r0, r1) * max(1, lateralScale); loft: the largest rx or ry, n at least 4; revolve: the largest radial value * max(1, lateralScale); rounded box: its corner radius): ellipsoid 2 n (max(2, ceil(n / 2)) - 1), capsule 4 n max(2, ceil(n / 4)), loft n (2 (sections - 1) + capStart + capEnd), revolve 2 * max(1, ceil(n * sweep / 2 pi)) * profile points, box 12 or with a corner radius 12 (2 max(2, ceil(n / 4)) + 1)^2, extrude 4 * (outline points + hole points), polyhedron its triangles; all times repeat.count. The model level of an asset is the highest L of fullLevel down to minSegments on which the nominal triangles of the solids of all its authored scales (each solid of the body, thermal and structure lists once, a reference none) are at most `modelTriangles`; a model heavier than that on the lowest level stays on it. The level of an authored scale is the model level, but from the second finest scale on the highest level not above it, down to minSegments, on which none of its three lists (references resolved to their body solids) has more nominal triangles than the same list of the next finer scale on its own level. `budget.levels` are the caps of `scales` on a level; `budget.models` are synthetic assets of `ellipsoids[scale]` = [count, radiusM] body ellipsoids of radii [radiusM, 0.7 radiusM, 0.8 radiusM] with their model level, the level, cap, chord and nominal triangles of every scale and the nominal triangles on the full level; `budget.catalog` are the catalog assets whose levels the oracle recomputes from their files; `budget.tessellatedPerNominal` is the range of the tessellated triangles of such a model over its nominal ones. Every solid of `kinds`'


def fixture(text):
    if '"budget"' in text:
        return text
    models = json.load(open(os.path.join(HERE, "stage", "models.json"), encoding="utf-8"))
    text = swap(text, 'feature(scale))). Every solid of `kinds`', FIXTURE_COMMENT, FIXTURE)
    text = swap(text, '  "maxSegments": { "20": 96, "50": 64, "100": 48, "200": 40, "500": 32 }\n },', '  "maxSegments": { "20": 96, "50": 64, "100": 48, "200": 40, "500": 32 },\n  "fullLevel": 64,\n  "modelTriangles": %d\n },' % MODEL_TRIANGLES, FIXTURE)
    one = lambda value: json.dumps(value, separators=(", ", ": ")).replace("{", "{ ").replace("}", " }")
    lines = ",\n".join("   " + one(model) for model in models)
    text = swap(text, ' "catalog": ["tealight", "thermostatic-radiator-valve", "standing-adult", "city-car"]\n}', ' "catalog": ["tealight", "thermostatic-radiator-valve", "standing-adult", "city-car"],\n "budget": {\n  "levels": [\n   { "level": 64, "segments": [96, 64, 48, 40, 32] },\n   { "level": 48, "segments": [72, 48, 36, 30, 24] },\n   { "level": 40, "segments": [60, 40, 30, 25, 24] },\n   { "level": 32, "segments": [48, 32, 24, 24, 24] },\n   { "level": 24, "segments": [36, 24, 24, 24, 24] },\n   { "level": 23, "segments": [23, 23, 23, 23, 23] },\n   { "level": 12, "segments": [12, 12, 12, 12, 12] }\n  ],\n  "models": [\n' + lines + '\n  ],\n  "catalog": ["tealight", "standing-adult", "city-car", "rowan", "sweetgum", "small-leaved-lime", "used-upright-piano", "walking-toddler"],\n  "tessellatedPerNominal": [0.8, 1.2]\n }\n}', FIXTURE)
    json.loads(text)
    return text


def skeleton(text):
    text = swap(text, "A fixed joint is fixed; the rig root is `free` (the evaluated root placement moves it).", "A fixed joint is fixed; the rig root is `free` (the evaluated root placement moves it). A facet of a radial fold (`rig.radialFolds`) is carried by its two rays, not by its own joint: it is `spherical` without limits, so every pose and clip turns it where the fold puts it (as a `fixed` joint the kernel ignores its pose and the canopy stays in its rest fold).", SKELETON)
    text = swap(text, """/** 🔩️ The mechanism joint of a bone: see the module description. */
export function jointOf(bone: Bone, root: boolean, outOfPlane: boolean): ModelJoint {
  if (root) return { type: "free", axis: [0, 1, 0] };""", """/** 🔩️ The mechanism joint of a bone: see the module description; `folded` marks a facet of a radial fold. */
export function jointOf(bone: Bone, root: boolean, outOfPlane: boolean, folded = false): ModelJoint {
  if (root) return { type: "free", axis: [0, 1, 0] };
  if (folded) return { type: "spherical", axis: [1, 0, 0], max: Math.PI, twistMin: -Math.PI, twistMax: Math.PI };""", SKELETON)
    text = swap(text, """  const moving = outOfPlaneBones(asset);
  return parentFirstOrder(asset.rig).map((bone) => ({""", """  const moving = outOfPlaneBones(asset);
  const folded = new Set((asset.rig.radialFolds ?? []).flatMap((fold) => fold.facets));
  return parentFirstOrder(asset.rig).map((bone) => ({""", SKELETON)
    text = swap(text, "    joint: jointOf(bone, bone.parent === null, moving.has(bone.id)),", "    joint: jointOf(bone, bone.parent === null, moving.has(bone.id), folded.has(bone.id)),", SKELETON)
    return text


def skeleton_test(text):
    return swap(text, """  it("replays every pose, named or key, of the sample figure to the evaluated bone frames", async () => {""", """  it.each(["person-with-umbrella", "market-parasol"])("%s: every facet of a radial fold is a limitless spherical joint and follows its rays in every pose", { timeout: 600_000 }, async (id) => {
    const asset = catalogAsset(id);
    const facets = asset.rig.radialFolds!.flatMap((fold) => fold.facets);
    const document = await modelOf(asset, { scale: 500, view: "light-solid", poses: asset.poses.map((p) => p.id), clips: [] });
    expect(facets.length).toBeGreaterThan(0);
    for (const facet of facets) expect(document.nodes.find((n) => n.id === facet)!.joint, facet).toEqual({ type: "spherical", axis: [1, 0, 0], max: Math.PI, twistMin: -Math.PI, twistMax: Math.PI });
    for (const pose of document.poses) {
      const replay = worldOf(document, pose.id);
      expect(replay.issues, pose.id).toEqual([]);
      const state = evaluate(asset, { kind: "pose", pose: pose.id });
      for (const facet of facets) near(replay.world[facet]!, expectedWorld(state.bones[facet]!.frame).toArray(), vectors.tolerances.poseWorldM, `${pose.id}/${facet}`);
    }
  });

  it("replays every pose, named or key, of the sample figure to the evaluated bone frames", async () => {""", SKELETON_TEST)


def skeleton_feature(text):
    if "Scenario: The facets of a radial fold follow their rays" in text:
        return text
    return text.rstrip("\\n") + """

  Scenario: The facets of a radial fold follow their rays
    Given the person with the umbrella and the market parasol, whose canopy facets are solved by `rig.radialFolds` between two ribs
    When every pose is replayed through the kernel
    Then every facet is a spherical joint without limits, no pose patch is dropped and three's matrices of the evaluated facet frames are the replayed ones
"""


ORACLE_FUNCTIONS = open(os.path.join(HERE, "stage", "budget.py"), encoding="utf-8").read()


def oracle(text):
    text = swap(text, '- `hardRim`: the vertices of the rims (y = 0 or y = length at the full radius) whose normal is axial."""', '- `hardRim`: the vertices of the rims (y = 0 or y = length at the full radius) whose normal is axial.\n\nWith `{"rule": the rule of the fixture, "models": [{"name", "largestM", "volumes": {scale: {"body": [authored solid, …], "thermal": […], "structure": […]}}}]}` the document also asks for the roundness levels of models. The output then holds per model name `{"model", "levels": {scale: level}, "segments": {scale: cap}, "chordM": {scale: chord}, "triangles": {scale: nominal triangles on its level}, "fullTriangles"}`: the highest level of `fullLevel` down to `minSegments` on which the nominal triangles of all scales (the grid sizes of the kernel tessellators, see the fixture) stay within `modelTriangles`, and per scale the level on which no list has more nominal triangles than the same list one scale finer."""', ORACLE)
    text = swap(text, 'document = json.load(open(sys.argv[1], encoding="utf-8"))\njson.dump({entry["name"]: measure(entry) for entry in document["meshes"]}, sys.stdout)', ORACLE_FUNCTIONS + '\n\ndocument = json.load(open(sys.argv[1], encoding="utf-8"))\njson.dump({**{entry["name"]: measure(entry) for entry in document.get("meshes", [])}, **{model["name"]: budget(model, document["rule"]) for model in document.get("models", [])}}, sys.stdout)', ORACLE)
    return text


def test(text):
    text = swap(text, 'import type { Asset } from "../../🧬️schema/🟦️.ts";', 'import type { Asset, AuthoredVolume } from "../../🧬️schema/🟦️.ts";', TEST)
    text = swap(text, 'import { qualityOf } from "../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";', 'import { ROUNDNESS, levelSegmentsAt, qualityOf, qualityOn, roundnessLevels, volumeTriangles } from "../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";\nimport { solidOf } from "../../🔨️modules/🧊️volume/🧊️solids/🟦️.ts";', TEST)
    text = swap(text, "type Rule = { paperFeatureMm: number; chordPerFeature: number; largestPerChord: number; roundSegments: number; minSegments: number; maxSegments: Record<string, number> };", "type Rule = { paperFeatureMm: number; chordPerFeature: number; largestPerChord: number; roundSegments: number; minSegments: number; maxSegments: Record<string, number>; fullLevel: number; modelTriangles: number };\ntype Budgeted = { model: number; levels: Record<string, number>; segments: Record<string, number>; chordM: Record<string, number>; triangles: Record<string, number>; fullTriangles: number };\ntype Budget = { levels: { level: number; segments: number[] }[]; models: ({ name: string; largestM: number; ellipsoids: Record<string, [number, number]> } & Budgeted)[]; catalog: string[]; tessellatedPerNominal: [number, number] };", TEST)
    text = swap(text, "type Vectors = { rule: Rule; scales: number[]; assets: { name: string; largestM: number }[]; radiiM: number[]; kinds: Kind[]; catalog: string[] };", "type Vectors = { rule: Rule; scales: number[]; assets: { name: string; largestM: number }[]; radiiM: number[]; kinds: Kind[]; catalog: string[]; budget: Budget };", TEST)
    text = swap(text, 'const cross = (a: readonly number[], b: readonly number[]): number[] =>', """const authored = (asset: Asset): AuthoredVolume[] => Object.values(asset.volumes).filter((volume): volume is AuthoredVolume => volume.status === "authored").sort((a, b) => a.denominator - b.denominator);
const budgetOf = (models: readonly { name: string; asset: Asset }[]): Record<string, Budgeted> => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-roundness-"));
  const light = (solids: readonly object[]): object[] => solids.map((solid) => ("vertices" in solid ? { ...solid, vertices: [] } : solid));
  try {
    writeFileSync(join(directory, "input.json"), JSON.stringify({ rule, models: models.map(({ name, asset }) => ({ name, largestM: Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM), volumes: Object.fromEntries(authored(asset).map((volume) => [volume.denominator, { body: light(volume.body), thermal: light(volume.thermal), structure: light(volume.structure) }])) })) }));
    return JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, Budgeted>;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};
const budgeted = (asset: Asset): Budgeted => {
  const levels = roundnessLevels(asset);
  const scales = authored(asset).map((volume) => [volume, levels.scales.get(volume.denominator)!] as const);
  const per = <T,>(value: (volume: AuthoredVolume, level: number) => T): Record<string, T> => Object.fromEntries(scales.map(([volume, level]) => [volume.denominator, value(volume, level)]));
  return { model: levels.model, levels: per((_, level) => level), segments: per((volume) => qualityOf(asset, volume.denominator).maxSegments), chordM: per((volume) => qualityOf(asset, volume.denominator).chordM), triangles: per((volume, level) => volumeTriangles(asset, volume, level)), fullTriangles: scales.reduce((sum, [volume]) => sum + volumeTriangles(asset, volume, rule.fullLevel), 0) };
};
const sameBudget = (found: Budgeted, wanted: Budgeted, name: string): void => {
  expect({ ...found, chordM: null }, name).toEqual({ ...wanted, chordM: null });
  for (const [scale, chordM] of Object.entries(wanted.chordM)) expect(found.chordM[scale]! / chordM, `${name} chord 1:${scale}`).toBeCloseTo(1, 9);
};

describe("⭕️ a model heavier than the triangle budget trades rounder for lighter, and no list grows towards a coarser scale", () => {
  const { budget } = vectors;
  const grove = (model: Budget["models"][number]): Asset => ({ ...assetOf(model.largestM), volumes: Object.fromEntries(Object.entries(model.ellipsoids).map(([key, [count, radiusM]]) => [key, { status: "authored", denominator: Number(key), body: Array.from({ length: count }, (_, i) => ({ id: `mass-${i}`, kind: "ellipsoid", radii: [radiusM, 0.7 * radiusM, 0.8 * radiusM] })), thermal: [], structure: [], decisions: [] }])) }) as unknown as Asset;
  const wantedOf = ({ model, levels, segments, chordM, triangles, fullTriangles }: Budgeted): Budgeted => ({ model, levels, segments, chordM, triangles, fullTriangles });

  it("the budget of the fixture is the budget of the engine", () => expect([ROUNDNESS.fullLevel, ROUNDNESS.modelTriangles]).toEqual([rule.fullLevel, rule.modelTriangles]));

  it.each(budget.levels)("level $level: the caps of the scales", ({ level, segments }) => {
    expect(vectors.scales.map((scale) => levelSegmentsAt(scale, level))).toEqual(segments);
    if (level === rule.fullLevel) expect(segments).toEqual(vectors.scales.map((scale) => rule.maxSegments[String(scale)]));
  });

  it("on the full level the quality is the rule, and an asset without volumes is on it", () => {
    for (const { largestM } of vectors.assets) {
      expect(roundnessLevels(assetOf(largestM)).model).toBe(rule.fullLevel);
      for (const scale of vectors.scales) expect(qualityOn(assetOf(largestM), scale, rule.fullLevel)).toEqual(qualityOf(assetOf(largestM), scale));
    }
  });

  it("the levels, caps, chords and nominal triangles of the fixture models, as the oracle recomputes them", { timeout: 600_000 }, () => {
    requireTool("uv");
    const models = budget.models.map((model) => ({ name: model.name, asset: grove(model) }));
    const oracle = budgetOf(models);
    for (const [index, model] of budget.models.entries()) {
      const { asset } = models[index]!;
      sameBudget(budgeted(asset), wantedOf(model), model.name);
      sameBudget(oracle[model.name]!, wantedOf(model), `${model.name} (oracle)`);
      const total = (level: number): number => authored(asset).reduce((sum, volume) => sum + volumeTriangles(asset, volume, level), 0);
      if (model.model > rule.minSegments) expect(total(model.model), model.name).toBeLessThanOrEqual(rule.modelTriangles);
      if (model.model < rule.fullLevel) expect(total(model.model + 1), model.name).toBeGreaterThan(rule.modelTriangles);
      const row = Object.keys(model.triangles).sort((a, b) => Number(a) - Number(b)).map((key) => model.triangles[key]!);
      for (let i = 1; i < row.length; i += 1) if (model.levels[Object.keys(model.levels)[i]!]! > rule.minSegments) expect(row[i]!, `${model.name} falls`).toBeLessThanOrEqual(row[i - 1]!);
    }
  });

  it("the catalog: the levels of the oracle, a light asset on the full level, and the tessellation near the nominal triangles", { timeout: 900_000 }, async () => {
    requireTool("uv");
    const entries = (await assetPathsFromFiles()).filter((e) => budget.catalog.includes(e.folder));
    expect(entries.map((e) => e.folder).sort()).toEqual([...budget.catalog].sort());
    const models: { name: string; asset: Asset }[] = [];
    for (const entry of entries) models.push({ name: entry.folder, asset: (await readStreamedAsset(entry)).asset });
    const oracle = budgetOf(models);
    for (const { name, asset } of models) {
      const mine = budgeted(asset);
      sameBudget(mine, oracle[name]!, name);
      if (mine.fullTriangles <= rule.modelTriangles) expect(mine.model, name).toBe(rule.fullLevel);
      let tessellated = 0;
      for (const volume of authored(asset)) for (const solid of [...volume.body, ...volume.thermal, ...volume.structure]) if ("kind" in solid) tessellated += tessellateSolid(solidOf(solid as never, "budget"), qualityOf(asset, volume.denominator)).indices.length / 3;
      const nominal = Object.values(mine.triangles).reduce((sum, n) => sum + n, 0);
      expect(tessellated / nominal, name).toBeGreaterThanOrEqual(budget.tessellatedPerNominal[0]);
      expect(tessellated / nominal, name).toBeLessThanOrEqual(budget.tessellatedPerNominal[1]);
    }
  });
});

const cross = (a: readonly number[], b: readonly number[]): number[] =>""", TEST)
    return text


def feature(text):
    text = swap(text, "so that the triangle budgets hold and a coarser scale never has more triangles than a finer one.", "so that a coarser scale never has more triangles of the same solid than a finer one. The caps hold on the full roundness level; a model whose solids of all scales would hold more than 300 000 nominal triangles (the grid sizes of the tessellators) is on a lower roundness level: every cap falls in proportion but not under the round floor of 24, the chord grows with the square of the same proportion, and only a model still too heavy with every cap at 24 goes below, to at least 12, so that the triangle and file budgets of the 3D files hold. A scale whose body, thermal or structure list would hold more nominal triangles than the same list one scale finer (markers and rods at the smallest feature grow with the scale) is on the highest level on which none does.", FEATURE)
    text = swap(text, "  Scenario: The catalog shows round candles, valves, heads, limbs and wheels", "  Scenario: A model heavier than the triangle budget trades rounder for lighter\n    Given the budget of the fixture, groves of one to two thousand foliage ellipsoids per scale and light and heavy assets of the catalog\n    When the roundness level of each is asked for\n    Then the level of the model is the highest on which the nominal triangles of all its scales stay within the budget, a scale is on the highest level on which none of its lists grows against the finer scale, the caps and chords are those of the levels, an independent oracle finds the same levels, caps, chords and triangles, and the tessellated triangles stay near the nominal ones\n\n  Scenario: The catalog shows round candles, valves, heads, limbs and wheels", FEATURE)
    return text


if "--stage" in sys.argv:
    out = sys.argv[sys.argv.index("--stage") + 1]
    staged = job(open(JOB, encoding="utf-8").read())
    for old, new in (('"../../../🧬️schema/🟦️.ts"', f'"{S}/🧬️schema/🟦️.ts"'), ('"../../../../../../🧰️framework/', f'"{ROOT}/🧰️framework/'), ('"../../🔗️contracts/🟦️.ts"', f'"{S}/🔨️modules/🔗️contracts/🟦️.ts"'), ('"../🧊️solids/🟦️.ts"', f'"{S}/🔨️modules/🧊️volume/🧊️solids/🟦️.ts"')):
        staged = staged.replace(old, new)
    open(os.path.join(out, "job.ts"), "w", encoding="utf-8").write(staged)
    print("staged", os.path.join(out, "job.ts"))
elif "--check" in sys.argv:
    for path, edit in ((JOB, job), (FIXTURE, fixture), (ORACLE, oracle), (TEST, test), (FEATURE, feature), (SKELETON, skeleton), (SKELETON_TEST, skeleton_test), (SKELETON_FEATURE, skeleton_feature)):
        before = open(path, encoding="utf-8").read()
        print("would change" if edit(before) != before else "unchanged", os.path.relpath(path, ROOT), len(edit(before)) - len(before))
elif "--live" in sys.argv:
    wanted = {"roundness": ((JOB, job), (FIXTURE, fixture), (ORACLE, oracle), (TEST, test), (FEATURE, feature)), "folds": ((SKELETON, skeleton), (SKELETON_TEST, skeleton_test), (SKELETON_FEATURE, skeleton_feature))}
    for path, edit in [pair for name in sys.argv[sys.argv.index("--live") + 1:] for pair in wanted[name]]:
        before = open(path, encoding="utf-8").read()
        after = edit(before)
        if after != before:
            open(path, "w", encoding="utf-8").write(after)
        print("written" if after != before else "unchanged", os.path.relpath(path, ROOT))
