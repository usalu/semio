/** ✏️ Redraws the 2D drawings of `humans/runner-with-blade-prosthesis` (ruling E1): every filled shape of every scale and view is the orthographic outline of the body solids of its part, written as an economical polygon. Elevation shapes are bone-local outlines; front and plan shapes are solids of the same body (a prism of the lateral outline swept through the depth of the part, a revolved limb, the blade as its side profile swept through its width), so the three views agree in every pose; trunk, neck and head keep a flat plan outline over a prism core. Detail strokes are stated by hand at the landmarks of the model.
 * Usage: `bun r3-runner-draw2d.ts` writes `🗑️generated/r3-runner-with-blade-prosthesis/geometry.json` (read by `r3-runner-roots.ts`).
 * @see ./r3-runner-figure.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { unionPolygons } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { SCALES, build, loadAsset } from "./r3-runner-figure.ts";
import { OUT } from "./r3-runner-roots.ts";
import type { Json, Kit } from "./r3-runner-kit.ts";

type P = { x: number; y: number };
type V = [number, number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };

const asset = loadAsset();
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
function pieces(s: Json, kit: Kit): V[][] {
  const t = (s.transform?.translation ?? [0, 0, 0]) as V;
  const q = s.transform?.rotation as number[] | undefined;
  const place = (p: V): V => (q === undefined ? p : rotate(q, p)).map((x, k) => x + t[k]!) as V;
  if (s.kind === "loft") {
    const ring = (c: Json): V[] => Array.from({ length: 48 }, (_, i) => {
      const a = (2 * Math.PI * i) / 48;
      const f = (x: number): number => Math.sign(x) * Math.abs(x) ** (2 / (c.exponent ?? 2));
      return rotate(c.rotation, [c.rx * f(Math.cos(a)), c.ry * f(Math.sin(a)), 0]).map((x, k) => x + c.at[k]) as V;
    });
    return (s.sections as Json[]).slice(1).map((c, i) => [...ring(s.sections[i]), ...ring(c)]);
  }
  if (s.kind === "ellipsoid") return [sphere([0, 0, 0], s.radii).map(place)];
  if (s.kind === "capsule") return [[...sphere(s.p0, [s.r0, s.r0, s.r0], 8), ...sphere(s.p1, [s.r1, s.r1, s.r1], 8)]];
  if (s.kind === "box") return [[-1, 1].flatMap((a) => [-1, 1].flatMap((b) => [-1, 1].map((c) => place([(a * s.size[0]) / 2, (b * s.size[1]) / 2, (c * s.size[2]) / 2]))))];
  if (s.kind === "extrude") return kit.convex.get(s.id)!.map((piece) => piece.map((p) => [...p] as V));
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
const below = (ring: readonly P[], u: number): P[] => above(ring.map((p) => ({ x: -p.x, y: p.y })), -u).map((p) => ({ x: -p.x, y: p.y }));
/** ✂️ Stations along the bone where the front prism of a trunk part is cut into bands, each with its own depth. */
const CUTS: Record<string, number[]> = {};
const pts = (ring: readonly P[]): number[][] => ring.map((p) => [p.x, p.y]);

/* ---------------------------------------------------------------- groups: which solids one drawn shape unites */

type Group = { id: string; bone: string; solids: Json[]; role: string; tone: number; region: string; material: string };
const GROUP_OF: [RegExp, string][] = [[/^(finger|fingertip)-\d-([lr])$/, "fingers-$1"], [/^(thumb|thumbtip)-([lr])$/, "hand-$2"], [/^sole-toe-l$/, "shoe-toe-l"], [/^sole-l$/, "shoe-l"], [/^seat-[lr]$/, "shorts"], [/^nose$/, "head"], [/^patella-l$/, "shank-l"], [/^liner-patella$/, "liner-knee"], [/^socket-trim$/, "socket"]];
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

