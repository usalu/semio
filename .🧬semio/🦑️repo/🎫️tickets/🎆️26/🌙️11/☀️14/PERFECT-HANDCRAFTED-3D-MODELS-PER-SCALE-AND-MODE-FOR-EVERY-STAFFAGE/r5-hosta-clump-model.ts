/** 🌿️ Round 5, vegetation/hosta-clump: builds the candidate asset (rig root fix, 2D redrawn per ruling E1 at every drawn scale and view, plan symbol, texts) and the volumes source (body, thermal, structure at 1:50, 1:100, 1:200, 1:500) from the hand-stated clump of `r5-hosta-clump-plant.ts`.
 * Usage: bun r5-hosta-clump-model.ts [--source]   (candidate: 🗑️generated/r3-hosta-clump/🔣️.json; --source also writes volumes/vegetation/hosta-clump.json)
 * @see ./r3-vegetation-hosta-clump.md (Round 5 handcraft) */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, TICKET, entryOf, toLocal, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, cross, dirOf, ellipsoidOf, loftOf, mul, r4, r5, sideOf, table, type Section } from "./r3-agave-kit.ts";
import { ALL, INNER, OUTER, SHOOTS, STATIONS, bladeId, cupDeg, outerWavy, petioleA, petioleB, petioleId, regionOf, shootCentre, wingAxis, wingPoint, type Built } from "./r5-hosta-clump-plant.ts";
import { OUTLETS, OUTLET_D, OUTLET_S, fixRig, rigCheck, shootFrame, shootId } from "./r5-hosta-clump-rig.ts";
import { DECISIONS, REP_DECISIONS, TEXTS } from "./r5-hosta-clump-texts.ts";

type Scale = 50 | 100 | 200 | 500;
type View = "elevation" | "front" | "plan";
const SCALES: Scale[] = [50, 100, 200, 500];
const DRAWN: Scale[] = [50, 100, 200];
const VIEWS: View[] = ["elevation", "front", "plan"];
const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const say = (line: string) => process.stdout.write(`${line}\n`);

const { entry } = entryOf("vegetation/hosta-clump");
const doc = JSON.parse(readFileSync(entry.path, "utf8"));
const asset = doc.asset;
for (const r of fixRig(asset)) say(`[DEBUG] ${r.slice(0, 600)}`);
for (const r of rigCheck(asset)) if (Number(r.split("error ")[1]) > 2e-4) say(`[DEBUG] RIG MISMATCH ${r}`);
say(`[DEBUG] rig check: ${rigCheck(asset).length} poses and keys solved`);

const toneOf = (b: Built) => (b.leaf.tier === "a" ? r4(0.38 + 0.004 * b.leaf.k) : b.leaf.tier === "b" ? r4(0.47 + 0.004 * (b.leaf.k - 10)) : r4(0.56 + 0.005 * (b.leaf.k - 19)));
const BLADE = (b: Built) => ({ material: "hosta-blade", role: "body", tone: toneOf(b) });
const PETIOLE = { material: "hosta-petiole", role: "body", tone: 0.33 };

/** 🫓️ Thickness of the lamina along the midrib: fleshy where the veins run together into the petiole, thin towards margin and tip. The base holds the smallest feature of the scale. */
const LAM: Record<50 | 100, [number, number][]> = {
  50: [[-0.13, 0.0034], [-0.04, 0.0058], [0, 0.0076], [0.05, 0.0076], [0.15, 0.0052], [0.35, 0.0036], [1, 0.0024]],
  100: [[-0.13, 0.006], [-0.04, 0.011], [0, 0.0152], [0.06, 0.0152], [0.22, 0.009], [0.5, 0.0062], [1, 0.0044]],
};
const S100 = [-0.13, -0.105, -0.06, 0, 0.1, 0.22, 0.36, 0.52, 0.68, 0.82, 0.93, 1];
const S200 = [-0.13, -0.07, 0.02, 0.2, 0.45, 0.7, 0.9, 1];
const S500 = [-0.13, -0.03, 0.2, 0.5, 0.8, 1];
const VEINS = [0.12, 0.22, 0.32, 0.42, 0.52, 0.62, 0.72, 0.82, 0.92];
const DRAWN_VEIN = 5;
/** 📏️ The clump as modelled (summer-full, margins and tips of the 1:50 model), within Grenfell and Shadrack (2009): mound 0.6-0.75 m high and 0.9-1.2 m wide. */
const DIMENSIONS = { heightM: 0.618, widthM: 0.982, depthM: 0.986 };
const VEIN_S = [0.012, 0.04, 0.08, 0.14, 0.22, 0.32, 0.44, 0.56, 0.68, 0.78, 0.86, 0.92];
const outerOf = (b: Built, s: number, scale: Scale) => (scale === 50 ? outerWavy(b.leaf, s) : table(OUTER, s));
const innerOf = (s: number) => (s < 0 ? table(INNER, s) : 0);
const veinD = (b: Built, f: number, s: number) => f * outerWavy(b.leaf, s) * (1 - Math.exp(-s / 0.055));

