/** 🪵️ Authoritative author script of `used-bentwood-dining-chairs`: four used bentwood café chairs of the Viennese No. 14 / 214 type, stored as two seat-on-seat pairs on a folded moving blanket, carried two at a time and set out again at a dining table.
 *
 * Run: `$BUN author-bentwood.ts` in this folder; writes `🔣️.json` next to it. Every number is a hand-chosen value of `author-geometry.ts` or of the tables below; nothing is fitted.
 *
 * @see ./author-geometry.ts — the chair geometry
 * @see ../../brief.md — the brief of the wave
 * @see ../../reports/used-bentwood-dining-chairs.md — reference table, measurements and decisions */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { BOW, CANE, FRONT, INNER, LEG_RING, REAR, SEAT, TEAR, Y0, bow, frontLeg, innerLoop, legRing, rearLeg, wholeBow, type Primitive, type Tube, type V2, type V3 } from "./author-geometry.ts";

const HERE = import.meta.dir;
const OUT = join(HERE, "🔣️.json");
const SCALES = [50, 100, 200, 500] as const;
type Scale = (typeof SCALES)[number];
type View = "elevation" | "front" | "plan";
type Draw = "fill" | "stroke" | "none";
type Json = any;
const T = (en: string, de: string) => ({ en, de });
const r5 = (x: number): number => (x !== 0 && Math.abs(x) < 0.01 ? Number(x.toPrecision(6)) : Math.round(x * 1e5) / 1e5) + 0;
const rnd = (v: any): any => (typeof v === "number" ? r5(v) : Array.isArray(v) ? v.map(rnd) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, rnd(x)])) : v);
const feature = (s: Scale): number => (0.15 * s) / 1000;

/** 🧱️ Materials with their physical data (aged surfaces where the use changed them). */
const DENSITY = { beech: 730, cane: 380, blanket: 100 };
const MATERIALS = [
  { id: "bentwood-beech", title: T("Steam-bent beech, aged lacquer", "Dampfgebogene Buche, gealterter Lack"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: DENSITY.beech, conductivityWmK: 0.16, specificHeatJKgK: 1600, elasticModulusPa: 14000000000, strengthPa: 100000000, source: "literature", reference: "wood-handbook-2021" },
  { id: "rattan-cane", title: T("Woven rattan cane (Viennese weave)", "Rattan-Rohrgeflecht (Wiener Geflecht)"), emissivity: 0.9, solarAbsorptivity: 0.45, densityKgM3: DENSITY.cane, conductivityWmK: 0.1, specificHeatJKgK: 1500, source: "authored-estimate", reference: "prosea-rattans-1993" },
  { id: "moving-blanket", title: T("Moving blanket (recycled cotton and polyester felt)", "Umzugsdecke (Recycling-Baumwoll-Polyester-Filz)"), emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: DENSITY.blanket, conductivityWmK: 0.045, specificHeatJKgK: 1300, source: "authored-estimate", reference: "iso-10456-2007" },
];

/** 🪑️ The four chairs: id, lateral rest position, upright (0) or seat-on-seat upside down (180), rest height of the seat-ring middle. */
const BLANKET = { x: 0.9, z: 1.0, t: 0.016 };
const CHAIRS = [
  { id: "a", z: 0.27, angle: 0, oy: BLANKET.t + Y0 },
  { id: "b", z: 0.27, angle: 180, oy: BLANKET.t + SEAT.y1 + (SEAT.y1 - Y0) },
  { id: "c", z: -0.27, angle: 0, oy: BLANKET.t + Y0 },
  { id: "d", z: -0.27, angle: 180, oy: BLANKET.t + SEAT.y1 + (SEAT.y1 - Y0) },
] as const;
type Chair = (typeof CHAIRS)[number];

/** 🦴️ The part bones of one chair in the chair frame (x, y above the floor): origin, tip, title. */
const RING = legRing();
const PART_BONES: Record<string, { o: V2; tip: V2; title: { en: string; de: string } }> = {
  seat: { o: [-SEAT.ro, Y0], tip: [SEAT.ro, Y0], title: T("Seat ring with cane", "Sitzring mit Geflecht") },
  "rear-legs": { o: [REAR.foot[0], 0], tip: [REAR.ring[0], REAR.ring[1]], title: T("Rear legs (pair, lower ends of the bow)", "Hinterbeine (Paar, untere Enden des Bogens)") },
  back: { o: [REAR.ring[0], REAR.ring[1]], tip: [REAR.ring[0] + BOW.slope * (BOW.ySpring + BOW.archH - REAR.ring[1]), BOW.ySpring + BOW.archH], title: T("Back bow with inner loop", "Lehnenbogen mit Innenbogen") },
  "front-legs": { o: [FRONT.foot[0], 0], tip: [FRONT.top[0], FRONT.top[1]], title: T("Front legs (pair)", "Vorderbeine (Paar)") },
  "leg-ring": { o: [RING.cx - RING.R, LEG_RING.y], tip: [RING.cx + RING.R, LEG_RING.y], title: T("Leg ring", "Beinring") },
};
const boneAngle = (b: string): number => Math.atan2(PART_BONES[b]!.tip[1] - PART_BONES[b]!.o[1], PART_BONES[b]!.tip[0] - PART_BONES[b]!.o[0]);
const boneLength = (b: string): number => Math.hypot(PART_BONES[b]!.tip[1] - PART_BONES[b]!.o[1], PART_BONES[b]!.tip[0] - PART_BONES[b]!.o[0]);

/** 🔁️ Chair frame (floor-based) to bone-local of a part bone. */
function toBone(b: string, p: V3): V3 {
  const o = PART_BONES[b]!.o;
  const a = boneAngle(b);
  const dx = p[0] - o[0];
  const dy = p[1] - o[1];
  return [Math.cos(a) * dx + Math.sin(a) * dy, -Math.sin(a) * dx + Math.cos(a) * dy, p[2]];
}
const dirToBone = (b: string, d: V3): V3 => {
  const a = boneAngle(b);
  return [Math.cos(a) * d[0] + Math.sin(a) * d[1], -Math.sin(a) * d[0] + Math.cos(a) * d[1], d[2]];
};

/** 🌍️ Chair frame (floor-based) to world rest coordinates of chair `c`. */
function toWorld(c: Chair, p: V3): V3 {
  const s = c.angle === 180 ? -1 : 1;
  return [s * p[0], c.oy + s * (p[1] - Y0), c.z + p[2]];
}

/** 🧩️ One part of one chair at one scale. */
type Part = { key: string; bone: string; prim: Primitive; material: string; region: string; tone: number; role: "body" | "detail" | "texture"; slender?: boolean; sheet?: boolean; texture?: boolean; draw: Record<View, Draw>; merge?: string; penMm?: number };

const FILL: Record<View, Draw> = { elevation: "fill", front: "fill", plan: "fill" };
const STROKE: Record<View, Draw> = { elevation: "stroke", front: "stroke", plan: "stroke" };

/** 🔪️ The tear of chair d as a hole in the cane, in the chair frame (x, z). */
const tearOf = (c: Chair): V2[][] => (c.id === "d" ? [TEAR] : []);

/** 🪑️ The parts of chair `c` as the scale `s` models and draws them. */
function partsOf(c: Chair, s: Scale): Part[] {
  const wood = { material: "bentwood-beech", region: "bentwood-frame", tone: 0.42, role: "body" as const };
  const cane = { material: "rattan-cane", region: "cane-seats", tone: 0.72, role: "body" as const };
  const ringC: V3 = [0, Y0, 0];
  const legRingC: V3 = [RING.cx, LEG_RING.y, 0];
  if (s === 50 || s === 100) {
    const fine = s === 50;
    const parts: Part[] = [
      { key: "cane", bone: "seat", prim: { kind: "disc", c: [0, 0, 0], r: CANE.r, y0: CANE.y0, y1: CANE.y1, holes: s === 50 || s === 100 ? tearOf(c) : [] }, ...cane, sheet: true, draw: FILL },
      { key: "seat-ring", bone: "seat", prim: { kind: "hoop", c: ringC, ri: SEAT.ri, ro: SEAT.ro, h: SEAT.y1 - SEAT.y0, chamfer: fine ? 0.005 : 0 }, ...wood, tone: 0.48, draw: FILL },
      { key: "rear-leg-l", bone: "rear-legs", prim: rearLeg(1, fine ? 8 : 3), ...wood, draw: FILL, merge: "rear-legs" },
      { key: "rear-leg-r", bone: "rear-legs", prim: rearLeg(-1, fine ? 8 : 3), ...wood, draw: FILL, merge: "rear-legs" },
      { key: "bow", bone: "back", prim: bow(fine ? 10 : 4, fine ? 16 : 8), ...wood, draw: FILL },
      { key: "inner-loop", bone: "back", prim: innerLoop(fine ? 5 : 2, fine ? 12 : 6), ...wood, tone: 0.46, draw: FILL },
      { key: "front-leg-l", bone: "front-legs", prim: frontLeg(1), ...wood, draw: FILL, merge: "front-legs" },
      { key: "front-leg-r", bone: "front-legs", prim: frontLeg(-1), ...wood, draw: FILL, merge: "front-legs" },
      { key: "leg-ring", bone: "leg-ring", prim: { kind: "hoop", c: legRingC, ri: RING.R - LEG_RING.w / 2, ro: RING.R + LEG_RING.w / 2, h: LEG_RING.h, chamfer: fine ? 0.004 : 0 }, ...wood, tone: 0.46, draw: FILL },
    ];
    if (fine && c.id === "b") parts.push({ key: "glue-collar", bone: "front-legs", prim: { kind: "tube", pts: [[FRONT.top[0] + 0.0012, 0.414, FRONT.top[2]], [FRONT.top[0], 0.422, FRONT.top[2]]], r: [0.0172, 0.0172] }, material: "bentwood-beech", region: "bentwood-frame", tone: 0.25, role: "detail", texture: true, draw: { elevation: "none", front: "none", plan: "none" } });
    return parts;
  }
  if (s === 200)
    return [
      { key: "seat", bone: "seat", prim: { kind: "slab", c: ringC, r: SEAT.ro, y0: SEAT.y0, y1: SEAT.y1, dish: { r: SEAT.ri, depth: SEAT.y1 - CANE.y1 } }, ...wood, tone: 0.5, draw: FILL },
      { key: "frame-bow", bone: "back", prim: wholeBow(2, 2, 6), ...wood, slender: true, draw: STROKE },
      { key: "inner-loop", bone: "back", prim: innerLoop(1, 4), ...wood, tone: 0.46, slender: true, draw: STROKE, penMm: 0.13 },
      { key: "front-leg-l", bone: "front-legs", prim: frontLeg(1, [FRONT.rTop, FRONT.rTop]), ...wood, draw: { elevation: "fill", front: "fill", plan: "fill" }, merge: "front-legs" },
      { key: "front-leg-r", bone: "front-legs", prim: frontLeg(-1, [FRONT.rTop, FRONT.rTop]), ...wood, draw: { elevation: "fill", front: "fill", plan: "fill" }, merge: "front-legs" },
      { key: "leg-ring", bone: "leg-ring", prim: { kind: "hoop", c: legRingC, ri: RING.R - LEG_RING.w / 2, ro: RING.R + LEG_RING.w / 2, h: LEG_RING.h, chamfer: 0 }, ...wood, tone: 0.46, slender: true, draw: STROKE, penMm: 0.13 },
    ];
  return [
    { key: "seat", bone: "seat", prim: { kind: "slab", c: ringC, r: SEAT.ro, y0: SEAT.y1 - 0.075, y1: SEAT.y1 }, ...wood, tone: 0.5, draw: FILL },
    { key: "frame-bow", bone: "back", prim: wholeBow(1, 1, 6), ...wood, slender: true, draw: STROKE },
    { key: "front-leg-l", bone: "front-legs", prim: frontLeg(1, [0.0105, 0.0105]), ...wood, slender: true, draw: STROKE, merge: "front-legs" },
    { key: "front-leg-r", bone: "front-legs", prim: frontLeg(-1, [0.0105, 0.0105]), ...wood, slender: true, draw: STROKE, merge: "front-legs" },
    { key: "leg-ring", bone: "leg-ring", prim: { kind: "hoop", c: legRingC, ri: RING.R - LEG_RING.w / 2, ro: RING.R + LEG_RING.w / 2, h: LEG_RING.h, chamfer: 0 }, ...wood, tone: 0.46, slender: true, draw: STROKE },
  ];
}

//#region 🧊️ Solids
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const unit = (a: V3): V3 => {
  const n = Math.hypot(...a) || 1;
  return [a[0] / n, a[1] / n, a[2] / n];
};

/** 🧭️ The unit quaternion of a section frame whose normal (+z) is the direction `t` and whose x axis lies as close as possible to `ref`, so neighbouring sections never twist against each other. */
function frameTo(t: V3, ref: V3): [number, number, number, number] {
  const z = unit(t);
  const d = ref[0] * z[0] + ref[1] * z[1] + ref[2] * z[2];
  const x = unit([ref[0] - d * z[0], ref[1] - d * z[1], ref[2] - d * z[2]]);
  const y: V3 = [z[1] * x[2] - z[2] * x[1], z[2] * x[0] - z[0] * x[2], z[0] * x[1] - z[1] * x[0]];
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]];
  const tr = m00 + m11 + m22;
  let q: [number, number, number, number];
  if (tr > 0) { const k = Math.sqrt(tr + 1) * 2; q = [(m21 - m12) / k, (m02 - m20) / k, (m10 - m01) / k, k / 4]; }
  else if (m00 > m11 && m00 > m22) { const k = Math.sqrt(1 + m00 - m11 - m22) * 2; q = [k / 4, (m01 + m10) / k, (m02 + m20) / k, (m21 - m12) / k]; }
  else if (m11 > m22) { const k = Math.sqrt(1 + m11 - m00 - m22) * 2; q = [(m01 + m10) / k, k / 4, (m12 + m21) / k, (m02 - m20) / k]; }
  else { const k = Math.sqrt(1 + m22 - m00 - m11) * 2; q = [(m02 + m20) / k, (m12 + m21) / k, k / 4, (m10 - m01) / k]; }
  const n = Math.hypot(...q);
  return q.map((c) => c / n) as [number, number, number, number];
}

