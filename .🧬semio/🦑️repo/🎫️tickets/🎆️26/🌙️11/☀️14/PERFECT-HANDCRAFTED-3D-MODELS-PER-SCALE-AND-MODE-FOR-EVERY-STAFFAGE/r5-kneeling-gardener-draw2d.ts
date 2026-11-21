/** ✏️ Redraws the 2D drawings of `humans/kneeling-gardener` (ruling E1): every filled shape of every authored scale and view is the orthographic outline of the body solids of its bone, written as an economical polygon. Elevation shapes are bone-local outlines; front and plan shapes are solids of the same body (a limb, the hat, the tub and the trowel handle revolved about their axis, trunk, head, hands, feet, mat and blade as the lateral outline swept through the depth of the part, handles, ears and the trowel neck as the side outline swept through their width), so the three views agree in every pose. Detail strokes are stated by hand at the landmarks of the model.
 * Usage: `bun r5-kneeling-gardener-draw2d.ts` writes `🗑️generated/r3-kneeling-gardener/geometry.json` (spliced by `r3-kneeling-gardener-draw2d.py`).
 * @see ./r5-kneeling-gardener-figure.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { unionPolygons } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { CANDIDATE, SCALES, build } from "./r5-kneeling-gardener-figure.ts";
import type { Json } from "./r3-kneeling-gardener-kit.ts";

type P = { x: number; y: number };
type V = [number, number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };

const asset = JSON.parse(readFileSync(CANDIDATE, "utf8")).asset as Json;
const REST = Object.fromEntries(Object.entries(restFrames(asset.rig) as Record<string, { frame: Frame }>).map(([id, b]) => [id, b.frame]));
const LENGTH = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, b.lengthM as number]));
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/* ---------------------------------------------------------------- convex pieces of a solid in its bone frame */

function rotate(q: readonly number[], p: V): V {
  const [x, y, z, w] = q as [number, number, number, number];
  const [tx, ty, tz] = [2 * (y * p[2] - z * p[1]), 2 * (z * p[0] - x * p[2]), 2 * (x * p[1] - y * p[0])];
  return [p[0] + w * tx + (y * tz - z * ty), p[1] + w * ty + (z * tx - x * tz), p[2] + w * tz + (x * ty - y * tx)];
}
function sphere(c: V, r: V, n = 10): V[] {
  const out: V[] = [];
  for (let i = 0; i <= n; i++) for (let j = 0; j < 2 * n; j++) out.push([r[0] * Math.cos((Math.PI * i) / n), r[1] * Math.sin((Math.PI * i) / n) * Math.cos((Math.PI * j) / n), r[2] * Math.sin((Math.PI * i) / n) * Math.sin((Math.PI * j) / n)].map((x, k) => x + c[k]!) as V);
  return out;
}
const sub = (a: readonly number[], b: readonly number[]): V => [a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!];
const cross3 = (a: V, b: V): V => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V): V => {
  const n = Math.hypot(...a) || 1;
  return [a[0] / n, a[1] / n, a[2] / n];
};
function pieces(s: Json): V[][] {
  const t = (s.transform?.translation ?? [0, 0, 0]) as V;
  const q = s.transform?.rotation as number[] | undefined;
  const place = (p: V): V => (q === undefined ? p : rotate(q, p)).map((x, k) => x + t[k]!) as V;
  if (s.kind === "loft") {
    const sections = s.sections as Json[];
    const ring = (c: Json, i: number): V[] => Array.from({ length: 48 }, (_, n) => {
      const a = (2 * Math.PI * n) / 48;
      const f = (x: number): number => Math.sign(x) * Math.abs(x) ** (2 / (c.exponent ?? 2));
      if (c.rotation !== undefined) return [c.at[0], c.at[1] + c.ry * f(Math.sin(a)), c.at[2] + c.rx * f(Math.cos(a))] as V;
      const tangent = unit(sub(sections[Math.min(i + 1, sections.length - 1)]!.at, sections[Math.max(i - 1, 0)]!.at));
      const x = unit(cross3(tangent, Math.abs(tangent[2]) > 0.9 ? [1, 0, 0] : [0, 0, 1]));
      const y = cross3(tangent, x);
      const r = Math.max(c.rx, c.ry);
      return [0, 1, 2].map((k) => c.at[k] + r * (Math.cos(a) * x[k]! + Math.sin(a) * y[k]!)) as V;
    });
    return sections.slice(1).map((c, i) => [...ring(sections[i]!, i), ...ring(c, i + 1)]);
  }
  if (s.kind === "ellipsoid") return [sphere([0, 0, 0], s.radii).map(place)];
  if (s.kind === "capsule") return [[...sphere(s.p0, [s.r0, s.r0, s.r0], 8), ...sphere(s.p1, [s.r1, s.r1, s.r1], 8)]];
  if (s.kind === "box") return [[-1, 1].flatMap((a) => [-1, 1].flatMap((b) => [-1, 1].map((c) => place([(a * s.size[0]) / 2, (b * s.size[1]) / 2, (c * s.size[2]) / 2]))))];
  if (s.kind === "revolve") {
    const along = s.axis[0] === 1 ? 0 : 1;
    const ls = s.lateralScale ?? 1;
    const ring = ([a, r]: number[]): V[] => Array.from({ length: 48 }, (_, n) => {
      const p: V = [s.origin[0], s.origin[1], s.origin[2] + r! * ls * Math.sin((Math.PI * n) / 24)];
      p[along] += a!;
      p[1 - along] += r! * Math.cos((Math.PI * n) / 24);
      return p;
    });
    return (s.profile as number[][]).slice(1).map((p, i) => [...ring(s.profile[i]), ...ring(p)]);
  }
  if (s.kind === "extrude") {
    const profile = s.profile as number[][];
    const n = profile.length;
    let [area, cx, cy] = [0, 0, 0];
    profile.forEach(([x0, y0], i) => {
      const [x1, y1] = profile[(i + 1) % n] as [number, number];
      const c = x0! * y1 - x1 * y0!;
      area += c / 2;
      cx += (x0! + x1) * c;
      cy += (y0! + y1) * c;
    });
    [cx, cy] = [cx / (6 * area), cy / (6 * area)];
    const [k0, k1] = (s.taperTo ?? [1, 1]) as [number, number];
    const ring = (u: number, k: number): V[] => profile.map(([v, w]) => [u, cx + (v! - cx) * k, cy + (w! - cy) * k] as V);
    return [[...ring(s.from, k0), ...ring(s.to, k1)]];
  }
  throw new Error(`no outline for ${s.kind}`);
}

