/** 🖍️ Redraws the 2D drawings of `humans/running-child` (ruling E1): every filled shape of every scale and view is the orthographic outline of the body solids of its bone, written as an economical polygon. Elevation shapes are bone-local outlines; front and plan shapes are solids of the same body (a prism of the lateral outline swept through the depth of the part, a revolved limb in plan, a hand swept through its thickness) so the three views agree in every pose; trunk and head keep a flat plan outline. Detail strokes are stated by hand at the landmarks of the model.
 * Usage: `bun r3-running-child-draw2d.ts` writes `🗑️generated/r3-running-child/geometry.json` (spliced by `r3-running-child-apply.py`).
 * @see ./r3-running-child-figure.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { unionPolygons } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { FOOT, HAIR, PACK, SHIRT, SHOE, SHORTS, SIDES, SKIN, SOCK, STRAP, T } from "./r3-running-child-body.ts";
import { SCALES, build, readAsset } from "./r3-running-child-figure.ts";
import { STRAP_COARSE } from "./r3-running-child-lists.ts";
import type { Json } from "./r3-running-child-kit.ts";

type P = { x: number; y: number };
type V = [number, number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };

const asset = await readAsset();
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
      const [x, y] = [rotate(c.rotation, [1, 0, 0]), rotate(c.rotation, [0, 1, 0])];
      return [0, 1, 2].map((k) => c.at[k] + c.rx * f(Math.cos(a)) * x[k]! + c.ry * f(Math.sin(a)) * y[k]!) as V;
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
const GROUP_OF: [RegExp, string][] = [[/^(finger|fingertip|thumb|thumbtip)(-\d)?-([lr])$/, "hand-$3"], [/^sole-toe-([lr])$/, "shoe-toe-$1"], [/^sole-([lr])$/, "shoe-$1"], [/^seat-[lr]$/, "shorts"], [/^nose$/, "head"], [/^patella-([lr])$/, "shank-$1"], [/^pack-handle$/, "pack"]];
function groups(body: readonly Json[], region: ReadonlyMap<string, string>): Group[] {
  const out = new Map<string, Group>();
  for (const s of body) {
    if (s.slender === true) continue;
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

const PART_Z: [RegExp, number][] = [[/^thigh/, 0], [/^shorts-leg/, 0.05], [/^shank/, 0.1], [/^sock/, 0.15], [/^shoe-toe/, 0.22], [/^shoe/, 0.2], [/^(upper-arm|arm)/, 0.38], [/^sleeve/, 0.42], [/^forearm/, 0.5], [/^hand/, 0.6]];
const ARMS = /^(upper-arm|arm|sleeve|forearm|hand)-/;
const TRUNK_Z: Record<string, Record<string, number>> = {
  elevation: { "strap-l": 2.6, shorts: 3.05, pack: 3.5, "pack-pocket": 3.6, neck: 3.8, shirt: 4, collar: 4.1, "strap-r": 4.4, "ear-l": 4.95, "eye-l": 4.96, head: 5, hair: 5.4, "ear-r": 5.6, "eye-r": 5.6 },
  front: { pack: 0.5, "pack-pocket": 0.4, shorts: 3.85, shirt: 4, collar: 4.1, "strap-l": 4.4, "strap-r": 4.4, neck: 3.9, "ear-l": 5.9, "ear-r": 5.9, head: 6, hair: 5.5, "eye-l": 6.6, "eye-r": 6.6 },
  plan: { shorts: 3.9, pack: 4.05, "pack-pocket": 4.1, shirt: 4.15, collar: 4.2, "strap-l": 4.4, "strap-r": 4.4, neck: 5, "ear-l": 5.9, "ear-r": 5.9, head: 6, "eye-l": 5.8, "eye-r": 5.8, hair: 6.4 },
};
function layer(g: Group, view: string): number {
  const part = PART_Z.find(([re]) => re.test(g.id));
  if (part === undefined) return TRUNK_Z[view]![g.id] ?? 5;
  const left = g.bone.endsWith("-l");
  if (view === "elevation") return r4((left ? 1 : 6) + part[1]);
  if (view === "front") return r4((ARMS.test(g.id) ? 4.5 : 1) + part[1]);
  return r4(ARMS.test(g.id) ? 4.2 + part[1] : 1.6 - part[1]);
}

/* ---------------------------------------------------------------- shapes of one group in the three views */

const TOL: Record<number, number> = { 50: 0.0008, 100: 0.0015, 200: 0.003, 500: 0.006 };
const TRUNK = new Set(["pelvis", "torso", "neck", "head"]);
const LIMB = /^(thigh|shank|arm|forearm)-/;
/** 🌗️ The far (left) limbs are a shade darker in the side elevation so the two legs and arms read apart. */
const FAR = 0.92;

