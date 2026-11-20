/** ✏️ Redraws the 2D drawings of `humans/pregnant-person` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. One id per part in all three views (the old elevation ids), with the tone, hatch, region and role the drawing had; which solids one shape unites, the layer order of every view and the strokes (boot zips, mouth) are stated by hand below. Arm and hand shapes are solids authored in their bone frame (prisms, sleeves of revolution in plan), so every pose and clip still reads.
 *
 * Usage: `bun r3-pregnant-person-draw2d.ts [--only 50]` writes `🗑️generated/r3-pregnant-person/shapes.json` (spliced by `r3-pregnant-person-roots.ts`).
 * @see ./r3-pregnant-person-build.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { OUT, SOURCE, candidate } from "./r3-pregnant-person-io.ts";

type V3 = [number, number, number];
const asset = candidate(false);
const ORIGINAL = JSON.parse(readFileSync(`${OUT}/original.json`, "utf8")).asset as Json;
const VOLUMES = JSON.parse(readFileSync(SOURCE, "utf8")).volumes as Json;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not a filled shape). */
function groupOf(id: string, view: View, scale: number): string | null {
  if (/^boot-zip/.test(id)) return null;
  if (/^boot-(vamp|rim)-/.test(id)) return id.replace(/vamp-|rim-/, "");
  if (/^boot-heel-/.test(id)) return id.replace("heel", "sole");
  if (/^knee-/.test(id)) return id.replace("knee", "shank");
  if (/^hand-[lr]/.test(id)) return id.slice(0, 6);
  if (id === "leggings-seat") return "hips";
  if (/^(lips|chin)$/.test(id)) return view === "front" && scale === 50 && id === "lips" ? "lips" : "head";
  if (/^(nose|brow-[lr]|eye-[lr])$/.test(id)) return view === "front" && scale <= 100 ? id : "head";
  if (/^ear-[lr]$/.test(id)) return view === "elevation" && id === "ear-l" ? "head" : id;
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^boot-toe-sole-l/, /^boot-sole-l/, /^boot-l$/, /^boot-toe-l$/, /^shank-l/, /^thigh-l/, /^hand-l/, /^cuff-rib-l/, /^forearm-l/, /^sleeve-l/, /^hips/, /^sweater$/, /^sweater-hem-rib/, /^belly-cover/, /^neck/, /^sweater-collar/, /^head$/, /^ear-r/, /^hair/, /^boot-toe-sole-r/, /^boot-sole-r/, /^boot-r$/, /^boot-toe-r$/, /^shank-r/, /^thigh-r/, /^sleeve-r/, /^forearm-r/, /^cuff-rib-r/, /^hand-r/],
  front: [/^hair/, /^hips/, /^thigh-/, /^shank-/, /^boot-[lr]$/, /^boot-sole-/, /^boot-toe-[lr]$/, /^boot-toe-sole-/, /^hand-r/, /^cuff-rib-r/, /^forearm-r/, /^sleeve-r/, /^sweater$/, /^sweater-hem-rib/, /^belly-cover/, /^sleeve-l/, /^forearm-l/, /^cuff-rib-l/, /^hand-l/, /^neck/, /^sweater-collar/, /^ear-/, /^head$/, /^nose/, /^lips/, /^eye-/, /^brow-/],
  plan: [/^boot-toe-sole-/, /^boot-sole-/, /^boot-[lr]$/, /^boot-toe-[lr]$/, /^shank-/, /^thigh-/, /^hips/, /^hand-r/, /^cuff-rib-r/, /^forearm-r/, /^hand-l/, /^cuff-rib-l/, /^forearm-l/, /^belly-cover/, /^sweater$/, /^sweater-hem-rib/, /^sleeve-/, /^neck/, /^sweater-collar/, /^ear-/, /^head$/, /^nose/, /^hair/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
/** 🎨 The drawing's own look of a part (tone, hatch, region, role), taken from the old drawing of that scale under its old id in any view. */
const ALIAS: Record<string, string[]> = { sweater: ["sweater", "torso"], "belly-cover": ["belly-cover", "belly"], hips: ["hips"], head: ["head"], hair: ["hair"], neck: ["neck"] };
function look(scale: number, id: string): Json {
  const reps = ORIGINAL.representations[String(scale)].geometry as Record<string, Json[]>;
  const side = id.match(/-(l|r)$/)?.[1];
  const base = id.replace(/-(l|r)$/, "");
  const names = ALIAS[id] ?? (side ? [id, `${{ "boot": "shoe", "boot-toe": "shoe-toe", sleeve: "upper-arm", "cuff-rib": "cuff", "boot-sole": "boot-sole" }[base] ?? base}-${side}`, `sil-${base}-${side}`] : [id]);
  for (const v of ["elevation", "front", "plan"]) for (const n of names) { const s = reps[v]?.find((q: Json) => q.id === n && q.filled); if (s) return { tone: s.tone, ...(s.hatch ? { hatch: s.hatch } : {}) }; }
  return {};
}
const REGION: Record<string, string> = { "knit-wool-sweater": "sweater", "stretch-leggings": "leggings", "leather-ankle-boot": "ankle-boots", hair: "hair" };
function regionOf(s: Json, group: string): string {
  if (group === "belly-cover") return "belly-sweater";
  if (/^brow-/.test(group)) return "face-skin";
  if (s.material === "skin") return /^hand-/.test(group) ? "hand-skin" : "face-skin";
  const r = REGION[s.material];
  if (!r) throw new Error(`no region for ${s.id}`);
  return r;
}
const ROLE = (group: string): string => (/^(sweater-hem-rib|sweater-collar|cuff-rib-|knee-|eye-|brow-|lips|nose|boot-sole-|boot-toe-sole-)/.test(group) ? "detail" : "body");
/** 🖊️ Filled parts inside the outline of their own bone: the line views draw only depth steps, so they also get their outline as a detail stroke. */
const LINES = /^(sweater-hem-rib|sweater-collar|belly-cover|eye-[lr]|brow-[lr]|lips|nose|cuff-rib-[lr]|boot-sole-[lr]|boot-toe-sole-[lr])$/;
const TOL: Record<number, number> = { 50: 0.0008, 100: 0.0016, 200: 0.0032, 500: 0.0065 };

const HINTED = /^(arm-|forearm-|hand-)/;
const ROUND = /^(arm-|forearm-)/;
/** 📍 The lateral position of a bone's origin in the rest pose: `z0`/`z1` of a w-swept solid are measured from there (rest-world lateral, left +). */
const REST_Z = (bone: string): number => { const b = (asset.rig.bones as Json[]).find((q) => q.id === bone); return b.z ?? (b.parent ? REST_Z(b.parent) : 0); };
const DEFAULT = rigAt(asset, 50, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
const VIEW_AXIS: Record<View, number> = { elevation: 2, front: 0, plan: 1 };
const IDENT: Record<string, Frame> = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] } as Frame]));
/** 📐 The outline and sweep of an arm or hand shape: the bone-local meshes projected along w or v, or the radial profile about u (sleeves in plan). */
function hinted(view: View, bone: string, solids: Json[], cell: number, tol: number): { points: P2[]; solid: Json } | null {
  const local = meshesIn(asset, IDENT, solids);
  const f = DEFAULT[bone]!;
  const k = 2;
  void f;
  let lo = Infinity;
  let hi = -Infinity;
  for (const m of local) for (let i = 0; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i + k]!); hi = Math.max(hi, m.world[i + k]!); }
  if (ROUND.test(bone)) {
    const rows = new Map<number, number>();
    const step = tol * 2;
    const flat = /^hand-/.test(bone);
    let wMax = 0;
    let vMax = 0;
    for (const m of local) for (let i = 0; i < m.world.length; i += 3) { wMax = Math.max(wMax, Math.abs(m.world[i + 2]!)); vMax = Math.max(vMax, Math.abs(m.world[i + 1]!)); }
    const ls = flat ? Math.min(1, wMax / vMax) : 1;
    for (const m of local) for (let i = 0; i < m.world.length; i += 3) { const u = Math.round(m.world[i]! / step); const r = flat ? Math.max(Math.abs(m.world[i + 1]!), Math.abs(m.world[i + 2]!) / ls) : Math.hypot(m.world[i + 1]!, m.world[i + 2]!); rows.set(u, Math.max(rows.get(u) ?? 0, r)); }
    const us = [...rows.keys()].sort((a, b) => a - b);
    const outer = us.map((u) => ({ x: u * step, y: rows.get(u)! }));
    const ring: P2[] = [{ x: outer[0]!.x, y: 0 }, ...outer, { x: outer[outer.length - 1]!.x, y: 0 }];
    return { points: simplifyRing(ring, tol), solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: 0, mirror: true, ...(flat ? { lateralScale: Math.round(ls * 1000) / 1000 } : {}) } };
  }
  const flat = k === 2 ? local : local.map((m) => { const w = new Float64Array(m.world.length); for (let i = 0; i < w.length; i += 3) { w[i] = m.world[i]!; w[i + 1] = m.world[i + 2]!; w[i + 2] = 0; } return { ...m, world: w }; });
  const mask = projectMask("elevation", flat, cell, 4 + Math.ceil(0.03 / cell));
  if (mask === null) return null;
  const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
  return { points: simplifyRing(rings[0]!, tol), solid: { kind: "extruded", sweep: "w", z0: r4(lo + REST_Z(bone)), z1: r4(hi + REST_Z(bone)) } };
}