/** 🧊️ The solid of a part in its bone frame. */
function solidOf(chair: Chair, part: Part, id: string): Json {
  const base: Json = { id, kind: "", bone: `${part.bone}-${chair.id}`, material: part.material, role: part.role, tone: part.tone, ...(part.slender ? { slender: true } : {}), ...(part.sheet ? { sheet: true } : {}), ...(part.texture ? { texture: true } : {}) };
  const p = part.prim;
  const b = part.bone;
  if (p.kind === "tube") {
    const pts = p.pts.map((q) => toBone(b, q));
    const sections = pts.map((at, i) => {
      const t = sub(pts[Math.min(i + 1, pts.length - 1)]!, pts[Math.max(i - 1, 0)]!);
      return { at, rx: p.r[i]!, ry: p.r[i]!, rotation: frameTo(t, dirToBone(b, [1, 0, 0])) };
    });
    return { ...base, kind: "loft", sections, capStart: true, capEnd: true };
  }
  if (p.kind === "hoop") {
    const h = p.h / 2;
    const c = p.chamfer;
    const loop: V2[] = c > 0 ? [[-h, p.ri + c], [-h, p.ro - c], [-h + c, p.ro], [h - c, p.ro], [h, p.ro - c], [h, p.ri + c], [h - c, p.ri], [-h + c, p.ri]] : [[-h, p.ri], [-h, p.ro], [h, p.ro], [h, p.ri]];
    return { ...base, kind: "revolve", profile: [...loop, loop[0]], axis: dirToBone(b, [0, 1, 0]), origin: toBone(b, p.c) };
  }
  if (p.kind === "slab") {
    const top = p.y1 - p.c[1];
    const bottom = p.y0 - p.c[1];
    const profile: V2[] = p.dish ? [[bottom, 0], [bottom, p.r], [top, p.r], [top, p.dish.r], [top - p.dish.depth, p.dish.r], [top - p.dish.depth, 0]] : [[bottom, 0], [bottom, p.r], [top, p.r], [top, 0]];
    return { ...base, kind: "revolve", profile, axis: dirToBone(b, [0, 1, 0]), origin: toBone(b, p.c) };
  }
  if (p.kind === "disc") {
    const o = PART_BONES[b]!.o;
    const ring = circle(p.r, 48).map(([x, z]): V2 => [z, x - o[0]]);
    const holes = p.holes.map((h) => h.map(([x, z]): V2 => [z, x - o[0]]));
    return { ...base, kind: "extrude", profile: ring, ...(holes.length ? { holes } : {}), plane: "zx", from: p.y0 - o[1], to: p.y1 - o[1] };
  }
  return { ...base, kind: "box", size: p.size, cornerRadius: p.corner, transform: { translation: p.c } };
}

const circle = (r: number, n: number, cx = 0, cy = 0): V2[] => Array.from({ length: n }, (_, i) => [cx + r * Math.cos((2 * Math.PI * i) / n), cy + r * Math.sin((2 * Math.PI * i) / n)] as V2);
//#endregion

//#region ✏️ Drawings
type Ring2 = V2[];
const pt = (p: V2) => ({ x: p[0], y: p[1] });

/** 🧮️ Douglas-Peucker on a closed ring. */
function simplify(ring: Ring2, tol: number): Ring2 {
  const dp = (pts: Ring2): Ring2 => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    let best = -1;
    let idx = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const p = pts[i]!;
      const L = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
      const d = Math.abs((b[0] - a[0]) * (a[1] - p[1]) - (a[0] - p[0]) * (b[1] - a[1])) / L;
      if (d > best) (best = d), (idx = i);
    }
    return best > tol ? [...dp(pts.slice(0, idx + 1)).slice(0, -1), ...dp(pts.slice(idx))] : [a, b];
  };
  let far = 0;
  for (let i = 1; i < ring.length; i++) if (Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]) > Math.hypot(ring[far]![0] - ring[0]![0], ring[far]![1] - ring[0]![1])) far = i;
  const out = [...dp(ring.slice(0, far + 1)).slice(0, -1), ...dp([...ring.slice(far), ring[0]!]).slice(0, -1)];
  return out.length >= 3 ? out : ring;
}

/** 🎯️ The 2D projection of a chair-frame point into a view: elevation in the chair frame (x, y), front and plan in world rest coordinates. */
function project(c: Chair, v: View, p: V3): V2 {
  if (v === "elevation") return [p[0], p[1]];
  const w = toWorld(c, p);
  return v === "front" ? [w[2], w[1]] : [w[0], w[2]];
}

/** 🌀️ The swept outline of a round rod in a view: discs at the points and tangent quads between them. */
function tubePolys(c: Chair, v: View, tube: Tube): Ring2[] {
  const q = tube.pts.map((p) => project(c, v, p));
  const polys: Ring2[] = q.map((p, i) => circle(tube.r[i]!, 20, p[0], p[1]));
  for (let i = 0; i + 1 < q.length; i++) {
    const [a, b] = [q[i]!, q[i + 1]!];
    const d = Math.hypot(b[0] - a[0], b[1] - a[1]);
    if (d < 1e-6) continue;
    const n: V2 = [-(b[1] - a[1]) / d, (b[0] - a[0]) / d];
    const ra = tube.r[i]!;
    const rb = tube.r[i + 1]!;
    polys.push([[a[0] + n[0] * ra, a[1] + n[1] * ra], [b[0] + n[0] * rb, b[1] + n[1] * rb], [b[0] - n[0] * rb, b[1] - n[1] * rb], [a[0] - n[0] * ra, a[1] - n[1] * ra]]);
  }
  return polys;
}

/** 🧾️ The filled polygons a part shows in a view (chair frame for elevation, world rest otherwise). */
function partPolys(c: Chair, v: View, part: Part): { outer: Ring2[]; holes: Ring2[] } {
  const p = part.prim;
  if (p.kind === "tube") return { outer: tubePolys(c, v, p), holes: [] };
  if (p.kind === "box") return { outer: [], holes: [] };
  const r = p.kind === "hoop" ? p.ro : p.r;
  const [y0, y1] = p.kind === "hoop" ? [p.c[1] - p.h / 2, p.c[1] + p.h / 2] : [p.y0, p.y1];
  if (v === "plan") {
    const n = 64;
    const outer = circle(r, n, p.c[0], p.c[2]).map(([x, z]) => project(c, v, [x, (y0 + y1) / 2, z]));
    const holes = p.kind === "hoop" ? [circle(p.ri, n, p.c[0], p.c[2]).map(([x, z]) => project(c, v, [x, (y0 + y1) / 2, z]))] : p.kind === "disc" ? p.holes.map((h) => h.map(([x, z]) => project(c, v, [x, (y0 + y1) / 2, z]))) : [];
    return { outer: [outer], holes };
  }
  const ch = p.kind === "hoop" ? p.chamfer : 0;
  const lat = v === "elevation" ? 0 : 2;
  const lo = p.c[lat] - r;
  const hi = p.c[lat] + r;
  const corner = (a: number, y: number): V3 => (v === "elevation" ? [a, y, 0] : [0, y, a]);
  const prof: V3[] = ch > 0 ? [corner(lo, y0 + ch), corner(lo + ch, y0), corner(hi - ch, y0), corner(hi, y0 + ch), corner(hi, y1 - ch), corner(hi - ch, y1), corner(lo + ch, y1), corner(lo, y1 - ch)] : [corner(lo, y0), corner(hi, y0), corner(hi, y1), corner(lo, y1)];
  return { outer: [prof.map((q) => project(c, v, q))], holes: [] };
}

/** 〰️ The centre line a slender part shows as a stroke in a view (open), or the centre circle of a hoop in plan (closed). */
function partStroke(c: Chair, v: View, part: Part): { pts: Ring2; closed: boolean } {
  const p = part.prim;
  if (p.kind === "tube") return { pts: p.pts.map((q) => project(c, v, q)), closed: false };
  if (p.kind === "hoop") {
    const R = (p.ri + p.ro) / 2;
    if (v === "plan") return { pts: circle(R, 48, p.c[0], p.c[2]).map(([x, z]) => project(c, v, [x, p.c[1], z])), closed: true };
    return { pts: v === "elevation" ? [project(c, v, [p.c[0] - R, p.c[1], 0]), project(c, v, [p.c[0] + R, p.c[1], 0])] : [project(c, v, [0, p.c[1], -R]), project(c, v, [0, p.c[1], R])], closed: false };
  }
  throw new Error(`no stroke for ${part.key}`);
}

const toLocal2 = (bone: string, p: V2): V2 => {
  const q = toBone(bone, [p[0], p[1], 0]);
  return [q[0], q[1]];
};

