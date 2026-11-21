/** ✏️ Redraws the 2D drawings of `humans/person-with-white-cane` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon; every rod (cane shaft, strap, wrist loop, glasses frame) is an open stroke along its centre line with the pen of its true width. Which solids one drawn shape unites, the layer order of every view and the detail strokes are stated by hand below.
 *
 * Usage: `bun r5-person-with-white-cane-draw2d.ts` writes `🗑️generated/r5-person-with-white-cane/geometry.json` (spliced by `r5-person-with-white-cane-apply.py`).
 * @see ./r5-person-with-white-cane-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { volumes } from "./r5-person-with-white-cane-build.ts";
import { OUT, candidate } from "./r5-person-with-white-cane-io.ts";
import { MAT, jacketX } from "./r5-person-with-white-cane-s50.ts";

type V3 = [number, number, number];
const asset = candidate(false);
const VOLUMES = await volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const HAIRLINE = 1.656;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(s: Json, view: View, scale500: boolean): string | null {
  const id = s.id as string;
  if (s.slender) return null;
  if (id === "neck" && scale500) return null;
  if (/^cane-joint-/.test(id)) return null;
  if (id === "cane-hook") return "cane-roller";
  if (/^hand-l/.test(id)) return "hand-l";
  if (/^hand-r/.test(id)) return "hand-r";
  const sole = /^boot-(heel|sole)-([lr])$/.exec(id);
  if (sole) return `boot-${sole[2]}`;
  const toeSole = /^boot-toe-sole-([lr])$/.exec(id);
  if (toeSole) return `boot-toe-${toeSole[1]}`;
  if (/^cuff-/.test(id)) return id.replace("cuff", "forearm");
  if (/^head-(nose|lips|chin)$/.test(id)) return "head";
  if (/^bag-lug-/.test(id) || id === "bag-flap-top") return "bag-body";
  if (/^jacket-button-/.test(id) || id === "shirt-front") return view === "front" ? id : view === "elevation" ? "jacket" : null;
  if (id === "jacket-waistband") return "jacket";
  if (/^jacket-pocket-stud-/.test(id)) return view === "front" ? id.replace("-stud", "") : view === "elevation" ? "jacket" : null;
  if (/^jacket-pocket-/.test(id)) return view === "front" ? id : "jacket";
  if (view === "elevation") {
    if (id === "lens-l") return null;
    if (id === "bag-flap-top") return "bag-flap";
  }
  if (view === "front" && /^bag-(flap|clasp)$/.test(id)) return "bag-body";
  if (view === "plan") {
    if (/^lens-[lr]$/.test(id) || id === "sunglasses") return "head";
    if (id === "bag-clasp") return "bag-flap";
    if (/^boot-shaft-/.test(id)) return null;
  }
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^boot-l/, /^boot-toe-l/, /^boot-shaft-l/, /^shank-l/, /^thigh-l/, /^wrist-l/, /^hand-l/, /^forearm-l/, /^sleeve-l/, /^boot-r/, /^boot-toe-r/, /^boot-shaft-r/, /^shank-r/, /^thigh-r/, /^hips/, /^neck/, /^jacket$/, /^collar/, /^head$/, /^hair$/, /^lens-r/, /^sunglasses/, /^temple-/, /^glasses-front/, /^bag-strap/, /^cane-shaft/, /^cane-band/, /^cane-roller/, /^cane-grip/, /^cane-loop/, /^wrist-r/, /^forearm-r/, /^sleeve-r/, /^hand-r/, /^bag-body/, /^bag-flap/, /^bag-clasp/],
  front: [/^hips/, /^boot-[lr]/, /^boot-shaft-/, /^boot-toe-/, /^shank-/, /^thigh-/, /^bag-body/, /^jacket$/, /^bag-strap/, /^collar/, /^shirt-front/, /^jacket-pocket/, /^jacket-button/, /^sleeve-l/, /^forearm-l/, /^wrist-l/, /^hand-l/, /^neck/, /^hair$/, /^head$/, /^hair-fringe/, /^lens-/, /^sunglasses/, /^temple-/, /^glasses-front/, /^sleeve-r/, /^forearm-r/, /^wrist-r/, /^cane-grip/, /^cane-loop/, /^cane-shaft/, /^cane-band/, /^cane-roller/, /^hand-r/],
  plan: [/^boot-[lr]/, /^boot-toe-/, /^shank-/, /^thigh-/, /^hips/, /^bag-body/, /^bag-flap/, /^cane-shaft/, /^cane-band/, /^cane-roller/, /^jacket$/, /^hand-l/, /^wrist-l/, /^forearm-l/, /^sleeve-l/, /^bag-strap/, /^cane-grip/, /^cane-loop/, /^hand-r/, /^wrist-r/, /^forearm-r/, /^sleeve-r/, /^collar/, /^neck/, /^head$/, /^hair$/, /^temple-/, /^glasses-front/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
function regionOf(s: Json): string {
  if (s.bone === "cane") return s.material === MAT.grip ? "cane-grip" : s.material === MAT.roller || s.id === "cane-band-tip" ? "cane-tip" : "cane-shaft";
  if (s.material === MAT.jacket || s.material === MAT.shirt) return "jacket";
  if (s.material === MAT.jeans) return "jeans";
  if (s.material === MAT.boot) return "boots";
  if (s.material === MAT.bag) return "bag";
  if (s.material === MAT.hair) return "hair";
  if (s.material === MAT.lens) return "sunglasses";
  if (s.material === MAT.skin) return /^(hand|wrist)/.test(s.id) ? "hands" : "face-skin";
  throw new Error(`no region for ${s.id}`);
}

/** 🧊 The bones that leave the sagittal plane in a pose or clip (spread or turned arms, the cane): their shapes are solids of the bone-local side profile, the same in every view. */
/** 🤚 The lateral place of the left arm chain at rest (the `z` of `arm-l`), from which the span of an extruded hand is measured. */
const ARM_Z = (asset.rig.bones as Json[]).find((b) => b.id === "arm-l")!.z as number;
/** 🤏 How far the relaxed fingers curl inwards of the wrist at the finger tips, which the lateral span of the hanging hand follows. */
const CURL = 0.021;
const SOLID_BONE = /^(arm|forearm|hand)-[lr]$|^cane$/;
const LOCAL: Frame = { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] };
/** 📐 The lateral extent of solids in the frame of their bone. */
function lateral(solids: readonly Json[]): [number, number] {
  let [lo, hi] = [Infinity, -Infinity];
  for (const m of meshesIn(asset, { [solids[0]!.bone]: LOCAL }, solids)) for (let i = 2; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i]!); hi = Math.max(hi, m.world[i]!); }
  return [r4(lo), r4(hi)];
}
function solidOf(id: string, bone: string, solids: readonly Json[], ring: readonly P2[]): Json {
  const [z0, z1] = lateral(solids);
  if (bone === "hand-l") return { kind: "extruded", sweep: "w", z0: r4(ARM_Z + Math.max(z0, -0.03)), z1: r4(ARM_Z + z1), zTip: [r4(ARM_Z + Math.max(z0, -0.03) - CURL), r4(ARM_Z + z1 - CURL)] };
  if (bone === "hand-r") return { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: 0 };
  const ys = ring.map((p) => p.y);
  const scale = (z1 - z0) / (Math.max(...ys) - Math.min(...ys));
  return { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: 0, ...(bone === "cane" || Math.abs(scale - 1) < 0.02 ? {} : { lateralScale: Math.round(scale * 100) / 100 }) };
}
/** 🖊️ Heavier outline pens for the small filled parts whose width alone stays under the printable feature at the coarse end of their band (1:1000 for the 1:500 drawing, 1:200 for the neck between hair and collar): the pen carries them over 0.2 mm. */
const HEAVY: Record<number, Record<string, number>> = { 200: { neck: 0.16 }, 500: { "hand-r": 0.17, sunglasses: 0.16, "hair-fringe": 0.16, "shank-l": 0.16, "shank-r": 0.16, "boot-l": 0.16, "boot-r": 0.16 } };
/** 🦯 Half the drawn width of the cane shaft: its true 12 mm where the scale prints it, else the thinnest strip that prints. */
const ROD_HALF: Record<number, number> = { 50: 0.006, 100: 0.006, 200: 0.012, 500: 0.0375 };
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
/** 🖊️ The pen of a rod: its true width on paper, never under the thinnest printable line. */
function rodPen(id: string, scale: number, view: View): number {
  const k = 1000 / scale;
  const width = id === "bag-strap" ? (view === "elevation" ? 0.006 : 0.027) : id.startsWith("cane") && id !== "cane-loop" ? 0.012 : 0.004;
  return Math.max(0.1, Math.round(width * k * 100) / 100);
}
/** 〰️ The centre line of a rod in the frame of its bone. */
function centreLine(s: Json): number[][] {
  if (s.kind === "capsule") return [s.p0, s.p1];
  return (s.sections as Json[]).map((q) => q.at);
}

