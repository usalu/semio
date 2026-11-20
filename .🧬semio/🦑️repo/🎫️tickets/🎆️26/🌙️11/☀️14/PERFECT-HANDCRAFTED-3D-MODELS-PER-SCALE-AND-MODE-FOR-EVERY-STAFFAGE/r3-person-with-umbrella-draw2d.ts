/** ✏️ Redraws the 2D drawings of `humans/person-with-umbrella` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view and the few detail strokes are stated by hand below; the canopy is drawn as its front and its rear half on the two ribs that stay in the median plane, so it folds with them in every pose; in the front view both halves are revolved about the shaft from their elevation profile, so they narrow as the umbrella furls (the plan keeps the true octagon of the default pose).
 *
 * Usage: `bun r3-person-with-umbrella-draw2d.ts` writes `🗑️generated/r3-person-with-umbrella/geometry.json` (spliced by `r3-person-with-umbrella-apply.py`).
 * @see ./r3-person-with-umbrella-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { ASSET } from "./r3-person-with-umbrella-kit.ts";
import { AZIMUTH } from "./r3-person-with-umbrella-roots.ts";
import { volumes } from "./r3-person-with-umbrella-build.ts";
import { COAT_SKIRT, COAT_UPPER, surfaceX } from "./r3-person-with-umbrella-s50.ts";

type V3 = [number, number, number];
const VOLUMES = await volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** ✂️ The part of an outline on one side of the vertical line at `x0` (`side` +1: ahead of it, -1: behind it), by Sutherland-Hodgman: at 1:500 the two canopy halves meet on the shaft line. */
function clipAhead<T extends { x: number; y: number }>(ring: readonly T[], x0: number, side: number): T[] {
  const inside = (p: T): boolean => side * (p.x - x0) >= 0;
  const out: T[] = [];
  ring.forEach((b, i) => {
    const a = ring[(i + ring.length - 1) % ring.length]!;
    if (inside(a) !== inside(b)) out.push({ ...b, x: x0, y: a.y + ((b.y - a.y) * (x0 - a.x)) / (b.x - a.x) });
    if (inside(b)) out.push(b);
  });
  return out;
}