const pathOf = (rings: Ring2[]): string => rings.map((r) => `M ${r.map((p) => `${r5(p[0])} ${r5(p[1])}`).join(" L ")} Z`).join(" ");

/** 🖌️ Union of polygons as outer rings and holes. */
function union(polys: Ring2[], tol: number): Ring2[] {
  const rings = unionPolygons(polys.map((r) => r.map(pt))).map((r) => r.map((q): V2 => [q.x, q.y]));
  return rings.map((r) => simplify(r, tol));
}
//#endregion

//#region 🖼️ Representations
const PENS: Record<Scale, Json> = {
  50: { body: 0.5, detail: 0.25, texture: 0.13, glazing: 0.18, ground: 0.7 },
  100: { body: 0.35, detail: 0.18, texture: 0.13, glazing: 0.13, ground: 0.5 },
  200: { body: 0.25, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 },
  500: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
};
const RANGES: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };

/** 🗺️ The three drawings of one scale. */
function geometryOf(s: Scale): Record<View, Json[]> {
  const out: Record<View, Json[]> = { elevation: [], front: [], plan: [] };
  const tol = 0.00025 * (s / 50);
  const pen = s === 50 ? 0.25 : s === 100 ? 0.18 : undefined;
  let z = 1;
  const blanketShape = (v: View): Json => {
    const [w, h] = v === "elevation" ? [BLANKET.x, BLANKET.t] : v === "front" ? [BLANKET.z, BLANKET.t] : [BLANKET.x, BLANKET.z];
    const x = v === "elevation" ? 0 : -w / 2;
    const y = v === "plan" ? -h / 2 : 0;
    return { id: "blanket", bone: "blanket", role: "body", z: 0.5, filled: true, tone: 0.55, kind: "rect", x, y, w, h, r: v === "plan" ? 0.02 : Math.min(0.006, BLANKET.t / 2 - 0.0005), region: "moving-blanket", material: "moving-blanket", ...(v === "plan" && s <= 100 ? { hatch: "quilting" } : {}), ...(pen ? { penMm: pen } : s === 500 ? { penMm: 0.2 } : {}) };
  };
  for (const v of ["elevation", "front", "plan"] as View[]) out[v].push(blanketShape(v));
  for (const c of CHAIRS) {
    const parts = partsOf(c, s).filter((p) => p.role !== "detail");
    const groups = new Map<string, Part[]>();
    for (const p of parts) groups.set(p.merge ?? p.key, [...(groups.get(p.merge ?? p.key) ?? []), p]);
    for (const v of ["elevation", "front", "plan"] as View[]) {
      for (const [key, group] of groups) {
        const draw = group[0]!.draw[v];
        if (draw === "none") continue;
        const head = group[0]!;
        const bone = `${head.bone}-${c.id}`;
        const meta = { bone, role: "body", tone: head.tone, region: head.region, material: head.material };
        const local = (r: Ring2): Ring2 => (v === "elevation" ? r.map((p) => toLocal2(head.bone, p)) : r);
        const members = v === "elevation" ? [group] : group.map((p) => [p]);
        members.forEach((set) => {
          const id = `${c.id}-${v === "elevation" || set.length === group.length ? key : set[0]!.key}`;
          if (draw === "stroke") {
            for (const [j, p] of set.entries()) {
              const st = partStroke(c, v, p);
              out[v].push({ id: set.length > 1 ? `${id}-${j}` : id, ...meta, z: z++, filled: false, kind: "poly", points: rnd(local(st.pts)), closed: st.closed, ...(p.penMm ? { penMm: p.penMm } : {}) });
            }
            return;
          }
          const polys = set.map((p) => partPolys(c, v, p));
          const outer = union(polys.flatMap((q) => q.outer), tol);
          const holes = polys.flatMap((q) => q.holes).map((h) => simplify(h, tol));
          const rings = [...outer, ...holes].map(local);
          const shape = rings.length === 1 ? { kind: "poly", points: rnd(rings[0]), closed: true } : { kind: "path", d: pathOf(rings) };
          out[v].push({ id, ...meta, z: z++, filled: true, ...shape, ...(s <= 100 ? { penMm: head.key === "seat-ring" || head.key === "seat" || head.key === "cane" ? (s === 50 ? 0.35 : 0.25) : pen } : {}) });
        });
      }
    }
    for (const t of texturesOf(c, s)) out[t.view as View].push({ ...t.shape, z: z++ });
  }
  if (s === 500) for (const c of CHAIRS) for (const shape of out.plan.filter((q) => q.bone === `seat-${c.id}`)) (shape.regions = ["bentwood-frame", "cane-seats"]), delete shape.region;
  if (s >= 200) for (const shape of [...out.elevation, ...out.front].filter((q) => q.bone.startsWith("seat-"))) (shape.regions = ["bentwood-frame", "cane-seats"]), delete shape.region;
  if (s === 200)
    for (const c of CHAIRS) {
      const cane = circle(SEAT.ri, 48).map(([x, zz]) => project(c, "plan", [x, CANE.y1, zz]));
      out.plan.push({ id: `${c.id}-cane`, bone: `seat-${c.id}`, role: "body", z: z++, filled: true, tone: 0.72, kind: "poly", points: rnd(cane), closed: true, region: "cane-seats", material: "rattan-cane" });
    }
  return out;
}

/** 🧶️ Texture and detail strokes of the traces of use at the fine scales: cane weave and tear (plan), glue bead, scuffs and worn lacquer (elevation). */
function texturesOf(c: Chair, s: Scale): { view: View; shape: Json }[] {
  if (s > 100) return [];
  const out: { view: View; shape: Json }[] = [];
  const step = s === 50 ? 0.03 : 0.06;
  const R = SEAT.ri - 0.012;
  const lines: string[] = [];
  const inTear = (x: number, zz: number): boolean => c.id === "d" && x > 0.07 && x < 0.15 && zz > 0.02 && zz < 0.105;
  for (const dir of [1, -1]) {
    for (let k = -R; k <= R + 1e-9; k += step) {
      const seg: V2[] = [];
      for (let t = -R; t <= R + 1e-9; t += 0.01) {
        const x = (t + dir * k) / Math.SQRT2;
        const zz = (dir * t - k) / Math.SQRT2;
        if (Math.hypot(x, zz) <= R && !inTear(x, zz)) seg.push(project(c, "plan", [x, CANE.y1, zz]));
        else if (seg.length) break;
      }
      if (seg.length >= 2) lines.push(`M ${seg.map((p) => `${r5(p[0])} ${r5(p[1])}`).join(" L ")}`);
    }
  }
  out.push({ view: "plan", shape: { id: `${c.id}-cane-weave`, bone: `seat-${c.id}`, role: "texture", filled: false, tone: 0.5, kind: "path", d: lines.join(" "), region: "cane-seats" } });
  if (c.id === "d") {
    const frayed = TEAR.map(([x, zz]) => project(c, "plan", [x, CANE.y1, zz]));
    out.push({ view: "plan", shape: { id: "d-cane-tear", bone: "seat-d", role: "detail", filled: false, tone: 0.2, kind: "poly", points: rnd(frayed), closed: true, region: "cane-seats" } });
  }
  if (s === 50) {
    if (c.id === "b") {
      const a = toBone("front-legs", [FRONT.top[0] - 0.016, 0.416, 0]);
      const b = toBone("front-legs", [FRONT.top[0] + 0.017, 0.418, 0]);
      out.push({ view: "elevation", shape: { id: "b-glue-bead", bone: "front-legs-b", role: "detail", filled: false, tone: 0.2, kind: "poly", points: rnd([[a[0], a[1]], [b[0], b[1]]]), closed: false, region: "bentwood-frame" } });
    }
    if (c.id === "c") {
      const marks = [0.02, 0.09, 0.16].map((x) => {
        const p = toBone("leg-ring", [x, LEG_RING.y + 0.004, 0]);
        const q = toBone("leg-ring", [x + 0.025, LEG_RING.y - 0.003, 0]);
        return `M ${r5(p[0])} ${r5(p[1])} L ${r5(q[0])} ${r5(q[1])}`;
      });
      out.push({ view: "elevation", shape: { id: "c-ring-scuffs", bone: "leg-ring-c", role: "texture", filled: false, tone: 0.3, kind: "path", d: marks.join(" "), region: "bentwood-frame" } });
    }
    if (c.id === "a") {
      const p = toBone("seat", [0.12, SEAT.y1 - 0.006, 0]);
      const q = toBone("seat", [0.2, SEAT.y1 - 0.006, 0]);
      out.push({ view: "elevation", shape: { id: "a-worn-lacquer", bone: "seat-a", role: "texture", filled: false, tone: 0.3, kind: "poly", points: rnd([[p[0], p[1]], [q[0], q[1]]]), closed: false, region: "bentwood-frame" } });
    }
  }
  return out;
}

