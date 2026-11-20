/** ✏️ Redraws the 2D drawings of `humans/leaning-person` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view and the strokes (drawstrings, mouth) are stated by hand below; the wall sample keeps its own shapes, drawn at its true 0.38 m thickness.
 *
 * Usage: `bun r3-leaning-person-draw2d.ts` writes `🗑️generated/r3-leaning-person/shapes.json` (spliced by `r3-leaning-person-roots.ts`).
 * @see ./r3-leaning-person-build.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { OUT, SOURCE, candidate } from "./r3-leaning-person-io.ts";

type V3 = [number, number, number];
const asset = candidate(false);
const VOLUMES = JSON.parse(readFileSync(SOURCE, "utf8")).volumes as Json;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not a filled shape: strokes, the wall, hidden parts). */
function groupOf(id: string, view: View, scale: number): string | null {
  if (/^(wall|hood-string|hood-aglet)/.test(id)) return null;
  if (/^sock-/.test(id)) return view === "plan" ? null : id.replace("sock", "shoe");
  if (/^laces-/.test(id)) return id.replace("laces", "shoe");
  if (/^toe-cap-/.test(id)) return id.replace("toe-cap", "shoe-toe");
  if (/^hand-[lr]/.test(id)) return id.slice(0, 6);
  if (id === "hood-roll") return "hood";
  if (/^(nose|lips|brow|eye-[lr])$/.test(id)) return view === "front" && scale <= 100 ? id : "head";
  if (/^ear-[lr]$/.test(id)) return view === "front" || view === "plan" || id === "ear-r" ? id : "head";
  if (id === "beanie-cuff") return "beanie-rim";
  if (id === "jeans-seat") return "pelvis";
  if (id === "hoodie") return "torso";
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^sole-l$/, /^sole-toe-l/, /^shoe-l$/, /^shoe-toe-l/, /^shank-l/, /^thigh-l/, /^hand-l/, /^wrist-l/, /^cuff-l/, /^forearm-l/, /^upper-arm-l/, /^pelvis/, /^hood$/, /^torso/, /^hoodie-hem/, /^hoodie-pocket/, /^neck/, /^head$/, /^ear-r/, /^hair/, /^beanie$/, /^beanie-rim/, /^sole-r$/, /^sole-toe-r/, /^shoe-r$/, /^shoe-toe-r/, /^shank-r/, /^thigh-r/, /^upper-arm-r/, /^forearm-r/, /^cuff-r/, /^wrist-r/, /^hand-r/, /^phone$/, /^phone-screen/],
  front: [/^hood$/, /^pelvis/, /^thigh-/, /^shank-/, /^shoe-[lr]$/, /^sole-[lr]$/, /^shoe-toe-/, /^sole-toe-/, /^torso/, /^hoodie-hem/, /^hoodie-pocket/, /^upper-arm-/, /^forearm-/, /^cuff-/, /^wrist-/, /^hand-l/, /^phone-screen/, /^phone$/, /^hand-r/, /^neck/, /^ear-/, /^hair/, /^head$/, /^nose/, /^lips/, /^eye-/, /^brow/, /^beanie$/, /^beanie-rim/],
  plan: [/^sole-/, /^sole-toe-/, /^shoe-/, /^shank-/, /^thigh-/, /^pelvis/, /^hand-l/, /^wrist-l/, /^cuff-l/, /^forearm-l/, /^torso/, /^hoodie-hem/, /^hoodie-pocket/, /^hood$/, /^upper-arm-/, /^wrist-r/, /^cuff-r/, /^forearm-r/, /^phone-screen/, /^phone$/, /^hand-r/, /^neck/, /^ear-/, /^head$/, /^hair/, /^beanie$/, /^beanie-rim/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
const REGION: Record<string, string> = { "cotton-clothing": "", skin: "", "leather-shoe": "shoes", hair: "hair", "wool-hat": "hair", "phone-glass": "phone-surface" };
function regionOf(s: Json, group: string): string {
  if (s.material === "skin") return /^(hand)/.test(group) ? "hand-skin" : /^wrist/.test(group) ? "arm-skin" : "head-skin";
  if (s.material === "cotton-clothing") return /^(upper-arm|forearm|cuff)/.test(group) ? "arm-clothing" : /^(thigh|shank|pelvis)/.test(group) ? "leg-clothing" : /^shoe/.test(group) ? "shoes" : "torso-clothing";
  const r = REGION[s.material];
  if (!r) throw new Error(`no region for ${s.id}`);
  return r;
}
const ROLE = (group: string, s: Json): string => (s.role === "glazing" ? "glazing" : /^(hoodie-hem|hoodie-pocket|beanie-rim|cuff-|eye-|brow|lips|nose|sole)/.test(group) ? "detail" : "body");

/** 🖊️ Filled parts that lie inside the outline of their own bone: the line views draw only depth steps, so they also get their outline as a detail stroke. */
const LINES = /^(hoodie-pocket|hoodie-hem|beanie|beanie-rim|eye-[lr]|brow|lips|nose|hood|ear-r)$/;
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
const PEN: Record<number, number> = { 50: 0.18, 100: 0.13, 200: 0.1, 500: 0.1 };

/** 🧱 The wall shapes of a scale and view: the drawn sample at its true 0.38 m depth (elevation and plan) and 1.2 x 1.9 m face (front). */
function wallShapes(scale: number, view: View): Json[] {
  const old = (asset.representations[String(scale)].geometry[view] as Json[]).filter((s) => s.bone === "wall");
  return old.map((s) => {
    const c = JSON.parse(JSON.stringify(s)) as Json;
    if (view === "elevation" && c.id === "wall-surface") return { id: c.id, bone: "wall", role: "ground", z: c.z, filled: false, tone: c.tone, kind: "rect", x: -0.38, y: 0, w: 0.38, h: 1.9, region: c.region, material: c.material };
    if (view === "elevation" && c.kind === "path") c.d = c.d.replace(/M -0\.10 /g, "M -0.38 ");
    if (view === "plan" && c.kind === "rect") Object.assign(c, { x: -0.38, w: 0.38 });
    if (view === "plan" && c.kind === "poly") c.points = c.points.map((p: number[]) => [p[0] === -0.12 ? -0.38 : p[0], p[1]]);
    return c;
  });
}

/** 🦾 Bones that spread or twist in a pose or clip: their shapes are solids (prisms or, for arm segments in plan, solids of revolution) authored in the bone frame, so every view follows the pose. */
const HINTED = /^(upper-arm|forearm|hand)/;
const DEFAULT = rigAt(asset, 50, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
const VIEW_AXIS: Record<View, number> = { elevation: 2, front: 0, plan: 1 };
const IDENT: Record<string, Frame> = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] } as Frame]));
/** 📐 The outline and sweep of a hinted shape: the bone-local meshes projected along w (elevation) or v (front, plan), or the radial profile about u (arm segments in plan). */
function hinted(view: View, scale: number, bone: string, solids: Json[], cell: number, tol: number): { points: P2[]; solid: Json } | null {
  const local = meshesIn(asset, IDENT, solids);
  let lo = Infinity;
  let hi = -Infinity;
  const f = DEFAULT[bone]!;
  const k = /^(upper-arm|forearm)/.test(bone) ? 1 : Math.abs(f.w[VIEW_AXIS[view]]!) >= Math.abs(f.v[VIEW_AXIS[view]]!) ? 2 : 1;
  for (const m of local) for (let i = 0; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i + k]!); hi = Math.max(hi, m.world[i + k]!); }
  if (/^(upper-arm|forearm)/.test(bone)) {
    const rows = new Map<number, number>();
    const step = tol * 2;
    for (const m of local) for (let i = 0; i < m.world.length; i += 3) { const u = Math.round(m.world[i]! / step); const r = Math.hypot(m.world[i + 1]!, m.world[i + 2]!); rows.set(u, Math.max(rows.get(u) ?? 0, r)); }
    const us = [...rows.keys()].sort((a, b) => a - b);
    const outer = us.map((u) => ({ x: u * step, y: rows.get(u)! }));
    const ring: P2[] = [{ x: outer[0]!.x, y: 0 }, ...outer, { x: outer[outer.length - 1]!.x, y: 0 }];
    return { points: simplifyRing(ring, tol), solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: 0, mirror: true } };
  }
  const flat = k === 2 ? local : local.map((m) => { const w = new Float64Array(m.world.length); for (let i = 0; i < w.length; i += 3) { w[i] = m.world[i]!; w[i + 1] = m.world[i + 2]!; w[i + 2] = 0; } return { ...m, world: w }; });
  const mask = projectMask("elevation", flat, cell, 4 + Math.ceil(0.03 / cell));
  if (mask === null) return null;
  const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
  void scale;
  return { points: simplifyRing(rings[0]!, tol), solid: { kind: "extruded", sweep: k === 2 ? "w" : "v", z0: r4(lo), z1: r4(hi) } };
}