/** 🪭 Whether a canopy solid is one of the two side gores, which at 1:500 close the rear half of the canopy as well as the front half in the side and front views (the rear half alone is a sliver under the print feature). */
const sideGore = (id: string): boolean => { const gore = /^canopy-(.+?)$/.exec(id); return gore !== null && AZIMUTH[gore[1]!] !== undefined && Math.abs(Math.cos((AZIMUTH[gore[1]!]! * Math.PI) / 180)) < 1e-9; };

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(id: string, view: View, scale: number): string | null {
  const gore = /^canopy-(.+?)(?:-([ab]))?$/.exec(id);
  if (gore) {
    const centre = AZIMUTH[gore[1]!]! + (gore[2] === "a" ? 22.5 : gore[2] === "b" ? -22.5 : 0);
    return Math.cos((centre * Math.PI) / 180) >= -1e-9 ? "canopy-front" : "canopy-back";
  }
  const tip = /^rib-tip-(.+)$/.exec(id);
  if (tip) return Math.cos((AZIMUTH[tip[1]!]! * Math.PI) / 180) >= -1e-9 ? "canopy-front" : "canopy-back";
  if (/^(rib-|stretcher-|runner$|shaft$)/.test(id)) return id === "shaft" && view !== "plan" ? "shaft" : null;
  if (/^(ferrule|notch)$/.test(id)) return "ferrule";
  if (/^handle/.test(id)) return "handle";
  const hand = /^hand-([lr])/.exec(id);
  if (hand) return `hand-${hand[1]}`;
  const shoe = /^shoe-(?:sole|heel)-([lr])$/.exec(id);
  if (shoe) return `shoe-${shoe[1]}`;
  const toe = /^shoe-toe-sole-([lr])$/.exec(id);
  if (toe) return `shoe-toe-${toe[1]}`;
  const strap = /^sleeve-strap-([lr])$/.exec(id);
  if (strap) return `forearm-${strap[1]}`;
  if (id === "hair-bun") return view === "front" ? "hair-bun" : "hair";
  if (/^head-ear-r$/.test(id) && view === "elevation" && scale <= 100) return "head-ear-r";
  if (/^head-eye-[lr]$/.test(id) && view === "front" && scale === 50) return id;
  if (/^head-/.test(id)) return "head";
  if (/^coat-lapel-[lr]$/.test(id) && view === "front") return id;
  if (/^coat-pocket-[lr]$/.test(id) && view === "front" && scale === 50) return id;
  if (/^(coat-yoke|coat-placket|coat-button-[123]|coat-lapel-[lr])$/.test(id)) return "coat";
  if (/^(coat-placket-skirt|coat-button-4|coat-pocket-[lr]|coat-fold-.*)$/.test(id)) return "coat-skirt";
  if (id === "coat-buckle") return "coat-belt";
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^shank-l/, /^shoe-l/, /^shoe-toe-l/, /^thigh-l/, /^hand-l/, /^shirt-cuff-l/, /^forearm-l/, /^sleeve-l/, /^shank-r/, /^shoe-r/, /^shoe-toe-r/, /^thigh-r/, /^neck/, /^shirt-collar/, /^coat-skirt/, /^coat-belt/, /^coat$/, /^coat-collar/, /^head$/, /^head-ear-r/, /^hair/, /^shaft/, /^canopy-back$/, /^canopy-front$/, /^ferrule/, /^handle/, /^sleeve-r/, /^forearm-r/, /^shirt-cuff-r/, /^hand-r/],
  front: [/^hair-bun/, /^thigh-/, /^shank-/, /^shoe-[lr]/, /^shoe-toe-/, /^coat-skirt/, /^coat-pocket-/, /^coat-belt/, /^coat$/, /^coat-lapel-/, /^neck/, /^shirt-collar/, /^coat-collar/, /^head$/, /^head-eye-/, /^hair$/, /^sleeve-l/, /^forearm-l/, /^shirt-cuff-l/, /^hand-l/, /^sleeve-r/, /^shaft/, /^forearm-r/, /^shirt-cuff-r/, /^handle/, /^hand-r/, /^canopy-back$/, /^canopy-front$/, /^ferrule/],
  plan: [/^shoe-[lr]/, /^shoe-toe-/, /^shank-/, /^thigh-/, /^coat-skirt/, /^coat-belt/, /^hand-l/, /^shirt-cuff-l/, /^forearm-l/, /^sleeve-l/, /^coat$/, /^coat-collar/, /^sleeve-r/, /^forearm-r/, /^shirt-cuff-r/, /^handle/, /^hand-r/, /^neck/, /^shirt-collar/, /^head$/, /^hair/, /^canopy-back$/, /^canopy-front$/, /^ferrule/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
function regionOf(s: Json): string {
  if (s.material === "umbrella-fabric") return "canopy";
  if (s.material === "umbrella-metal" || s.material === "umbrella-handle") return "umbrella-frame";
  if (s.material === "rain-shell") return "coat";
  if (s.material === "leather-shoe") return "shoes";
  if (s.material === "hair") return "hair";
  if (s.material === "skin") return /^hand/.test(s.id) ? "hand-skin" : "head-skin";
  if (s.material === "cotton-clothing") return s.id === "shirt-collar" ? "torso-clothing" : /^shirt-cuff/.test(s.id) ? "arm-clothing" : "leg-clothing";
  throw new Error(`no region for ${s.id}`);
}
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
/** 🌂 The width the slender shaft is drawn with: its true 10 mm where that prints, else the thinnest printable strip. */
const SHAFT_W: Record<number, number> = { 50: 0.01, 100: 0.012, 200: 0.024, 500: 0.06 };

type Stroke = { id: string; bone: string; after: string; tone: number; region: string; material: string; points: V3[] };
/** 🖋️ The detail strokes of a scale and view, stated as points of the standing figure (x ahead, y up, z left). */
function strokes(scale: number, view: View): Stroke[] {
  if (scale !== 50 || view !== "front") return [];
  const coat = (id: string, bone: string, t: typeof COAT_UPPER, ys: number[]): Stroke => ({ id, bone, after: bone === "torso" ? "coat-lapel-" : "coat-belt", tone: 0.3, region: "coat", material: "rain-shell", points: ys.map((y) => [surfaceX(t, y, 0.012, 1) + 0.004, y, 0.012] as V3) });
  return [
    coat("coat-placket-line", "torso", COAT_UPPER, [1.388, 1.25, 1.1, 1.048]),
    coat("coat-placket-skirt-line", "pelvis", COAT_SKIRT, [0.99, 0.8, 0.6, 0.54]),
    { id: "mouth-line", bone: "head", after: "head-eye-", tone: 0.3, region: "head-skin", material: "skin", points: [[0.098, 1.5355, 0.019], [0.104, 1.534, 0], [0.098, 1.5355, -0.019]] },
  ];
}

const work = JSON.parse(JSON.stringify(ASSET)) as Json;
const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of [50, 100, 200, 500]) {
  work.volumes = VOLUMES;
  const frames: Record<string, Frame> = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s.id, view, scale);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
      if (g === "canopy-front" && scale === 500 && view !== "plan" && sideGore(s.id)) (groups.get("canopy-back") ?? groups.set("canopy-back", []).get("canopy-back")!).push(s);
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean }[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids.find((s) => /^canopy/.test(s.id) && s.bone === `rib-${id.slice(7)}`) ?? solids[0]!;
      const bone = /^canopy-/.test(id) ? `rib-${id.slice(7)}` : own.bone;
      const meta: Json = { id, bone, role: own.role === "detail" && !/^(ferrule|coat-belt)$/.test(id) ? "detail" : "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      if (id === "shaft") {
        const f = frames.umbrella!;
        const at = (u: number): P2 => projectWorld(view, [0, 1, 2].map((k) => f.o[k]! + u * f.u[k]!) as V3);
        const [a, b] = [at(0.05), at(scale === 500 ? 0.84 : 0.8)];
        const d = { x: b.x - a.x, y: b.y - a.y };
        const n = Math.hypot(d.x, d.y);
        const h = SHAFT_W[scale]! / 2;
        const o = { x: (-d.y / n) * h, y: (d.x / n) * h };
        jobs.push({ meta, target: [{ x: a.x - o.x, y: a.y - o.y }, { x: b.x - o.x, y: b.y - o.y }, { x: b.x + o.x, y: b.y + o.y }, { x: a.x + o.x, y: a.y + o.y }], closed: true });
        continue;
      }
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const half = scale === 500 && view === "elevation" && /^canopy-(front|back)$/.test(id) ? (id === "canopy-front" ? 1 : -1) : 0;
      const outline = half === 0 ? rings[0]! : clipAhead(rings[0]!, projectWorld(view, frames.umbrella!.o as V3).x - half * 0.005, half);
      jobs.push({ meta, target: simplifyRing(outline, tol), closed: true });
    }
    for (const s of strokes(scale, view)) jobs.push({ meta: { id: s.id, bone: s.bone, role: "detail", z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, region: s.region, material: s.material }, target: s.points.map((p) => projectWorld(view, p)), closed: false });
    const solved = inverse(work, scale, view, jobs);
    const shapes = jobs.map((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      const twin = /^canopy-/.test(j.meta.id) && (view === "front" || (view === "plan" && scale === 500)) ? views.elevation!.find((e) => e.id === j.meta.id) : undefined;
      if (twin !== undefined) return { ...j.meta, kind: "poly", points: twin.points, closed: true, solid: { kind: "revolved", axisBone: "umbrella", axis: "u", axisX: 0, axisZ: 0, mirror: true } };
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed };
    });
    shapes.sort((a, b) => a.z - b.z);
    views[view] = shapes;
    work.representations[String(scale)].geometry[view] = shapes;
    console.log(scale, view, `${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
  geometry[String(scale)] = views;
}
const out = join(import.meta.dir, "🗑️generated", "r3-person-with-umbrella");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "geometry.json"), JSON.stringify(geometry));