const REP_DECISIONS: Record<Scale, Json> = {
  50: {
    silhouette: T("Side elevation 18 x 18.7 mm: the folded blanket on the floor, on it the two seat-on-seat pairs drawn over each other (the near pair hides the far one): the lower chair upright with its seat ring, cane, rear legs bending at the ring into the reclined back bow, the inner loop, the tapered front legs and the leg ring; the upper chair upside down on it, legs up, its back bow hanging down in front of the lower chair. Front view: the two pairs side by side. Plan: both pairs from above with the upturned legs, leg rings and cane seats of the upper chairs.", "Seitenansicht 18 x 18,7 mm: die gefaltete Decke auf dem Boden, darauf die beiden Sitz-auf-Sitz-Paare übereinander gezeichnet (das nahe Paar verdeckt das ferne): der untere Stuhl aufrecht mit Sitzring, Geflecht, Hinterbeinen, die am Ring in den geneigten Lehnenbogen übergehen, Innenbogen, konischen Vorderbeinen und Beinring; der obere Stuhl kopfüber darauf, Beine nach oben, sein Lehnenbogen hängt vor dem unteren Stuhl herab. Vorderansicht: die beiden Paare nebeneinander. Grundriss: beide Paare von oben mit den hochstehenden Beinen, Beinringen und Geflechtsitzen der oberen Stühle."),
    retained: T("Every bent part of each chair (seat ring, back bow with the rear legs, inner loop, both front legs, leg ring), the cane seat with the tear of the fourth chair, the blanket.", "Jedes gebogene Teil jedes Stuhls (Sitzring, Lehnenbogen mit Hinterbeinen, Innenbogen, beide Vorderbeine, Beinring), der Geflechtsitz mit dem Riss des vierten Stuhls, die Decke."),
    merged: T("The two front legs and the two rear legs of one chair are one projected outline each in the side elevation, where they coincide.", "Die beiden Vorder- und die beiden Hinterbeine eines Stuhls sind in der Seitenansicht je ein projizierter Umriss, weil sie zusammenfallen."),
    omitted: T("The 4 mm screw heads of the leg ring and the 2 mm glue bead are below the 0.2 mm print minimum at 1:75; the bead is a detail stroke, the cane is hidden behind the seat ring in elevation and front view.", "Die 4-mm-Schraubenköpfe des Beinrings und die 2-mm-Leimwulst liegen unter dem Druckminimum von 0,2 mm bei 1:75; die Wulst ist ein Detailstrich, das Geflecht ist in Seiten- und Vorderansicht hinter dem Sitzring verborgen."),
    thinElements: T("The 20 to 30 mm rods are filled outlines of 0.4 to 0.6 mm with a 0.25 mm pen; the 3 mm cane only shows in plan as a filled disc with its weave.", "Die Stäbe von 20 bis 30 mm sind gefüllte Umrisse von 0,4 bis 0,6 mm mit 0,25-mm-Stift; das 3-mm-Geflecht erscheint nur im Grundriss als gefüllte Scheibe mit Webmuster."),
    hatching: T("Quilting of the blanket as a cross hatch in plan; cane weave as diagonal texture strokes; scuffs on the leg ring and worn lacquer on the seat ring as short texture strokes.", "Steppung der Decke als Kreuzschraffur im Grundriss; Geflecht als diagonale Texturstriche; Schrammen am Beinring und abgegriffener Lack am Sitzring als kurze Texturstriche."),
    styles: T("Bentwood mid-dark (0.42-0.48), cane light (0.72), blanket grey (0.55); body 0.5, rods 0.25, seat 0.35, detail 0.25, texture 0.13.", "Bugholz mitteldunkel (0,42-0,48), Geflecht hell (0,72), Decke grau (0,55); Körper 0,5, Stäbe 0,25, Sitz 0,35, Detail 0,25, Textur 0,13."),
    overlays: T("Every wooden part is bentwood-frame, the cane cane-seats, the blanket moving-blanket.", "Jedes Holzteil ist bentwood-frame, das Geflecht cane-seats, die Decke moving-blanket."),
  },
  100: {
    silhouette: T("Side elevation 9 x 9.4 mm with the same parts as 1:50 on simplified outlines (square ring sections, fewer bow points).", "Seitenansicht 9 x 9,4 mm mit denselben Teilen wie 1:50 auf vereinfachten Umrissen (rechteckige Ringquerschnitte, weniger Bogenpunkte)."),
    retained: T("Seat ring, cane with the tear, back bow with rear legs, inner loop, front legs, leg ring, blanket.", "Sitzring, Geflecht mit Riss, Lehnenbogen mit Hinterbeinen, Innenbogen, Vorderbeine, Beinring, Decke."),
    merged: T("Chamfers of the rings are dropped; near and far legs are one outline in elevation.", "Die Fasen der Ringe entfallen; nahe und ferne Beine sind ein Umriss im Aufriss."),
    omitted: T("Glue bead, scuffs and worn lacquer (under 0.2 mm at 1:150).", "Leimwulst, Schrammen und abgegriffener Lack (unter 0,2 mm bei 1:150)."),
    thinElements: T("Rods 0.2 to 0.3 mm filled outlines with a 0.18 mm pen.", "Stäbe 0,2 bis 0,3 mm als gefüllte Umrisse mit 0,18-mm-Stift."),
    hatching: T("Blanket quilting as cross hatch, cane weave at 60 mm.", "Steppung der Decke als Kreuzschraffur, Geflecht mit 60 mm Teilung."),
    styles: T("As 1:50; body 0.35, rods 0.18, seat 0.25.", "Wie 1:50; Körper 0,35, Stäbe 0,18, Sitz 0,25."),
    overlays: T("As 1:50.", "Wie 1:50."),
  },
  200: {
    silhouette: T("Side elevation 4.5 x 4.7 mm: blanket, seat bands, frame bows and inner loops as strokes, front legs as narrow filled bands, leg rings as strokes.", "Seitenansicht 4,5 x 4,7 mm: Decke, Sitzbänder, Gestellbögen und Innenbögen als Striche, Vorderbeine als schmale gefüllte Bänder, Beinringe als Striche."),
    retained: T("Seat with cane, frame bow, inner loop, front legs, leg ring, blanket.", "Sitz mit Geflecht, Gestellbogen, Innenbogen, Vorderbeine, Beinring, Decke."),
    merged: T("Seat ring and cane are one seat band; rear legs and back bow are one continuous stroke from foot to foot.", "Sitzring und Geflecht sind ein Sitzband; Hinterbeine und Lehnenbogen sind ein durchgehender Strich von Fuß zu Fuß."),
    omitted: T("The tear and every trace of use (under 0.2 mm at 1:350).", "Der Riss und alle Gebrauchsspuren (unter 0,2 mm bei 1:350)."),
    thinElements: T("The frame bows are 0.25 mm body strokes on their centre lines, the 20 mm inner loops and the 24 mm leg rings thinner 0.13 mm strokes (26 mm, their true width); the front legs keep their 30 mm top section as filled bands.", "Die Gestellbögen sind 0,25-mm-Körperstriche auf ihren Mittellinien, die 20-mm-Innenbögen und die 24-mm-Beinringe dünnere 0,13-mm-Striche (26 mm, ihre wahre Breite); die Vorderbeine behalten ihren 30-mm-Kopfquerschnitt als gefüllte Bänder."),
    hatching: T("None.", "Keine."),
    styles: T("Body 0.25, inner loops and leg rings 0.13.", "Körper 0,25, Innenbögen und Beinringe 0,13."),
    overlays: T("As 1:50; the cane is drawn as its own disc in plan.", "Wie 1:50; das Geflecht ist im Grundriss eine eigene Scheibe."),
  },
  500: {
    silhouette: T("Chair stack sign 1.8 x 1.9 mm: blanket, four seat slabs, four bow strokes, front leg strokes and leg ring strokes.", "Stuhlstapel-Zeichen 1,8 x 1,9 mm: Decke, vier Sitzplatten, vier Bogenstriche, Vorderbein- und Beinringstriche."),
    retained: T("Stack height, the two pairs, seat height, the hanging backs of the upper chairs.", "Stapelhöhe, die beiden Paare, Sitzhöhe, die herabhängenden Lehnen der oberen Stühle."),
    merged: T("Seat ring and cane as a 75 mm seat slab with both regions; rear legs and bow one stroke.", "Sitzring und Geflecht als 75-mm-Sitzplatte mit beiden Regionen; Hinterbeine und Bogen ein Strich."),
    omitted: T("Inner loops (inside the bow at this scale) and every trace of use.", "Innenbögen (bei diesem Maßstab im Bogen) und alle Gebrauchsspuren."),
    thinElements: T("Every rod and ring is a 0.18 mm body stroke.", "Jeder Stab und Ring ist ein 0,18-mm-Körperstrich."),
    hatching: T("None.", "Keine."),
    styles: T("Body 0.18.", "Körper 0,18."),
    overlays: T("Seat slabs carry bentwood-frame and cane-seats; the rest as 1:50.", "Sitzplatten tragen bentwood-frame und cane-seats; sonst wie 1:50."),
  },
};

function representationOf(s: Scale): Json {
  return { status: "authored", denominator: s, validRange: { minDenominator: RANGES[s][0], maxDenominator: RANGES[s][1] }, pens: PENS[s], hatches: s <= 100 ? [{ id: "quilting", title: T("Quilted blanket seams", "Steppnähte der Decke"), pattern: "cross", angleDeg: 45, spacingMm: s === 50 ? 2.4 : 1.6, penMm: 0.13 }] : [], geometry: geometryOf(s), decisions: REP_DECISIONS[s] };
}
//#endregion

//#region 🧊️ Volumes
const blanketSolid = (s: Scale): Json => ({ id: "blanket", kind: "box", bone: "blanket", material: "moving-blanket", role: "body", tone: 0.55, size: [BLANKET.x, BLANKET.t, BLANKET.z], cornerRadius: s === 50 ? 0.006 : s === 100 ? 0.004 : 0, ...(s >= 200 ? { sheet: true } : {}), transform: { translation: [BLANKET.x / 2, BLANKET.t / 2, 0] } });

/** 🏗️ An own capsule member along a chair-frame segment on a part bone. */
const capsule = (c: Chair, id: string, bone: string, a: V3, b: V3, r: number, extra: Json): Json => ({ id, kind: "capsule", bone: `${bone}-${c.id}`, material: "bentwood-beech", role: "body", tone: 0.42, p0: toBone(bone, a), p1: toBone(bone, b), r0: r, r1: r, ...extra });

/** 📍️ The rig joints of one chair at a scale: front leg tops, bow at the ring, leg ring at the four legs. */
function jointsOf(c: Chair, s: Scale): Json[] {
  const r = Math.max(0.016, feature(s) * 0.55);
  const ell = (id: string, bone: string, p: V3): Json => ({ id: `${c.id}-joint-${id}`, kind: "ellipsoid", bone: `${bone}-${c.id}`, material: "bentwood-beech", role: "detail", tone: 0.3, radii: [r, r, r], transform: { translation: toBone(bone, p) }, member: `${bone}-${c.id}`, part: "joint" });
  const ringAt = (front: boolean, side: number): V3 => {
    const a = front ? FRONT.foot : REAR.foot;
    const b = front ? FRONT.top : REAR.ring;
    const t = (LEG_RING.y - a[1]) / (b[1] - a[1]);
    return [a[0] + (b[0] - a[0]) * t, LEG_RING.y, side * (a[2] + (b[2] - a[2]) * t)];
  };
  if (s === 500) return [ell("front", "front-legs", [FRONT.top[0], FRONT.top[1], 0]), ell("back", "back", [REAR.ring[0], REAR.ring[1], 0])];
  const out = [ell("front-l", "front-legs", [FRONT.top[0], FRONT.top[1], FRONT.top[2]]), ell("front-r", "front-legs", [FRONT.top[0], FRONT.top[1], -FRONT.top[2]]), ell("bow-l", "back", REAR.ring), ell("bow-r", "back", [REAR.ring[0], REAR.ring[1], -REAR.ring[2]])];
  if (s <= 100) out.push(ell("ring-fl", "leg-ring", ringAt(true, 1)), ell("ring-fr", "leg-ring", ringAt(true, -1)), ell("ring-rl", "leg-ring", ringAt(false, 1)), ell("ring-rr", "leg-ring", ringAt(false, -1)));
  return out;
}

/** 🧮️ Mass and centre (chair frame) of every part bone of one chair, from the 1:50 primitives. */
function bodyMass(c: Chair): Record<string, { kg: number; com: V3 }> {
  const acc: Record<string, { kg: number; m: V3 }> = {};
  const add = (bone: string, kg: number, at: V3) => {
    const e = (acc[bone] ??= { kg: 0, m: [0, 0, 0] });
    e.kg += kg;
    e.m = [e.m[0] + kg * at[0], e.m[1] + kg * at[1], e.m[2] + kg * at[2]];
  };
  for (const part of partsOf(c, 50)) {
    if (part.texture) continue;
    const p = part.prim;
    const rho = part.material === "rattan-cane" ? DENSITY.cane : DENSITY.beech;
    if (p.kind === "tube")
      for (let i = 0; i + 1 < p.pts.length; i++) {
        const [a, b] = [p.pts[i]!, p.pts[i + 1]!];
        const L = Math.hypot(...sub(b, a));
        const r = (p.r[i]! + p.r[i + 1]!) / 2;
        add(part.bone, rho * Math.PI * r * r * L, [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2]);
      }
    else if (p.kind === "hoop") add(part.bone, rho * Math.PI * (p.ro * p.ro - p.ri * p.ri) * p.h, p.c);
    else if (p.kind === "disc") add(part.bone, rho * (Math.PI * p.r * p.r - p.holes.reduce((s2, h) => s2 + Math.abs(h.reduce((a2, q, i) => a2 + q[0] * h[(i + 1) % h.length]![1] - h[(i + 1) % h.length]![0] * q[1], 0) / 2), 0)) * (p.y1 - p.y0), [0, (p.y0 + p.y1) / 2, 0]);
  }
  return Object.fromEntries(Object.entries(acc).map(([k, e]) => [k, { kg: e.kg, com: [e.m[0] / e.kg, e.m[1] / e.kg, e.m[2] / e.kg] as V3 }]));
}