/** 🪽️ The sections of one blade half: from the lobe behind the junction to the tip, the half spanning from the sinus (or a little across the midrib) to the margin, tilted up by the cup angle. */
function wing(b: Built, sign: 1 | -1, ss: readonly number[], scale: 50 | 100): Section[] {
  return ss.map((s) => {
    const out = outerOf(b, s, scale), inn = s < 0 ? innerOf(s) : -0.035;
    const x = wingAxis(b, s, sign);
    return { p: wingPoint(b, s, sign, (inn + out) / 2), x, y: cross(b.mid.tangent(s), x), a: Math.max(((out - inn) / 2) * b.leaf.W * 0.5, 0.0016), b: table(LAM[scale], s) / 2, n: 2.6 };
  });
}

/** 🫘️ The lens of 1:200 and 1:500: the cupped blade from the midrib underside to the margin tops in one section. */
function lensAt(b: Built, s: number, scale: 200 | 500) {
  const hw = table(OUTER, s) * b.leaf.W * 0.5 * Math.cos(cupDeg(b.leaf, s) * D2R);
  const bottom = -0.005, top = hw * Math.tan(cupDeg(b.leaf, s) * D2R) + 0.004;
  const half = Math.max((top - bottom) / 2, scale === 200 ? (FEATURE[200] / 2 + 0.0056) * Math.min(1, table(OUTER, s) + 0.25) : 0.006);
  return { hw: Math.max(hw, 0.004), mid: (top + bottom) / 2, half };
}
const lens = (b: Built, ss: readonly number[], scale: 200 | 500): Section[] => ss.map((s) => {
  const q = lensAt(b, s, scale);
  return { p: add(b.mid.at(s), mul(b.mid.normal(s), q.mid)), x: b.side, y: cross(b.mid.tangent(s), b.side), a: q.hw, b: q.half, n: 2.2 };
});

/** 🌱️ Half size of the clasping petiole base at its insertion: the real flared base at 1:50, and at 1:100 and 1:200 the base as it swells into the bundle of the shoot (it holds the smallest feature there, inside the shoot neck); the stalk above keeps its true section at every scale. */
const BASE: Record<Scale, number> = { 50: 0, 100: 0.0077, 200: 0.0156, 500: 0 };
const PET_T: Record<Scale, number[]> = { 50: [0, 0.06, 0.16, 0.3, 0.45, 0.6, 0.75, 0.88, 1, 1.03], 100: [0, 0.1, 0.25, 0.5, 0.75, 1.03], 200: [0, 0.1, 0.5, 1.02], 500: [0, 0.55, 1] };
function petiole(b: Built, scale: Scale, share = 1): Section[] {
  const size = (t: number, f: (l: Built["leaf"], t: number) => number) => (scale === 500 ? f(b.leaf, 0.5) : t === 0 ? Math.max(f(b.leaf, 0), BASE[scale]) : f(b.leaf, Math.min(1, t))) * share;
  return PET_T[scale].map((t) => ({ p: b.pet.at(t), x: b.side, y: cross(b.pet.tangent(t), b.side), a: size(t, petioleA), b: size(t, petioleB), n: 2.6 }));
}

const RIB_S: Record<Scale, number[]> = { 50: [-0.02, 0.05, 0.15, 0.3, 0.45, 0.6, 0.75, 0.88, 0.97], 100: [0, 0.2, 0.45, 0.7, 0.95], 200: [0, 0.35, 0.7, 0.95], 500: [0, 0.5, 0.95] };
const RIB_A: [number, number][] = [[-0.02, 0.0052], [0.1, 0.0048], [0.5, 0.0032], [0.97, 0.0012]], RIB_B: [number, number][] = [[-0.02, 0.0046], [0.1, 0.004], [0.5, 0.0027], [0.97, 0.0011]];
const rib = (b: Built, scale: Scale, drop: number): Section[] => RIB_S[scale].map((s, i) => ({ p: add(b.mid.at(s), mul(b.mid.normal(s), -drop)), x: b.side, y: cross(b.mid.tangent(s), b.side), a: Math.max(table(RIB_A, s), i === 0 ? BASE[scale] : 0), b: Math.max(table(RIB_B, s), i === 0 ? BASE[scale] : 0) }));

