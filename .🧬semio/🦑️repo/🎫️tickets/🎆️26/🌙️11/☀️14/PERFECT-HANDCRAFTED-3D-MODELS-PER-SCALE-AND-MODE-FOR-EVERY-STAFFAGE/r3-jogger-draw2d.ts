/** ✏️ Redraws the 2D drawings of `humans/jogger` (ruling E1): every filled shape of every scale and view is the orthographic outline of the body solids of its bone, written as an economical polygon. Elevation shapes are bone-local outlines; front and plan shapes are solids of the same body (a prism of the lateral outline swept through the depth of the part, a revolved limb in plan, a hand swept through its thickness) so the three views agree in every pose; trunk and head keep a flat plan outline. Detail strokes are stated by hand at the landmarks of the model.
 * Usage: `bun r3-jogger-draw2d.ts` writes `🗑️generated/r3-jogger/geometry.json` (spliced by `r3-jogger-draw2d.py`).
 * @see ./r3-jogger-figure.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { unionPolygons } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { ASSET_FILE, SCALES, build } from "./r3-jogger-figure.ts";
import type { Json } from "./r3-jogger-kit.ts";

type P = { x: number; y: number };
type V = [number, number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };

const asset = JSON.parse(await Bun.file(ASSET_FILE).text()).asset as Json;
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
function pieces(s: Json): V[][] {
  const t = (s.transform?.translation ?? [0, 0, 0]) as V;
  const q = s.transform?.rotation as number[] | undefined;
  const place = (p: V): V => (q === undefined ? p : rotate(q, p)).map((x, k) => x + t[k]!) as V;
  if (s.kind === "loft") {
    const ring = (c: Json): V[] => Array.from({ length: 48 }, (_, i) => {
      const a = (2 * Math.PI * i) / 48;
      const f = (x: number): number => Math.sign(x) * Math.abs(x) ** (2 / (c.exponent ?? 2));
      return [c.at[0], c.at[1] + c.ry * f(Math.sin(a)), c.at[2] + c.rx * f(Math.cos(a))] as V;
    });
    return (s.sections as Json[]).slice(1).map((c, i) => [...ring(s.sections[i]), ...ring(c)]);
  }
  if (s.kind === "ellipsoid") return [sphere([0, 0, 0], s.radii).map(place)];
  if (s.kind === "capsule") return [[...sphere(s.p0, [s.r0, s.r0, s.r0], 8), ...sphere(s.p1, [s.r1, s.r1, s.r1], 8)]];
  if (s.kind === "box") return [[-1, 1].flatMap((a) => [-1, 1].flatMap((b) => [-1, 1].map((c) => place([(a * s.size[0]) / 2, (b * s.size[1]) / 2, (c * s.size[2]) / 2]))))];
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
const GROUP_OF: [RegExp, string][] = [[/^(finger|fingertip|thumb|thumbtip)(-\d)?-([lr])$/, "hand-$3"], [/^sole-toe-([lr])$/, "shoe-toe-$1"], [/^sole-([lr])$/, "shoe-$1"], [/^seat-[lr]$/, "shorts"], [/^nose$/, "head"], [/^armband-pouch$/, "armband"]];
function groups(body: readonly Json[], region: ReadonlyMap<string, string>): Group[] {
  const out = new Map<string, Group>();
  for (const s of body) {
    const rule = GROUP_OF.find(([re]) => re.test(s.id));
    const id = rule === undefined ? s.id : s.id.replace(rule[0], rule[1]);
    const g = out.get(id);
    if (g !== undefined) g.solids.push(s);
    else out.set(id, { id, bone: s.bone, solids: [s], role: s.role, tone: s.tone, region: region.get(s.id)!, material: s.material });
  }
  for (const g of out.values()) {
    const own = g.solids.find((s) => s.id === g.id);
    if (own !== undefined) [g.tone, g.material, g.role] = [own.tone, own.material, own.role];
  }
  return [...out.values()];
}

/* ---------------------------------------------------------------- layer order */