/* ---------------------------------------------------------------- outlines */

function hull(points: readonly P[]): P[] {
  const p = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const cross = (o: P, a: P, b: P): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const half = (list: readonly P[]): P[] => {
    const h: P[] = [];
    for (const q of list) {
      while (h.length >= 2 && cross(h[h.length - 2]!, h[h.length - 1]!, q) <= 0) h.pop();
      h.push(q);
    }
    h.pop();
    return h;
  };
  return [...half(p), ...half([...p].reverse())];
}
const areaOf = (r: readonly P[]): number => r.reduce((a, p, i) => a + (p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y) / 2, 0);
function simplify(ring: readonly P[], tol: number): P[] {
  const far = ring.reduce((b, p, i) => (Math.hypot(p.x - ring[0]!.x, p.y - ring[0]!.y) > Math.hypot(ring[b]!.x - ring[0]!.x, ring[b]!.y - ring[0]!.y) ? i : b), 0);
  const dp = (pts: readonly P[]): P[] => {
    if (pts.length < 3) return [...pts];
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const len = Math.hypot(b.x - a.x, b.y - a.y) || 1e-12;
    let worst = 0;
    let at = 0;
    pts.forEach((p, i) => {
      const d = Math.abs((b.x - a.x) * (a.y - p.y) - (a.x - p.x) * (b.y - a.y)) / len;
      if (d > worst) [worst, at] = [d, i];
    });
    return worst <= tol ? [a, b] : [...dp(pts.slice(0, at + 1)).slice(0, -1), ...dp(pts.slice(at))];
  };
  return [...dp(ring.slice(0, far + 1)).slice(0, -1), ...dp([...ring.slice(far), ring[0]!]).slice(0, -1)];
}
function outline(clouds: readonly V[][], project: (p: V) => P, tol: number): P[] {
  const rings = unionPolygons(clouds.map((c) => hull(c.map(project))));
  const big = rings.reduce((a, b) => (Math.abs(areaOf(b)) > Math.abs(areaOf(a)) ? b : a));
  return simplify(big, tol).map((p) => ({ x: r4(p.x), y: r4(p.y) }));
}
/** ↕️ The extent of a ring across the line x = `u`. */
function across(ring: readonly P[], u: number): [number, number] {
  const ys: number[] = [];
  ring.forEach((a, i) => {
    const b = ring[(i + 1) % ring.length]!;
    if ((a.x - u) * (b.x - u) <= 0 && a.x !== b.x) ys.push(a.y + ((b.y - a.y) * (u - a.x)) / (b.x - a.x));
  });
  return ys.length === 0 ? [0, 0] : [Math.min(...ys), Math.max(...ys)];
}
/** ✂️ The part of a ring at x >= `u` (one clip against a line). */
function above(ring: readonly P[], u: number): P[] {
  const out: P[] = [];
  ring.forEach((a, i) => {
    const b = ring[(i + 1) % ring.length]!;
    if (a.x >= u) out.push(a);
    if ((a.x - u) * (b.x - u) < 0) out.push({ x: u, y: r4(a.y + ((b.y - a.y) * (u - a.x)) / (b.x - a.x)) });
  });
  return out;
}
const pts = (ring: readonly P[]): number[][] => ring.map((p) => [p.x, p.y]);