function vein(b: Built, sign: 1 | -1, f: number, extra: number): Section[] {
  return VEIN_S.map((s) => {
    const x = wingAxis(b, s, sign);
    return { p: wingPoint(b, s, sign, veinD(b, f, s)), x, y: cross(b.mid.tangent(s), x), a: 0.0013, b: table(LAM[50], s) / 2 + extra };
  });
}
const LOBE_VEIN: [number, number][] = [[0.004, 0.035], [-0.04, 0.22], [-0.085, 0.37]];
const lobeVein = (b: Built, sign: 1 | -1): Section[] => LOBE_VEIN.map(([s, d]) => {
  const x = wingAxis(b, s, sign);
  return { p: wingPoint(b, s, sign, d), x: b.mid.tangent(s), y: mul(cross(b.mid.tangent(s), x), -1), a: 0.0013, b: table(LAM[50], s) / 2 + 0.0009 };
});

const NECK: Record<Scale, [number, number][]> = {
  50: [[0, 1], [0.25, 1], [0.5, 0.9], [0.75, 0.72], [0.92, 0.52], [1, 0.3]],
  100: [[0, 1], [0.4, 0.95], [0.8, 0.66], [1, 0.34]],
  200: [[0, 1], [0.5, 0.88], [1, 0.4]],
  500: [[0, 1], [1, 0.45]],
};

type Draft = { kind: "plate"; id: string; bone: string; F: Frame; len: number; ring: V3[]; lows: V3[][]; highs: V3[][]; flat?: boolean; tone: number; region: string; centre: V3 } | { kind: "stroke"; id: string; bone: string; F: Frame; len: number; pts: V3[]; tone: number; role: string; pen?: number; region?: string; centre: V3 } | { kind: "revolved"; id: string; bone: string; profile: [number, number][]; tone: number; region: string; centre: V3 };
type Parts = { body: Json[]; thermal: Json[]; structure: Json[]; drafts: Draft[] };

const between = (ss: readonly number[], a: number, b: number): number[] => [a, ...ss.filter((s) => s > a + 1e-6 && s < b - 1e-6), b];
const SEGMENTS: Record<50 | 100 | 200, [number, number][]> = { 50: [[0, 0.2], [0.2, 0.4], [0.4, 0.6], [0.6, 0.8], [0.8, 1]], 100: [[0, 0.25], [0.25, 0.52], [0.52, 0.78], [0.78, 1]], 200: [[0, 0.3], [0.3, 0.7], [0.7, 1]] };
const LOBES: Record<50 | 100 | 200, [number, number][]> = { 50: [[-0.13, 0]], 100: [[-0.13, 0]], 200: [[-0.13, 0]] };