type Stroke = { id: string; bone: string; after: string; tone: number; region: string; material: string; points: V3[] };
/** 🖋️ The detail strokes of a scale and view, stated as points of the standing figure (x ahead, y up, z left). */
function strokes(scale: number, view: View): Stroke[] {
  const out: Stroke[] = [];
  const jacket = (id: string, points: V3[], after: string): Stroke => ({ id, bone: "torso", after, tone: 0.2, region: "jacket", material: MAT.jacket, points });
  const placket = jacket("jacket-placket", [1.44, 1.3, 1.15, 1.0, 0.954].map((y) => [jacketX(y, -0.014, 1) + 0.002, y, -0.014] as V3), "jacket-button");
  if (scale <= 100 && view === "front") out.push(placket);
  if (scale !== 50) return out;
  if (view === "front") {
    out.push({ id: "mouth-line", bone: "head", after: "lens-", tone: 0.3, region: "face-skin", material: MAT.skin, points: [[0.109, 1.5305, -0.019], [0.1145, 1.5295, 0], [0.109, 1.5305, 0.019]] });
    out.push(jacket("jacket-band", [-0.18, -0.09, 0, 0.09, 0.18].map((z) => [jacketX(0.996, z, 1) + 0.0065, 0.996, z] as V3), "jacket-button"));
    for (const side of [1, -1]) out.push(jacket(`jacket-yoke-${side > 0 ? "l" : "r"}`, [[jacketX(1.33, side * 0.012, 1), 1.33, side * 0.012], [jacketX(1.335, side * 0.1, 1), 1.335, side * 0.1], [jacketX(1.35, side * 0.17, 1), 1.35, side * 0.17]], "jacket-button"));
  }
  if (view === "elevation") {
    out.push({ id: "mouth-line", bone: "head", after: "lens-r", tone: 0.3, region: "face-skin", material: MAT.skin, points: [[0.1145, 1.5305, -0.01], [0.098, 1.5285, -0.028]] });
    out.push(jacket("jacket-band", [[-0.118, 1.0, -0.1], [0.0, 1.0, -0.177], [0.111, 1.0, -0.06]], "collar"));
    out.push(jacket("jacket-side-seam", [[0.0, 1.0, -0.177], [0.002, 1.15, -0.158], [0.004, 1.3, -0.172]], "collar"));
  }
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
  const hatched = (work.representations[String(scale)].hatches ?? []).some((h: Json) => h.id === "denim");
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s, view, scale === 500);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    if (view === "front") {
      const hair = body.find((s) => s.id === "hair")!;
      const above = hair.sections.filter((q: Json) => world("head", q.at)[1] >= HAIRLINE);
      if (above.length >= 2) groups.set("hair-fringe", [{ ...hair, id: "hair-fringe", sections: above }]);
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean }[] = [];
    const fixed: Json[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const solid = SOLID_BONE.test(own.bone);
      const mask = projectMask(solid ? "elevation" : view, meshesIn(work, solid ? { [own.bone]: LOCAL } : frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      const role = own.role === "detail" || own.role === "glazing" ? (scale === 500 ? "body" : "detail") : "body";
      const meta: Json = { id, bone: own.bone, role, z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      if (hatched && own.material === MAT.jacket && meta.role === "body") meta.hatch = "denim";
      const pen = HEAVY[scale]?.[id];
      if (pen !== undefined) meta.penMm = pen;
      if (solid) { fixed.push({ ...meta, solid: solidOf(id, own.bone, solids, ring), kind: "poly", points: ring.map((p) => [r4(p.x), r4(p.y)]), closed: true }); continue; }
      jobs.push({ meta, target: ring, closed: true });
    }
    for (const s of body.filter((q) => q.slender)) {
      if (s.id === "cane-loop") continue;
      const line = centreLine(s).map((p) => projectWorld(view, world(s.bone, p)));
      const pts = s.kind === "capsule" ? line : simplifyOpen(line, tol / 2);
      const stroke = { id: s.id, bone: s.bone, role: s.bone === "cane" && s.id !== "cane-loop" ? "body" : scale === 500 ? "body" : "detail", z: layer(s.id, view), filled: false, tone: s.tone > 0.9 ? 0.25 : s.tone, region: regionOf(s), material: s.material, penMm: rodPen(s.id, scale, view) };
      if (SOLID_BONE.test(s.bone)) {
        const half = ROD_HALF[scale]!;
        const [a, b] = [s.p0[0] as number, s.p1[0] as number];
        fixed.push({ id: s.id, bone: s.bone, role: "body", penMm: scale === 500 || scale === 50 ? 0.13 : 0.1, z: stroke.z, filled: true, tone: s.tone, region: stroke.region, material: s.material, solid: { kind: "revolved", axisBone: s.bone, axis: "u", axisX: 0, axisZ: 0 }, kind: "poly", points: [[r4(a), -half], [r4(b), -half], [r4(b), half], [r4(a), half]], closed: true });
      }
      else jobs.push({ meta: stroke, target: pts, closed: false });
    }
    for (const s of strokes(scale, view)) jobs.push({ meta: { id: s.id, bone: s.bone, role: "detail", z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, region: s.region, material: s.material }, target: s.points.map((p) => projectWorld(view, p)), closed: false });
    const solved = inverse(work, scale, view, jobs);
    const shapes = jobs.map((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed };
    });
    shapes.push(...(fixed as never[]));
    shapes.sort((a, b) => a.z - b.z);
    views[view] = shapes;
    work.representations[String(scale)].geometry[view] = shapes;
    console.log(scale, view, `${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
  geometry[String(scale)] = views;
}
/** ✂️ Douglas-Peucker on an open line. */
function simplifyOpen(line: P2[], tol: number): P2[] {
  if (line.length < 3) return line;
  const a = line[0]!;
  const b = line[line.length - 1]!;
  const len = Math.hypot(b.x - a.x, b.y - a.y) || 1e-12;
  let worst = 0;
  let at = 0;
  for (let i = 1; i + 1 < line.length; i++) {
    const d = Math.abs((b.x - a.x) * (a.y - line[i]!.y) - (a.x - line[i]!.x) * (b.y - a.y)) / len;
    if (d > worst) { worst = d; at = i; }
  }
  if (worst <= tol) return [a, b];
  return [...simplifyOpen(line.slice(0, at + 1), tol).slice(0, -1), ...simplifyOpen(line.slice(at), tol)];
}
mkdirSync(OUT, { recursive: true });
writeFileSync(`${OUT}/geometry.json`, JSON.stringify(geometry));