/* ---------------------------------------------------------------- groups: which solids one drawn shape unites */

type Group = { id: string; bone: string; solids: Json[]; role: string; tone: number; region: string; material: string };
const GROUP_OF: [RegExp, string][] = [[/^(finger|fingers|fingermid|fingertip|thumb|thumbtip|palm)(-\d)?-([lr])$/, "hand-$3"], [/^clog-toe-sole-([lr])$/, "clog-toe-$1"], [/^clog-sole-([lr])$/, "clog-$1"], [/^seat-[lr]$/, "trousers-seat"], [/^(nose|lips)$/, "head"], [/^hair-bun$/, "hair"], [/^trowel-blade-(shoulder|point)$/, "trowel-blade"]];
function groups(body: readonly Json[], region: ReadonlyMap<string, string>): Group[] {
  const out = new Map<string, Group>();
  for (const s of body) {
    const rule = GROUP_OF.find(([re]) => re.test(s.id));
    const id = rule === undefined ? s.id : s.id.replace(rule[0], rule[1]);
    const g = out.get(id);
    if (g !== undefined) g.solids.push(s);
    else out.set(id, { id, bone: s.bone, solids: [s], role: s.role === "detail" && rule !== undefined ? "body" : s.role, tone: s.tone, region: region.get(s.id)!, material: s.material });
  }
  for (const g of out.values()) {
    const own = g.solids.find((s) => s.id === g.id);
    if (own !== undefined) [g.tone, g.material, g.role] = [own.tone, own.material, own.role];
  }
  return [...out.values()];
}

/* ---------------------------------------------------------------- layer order */

const PART_Z: [RegExp, number][] = [[/^thigh/, 0], [/^knee-patch/, 0.04], [/^shank/, 0.1], [/^sock-foot/, 0.14], [/^sock/, 0.16], [/^clog-toe/, 0.22], [/^clog-strap/, 0.24], [/^clog/, 0.2], [/^upper-arm/, 0.38], [/^sleeve-roll/, 0.44], [/^sleeve/, 0.42], [/^forearm/, 0.5], [/^glove-cuff/, 0.55], [/^hand/, 0.6]];
const ARMISH = /arm|hand|sleeve|glove/;
const TRUNK_Z: Record<string, number> = { "trousers-seat": 3.05, shirt: 3.2, "shirt-collar": 3.3, neck: 4.8, head: 5, "ear-l": 4.95, "ear-r": 5.6, "eye-l": 4.96, "eye-r": 5.6, hair: 5.4, "hat-brim": 5.7, "hat-crown": 5.8, "hat-band": 5.9 };
const GEAR_Z: Record<string, [elevation: number, front: number, plan: number]> = { pad: [0.5, 0.5, 0.5], trug: [0.6, 8, 0.6], "trug-handle-l": [0.52, 8.1, 0.7], "trug-handle-r": [0.7, 8.1, 0.7], "trowel-handle": [7.55, 7.2, 4.9], "trowel-neck": [7.8, 7.3, 4.86], "trowel-blade": [7.82, 7.4, 4.84] };
function layer(g: Group, view: string): number {
  if (/^weeds-/.test(g.id)) return view === "elevation" ? 0.55 : view === "front" ? 7.9 : 0.65;
  if (/^weed-/.test(g.id)) return view === "elevation" ? 0.62 : view === "front" ? 8.05 : 0.75;
  if (g.id === "hat-band" && view !== "elevation") return 6.75;
  const gear = GEAR_Z[g.id];
  if (gear !== undefined) return gear[view === "elevation" ? 0 : view === "front" ? 1 : 2];
  const part = PART_Z.find(([re]) => re.test(g.id));
  if (part === undefined) {
    const z = TRUNK_Z[g.id] ?? 5;
    return view === "elevation" ? z : g.bone === "head" ? z + 1 : z + 0.8;
  }
  const left = g.bone.endsWith("-l");
  const arm = ARMISH.test(g.id);
  if (view === "elevation") return r4((left ? (arm ? 1.6 : 1) : arm ? 7 : 6) + part[1]);
  if (view === "front") return r4((arm ? 4.6 : 2.4) + (arm ? part[1] : -part[1]));
  return r4((arm ? 4.4 : 1.6) + (arm ? part[1] : -part[1]));
}

