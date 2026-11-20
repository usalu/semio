/** ✏️ Redraws the 2D drawings of `humans/standing-adult` (ruling E1): every filled shape of every scale and view is the orthographic outline of the body solids of its part at that scale, written as an economical polygon in the frame of its bone; the inner lines come hand-stated from `r3_standing_adult_strokes.py`.
 *
 * Side elevation: bone-local outlines (arm parts as prisms swept across, because the arms spread). Front elevation: the lateral outline of every part as a prism swept through its depth, so the view is exact in the default pose and follows every pose. Plan: arm and leg segments as solids of revolution about their bone with the lateral half-width as radius, hands as prisms, trunk, head and feet as flat outlines carried at the depth of their elevation twin.
 * Usage: `bun r3-standing-adult-2d.ts` writes `🗑️generated/r3-standing-adult/shapes.json` (read by `r3-standing-adult-roots.py`).
 * @see ./round3-brief.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { unionPolygons } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { frames, type Json } from "./volumes-lib.ts";
import { OUT, draftAsset } from "./r3-standing-adult-view.ts";

type P = { x: number; y: number };
type V = [number, number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };

const asset = draftAsset();
const REST = frames(asset) as Record<string, Frame>;
const LENGTH = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, b.lengthM as number]));
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

function rotate(q: readonly number[], p: V): V {
  const [x, y, z, w] = q as [number, number, number, number];
  const [tx, ty, tz] = [2 * (y * p[2] - z * p[1]), 2 * (z * p[0] - x * p[2]), 2 * (x * p[1] - y * p[0])];
  return [p[0] + w * tx + (y * tz - z * ty), p[1] + w * ty + (z * tx - x * tz), p[2] + w * tz + (x * ty - y * tx)];
}
function ball(c: V, r: V, n = 12): V[] {
  const out: V[] = [];
  for (let i = 0; i <= n; i++) for (let j = 0; j < 2 * n; j++) out.push([r[0] * Math.cos((Math.PI * i) / n), r[1] * Math.sin((Math.PI * i) / n) * Math.cos((Math.PI * j) / n), r[2] * Math.sin((Math.PI * i) / n) * Math.sin((Math.PI * j) / n)].map((x, k) => x + c[k]!) as V);
  return out;
}
/** 🧩 The convex pieces of a solid as point clouds in the frame of its bone. */
function pieces(s: Json): V[][] {
  const t = (s.transform?.translation ?? [0, 0, 0]) as V;
  const q = s.transform?.rotation as number[] | undefined;
  const place = (p: V): V => (q === undefined ? p : rotate(q, p)).map((x, k) => x + t[k]!) as V;
  if (s.kind === "loft") {
    const ring = (c: Json): V[] => Array.from({ length: 64 }, (_, i) => {
      const a = (2 * Math.PI * i) / 64;
      const f = (x: number): number => Math.sign(x) * Math.abs(x) ** (2 / (c.exponent ?? 2));
      return rotate(c.rotation ?? [0, 0, 0, 1], [c.rx * f(Math.cos(a)), c.ry * f(Math.sin(a)), 0]).map((x, k) => x + c.at[k]) as V;
    });
    return (s.sections as Json[]).slice(1).map((c, i) => [...ring(s.sections[i]), ...ring(c)]);
  }
  if (s.kind === "ellipsoid") return [ball([0, 0, 0], s.radii).map(place)];
  if (s.kind === "capsule") return [[...ball(s.p0, [s.r0, s.r0, s.r0], 10), ...ball(s.p1, [s.r1, s.r1, s.r1], 10)]];
  if (s.kind === "box") return [[-1, 1].flatMap((a) => [-1, 1].flatMap((b) => [-1, 1].map((c) => place([(a * s.size[0]) / 2, (b * s.size[1]) / 2, (c * s.size[2]) / 2]))))];
  throw new Error(`no outline for ${s.kind}`);
}

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
function largest(rings: readonly P[][]): P[] {
  return rings.reduce((a, b) => (Math.abs(areaOf(b)) > Math.abs(areaOf(a)) ? b : a));
}
function outline(clouds: readonly V[][], project: (p: V) => P, tol: number): P[] {
  return simplify(largest(unionPolygons(clouds.map((c) => hull(c.map(project))))), tol).map((p) => ({ x: r4(p.x), y: r4(p.y) }));
}
function across(ring: readonly P[], u: number): [number, number] {
  const ys: number[] = [];
  ring.forEach((a, i) => {
    const b = ring[(i + 1) % ring.length]!;
    if ((a.x - u) * (b.x - u) <= 0 && a.x !== b.x) ys.push(a.y + ((b.y - a.y) * (u - a.x)) / (b.x - a.x));
  });
  return ys.length === 0 ? [0, 0] : [Math.min(...ys), Math.max(...ys)];
}
/** ✂️ A ring clipped to the convex region left of every directed edge of `clip` (counter-clockwise). */
function clipConvex(ring: readonly P[], clip: readonly P[]): P[] {
  let out = [...ring];
  clip.forEach((a, i) => {
    const b = clip[(i + 1) % clip.length]!;
    const side = (p: P): number => (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    const next: P[] = [];
    out.forEach((p, k) => {
      const q = out[(k + 1) % out.length]!;
      const [sp, sq] = [side(p), side(q)];
      if (sp >= 0) next.push(p);
      if ((sp > 0 && sq < 0) || (sp < 0 && sq > 0)) next.push({ x: p.x + ((q.x - p.x) * sp) / (sp - sq), y: p.y + ((q.y - p.y) * sp) / (sp - sq) });
    });
    out = next;
  });
  return out;
}
const pts = (ring: readonly P[]): number[][] => ring.map((p) => [p.x, p.y]);

type Spec = { id: string; solids: string[]; role?: string; tone?: number; region: string; z: [number, number, number]; views?: string; plan?: "flat" | "round" | "prism"; extraPlan?: string[]; hatch?: string; fringe?: boolean };
const TOL: Record<number, number> = { 50: 0.0008, 100: 0.0015, 200: 0.003, 500: 0.006 };
const ARCH: [number, number][] = [[-0.09, 1.640], [-0.062, 1.663], [-0.032, 1.674], [0, 1.677], [0.032, 1.674], [0.062, 1.663], [0.09, 1.640]];

function sided(make: (s: string, leg: number, arm: number) => Spec[]): Spec[] {
  return [...make("l", 1, 0.5), ...make("r", 3, 6)];
}

function specs(scale: number): Spec[] {
  const fingers = (s: string): string[] => [0, 1, 2, 3].flatMap((i) => [`finger-${s}-${i}a`, `finger-${s}-${i}b`]);
  if (scale === 50)
    return [
      ...sided((s, leg, arm) => [
        { id: `sleeve-${s}`, solids: [`sleeve-${s}`], region: "hoodie", z: [arm, 4.5, 2.3], plan: "round", hatch: "knit" },
        { id: `forearm-${s}`, solids: [`forearm-${s}`], region: "hoodie", z: [arm + 0.01, 4.52, 2.2], plan: "round", hatch: "knit" },
        { id: `cuff-rib-${s}`, solids: [`cuff-rib-${s}`], region: "hoodie", z: [arm + 0.03, 4.53, 2.1], plan: "round" },
        { id: `hand-${s}`, solids: [`hand-${s}`, `thenar-${s}`, ...fingers(s)], region: "hand-skin", z: [arm + 0.2, 4.7, 2.0], plan: "prism" },
        { id: `thumb-${s}`, solids: [`thumb-${s}-0`, `thumb-${s}-1`], region: "hand-skin", z: [arm + 0.22, 4.72, 2.01], plan: "prism" },
        { id: `thigh-${s}`, solids: [`thigh-${s}`, `kneecap-${s}`], region: "jeans", z: [leg, 1, 1.4], plan: "round", hatch: "denim" },
        { id: `shank-${s}`, solids: [`shank-${s}`], region: "jeans", z: [leg + 0.02, 1.02, 1.3], plan: "round", hatch: "denim" },
        { id: `sock-${s}`, solids: [`sock-${s}`], region: "sneakers", z: [leg + 0.01, 1.01, 1.2], views: "ef" },
        { id: `shoe-${s}`, solids: [`shoe-${s}`], region: "sneakers", z: [leg + 0.1, 1.1, 1.1] },
        { id: `sole-${s}`, solids: [`sole-${s}`], region: "sneakers", z: [leg + 0.15, 1.15, 1.0] },
        { id: `shoe-toe-${s}`, solids: [`shoe-toe-${s}`], region: "sneakers", z: [leg + 0.1, 1.1, 1.1], views: "e" },
        { id: `toe-cap-${s}`, solids: [`toe-cap-${s}`], region: "sneakers", z: [leg + 0.12, 1.12, 1.12] },
        { id: `toe-sole-${s}`, solids: [`toe-sole-${s}`], region: "sneakers", z: [leg + 0.15, 1.15, 1.0] },
      ]),
      { id: "jeans-seat", solids: ["jeans-seat", "buttock-l", "buttock-r"], region: "jeans", z: [2, 0.9, 0], views: "ef", hatch: "denim" },
      { id: "neck", solids: ["neck"], region: "face-skin", z: [3.8, 3.8, 4.8] },
      { id: "hood", solids: ["hood"], region: "hoodie", z: [3.9, 3.9, 4.05] },
      { id: "hoodie", solids: ["hoodie", "collar-rib", "kangaroo-pocket"], region: "hoodie", z: [4, 4, 4], hatch: "knit" },
      { id: "hood-roll", solids: ["hood-roll-l", "hood-roll-r"], region: "hoodie", tone: 0.4, z: [4.05, 4.05, 4.1] },
      { id: "hem-rib", solids: ["hem-rib"], region: "hoodie", z: [4.1, 4.1, 0], views: "ef" },
      { id: "ear-l", solids: ["ear-l"], region: "face-skin", z: [4.9, 4.95, 5.9], views: "fp" },
      { id: "head", solids: ["head", "nose", "chin", "lips", "brow-l", "brow-r"], region: "face-skin", z: [5, 5, 6] },
      { id: "ear-r", solids: ["ear-r"], region: "face-skin", z: [5.6, 4.95, 5.9], views: "fp" },
      { id: "ear", solids: ["ear-r"], region: "face-skin", z: [5.6, 0, 0], views: "e" },
      { id: "eye", solids: ["eye-r"], region: "face-skin", z: [5.3, 0, 0], views: "e" },
      { id: "eye-l", solids: ["eye-l"], region: "face-skin", z: [0, 5.3, 0], views: "f" },
      { id: "eye-r", solids: ["eye-r"], region: "face-skin", z: [0, 5.3, 0], views: "f" },
      { id: "hair", solids: ["hair"], region: "hair", z: [5.5, 4.9, 6.1], fringe: true },
    ];
  if (scale === 100)
    return [
      ...sided((s, leg, arm) => [
        { id: `sleeve-${s}`, solids: [`sleeve-${s}`], region: "hoodie", z: [arm, 4.5, 2.3], plan: "round" },
        { id: `forearm-${s}`, solids: [`forearm-${s}`], region: "hoodie", z: [arm + 0.01, 4.52, 2.2], plan: "round" },
        { id: `hand-${s}`, solids: [`hand-${s}`], region: "hand-skin", z: [arm + 0.2, 4.7, 2.0], plan: "prism" },
        { id: `thumb-${s}`, solids: [`thumb-${s}`], region: "hand-skin", z: [arm + 0.22, 4.72, 2.01], plan: "prism" },
        { id: `thigh-${s}`, solids: [`thigh-${s}`], region: "jeans", z: [leg, 1, 1.4], plan: "round", hatch: "denim" },
        { id: `shank-${s}`, solids: [`shank-${s}`], region: "jeans", z: [leg + 0.02, 1.02, 1.3], plan: "round", hatch: "denim" },
        { id: `shoe-${s}`, solids: [`shoe-${s}`], region: "sneakers", z: [leg + 0.1, 1.1, 1.1] },
        { id: `shoe-toe-${s}`, solids: [`shoe-toe-${s}`], region: "sneakers", z: [leg + 0.1, 1.1, 1.1] },
      ]),
      { id: "jeans-seat", solids: ["jeans-seat"], region: "jeans", z: [2, 0.9, 0], views: "ef", hatch: "denim" },
      { id: "neck", solids: ["neck"], region: "face-skin", z: [3.8, 3.8, 4.8] },
      { id: "hood", solids: ["hood"], region: "hoodie", z: [3.9, 3.9, 4.05] },
      { id: "hoodie", solids: ["hoodie"], region: "hoodie", z: [4, 4, 4] },
      { id: "ear-l", solids: ["ear-l"], region: "face-skin", z: [4.9, 4.95, 5.9], views: "f" },
      { id: "head", solids: ["head", "nose"], region: "face-skin", z: [5, 5, 6] },
      { id: "ear-r", solids: ["ear-r"], region: "face-skin", z: [5.6, 4.95, 5.9], views: "f" },
      { id: "ear", solids: ["ear-r"], region: "face-skin", z: [5.6, 0, 0], views: "e" },
      { id: "hair", solids: ["hair"], region: "hair", z: [5.5, 4.9, 6.1], fringe: true },
    ];
  if (scale === 200)
    return [
      ...sided((s, leg, arm) => [
        { id: `sleeve-${s}`, solids: [`sleeve-${s}`], region: "hoodie", z: [arm, 4.5, 2.3], plan: "round" },
        { id: `forearm-${s}`, solids: [`forearm-${s}`], region: "hoodie", z: [arm + 0.01, 4.52, 2.2], plan: "round" },
        { id: `hand-${s}`, solids: [`hand-${s}`], region: "hand-skin", z: [arm + 0.2, 4.7, 2.0], plan: "prism" },
        { id: `thigh-${s}`, solids: [`thigh-${s}`], region: "jeans", z: [leg, 1, 1.4], plan: "round" },
        { id: `shank-${s}`, solids: [`shank-${s}`], region: "jeans", z: [leg + 0.02, 1.02, 1.3], plan: "round" },
        { id: `shoe-${s}`, solids: [`shoe-${s}`], region: "sneakers", tone: 0.15, z: [leg + 0.1, 1.1, 1.1] },
        { id: `shoe-toe-${s}`, solids: [`shoe-toe-${s}`], region: "sneakers", tone: 0.15, z: [leg + 0.1, 1.1, 1.1] },
      ]),
      { id: "jeans-seat", solids: ["jeans-seat"], region: "jeans", z: [2, 0.9, 0], views: "ef" },
      { id: "neck", solids: ["neck"], region: "face-skin", z: [3.8, 3.8, 4.8] },
      { id: "hood", solids: ["hood"], region: "hoodie", z: [3.9, 3.9, 4.05] },
      { id: "hoodie", solids: ["hoodie"], region: "hoodie", z: [4, 4, 4] },
      { id: "head", solids: ["head"], region: "face-skin", z: [5, 5, 6] },
      { id: "hair", solids: ["hair"], region: "hair", z: [5.5, 4.9, 6.1], fringe: true },
    ];
  return [
    ...sided((s, leg, arm) => [
      { id: `sil-arm-${s}`, solids: [`sleeve-${s}`], region: "hoodie", tone: 0.4, z: [arm, 4.5, 2.3], plan: "round" },
      { id: `sil-forearm-${s}`, solids: [`forearm-${s}`], region: "hoodie", tone: 0.4, z: [arm + 0.01, 4.52, 2.2], plan: "round" },
      { id: `sil-hand-${s}`, solids: [`hand-${s}`], region: "hand-skin", tone: 0.3, z: [arm + 0.3, 4.55, 2.0], plan: "prism" },
      { id: `sil-thigh-${s}`, solids: [`thigh-${s}`], region: "jeans", tone: 0.6, z: [leg, 1, 1.4], plan: "round" },
      { id: `sil-shank-${s}`, solids: [`shank-${s}`], region: "jeans", tone: 0.6, z: [leg + 0.02, 1.02, 1.3], plan: "round" },
      { id: `sil-shoe-${s}`, solids: [`shoe-${s}`], region: "sneakers", tone: 0.25, z: [leg + 0.1, 1.1, 1.1] },
      { id: `sil-toe-${s}`, solids: [`shoe-toe-${s}`], region: "sneakers", tone: 0.25, z: [leg + 0.1, 1.1, 1.1] },
    ]),
    { id: "sil-seat", solids: ["jeans-seat"], region: "jeans", tone: 0.6, z: [2, 0.9, 0], views: "ef" },
    { id: "sil-neck", solids: ["neck"], region: "face-skin", tone: 0.4, z: [3.8, 3.8, 4.8] },
    { id: "sil-body", solids: ["hoodie"], region: "hoodie", tone: 0.4, z: [4, 4, 4] },
    { id: "sil-head", solids: ["head", "hair"], region: "face-skin+hair", tone: 0.4, z: [5, 5, 6] },
  ];
}

const ARM = /^(arm|forearm|hand)-/;

function shapesOf(spec: Spec, scale: number, body: ReadonlyMap<string, Json>): Record<string, Json[]> {
  const tol = TOL[scale]!;
  const own = spec.solids.map((id) => {
    const s = body.get(id);
    if (s === undefined) throw new Error(`1:${scale} has no body solid ${id}`);
    return s;
  });
  const bone = own[0]!.bone as string;
  if (own.some((s) => s.bone !== bone)) throw new Error(`group ${spec.id} spans bones`);
  const f = REST[bone]!;
  const first = own.find((s) => s.id === spec.id) ?? own[0]!;
  const clouds = own.flatMap(pieces);
  const all = clouds.flat();
  const world = (p: V): V => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V;
  const views = spec.views ?? "efp";
  const index = { elevation: 0, front: 1, plan: 2 } as const;
  const base = (view: keyof typeof index): Json => ({ id: spec.id, bone, role: spec.role ?? first.role, z: spec.z[index[view]], filled: true, tone: spec.tone ?? first.tone, ...(spec.region.includes("+") ? { regions: spec.region.split("+") } : { region: spec.region }), ...(spec.hatch !== undefined && hatched(spec, scale, view) ? { hatch: spec.hatch } : {}) });
  const poly = (view: keyof typeof index, ring: readonly P[], extra: Json = {}): Json => ({ ...base(view), kind: "poly", points: pts(ring), closed: true, ...extra });
  const range = (k: number, cloud: readonly V[] = all): [number, number] => [Math.min(...cloud.map((p) => p[k]!)), Math.max(...cloud.map((p) => p[k]!))];
  const out: Record<string, Json[]> = { elevation: [], front: [], plan: [] };
  const side = outline(clouds, (p) => ({ x: p[0], y: p[1] }), tol);
  const lateral = outline(clouds, (p) => ({ x: p[0], y: p[2] }), tol);
  const [u0, u1] = range(0);
  const ws = range(2);
  if (views.includes("e")) out.elevation!.push(poly("elevation", side, ARM.test(bone) ? { solid: { kind: "extruded", sweep: "w", z0: r4(f.o[2]! + ws[0]), z1: r4(f.o[2]! + ws[1]) } } : {}));
  const third = (u1 - u0) / 3;
  const band = (lo: number, hi: number): [number, number] => range(1, all.filter((p) => p[0] >= lo && p[0] <= hi));
  const [a, b] = [band(u0, u0 + third), band(u1 - third, u1)];
  const [ua, ub] = [u0 + third / 2, u1 - third / 2];
  const L = LENGTH[bone]!;
  const at = (u: number, k: 0 | 1): number => r4(a[k] + ((b[k] - a[k]) * (Math.min(Math.max(u, u0), u1) - ua)) / (ub - ua));
  const prism = { kind: "extruded", sweep: "v", z0: at(0, 0), z1: at(0, 1), zTip: [at(L, 0), at(L, 1)] };
  const grounded = /^(foot|toe)-/.test(bone);
  if (views.includes("f")) {
    out.front!.push(grounded ? poly("front", outline(clouds.map((c) => c.map(world)), (p) => ({ x: p[2], y: p[1] }), tol)) : poly("front", lateral, { solid: prism }));
    if (spec.fringe === true) {
      const local = ARCH.map(([z, y]) => ({ x: y - f.o[1]!, y: z }));
      const strips = local.slice(1).map((q, i) => clipConvex(lateral, [{ x: local[i]!.x, y: local[i]!.y }, { x: 9, y: local[i]!.y }, { x: 9, y: q.y }, { x: q.x, y: q.y }])).filter((r) => r.length >= 3 && Math.abs(areaOf(r)) > 1e-7);
      const fringe = simplify(largest(unionPolygons(strips.map((r) => (areaOf(r) < 0 ? [...r].reverse() : r)))), tol).map((p) => ({ x: r4(p.x), y: r4(p.y) }));
      out.front!.push({ ...poly("front", fringe, { solid: prism }), id: `${spec.id}-fringe`, z: 5.5 });
    }
  }
  if (views.includes("p")) {
    const extra = (spec.extraPlan ?? []).map((id) => body.get(id)!).flatMap(pieces);
    const cloudsP = [...clouds, ...extra];
    const allP = cloudsP.flat();
    if (spec.plan === "prism") {
      const vs = range(1, allP);
      out.plan!.push(poly("plan", extra.length === 0 ? lateral : outline(cloudsP, (p) => ({ x: p[0], y: p[2] }), tol), { solid: { kind: "extruded", sweep: "v", z0: r4(vs[0]), z1: r4(vs[1]) } }));
    } else if (spec.plan === "round") {
      const [p0, p1] = range(0, allP);
      const vs = range(1, allP);
      const wsP = range(2, allP);
      const [cv, cw] = [(vs[0] + vs[1]) / 2, (wsP[0] + wsP[1]) / 2];
      const n = Math.max(4, Math.min(12, Math.round((p1 - p0) / (tol * 40))));
      const half: P[] = [{ x: r4(p0), y: r4(cv) }];
      for (let i = 0; i <= n; i++) {
        const u = p0 + ((p1 - p0) * i) / n;
        const w = across(lateral, Math.min(Math.max(u, p0 + 1e-4), p1 - 1e-4));
        half.push({ x: r4(u), y: r4(cv + (w[1] - w[0]) / 2) });
      }
      half.push({ x: r4(p1), y: r4(cv) });
      out.plan!.push(poly("plan", simplify(half, tol / 2), { solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: r4(axisAhead(f, cv)), axisZ: r4(cw), mirror: true } }));
    } else out.plan!.push(poly("plan", outline(clouds.map((c) => c.map(world)), (p) => ({ x: p[0], y: p[2] }), tol)));
  }
  return out;
}
/** ↗️ The world x of a point at local v on the bone normal through its rest origin (the bone's v is not world x when the bone leans). */
function axisAhead(f: Frame, v: number): number {
  return v * f.v[0]!;
}
function hatched(spec: Spec, scale: number, view: string): boolean {
  if (spec.hatch === "denim") return scale <= 100;
  return scale === 50 && (view === "elevation" || /^sleeve-/.test(spec.id));
}

