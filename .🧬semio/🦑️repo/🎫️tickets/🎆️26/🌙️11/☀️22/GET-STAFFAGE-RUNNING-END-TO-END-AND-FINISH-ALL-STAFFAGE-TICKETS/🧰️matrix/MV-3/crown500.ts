/** 🌳️ Gives the 1:500 crown of a tree the foliage masses of its own individual, a step of the chain of a plant after its silhouette ops: `bun crown500.ts <id|file.json> [--out file] [--write]`, configured by `crown500/<id>.json` (`target`: the bone the 1:500 crown rides on; `groups`: a regex over the bones of the 1:200 foliage solids; `points`: vertices per outline; `decisions`: the texts of the representation that change; `volume`: the first decision text of the volume).
 *
 * A 1:500 crown built as rings of equal masses round the stem is the same body from ahead and from the side, so its front elevation equals its side elevation (`derived-view-in-band`, cause 5.4 of `multiview-diagnosis.md`). The 1:200 body of the same tree carries the masses of this individual, one group of solids per foliage bone. The run
 * - measures, in the default pose, the box of every such group of the 1:200 body in world axes and sets one ellipsoid per group into `volumes.500.body`, bound to `target`, with the centre and the half extents of that box (so the 1:500 crown has exactly the reach of the 1:200 crown to every side); the former crown ellipsoids of the foliage materials on `target` go, and the thermal references of `volumes.500.thermal` follow (`th-<solid>` per mass, region by material);
 * - draws every mass in the three views as its true outline (the ellipse an ellipsoid is seen as), one filled shape per mass on `target` in the look of the former crown shape, painted in the order of their depth towards the viewer; the former single `crown` outline goes. The coordinates each view is authored in are solved from the engine itself: the shapes are evaluated and written again until every evaluated outline lies on its ellipse;
 * - verifies with the engine (`worldSolids`) that each mass stands where its box is, and prints the body box, the derived-view findings of the suite for 1:500 and the largest distance of a drawn outline from its ellipse.
 * The run is idempotent: it finds its own masses and shapes (`crown-<bone>`) and writes the same ones again.
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { worldSolids, project, bodyDimensions } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volumes", "🌓️silhouette", "🟦️.ts"));

type V = [number, number, number];
type P = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const config = JSON.parse(readFileSync(join(dirname(Bun.main), "crown500", `${asset.id}.json`), "utf8"));
const TARGET: string = config.target;
const GROUPS = new RegExp(config.groups, "u");
const N: number = config.points ?? 32;
const VIEWS = ["elevation", "front", "plan"] as const;
const round = (value: number, digits = 4): number => Number(value.toFixed(digits));
const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
const stateAt = (scale: number): any => evaluate(asset, pose, engine.scaleSelector(scale));
const boxOf = (worlds: readonly any[]): { min: V; max: V } => {
  const [min, max]: [V, V] = [[Infinity, Infinity, Infinity], [-Infinity, -Infinity, -Infinity]];
  for (const world of worlds) for (let i = 0; i < world.positions.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, world.positions[i + k]); max[k] = Math.max(max[k]!, world.positions[i + k]); }
  return { min, max };
};

const fine = asset.volumes["200"].body as any[];
const boneOf = new Map(fine.map((solid) => [solid.id, solid]));
const worlds200 = (worldSolids(asset, stateAt(200).bones, 200) as any[]).filter((world) => world.texture !== true && GROUPS.test(boneOf.get(world.id)?.bone ?? ""));
const groups = new Map<string, any[]>();
for (const world of worlds200) groups.set(boneOf.get(world.id).bone, [...(groups.get(boneOf.get(world.id).bone) ?? []), world]);
if (groups.size === 0) throw new Error(`${asset.id}: no 1:200 solid on a bone matching ${config.groups}`);

const volume = asset.volumes["500"];
const isLeaf = (solid: any): boolean => /leaf/u.test(solid.material ?? "");
const former = (volume.body as any[]).filter((solid) => solid.bone === TARGET && solid.kind === "ellipsoid" && isLeaf(solid));
if (former.length === 0) throw new Error(`${asset.id}: no foliage ellipsoid on ${TARGET} at 1:500`);
const like = (material: string): any => former.find((solid) => solid.material === material) ?? former[0];
const frame = stateAt(500).bones[TARGET].frame;
const vec = (p: { x: number; y: number; z: number }): V => [p.x, p.y, p.z];
const [o, u, v, w] = [vec(frame.o), vec(frame.u), vec(frame.v), vec(frame.w)];
const dot = (a: V, b: V): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const quaternion = (m: V[]): [number, number, number, number] => {
  const [m00, m01, m02, m10, m11, m12, m20, m21, m22] = [m[0]![0], m[0]![1], m[0]![2], m[1]![0], m[1]![1], m[1]![2], m[2]![0], m[2]![1], m[2]![2]];
  const trace = m00 + m11 + m22;
  if (trace > 0) { const s = Math.sqrt(trace + 1) * 2; return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]; }
  if (m00 > m11 && m00 > m22) { const s = Math.sqrt(1 + m00 - m11 - m22) * 2; return [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]; }
  if (m11 > m22) { const s = Math.sqrt(1 + m11 - m00 - m22) * 2; return [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]; }
  const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
  return [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s];
};
const masses = [...groups].map(([bone, worlds]) => {
  const box = boxOf(worlds);
  const centre: V = [(box.min[0] + box.max[0]) / 2, (box.min[1] + box.max[1]) / 2, (box.min[2] + box.max[2]) / 2];
  const radii: V = [(box.max[0] - box.min[0]) / 2, (box.max[1] - box.min[1]) / 2, (box.max[2] - box.min[2]) / 2];
  const materials = worlds.map((world) => boneOf.get(world.id).material as string).filter((material) => /leaf/u.test(material));
  const material = materials.sort((a, b) => materials.filter((m) => m === b).length - materials.filter((m) => m === a).length)[0] ?? former[0].material;
  return { id: `crown-${bone}`, bone, centre, radii, material };
});

/** 🧭️ The placement of a world-axis ellipsoid on the target bone: the engine's own convention is found by trying the candidates and measuring the mesh it builds. */
const placements = (mass: (typeof masses)[number]): any[] => {
  const d: V = [mass.centre[0] - o[0], mass.centre[1] - o[1], mass.centre[2] - o[2]];
  const out: any[] = [];
  for (const sign of [1, -1]) {
    const ws: V = [w[0] * sign, w[1] * sign, w[2] * sign];
    const rows: V[] = [[u[0], u[1], u[2]], [v[0], v[1], v[2]], ws];
    const columns: V[] = [[u[0], v[0], ws[0]], [u[1], v[1], ws[1]], [u[2], v[2], ws[2]]];
    for (const matrix of [rows, columns]) {
      const q = quaternion(matrix);
      for (const conj of [1, -1]) out.push({ translation: [round(dot(d, u), 5), round(dot(d, v), 5), round(sign * dot(d, w), 5)], rotation: [round(conj * q[0], 5), round(conj * q[1], 5), round(conj * q[2], 5), round(q[3], 5)] });
    }
  }
  return out;
};
const solidOf = (mass: (typeof masses)[number], transform: any): any => {
  const model = like(mass.material);
  return { id: mass.id, kind: "ellipsoid", bone: TARGET, material: mass.material, role: model.role, tone: model.tone, radii: mass.radii.map((r) => round(r, 5)), transform };
};
const kept = (volume.body as any[]).filter((solid) => !former.includes(solid));
const firstAt = (volume.body as any[]).indexOf(former[0]);
const errorOf = (mass: (typeof masses)[number], transform: any): number => {
  volume.body = [...kept, solidOf(mass, transform)];
  const world = (worldSolids(asset, stateAt(500).bones, 500) as any[]).find((candidate) => candidate.id === mass.id);
  const box = boxOf([world]);
  return Math.max(...[0, 1, 2].flatMap((k) => [Math.abs(box.min[k]! - (mass.centre[k]! - mass.radii[k]!)), Math.abs(box.max[k]! - (mass.centre[k]! + mass.radii[k]!))]));
};
const probe = masses.reduce((best, mass) => (Math.max(...mass.radii) / Math.min(...mass.radii) > Math.max(...best.radii) / Math.min(...best.radii) ? mass : best));
const errors = placements(probe).map((transform) => errorOf(probe, transform));
const choice = errors.indexOf(Math.min(...errors));
console.log(`[DEBUG] ${asset.id}: placement candidates on ${TARGET}, box error in mm: ${errors.map((e) => (e * 1000).toFixed(0)).join(" ")}; taken ${choice}`);
const solids = masses.map((mass) => solidOf(mass, placements(mass)[choice]));
volume.body = [...kept.slice(0, firstAt), ...solids, ...kept.slice(firstAt)];
const worst = Math.max(...masses.map((mass) => { const box = boxOf([(worldSolids(asset, stateAt(500).bones, 500) as any[]).find((candidate) => candidate.id === mass.id)]); return Math.max(...[0, 1, 2].flatMap((k) => [Math.abs(box.min[k]! - (mass.centre[k]! - mass.radii[k]!)), Math.abs(box.max[k]! - (mass.centre[k]! + mass.radii[k]!))])); }));
const SAG = 0.015 * Math.max(...masses.flatMap((mass) => mass.radii)) + 0.005;
console.log(`[DEBUG] ${asset.id}: the meshes of the masses stay ${(worst * 1000).toFixed(0)} mm inside their boxes (the facets of the engine's ellipsoid; allowed ${(SAG * 1000).toFixed(0)} mm)`);
if (worst > SAG) throw new Error(`${asset.id}: a mass stands ${(worst * 1000).toFixed(0)} mm off its box; no placement convention fits`);