/** ✏️ The plates, strokes and profiles that draw one leaf at one scale. */
function draftLeaf(b: Built, scale: 50 | 100 | 200, drafts: Draft[]): void {
  const l = b.leaf, bone = bladeId(l.k), tone = toneOf(b), region = regionOf(l);
  const ss = scale === 50 ? STATIONS : scale === 100 ? S100 : S200;
  LOBES[scale].forEach(([a0, a1], i) => {
    const st = between(ss, a0, a1), centre = b.mid.at((a0 + a1) / 2);
    if (scale === 200) {
      const q = st.map((s) => ({ s, ...lensAt(b, s, 200) }));
      const at = (x: (typeof q)[number], sg: number, v: number) => add(add(b.mid.at(x.s), mul(b.side, sg * x.hw)), mul(b.mid.normal(x.s), v));
      drafts.push({ kind: "plate", id: `${bone}-b${i + 1}`, bone, F: b.fb, len: r4(b.Lb), ring: [...q.map((x) => at(x, 1, x.mid)), ...[...q].reverse().map((x) => at(x, -1, x.mid))], lows: q.map((x) => [at(x, 0, x.mid - x.half)]), highs: q.map((x) => [at(x, 0, x.mid + x.half)]), flat: true, tone, region, centre });
      return;
    }
    const half = (s: number) => table(LAM[scale], s) / 2;
    for (const sg of [1, -1] as const) {
      const ring = [...st.map((s) => wingPoint(b, s, sg, outerOf(b, s, scale))), ...[...st].reverse().filter((s) => innerOf(s) < outerOf(b, s, scale) - 1e-6).map((s) => wingPoint(b, s, sg, innerOf(s)))];
      drafts.push({ kind: "plate", id: `${bone}-b${sg === 1 ? "l" : "r"}${i + 1}`, bone, F: b.fb, len: r4(b.Lb), ring, lows: st.map((s) => [wingPoint(b, s, sg, outerOf(b, s, scale), -0.5 * half(s)), wingPoint(b, s, sg, innerOf(s), -0.5 * half(s))]), highs: st.map((s) => [wingPoint(b, s, sg, outerOf(b, s, scale), 0.5 * half(s)), wingPoint(b, s, sg, innerOf(s), 0.5 * half(s))]), flat: true, tone, region, centre });
    }
  });
  SEGMENTS[scale].forEach(([a0, a1], i, all) => {
    const lo = a0 - 0.008, hi = i < all.length - 1 ? a1 + 0.008 : a1;
    const st = between(ss, lo, hi);
    const ring: V3[] = [], lows: V3[][] = [], highs: V3[][] = [];
    if (scale === 200) {
      for (const sg of [1, -1]) for (const s of sg === 1 ? st : [...st].reverse()) {
        const q = lensAt(b, s, 200);
        ring.push(add(add(b.mid.at(s), mul(b.side, sg * q.hw)), mul(b.mid.normal(s), q.mid)));
        if (sg === 1) {
          lows.push([add(b.mid.at(s), mul(b.mid.normal(s), q.mid - q.half))]);
          highs.push([add(b.mid.at(s), mul(b.mid.normal(s), q.mid + q.half))]);
        }
      }
    } else {
      const half = (s: number) => table(LAM[scale], s) / 2;
      const out = (s: number) => outerOf(b, s, scale);
      const split = a0 < 0.7 ? 0.5 : 1;
      for (const s of st) ring.push(wingPoint(b, s, 1, split * out(s)));
      for (const s of [...st].reverse()) ring.push(wingPoint(b, s, -1, split * out(s)));
      for (const s of st) {
        lows.push([add(b.mid.at(s), mul(b.mid.normal(s), -0.5 * half(s))), ...(split === 1 ? ([1, -1] as const).map((sg) => wingPoint(b, s, sg, out(s), -half(s))) : [])]);
        highs.push(([1, -1] as const).map((sg) => wingPoint(b, s, sg, split * out(s), half(s) * (split === 1 ? 0.2 : 1))));
      }
      if (split < 1) for (const sg of [1, -1] as const) drafts.push({ kind: "plate", id: `${bone}-p${i + 1}${sg === 1 ? "l" : "r"}`, bone, F: b.fb, len: r4(b.Lb), ring: [...st.map((s) => wingPoint(b, s, sg, out(s))), ...[...st].reverse().map((s) => wingPoint(b, s, sg, split * out(s)))], lows: st.map((s) => [wingPoint(b, s, sg, split * out(s), -half(s))]), highs: st.map((s) => [wingPoint(b, s, sg, out(s), half(s) * 0.2)]), tone, region, centre: wingPoint(b, (a0 + a1) / 2, sg, 0.75 * out((a0 + a1) / 2), 0.001) });
    }
    const centre = b.mid.at((a0 + a1) / 2);
    drafts.push({ kind: "plate", id: `${bone}-p${i + 1}`, bone, F: b.fb, len: r4(b.Lb), ring, lows, highs, tone, region, centre });
    if (scale !== 200) drafts.push({ kind: "stroke", id: `${bone}-m${i + 1}`, bone, F: b.fb, len: r4(b.Lb), pts: between(scale === 50 ? RIB_S[50] : RIB_S[100], Math.max(0, lo), Math.min(0.97, hi)).map((s) => add(b.mid.at(s), mul(b.mid.normal(s), 0.002))), tone: 0.25, role: "texture", pen: scale === 50 ? 0.18 : 0.13, centre: add(centre, mul(b.mid.normal(0.5), 0.004)) });
    if (scale === 50 && hi > 0.05 && lo < 0.9) for (const sg of [1, -1] as const) drafts.push({ kind: "stroke", id: `${bone}-v${sg === 1 ? "l" : "r"}${i + 1}`, bone, F: b.fb, len: r4(b.Lb), pts: between(VEIN_S, Math.max(0.012, lo), Math.min(0.92, hi)).map((s) => wingPoint(b, s, sg, veinD(b, VEINS[DRAWN_VEIN]!, s), 0.001)), tone: 0.3, role: "texture", centre: add(centre, mul(b.mid.normal(0.5), 0.005)) });
  });
  const pb = petioleId(l.k);
  const pieces: [number, number][] = [[0, 0.5], [0.5, 1.02]];
  pieces.forEach(([t0, t1], i) => {
    const ts = between(PET_T[scale], i > 0 ? t0 - 0.02 : t0, t1);
    const centre = b.pet.at((t0 + t1) / 2);
    const q = ts.map((t) => ({ c: b.pet.at(t), n: b.pet.normal(t), a: 0.87 * (t === 0 ? Math.max(petioleA(l, 0), BASE[scale]) : petioleA(l, Math.min(1, t))), h: 0.87 * (t === 0 ? Math.max(petioleB(l, 0), BASE[scale]) : petioleB(l, Math.min(1, t))) }));
    drafts.push({ kind: "plate", id: `${pb}-p${i + 1}`, bone: pb, F: b.fa, len: r4(b.La), ring: [...q.map((x) => add(x.c, mul(b.side, x.a))), ...[...q].reverse().map((x) => add(x.c, mul(b.side, -x.a)))], lows: q.map((x) => [add(x.c, mul(x.n, -x.h))]), highs: q.map((x) => [add(x.c, mul(x.n, x.h))]), tone: 0.33, region: "petioles", centre });
  });
}

