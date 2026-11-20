/** ✏️ Redraws the 2D drawings of `humans/elderly-with-cane` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view and the detail strokes are stated by hand below.
 *
 * Usage: `bun r3-elderly-with-cane-draw2d.ts` writes `🗑️generated/r3-elderly-with-cane/geometry.json` (spliced by `r3-elderly-with-cane-edit.py`).
 * @see ./r3-elderly-with-cane-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, entryOf, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { volumes } from "./r3-elderly-with-cane-build.ts";
import { crookPath, jacketX } from "./r3-elderly-with-cane-s50.ts";

type V3 = [number, number, number];
const { asset } = entryOf("humans/elderly-with-cane");
const VOLUMES = volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(id: string, view: View): string | null {
  if (/^glasses-/.test(id)) return null;
  if (/^hand-l/.test(id)) return "hand-l";
  if (/^hand-r/.test(id)) return "hand-r";
  const sole = /^shoe-(heel|sole)-([lr])$/.exec(id);
  if (sole) return view === "plan" ? `shoe-rear-${sole[2]}` : `shoe-sole-${sole[2]}`;
  const toeSole = /^shoe-toe-sole-([lr])$/.exec(id);
  if (toeSole && view === "plan") return `shoe-toe-${toeSole[1]}`;
  if (/^head-(nose|nostrils|brow|chin|cheek-[lr])$/.test(id)) return "head";
  if (/^cane-(shaft|crook|crook-nose)$/.test(id)) return "cane";
  if (view === "elevation") {
    if (/^(lapel-[lr]|waistcoat|shirt-front|jacket-breast-pocket|jacket-pocket-l|jacket-button-\d)$/.test(id)) return "jacket";
    if (/^(head-ear-l|head-eye-l|hair-temple-l|head-lips)$/.test(id)) return "head";
  }
  if (view === "front" && id === "hair-white") return "head";
  if (view === "plan") {
    if (/^(lapel-[lr]|waistcoat|shirt-front|jacket-breast-pocket|jacket-pocket-[lr]|jacket-button-\d)$/.test(id)) return "jacket";
    if (/^(head-ear-[lr]|head-eye-[lr]|hair-temple-[lr]|head-lips|hair-white)$/.test(id)) return "head";
    const sock = /^sock-([lr])$/.exec(id);
    if (sock) return `shank-${sock[1]}`;
  }
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^sock-l/, /^shoe-sole-l/, /^shoe-toe-sole-l/, /^shoe-rear-l/, /^shoe-toe-l/, /^shoe-l/, /^shank-l/, /^thigh-l/, /^wrist-l/, /^hand-l/, /^shirt-cuff-l/, /^forearm-l/, /^sleeve-l/, /^sock-r/, /^shoe-sole-r/, /^shoe-toe-sole-r/, /^shoe-rear-r/, /^shoe-toe-r/, /^shoe-r/, /^shank-r/, /^thigh-r/, /^pelvis-trousers/, /^neck/, /^shirt-collar/, /^jacket$/, /^collar/, /^jacket-/, /^head$/, /^head-eye/, /^hair-white/, /^hair-temple/, /^head-ear/, /^cap$/, /^cap-brim/, /^cane$/, /^cane-ferrule/, /^wrist-r/, /^shirt-cuff-r/, /^forearm-r/, /^sleeve-r/, /^hand-r/],
  front: [/^pelvis-trousers/, /^thigh-/, /^shank-/, /^sock-/, /^shoe-sole-/, /^shoe-rear-/, /^shoe-toe-sole-/, /^shoe-toe-/, /^shoe-[lr]/, /^jacket$/, /^waistcoat/, /^shirt-front/, /^lapel-/, /^jacket-/, /^sleeve-/, /^forearm-/, /^shirt-cuff-/, /^wrist-/, /^hand-l/, /^neck/, /^shirt-collar/, /^collar/, /^head-ear/, /^head$/, /^head-lips/, /^head-eye/, /^hair-temple/, /^cap$/, /^cap-brim/, /^cane$/, /^cane-ferrule/, /^hand-r/],
  plan: [/^shoe-sole-/, /^shoe-toe-sole-/, /^shoe-rear-/, /^shoe-toe-/, /^shoe-[lr]/, /^sock-/, /^shank-/, /^thigh-/, /^cane-ferrule/, /^cane$/, /^hand-/, /^wrist-/, /^shirt-cuff-/, /^forearm-/, /^pelvis-trousers/, /^jacket$/, /^sleeve-/, /^neck/, /^shirt-collar/, /^collar/, /^head$/, /^cap-brim/, /^cap$/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
function regionOf(s: Json): string {
  if (s.bone === "cane") return "cane-wood";
  if (s.material === "tweed" || s.material === "cotton-flannel" || s.material === "wool-knit") return "jacket";
  if (s.material === "wool-trouser") return "trousers";
  if (s.material === "leather-shoe" || s.material === "wool-sock") return "shoes";
  if (s.material === "cap-tweed") return "cap";
  if (s.material === "hair") return "hair";
  if (s.material === "skin") return /^(hand|wrist)/.test(s.id) ? "hand-skin" : "face-skin";
  throw new Error(`no region for ${s.id}`);
}

const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
/** 🩼 The width the slender cane is drawn with where its true 22 mm would fall below the printable line. */
const CANE_WIDTH: Record<number, number> = { 200: 0.036, 500: 0.065 };

