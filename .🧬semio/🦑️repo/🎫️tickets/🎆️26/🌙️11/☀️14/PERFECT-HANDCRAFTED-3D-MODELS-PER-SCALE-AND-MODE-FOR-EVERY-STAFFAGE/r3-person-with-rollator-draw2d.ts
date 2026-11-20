/** ✏️ Redraws the 2D drawings of `humans/person-with-rollator` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for. Which solids one drawn shape unites, how a shape is carried (a flat outline solved into the frame of its bone through the product's own placement, a profile revolved about the bone for the arms, a profile extruded across for the hands, the wheels and the cross parts of the rollator, so that spread arms and the turned rollator of the seated pose are drawn right) and the detail strokes are stated by hand below; the layer order of a view follows the nearest point of every part to the viewer.
 *
 * Usage: `bun r3-person-with-rollator-draw2d.ts` writes `🗑️generated/r3-person-with-rollator/geometry.json` (laid over the asset by `r3-person-with-rollator-candidate.py`).
 * @see ./r3-person-with-rollator-build.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, sourcePath, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { ASSET, ENTRY, OUT } from "./r3-person-with-rollator-kit.ts";
import { coatX } from "./r3-person-with-rollator-figure.ts";
import { G } from "./r3-person-with-rollator-aid.ts";
import { AID_REGION } from "./r3-person-with-rollator-s50.ts";

type V3 = [number, number, number];
type Mode = "flat" | "revolved" | "extruded" | "disc";
type Spec = { id: string; mode: Mode; role?: string; hatch?: string; lift?: number };
const VOLUMES = (JSON.parse(readFileSync(sourcePath(ENTRY), "utf8")) as Json).volumes;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const IDENTITY: Frame = { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] };

/** 🧩 The drawn shape a body solid belongs to in a view and how it is carried (`null`: drawn by a stroke or not at all). */
function specOf(s: Json, view: View, scale: number): Spec | null {
  const id: string = s.id;
  const side = /-(l|r)(?=$|-)/.exec(id)?.[1];
  if (/^(arm|forearm)-/.test(s.bone)) return { id, mode: "revolved", hatch: s.material === "loden-wool" ? (side === "l" ? "far-side" : "loden-twill") : undefined };
  if (/^hand-/.test(s.bone)) return { id: s.bone, mode: "extruded" };
  if (/^wheel-/.test(s.bone)) {
    const name = /-(rear|front)-(l|r)$/.exec(id)![0].slice(1);
    if (/^tyre-/.test(id)) return { id, mode: "disc" };
    if (view !== "elevation") return null;
    if (/^rim-/.test(id)) return { id, mode: "flat", role: "detail", lift: 0.02 };
    return { id: `hub-${name}`, mode: "flat", role: "detail", lift: 0.04 };
  }
  if (s.bone === "frame") {
    if (s.slender === true) return null;
    if (/^(seat|cross-upper|cross-lower|backrest|bag)$/.test(id)) return { id, mode: "extruded" };
    if (/^basket(-floor|-rim)?$/.test(id)) return { id: "basket", mode: "extruded", hatch: "basket-mesh" };
    if (/^(post-clamp|post-knob)-/.test(id)) return { id: `handle-post-${side}`, mode: "flat" };
    if (/^(axle-plate|axle-rear|brake-block)-/.test(id)) return { id: `rear-leg-${side}`, mode: "flat" };
    if (/^(caster-|axle-front)/.test(id)) return { id: `caster-${side}`, mode: "flat" };
    if (/^brake-(housing|lever)-/.test(id)) return { id: `brake-lever-${side}`, mode: "flat" };
    return { id, mode: "flat" };
  }
  if (/^glasses-/.test(id)) return null;
  if (id === "beret-stalk") return { id: "beret", mode: "flat" };
  if (/^head-(nose|nostrils|brow|chin|cheek-[lr])$/.test(id)) return { id: "head", mode: "flat" };
  const sole = /^shoe-(heel|sole)-([lr])$/.exec(id);
  if (sole) return view === "plan" ? { id: `shoe-rear-${sole[2]}`, mode: "flat" } : { id: `shoe-sole-${sole[2]}`, mode: "flat" };
  const toeSole = /^shoe-toe-sole-([lr])$/.exec(id);
  if (toeSole && view === "plan") return { id: `shoe-toe-${toeSole[1]}`, mode: "flat" };
  const lace = /^shoe-lace-\d-([lr])$/.exec(id);
  if (lace) return { id: `shoe-rear-${lace[1]}`, mode: "flat" };
  if (view === "elevation") {
    if (/^(coat-lapel-[lr]|coat-placket|coat-pocket-l)$/.test(id)) return { id: "coat", mode: "flat", hatch: "loden-twill" };
    if (id === "coat-pocket-r") return { id, mode: "flat", role: "detail", lift: 0.03 };
    if (id === "coat-placket-hip") return { id: "coat-hip", mode: "flat", hatch: "loden-twill" };
    if (/^coat-button-\d$/.test(id)) return { id: id === "coat-button-3" ? "coat-hip" : "coat", mode: "flat", hatch: "loden-twill" };
    if (/^(head-ear-l|head-eye-l|hair-temple-l|head-lips)$/.test(id)) return { id: "head", mode: "flat" };
    if (id === "head-eye-r") return { id, mode: "flat", role: "detail", lift: 0.03 };
    if (/^(coat|coat-hip|skirt-r)$/.test(id)) return { id, mode: "flat", hatch: "loden-twill" };
    if (id === "skirt-l") return { id, mode: "flat", hatch: "far-side" };
  }
  if (view === "front") {
    if (id === "hair-nape") return { id: "head", mode: "flat" };
    if (/^coat-(button-\d|pocket-[lr])$/.test(id) || /^head-(eye-[lr]|lips)$/.test(id)) return { id, mode: "flat", role: "detail", lift: 0.05 };
    if (/^coat-lapel-/.test(id)) return { id, mode: "flat", role: "detail", lift: 0.04 };
    if (id === "coat-placket") return { id: "coat", mode: "flat" };
    if (id === "coat-placket-hip") return { id: "coat-hip", mode: "flat" };
  }
  if (view === "plan") {
    if (/^(coat-lapel-[lr]|coat-placket|coat-button-[012])$/.test(id)) return { id: "coat", mode: "flat" };
    if (/^(coat-placket-hip|coat-pocket-[lr]|coat-button-3)$/.test(id)) return { id: "coat-hip", mode: "flat" };
    if (/^(head-ear-[lr]|head-eye-[lr]|hair-temple-[lr]|head-lips|hair-nape)$/.test(id)) return { id: "head", mode: "flat" };
    const sock = /^sock-([lr])$/.exec(id);
    if (sock) return { id: `shank-${sock[1]}`, mode: "flat" };
  }
  return { id, mode: "flat", role: s.role === "detail" ? "detail" : undefined };
}
function regionOf(s: Json): string {
  const aid = AID_REGION(s);
  if (aid) return aid;
  if (s.material === "loden-wool" || s.material === "cotton-blouse" || (s.material === "wool-knit" && /cuff/.test(s.id))) return "coat";
  if (s.material === "wool-trouser") return "trousers";
  if (s.material === "leather-shoe" || s.material === "wool-knit") return "shoes";
  if (s.material === "wool-beret") return "beret";
  if (s.material === "hair-white") return "hair";
  if (s.material === "skin-aged") return /^(hand|wrist)/.test(s.id) ? "hands" : "face-skin";
  throw new Error(`no region for ${s.id}`);
}
const DISC: Record<number, number> = { 50: 32, 100: 24, 200: 16, 500: 12 };
/** ↔️ The lateral position of a bone origin at rest: the span of an extruded shape is stated from the symmetry plane. */
function restZ(bone: string): number {
  let z = 0;
  for (let b = (ASSET.rig.bones as Json[]).find((q) => q.id === bone); b !== undefined; b = (ASSET.rig.bones as Json[]).find((q) => q.id === b.parent)) z += b.z ?? 0;
  return z;
}
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
const DEPTH: Record<View, (p: Float64Array, i: number) => number> = { elevation: (p, i) => -p[i + 2]!, front: (p, i) => p[i]!, plan: (p, i) => p[i + 1]! };