/** 🏗️ All solids and drawing drafts of one scale. */
function build(scale: Scale): Parts {
  const f = FEATURE[scale];
  const floor = (x: number) => (scale === 500 ? x : Math.max(x, f / 2 + 0.0006));
  const body: Json[] = [], thermal: Json[] = [], structure: Json[] = [], drafts: Draft[] = [];
  const put = (solid: Json, region: string) => { body.push(solid); thermal.push({ id: `th-${solid.id}`, of: solid.id, region }); };
  SHOOTS.forEach((s, i) => {
    const bone = shootId(s.n), F = shootFrame(i), c = shootCentre(s);
    const profile = NECK[scale].map(([h, r]) => [r5(h * s.h), r5(r * s.neck)] as [number, number]);
    put({ id: `${bone}-neck`, kind: "revolve", bone, ...PETIOLE, tone: 0.3, axis: [1, 0, 0], origin: [0, 0, 0], profile }, "petioles");
    if (scale !== 500) drafts.push({ kind: "revolved", id: `${bone}-neck`, bone, profile, tone: 0.3, region: "petioles", centre: add(c, [0, s.h / 2, 0]) });
    if (scale === 50) [25, 150, 265].forEach((d, n) => {
      const phi = s.az + d + 12 * i, rho = dirOf(phi, 0), tau = sideOf(phi);
      const rows: [number, number, number, number][] = [[0, 0.94, 0.013, 0.004], [0.3, 0.95, 0.0125, 0.004], [0.62, 0.84, 0.008, 0.0032], [0.9, 0.64, 0.0018, 0.0014]];
      body.push(loftOf(`${bone}-scale-${n + 1}`, bone, F, rows.map(([h, r, a, t]) => ({ p: add(add(c, mul(rho, r * s.neck)), [0, h * s.h, 0]), x: tau, y: rho, a, b: t, n: 2.4 })), { material: "hosta-petiole", role: "detail", tone: 0.22 }));
    });
    const core = floor(0.014);
    structure.push({ id: `m-${bone}`, kind: "revolve", bone, material: "hosta-petiole", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[0, r5(core)], [r5(s.h * 0.9), r5(Math.max(0.6 * core, scale === 500 ? 0 : f / 2 + 0.0006))]], member: bone, part: "member" });
    if (scale <= 100) structure.push({ id: `j-${bone}`, kind: "ellipsoid", bone, material: "hosta-petiole", role: "body", tone: 0.5, radii: [r5(floor(0.008)), 0.019, 0.019], transform: { translation: [r5(floor(0.008)), 0, 0] }, member: bone, part: "joint" });
  });
  for (const b of ALL) {
    const l = b.leaf, pb = petioleId(l.k), bb = bladeId(l.k), region = regionOf(l);
    put(loftOf(pb, pb, b.fa, petiole(b, scale), PETIOLE), "petioles");
    if (scale <= 100) {
      const ss = scale === 50 ? STATIONS : S100;
      put(loftOf(`${bb}-l`, bb, b.fb, wing(b, 1, ss, scale as 50 | 100), BLADE(b)), region);
      put(loftOf(`${bb}-r`, bb, b.fb, wing(b, -1, ss, scale as 50 | 100), BLADE(b)), region);
    } else put(loftOf(bb, bb, b.fb, lens(b, scale === 200 ? S200 : S500, scale as 200 | 500), BLADE(b)), region);
    if (scale === 50) {
      put(loftOf(`${bb}-midrib`, bb, b.fb, rib(b, 50, 0.0022), { material: "hosta-blade", role: "detail", tone: r4(toneOf(b) - 0.1) }), region);
      for (const sg of [1, -1] as const) {
        const side = sg === 1 ? "l" : "r";
        VEINS.forEach((fr, n) => body.push(loftOf(`${bb}-vein-${side}${n + 1}`, bb, b.fb, vein(b, sg, fr, 0.0009), { material: "hosta-blade", role: "texture", tone: r4(toneOf(b) - 0.08), texture: true })));
        body.push(loftOf(`${bb}-vein-${side}0`, bb, b.fb, lobeVein(b, sg), { material: "hosta-blade", role: "texture", tone: r4(toneOf(b) - 0.08), texture: true }));
      }
    }
    if (scale !== 500) draftLeaf(b, scale, drafts);
    structure.push(scale === 50 ? { ...loftOf(`m-${pb}`, pb, b.fa, petiole(b, scale, 0.6), { material: "hosta-petiole", role: "body", tone: 0.5 }), member: pb, part: "member" } : { id: `m-${pb}`, of: pb, member: pb, part: "member" });
    structure.push({ ...loftOf(`m-${bb}`, bb, b.fb, rib(b, scale, 0.002), { material: "hosta-blade", role: "body", tone: 0.5 }), member: bb, part: "member" });
    if (scale === 50) for (const sg of [1, -1] as const) structure.push({ ...loftOf(`m-${bb}-rib-${sg === 1 ? "l" : "r"}`, bb, b.fb, vein(b, sg, VEINS[DRAWN_VEIN]!, 0.0004), { material: "hosta-blade", role: "texture", tone: 0.5, texture: true }), member: bb, part: "member" });
    if (scale <= 100) {
      structure.push({ ...ellipsoidOf(`j-${pb}`, pb, b.fa, b.pet.at(0.02), [b.pet.tangent(0), b.pet.normal(0), b.side], [floor(0.011), floor(0.0085), floor(0.013)], { material: "hosta-petiole", role: "body", tone: 0.5 }), member: pb, part: "joint" });
      structure.push({ ...ellipsoidOf(`j-${bb}`, bb, b.fb, b.j, [b.mid.tangent(0), b.mid.normal(0), b.side], [floor(0.009), floor(0.0065), floor(0.009)], { material: "hosta-blade", role: "body", tone: 0.5 }), member: bb, part: "joint" });
    }
  }
  for (const o of OUTLETS) {
    const b = ALL[o.k]!, emitter = (asset.thermal.emitters as Json[]).find((e) => e.anchors.includes(o.id))!;
    const s = OUTLET_S, T = b.mid.tangent(s);
    const fields = { material: "hosta-blade", role: "texture", tone: 0.8 };
    const size: V3 = [0.05, 0.001, 0.028];
    if (scale <= 100) {
      const x = wingAxis(b, s, 1), up = mul(cross(T, x), -1);
      thermal.push({ ...ellipsoidOf(`th-${o.id}`, bladeId(o.k), b.fb, wingPoint(b, s, 1, OUTLET_D * outerOf(b, s, scale), table(LAM[scale as 50 | 100], s) / 2 * 0.86), [T, up, x], size, fields), region: emitter.region, tags: ["emitter", emitter.id] });
    } else {
      const q = lensAt(b, s, scale as 200 | 500);
      thermal.push({ ...ellipsoidOf(`th-${o.id}`, bladeId(o.k), b.fb, add(b.mid.at(s), mul(b.mid.normal(s), q.mid + q.half)), [T, b.mid.normal(s), b.side], size, fields), region: emitter.region, tags: ["emitter", emitter.id] });
    }
  }
  const plate = scale === 500 ? 0.03 : Math.max(0.022, f + 0.001);
  structure.push({ id: "m-root-plate", kind: "revolve", bone: "root-plate", material: "hosta-petiole", role: "body", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.046], [r5(plate), 0.046]], member: "root-plate", part: "member" });
  structure.push({ id: "s-base-support", kind: "revolve", bone: "root-plate", material: "hosta-petiole", role: "ground", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.056], [0.008, 0.056]], member: "root-plate", part: "support", anchor: "base-support" });
  const k = floor(0.016);
  structure.push({ id: "k-root-plate", kind: "ellipsoid", bone: "root-plate", material: "hosta-petiole", role: "body", tone: 0.5, radii: [0.03, r5(k), 0.03], transform: { translation: [0, r5(Math.max(0.02, k)), 0] }, member: "root-plate", part: "mass" });
  return { body, thermal, structure, drafts };
}