/** 🧊️ The three lists of one scale. */
function volumeOf(s: Scale): Json {
  const body: Json[] = [blanketSolid(s)];
  const thermal: Json[] = [{ id: "th-blanket", of: "blanket", region: "moving-blanket" }];
  const structure: Json[] = [{ id: "m-blanket", of: "blanket", member: "blanket", part: "member" }];
  const padH = Math.max(0.008, feature(s));
  const padW = Math.max(0.06, feature(s) * 1.2);
  for (const a of BLANKET_ANCHORS) structure.push({ id: `support-${a.id}`, kind: "box", bone: "blanket", material: "moving-blanket", role: "detail", tone: 0.2, size: [padW, Math.min(padH, BLANKET.t), padW], transform: { translation: [a.x, Math.min(padH, BLANKET.t) / 2, a.z] }, member: "blanket", part: "support", anchor: a.id });
  for (const c of CHAIRS) {
    const parts = partsOf(c, s);
    for (const part of parts) {
      const id = `${c.id}-${part.key}`;
      body.push(solidOf(c, part, id));
      if (!part.texture) thermal.push({ id: `th-${id}`, of: id, region: part.region });
    }
    if (s >= 200) {
      const disc: Part = { key: "cane-skin", bone: "seat", prim: { kind: "disc", c: [0, 0, 0], r: SEAT.ri, y0: s === 200 ? CANE.y1 - 0.003 : SEAT.y1, y1: s === 200 ? CANE.y1 : SEAT.y1 + 0.003, holes: [] }, material: "rattan-cane", region: "cane-seats", tone: 0.72, role: "body", sheet: true, draw: { elevation: "none", front: "none", plan: "none" } };
      thermal.push({ ...solidOf(c, disc, `th-${c.id}-cane`), region: "cane-seats" });
    }
    const member = (bone: string) => `${bone}-${c.id}`;
    if (s <= 100) {
      for (const part of parts.filter((p) => !p.texture)) structure.push({ id: `m-${c.id}-${part.key}`, of: `${c.id}-${part.key}`, member: member(part.bone), part: "member" });
    } else {
      const legs = [1, -1].map((side) => capsule(c, `m-${c.id}-rear-leg-${side > 0 ? "l" : "r"}`, "rear-legs", [REAR.foot[0], REAR.foot[1] + 0.012, side * REAR.foot[2]], [REAR.ring[0], REAR.ring[1], side * REAR.ring[2]], s === 200 ? 0.0145 : 0.0145, { slender: true, member: member("rear-legs"), part: "member" }));
      const top: V3 = [REAR.ring[0] + BOW.slope * (BOW.ySpring + BOW.archH - REAR.ring[1]), BOW.ySpring + BOW.archH, 0];
      const spring = (side: number): V3 => [REAR.ring[0] + BOW.slope * (BOW.ySpring - REAR.ring[1]), BOW.ySpring, side * BOW.zSpring];
      const posts = [1, -1].map((side) => capsule(c, `m-${c.id}-post-${side > 0 ? "l" : "r"}`, "back", [REAR.ring[0], REAR.ring[1], side * REAR.ring[2]], spring(side), 0.013, { slender: true, member: member("back"), part: "member" }));
      const arch = [1, -1].map((side) => capsule(c, `m-${c.id}-arch-${side > 0 ? "l" : "r"}`, "back", spring(side), top, 0.012, { slender: true, member: member("back"), part: "member" }));
      structure.push(...legs, ...posts, ...arch);
      structure.push({ id: `m-${c.id}-seat`, of: `${c.id}-seat`, member: member("seat"), part: "member" });
      for (const side of ["l", "r"]) structure.push({ id: `m-${c.id}-front-leg-${side}`, of: `${c.id}-front-leg-${side}`, member: member("front-legs"), part: "member" });
      structure.push({ id: `m-${c.id}-leg-ring`, of: `${c.id}-leg-ring`, member: member("leg-ring"), part: "member" });
      if (s === 200) structure.push({ id: `m-${c.id}-inner-loop`, of: `${c.id}-inner-loop`, member: member("back"), part: "member" });
    }
    structure.push(...jointsOf(c, s));
  }
  return { status: "authored", denominator: s, body, thermal, structure, decisions: VOLUME_DECISIONS[s] };
}

const VOLUME_DECISIONS: Record<Scale, Json[]> = {
  50: [
    T("1:50 is the full construction of each chair: the steam-bent seat ring (25 x 38 mm, 5 mm chamfers) as a slender hoop, the 3 mm cane sheet set 5 mm into its groove (the fourth chair with its 60 x 65 mm tear as a hole), the rear legs from the floor to the ring overlapping the bow at the bend, the back bow from ring to ring over its arch (Ø29 to Ø24), the inner loop (Ø20), the tapered front legs (Ø24 to Ø30) entering the ring 6 mm and the leg ring (20 x 24 mm) glued to the inside of the four legs; the glue collar of the repaired front leg of the second chair is a texture solid. The blanket is folded twice to 0.9 x 1.0 m and 16 mm.", "1:50 ist die volle Konstruktion jedes Stuhls: der dampfgebogene Sitzring (25 x 38 mm, 5-mm-Fasen) als schlanker Reif, das 3-mm-Geflecht 5 mm in seiner Nut (der vierte Stuhl mit seinem 60 x 65 mm großen Riss als Loch), die Hinterbeine vom Boden bis zum Ring, am Bogen überlappend, der Lehnenbogen von Ring zu Ring über seinen Scheitel (Ø29 bis Ø24), der Innenbogen (Ø20), die konischen Vorderbeine (Ø24 bis Ø30), 6 mm in den Ring eingesteckt, und der Beinring (20 x 24 mm), innen an die vier Beine geleimt; der Leimkragen des reparierten Vorderbeins des zweiten Stuhls ist ein Texturkörper. Die Decke ist zweimal auf 0,9 x 1,0 m und 16 mm gefaltet."),
    T("Thermal references every body solid; structure: every rod, ring and cane sheet as the member of its bone, joints at the leg tops, at the bow beside the ring and where the leg ring meets the legs, four contact pads under the blanket corners, no mass markers: the masses of the bent rods are distributed along their members (bone masses in the rig).", "Thermik verweist auf jeden Körper; Tragwerk: jeder Stab, Ring und das Geflecht als Glied seines Knochens, Gelenke an den Beinköpfen, am Bogen neben dem Ring und wo der Beinring die Beine trifft, vier Kontaktpolster unter den Deckenecken, keine Massenmarken: die Massen der gebogenen Stäbe sind über ihre Glieder verteilt (Knochenmassen im Rig)."),
  ],
  100: [
    T("1:100 keeps every part but draws it simpler: rings of square section without chamfers, the bow on 13 points instead of 27, the inner loop on 12, the rear legs on 5; the tear stays a hole (60 mm is four features), the glue collar is dropped.", "1:100 behält jedes Teil, aber einfacher: Ringe mit rechteckigem Querschnitt ohne Fasen, der Bogen auf 13 statt 27 Punkten, der Innenbogen auf 12, die Hinterbeine auf 5; der Riss bleibt ein Loch (60 mm sind vier Merkmale), der Leimkragen entfällt."),
    T("Thermal references the body; structure references every rod and ring as members, joints of 16 mm radius, 60 mm pads, distributed masses as at 1:50.", "Thermik verweist auf den Körper; Tragwerk verweist auf jeden Stab und Ring als Glieder, Gelenke mit 16 mm Radius, 60-mm-Polster, verteilte Massen wie bei 1:50."),
  ],
  200: [
    T("1:200 merges seat ring and cane into one seat body of 38 mm with a 3 mm dish; rear legs and bow become one slender frame rod from foot to foot at the true section (the 2D draws it as a stroke), the inner loop a slender rod, the leg ring a slender hoop; the front legs keep their 30 mm top section all along (one smallest feature) as filled bands.", "1:200 verschmilzt Sitzring und Geflecht zu einem 38-mm-Sitzkörper mit 3 mm Mulde; Hinterbeine und Bogen werden ein schlanker Gestellstab von Fuß zu Fuß im wahren Querschnitt (die 2D zeichnet ihn als Strich), der Innenbogen ein schlanker Stab, der Beinring ein schlanker Reif; die Vorderbeine behalten durchgehend ihren 30-mm-Kopfquerschnitt (ein kleinstes Merkmal) als gefüllte Bänder."),
    T("Thermal references the body and adds the 3 mm cane skin in the dish of each seat; structure: true-size slender bars for rear legs, posts and arch halves, the seat, front legs and leg ring and inner loop referenced, four joints per chair, 60 mm pads, distributed masses.", "Thermik verweist auf den Körper und ergänzt die 3-mm-Geflechthaut in der Mulde jedes Sitzes; Tragwerk: schlanke Stäbe in wahrer Größe für Hinterbeine, Pfosten und Bogenhälften, Sitz, Vorderbeine, Beinring und Innenbogen referenziert, vier Gelenke je Stuhl, 60-mm-Polster, verteilte Massen."),
  ],
  500: [
    T("1:500 is the stack sign: per chair a 75 mm seat slab (ring and cane merged, top at the true seat height), the frame bow as one slender rod on 11 points, the front legs as slender rods at their Ø21 foot section, the leg ring as a slender hoop; the inner loop is omitted (it lies inside the bow within one feature).", "1:500 ist das Stapelzeichen: je Stuhl eine 75-mm-Sitzplatte (Ring und Geflecht verschmolzen, Oberkante auf wahrer Sitzhöhe), der Gestellbogen als ein schlanker Stab auf 11 Punkten, die Vorderbeine als schlanke Stäbe mit ihrem Fußquerschnitt Ø21, der Beinring als schlanker Reif; der Innenbogen entfällt (er liegt innerhalb eines Merkmals im Bogen)."),
    T("Thermal references the body and lays a 3 mm cane skin on each slab; structure: true-size slender bars for rear legs, posts and arch, seat, front legs and leg ring referenced, two 80 mm joints per chair at the seat, 90 mm pads, distributed masses.", "Thermik verweist auf den Körper und legt eine 3-mm-Geflechthaut auf jede Platte; Tragwerk: schlanke Stäbe in wahrer Größe für Hinterbeine, Pfosten und Bogen, Sitz, Vorderbeine und Beinring referenziert, zwei 80-mm-Gelenke je Stuhl am Sitz, 90-mm-Polster, verteilte Massen."),
  ],
};
//#endregion

//#region 🦴️ Rig and anchors
const BLANKET_ANCHORS = [
  { id: "blanket-rear-left", x: 0.15, z: 0.3 },
  { id: "blanket-rear-right", x: 0.15, z: -0.3 },
  { id: "blanket-front-left", x: 0.75, z: 0.3 },
  { id: "blanket-front-right", x: 0.75, z: -0.3 },
];