/* ---------------------------------------------------------------- shapes of one group in the three views */

const TOL: Record<number, number> = { 50: 0.0008, 100: 0.0015 };
const REV_U = /^(thigh|knee-patch|shank|sock|sleeve|sleeve-roll|upper-arm)-[lr]$|^(forearm|glove-cuff)-r$|^(neck|hat-crown|hat-brim|hat-band|trowel-handle)$/;
/** 🔪️ Where the swept outline of a long part is cut along its bone so every piece carries its own depth. */
const SPLIT: Record<string, number[]> = {};
/** 🥚️ Parts drawn in front and plan as pieces revolved about the bone with an elliptic section (none: the 2D evaluation draws every revolved shape circular). */
const ELLIPTIC = /^$/;
const REV_V = /^(trug|weeds-.*)$/;
const SWEEP_W = /^(trug-handle-[lr]|trowel-neck|ear-[lr]|eye-[lr])$/;
const LATERAL: Record<string, number> = {};

function shapesOf(g: Group, scale: number): Record<string, Json[]> {
  const tol = TOL[scale]!;
  const clouds = g.solids.flatMap(pieces);
  const all = clouds.flat();
  const f = REST[g.bone]!;
  const base = (view: string): Json => ({ id: g.id, bone: g.bone, role: g.role, z: layer(g, view), filled: true, tone: g.tone, region: g.region, material: g.material });
  const poly = (view: string, ring: readonly P[], extra: Json = {}): Json => ({ ...base(view), ...(scale === 50 && view === "elevation" && g.id === "hat-crown" ? { hatch: "straw-weave" } : {}), kind: "poly", points: pts(ring), closed: true, ...extra });
  const elevationRing = outline(clouds, (p) => ({ x: p[0], y: p[1] }), tol);
  const elevation = [poly("elevation", elevationRing)];
  const us = all.map((p) => p[0]);
  const [u0, u1] = [Math.min(...us), Math.max(...us)];
  const band = (lo: number, hi: number, k: number): [number, number] => {
    const inside = all.filter((p) => p[0] >= lo && p[0] <= hi).map((p) => p[k]!);
    return [Math.min(...inside), Math.max(...inside)];
  };
  const ws = band(u0, u1, 2);
  const vs = band(u0, u1, 1);
  if (SWEEP_W.test(g.id) || /^weed-stalk-/.test(g.id) || g.id === "weed-leaf-0" || (scale === 50 && g.id === "weed-leaf-3")) {
    const solid = { kind: "extruded", sweep: "w", z0: r4(f.o[2]! + ws[0]), z1: r4(f.o[2]! + ws[1]) };
    return { elevation, front: [poly("front", elevationRing, { solid })], plan: [poly("plan", elevationRing, { solid })] };
  }
  if (REV_V.test(g.id)) {
    const centre = g.id === "trug" ? [0, 0] : [(u0 + u1) / 2, (ws[0] + ws[1]) / 2];
    const right = above(elevationRing, centre[0]!);
    const half = simplify(right, tol / 2);
    const round = (view: string): Json => poly(view, half, { solid: { kind: "revolved", axisBone: g.bone, axis: "v", axisX: r4(centre[0]!), axisZ: r4(centre[1]!), mirror: true } });
    return { elevation, front: [round("front")], plan: [round("plan")] };
  }
  const lateral = outline(clouds, (p) => ({ x: p[0], y: p[2] }), tol);
  if (REV_U.test(g.id) || ELLIPTIC.test(g.id)) {
    const depth = outline(clouds, (p) => ({ x: p[0], y: p[1] }), tol / 2);
    const edges = [u0, ...(ELLIPTIC.test(g.id) ? (SPLIT[g.id] ?? []) : []), u1];
    const ahead = f.v[0]!;
    const piecesOf = (view: string): Json[] => edges.slice(1).map((hi, k) => {
      const lo = edges[k]!;
      const n = Math.max(3, Math.min(14, Math.round((hi - lo) / (tol * 40))));
      const us = Array.from({ length: n + 1 }, (_, i) => Math.min(Math.max(lo + ((hi - lo) * i) / n, u0 + 1e-4), u1 - 1e-4));
      const [dv, dw] = [us.map((u) => across(depth, u)), us.map((u) => across(lateral, u))];
      const mean = (list: readonly number[]): number => list.reduce((a, b) => a + b, 0) / list.length;
      const cv = mean(dv.map((d) => (d[0] + d[1]) / 2));
      const cw = mean(dw.map((d) => (d[0] + d[1]) / 2));
      const fixed = LATERAL[g.id];
      const elliptic = fixed !== undefined || ELLIPTIC.test(g.id);
      const ls = fixed ?? mean(dw.map((d) => d[1] - d[0])) / Math.max(mean(dv.map((d) => d[1] - d[0])), 1e-6);
      const half: P[] = [{ x: r4(lo), y: r4(cv) }, ...us.map((u, i) => ({ x: r4(i === 0 ? lo : i === n ? hi : u), y: r4(cv + ((elliptic ? dv : dw)[i]![1] - (elliptic ? dv : dw)[i]![0]) / 2) })), { x: r4(hi), y: r4(cv) }];
      return { ...poly(view, simplify(half, tol / 2), { solid: { kind: "revolved", axisBone: g.bone, axis: "u", axisX: r4(cv * ahead), axisZ: r4(cw), mirror: true, ...(elliptic ? { lateralScale: r4(ls) } : {}) } }), ...(k === 0 ? {} : { id: `${g.id}-${k + 1}`, z: r4(layer(g, view) + 0.001 * k) }) };
    });
    return { elevation, front: piecesOf("front"), plan: piecesOf("plan") };
  }
  const third = (u1 - u0) / 3;
  const [a, b] = [band(u0, u0 + third, 1), band(u1 - third, u1, 1)];
  const [ua, ub] = [u0 + third / 2, u1 - third / 2];
  const L = LENGTH[g.bone]!;
  const at = (u: number, k: 0 | 1): number => r4(a[k] + ((b[k] - a[k]) * (Math.min(Math.max(u, u0), u1) - ua)) / (ub - ua));
  const flat = /^(pad|trowel-blade|weed-.*)$/.test(g.id);
  const prism = flat ? { kind: "extruded", sweep: "v", z0: r4(vs[0]), z1: r4(vs[1]) } : { kind: "extruded", sweep: "v", z0: at(0, 0), z1: at(0, 1), zTip: [at(L, 0), at(L, 1)] };
  const cuts = SPLIT[g.id];
  if (cuts === undefined) return { elevation, front: [poly("front", lateral, { solid: prism })], plan: [poly("plan", lateral, { solid: prism })] };
  const edges = [u0, ...cuts, u1];
  const piecesOf = (view: string): Json[] => edges.slice(1).map((hi, i) => {
    const lo = edges[i]!;
    const part = above(above(lateral, lo).map((p) => ({ x: -p.x, y: p.y })), -hi).map((p) => ({ x: -p.x, y: p.y }));
    const [v0, v1] = band(lo, hi, 1);
    return { ...poly(view, part, { solid: { kind: "extruded", sweep: "v", z0: r4(v0), z1: r4(v1) } }), ...(i === 0 ? {} : { id: `${g.id}-${i + 1}`, z: r4(layer(g, view) + 0.001 * i) }) };
  });
  return { elevation, front: piecesOf("front"), plan: piecesOf("plan") };
}