const gone = new Set(former.map((solid) => solid.id));
const regionOf = (material: string): string => (volume.thermal as any[]).find((entry) => gone.has(entry.of) && like(material).id === entry.of)?.region ?? (/shade/u.test(material) ? "leaf-shade" : "leaf-sun");
const thermalAt = (volume.thermal as any[]).findIndex((entry) => gone.has(entry.of));
const references = solids.map((solid) => ({ id: `th-${solid.id}`, of: solid.id, region: regionOf(solid.material) }));
const others = (volume.thermal as any[]).filter((entry) => !gone.has(entry.of));
volume.thermal = thermalAt < 0 ? others : [...others.slice(0, thermalAt), ...references, ...others.slice(thermalAt)];
const dangling = [...gone].filter((id) => !solids.some((solid) => solid.id === id) && JSON.stringify({ ...next.asset, volumes: { "500": { ...volume, body: [], thermal: [] } }, representations: null }).includes(`"th-${id}"`));
if (dangling.length > 0) console.log(`[DEBUG] ${asset.id}: thermal ids still named elsewhere: ${dangling.map((id) => `th-${id}`).join(", ")}`);

const rep = asset.representations["500"];
const linear = (view: string, a: V): P => { const p = project(view, a[0], a[1], a[2]); return [p.x, p.y]; };
const depth = (view: string, c: V): number => (view === "elevation" ? -c[2] : view === "front" ? c[0] : c[1]);
const targets = new Map<string, P[]>();
for (const view of VIEWS) {
  for (const mass of masses) {
    const centre = linear(view, mass.centre);
    const axes = ([[mass.radii[0], 0, 0], [0, mass.radii[1], 0], [0, 0, mass.radii[2]]] as V[]).map((axis) => linear(view, axis)).filter((p) => Math.hypot(p[0], p[1]) > 1e-9);
    if (axes.length !== 2) throw new Error(`${asset.id}: ${view} does not see two axes of a mass`);
    targets.set(`${view}/${mass.id}`, Array.from({ length: N }, (_, i): P => { const t = (2 * Math.PI * i) / N; return [centre[0] + Math.cos(t) * axes[0]![0] + Math.sin(t) * axes[1]![0], centre[1] + Math.cos(t) * axes[0]![1] + Math.sin(t) * axes[1]![1]]; }));
  }
  const list = rep.geometry[view] as any[];
  const old = list.find((shape) => shape.id === "crown") ?? list.find((shape) => /^crown-/u.test(shape.id));
  if (old === undefined) throw new Error(`${asset.id} 1:500 ${view}: no crown shape`);
  const at = list.findIndex((shape) => shape.id === "crown" || /^crown-/u.test(shape.id));
  const { kind: _kind, d: _d, points: _points, closed: _closed, id: _id, z: _z, tone: _tone, material: _material, ...look } = old;
  const base = Math.floor(Math.min(...list.filter((shape) => shape.id === "crown" || /^crown-/u.test(shape.id)).map((shape) => shape.z)) * 100) / 100;
  const ordered = [...masses].sort((a, b) => depth(view, a.centre) - depth(view, b.centre) || a.id.localeCompare(b.id));
  const shapes = ordered.map((mass, rank) => ({ id: mass.id, ...look, z: round(base + rank * 0.001, 3), tone: like(mass.material).tone, material: mass.material, kind: "poly", closed: true, points: targets.get(`${view}/${mass.id}`)!.map((p) => [round(p[0]), round(p[1])]) }));
  const rest = list.filter((shape) => shape.id !== "crown" && !/^crown-/u.test(shape.id));
  const before = list.slice(0, at).filter((shape) => rest.includes(shape)).length;
  rep.geometry[view] = [...rest.slice(0, before), ...shapes, ...rest.slice(before)];
}
/** 📐️ The affine map `a -> m · a + t` that takes the points `from` onto the points `to` (least squares). */
const affine = (from: P[], to: P[]): { m: [number, number, number, number]; t: P } => {
  const n = from.length;
  const mean = (points: P[]): P => [points.reduce((s, p) => s + p[0], 0) / n, points.reduce((s, p) => s + p[1], 0) / n];
  const [fm, tm] = [mean(from), mean(to)];
  let [sxx, sxy, syy, ax, ay, bx, by] = [0, 0, 0, 0, 0, 0, 0];
  for (let i = 0; i < n; i++) {
    const [x, y] = [from[i]![0] - fm[0], from[i]![1] - fm[1]];
    const [p, q] = [to[i]![0] - tm[0], to[i]![1] - tm[1]];
    sxx += x * x; sxy += x * y; syy += y * y; ax += p * x; ay += p * y; bx += q * x; by += q * y;
  }
  const det = sxx * syy - sxy * sxy;
  const m: [number, number, number, number] = [(ax * syy - ay * sxy) / det, (ay * sxx - ax * sxy) / det, (bx * syy - by * sxy) / det, (by * sxx - bx * sxy) / det];
  return { m, t: [tm[0] - m[0] * fm[0] - m[1] * fm[1], tm[1] - m[2] * fm[0] - m[3] * fm[1]] };
};
let off = Infinity;
for (let pass = 0; pass < 4 && off > 0.0005; pass++) {
  const state = stateAt(500);
  off = 0;
  for (const view of VIEWS)
    for (const shape of (rep.geometry[view] as any[]).filter((candidate) => /^crown-/u.test(candidate.id))) {
      const evaluated = (state.shapes[view] as any[]).find((candidate) => candidate.shape.id === shape.id && candidate.shape.bone === TARGET);
      const ring: P[] = evaluated.rings[0].slice(0, N).map((p: any) => [p.x, p.y]);
      if (evaluated.rings[0].length < N) throw new Error(`${asset.id} 1:500 ${view} ${shape.id}: the engine returns ${evaluated.rings[0].length} of ${N} points`);
      const target = targets.get(`${view}/${shape.id}`)!;
      const here = Math.max(...ring.map((p, i) => Math.hypot(p[0] - target[i]![0], p[1] - target[i]![1])));
      off = Math.max(off, here);
      if (here <= 0.0005) continue;
      const { m, t } = affine(ring, shape.points as P[]);
      shape.points = target.map((p) => [round(m[0] * p[0] + m[1] * p[1] + t[0]), round(m[2] * p[0] + m[3] * p[1] + t[1])]);
    }
  console.log(`[DEBUG] ${asset.id}: pass ${pass}, largest distance of a drawn outline from its ellipse ${(off * 1000).toFixed(1)} mm`);
}
if (off > 0.002) throw new Error(`${asset.id}: the outlines stay ${(off * 1000).toFixed(1)} mm off their ellipses`);