const PART_Z: [RegExp, number][] = [[/^thigh/, 0], [/^shorts-leg/, 0.05], [/^patella/, 0.08], [/^shank/, 0.1], [/^sock/, 0.15], [/^shoe-toe/, 0.22], [/^shoe/, 0.2], [/^upper-arm/, 0.38], [/^shoulder/, 0.4], [/^sleeve/, 0.42], [/^armband/, 0.44], [/^forearm/, 0.5], [/^hand/, 0.6]];
const TRUNK_Z: Record<string, number> = { shorts: 3.05, shirt: 3.2, "shirt-collar": 3.3, neck: 4.8, head: 5, "ear-l": 4.95, "ear-r": 5.6, "eye-l": 4.96, "eye-r": 5.6, hair: 5.4, headband: 5.5 };
function layer(g: Group, view: string): number {
  const part = PART_Z.find(([re]) => re.test(g.id));
  if (part === undefined) {
    const z = TRUNK_Z[g.id] ?? 5;
    return view === "elevation" ? z : view === "front" ? (g.bone === "head" ? z + 1 : z + 0.8) : g.bone === "head" ? z + 1 : z + 0.9;
  }
  const left = g.bone.endsWith("-l");
  if (view === "elevation") return r4((left ? 1 : 6) + part[1]);
  if (view === "front") return r4((/arm|hand|shoulder|sleeve/.test(g.id) ? 4.5 : 1) + part[1]);
  return r4((/arm|hand|shoulder|sleeve/.test(g.id) ? 4.2 : 1.6) - part[1] + (/arm|hand|shoulder|sleeve/.test(g.id) ? 2 * part[1] : 0));
}

/* ---------------------------------------------------------------- shapes of one group in the three views */

const TOL: Record<number, number> = { 50: 0.0008, 100: 0.0015, 200: 0.003, 500: 0.006 };
const TRUNK = new Set(["pelvis", "torso", "head"]);
const LIMB = /^(thigh|shank|upper-arm|forearm)-/;

function shapesOf(g: Group, scale: number): Record<string, Json[]> {
  const tol = TOL[scale]!;
  const clouds = g.solids.flatMap(pieces);
  const all = clouds.flat();
  const f = REST[g.bone]!;
  const world = (p: V): V => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V;
  const base = (view: string): Json => ({ id: g.id, bone: g.bone, role: g.role, z: layer(g, view), filled: true, tone: g.tone, region: g.region, material: g.material });
  const poly = (view: string, ring: readonly P[], extra: Json = {}): Json => ({ ...base(view), ...(scale === 50 && view === "elevation" && (g.id === "shirt" || g.id === "shorts") ? { hatch: g.id === "shirt" ? "knit" : "weave" } : {}), kind: "poly", points: pts(ring), closed: true, ...extra });
  const elevationRing = outline(clouds, (p) => ({ x: p[0], y: p[1] }), tol);
  const lateral = outline(clouds, (p) => ({ x: p[0], y: p[2] }), tol);
  const us = all.map((p) => p[0]);
  const [u0, u1] = [Math.min(...us), Math.max(...us)];
  const band = (lo: number, hi: number, k: number): [number, number] => {
    const inside = all.filter((p) => p[0] >= lo && p[0] <= hi).map((p) => p[k]!);
    return [Math.min(...inside), Math.max(...inside)];
  };
  const third = (u1 - u0) / 3;
  const [a, b] = [band(u0, u0 + third, 1), band(u1 - third, u1, 1)];
  const [ua, ub] = [u0 + third / 2, u1 - third / 2];
  const L = LENGTH[g.bone]!;
  const at = (u: number, k: 0 | 1): number => r4(a[k] + ((b[k] - a[k]) * (Math.min(Math.max(u, u0), u1) - ua)) / (ub - ua));
  const prism = { kind: "extruded", sweep: "v", z0: at(0, 0), z1: at(0, 1), zTip: [at(L, 0), at(L, 1)] };
  const ws = band(u0, u1, 2);
  const ARM = /^(upper-arm|forearm|hand)-/.test(g.bone);
  const elevation = [poly("elevation", elevationRing, ARM ? { solid: { kind: "extruded", sweep: "w", z0: r4(f.o[2]! + ws[0]), z1: r4(f.o[2]! + ws[1]) } } : {})];
  const front = [poly("front", lateral, { solid: prism })];
  if (g.id === "hair") {
    front[0]!.z = layer(g, "front") - 0.5;
    front.push({ ...poly("front", above(lateral, scale <= 100 ? 0.185 : 0.19), { solid: prism }), id: "hair-top" });
  }
  if (TRUNK.has(g.bone)) {
    const flat = outline(clouds.map((c) => c.map(world)), (p) => ({ x: p[0], y: p[2] }), tol);
    const plan = [g.id === "headband" ? { ...poly("plan", flat), filled: false } : poly("plan", flat)];
    if (g.id === "shirt") {
      const core = lateral.map((p) => ({ x: p.x, y: r4(Math.min(Math.max(p.y, -0.14), 0.14)) }));
      plan.unshift({ ...poly("plan", simplify(core, tol), { solid: { kind: "extruded", sweep: "v", z0: -0.06, z1: 0.07 } }), id: "shirt-core", z: layer(g, "plan") - 0.01 });
    }
    if (g.id === "hair") {
      const core = lateral.map((p) => ({ x: p.x, y: r4(Math.min(Math.max(p.y, -0.05), 0.05)) }));
      plan.unshift({ ...poly("plan", simplify(core, tol), { solid: { kind: "extruded", sweep: "v", z0: -0.05, z1: 0.07 } }), id: "hair-core", z: layer(g, "plan") - 0.01 });
    }
    return { elevation, front, plan };
  }
  if (LIMB.test(g.bone) && !/^armband/.test(g.id)) {
    const n = Math.max(4, Math.min(12, Math.round((u1 - u0) / (tol * 40))));
    const cw = (ws[0] + ws[1]) / 2;
    const cv = (band(u0, u1, 1)[0] + band(u0, u1, 1)[1]) / 2;
    const half: P[] = [{ x: r4(u0), y: r4(cv) }];
    for (let i = 0; i <= n; i++) {
      const u = u0 + ((u1 - u0) * i) / n;
      const w = across(lateral, Math.min(Math.max(u, u0 + 1e-4), u1 - 1e-4));
      half.push({ x: r4(u), y: r4(cv + (w[1] - w[0]) / 2) });
    }
    half.push({ x: r4(u1), y: r4(cv) });
    const round = (view: string): Json => poly(view, simplify(half, tol / 2), { solid: { kind: "revolved", axisBone: g.bone, axis: "u", axisX: r4(cv), axisZ: r4(cw), mirror: true } });
    return { elevation, front: [round("front")], plan: [round("plan")] };
  }
  return { elevation, front, plan: [poly("plan", lateral, { solid: prism })] };
}