/* ---------------------------------------------------------------- hand-stated strokes (side elevation, bone-local) */

type Stroke = [id: string, bone: string, role: string, z: number, tone: number, region: string, material: string, points: number[][]];
function strokes(scale: number): Stroke[] {
  const side = (f: (s: string, z: number) => Stroke[]): Stroke[] => [...f("l", 1), ...f("r", 6)];
  const out: Stroke[] = [["shirt-hem", "torso", "detail", 3.31, 0.4, "shirt-torso", "linen", [[-0.07, -0.13], [-0.07, 0.09]]], ["trug-rim", "trug", "detail", 0.72, 0.3, "trug", "polyethylene", [[-0.243, 0.2], [0.243, 0.2]]], ...side((s, z) => [[`trouser-hem-${s}`, `shank-${s}`, "detail", z + 0.11, 0.25, "trousers", "canvas", [[0.35, -0.051], [0.35, 0.047]]]] as Stroke[])];
  if (scale === 50) out.push(["brow", "head", "detail", 5.65, 0.3, "face", "skin", [[0.147, -0.094], [0.145, -0.074]]], ["mouth-line", "head", "detail", 5.65, 0.3, "face", "skin", [[0.0655, -0.09], [0.067, -0.074]]], ["jaw-line", "head", "detail", 5.65, 0.55, "face", "skin", [[0.03, -0.08], [0.05, -0.032], [0.085, -0.002]]], ["shirt-placket", "torso", "texture", 3.32, 0.55, "shirt-torso", "linen", [[-0.06, -0.128], [0.1, -0.138], [0.26, -0.148], [0.4, -0.078]]], ["trowel-rib", "trowel", "detail", 7.9, 0.4, "trowel", "steel", [[0.118, -0.02], [0.25, -0.02]]], ["pad-edge", "pad", "texture", 0.51, 0.35, "pad", "eva-foam", [[-0.545, 0.03], [0.185, 0.03]]], ...side((s, z) => [[`sole-line-${s}`, `foot-${s}`, "detail", z + 0.21, 0.5, "clogs", "rubber-clog", [[-0.07, -0.052], [0, -0.0535], [0.06, -0.0562], [0.13, -0.058]]], [`toe-sole-line-${s}`, `toe-${s}`, "detail", z + 0.23, 0.5, "clogs", "rubber-clog", [[0, 0.0175], [0.05, 0.019], [0.082, 0.0225]]], [`trouser-seam-${s}`, `thigh-${s}`, "texture", z + 0.02, 0.3, "trousers", "canvas", [[0, 0.004], [0.2, 0.012], [0.4, 0.008]]], [`sleeve-seam-${s}`, `upper-arm-${s}`, "texture", z + (s === "l" ? 1.03 : 1.43), 0.6, "shirt-sleeves", "linen", [[-0.03, 0.02], [0.02, 0.05], [0.06, 0.058]]]] as Stroke[]));
  return out;
}

