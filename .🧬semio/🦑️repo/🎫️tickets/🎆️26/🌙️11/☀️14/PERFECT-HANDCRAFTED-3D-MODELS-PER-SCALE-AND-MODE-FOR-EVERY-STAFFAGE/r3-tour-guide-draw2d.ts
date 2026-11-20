/** ✏️ Redraws the 2D drawings of `humans/tour-guide` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view and the detail strokes (zip, pocket zips, lanyard, mouth) are stated by hand below; pointer and pole are drawn as strips of their printable width where their true diameter would not print.
 *
 * Usage: `bun r3-tour-guide-draw2d.ts` writes `🗑️generated/r3-tour-guide/geometry.json` (spliced by `r3-tour-guide-apply.py`).
 * @see ./r3-tour-guide-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { ASSET } from "./r3-tour-guide-kit.ts";
import { volumes } from "./r3-tour-guide-build.ts";
import { JACKET_LOW, JACKET_UP, YV, axisX, lanyardPath, surfaceX } from "./r3-tour-guide-s50.ts";

type V3 = [number, number, number];
const VOLUMES = await volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(id: string, view: View, scale: number): string | null {
  if (/^lanyard/.test(id)) return null;
  if (/^pointer/.test(id)) return "pointer";
  if (/^(pole|pole-ferrule|flag-sleeve)$/.test(id)) return "pole";
  if (id === "pole-finial") return "pole-finial";
  const hand = /^hand-([lr])/.exec(id);
  if (hand) return `hand-${hand[1]}`;
  const shoe = /^shoe-(?:sole|heel|lacing)-([lr])$/.exec(id);
  if (shoe) return `shoe-${shoe[1]}`;
  const toe = /^shoe-toe-sole-([lr])$/.exec(id);
  if (toe) return `shoe-toe-${toe[1]}`;
  const cuff = /^cuff-([lr])$/.exec(id);
  if (cuff) return `forearm-${cuff[1]}`;
  if (/^head-ear-r$/.test(id) && view === "elevation" && scale <= 100) return "head-ear-r";
  if (/^head-eye-[lr]$/.test(id) && view === "front" && scale === 50) return id;
  if (/^head-/.test(id)) return "head";
  if (/^cap/.test(id)) return "cap";
  if (id === "badge" || id === "badge-clip") return view === "front" ? "badge" : "jacket";
  if (/^jacket-(zip-collar|zip-puller)$/.test(id)) return "jacket-collar";
  if (/^jacket-(zip|chest-pocket)$/.test(id)) return "jacket";
  if (/^jacket-(zip-skirt|pocket-[lr]|hem)$/.test(id)) return scale === 500 ? "jacket" : "jacket-skirt";
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^shank-l/, /^shoe-l/, /^shoe-toe-l/, /^thigh-l/, /^sleeve-l/, /^forearm-l/, /^trousers-seat/, /^shank-r/, /^shoe-r/, /^shoe-toe-r/, /^thigh-r/, /^jacket-skirt/, /^jacket$/, /^neck/, /^jacket-collar/, /^head$/, /^hair/, /^cap/, /^head-ear-r/, /^flag$/, /^pole$/, /^pole-finial/, /^hand-l/, /^sleeve-r/, /^forearm-r/, /^wrist-r/, /^hand-r/, /^pointer/],
  front: [/^flag$/, /^trousers-seat/, /^thigh-/, /^shank-/, /^shoe-[lr]/, /^shoe-toe-/, /^jacket-skirt/, /^jacket$/, /^badge/, /^neck/, /^jacket-collar/, /^hair/, /^head$/, /^head-eye-/, /^cap/, /^sleeve-l/, /^forearm-l/, /^pole$/, /^pole-finial/, /^hand-l/, /^sleeve-r/, /^forearm-r/, /^wrist-r/, /^hand-r/, /^pointer/],
  plan: [/^shoe-[lr]/, /^shoe-toe-/, /^shank-/, /^thigh-/, /^trousers-seat/, /^jacket-skirt/, /^hand-l/, /^forearm-l/, /^jacket$/, /^sleeve-l/, /^jacket-collar/, /^neck/, /^head$/, /^hair/, /^cap/, /^sleeve-r/, /^forearm-r/, /^wrist-r/, /^hand-r/, /^pointer/, /^pole$/, /^pole-finial/, /^flag$/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
function regionOf(s: Json): string {
  if (s.material === "skin") return /^hand/.test(s.id) ? "hand-skin" : /^wrist/.test(s.id) ? "arm-skin" : "head-skin";
  if (s.material === "hair" || s.material === "cap-fabric") return "hair";
  if (s.material === "softshell") return /^(sleeve|forearm|cuff)/.test(s.id) ? "arm-clothing" : "torso-clothing";
  if (s.material === "cotton-clothing") return /^(badge|lanyard)/.test(s.id) ? "torso-clothing" : "leg-clothing";
  if (s.material === "leather-shoe") return "shoes";
  if (s.material === "pointer-aluminium") return /^pointer/.test(s.id) ? "pointer-metal" : "pole-metal";
  if (s.material === "flag-textile") return "flag-cloth";
  throw new Error(`no region for ${s.id}`);
}
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
/** 🪄 The width pointer and pole are drawn with where their true diameter does not print (1:200 and 1:500); at 1:50 and 1:100 their true outline is traced. The pole of 1:500 stays a 36 mm strip: a wider one cuts the left sleeve behind it into slivers below the print minimum. */
const STRIP: Record<number, { pointer: number; pole: number } | null> = { 50: null, 100: null, 200: { pointer: 0.024, pole: 0.03 }, 500: { pointer: 0.06, pole: 0.036 } };

