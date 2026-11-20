/** ✏️ Redraws the 2D drawings of `humans/walking-commuter` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view and the detail strokes are stated by hand below.
 *
 * Usage: `bun r3-walking-commuter-draw2d.ts` writes `🗑️generated/r3-walking-commuter/shapes.json` (spliced by `r3-walking-commuter-root.py`).
 * @see ./r3-walking-commuter-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { volumes } from "./r3-walking-commuter-build.ts";
import { OUT, candidate } from "./r3-walking-commuter-io.ts";
import { coatX } from "./r3-walking-commuter-s50.ts";

type V3 = [number, number, number];
const asset = candidate(false);
const VOLUMES = await volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const HAIRLINE = 1.75;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(id: string, view: View, scale: number): string | null {
  if (/^sock-/.test(id) || id === "bag-strap") return null;
  if (/^coat-(skirt-[lr]|centre)$/.test(id)) return "coat";
  if (/^hand-l/.test(id)) return "hand-l";
  if (/^hand-r/.test(id)) return scale === 500 ? "forearm-r" : "hand-r";
  const sole = /^shoe-(heel|sole)-([lr])$/.exec(id);
  if (sole) return view === "plan" ? `shoe-rear-${sole[2]}` : `shoe-sole-${sole[2]}`;
  const toeSole = /^shoe-toe-sole-([lr])$/.exec(id);
  if (toeSole) return view === "plan" ? `shoe-toe-${toeSole[1]}` : `shoe-toe-sole-${toeSole[1]}`;
  if (/^head-(nose|brow|chin|lips)$/.test(id)) return "head";
  if (/^bag-lug-/.test(id) || id === "bag-flap-top") return view === "elevation" && id === "bag-flap-top" ? "bag-flap" : "bag-body";
  if (/^coat-button-/.test(id)) return view === "front" ? id : null;
  if (id === "scarf-front") return view === "front" ? id : null;
  if (view === "elevation") {
    if (/^(lapel-[lr]|coat-pocket-l)$/.test(id)) return "coat";
    if (/^(head-ear-l|head-eye-l)$/.test(id)) return "head";
  }
  if (view === "front" && /^bag-(flap|clasp)$/.test(id)) return "bag-body";
  if (view === "plan") {
    if (/^(lapel-[lr]|coat-pocket-[lr])$/.test(id)) return "coat";
    if (/^head-(ear|eye)-[lr]$/.test(id)) return "head";
    if (id === "bag-clasp") return "bag-flap";
  }
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^shoe-sole-l/, /^shoe-toe-sole-l/, /^shoe-rear-l/, /^shoe-toe-l/, /^shoe-l/, /^shank-l/, /^thigh-l/, /^wrist-l/, /^hand-l/, /^shirt-cuff-l/, /^forearm-l/, /^sleeve-l/, /^shoe-sole-r/, /^shoe-toe-sole-r/, /^shoe-rear-r/, /^shoe-toe-r/, /^shoe-r/, /^shank-r/, /^thigh-r/, /^pelvis-trousers/, /^neck/, /^coat$/, /^collar/, /^scarf$/, /^coat-pocket/, /^head$/, /^head-eye/, /^hair$/, /^head-ear/, /^bag-strap/, /^bag-body/, /^bag-flap/, /^bag-clasp/, /^wrist-r/, /^shirt-cuff-r/, /^forearm-r/, /^sleeve-r/, /^hand-r/],
  front: [/^pelvis-trousers/, /^thigh-/, /^shank-/, /^shoe-sole-/, /^shoe-rear-/, /^shoe-toe-sole-/, /^shoe-toe-/, /^shoe-[lr]/, /^bag-body/, /^coat$/, /^collar/, /^scarf-front/, /^lapel-/, /^coat-pocket/, /^coat-button/, /^bag-strap/, /^sleeve-/, /^forearm-/, /^shirt-cuff-/, /^wrist-/, /^hand-/, /^neck/, /^scarf$/, /^hair$/, /^head-ear/, /^head$/, /^hair-fringe/, /^head-eye/],
  plan: [/^shoe-sole-/, /^shoe-toe-sole-/, /^shoe-rear-/, /^shoe-toe-/, /^shoe-[lr]/, /^shank-/, /^thigh-/, /^pelvis-trousers/, /^bag-body/, /^bag-flap/, /^coat$/, /^hand-l/, /^wrist-/, /^shirt-cuff-/, /^forearm-/, /^sleeve-/, /^bag-strap/, /^hand-r/, /^collar/, /^neck/, /^scarf$/, /^head$/, /^hair$/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
function regionOf(s: Json): string {
  if (s.material === "wool-coat" || s.material === "cotton-shirt") return "coat";
  if (s.material === "wool-scarf") return "scarf";
  if (s.material === "wool-trouser") return "trousers";
  if (s.material === "leather-shoe" || s.material === "cotton-sock") return "shoes";
  if (s.material === "leather-bag") return "bag-leather";
  if (s.material === "hair") return "hair";
  if (s.material === "skin") return /^(hand|wrist)/.test(s.id) ? "hand-skin" : "face-skin";
  throw new Error(`no region for ${s.id}`);
}

const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
/** 🎗️ The pen of the strap stroke: the edge of the 6 mm leather in the side view, the true 36 mm of the band in front and plan down to the thinnest printable line. */
function strapPen(scale: number, view: View): number | null {
  if (view === "elevation") return scale === 500 ? 0.13 : scale === 50 ? 0.13 : 0.1;
  return scale === 50 ? 0.7 : scale === 100 ? 0.35 : scale === 200 ? 0.18 : 0.13;
}