/* ---------------------------------------------------------------- hand-stated strokes */

type Stroke = [id: string, bone: string, role: string, z: number, tone: number, region: string, material: string, points: number[][]];
const HY = REST.head!.o[1]!;
const TY = REST.torso!.o[1]!;
function strokes(scale: number): Record<string, Stroke[]> {
  if (scale > 100) return { elevation: [], front: [], plan: [] };
  const side = (f: (s: string, g: number, z: number) => Stroke[]): Stroke[] => [...f("l", 1, 1), ...f("r", -1, 6)];
  const hems: Stroke[] = [["shirt-hem", "torso", "detail", 3.31, 0.3, "torso-clothing", "nylon-clothing", [[-0.079, -0.119], [-0.079, 0.147]]], ...side((s, _g, z) => [[`sleeve-hem-${s}`, `upper-arm-${s}`, "detail", z + 0.43, 0.3, "arm-clothing", "nylon-clothing", [[0.152, -0.055], [0.152, 0.061]]], [`shorts-hem-${s}`, `thigh-${s}`, "detail", z + 0.06, 0.6, "leg-clothing", "nylon-clothing", [[0.154, -0.092], [0.154, 0.112]]], [`sock-top-${s}`, `shank-${s}`, "detail", z + 0.16, 0.4, "leg-skin", "cotton-clothing", [[0.39, -0.043], [0.39, 0.039]]]] as Stroke[])];
  const fine: Stroke[] = scale === 50 ? [["brow", "head", "detail", 5.6, 0.2, "head-skin", "skin", [[0.143, -0.101], [0.141, -0.078]]], ["mouth-line", "head", "detail", 5.6, 0.3, "head-skin", "skin", [[0.052, -0.1], [0.054, -0.082]]], ["nostril", "head", "detail", 5.65, 0.2, "head-skin", "skin", [[0.085, -0.117], [0.083, -0.106], [0.088, -0.101]]], ["jaw-line", "head", "detail", 5.65, 0.5, "head-skin", "skin", [[0.03, -0.088], [0.048, -0.036], [0.088, -0.004]]], ["hair-strands", "head", "texture", 5.45, 0.4, "hair", "hair", [[0.238, -0.03], [0.222, 0.03], [0.19, 0.075]]], ["hair-strands-2", "head", "texture", 5.45, 0.4, "hair", "hair", [[0.165, 0.04], [0.13, 0.07], [0.1, 0.066]]], ...side((s, _g, z) => [[`sole-${s}`, `foot-${s}`, "detail", z + 0.21, 0.1, "shoes", "leather-shoe", [[-0.07, -0.054], [0.0, -0.053], [0.07, -0.056], [0.1344, -0.06]]], [`sole-toe-${s}`, `toe-${s}`, "detail", z + 0.23, 0.1, "shoes", "leather-shoe", [[0, 0.0227], [0.06, 0.024], [0.092, 0.0255]]], [`lace-${s}`, `foot-${s}`, "texture", z + 0.3, 0.4, "shoes", "leather-shoe", [[0.03, 0.003], [0.07, -0.003], [0.11, -0.015]]], [`shorts-stripe-${s}`, `thigh-${s}`, "detail", z + 0.07, 0.75, "leg-clothing", "nylon-clothing", [[-0.01, 0.006], [0.152, 0.012]]]] as Stroke[])] : [];
  const front: Stroke[] = [["shirt-hem", "torso", "detail", 4.11, 0.3, "torso-clothing", "nylon-clothing", [[-0.18, TY - 0.079], [0.18, TY - 0.079]]], ...side((s, g) => [[`shorts-hem-${s}`, `thigh-${s}`, "detail", 1.06, 0.6, "leg-clothing", "nylon-clothing", [[0.09 * g - 0.09, -0.154], [0.09 * g + 0.09, -0.154]]], [`sock-top-${s}`, `shank-${s}`, "detail", 1.16, 0.4, "leg-skin", "cotton-clothing", [[0.09 * g - 0.034, -0.4447 - 0.39], [0.09 * g + 0.034, -0.4447 - 0.39]]]] as Stroke[]), ...(scale === 50 ? ([["nose", "head", "detail", 6.3, 0.3, "head-skin", "skin", [[0.004, HY + 0.128], [0.012, HY + 0.09], [-0.012, HY + 0.09]]], ["mouth", "head", "detail", 6.3, 0.3, "head-skin", "skin", [[-0.022, HY + 0.052], [0.022, HY + 0.052]]], ...side((s, g) => [[`brow-${s}`, "head", "detail", 6.3, 0.2, "head-skin", "skin", [[0.016 * g, HY + 0.142], [0.046 * g, HY + 0.141]]]] as Stroke[])] as Stroke[]) : [])];
  return { elevation: [...hems, ...fine], front, plan: [] };
}