type Stroke = { id: string; bone: string; after: string; tone: number; region: string; material: string; points: V3[] };
/** 🖋️ The detail strokes of a scale and view, stated as points of the standing figure (x ahead, y up, z left). */
function strokes(scale: number, view: View): Stroke[] {
  const out: Stroke[] = [];
  if (view !== "front") return out;
  if (scale <= 100) for (const side of [1, -1] as const) out.push({ id: `lanyard-${side > 0 ? "l" : "r"}`, bone: "torso", after: "jacket", tone: 0.1, region: "torso-clothing", material: "cotton-clothing", points: lanyardPath(side).slice(2) });
  if (scale !== 50) return out;
  const zip = (id: string, bone: string, t: typeof JACKET_UP, ys: number[], z: number, after: string): Stroke => ({ id, bone, after, tone: 0.15, region: "torso-clothing", material: "softshell", points: ys.map((y) => [surfaceX(bone, t, y, z, 1) + 0.004, y, z] as V3) });
  out.push(zip("jacket-zip-line", "torso", JACKET_UP, [1.46, 1.3, 1.15, 1.03], 0, "jacket"));
  out.push(zip("jacket-zip-skirt-line", "pelvis", JACKET_LOW, [1.065, 0.95, 0.85], 0, "jacket-skirt"));
  out.push({ ...zip("jacket-pocket-l-line", "pelvis", JACKET_LOW, [0.92, 1.045], 0, "jacket-skirt"), points: [[surfaceX("pelvis", JACKET_LOW, 0.92, 0.14, 1) + 0.004, 0.92, 0.14], [surfaceX("pelvis", JACKET_LOW, 1.045, 0.104, 1) + 0.004, 1.045, 0.104]] });
  out.push({ ...zip("jacket-pocket-r-line", "pelvis", JACKET_LOW, [0.92, 1.045], 0, "jacket-skirt"), points: [[surfaceX("pelvis", JACKET_LOW, 0.92, -0.14, 1) + 0.004, 0.92, -0.14], [surfaceX("pelvis", JACKET_LOW, 1.045, -0.104, 1) + 0.004, 1.045, -0.104]] });
  out.push({ ...zip("jacket-chest-pocket-line", "torso", JACKET_UP, [1.28, 1.375], 0.09, "jacket"), points: [[surfaceX("torso", JACKET_UP, 1.28, 0.09, 1) + 0.004, 1.28, 0.09], [surfaceX("torso", JACKET_UP, 1.375, 0.102, 1) + 0.004, 1.375, 0.102]] });
  const hx = (d: number): number => axisX("head", YV - d);
  out.push({ id: "mouth-line", bone: "head", after: "head-eye-", tone: 0.3, region: "head-skin", material: "skin", points: [[hx(0.19) + 0.1, YV - 0.1895, 0.02], [hx(0.19) + 0.108, YV - 0.191, 0], [hx(0.19) + 0.1, YV - 0.1895, -0.02]] });
  return out;
}