type Stroke = { id: string; bone: string; role: string; after: string; tone: number; region: string; material: string; closed: boolean; points: V3[] };
/** 🖋️ The detail strokes of a scale and view, stated as points of the standing figure (x ahead, y up, z left). */
function strokes(scale: number, view: View): Stroke[] {
  const head = (id: string, points: V3[], tone = 0.25): Stroke => ({ id, bone: "head", role: "detail", after: "head-eye", tone, region: "face-skin", material: "skin", closed: false, points });
  const coat = (id: string, points: V3[], after = "coat-pocket"): Stroke => ({ id, bone: "torso", role: "detail", after, tone: 0.12, region: "coat", material: "wool-coat", closed: false, points });
  const out: Stroke[] = [];
  const edge = coat("coat-front-edge", [1.365, 1.2, 1.0, 0.8, 0.64, 0.557].map((y) => [coatX(y, 0.006, 1) + 0.002, y, -0.004] as V3), "coat-button");
  if (scale === 100 && view === "front") out.push(edge);
  if (scale !== 50) return out;
  if (view === "elevation") {
    out.push(head("mouth-line", [[0.112, 1.6255, -0.012], [0.094, 1.6235, -0.03]]));
    out.push(head("brow", [[0.113, 1.716, -0.018], [0.097, 1.72, -0.05]], 0.14));
    out.push(coat("coat-back-vent", [[coatX(0.86, 0.01, -1) + 0.012, 0.86, -0.01], [coatX(0.7, 0.01, -1) + 0.011, 0.7, -0.01], [coatX(0.557, 0.01, -1) + 0.01, 0.557, -0.01]]));
    out.push(coat("coat-side-seam", [[0.0, 1.36, -0.178], [-0.004, 1.12, -0.174], [-0.004, 0.9, -0.198], [-0.002, 0.557, -0.212]]));
  }
  if (view === "front") {
    out.push(head("mouth-line", [[0.111, 1.6245, -0.022], [0.113, 1.6235, 0], [0.111, 1.6245, 0.022]]));
    out.push(head("nose-line", [[0.118, 1.706, 0.006], [0.136, 1.668, 0.0125], [0.132, 1.656, 0.0], [0.132, 1.657, -0.0105]], 0.4));
    for (const side of [1, -1]) out.push(head(`brow-${side > 0 ? "l" : "r"}`, [[0.114, 1.714, side * 0.014], [0.112, 1.718, side * 0.034], [0.104, 1.713, side * 0.05]], 0.14));
    out.push(edge);
  }
  if (view === "plan") out.push({ ...coat("coat-shoulder-seam-l", [[-0.004, 1.5, 0.095], [-0.008, 1.5, 0.218]], "sleeve-"), id: "coat-shoulder-seam-l" }, coat("coat-shoulder-seam-r", [[-0.004, 1.5, -0.095], [-0.008, 1.5, -0.218]], "sleeve-"));
  return out;
}

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of [50, 100, 200, 500]) {
  const work = JSON.parse(JSON.stringify(asset)) as Json;
  const frames: Record<string, Frame> = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const world = (bone: string, p: readonly number[]): V3 => { const f = frames[bone]!; return [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + p[2]! * f.w[k]!) as V3; };
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const hatched = (work.representations[String(scale)].hatches ?? []).some((h: Json) => h.id === "wool");
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s.id, view, scale);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    if (view === "front" && scale !== 500) {
      const hair = body.find((s) => s.id === "hair")!;
      const above = hair.sections.filter((q: Json) => world("head", q.at)[1] >= HAIRLINE);
      if (above.length >= 2) groups.set("hair-fringe", [{ ...hair, id: "hair-fringe", sections: above }]);
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean }[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      const meta: Json = { id, bone: own.bone, role: own.role === "detail" ? "detail" : "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      if (hatched && own.material === "wool-coat" && meta.role === "body") meta.hatch = "wool";
      jobs.push({ meta, target: ring, closed: true });
    }
    const strap = body.find((s) => s.id === "bag-strap");
    if (strap) {
      const pen = strapPen(scale, view);
      const meta: Json = { id: "bag-strap", bone: "bag", role: "detail", z: layer("bag-strap", view), filled: pen === null, tone: strap.tone, region: "bag-leather", material: strap.material };
      if (pen === null) {
        const mask = projectMask(view, meshesIn(work, frames, [strap]), cell, 4 + Math.ceil(0.03 / cell))!;
        const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
        jobs.push({ meta, target: simplifyRing(rings[0]!, tol), closed: true });
      } else jobs.push({ meta: { ...meta, penMm: pen }, target: strap.sections.map((q: Json) => projectWorld(view, world("bag", q.at))), closed: false });
    }
    for (const s of strokes(scale, view)) jobs.push({ meta: { id: s.id, bone: s.bone, role: s.role, z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, region: s.region, material: s.material }, target: s.points.map((p) => projectWorld(view, p)), closed: s.closed });
    const solved = inverse(work, scale, view, jobs);
    const shapes = jobs.map((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed };
    });
    shapes.sort((a, b) => a.z - b.z);
    views[view] = shapes;
    work.representations[String(scale)].geometry[view] = shapes;
    console.log(scale, view, `${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
  geometry[String(scale)] = views;
}
mkdirSync(OUT, { recursive: true });
writeFileSync(`${OUT}/shapes.json`, JSON.stringify(geometry));