function rig(): Json {
  const bones: Json[] = [
    { id: "ground", parent: null, title: T("Ground frame", "Bezugsrahmen"), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: { type: "fixed" } },
    { id: "blanket", parent: "ground", title: T("Folded moving blanket (stays behind)", "Gefaltete Umzugsdecke (bleibt liegen)"), x: -BLANKET.x / 2, y: 0, angleDeg: 0, lengthM: BLANKET.x, joint: { type: "fixed" }, mass: { kg: BLANKET_KG, comX: BLANKET.x / 2, comY: BLANKET.t / 2, distribution: "uniform", source: "authored-estimate", reference: "moving-blanket-catalogue" } },
    { id: "floor-z", parent: "ground", title: T("Lateral floor axis", "Seitliche Bodenachse"), x: 0, y: 0, angleDeg: 0, abductionDeg: 90, lengthM: 0, joint: { type: "fixed" } },
  ];
  for (const c of CHAIRS) {
    const masses = bodyMass(c);
    bones.push(
      { id: `slide-z-${c.id}`, parent: "floor-z", title: T(`Chair ${c.id}: lateral travel`, `Stuhl ${c.id}: seitliche Verschiebung`), x: c.z, y: 0, angleDeg: 0, abductionDeg: -90, lengthM: 0, joint: { type: "prismatic", axisDeg: 0, minM: -0.8, maxM: 0.8 } },
      { id: `slide-x-${c.id}`, parent: `slide-z-${c.id}`, title: T(`Chair ${c.id}: travel ahead`, `Stuhl ${c.id}: Verschiebung nach vorn`), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: { type: "prismatic", axisDeg: 0, minM: -0.3, maxM: 1.2 } },
      { id: `lift-${c.id}`, parent: `slide-x-${c.id}`, title: T(`Chair ${c.id}: lift`, `Stuhl ${c.id}: Anheben`), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: { type: "prismatic", axisDeg: 90, minM: -0.1, maxM: 0.5 } },
      { id: `chair-${c.id}`, parent: `lift-${c.id}`, title: T(`Chair ${c.id}: turn about the seat centre`, `Stuhl ${c.id}: Drehung um die Sitzmitte`), x: 0, y: c.oy, angleDeg: c.angle, lengthM: 0, joint: c.angle === 180 ? { type: "revolute", minDeg: -180, maxDeg: 0 } : { type: "revolute", minDeg: -15, maxDeg: 15 } },
    );
    for (const [b, def] of Object.entries(PART_BONES)) {
      const m = masses[b]!;
      const com = toBone(b, m.com);
      bones.push({ id: `${b}-${c.id}`, parent: `chair-${c.id}`, title: T(`Chair ${c.id}: ${def.title.en.charAt(0).toLowerCase()}${def.title.en.slice(1)}`, `Stuhl ${c.id}: ${def.title.de}`), x: def.o[0], y: def.o[1] - Y0, angleDeg: (boneAngle(b) * 180) / Math.PI, lengthM: boneLength(b), joint: { type: "fixed" }, mass: { kg: m.kg, comX: com[0], comY: com[1], distribution: "point", source: "authored-estimate", reference: "wood-handbook-2021" }, section: SECTIONS[b] });
    }
  }
  return { kind: "structural", root: "ground", bones };
}

const roundSection = (d: number, n: number, title: { en: string; de: string }) => ({ areaM2: (n * Math.PI * d * d) / 4, secondMomentM4: (n * Math.PI * d ** 4) / 64, extremeFibreM: d / 2, material: "bentwood-beech", title });
const SECTIONS: Record<string, Json> = {
  seat: { areaM2: 2 * (SEAT.ro - SEAT.ri) * (SEAT.y1 - SEAT.y0), secondMomentM4: (2 * (SEAT.ro - SEAT.ri) * (SEAT.y1 - SEAT.y0) ** 3) / 12, extremeFibreM: (SEAT.y1 - SEAT.y0) / 2, material: "bentwood-beech", title: T("Both sides of the seat ring, 25 x 38 mm each", "Beide Seiten des Sitzrings, je 25 x 38 mm") },
  "rear-legs": roundSection(0.029, 2, T("Two round legs Ø29 mm", "Zwei Rundbeine Ø29 mm")),
  back: roundSection(0.026, 2, T("Two bow posts Ø26 mm", "Zwei Bogenpfosten Ø26 mm")),
  "front-legs": roundSection(0.024, 2, T("Two tapered legs, Ø24 mm at the foot", "Zwei konische Beine, Ø24 mm am Fuß")),
  "leg-ring": { areaM2: 2 * LEG_RING.w * LEG_RING.h, secondMomentM4: (2 * LEG_RING.w * LEG_RING.h ** 3) / 12, extremeFibreM: LEG_RING.h / 2, material: "bentwood-beech", title: T("Both sides of the leg ring, 20 x 24 mm each", "Beide Seiten des Beinrings, je 20 x 24 mm") },
};

const BLANKET_KG = 1.44;

function anchors(): Json[] {
  const out: Json[] = BLANKET_ANCHORS.map((a) => ({ id: a.id, bone: "blanket", x: a.x, y: 0, z: a.z, kind: "contact", title: T(`Blanket on the floor (${a.id.replace("blanket-", "").replace("-", " ")})`, `Decke auf dem Boden (${a.id.replace("blanket-", "").replace("rear", "hinten").replace("front", "vorn").replace("left", "links").replace("right", "rechts").replace("-", " ")})`), support: "contact", normalDeg: 90, footprintLengthM: 0.3, footprintWidthM: 0.4, frictionCoefficient: 0.5 }));
  for (const c of CHAIRS) {
    for (const [bone, side, z, en, de] of [["rear-legs", "rl", REAR.foot[2], "rear left", "hinten links"], ["rear-legs", "rr", -REAR.foot[2], "rear right", "hinten rechts"], ["front-legs", "fl", FRONT.foot[2], "front left", "vorn links"], ["front-legs", "fr", -FRONT.foot[2], "front right", "vorn rechts"]] as const)
      out.push({ id: `foot-${c.id}-${side}`, bone: `${bone}-${c.id}`, x: 0, y: 0, z, kind: "contact", title: T(`Chair ${c.id}: foot ${en}`, `Stuhl ${c.id}: Fuß ${de}`), support: "contact", normalDeg: 90, footprintLengthM: 0.024, footprintWidthM: 0.024, frictionCoefficient: 0.4 });
  }
  const grip = (c: string, side: number) => ({ id: `grip-${c}`, bone: `seat-${c}`, x: SEAT.ro, y: SEAT.y1 - Y0, z: side * SEAT.ro, kind: "support", title: T(`Hand grip at the seat ring of chair ${c}`, `Handgriff am Sitzring von Stuhl ${c}`), support: "fixed" });
  out.push(grip("a", -1), grip("b", 1));
  return out;
}
//#endregion

//#region 🧍️ Poses and clips
type Place = { dz?: number; dx?: number; dy?: number; turn?: number };
const AREA = { frame: 0, cane: 0, blanket: 0 };
const allFeet = CHAIRS.flatMap((c) => ["rl", "rr", "fl", "fr"].map((s) => `foot-${c.id}-${s}`));
const BL = BLANKET_ANCHORS.map((a) => a.id);

function bonesOf(places: Partial<Record<string, Place>>): Json {
  const out: Json = {};
  for (const [c, p] of Object.entries(places)) {
    if (!p) continue;
    if (p.dz) out[`slide-z-${c}`] = { dx: p.dz };
    if (p.dx) out[`slide-x-${c}`] = { dx: p.dx };
    if (p.dy) out[`lift-${c}`] = { dy: p.dy };
    if (p.turn) out[`chair-${c}`] = { angleDeg: p.turn };
  }
  return out;
}

const binding = (frame: number, cane: number, caneContact?: { fraction: number; tempC: number }, frameContact?: { fraction: number; tempC: number }) => ({ regions: { "bentwood-frame": { exposedFraction: frame, sunlitFraction: 0, ...(frameContact ? { contact: frameContact } : {}) }, "cane-seats": { exposedFraction: cane, sunlitFraction: 0, ...(caneContact ? { contact: caneContact } : {}) }, "moving-blanket": { exposedFraction: 0.5, sunlitFraction: 0, contact: { fraction: 0.5, tempC: 18 } } }, emitters: {} });

const CARRY_Y = 0.76 - (SEAT.y1 - Y0);
const PLACES = {
  carried: { a: { dz: 0.46 - 0.27, dx: 0.85, dy: CARRY_Y - (BLANKET.t + Y0) }, b: { dz: -0.46 - 0.27, dx: 0.85, dy: CARRY_Y - CHAIRS[1].oy, turn: -180 } },
  inUse: { a: { dz: 0.9 - 0.27, dx: 0.85, dy: Y0 - (BLANKET.t + Y0) }, b: { dz: 0.3 - 0.27, dx: 0.85, dy: Y0 - CHAIRS[1].oy, turn: -180 }, c: { dz: -0.3 + 0.27, dx: 0.85, dy: Y0 - (BLANKET.t + Y0) }, d: { dz: -0.9 + 0.27, dx: 0.85, dy: Y0 - CHAIRS[3].oy, turn: -180 } },
};

const OCCUPANT = (factor = 1) => [
  { id: "occupant", kind: "occupant", title: T("Seated adult on chair a (75 kg)", "Sitzende erwachsene Person auf Stuhl a (75 kg)"), bone: "seat-a", x: SEAT.ro - 0.03, y: CANE.y1 - Y0, z: 0, fxN: 0, fyN: -735.5 * factor, source: "authored-estimate", reference: "en-1991-1-1" },
  { id: "occupant-back", kind: "occupant", title: T("Occupant leaning on the bow of chair a", "Person lehnt am Bogen von Stuhl a"), bone: "back-a", x: 0.26, y: 0, z: 0, fxN: -60 * factor, fyN: 0, source: "authored-estimate", reference: "en-12520-2015" },
];