for (const [key, value] of Object.entries(config.decisions ?? {})) rep.decisions[key] = value;
if (config.volume !== undefined) volume.decisions[0] = config.volume;
const dims = bodyDimensions((worldSolids(asset, stateAt(500).bones, 500) as any[]).filter((world) => world.texture !== true));
const fineDims = bodyDimensions((worldSolids(asset, stateAt(200).bones, 200) as any[]).filter((world) => world.texture !== true));
console.log(`${asset.id}: body 1:500 ${dims.map((n: number) => n.toFixed(4)).join(" × ")} m, body 1:200 ${fineDims.map((n: number) => n.toFixed(4)).join(" × ")} m, declared ${asset.dimensions.widthM} × ${asset.dimensions.heightM} × ${asset.dimensions.depthM} m`);
for (const mass of masses) console.log(`   ${mass.id} ${mass.material} centre ${mass.centre.map((n) => n.toFixed(2)).join(", ")} radii ${mass.radii.map((n) => n.toFixed(2)).join(", ")}`);
const derived = engine.derivedFindings(asset, "500", 500);
console.log(`${asset.id}: derived-view findings at 1:500: ${derived.length === 0 ? "none" : derived.map((finding: any) => finding.message).join("; ")}`);
const written = rewrite(text, previous, next);
console.log(`${asset.id}: ${masses.length} masses on ${TARGET} (${former.length} before), ${references.length} thermal references; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || (out !== undefined && !existsSync(out))) writeFileSync(out ?? file, written);
}