/* ---------------------------------------------------------------- assembly */

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of SCALES) {
  const { volume, kit } = build(asset, scale);
  const views: Record<string, Json[]> = { elevation: [], front: [], plan: [] };
  for (const g of groups(volume.body, kit.region)) for (const [view, shapes] of Object.entries(shapesOf(g, scale))) views[view]!.push(...shapes);
  for (const [id, bone, role, z, tone, region, material, points] of strokes(scale)) views.elevation!.push({ id, bone, role, z, filled: false, tone, region, material, kind: "poly", points, closed: false });
  views.plan!.push({ id: "trug-rim", bone: "trug", role: "detail", z: 0.61, filled: false, tone: 0.3, region: "trug", material: "polyethylene", kind: "ellipse", cx: REST.trug!.o[0]!, cy: 0, rx: 0.2435, ry: 0.2435 });
  for (const list of Object.values(views)) list.sort((p, q) => p.z - q.z);
  geometry[String(scale)] = views;
  console.log(scale, Object.entries(views).map(([v, l]) => `${v} ${l.length} shapes ${l.reduce((n, s) => n + (s.points?.length ?? 0), 0)} points`).join(", "));
}
const out = join(import.meta.dir, "🗑️generated", "r3-kneeling-gardener");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "geometry.json"), JSON.stringify(geometry));