function poses(): Json[] {
  const stored = { id: "stored", title: T("Stored as two seat-on-seat pairs", "Gelagert als zwei Sitz-auf-Sitz-Paare"), description: T("At the flat clearance: on a moving blanket folded to 0.9 x 1.0 m, two pairs side by side, each an upright chair with a second one upside down on its seat, legs up and the back bow hanging in front.", "Bei der Haushaltsauflösung: auf einer auf 0,9 x 1,0 m gefalteten Umzugsdecke zwei Paare nebeneinander, je ein aufrechter Stuhl mit einem zweiten kopfüber auf seinem Sitz, Beine nach oben, der Lehnenbogen hängt vorn herab."), bones: {}, ground: "authored", contacts: BL, thermal: binding(0.95, 1), loads: [] };
  const key = (id: string, en: string, de: string, places: Json, contacts: string[], loads: Json[] = [], th = binding(0.97, 0.8)) => ({ id, key: true, title: T(en, de), description: T(en, de), bones: bonesOf(places), ground: "authored", contacts, thermal: th, loads });
  const carried = { id: "carried", title: T("Carried two at a time", "Zu zweit getragen"), description: T("One mover carries the first pair: a chair in each hand, gripped at the side of the seat ring at knuckle height (0.76 m), feet 0.30 m above the floor, walking ahead off the blanket; the second pair and the blanket stay behind.", "Ein Träger trägt das erste Paar: je Hand einen Stuhl, seitlich am Sitzring in Knöchelhöhe (0,76 m) gefasst, die Füße 0,30 m über dem Boden, vom Lager weg nach vorn; das zweite Paar und die Decke bleiben zurück."), bones: bonesOf(PLACES.carried), ground: "authored", contacts: [...BL, "grip-a", "grip-b"], thermal: binding(0.98, 0.8), loads: [] };
  const inUse = { id: "in-use", title: T("In use at the dining table", "Im Gebrauch am Esstisch"), description: T("Back in a home: the four chairs stand upright side by side along a dining table at 0.6 m place width (DIN 18011), the first one occupied by a seated adult leaning on the bow; the blanket lies folded where the stack stood.", "Wieder in einer Wohnung: die vier Stühle stehen aufrecht nebeneinander an einem Esstisch mit 0,6 m Platzbreite (DIN 18011), auf dem ersten sitzt eine erwachsene Person und lehnt sich an den Bogen; die Decke liegt gefaltet, wo der Stapel stand."), bones: bonesOf(PLACES.inUse), ground: "authored", contacts: [...BL, ...allFeet], thermal: binding(0.99, 0.82, { fraction: 0.18, tempC: 30 }, { fraction: 0.01, tempC: 30 }), loads: OCCUPANT() };
  const pulled = { ...PLACES.inUse, a: { ...PLACES.inUse.a, dx: 0.6 } };
  return [
    stored,
    carried,
    inUse,
    key("carry-lift-upper", "Upper chair lifted off the first pair", "Oberer Stuhl vom ersten Paar abgehoben", { b: { dy: 0.3 } }, [...BL, "grip-b"]),
    key("carry-forward-upper", "Upper chair carried ahead off the blanket", "Oberer Stuhl nach vorn von der Decke getragen", { b: { dy: 0.3, dx: 0.85 } }, [...BL, "grip-b"]),
    key("carry-turn-upper", "Upper chair turned upright beside the mover", "Oberer Stuhl neben dem Träger aufrecht gedreht", PLACES.carried.b ? { b: PLACES.carried.b } : {}, [...BL, "grip-b"]),
    key("in-use-pulled-out", "Chair a pulled out from the table, empty", "Stuhl a vom Tisch abgerückt, leer", pulled, [...BL, ...allFeet], [], binding(0.99, 0.82)),
    key("in-use-seated-pulled-out", "Adult seated on chair a, still pulled out", "Person sitzt auf Stuhl a, noch abgerückt", pulled, [...BL, ...allFeet], OCCUPANT(), binding(0.99, 0.82, { fraction: 0.18, tempC: 30 }, { fraction: 0.01, tempC: 30 })),
  ];
}

function clips(): Json[] {
  return [
    {
      id: "carry",
      title: T("Unstack and carry two at a time", "Abstapeln und zu zweit tragen"),
      description: T("The mover lifts the upper chair off the first pair, carries it ahead off the blanket, turns it upright beside him, then takes the lower chair in the other hand and lifts it; the second pair and the blanket stay.", "Der Träger hebt den oberen Stuhl vom ersten Paar, trägt ihn nach vorn von der Decke, dreht ihn neben sich aufrecht, nimmt dann den unteren Stuhl in die andere Hand und hebt ihn an; das zweite Paar und die Decke bleiben."),
      durationS: 4.6,
      fps: 25,
      loop: false,
      rootMotion: "none",
      keys: [
        { t: 0, pose: "stored", ease: "smooth" },
        { t: 0.8, pose: "carry-lift-upper", ease: "smooth" },
        { t: 1.8, pose: "carry-forward-upper", ease: "smooth" },
        { t: 3.0, pose: "carry-turn-upper", ease: "smooth" },
        { t: 4.2, pose: "carried", ease: "smooth" },
        { t: 4.6, pose: "carried", ease: "smooth" },
      ],
      contacts: [...BL.map((anchor) => ({ anchor, fromS: 0, toS: 4.6 })), { anchor: "grip-b", fromS: 0, toS: 4.6 }, { anchor: "grip-a", fromS: 3.0, toS: 4.6 }],
      phases: [
        { id: "lift-upper", fromS: 0, toS: 0.8, title: T("Lifting the upper chair", "Oberen Stuhl abheben") },
        { id: "carry-ahead", fromS: 0.8, toS: 1.8, title: T("Carrying it off the blanket", "Von der Decke tragen") },
        { id: "turn-upright", fromS: 1.8, toS: 3.0, title: T("Turning it upright", "Aufrecht drehen") },
        { id: "take-lower", fromS: 3.0, toS: 4.2, title: T("Taking the lower chair", "Unteren Stuhl nehmen") },
        { id: "carried", fromS: 4.2, toS: 4.6, title: T("Carried two at a time", "Zu zweit getragen") },
      ],
      events: [
        { t: 0, kind: "contact-on", title: T("Hand grips the upper chair", "Hand fasst den oberen Stuhl"), target: "grip-b" },
        { t: 1.8, kind: "motion", title: T("Upper chair clear of the blanket", "Oberer Stuhl frei von der Decke"), target: "chair-b" },
        { t: 3.0, kind: "contact-on", title: T("Other hand grips the lower chair", "Andere Hand fasst den unteren Stuhl"), target: "grip-a" },
      ],
      tracks: [],
    },
    {
      id: "pull-out-and-sit",
      title: T("Pull out and sit down", "Abrücken und Hinsetzen"),
      description: T("At the dining table chair a is pulled out, an adult sits down (75 kg seat load and 60 N on the bow ramp in) and draws up to the table again.", "Am Esstisch wird Stuhl a abgerückt, eine erwachsene Person setzt sich (75 kg Sitzlast und 60 N am Bogen steigen an) und rückt wieder an den Tisch."),
      durationS: 3,
      fps: 25,
      loop: false,
      rootMotion: "none",
      keys: [
        { t: 0, pose: "in-use-pulled-out", ease: "smooth" },
        { t: 0.6, pose: "in-use-pulled-out", ease: "smooth" },
        { t: 1.4, pose: "in-use-seated-pulled-out", ease: "smooth" },
        { t: 2.6, pose: "in-use", ease: "smooth" },
        { t: 3, pose: "in-use", ease: "smooth" },
      ],
      contacts: [...[...BL, ...allFeet].map((anchor) => ({ anchor, fromS: 0, toS: 3 }))],
      phases: [
        { id: "pulled-out", fromS: 0, toS: 0.6, title: T("Chair pulled out", "Stuhl abgerückt") },
        { id: "sit-down", fromS: 0.6, toS: 1.4, title: T("Sitting down", "Hinsetzen") },
        { id: "draw-up", fromS: 1.4, toS: 2.6, title: T("Drawing up to the table", "An den Tisch rücken") },
        { id: "seated", fromS: 2.6, toS: 3, title: T("Seated", "Sitzend") },
      ],
      events: [
        { t: 1.4, kind: "load-on", title: T("Occupant loads the seat", "Person belastet den Sitz"), target: "occupant" },
        { t: 2.6, kind: "load-on", title: T("Occupant leans on the bow", "Person lehnt am Bogen"), target: "occupant-back" },
      ],
      tracks: [
        { target: "load:occupant", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 0.6, v: 0 }, { t: 1.4, v: 1 }, { t: 3, v: 1 }] },
        { target: "load:occupant-back", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 1.4, v: 0 }, { t: 2.6, v: 1 }, { t: 3, v: 1 }] },
      ],
    },
  ];
}
//#endregion

//#region 🌡️ Thermal, structure, provenance
function thermal(): Json {
  const layer = (id: string, en: string, de: string, kind: string, material: string, t: number, k: number, rho: number, cp: number, ref: string) => ({ id, title: T(en, de), kind, material, thicknessM: t, resistanceM2KW: r5(t / k), heatCapacityJM2K: Math.round(rho * cp * t), source: "literature", reference: ref });
  return {
    environment: { airTempC: 18, meanRadiantTempC: 18, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 },
    operating: [],
    passengers: [],
    regions: [
      { id: "bentwood-frame", title: T("Bentwood frames of the four chairs (aged lacquer)", "Bugholzgestelle der vier Stühle (gealterter Lack)"), kind: "surface", model: "passive", material: "bentwood-beech", areaM2: AREA.frame, characteristicLengthM: 0.027, geometry: "cylinder", skyViewFactor: 0.5, layers: [layer("beech-rod", "Beech rod, half section", "Buchenstab, halber Querschnitt", "timber", "bentwood-beech", 0.013, 0.16, DENSITY.beech, 1600, "iso-10456-2007")], heatCapacityJM2K: Math.round(DENSITY.beech * 1600 * 0.013), ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "iso-10456-2007" },
      { id: "cane-seats", title: T("Cane seats (one torn)", "Geflechtsitze (einer gerissen)"), kind: "surface", model: "passive", material: "rattan-cane", areaM2: AREA.cane, characteristicLengthM: 0.38, geometry: "horizontal-plate-up", skyViewFactor: 0.5, layers: [layer("cane", "Woven cane", "Rohrgeflecht", "timber", "rattan-cane", 0.003, 0.1, DENSITY.cane, 1500, "prosea-rattans-1993")], heatCapacityJM2K: Math.round(DENSITY.cane * 1500 * 0.003), source: "authored-estimate", reference: "prosea-rattans-1993" },
      { id: "moving-blanket", title: T("Folded moving blanket", "Gefaltete Umzugsdecke"), kind: "surface", model: "passive", material: "moving-blanket", areaM2: AREA.blanket, characteristicLengthM: 0.95, geometry: "horizontal-plate-up", skyViewFactor: 0.5, layers: [layer("felt", "Four felt layers", "Vier Filzlagen", "insulation", "moving-blanket", BLANKET.t, 0.045, DENSITY.blanket, 1300, "iso-10456-2007")], heatCapacityJM2K: Math.round(DENSITY.blanket * 1300 * BLANKET.t), source: "authored-estimate", reference: "iso-10456-2007" },
    ],
    emitters: [],
    assumptions: [
      T("Indoor depot and dining room at 18 degC without sun; all surfaces are passive (no power, 0 W).", "Lager und Esszimmer innen bei 18 degC ohne Sonne; alle Flächen sind passiv (keine Leistung, 0 W)."),
      T("Aged, darkened and partly worn lacquer: solar absorptance 0.6 against 0.5 of new light beech lacquer (authored estimate); emissivity 0.9 unchanged.", "Gealterter, nachgedunkelter, teils abgegriffener Lack: solare Absorption 0,6 statt 0,5 bei neuem hellem Buchenlack (Schätzwert); Emissionsgrad 0,9 unverändert."),
      T("Cane weave as a 3 mm sheet with an effective density of 380 kg/m3 (rattan strands about 650 kg/m3, 40 % open weave); the torn seat has 60 x 65 mm less cane.", "Rohrgeflecht als 3-mm-Schicht mit wirksamer Dichte 380 kg/m3 (Rattanstreifen etwa 650 kg/m3, 40 % offenes Geflecht); dem gerissenen Sitz fehlen 60 x 65 mm Geflecht."),
      T("Seat-on-seat, the cane of the lower chair faces the cane of the upper one across 6 mm of still air closed by the two rings: those faces are no outer surface, so areaM2 of the cane is the outer area measured on the stored 1:50 model (0.69 m2 of 0.90 m2 of both faces of four seats). The blanket lies on the floor with half its area in contact at 18 degC.", "Sitz auf Sitz liegt das Geflecht des unteren Stuhls dem des oberen über 6 mm ruhende, von den beiden Ringen abgeschlossene Luft gegenüber: diese Seiten sind keine Außenfläche, daher ist areaM2 des Geflechts die am gelagerten 1:50-Modell gemessene Außenfläche (0,69 m2 von 0,90 m2 beider Seiten der vier Sitze). Die Decke liegt mit der halben Fläche auf dem 18 degC warmen Boden."),
      T("A seated adult covers about 0.07 m2 of one cane seat at 30 degC (ISO 7730 clothing surface), 18 % of the cane area of the four chairs.", "Eine sitzende Person bedeckt etwa 0,07 m2 eines Geflechtsitzes bei 30 degC (Bekleidungsoberfläche nach ISO 7730), 18 % der Geflechtfläche der vier Stühle."),
    ],
  };
}