const parts = Object.fromEntries(SCALES.map((s) => [s, build(s)])) as Record<Scale, Parts>;
const volumes: Json = {};
for (const s of SCALES) volumes[String(s)] = { status: "authored", denominator: s, body: parts[s].body, thermal: parts[s].thermal, structure: parts[s].structure, decisions: DECISIONS[s](parts[s]) };

/** 📏️ A line through points (u, v) that bounds them from below (sign -1), above (+1) or fits them (0): least squares, shifted to the extreme residual. */
function line(pts: [number, number][], sign: 1 | -1 | 0): [number, number] {
  const n = pts.length, mu = pts.reduce((s, p) => s + p[0], 0) / n, mv = pts.reduce((s, p) => s + p[1], 0) / n;
  const sxx = pts.reduce((s, p) => s + (p[0] - mu) ** 2, 0), sxy = pts.reduce((s, p) => s + (p[0] - mu) * (p[1] - mv), 0);
  const k = sxx > 1e-12 ? sxy / sxx : 0;
  const c0 = mv - k * mu;
  const res = pts.map((p) => p[1] - (c0 + k * p[0]));
  return [c0 + (sign > 0 ? Math.max(...res) : sign < 0 ? Math.min(...res) : 0), k];
}

/** 📐️ The two parallel faces of a plate over an arched blade part: the slope of the chords through the first and last station extremes (u, v) of underside and top, each face set to the middle of its stations' offsets from that slope, so that the plate follows the arch with the least error on both sides and its faces never cross. Returns the offsets of underside and top at u = 0 and the slope. */
function faces(lows: [number, number][], highs: [number, number][]): [number, number, number] {
  const slope = (pts: [number, number][]) => { const [a, z] = [pts[0]!, pts[pts.length - 1]!]; return Math.abs(z[0] - a[0]) > 1e-9 ? (z[1] - a[1]) / (z[0] - a[0]) : 0; };
  const k = (slope(lows) + slope(highs)) / 2;
  const middle = (pts: [number, number][]) => { const r = pts.map((p) => p[1] - k * p[0]); return (Math.min(...r) + Math.max(...r)) / 2; };
  const lo = middle(lows);
  return [lo, Math.max(middle(highs), lo + 0.0016), k];
}

const TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** ✏️ The 2D shapes of one scale: every leaf as tilted plates whose outline is the true planform of the cupped blade (with lobes and sinus) and whose span is the blade from midrib underside to margin tops, midribs and veins as strokes on them, petioles as tapered strips (1:50) or strokes, the shoot necks as revolved profiles; drawn far to near in every view. */
function drawScale(scale: 50 | 100 | 200): Record<View, Json[]> {
  const shapes = parts[scale].drafts.map((d) => {
    if (d.kind === "revolved") {
      const last = d.profile[d.profile.length - 1]!;
      return { centre: d.centre, shape: { id: d.id, bone: d.bone, role: "body", filled: true, tone: d.tone, region: d.region, kind: "poly", points: [[0, 0], ...d.profile, [last[0], 0]], closed: true, solid: { kind: "revolved", axisBone: d.bone, axis: "u", axisX: 0, mirror: true } } };
    }
    const loc = (p: V3) => toLocal(d.F, p);
    if (d.kind === "plate") {
      if (d.flat) {
        const vs = [...d.lows, ...d.highs].flat().map((p) => loc(p)[1]);
        return { centre: d.centre, shape: { id: d.id, bone: d.bone, role: "body", filled: true, tone: d.tone, region: d.region, kind: "poly", points: d.ring.map((p) => { const q = loc(p); return [r5(q[0]), r5(q[2])]; }), closed: true, solid: { kind: "extruded", sweep: "v", z0: r5(Math.min(...vs)), z1: r5(Math.max(...vs)) } } };
      }
      const extreme = (groups: V3[][], sign: 1 | -1) => groups.map((ps) => ps.map(loc).reduce((m, q) => (sign * q[1] > sign * m[1] ? q : m))).map((q) => [q[0], q[1]] as [number, number]);
      const [lo, hi, k] = faces(extreme(d.lows, -1), extreme(d.highs, 1));
      return { centre: d.centre, shape: { id: d.id, bone: d.bone, role: "body", filled: true, tone: d.tone, region: d.region, kind: "poly", points: d.ring.map((p) => { const q = loc(p); return [r5(q[0]), r5(q[2])]; }), closed: true, solid: { kind: "extruded", sweep: "v", z0: r5(lo), z1: r5(hi), zTip: [r5(lo + k * d.len), r5(hi + k * d.len)] } } };
    }
    const [c0, k] = line(d.pts.map((p) => { const q = loc(p); return [q[0], q[1]] as [number, number]; }), 0);
    return { centre: d.centre, shape: { id: d.id, bone: d.bone, role: d.role, filled: false, tone: d.tone, ...(d.region ? { region: d.region } : {}), ...(d.pen ? { penMm: d.pen } : {}), kind: "poly", points: d.pts.map((p) => { const q = loc(p); return [r5(q[0]), r5(q[2])]; }), closed: false, solid: { kind: "extruded", sweep: "v", z0: r5(c0 - 0.0006), z1: r5(c0 + 0.0006), zTip: [r5(c0 + k * d.len - 0.0006), r5(c0 + k * d.len + 0.0006)] } } };
  });
  const out = { elevation: [] as Json[], front: [] as Json[], plan: [] as Json[] };
  const near = (view: View, p: V3) => p[0] * TOWARD[view][0] + p[1] * TOWARD[view][1] + p[2] * TOWARD[view][2];
  for (const view of VIEWS) {
    const order = shapes.map((x, i) => ({ x, i })).sort((p, q) => near(view, p.x.centre) - near(view, q.x.centre) || p.i - q.i);
    out[view] = order.map(({ x }, i) => {
      const { solid, ...rest } = x.shape as Json;
      const { id, bone, role, ...tail } = rest;
      return { id, bone, role, z: r4(1 + i / 1000), ...tail, solid };
    });
  }
  return out;
}