const PART_Z: [RegExp, number][] = [[/^thigh/, 0], [/^shorts-leg/, 0.05], [/^liner-thigh/, 0.06], [/^shank/, 0.1], [/^liner-knee/, 0.1], [/^sock-(l|r)$/, 0.15], [/^socket-adapter/, 0.13], [/^socket-pyramid/, 0.14], [/^socket/, 0.16], [/^shoe-toe/, 0.22], [/^shoe/, 0.2], [/^blade-upper/, 0.11], [/^blade-curl/, 0.1], [/^blade-toe/, 0.09], [/^arm/, 0.4], [/^forearm/, 0.5], [/^hand/, 0.6], [/^fingers/, 0.62]];
const TRUNK_Z: Record<string, number> = { shorts: 3.05, shoulders: 3.1, singlet: 3.2, neck: 4.8, head: 5, "ear-l": 4.95, "ear-r": 5.6, "eye-l": 4.96, "eye-r": 5.6, hair: 5.4 };
const UPPER = /^(arm|forearm|hand|fingers)-/;
function layer(g: Group, view: string): number {
  const part = PART_Z.find(([re]) => re.test(g.id));
  if (part === undefined) {
    const z = TRUNK_Z[g.id] ?? 5;
    return view === "elevation" ? z : g.bone === "head" ? z + 1 : view === "front" ? z + 0.8 : z + 0.9;
  }
  const left = g.bone.endsWith("-l");
  const arm = UPPER.test(g.id);
  if (view === "elevation") return r4((left ? 1 : 6) + part[1]);
  if (view === "front") return r4((arm ? 4.5 : 1) + part[1]);
  return r4(arm ? 4.2 + part[1] : 1.6 - part[1]);
}

/* ---------------------------------------------------------------- shapes of one group in the three views */

const TOL: Record<number, number> = { 50: 0.0008, 100: 0.0015, 200: 0.003, 500: 0.006 };
const TRUNK = new Set(["pelvis", "torso", "neck", "head"]);
const LIMB = /^(thigh|shank|arm|forearm)-/;
const BLADE = /^blade-/;