function shapesOf(g: Group, scale: number): Record<string, Json[]> {
  const tol = TOL[scale]!;
  const clouds = g.solids.flatMap(pieces);
  const all = clouds.flat();
  const f = REST[g.bone]!;
  const world = (p: V): V => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V;
  const base = (view: string): Json => ({ id: g.id, bone: g.bone, role: g.role, z: layer(g, view), filled: true, tone: view === "elevation" && /-l$/.test(g.bone) ? r4(g.tone * FAR) : g.tone, region: g.region, material: g.material });
  const poly = (view: string, ring: readonly P[], extra: Json = {}): Json => ({ ...base(view), kind: "poly", points: pts(ring), closed: true, ...extra });
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
  const ARM = /^(arm|forearm|hand)-/.test(g.bone);
  const elevation = [poly("elevation", elevationRing, ARM ? { solid: { kind: "extruded", sweep: "w", z0: r4(f.o[2]! + ws[0]), z1: r4(f.o[2]! + ws[1]) } } : {})];
  const front = [poly("front", lateral, { solid: prism })];
  if (g.id === "hair") {
    front.push({ ...poly("front", above(lateral, 0.08), { solid: prism }), id: "hair-top", z: 6.4 });
  }
  if (TRUNK.has(g.bone)) {
    const flat = outline(clouds.map((c) => c.map(world)), (p) => ({ x: p[0], y: p[2] }), tol);
    const plan = [poly("plan", flat)];
    if (g.id === "shirt") {
      const core = lateral.map((p) => ({ x: p.x, y: r4(Math.min(Math.max(p.y, -0.09), 0.09)) }));
      plan.unshift({ ...poly("plan", simplify(core, tol), { solid: { kind: "extruded", sweep: "v", z0: -0.06, z1: 0.06 } }), id: "shirt-core", z: layer(g, "plan") - 0.01 });
    }
    if (g.id === "hair") {
      const core = lateral.map((p) => ({ x: p.x, y: r4(Math.min(Math.max(p.y, -0.04), 0.04)) }));
      plan.unshift({ ...poly("plan", simplify(core, tol), { solid: { kind: "extruded", sweep: "v", z0: -0.05, z1: 0.07 } }), id: "hair-core", z: layer(g, "plan") - 0.01 });
    }
    return { elevation, front, plan };
  }
  if (LIMB.test(g.bone)) {
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
  if (/^foot-/.test(g.bone)) {
    const mid = (ws[0] + ws[1]) / 2;
    const span = { solid: { kind: "extruded", sweep: "w", z0: r4(f.o[2]! + mid + 0.88 * (ws[0] - mid)), z1: r4(f.o[2]! + mid + 0.88 * (ws[1] - mid)) } };
    return { elevation, front: [poly("front", elevationRing, span)], plan: [poly("plan", elevationRing, span)] };
  }
  return { elevation, front, plan: [poly("plan", lateral, { solid: prism })] };
}

/* ---------------------------------------------------------------- hand-stated strokes */

type Stroke = [id: string, bone: string, role: string, z: number, tone: number, region: string, material: string, points: number[][]];
const HY = REST.head!.o[1]!;
const TY = REST.torso!.o[1]!;
const FA = (FOOT * Math.PI) / 180;
/** 🦶️ A point of the upright foot axes (x ahead, y up from the ankle) in the foot bone frame of the elevation. */
const ft = (x: number, y: number): number[] => [r4(x * Math.cos(FA) - y * Math.sin(FA)), r4(x * Math.sin(FA) + y * Math.cos(FA))];
/** ✒️ The open strokes, stated by hand at the landmarks of the model: the two pack straps (slender solids are drawn as strokes), hems, sock tops, and at 1:50 the face, hair, sole and lace lines and the seam of the pack. */
function strokes(scale: number): Record<string, Stroke[]> {
  if (scale > 100) return { elevation: [], front: [], plan: [] };
  const path = scale === 50 ? STRAP : STRAP_COARSE;
  const side = (f: (s: string, g: number, z: number) => Stroke[]): Stroke[] => [...f("l", 1, 1), ...f("r", -1, 6)];
  const strap = (view: string, s: string, z: number, points: number[][]): Stroke => [`strap-${s}`, "torso", "detail", z, T.strap, "backpack", PACK, points.map(([x, y]) => [r4(x!), r4(y!)])];
  const top = path.filter(([, v]) => v <= 0.02);
  const elevation: Stroke[] = [
    strap("elevation", "l", 2.6, path.map(([u, v]) => [u, v])),
    strap("elevation", "r", 4.4, path.map(([u, v]) => [u, v])),
    ["shirt-hem", "torso", "detail", 4.05, 0.4, "tshirt", SHIRT, [[-0.066, -0.083], [-0.066, 0.103]]],
    ...side((s, _g, z) => [
      [`sleeve-hem-${s}`, `arm-${s}`, "detail", z + 0.43, 0.4, "tshirt", SHIRT, [[0.094, -0.04], [0.094, 0.042]]],
      [`shorts-hem-${s}`, `thigh-${s}`, "detail", z + 0.06, 0.62, "shorts", SHORTS, [[0.109, -0.06], [0.109, 0.065]]],
      [`sock-top-${s}`, `shank-${s}`, "detail", z + 0.16, 0.5, "socks", SOCK, [[0.227, -0.031], [0.227, 0.027]]],
    ] as Stroke[]),
  ];
  const fine: Stroke[] = scale !== 50 ? [] : [
    ["brow", "head", "detail", 5.65, 0.25, "face-skin", SKIN, [[0.046, -0.086], [0.045, -0.07]]],
    ["mouth-line", "head", "detail", 5.65, 0.3, "face-skin", SKIN, [[-0.02, -0.081], [-0.019, -0.066]]],
    ["jaw-line", "head", "detail", 5.65, 0.55, "face-skin", SKIN, [[-0.04, -0.07], [-0.027, -0.032], [-0.002, -0.006]]],
    ["hair-strands", "head", "texture", 5.45, 0.5, "hair", HAIR, [[0.127, -0.03], [0.116, 0.03], [0.09, 0.07]]],
    ["hair-strands-2", "head", "texture", 5.45, 0.5, "hair", HAIR, [[0.072, 0.05], [0.042, 0.074], [0.012, 0.064]]],
    ["pack-seam", "pack", "detail", 3.7, 0.55, "backpack", PACK, [[0.132, 0.036], [0.132, 0.123]]],
    ...side((s, _g, z) => [
      [`sole-line-${s}`, `foot-${s}`, "detail", z + 0.21, 0.35, "shoes", SHOE, [ft(-0.052, -0.043), ft(0.02, -0.043), ft(0.083, -0.043)]],
      [`sole-line-toe-${s}`, `toe-${s}`, "detail", z + 0.23, 0.35, "shoes", SHOE, [[0, 0.013], [0.04, 0.0135], [0.057, 0.0145]]],
      [`lace-${s}`, `foot-${s}`, "texture", z + 0.3, 0.45, "shoes", SHOE, [ft(0.018, -0.001), ft(0.045, -0.006), ft(0.072, -0.014)]],
    ] as Stroke[]),
  ];
  const front: Stroke[] = [
    ...SIDES.map(([s, g]) => strap("front", s, 4.4, top.map(([u, , w]) => [w * g, TY + u]))),
    ["shirt-hem", "torso", "detail", 4.05, 0.4, "tshirt", SHIRT, [[-0.114, TY - 0.066], [0.114, TY - 0.066]]],
    ...(scale !== 50 ? [] : ([
      ["nose", "head", "detail", 6.7, 0.35, "face-skin", SKIN, [[0.001, HY + 0.027], [0.008, HY + 0.004], [-0.008, HY + 0.004]]],
      ["mouth", "head", "detail", 6.7, 0.3, "face-skin", SKIN, [[-0.015, HY - 0.02], [0.015, HY - 0.02]]],
      ...SIDES.map(([s, g]) => [`brow-${s}`, "head", "detail", 6.7, 0.25, "face-skin", SKIN, [[0.013 * g, HY + 0.047], [0.038 * g, HY + 0.046]]] as Stroke),
    ] as Stroke[])),
  ];
  const plan: Stroke[] = SIDES.map(([s, g]) => strap("plan", s, 4.4, path.filter(([u]) => u >= 0.19).map(([, v, w]) => [-v, w * g])));
  return { elevation: [...elevation, ...fine], front, plan };
}

/* ---------------------------------------------------------------- assembly */

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of SCALES) {
  const { volume, kit } = build(asset, scale);
  const views: Record<string, Json[]> = { elevation: [], front: [], plan: [] };
  for (const g of groups(volume.body, kit.region)) for (const [view, shapes] of Object.entries(shapesOf(g, scale))) views[view]!.push(...shapes);
  for (const [view, list] of Object.entries(strokes(scale))) for (const [id, bone, role, z, tone, region, material, points] of list) views[view]!.push({ id, bone, role, z, filled: false, tone, region, material, kind: "poly", points, closed: false, ...(view === "elevation" && /^(arm|forearm|hand)-/.test(bone) ? { solid: { kind: "extruded", sweep: "w", z0: r4(REST[bone]!.o[2]! - 0.03), z1: r4(REST[bone]!.o[2]! + 0.03) } } : {}) });
  for (const list of Object.values(views)) list.sort((p, q) => p.z - q.z);
  geometry[String(scale)] = views;
  console.log(scale, Object.entries(views).map(([v, l]) => `${v} ${l.length} shapes ${l.reduce((n, s) => n + s.points.length, 0)} points`).join(", "));
}
const out = join(import.meta.dir, "🗑️generated", "r3-running-child");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "geometry.json"), JSON.stringify(geometry));