/* ---------------------------------------------------------------- assembly */

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of SCALES) {
  const { volume, kit } = build(asset, scale);
  const views: Record<string, Json[]> = { elevation: [], front: [], plan: [] };
  for (const g of groups(volume.body, kit.region)) for (const [view, shapes] of Object.entries(shapesOf(g, scale))) views[view]!.push(...shapes);
  for (const [view, list] of Object.entries(strokes(scale))) for (const [id, bone, role, z, tone, region, material, points] of list) views[view]!.push({ id, bone, role, z, filled: false, tone, region, material, kind: "poly", points, closed: false, ...(view === "elevation" && /^(upper-arm|forearm|hand)-/.test(bone) ? { solid: { kind: "extruded", sweep: "w", z0: r4(REST[bone]!.o[2]! - 0.055), z1: r4(REST[bone]!.o[2]! + 0.06) } } : {}) });
  for (const list of Object.values(views)) list.sort((p, q) => p.z - q.z);
  geometry[String(scale)] = views;
  console.log(scale, Object.entries(views).map(([v, l]) => `${v} ${l.length} shapes ${l.reduce((n, s) => n + s.points.length, 0)} points`).join(", "));
}
const out = join(import.meta.dir, "🗑️generated", "r3-jogger");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "geometry.json"), JSON.stringify(geometry));