/** 🔣️ The plan symbol: generalised outlines of the mound in the three views (upper envelope over the bed, lobed plan outline) and the ring of the shoots. */
function symbolGeometry(): Record<View, Json[]> {
  const pts: V3[] = ALL.flatMap((b) => STATIONS.flatMap((s) => [1, -1].map((sg) => wingPoint(b, s, sg as 1 | -1, table(OUTER, s), 0.003))));
  const envelope = (axis: 0 | 2): number[][] => {
    const xs = pts.map((p) => p[axis]), [lo, hi] = [Math.min(...xs), Math.max(...xs)], n = 22;
    const top = Array.from({ length: n }, () => 0);
    for (const p of pts) { const i = Math.min(n - 1, Math.floor(((p[axis] - lo) / (hi - lo)) * n)); top[i] = Math.max(top[i]!, p[1]); }
    const smooth = top.map((_, i) => Math.max(top[Math.max(0, i - 1)]!, top[i]!, top[Math.min(n - 1, i + 1)]!));
    return [[r4(lo), 0], ...smooth.map((y, i) => [r4(lo + ((i + 0.5) / n) * (hi - lo)), r4(y)]), [r4(hi), 0]];
  };
  const n = 36, reach = Array.from({ length: n }, () => 0);
  for (const p of pts) { const i = ((Math.round((Math.atan2(p[2], p[0]) / (2 * Math.PI)) * n) % n) + n) % n; reach[i] = Math.max(reach[i]!, Math.hypot(p[0], p[2])); }
  const plan = reach.map((r, i) => [r4(r * Math.cos((2 * Math.PI * i) / n)), r4(r * Math.sin((2 * Math.PI * i) / n))]);
  const heart = Array.from({ length: 16 }, (_, i) => [r4(0.088 * Math.cos((2 * Math.PI * i) / 16)), r4(0.088 * Math.sin((2 * Math.PI * i) / 16))]);
  const crown = (points: number[][]) => ({ id: "crown", bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points, closed: true });
  return { elevation: [crown(envelope(0))], front: [crown(envelope(2))], plan: [crown(plan), { id: "heart", bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.4, kind: "poly", points: heart, closed: true }] };
}

const cand = JSON.parse(JSON.stringify(asset));
for (const s of DRAWN) {
  const rep = cand.representations[String(s)];
  rep.geometry = drawScale(s);
  rep.decisions = REP_DECISIONS[s];
}
cand.symbols[0].geometry = symbolGeometry();
cand.dimensions = { ...cand.dimensions, ...DIMENSIONS };
cand.representations["100"].validRange = { ...cand.representations["100"].validRange, maxDenominator: 140 };
cand.representations["200"].validRange = { ...cand.representations["200"].validRange, minDenominator: 140 };
cand.representations["500"].reason = { en: "A hosta of 1.0 m by 0.62 m is 2.0 mm by 1.2 mm at 1:500: a dot. Its blades (0.3 m, 0.6 mm) and their overlaps cannot be told apart, so any elevation would be a blob without information beyond the plan symbol.", de: "Eine Funkie von 1,0 m mal 0,62 m misst bei 1:500 2,0 mm mal 1,2 mm: ein Punkt. Ihre Spreiten (0,3 m, 0,6 mm) und deren Überlagerungen sind nicht unterscheidbar; jede Ansicht wäre ein Fleck ohne mehr Information als das Grundrisssymbol." };
TEXTS(cand);
cand.volumes = volumes;
mkdirSync(join(GENERATED, "r5-hosta-clump"), { recursive: true });
writeFileSync(join(GENERATED, "r5-hosta-clump", "🔣️.json"), JSON.stringify({ ...doc, asset: cand }, null, 2) + "\n");
say(`[DEBUG] candidate ${join(GENERATED, "r5-hosta-clump", "🔣️.json")}`);
if (process.argv.includes("--source")) {
  writeFileSync(join(TICKET, "volumes", "vegetation", "hosta-clump.json"), JSON.stringify({ volumes }, null, 2) + "\n");
  say("[DEBUG] source volumes/vegetation/hosta-clump.json written");
}
for (const s of SCALES) say(`[DEBUG] 1:${s} body ${parts[s].body.length} thermal ${parts[s].thermal.length} structure ${parts[s].structure.length} shapes ${s === 500 ? 0 : cand.representations[String(s)].geometry.elevation.length}`);