type Stroke = { id: string; bone: string; role: string; after: string; tone: number; region: string; material: string; closed: boolean; points: V3[] };
/** 🖋️ The detail strokes of a scale and view, stated as points of the standing figure (x ahead, y up, z left). */
function strokes(scale: number, view: View, frames: Record<string, Frame>): Stroke[] {
  const at = (bone: string, p: V3): V3 => { const f = frames[bone]!; return [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3; };
  const head = (id: string, points: V3[], closed = false, tone = 0.15): Stroke => ({ id, bone: "head", role: "detail", after: "head-eye", tone, region: "face-skin", material: "skin", closed, points: points.map((p) => at("head", p)) });
  const out: Stroke[] = [];
  const edge: Stroke = { id: "jacket-front-edge", bone: "torso", role: "detail", after: "jacket-", tone: 0.3, region: "jacket", material: "tweed", closed: false, points: [1.125, 1.02, 0.9, 0.768].map((y) => [jacketX(y, 0.004, 1) + 0.002, y, 0.004] as V3) };
  if (scale === 100 && view === "front") out.push(edge);
  if (scale !== 50) return out;
  if (view === "elevation") {
    out.push(head("glasses-temple", [[0.068, -0.094, -0.0555], [0.071, 0.012, -0.0795]]));
    out.push(head("glasses-rim", [[0.041, -0.0965, -0.0315], [0.083, -0.0965, -0.0315]]));
    out.push(head("mouth-line", [[-0.004, -0.09, -0.01], [-0.001, -0.07, -0.03]], false, 0.3));
  }
  if (view === "front") {
    for (const side of [1, -1]) out.push(head(`glasses-rim-${side > 0 ? "l" : "r"}`, Array.from({ length: 14 }, (_, i) => [0.062 + 0.021 * Math.cos((i * Math.PI) / 7), -0.0965, side * 0.0315 + 0.021 * Math.sin((i * Math.PI) / 7)] as V3), true));
    out.push(head("glasses-bar", [[0.0705, -0.0975, -0.0105], [0.0705, -0.0975, 0.0105]]));
    out.push(head("mouth-line", [[-0.005, -0.089, -0.021], [-0.0035, -0.09, 0], [-0.005, -0.089, 0.021]], false, 0.3));
    out.push(head("nose-line", [[0.072, -0.095, 0.007], [0.038, -0.113, 0.0125], [0.03, -0.112, 0.0], [0.031, -0.112, -0.01]], false, 0.4));
    out.push(edge);
  }
  return out;
}

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of [50, 100, 200, 500]) {
  const work = JSON.parse(JSON.stringify(asset)) as Json;
  const frames = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s.id, view);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean }[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      if (own.slender === true) {
        const f = frames.cane!;
        const at = (p: V3): P2 => projectWorld(view, [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3);
        const [a, b] = [at([0, 0, 0]), at([0.734, 0, 0])];
        const h = CANE_WIDTH[scale]! / 2;
        const strip: P2[] = view === "plan" ? [{ x: a.x - 0.011, y: a.y - h }, { x: b.x + 0.011, y: b.y - h }, { x: b.x + 0.011, y: b.y + h }, { x: a.x - 0.011, y: a.y + h }] : [{ x: a.x - h, y: a.y }, { x: b.x - h, y: b.y }, { x: b.x + h, y: b.y }, { x: a.x + h, y: a.y }];
        const meta: Json = { id, bone: own.bone, role: "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
        jobs.push({ meta, target: strip, closed: true });
        if (scale === 200 && view === "elevation") jobs.push({ meta: { ...meta, id: "cane-crook", z: r4(meta.z + 0.05), filled: false, penMm: 0.18 }, target: crookPath(6).map(at), closed: false });
        continue;
      }
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      const meta: Json = { id, bone: own.bone, role: own.role === "detail" ? "detail" : "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      if (scale === 50 && view === "elevation" && id === "jacket") meta.hatch = "tweed-weave";
      jobs.push({ meta, target: ring, closed: true });
    }
    for (const s of strokes(scale, view, frames)) jobs.push({ meta: { id: s.id, bone: s.bone, role: s.role, z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, region: s.region, material: s.material }, target: s.points.map((p) => projectWorld(view, p)), closed: s.closed });
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
const out = join(import.meta.dir, "🗑️generated", "r3-elderly-with-cane");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "geometry.json"), JSON.stringify(geometry));