const geometry: Record<string, Record<string, Json[]>> = {};
const ONLY = process.argv.includes("--only") ? [Number(process.argv[process.argv.indexOf("--only") + 1])] : [50, 100, 200, 500];
for (const scale of ONLY) {
  const work = JSON.parse(JSON.stringify(asset)) as Json;
  const frames: Record<string, Frame> = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const world = (bone: string, p: readonly number[]): V3 => { const f = frames[bone]!; return [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + p[2]! * f.w[k]!) as V3; };
  const body: Json[] = VOLUMES[String(scale)].body.filter((s: Json) => !s.texture);
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
      const meta: Json = { id, bone: own.bone, role: ROLE(id), z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own, id), ...look(scale, id) };
      if (HINTED.test(own.bone)) {
        const h = hinted(view, own.bone, solids, cell, tol);
        if (h !== null) direct.push({ ...meta, kind: "poly", points: h.points.map((p) => [r4(p.x), r4(p.y)]), closed: true, solid: h.solid });
        if (h !== null && LINES.test(id) && scale !== 500) direct.push({ ...meta, id: `${id}-line`, role: "detail", z: r4(meta.z + 0.05), filled: false, tone: 0.2, kind: "poly", points: h.points.map((p) => [r4(p.x), r4(p.y)]), closed: true, solid: h.solid, hatch: undefined });
        continue;
      }
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      jobs.push({ meta, target: ring, closed: true });
      if (LINES.test(id) && scale !== 500) { const { hatch: _h, ...plain } = meta; jobs.push({ meta: { ...plain, id: `${id}-line`, role: "detail", z: r4(meta.z + 0.05), filled: false, tone: 0.2 }, target: ring, closed: true }); }
    }
    if (scale <= 100 && view !== "plan") for (const s of VOLUMES[String(scale)].body.filter((q: Json) => /^boot-zip-[lr]$/.test(q.id))) {
      const pts = s.sections.map((q: Json) => projectWorld(view, world(s.bone, q.at)));
      jobs.push({ meta: { id: s.id, bone: s.bone, role: "texture", z: r4(layer(s.id.replace("zip", "toe"), view) + 0.1), filled: false, tone: 0.1, region: "ankle-boots" }, target: pts, closed: false });
    }
    if (view === "front" && scale === 50) {
      const mouth: V3[] = [[-0.024, -0.104, 0.016], [-0.0235, -0.1048, 0], [-0.024, -0.104, -0.016]];
      jobs.push({ meta: { id: "mouth", bone: "head", role: "texture", z: r4(layer("lips", view) + 0.1), filled: false, tone: 0.1, region: "face-skin" }, target: mouth.map((p) => projectWorld(view, world("head", p))), closed: false });
    }
    const solved = inverse(work, scale, view, jobs);
    const shapes: Json[] = jobs.map((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed };
    });
    shapes.push(...direct.map((d) => JSON.parse(JSON.stringify(d))));
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