type Stroke = [id: string, bone: string, role: string, z: number, tone: number, region: string, points: V[]];

function strokeShape(view: string, [id, bone, role, z, tone, region, points]: Stroke): Json {
  const f = REST[bone]!;
  const base = { id, bone, role, z, filled: false, tone, region, kind: "poly", closed: false };
  if (view === "front") return { ...base, points: points.map((p) => [r4(p[2]), r4(p[1])]) };
  if (view === "plan") return { ...base, points: points.map((p) => [r4(p[0]), r4(p[2])]) };
  const local = points.map((p) => {
    const d = [p[0] - f.o[0]!, p[1] - f.o[1]!, p[2] - f.o[2]!];
    return [r4(d[0]! * f.u[0]! + d[1]! * f.u[1]! + d[2]! * f.u[2]!), r4(d[0]! * f.v[0]! + d[1]! * f.v[1]! + d[2]! * f.v[2]!)];
  });
  const zs = points.map((p) => p[2]);
  return { ...base, points: local, ...(ARM.test(bone) ? { solid: { kind: "extruded", sweep: "w", z0: r4(Math.min(...zs) - 0.03), z1: r4(Math.max(...zs) + 0.03) } } : {}) };
}

const strokes = JSON.parse(readFileSync(join(OUT, "strokes.json"), "utf8")) as Record<string, Record<string, Stroke[]>>;
const result: Record<string, { geometry: Record<string, Json[]> }> = {};
for (const scale of [50, 100, 200, 500]) {
  const body = new Map((asset.volumes[String(scale)].body as Json[]).map((s) => [s.id as string, s]));
  const views: Record<string, Json[]> = { elevation: [], front: [], plan: [] };
  const used = new Set<string>();
  for (const spec of specs(scale)) {
    spec.solids.forEach((id) => used.add(id));
    for (const [view, shapes] of Object.entries(shapesOf(spec, scale, body))) views[view]!.push(...shapes);
  }
  const unused = [...body.values()].filter((s) => !used.has(s.id) && s.texture !== true && s.slender !== true).map((s) => s.id);
  if (unused.length > 0) throw new Error(`1:${scale}: body solids without a drawn shape: ${unused.join(", ")}`);
  for (const [view, list] of Object.entries(strokes[String(scale)]!)) views[view]!.push(...list.map((s) => strokeShape(view, s)));
  for (const list of Object.values(views)) list.sort((p, q) => p.z - q.z);
  result[String(scale)] = { geometry: views };
  console.log(scale, Object.entries(views).map(([v, l]) => `${v} ${l.length} shapes ${l.reduce((n, s) => n + s.points.length, 0)} points`).join(", "));
}
writeFileSync(join(OUT, "shapes.json"), JSON.stringify(result));