/** 🌀 The bones that a pose or clip turns out of the rig plane (the trunk twists, the right arm spreads): their shapes are solids of the rig, one 3D definition carried by the full frame of the bone. */
const SPREAD = /^(torso|head|upper-arm-[lr]|forearm-[lr]|hand-[lr]|pointer|flag-pole|flag)$/;
const ROUND = /^(upper-arm-[lr]|forearm-[lr]|pointer|flag-pole)$/;
const IDENTITY: Frame = { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] };
const LOCAL = new Proxy({} as Record<string, Frame>, { get: () => IDENTITY });
const CAM: Record<View, { x: V3; y: V3 }> = Object.fromEntries((["elevation", "front", "plan"] as View[]).map((view) => {
  const e = (p: V3): P2 => projectWorld(view, p);
  const c = [e([1, 0, 0]), e([0, 1, 0]), e([0, 0, 1])];
  return [view, { x: c.map((q) => q.x) as V3, y: c.map((q) => q.y) as V3 }];
})) as Record<View, { x: V3; y: V3 }>;
const dot3 = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
/** 🎯 The coefficients (a, b) with `view(base + a A + b B) = target`. */
function solve2(view: View, base: V3, A: V3, B: V3, target: P2): [number, number] {
  const c = CAM[view];
  const [m00, m10, m01, m11] = [dot3(c.x, A), dot3(c.y, A), dot3(c.x, B), dot3(c.y, B)];
  const [dx, dy] = [target.x - dot3(c.x, base), target.y - dot3(c.y, base)];
  const det = m00 * m11 - m01 * m10;
  if (Math.abs(det) < 0.05) throw new Error(`plane seen edge-on in ${view}`);
  return [(m11 * dx - m01 * dy) / det, (-m10 * dx + m00 * dy) / det];
}
function outline(view: View, list: ReturnType<typeof meshesIn>, cell: number, tol: number): P2[] | null {
  const mask = projectMask(view, list, cell, 4 + Math.ceil(0.03 / cell));
  if (mask === null) return null;
  const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
  return rings.length === 0 ? null : simplifyRing(rings[0]!, tol);
}
/** 📏 The range of the vertices of meshes along one axis. */
function range(list: ReturnType<typeof meshesIn>, axis: 0 | 1 | 2): [number, number] {
  let lo = Infinity;
  let hi = -Infinity;
  for (const m of list) for (let i = axis; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i]!); hi = Math.max(hi, m.world[i]!); }
  return [lo, hi];
}
/** 🏺 The half profile (u, radius) of a part seen as a body of revolution about its bone: the widest reach of its solids from the axis through their lateral middle, parallel to the bone, station by station. */
function meridian(list: ReturnType<typeof meshesIn>, step: number, tol: number, wc: number): [number, number][] {
  const [lo, hi] = range(list, 0);
  const n = Math.max(2, Math.ceil((hi - lo) / step));
  const r = new Array<number>(n + 1).fill(0);
  for (const m of list) {
    const w = m.world;
    for (let t = 0; t < m.indices.length; t += 3) {
      const P = [0, 1, 2].map((k) => [w[m.indices[t + k]! * 3]!, Math.hypot(w[m.indices[t + k]! * 3 + 1]!, w[m.indices[t + k]! * 3 + 2]! - wc)] as [number, number]);
      for (let e = 0; e < 3; e++) {
        const [a, b] = [P[e]!, P[(e + 1) % 3]!];
        const [i0, i1] = [Math.ceil(((Math.min(a[0], b[0]) - lo) / (hi - lo)) * n), Math.floor(((Math.max(a[0], b[0]) - lo) / (hi - lo)) * n)];
        for (let i = Math.max(0, i0); i <= Math.min(n, i1); i++) {
          const u = lo + ((hi - lo) * i) / n;
          const k = Math.abs(b[0] - a[0]) < 1e-12 ? 0 : (u - a[0]) / (b[0] - a[0]);
          r[i] = Math.max(r[i]!, a[1] + (b[1] - a[1]) * k);
        }
        for (const q of [a, b]) { const i = Math.round(((q[0] - lo) / (hi - lo)) * n); r[i] = Math.max(r[i]!, q[1]); }
      }
    }
  }
  const ring: P2[] = [{ x: lo, y: 0 }, ...r.map((y, i) => ({ x: lo + ((hi - lo) * i) / n, y: Math.max(y, 0.0005) })), { x: hi, y: 0 }];
  return simplifyRing(ring, tol).map((p) => [r4(p.x), r4(Math.max(0, p.y))]);
}
const REST = restFrames(ASSET.rig as never) as unknown as Record<string, { frame: Frame }>;
const WEDGE = { a: 0.001, W: 1 } as const;
const LENGTH: Record<string, number> = Object.fromEntries(ASSET.rig.bones.map((b: Json) => [b.id, b.lengthM]));
/** 🧊 A filled part of a turned bone as a solid of the rig. Limbs and rods are bodies of revolution about their bone (the same profile in every view). Hands and the flag are their bone-local outline swept through their real thickness (`w` in the side elevation, `v` in front and plan; the flag along `w` everywhere). Trunk and head are their side outline swept across a quarter of their breadth (the depth an elliptical section keeps when the trunk twists), their front outline as a sheet in the frontal plane through the bone axis (every front sheet of a bone lies in that one plane, so face and badge stay on head and chest when the trunk twists), and their plan outline as a flat wedge at the joint whose lateral span opens with u, so the plan keeps its true outline and turns with the bone. */
function solidShape(view: View, bone: string, frame: Frame, solids: Json[], target: P2[], cell: number, tol: number, strip: number | null): Json {
  const local = meshesIn(work, LOCAL, solids);
  const zr = REST[bone]!.frame.o[2];
  if (ROUND.test(bone)) {
    const [lo, hi] = range(local, 0);
    const [wa, wb] = range(local, 2);
    const wc = strip !== null ? 0 : r4((wa + wb) / 2);
    const points = strip !== null ? [[r4(lo), 0], [r4(lo), strip / 2], [r4(hi), strip / 2], [r4(hi), 0]] : meridian(local, cell * 2, tol / 2, wc);
    return { kind: "poly", points, closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: wc, mirror: true } };
  }
  const uv = (): number[][] => outline("elevation", local, cell, tol)!.map((p) => { const [a, b] = solve2("elevation", [0, 0, 0], [1, 0, 0], [0, 1, 0], p); return [r4(a), r4(b)]; });
  const uw = (): number[][] => outline("plan", local, cell, tol)!.map((p) => { const [a, b] = solve2("plan", [0, 0, 0], [1, 0, 0], [0, 0, 1], p); return [r4(a), r4(b)]; });
  const [w0, w1] = range(local, 2);
  const [v0, v1] = range(local, 1);
  if (bone === "flag") return { kind: "poly", points: uv(), closed: true, solid: { kind: "extruded", sweep: "w", z0: r4(zr + w0), z1: r4(zr + w1) } };
  if (/^hand-/.test(bone)) return view === "elevation" ? { kind: "poly", points: uv(), closed: true, solid: { kind: "extruded", sweep: "w", z0: r4(zr + w0), z1: r4(zr + w1) } } : { kind: "poly", points: uw(), closed: true, solid: { kind: "extruded", sweep: "v", z0: r4(v0), z1: r4(v1) } };
  if (view === "elevation") {
    const half = Math.max(0.001, 0.26 * (w1 - w0) / 2);
    return { kind: "poly", points: uv(), closed: true, solid: { kind: "extruded", sweep: "w", z0: r4(zr + (w0 + w1) / 2 - half), z1: r4(zr + (w0 + w1) / 2 + half) } };
  }
  if (view === "front") {
    const c = 0;
    const base = frame.o;
    return { kind: "poly", points: target.map((p) => solve2("front", base, frame.u, frame.w, p).map(r4)), closed: true, solid: { kind: "extruded", sweep: "v", z0: r4(c - 0.001), z1: r4(c + 0.001) } };
  }
  const world = target.map((p) => solve2("plan", [0, 0, 0], [1, 0, 0], [0, 0, 1], p));
  const back = world.reduce((k, p, i) => (p[0] < world[k]![0] ? i : k), 0);
  const fore = world.reduce((k, p, i) => (p[0] > world[k]![0] ? i : k), 0);
  const chain = (from: number, to: number): [number, number][] => { const out: [number, number][] = []; for (let i = from; ; i = (i + 1) % world.length) { out.push(world[i]!); if (i === to) break; } return out; };
  const [one, two] = [chain(back, fore), chain(fore, back).reverse()];
  const mean = (c: [number, number][]): number => c.reduce((n, p) => n + p[1] - frame.o[2], 0) / c.length;
  const left = mean(one) > mean(two) ? one : two;
  const at = (x: number, hw: number): number[] => {
    const u = (LENGTH[bone]! * (Math.max(hw, WEDGE.a) - WEDGE.a)) / (WEDGE.W - WEDGE.a);
    return [r5(u), r5((x - frame.o[0] - u * frame.u[0]) / frame.v[0])];
  };
  const points = [...left.map(([x, z]) => at(x, z - frame.o[2])), at(left[left.length - 1]![0], 0), at(left[0]![0], 0)];
  return { kind: "poly", points, closed: true, solid: { kind: "extruded", sweep: "w", z0: r4(zr - WEDGE.a), z1: r4(zr + WEDGE.a), zTip: [r4(zr - WEDGE.W), r4(zr + WEDGE.W)] } };
}
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
/** 🎀 A stroke as the outline of a ribbon of its real width (a stroke of a turned bone is drawn as a solid sheet). */
function ribbon(path: P2[], width: number): P2[] {
  const side = (s: number): P2[] => path.map((p, i) => {
    const [a, b] = [path[Math.max(0, i - 1)]!, path[Math.min(path.length - 1, i + 1)]!];
    const n = Math.hypot(b.x - a.x, b.y - a.y) || 1;
    return { x: p.x - ((b.y - a.y) / n) * s * width / 2, y: p.y + ((b.x - a.x) / n) * s * width / 2 };
  });
  return [...side(1), ...side(-1).reverse()];
}
const WIDTH: Record<string, number> = { "lanyard-l": 0.007, "lanyard-r": 0.007, "jacket-zip-line": 0.006, "jacket-chest-pocket-line": 0.006, "mouth-line": 0.004 };

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
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean; solids?: Json[]; strip?: number | null }[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids.find((s) => s.role !== "detail") ?? solids[0]!;
      const meta: Json = { id, bone: own.bone, role: own.role === "detail" && !/^(badge)$/.test(id) ? "detail" : "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      const strip = STRIP[scale];
      if (strip !== null && (id === "pointer" || id === "pole")) {
        const f = frames[id === "pointer" ? "pointer" : "flag-pole"]!;
        const at = (u: number): P2 => projectWorld(view, [0, 1, 2].map((k) => f.o[k]! + u * f.u[k]!) as V3);
        const [a, b] = id === "pointer" ? [at(-0.05), at(0.55)] : [at(-0.33), at(1.0)];
        const d = { x: b.x - a.x, y: b.y - a.y };
        const n = Math.hypot(d.x, d.y);
        if (n < 1e-6) {
          const h = strip[id] / 2;
          jobs.push({ meta, target: [{ x: a.x - h, y: a.y - h }, { x: a.x + h, y: a.y - h }, { x: a.x + h, y: a.y + h }, { x: a.x - h, y: a.y + h }], closed: true, solids, strip: strip[id] });
          continue;
        }
        const h = strip[id] / 2;
        const o = { x: (-d.y / n) * h, y: (d.x / n) * h };
        jobs.push({ meta, target: [{ x: a.x - o.x, y: a.y - o.y }, { x: b.x - o.x, y: b.y - o.y }, { x: b.x + o.x, y: b.y + o.y }, { x: a.x + o.x, y: a.y + o.y }], closed: true, solids, strip: strip[id] });
        continue;
      }
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      jobs.push({ meta, target: simplifyRing(rings[0]!, tol), closed: true, solids, strip: null });
    }
    for (const s of strokes(scale, view)) jobs.push({ meta: { id: s.id, bone: s.bone, role: "detail", z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, region: s.region, material: s.material }, target: s.points.map((p) => projectWorld(view, p)), closed: false });
    const flat = jobs.filter((j) => !SPREAD.test(j.meta.bone));
    const solved = inverse(work, scale, view, flat);
    const shapes = jobs.map((j) => {
      if (SPREAD.test(j.meta.bone)) {
        const f = frames[j.meta.bone]!;
        if (j.solids !== undefined) return { ...j.meta, ...solidShape(view, j.meta.bone, f, j.solids, j.target, cell, tol, j.strip ?? null) };
        const c = 0;
        const base = f.o;
        return { ...j.meta, kind: "poly", points: ribbon(j.target, WIDTH[j.meta.id]!).map((p) => solve2("front", base, f.u, f.w, p).map(r4)), closed: true, solid: { kind: "extruded", sweep: "v", z0: r4(c - 0.001), z1: r4(c + 0.001) } };
      }
      const k = flat.indexOf(j);
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
const out = join(import.meta.dir, "🗑️generated", "r3-tour-guide");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "geometry.json"), JSON.stringify(geometry));