function shapesOf(g: Group, scale: number, kit: Kit): Record<string, Json[]> {
  const tol = TOL[scale]!;
  const clouds = g.solids.flatMap((s) => pieces(s, kit));
  const all = clouds.flat();
  const f = REST[g.bone]!;
  const world = (p: V): V => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V;
  const hatch = scale === 50 && g.id === "socket" ? { hatch: "carbon" } : {};
  const base = (view: string): Json => ({ id: g.id, bone: g.bone, role: g.role, z: layer(g, view), filled: true, tone: g.tone, region: g.region, material: g.material });
  const poly = (view: string, ring: readonly P[], extra: Json = {}): Json => ({ ...base(view), ...(view === "elevation" ? hatch : {}), kind: "poly", points: pts(ring), closed: true, ...extra });
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
  const own = g.solids.find((x) => x.id === g.id);
  const ownW = own === undefined || !/^hand-/.test(g.id) ? undefined : pieces(own, kit).flat().map((p) => p[2]);
  const ws = ownW === undefined ? band(u0, u1, 2) : ([Math.min(...ownW), Math.max(...ownW)] as [number, number]);
  const span = { kind: "extruded", sweep: "w", z0: r4(f.o[2]! + ws[0]), z1: r4(f.o[2]! + ws[1]) };
  if (BLADE.test(g.bone)) return { elevation: [poly("elevation", elevationRing, { solid: span })], front: [poly("front", elevationRing, { solid: span })], plan: [poly("plan", elevationRing, { solid: span })] };
  const elevation = [scale >= 100 && /^hand-/.test(g.id) ? poly("elevation", lateral, { solid: prism }) : poly("elevation", elevationRing, UPPER.test(g.bone) ? { solid: span } : {})];
  const cuts = scale === 500 ? [] : (CUTS[g.id] ?? []);
  const front = cuts.length === 0 ? [poly("front", lateral, { solid: prism })] : [u0 - 1, ...cuts, u1 + 1].slice(1).map((hi, i) => {
    const lo = [u0 - 1, ...cuts][i]!;
    const [a0, a1] = [Math.max(lo, u0), Math.min(hi, u1)];
    const d = (a1 - a0) / 4;
    const [p, q] = [across(elevationRing, a0 + d / 2), across(elevationRing, a1 - d / 2)];
    const lin = (u: number, k: 0 | 1): number => r4(p[k] + ((q[k] - p[k]) * (u - a0 - d / 2)) / (a1 - a0 - d));
    const ring = below(above(lateral, a0), a1);
    return { ...poly("front", ring, { solid: { kind: "extruded", sweep: "v", z0: lin(0, 0), z1: lin(0, 1), zTip: [lin(L, 0), lin(L, 1)] } }), id: i === 0 ? g.id : `${g.id}-${i + 1}` };
  });
  if (g.id === "hair") {
    front[0]!.z = layer(g, "front") - 0.5;
    front.push({ ...poly("front", above(lateral, scale <= 100 ? 0.093 : 0.098), { solid: prism }), id: "hair-top" });
  }
  if (TRUNK.has(g.bone)) {
    const flat = outline(clouds.map((c) => c.map(world)), (p) => ({ x: p[0], y: p[2] }), tol);
    const plan = [poly("plan", flat)];
    const core = (id: string, half: number, z0: number, z1: number): void => {
      const ring = lateral.map((p) => ({ x: p.x, y: r4(Math.min(Math.max(p.y, -half), half)) }));
      plan.unshift({ ...poly("plan", simplify(ring, tol), { solid: { kind: "extruded", sweep: "v", z0, z1 } }), id, z: layer(g, "plan") - 0.01 });
    };
    if (g.id === "singlet") core("singlet-core", 0.13, -0.06, 0.07);
    if (g.id === "hair") core("hair-core", 0.05, -0.05, 0.06);
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
  return { elevation, front, plan: [poly("plan", lateral, { solid: prism })] };
}

/* ---------------------------------------------------------------- hand-stated strokes */

type Stroke = [id: string, bone: string, role: string, z: number, tone: number, region: string, material: string, points: number[][]];
const HY = REST.head!.o[1]!;
const TY = REST.torso!.o[1]!;
function strokes(scale: number): Record<string, Stroke[]> {
  if (scale > 100) return { elevation: [], front: [], plan: [] };
  const legs = (f: (s: string, z: number) => Stroke[]): Stroke[] => [...f("l", 1), ...f("r", 6)];
  const hems: Stroke[] = [
    ["singlet-hem", "torso", "detail", 3.31, 0.3, "singlet", "running-singlet", [[-0.072, -0.11], [-0.072, 0.138]]],
    ["singlet-armhole", "torso", "detail", 3.32, 0.45, "singlet", "running-singlet", [[0.478, 0.022], [0.44, 0.05], [0.39, 0.052], [0.356, 0.024], [0.352, -0.02], [0.384, -0.056], [0.436, -0.052], [0.476, -0.012]]],
    ...legs((s, z) => [[`shorts-hem-${s}`, `thigh-${s}`, "detail", z + 0.07, 0.6, "shorts", "compression-shorts", [[0.12, -0.083], [0.12, 0.103]]]] as Stroke[]),
    ["sock-top", "shank-l", "detail", 1.17, 0.4, "running-shoe", "cotton-sock", [[0.383, -0.043], [0.383, 0.039]]],
    ["liner-top", "thigh-r", "detail", 6.08, 0.3, "socket", "silicone-liner", [[0.341, -0.064], [0.341, 0.08]]],
  ];
  const fine: Stroke[] = scale === 50 ? [
    ["brow", "head", "detail", 5.6, 0.2, "face-skin", "skin", [[0.047, -0.109], [0.045, -0.086]]],
    ["mouth-line", "head", "detail", 5.6, 0.3, "face-skin", "skin", [[-0.042, -0.108], [-0.04, -0.09]]],
    ["nostril", "head", "detail", 5.65, 0.2, "face-skin", "skin", [[-0.011, -0.125], [-0.013, -0.114], [-0.008, -0.109]]],
    ["jaw-line", "head", "detail", 5.65, 0.5, "face-skin", "skin", [[-0.066, -0.096], [-0.048, -0.044], [-0.008, -0.012]]],
    ["hair-strands", "head", "texture", 5.45, 0.4, "hair", "hair", [[0.142, -0.038], [0.126, 0.022], [0.094, 0.067]]],
    ["hair-strands-2", "head", "texture", 5.45, 0.4, "hair", "hair", [[0.069, 0.032], [0.034, 0.062], [0.004, 0.058]]],
    ["singlet-seam", "torso", "texture", 3.33, 0.6, "singlet", "running-singlet", [[-0.072, 0.014], [0.09, 0.0], [0.23, -0.004], [0.345, 0.0]]],
    ["shorts-seam-l", "thigh-l", "texture", 1.08, 0.55, "shorts", "compression-shorts", [[-0.02, 0.008], [0.12, 0.011]]],
    ["shorts-seam-r", "thigh-r", "texture", 6.08, 0.55, "shorts", "compression-shorts", [[-0.02, 0.008], [0.12, 0.011]]],
    ["calf-line", "shank-l", "texture", 1.12, 0.5, "leg-skin", "skin", [[0.03, -0.04], [0.12, -0.05], [0.21, -0.036], [0.3, -0.022]]],
    ["socket-seam", "shank-r", "texture", 6.2, 0.5, "socket", "carbon-socket", [[0.03, -0.004], [0.09, -0.006], [0.148, -0.002]]],
    ["socket-brim", "shank-r", "detail", 6.21, 0.55, "socket", "carbon-socket", [[0.015, 0.066], [0.045, -0.066]]],
  ] : [];
  const front: Stroke[] = [
    ["singlet-hem", "torso", "detail", 4.11, 0.3, "singlet", "running-singlet", [[-0.17, TY - 0.072], [0.17, TY - 0.072]]],
    ...(scale === 50 ? ([["nose", "head", "detail", 6.3, 0.3, "face-skin", "skin", [[0.004, HY + 0.032], [0.012, HY - 0.006], [-0.012, HY - 0.006]]], ["mouth", "head", "detail", 6.3, 0.3, "face-skin", "skin", [[-0.022, HY - 0.042], [0.022, HY - 0.042]]], ["brow-l", "head", "detail", 6.3, 0.2, "face-skin", "skin", [[0.016, HY + 0.046], [0.046, HY + 0.045]]], ["brow-r", "head", "detail", 6.3, 0.2, "face-skin", "skin", [[-0.016, HY + 0.046], [-0.046, HY + 0.045]]]] as Stroke[]) : []),
  ];
  return { elevation: [...hems, ...fine], front, plan: [] };
}

/* ---------------------------------------------------------------- assembly */

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of SCALES) {
  const { volume, kit } = build(asset, scale);
  const views: Record<string, Json[]> = { elevation: [], front: [], plan: [] };
  for (const g of groups(volume.body, kit.region)) for (const [view, shapes] of Object.entries(shapesOf(g, scale, kit))) views[view]!.push(...shapes);
  for (const [view, list] of Object.entries(strokes(scale))) for (const [id, bone, role, z, tone, region, material, points] of list) views[view]!.push({ id, bone, role, z, filled: false, tone, region, material, kind: "poly", points, closed: false });
  for (const list of Object.values(views)) list.sort((p, q) => p.z - q.z);
  geometry[String(scale)] = views;
  console.log(scale, Object.entries(views).map(([v, l]) => `${v} ${l.length} shapes ${l.reduce((n, s) => n + s.points.length, 0)} points`).join(", "));
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "geometry.json"), JSON.stringify(geometry));