const STRUCTURE = {
  gravityMS2: 9.80665,
  dynamics: "quasi-static",
  supports: BL,
  assumptions: [
    T("The solver treats each pose as one rigid body on elastic supports: stored, the folded blanket carries the two seat-on-seat pairs on the floor (its four corner contacts); carried, the two hand grips (fixed: the wrist holds the 6 N m roll moment of a chair gripped at the side of its ring) and the blanket under the second pair; in use, the sixteen feet on the floor and the blanket.", "Der Löser behandelt jede Pose als einen starren Körper auf elastischen Auflagern: gelagert trägt die gefaltete Decke die beiden Sitz-auf-Sitz-Paare auf dem Boden (ihre vier Eckkontakte); getragen die beiden Handgriffe (eingespannt: das Handgelenk hält das Rollmoment von 6 N m eines seitlich am Ring gefassten Stuhls) und die Decke unter dem zweiten Paar; im Gebrauch die sechzehn Füße auf dem Boden und die Decke."),
    T("Occupant per EN 1991-1-1 practice and EN 12520: a 75 kg adult (735.5 N) on the cane of chair a and a 60 N lean on its bow (authored estimates); the static seat test of EN 1728 with 1300 N is far above it.", "Person nach EN 1991-1-1 und EN 12520: eine erwachsene Person von 75 kg (735,5 N) auf dem Geflecht von Stuhl a und 60 N Anlehnung am Bogen (Schätzwerte); die statische Sitzprüfung nach EN 1728 mit 1300 N liegt weit darüber."),
    T("Masses from the modelled 1:50 volumes: beech 730 kg/m3 (steam-bent, 12 % moisture), cane 380 kg/m3 effective, blanket 1.44 kg (3.6 m2 at 400 g/m2). Each pair of legs, the seat ring and the leg ring are one member bone each with the section of both sides.", "Massen aus den modellierten 1:50-Volumen: Buche 730 kg/m3 (dampfgebogen, 12 % Feuchte), Geflecht wirksam 380 kg/m3, Decke 1,44 kg (3,6 m2 zu 400 g/m2). Jedes Beinpaar, der Sitzring und der Beinring sind je ein Gliedknochen mit dem Querschnitt beider Seiten."),
  ],
  clinical: false,
};

const PROVENANCE = {
  author: "Semio staffage content (used household objects wave)",
  method: "handcrafted",
  created: "2026-11-22",
  references: [
    { id: "thonet-214", citation: "Thonet GmbH, bentwood chair 214 (formerly No. 14), product data: width 43 cm, height 84 cm, seat height 46 cm, depth 52 cm, beech bentwood, cane seat, https://www.thonet.de/en/all-products/detail/214, accessed 2026-11-20 (the type, not a branded product, is modelled)." },
    { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (density of European beech about 0.71-0.75 g/cm3 at 12 % moisture, modulus of elasticity about 14 GPa, steam bending)." },
    { id: "iso-10456-2007", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity and heat capacity of timber 0.13-0.18 W/mK, 1600 J/kgK; textiles and felts)." },
    { id: "prosea-rattans-1993", citation: "Dransfield, J., Manokaran, N. (eds.) (1993). Plant Resources of South-East Asia No. 6: Rattans. PROSEA, Wageningen (density of cane strands about 0.4-0.75 g/cm3; split cane for chair seats)." },
    { id: "moving-blanket-catalogue", citation: "Catalogue-typical data of furniture moving blankets 180 x 200 cm of recycled cotton and polyester felt, about 350-450 g/m2 and 3-4 mm (authored estimate of the type, no product)." },
    { id: "en-1728-2012", citation: "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (seat static load 1300 N, back static load)." },
    { id: "en-12520-2015", citation: "EN 12520:2015 Furniture - Strength, durability and safety - Requirements for domestic seating." },
    { id: "en-1991-1-1", citation: "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: General actions - Densities, self-weight, imposed loads for buildings." },
    { id: "din-18011-2006", citation: "DIN 18011:2006 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (place width of 60 cm per person at the dining table)." },
    { id: "din-33402-2-2020", citation: "DIN 33402-2:2020 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (knuckle height of standing adults about 0.70-0.80 m)." },
    { id: "iso-7730-2005", citation: "ISO 7730:2005 Ergonomics of the thermal environment (clothing surface temperatures of seated persons around 28-32 degC)." },
    { id: "incropera-2018", citation: "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of lacquered wood and textiles about 0.9)." },
  ],
  notes: T(
    "Four second-hand bentwood café chairs of the Viennese No. 14 / 214 type from a flat clearance. Per chair: steam-bent beech seat ring Ø42 cm (25 x 38 mm), hand-woven cane seat, one bow that forms the rear legs and the back loop (Ø29 at the foot to Ø24 at the top), an inner back loop Ø20, two tapered front legs Ø30 to Ø24 screwed into the ring, a leg ring 20 x 24 mm at 0.2 m; 84 x 43 x 52 cm, seat height 46 cm, 3.0 kg modelled. Traces of use: worn lacquer on the seat rings and bow tops, scuffed leg rings where shoes rest, the reglued front leg of chair b with its glue bead, the torn cane of chair d (60 x 65 mm, still sittable at the back). Stored at the clearance as two seat-on-seat pairs on a folded moving blanket. Parameters are literature values or authored estimates; nothing is measured.",
    "Vier gebrauchte Bugholz-Caféhausstühle vom Wiener Typ Nr. 14 / 214 aus einer Haushaltsauflösung. Je Stuhl: dampfgebogener Buchen-Sitzring Ø42 cm (25 x 38 mm), handgeflochtener Rohrsitz, ein Bogen, der Hinterbeine und Rückenschlaufe bildet (Ø29 am Fuß bis Ø24 oben), ein Innenbogen Ø20, zwei konische Vorderbeine Ø30 bis Ø24, in den Ring geschraubt, ein Beinring 20 x 24 mm auf 0,2 m; 84 x 43 x 52 cm, Sitzhöhe 46 cm, 3,0 kg modelliert. Gebrauchsspuren: abgegriffener Lack an Sitzringen und Bogenscheiteln, verschrammte Beinringe, wo die Schuhe ruhen, das neu verleimte Vorderbein von Stuhl b mit seiner Leimwulst, das gerissene Geflecht von Stuhl d (60 x 65 mm, hinten noch besitzbar). Bei der Auflösung als zwei Sitz-auf-Sitz-Paare auf einer gefalteten Umzugsdecke gelagert. Parameter sind Literaturwerte oder Schätzwerte; nichts ist gemessen.",
  ),
};
//#endregion

/** 📐️ Outer areas of the three regions from the 1:50 primitives (rod mantles, ring faces, both cane faces, blanket box). */
function areas(): void {
  for (const c of CHAIRS)
    for (const part of partsOf(c, 50)) {
      if (part.texture) continue;
      const p = part.prim;
      if (p.kind === "tube") for (let i = 0; i + 1 < p.pts.length; i++) AREA.frame += Math.PI * (p.r[i]! + p.r[i + 1]!) * Math.hypot(...sub(p.pts[i + 1]!, p.pts[i]!));
      else if (p.kind === "hoop") AREA.frame += 2 * Math.PI * (p.ro + p.ri) * p.h + 2 * Math.PI * (p.ro * p.ro - p.ri * p.ri);
      else if (p.kind === "disc") AREA.cane += 2 * Math.PI * p.r * p.r - (p.holes.length ? 2 * 0.0035 : 0);
    }
  AREA.blanket = 2 * BLANKET.x * BLANKET.z + 2 * (BLANKET.x + BLANKET.z) * BLANKET.t;
  AREA.frame = r5(AREA.frame * FRAME_AREA_FACTOR);
  AREA.cane = r5(AREA.cane * CANE_AREA_FACTOR);
  AREA.blanket = r5(AREA.blanket);
}

/** ⚖️ Hand corrections of the analytic areas to the outer area of the union (overlaps of legs, bow and ring buried in each other; measured by `volumes-thermal-coverage`, see the report). */
const FRAME_AREA_FACTOR = 1;
const CANE_AREA_FACTOR = 0.766;

function main(): void {
  areas();
  const masses = CHAIRS.map((c) => Object.values(bodyMass(c)).reduce((s, m) => s + m.kg, 0));
  const massKg = masses.reduce((a, b) => a + b, 0) + BLANKET_KG;
  const representations = Object.fromEntries(SCALES.map((s) => [String(s), representationOf(s)]));
  const volumes = Object.fromEntries(SCALES.map((s) => [String(s), volumeOf(s)]));
  const asset = {
    id: "used-bentwood-dining-chairs",
    category: "furniture",
    title: T("Used Bentwood Dining Chairs", "Gebrauchte Bugholzstühle"),
    summary: T(
      `Four second-hand bentwood café chairs of the Viennese No. 14 type from a flat clearance, 84 cm high with a 46 cm seat, each a steam-bent beech seat ring with a cane seat, one bow forming rear legs and back loop, an inner loop, two tapered front legs and a leg ring. Their lacquer is worn, one front leg is reglued and one cane seat is torn. They are stored as two seat-on-seat pairs on a folded moving blanket, carried two at a time and set out again side by side at a dining table, one with a seated adult.`,
      `Vier gebrauchte Bugholz-Caféhausstühle vom Wiener Typ Nr. 14 aus einer Haushaltsauflösung, 84 cm hoch mit 46 cm Sitzhöhe, je ein dampfgebogener Buchen-Sitzring mit Rohrgeflecht, ein Bogen als Hinterbeine und Rückenschlaufe, ein Innenbogen, zwei konische Vorderbeine und ein Beinring. Ihr Lack ist abgegriffen, ein Vorderbein ist neu verleimt und ein Geflechtsitz gerissen. Sie lagern als zwei Sitz-auf-Sitz-Paare auf einer gefalteten Umzugsdecke, werden zu zweit getragen und wieder nebeneinander an einen Esstisch gestellt, einer mit einer sitzenden Person.`,
    ),
    tags: ["used", "household", "second-hand", "living", "kitchen", "chair", "bentwood", "beech", "cane", "stack", "moving-blanket"],
    provenance: PROVENANCE,
    dimensions: { heightM: r5(CHAIRS[1].oy + Y0), widthM: BLANKET.x, depthM: BLANKET.z, massKg: Math.round(massKg * 100) / 100 },
    materials: MATERIALS,
    rig: rig(),
    volumes,
    anchors: anchors(),
    representations,
    symbols: [],
    poses: poses(),
    clips: clips(),
    thermal: thermal(),
    structure: STRUCTURE,
    presentation: { defaultPose: "stored", defaultClip: "carry", defaultScale: 50 },
    exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    completion: "authored",
  };
  writeFileSync(OUT, `${JSON.stringify(rnd({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }), null, 1)}\n`);
  console.log(`[DEBUG] wrote ${OUT}: mass per chair ${masses.map((m) => m.toFixed(3)).join(", ")} kg, total ${massKg.toFixed(2)} kg; areas frame ${AREA.frame}, cane ${AREA.cane}, blanket ${AREA.blanket} m2; leg ring cx ${RING.cx.toFixed(4)} R ${RING.R.toFixed(4)}; dims ${JSON.stringify(asset.dimensions)}`);
}

main();