const geometry: Record<string, Record<string, Json[]>> = {};
const ONLY = process.argv.includes("--only") ? [Number(process.argv[process.argv.indexOf("--only") + 1])] : [50, 100, 200, 500];
for (const scale of ONLY) {
  const work = JSON.parse(JSON.stringify(asset)) as Json;
  const frames: Record<string, Frame> = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const world = (bone: string, p: readonly number[]): V3 => { const f = frames[bone]!; return [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + p[2]! * f.w[k]!) as V3; };
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s.id, view, scale);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean }[] = [];
    const direct: Json[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids.find((s) => s.role !== "detail") ?? solids[0]!;
      if (HINTED.test(own.bone)) {
        const h = hinted(view, scale, own.bone, solids, cell, tol);
        if (h !== null) direct.push({ id, bone: own.bone, role: ROLE(id, own), z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own, id), material: own.material, kind: "poly", points: h.points.map((p) => [r4(p.x), r4(p.y)]), closed: true, solid: h.solid });
        continue;
      }
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      const meta: Json = { id, bone: own.bone, role: ROLE(id, own), z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own, id), material: own.material };
      jobs.push({ meta, target: ring, closed: true });
      if (LINES.test(id) && scale !== 500) jobs.push({ meta: { ...meta, id: `${id}-line`, role: "detail", z: r4(meta.z + 0.05), filled: false, tone: 0.2 }, target: ring, closed: true });
    }
    for (const s of body.filter((q) => /^hood-string-/.test(q.id))) {
      const end = body.find((q) => q.id === s.id.replace("string", "aglet"));
      const pts = [...s.sections.map((q: Json) => q.at), ...(end ? [end.p1] : [])].map((p: number[]) => projectWorld(view, world(s.bone, p)));
      jobs.push({ meta: { id: s.id, bone: s.bone, role: "detail", z: r4(layer("hoodie-pocket", view) + 0.1), filled: false, tone: s.tone, region: "torso-clothing", material: s.material, penMm: PEN[scale] }, target: pts, closed: false });
    }
    if (view === "front" && scale === 50) {
      const mouth: V3[] = [[0.066, 0.106, 0.019], [0.064, 0.108, 0], [0.066, 0.106, -0.019]];
      jobs.push({ meta: { id: "mouth-line", bone: "head", role: "detail", z: r4(layer("lips", view) + 0.1), filled: false, tone: 0.3, region: "head-skin", material: "skin" }, target: mouth.map((p) => projectWorld(view, world("head", [p[0], -p[1], p[2]]))), closed: false });
    }
    const solved = inverse(work, scale, view, jobs);
    const shapes: Json[] = jobs.map((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed };
    });
    shapes.push(...direct, ...wallShapes(scale, view));
    shapes.sort((a, b) => a.z - b.z);
    views[view] = shapes;
    work.representations[String(scale)].geometry[view] = shapes;
    console.log(scale, view, `${shapes.length} shapes, ${shapes.reduce((n, s) => n + (s.points?.length ?? 0), 0)} points`);
  }
  geometry[String(scale)] = views;
}
mkdirSync(OUT, { recursive: true });
let previous: Json = {};
try { previous = JSON.parse(readFileSync(`${OUT}/shapes.json`, "utf8")); } catch { previous = {}; }
writeFileSync(`${OUT}/shapes.json`, JSON.stringify({ ...previous, ...geometry }));