type Stroke = { id: string; bone: string; tone: number; region: string; material: string; closed: boolean; points: V3[]; lift: number };
/** 🖋️ The detail strokes of a scale and view, stated as bone-local points of their solids. */
function strokes(scale: number, view: View, frames: Record<string, Frame>): Stroke[] {
  const at = (bone: string, p: V3): V3 => { const f = frames[bone]!; return [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3; };
  const head = (id: string, points: V3[], closed = false, tone = 0.15): Stroke => ({ id, bone: "head", tone, region: "face-skin", material: "skin-aged", closed, points: points.map((p) => at("head", p)), lift: 0.06 });
  const out: Stroke[] = [];
  const placket = (ys: number[], bone: string, id: string): Stroke => ({ id, bone, tone: 0.2, region: "coat", material: "loden-wool", closed: false, points: ys.map((y) => [coatX(y, 0.003, 1) + 0.003, y, 0.003] as V3), lift: 0.06 });
  if (view === "front" && scale <= 100) out.push(placket([1.3, 1.2, 1.1, 1.0, 0.9], "torso", "coat-front-edge"), placket([0.9, 0.8, 0.72], "pelvis", "coat-front-edge-hip"));
  if (scale !== 50) return out;
  const frame = (id: string, points: V3[], region: string, material: string, tone = 0.3): Stroke => ({ id, bone: "frame", tone, region, material, closed: false, points: points.map((p) => at("frame", p)), lift: 0.03 });
  const g = G.bag;
  const loop = (u: number, lean: number): V3[] => Array.from({ length: 9 }, (_, i) => { const a = (i / 8) * Math.PI; return [u + lean * Math.sin(a), g.v1 - 0.004 + 0.066 * Math.sin(a), -0.045 * Math.cos(a)] as V3; });
  out.push(frame("bag-handle-rear", loop(g.u0 + 0.034, -0.012), "bag", "cotton-tote", 0.4), frame("bag-handle-front", loop(g.u1 - 0.034, 0.014), "bag", "cotton-tote", 0.4));
  for (const k of view === "elevation" ? [-1] : [1, -1]) {
    const S = k * G.side;
    out.push(frame(`brake-cable-${k > 0 ? "l" : "r"}`, [[0.236, 0.752, S + k * 0.004], [0.268, 0.742, S + k * 0.012], [0.284, 0.71, S + k * 0.018], [0.262, 0.652, S + k * 0.018], [0.15, 0.4, S + k * 0.018], [0.07, 0.24, S + k * 0.02], [0.045, 0.214, S + k * 0.026]], "frame", "nylon-grey", 0.2));
  }
  if (view === "elevation") {
    out.push(head("glasses-temple", [[0.083, -0.088, -0.052], [0.086, 0.012, -0.075]]));
    out.push(head("glasses-rim", [[0.058, -0.0905, -0.0295], [0.097, -0.0905, -0.0295]]));
    out.push(head("mouth-line", [[0.019, -0.084, -0.008], [0.021, -0.066, -0.026]], false, 0.3));
  }
  if (view === "front") {
    for (const side of [1, -1]) out.push(head(`glasses-rim-${side > 0 ? "l" : "r"}`, Array.from({ length: 14 }, (_, i) => [0.0775 + 0.0195 * Math.cos((i * Math.PI) / 7), -0.0905, side * 0.0295 + 0.0195 * Math.sin((i * Math.PI) / 7)] as V3), true));
    out.push(head("glasses-bar", [[0.085, -0.0915, -0.01], [0.085, -0.0915, 0.01]]));
    out.push(head("nose-line", [[0.084, -0.088, 0.006], [0.05, -0.105, 0.011], [0.042, -0.104, 0.0], [0.043, -0.104, -0.009]], false, 0.4));
  }
  return out;
}

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of [50, 100, 200, 500]) {
  const work = JSON.parse(JSON.stringify(ASSET)) as Json;
  const hatches = new Set<string>((work.representations[String(scale)].hatches as Json[]).map((h) => h.id));
  const frames = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const outline = (view: View, meshes: ReturnType<typeof meshesIn>, id: string): Ring | null => {
    const mask = projectMask(view, meshes, cell, 4 + Math.ceil(0.03 / cell));
    if (mask === null) return null;
    const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
    const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
    if (lost > 0.03 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
    return simplifyRing(rings[0]!, tol);
  };
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, { spec: Spec; solids: Json[] }>();
    for (const s of body) {
      const spec = specOf(s, view, scale);
      if (spec === null) continue;
      const g = groups.get(spec.id);
      if (g) { g.solids.push(s); if (s.id === spec.id) g.spec = spec; } else groups.set(spec.id, { spec, solids: [s] });
    }
    type Item = { meta: Json; depth: number; local?: Json; target?: P2[]; closed: boolean };
    const items: Item[] = [];
    for (const [id, { spec, solids }] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const world = meshesIn(work, frames, solids);
      let depth = -Infinity;
      for (const m of world) for (let i = 0; i < m.world.length; i += 3) depth = Math.max(depth, DEPTH[view](m.world, i));
      depth += (spec.lift ?? 0) + (own.material === "hair-white" ? 0.02 : 0);
      const meta: Json = { id, bone: own.bone, role: spec.role ?? "body", z: 0, filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      if (spec.hatch !== undefined && hatches.has(spec.hatch) && view === "elevation") meta.hatch = spec.hatch;
      const rings: [number, number] | null = scale === 50 && view === "elevation" ? (/^tyre-/.test(id) ? [G.wheelR, 0.076] : /^rim-/.test(id) ? [0.078, 0.06] : null) : null;
      if (rings !== null) {
        const circle = (r: number): string => `M ${Array.from({ length: 32 }, (_, i) => `${r4(r * Math.cos((Math.PI * i) / 16))} ${r4(r * Math.sin((Math.PI * i) / 16))}`).join(" L ")} Z`;
        items.push({ meta, depth, closed: true, local: { kind: "path", d: `${circle(rings[0])} ${circle(rings[1])}` } });
        continue;
      }
      if (spec.mode === "flat") {
        const ring = outline(view, world, id);
        if (ring !== null) items.push({ meta, depth, target: ring, closed: true });
        continue;
      }
      const local = meshesIn(work, Object.fromEntries(solids.map((s) => [s.bone, IDENTITY])), solids);
      let [z0, z1] = [Infinity, -Infinity];
      for (const m of local) for (let i = 2; i < m.world.length; i += 3) { z0 = Math.min(z0, m.world[i]!); z1 = Math.max(z1, m.world[i]!); }
      if (spec.mode === "revolved") {
        const half = (own.sections as Json[]).map((q) => ({ x: q.at[0] as number, y: q.ry as number }));
        const ring = simplifyRing([{ x: half[0]!.x, y: 0 }, ...half, { x: half[half.length - 1]!.x, y: 0 }], tol / 2);
        const start = ring.findIndex((p) => p.y === 0 && p.x === half[0]!.x);
        const turned = start < 0 ? ring : [...ring.slice(start), ...ring.slice(0, start)];
        items.push({ meta, depth, closed: true, local: { kind: "poly", points: turned.map((p) => [r4(p.x), r4(Math.max(0, p.y))]), closed: true, solid: { kind: "revolved", axisBone: own.bone, axis: "u", axisX: 0, mirror: true, lateralScale: r4((own.sections[1].rx as number) / (own.sections[1].ry as number)) } } });
        continue;
      }
      const ring = spec.mode === "disc" ? Array.from({ length: DISC[scale]! }, (_, i) => ({ x: G.wheelR * Math.cos((2 * Math.PI * i) / DISC[scale]!), y: G.wheelR * Math.sin((2 * Math.PI * i) / DISC[scale]!) })) : outline("elevation", local, id);
      if (ring === null) continue;
      const tyre = spec.mode === "disc" ? Math.sign(z0 + z1) * (own.bone === "wheel-rear" ? G.rearTrack : G.side) : 0;
      const span = spec.mode === "disc" ? (scale === 500 ? [z0, z1] : [tyre - G.tyreHalf, tyre + G.tyreHalf]) : [z0, z1];
      const rest = restZ(own.bone);
      items.push({ meta, depth, closed: true, local: { kind: "poly", points: ring.map((p) => [r4(p.x), r4(p.y)]), closed: true, solid: { kind: "extruded", sweep: "w", z0: r4(rest + span[0]!), z1: r4(rest + span[1]!) } } });
    }
    for (const s of strokes(scale, view, frames)) {
      let depth = -Infinity;
      for (const p of s.points) depth = Math.max(depth, DEPTH[view](new Float64Array(p), 0));
      items.push({ meta: { id: s.id, bone: s.bone, role: "detail", z: 0, filled: false, tone: s.tone, region: s.region, material: s.material }, depth: depth + s.lift, target: s.points.map((p) => projectWorld(view, p)), closed: s.closed });
    }
    items.sort((a, b) => a.depth - b.depth);
    items.forEach((it, rank) => { it.meta.z = Math.round((1 + rank * 0.1) * 100) / 100; });
    const flat = items.filter((it) => it.target !== undefined);
    const solved = inverse(work, scale, view, flat.map((it) => ({ meta: it.meta, target: it.target! })));
    flat.forEach((it, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${it.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      it.local = { kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: it.closed };
    });
    const shapes = items.map((it) => ({ ...it.meta, ...it.local }));
    views[view] = shapes;
    work.representations[String(scale)].geometry[view] = shapes;
    console.log(scale, view, `${shapes.length} shapes, ${shapes.reduce((n, s) => n + (s.points?.length ?? 0), 0)} points`);
  }
  geometry[String(scale)] = views;
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "geometry.json"), JSON.stringify(geometry));
