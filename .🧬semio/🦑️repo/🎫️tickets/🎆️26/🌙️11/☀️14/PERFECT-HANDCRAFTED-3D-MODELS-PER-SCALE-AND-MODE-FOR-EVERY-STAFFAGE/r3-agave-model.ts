/** 🌵️ Round 3, vegetation/agave: builds the candidate asset (rig root fix, 2D redrawn per ruling E1 at every scale and view, texts) and the volumes source (body, thermal, structure at 1:50, 1:100, 1:200, 1:500) from the hand-stated rosette of `r3-agave-plant.ts`.
 * Usage: bun r3-agave-model.ts [--source]   (candidate: 🗑️generated/r3-agave/🔣️.json; --source also writes volumes/vegetation/agave.json)
 * @see ./r3-vegetation-agave.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, TICKET, entryOf, toLocal, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, capsuleOf, cross, dot, ellipsoidOf, lin, loftOf, mul, r4, r5, sub, table, unit, type Section } from "./r3-agave-kit.ts";
import { CAUDEX, KNEE, LEAVES, SPEAR, SPINE, blade, gammaDeg, halfWidth, leafId, regionOf, thickness, type Blade, type Leaf } from "./r3-agave-plant.ts";
import { fixRig, rigCheck, type Chords } from "./r3-agave-rig.ts";
import { DECISIONS, REP_DECISIONS, TEXTS } from "./r3-agave-texts.ts";

type Scale = 50 | 100 | 200 | 500;
type View = "elevation" | "front" | "plan";
const SCALES: Scale[] = [50, 100, 200, 500];
const VIEWS: View[] = ["elevation", "front", "plan"];
const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const say = (line: string) => process.stdout.write(`${line}\n`);

const { entry } = entryOf("vegetation/agave");
const doc = JSON.parse(readFileSync(entry.path, "utf8"));
const asset = doc.asset;
const { chords, report } = fixRig(asset);
for (const r of report) say(`[DEBUG] ${r.slice(0, 400)}`);
const frameErr = Math.max(...rigCheck(asset, asset.presentation.defaultPose).map((l) => Number(l.split("error ")[1])));
say(`[DEBUG] default pose frames against the stated chords: max error ${frameErr.toExponential(2)}`);

const CAUDEX_F: Frame = { o: [0, 0, 0], u: [0, 1, 0], v: [-1, 0, 0], w: [0, 0, 1] };
const ROOT_F: Frame = { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] };
const SPEAR_F: Frame = { o: [0, SPEAR.base, 0], u: [0, 1, 0], v: [-1, 0, 0], w: [0, 0, 1] };
const BLADES = chords.map((c) => ({ c, b: blade(c.leaf) }));
const toneOf = (k: number) => r4(0.4 + 0.14 * (k / 27));
const OV = 0.03;

/** 🧩️ The arc range of the proximal (a) and distal (b) part of a leaf, overlapping at the knee so that every pose stays closed; through the overlap the proximal part draws in (`fit` in `build`), so its end lies inside the distal part and no two skins coincide. */
const partRange = (part: "a" | "b", scale: Scale, l: Leaf): [number, number] => (part === "a" ? [0, KNEE + OV] : [KNEE - OV, 1 + (scale === 50 ? 0 : SPINE.length / l.L)]);
const frameOf = (c: Chords, part: "a" | "b"): Frame => (part === "a" ? c.fa : c.fb);
const boneOf = (l: Leaf, part: "a" | "b") => `${leafId(l.k)}-${part}`;

/** 📐️ Stations of arc share in [s0, s1] at most `step` metres apart along the blade, always including both ends. */
function stations(l: Leaf, s0: number, s1: number, step: number): number[] {
  const n = Math.max(1, Math.ceil(((s1 - s0) * l.L) / step));
  return Array.from({ length: n + 1 }, (_, i) => s0 + ((s1 - s0) * i) / n);
}

/** 🦷️ Half width with the crenate margin of 1:50: prominences every 4 cm carrying the marginal teeth (5-8 mm, merged into the prominence at this scale). */
const crenate = (l: Leaf, s: number, peak: boolean) => halfWidth(l, s) + (peak ? 0.0055 * table([[0.12, 0], [0.2, 1], [0.88, 1], [0.95, 0]], s) : 0);

/** 🍃️ Geometry of the blade at one station: centre, tangent, normal, lateral, half width, thickness, channel half angle (tip beyond s = 1 runs out into the spine). */
function station(bl: Blade, s: number, hw?: number) {
  const l = bl.leaf;
  const sc = Math.min(1, s);
  const run = s > 1 ? (s - 1) / (SPINE.length / l.L) : 0;
  return { C: bl.at(s), T: bl.tangent(s), N: bl.normal(s), S: bl.side, hw: s > 1 ? Math.max(0.0025, halfWidth(l, 1) * (1 - run)) : (hw ?? halfWidth(l, sc)), t: s > 1 ? Math.max(0.004, thickness(l, 1) * (1 - 0.7 * run)) : thickness(l, sc), g: gammaDeg(sc) * D2R };
}

/** 🥬️ The V section of 1:50 and 1:100: a rounded keel core and two wings rising to the margins by the channel angle. */
function vSections(bl: Blade, ss: number[], wingSS: number[], crenated: boolean, fit: (s: number) => number) {
  const core: Section[] = ss.map((s) => {
    const q = station(bl, s);
    return { p: add(q.C, mul(q.N, -0.2 * q.t)), x: q.S, y: cross(q.T, q.S), a: Math.max(0.3 * q.hw, 0.004) * fit(s), b: 0.45 * q.t * fit(s) };
  });
  const wing = (sign: 1 | -1): Section[] => wingSS.map((s) => {
    const hw = crenated ? crenate(bl.leaf, s, isPeak(bl.leaf, s)) : undefined;
    const q = station(bl, s, hw);
    const xw = lin(q.S, sign * Math.cos(q.g), q.N, Math.sin(q.g));
    const span = q.hw / Math.cos(q.g);
    return { p: add(q.C, mul(xw, 0.47 * span)), x: xw, y: cross(q.T, xw), a: 0.53 * span * fit(s), b: 0.36 * q.t * fit(s), n: 2.5 };
  });
  return { core, wingL: wing(1), wingR: wing(-1) };
}

/** 🫘️ Span of the envelope lens of 1:200 and 1:500 at one station: keel bottom to margin tops, kept at the scale's smallest feature along the blade and run out to the point over the last eighth of the leaf. */
function lensSpan(bl: Blade, s: number, scale: Scale) {
  const q = station(bl, s);
  const run = table([[0, 1], [0.86, 1], [0.94, 0.62], [1, 0.2], [1.05, 0.08]], s);
  const bottom = -0.65 * q.t, top = q.hw * Math.tan(q.g) + 0.2 * q.t;
  const half = Math.max((top - bottom) / 2, (FEATURE[scale] / 2 + 0.0008) * run);
  return { q, mid: (top + bottom) / 2, half, hw: Math.max(q.hw, 0.003) };
}

/** 🫘️ Envelope lens of 1:200 and 1:500: keel to margin tops in one section. */
function lensSections(bl: Blade, ss: number[], scale: Scale, fit: (s: number) => number): Section[] {
  return ss.map((s) => {
    const { q, mid, half, hw } = lensSpan(bl, s, scale);
    return { p: add(q.C, mul(q.N, mid)), x: q.S, y: cross(q.T, q.S), a: hw * fit(s), b: half * fit(s), n: 2.2 };
  });
}

/** 🪜️ The nested plates that draw one blade section, widest first: share of the half width and the span along the blade normal. The projection of a prismatic leaf is the projection of the hull of its section, so the plates step along that hull: the channelled V of 1:50 (three steps) and 1:100 (two), the lens of 1:200 (two) and 1:500 (one). */
function levels(bl: Blade, s: number, scale: Scale): { k: number; lo: number; hi: number }[] {
  if (scale >= 200) {
    const { mid, half } = lensSpan(bl, s, scale);
    return scale === 200 ? [{ k: 1, lo: mid - 0.49 * half, hi: mid + 0.49 * half }, { k: 0.62, lo: mid - half, hi: mid + half }] : [{ k: 0.94, lo: mid - 0.9 * half, hi: mid + 0.9 * half }];
  }
  const q = station(bl, s);
  const rise = q.hw * Math.tan(q.g), wt = 0.36 * q.t, keel = -0.65 * q.t;
  if (scale === 50) return [{ k: 1, lo: 0.65 * rise - wt, hi: rise + 0.5 * wt }, { k: 0.65, lo: 0.3 * rise - wt, hi: 0.65 * rise - wt + 0.002 }, { k: 0.3, lo: keel, hi: 0.3 * rise - wt + 0.002 }];
  return [{ k: 1, lo: 0.5 * rise - wt, hi: rise + 0.5 * wt }, { k: 0.45, lo: keel, hi: 0.5 * rise - wt + 0.002 }];
}

const LEVELS: Record<Scale, number> = { 50: 3, 100: 2, 200: 2, 500: 1 };

/** 🫥️ Inset of the face plate from the margin: it lies inside the blade in every view and covers the joints of the stepped plates. */
const INSET = 0.003;

const BODY = (k: number) => ({ material: "agave-leaf", role: "body", tone: toneOf(k) });

type Parts = { body: Json[]; thermal: Json[]; structure: Json[]; plates: Json[] };

const SEGMENTS: Record<Scale, number> = { 50: 3, 100: 3, 200: 2, 500: 2 };
const STEP: Record<Scale, { core: number; wing: number; member: number }> = { 50: { core: 0.05, wing: 0.02, member: 0.1 }, 100: { core: 0.08, wing: 0.06, member: 0.15 }, 200: { core: 0.11, wing: 0.11, member: 0.25 }, 500: { core: 0.2, wing: 0.2, member: 0.3 } };

/** 🏗️ All solids and plates of one scale. */
function build(scale: Scale): Parts {
  const f = FEATURE[scale];
  const body: Json[] = [], thermal: Json[] = [], structure: Json[] = [], plates: Json[] = [];
  const caudexProfile: [number, number][] = scale === 50 ? CAUDEX.profile : scale === 100 ? [[0, 0.132], [0.12, 0.128], [0.26, 0.105], [0.3, 0.07]] : scale === 200 ? [[0, 0.132], [0.2, 0.12], [0.3, 0.075]] : [[0, 0.13], [0.3, 0.08]];
  body.push({ id: "caudex", kind: "revolve", bone: "caudex", material: "agave-caudex", role: "body", tone: 0.3, axis: [1, 0, 0], origin: [0, 0, 0], profile: caudexProfile.map(([u, r]) => [r5(u), r5(r)]) });
  thermal.push({ id: "th-caudex", of: "caudex", region: "caudex" });
  const spearProfile: [number, number][] = scale === 50 ? SPEAR.profile : scale === 100 ? [[0, 0.072], [0.3, 0.068], [0.7, 0.049], [1.0, 0.02], [1.145, 0]] : scale === 200 ? [[0, 0.072], [0.5, 0.06], [0.95, 0.03], [1.145, 0]] : [[0, 0.075], [0.6, 0.058], [1.145, 0]];
  body.push({ id: "spear", kind: "revolve", bone: "spear", material: "agave-leaf", role: "body", tone: 0.52, axis: [1, 0, 0], origin: [0, 0, 0], profile: spearProfile.map(([u, r]) => [r5(u), r5(r)]) });
  thermal.push({ id: "th-spear", of: "spear", region: "leaves-erect" });
  plates.push({ kind: "revolved", id: "caudex", bone: "caudex", profile: caudexProfile, tone: 0.3, region: "caudex", gcentre: [0, 0.15, 0] as V3 });
  plates.push({ kind: "revolved", id: "spear", bone: "spear", profile: spearProfile, tone: 0.52, region: "leaves-erect", gcentre: [0, SPEAR.base + 0.5, 0] as V3 });
  for (const { c, b } of BLADES) {
    const l = c.leaf, region = regionOf(l.k);
    for (const part of ["a", "b"] as const) {
      const F = frameOf(c, part), bone = boneOf(l, part);
      const [s0, s1] = partRange(part, scale, l);
      const fit = (s: number) => (part === "a" ? 1 - (0.14 * Math.max(0, s - (KNEE - OV))) / (2 * OV) : 1);
      if (scale <= 100) {
        const ss = stations(l, s0, s1, STEP[scale].core);
        const wss = scale === 50 ? crenationStations(l, s0, s1) : stations(l, s0, s1, STEP[scale].wing);
        const v = vSections(b, ss, wss, scale === 50, fit);
        for (const [name, secs] of [["core", v.core], ["wing-l", v.wingL], ["wing-r", v.wingR]] as const) {
          body.push(loftOf(`${bone}-${name}`, bone, F, secs, BODY(l.k)));
          thermal.push({ id: `th-${bone}-${name}`, of: `${bone}-${name}`, region });
        }
      } else {
        body.push(loftOf(bone, bone, F, lensSections(b, stations(l, s0, s1, STEP[scale].core), scale, fit), { material: "agave-leaf", role: "body", tone: toneOf(l.k) }));
        thermal.push({ id: `th-${bone}`, of: bone, region });
      }
      const nseg = SEGMENTS[scale];
      for (let i = 0; i < nseg; i++) {
        const a0 = s0 + ((s1 - s0) * i) / nseg, a1 = s0 + ((s1 - s0) * (i + 1)) / nseg;
        plates.push({ kind: "plate", seg: i, gcentre: b.at((s0 + s1) / 2), id: `${bone}-p${i + 1}`, bone, length: part === "a" ? c.La : c.Lb, wss: scale === 50 ? crenationStations(l, s0, s1) : [], frame: F, blade: b, range: [Math.max(s0, a0 - (i > 0 ? 0.008 : 0)), Math.min(s1, a1 + (i < nseg - 1 ? 0.008 : 0))], tone: toneOf(l.k), region, centre: b.at((a0 + a1) / 2), groove: scale === 50 });
      }
      if (scale <= 100) plates.push({ kind: "face", id: `${bone}-face`, bone, length: part === "a" ? c.La : c.Lb, frame: F, blade: b, range: [s0, Math.min(s1, 1)], tone: toneOf(l.k), region, gcentre: b.at((s0 + s1) / 2) });
      const ms = stations(l, part === "a" ? 0.02 : KNEE, part === "a" ? KNEE : 0.97, STEP[scale].member).map((s) => {
        const q = station(b, s);
        const floor = (f / 2 + 0.0005) * table([[0, 1], [0.8, 1], [0.9, 0.6], [0.97, 0.22]], s);
        return { p: add(q.C, mul(q.N, -0.15 * q.t)), x: q.S, y: cross(q.T, q.S), a: Math.max(0.32 * q.hw, floor), b: Math.max(0.32 * q.t, floor) } as Section;
      });
      structure.push({ ...loftOf(`m-${bone}`, bone, F, ms, { material: "agave-leaf", role: "body", tone: 0.5 }), member: bone, part: "member" });
      if (scale <= 100) {
        const s = part === "a" ? 0.03 : KNEE;
        const q = station(b, s);
        structure.push({ ...ellipsoidOf(`j-${bone}`, bone, F, add(q.C, mul(q.N, -0.15 * q.t)), [q.T, q.N, q.S], [Math.max(0.025, f / 2 + 0.001), Math.max(0.3 * q.t, f / 2 + 0.0008), Math.max(0.28 * q.hw, f / 2 + 0.001)], { material: "agave-leaf", role: "body", tone: 0.5 }), member: bone, part: "joint" });
      }
    }
    if (scale === 50) {
      const tip = b.tip, T = b.tangent(1);
      body.push({ ...capsuleOf(`${leafId(l.k)}-spine`, boneOf(l, "b"), c.fb, add(tip, mul(T, -0.012)), add(tip, mul(T, SPINE.length)), SPINE.radius, 0, { material: "agave-leaf", role: "detail", tone: 0.15 }) });
      thermal.push({ id: `th-${leafId(l.k)}-spine`, of: `${leafId(l.k)}-spine`, region });
    }
  }
  if (scale === 50) {
    body.push({ id: "spear-spine", kind: "capsule", bone: "spear", material: "agave-leaf", role: "detail", tone: 0.15, p0: [1.105, 0, 0], p1: [1.15, 0, 0], r0: 0.0045, r1: 0 });
    thermal.push({ id: "th-spear-spine", of: "spear-spine", region: "leaves-erect" });
  }
  const pv = Math.max(0.004, f / 2 + 0.0008);
  for (const a of asset.anchors.filter((x: Json) => x.kind === "emitter")) {
    const emitter = asset.thermal.emitters.find((e: Json) => e.anchors.includes(a.id));
    if (a.bone === "spear") {
      const r = table(scale === 50 ? SPEAR.profile : spearProfile, a.x);
      thermal.push({ id: `th-${a.id}`, kind: "ellipsoid", bone: "spear", material: "agave-leaf", role: "body", tone: 0.8, radii: [r5(Math.max(0.05, 2.2 * pv)), r5(pv), r5(Math.max(0.02, pv))], transform: { translation: [r5(a.x), r5(r + (scale >= 200 ? -0.6 * pv : pv - 0.002)), 0] }, region: emitter.region, tags: ["emitter", emitter.id] });
      continue;
    }
    const k = Number(a.bone.slice(5, 7)) - 1;
    const { c, b } = BLADES[k]!;
    const s = KNEE + (1 - KNEE) * 0.35;
    const q = station(b, s);
    const coarse = scale >= 200;
    const lift = coarse ? lensSpan(b, s, scale).mid + lensSpan(b, s, scale).half - 1.6 * pv : 0.25 * q.t;
    thermal.push({ ...ellipsoidOf(`th-${a.id}`, a.bone, c.fb, add(q.C, mul(q.N, lift + pv - 0.001)), [q.T, q.N, q.S], [Math.max(0.06, 2.2 * pv), pv, Math.max(0.022, coarse ? 0.6 * q.hw : pv)], { material: "agave-leaf", role: "body", tone: 0.8 }), region: emitter.region, tags: ["emitter", emitter.id] });
  }
  const memberRevolve = (id: string, bone: string, profile: [number, number][], part: string, extra: Json = {}) => ({ id, kind: "revolve", bone, material: bone === "spear" ? "agave-leaf" : "agave-caudex", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: profile.map(([u, r]) => [r5(u), r5(r)]), member: bone, part, ...extra });
  structure.push(memberRevolve("m-caudex", "caudex", [[0, Math.max(0.085, f / 2 + 0.001)], [0.3, Math.max(0.06, f / 2 + 0.001)]], "member"));
  structure.push(memberRevolve("m-spear", "spear", [[0, Math.max(0.035, f / 2 + 0.001)], [0.7, Math.max(0.024, f / 2 + 0.001)], [1.05, Math.max(0.008, 0.3 * (f / 2 + 0.001))]], "member"));
  const plateT = Math.max(0.04, f + 0.001);
  structure.push({ id: "m-root-plate", kind: "revolve", bone: "root-plate", material: "agave-caudex", role: "body", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.12], [r5(plateT), 0.12]], member: "root-plate", part: "member" });
  const padT = Math.max(0.02, f + 0.001);
  structure.push({ id: "s-base-support", kind: "revolve", bone: "root-plate", material: "agave-caudex", role: "ground", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.125], [r5(padT), 0.125]], member: "root-plate", part: "support", anchor: "base-support" });
  structure.push({ id: "k-caudex", kind: "ellipsoid", bone: "caudex", material: "agave-caudex", role: "body", tone: 0.5, radii: [0.05, 0.05, 0.05].map((r) => r5(Math.max(r, f / 2 + 0.001))), transform: { translation: [0.15, 0, 0] }, member: "caudex", part: "mass" });
  const rootR = Math.max(0.02, f / 2 + 0.001);
  structure.push({ id: "k-root-plate", kind: "ellipsoid", bone: "root-plate", material: "agave-caudex", role: "body", tone: 0.5, radii: [r5(Math.max(0.07, rootR)), r5(rootR), r5(Math.max(0.07, rootR))], transform: { translation: [0, r5(Math.max(0.03, rootR)), 0] }, member: "root-plate", part: "mass" });
  if (scale <= 100) {
    structure.push({ id: "j-caudex", kind: "ellipsoid", bone: "caudex", material: "agave-caudex", role: "body", tone: 0.5, radii: [r5(Math.max(0.03, f / 2 + 0.001)), 0.06, 0.06], transform: { translation: [0.04, 0, 0] }, member: "caudex", part: "joint" });
    structure.push({ id: "j-spear", kind: "ellipsoid", bone: "spear", material: "agave-leaf", role: "body", tone: 0.5, radii: [r5(Math.max(0.03, f / 2 + 0.001)), 0.04, 0.04], transform: { translation: [0.02, 0, 0] }, member: "spear", part: "joint" });
  }
  return { body, thermal, structure, plates };
}

/** 🦷️ Wing stations of 1:50: one global grid of 2 cm along every blade (valley, prominence, valley ...), so that the proximal and distal parts carry the same margin where they overlap at the knee. */
function crenationStations(l: Leaf, s0: number, s1: number): number[] {
  const d = 0.02 / l.L;
  const out = [s0];
  for (let i = Math.floor(s0 / d) + 1; i * d < s1 - 1e-9; i++) if (i * d > s0 + 1e-9) out.push(i * d);
  out.push(s1);
  return out;
}

/** 🦷️ Whether a station of the 2 cm grid is a prominence. */
const isPeak = (l: Leaf, s: number): boolean => {
  const i = (s * l.L) / 0.02;
  return Math.abs(i - Math.round(i)) < 1e-6 && Math.round(i) % 2 === 1;
};

const parts = Object.fromEntries(SCALES.map((s) => [s, build(s)])) as Record<Scale, Parts>;

const volumes: Json = {};
for (const s of SCALES) volumes[String(s)] = { status: "authored", denominator: s, body: parts[s].body, thermal: parts[s].thermal, structure: parts[s].structure, decisions: DECISIONS[s](parts[s]) };

const VIEW_TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
const nearness = (view: View, p: V3) => dot(p, VIEW_TOWARD[view]);

/** 📏️ The least-squares line v = c + k u through points (u, v). */
function fitLine(pts: [number, number][]): [number, number] {
  const n = pts.length, mu = pts.reduce((s, p) => s + p[0], 0) / n, mv = pts.reduce((s, p) => s + p[1], 0) / n;
  const sxx = pts.reduce((s, p) => s + (p[0] - mu) ** 2, 0), sxy = pts.reduce((s, p) => s + (p[0] - mu) * (p[1] - mv), 0);
  const k = sxx > 1e-12 ? sxy / sxx : 0;
  return [mv - k * mu, k];
}

/** ✏️ The 2D shapes of one scale: every leaf part as tilted plates (outline = the true planform of the blade with its crenate margin at 1:50, span = keel to margin tops), the groove of the channel as a stroke at 1:50 and 1:100, stem and spear as revolved profiles. */
function drawScale(scale: Scale): Record<View, Json[]> {
  const P = parts[scale];
  const out = { elevation: [] as Json[], front: [] as Json[], plan: [] as Json[] };
  const shapes: { shape: Json; group: string; gcentre: V3; rank: number }[] = [];
  for (const pl of P.plates) {
    if (pl.kind === "revolved") {
      const prof = pl.profile as [number, number][];
      const points = [[0, 0], ...prof.map(([u, r]) => [r5(u), r5(r)]), ...(prof[prof.length - 1]![1] > 0 ? [[r5(prof[prof.length - 1]![0]), 0]] : [])];
      shapes.push({ group: pl.bone, gcentre: pl.gcentre, rank: 0, shape: { id: pl.id, bone: pl.bone, role: "body", filled: true, tone: pl.tone, region: pl.region, kind: "poly", points, closed: true, solid: { kind: "revolved", axisBone: pl.bone, axis: "u", axisX: 0, mirror: true } } });
      continue;
    }
    const F: Frame = pl.frame, b: Blade = pl.blade;
    const loc = (p: V3) => toLocal(F, p);
    const Lb: number = pl.length;
    if (pl.kind === "face") {
      const qs = stations(b.leaf, pl.range[0], Math.min(pl.range[1], 0.9), scale === 50 ? 0.04 : 0.07).map((s) => station(b, s));
      const win = qs.map((q) => { const rise = q.hw * Math.tan(q.g), wt = 0.36 * q.t; return { q, top: rise + 0.5 * wt, shoulder: rise - wt, keel: -0.65 * q.t, m: loc(add(q.C, mul(q.N, rise - 0.25 * wt))) }; });
      const [c, k] = fitLine(win.map((x) => [x.m[0], x.m[1]]));
      const over = Math.max(0, ...win.map((x) => c + k * x.m[0] - (x.m[1] + 0.75 * 0.36 * x.q.t - 0.002)));
      const c0 = c - over;
      const level = win.map((x) => x.top - 0.75 * 0.36 * x.q.t + (c0 + k * x.m[0] - x.m[1]));
      const half = win.map((x, i) => {
        const v = level[i]! - 0.0015;
        const room = v >= x.shoulder ? 1 : Math.max(0, 0.3 + (0.7 * (v - x.keel)) / (x.shoulder - x.keel));
        return Math.max(x.q.hw * room - INSET, 0.0005);
      });
      const edge = (i: number, sign: number) => loc(add(add(win[i]!.q.C, mul(win[i]!.q.S, sign * half[i]!)), mul(win[i]!.q.N, level[i]!)));
      const ring = [...win.map((_, i) => edge(i, 1)), ...win.map((_, i) => edge(i, -1)).reverse()].map((q) => [r5(q[0]), r5(q[2])]);
      const solid = { kind: "extruded", sweep: "v", z0: r5(c0 - 0.0015), z1: r5(c0 + 0.0015), zTip: [r5(c0 + k * Lb - 0.0015), r5(c0 + k * Lb + 0.0015)] };
      shapes.push({ group: pl.bone, gcentre: pl.gcentre, rank: 1000, shape: { id: pl.id, bone: pl.bone, role: "body", filled: true, tone: pl.tone, region: pl.region, kind: "poly", points: ring, closed: true, solid } });
      continue;
    }
    const step = scale === 50 ? 0.02 : scale === 100 ? 0.05 : scale === 200 ? 0.08 : 0.14;
    const ss: [number, boolean][] = scale === 50 ? [[pl.range[0], false], ...(pl.wss as number[]).map((s) => [s, isPeak(b.leaf, s)] as [number, boolean]).filter(([s]) => s > pl.range[0] + 1e-6 && s < pl.range[1] - 1e-6), [pl.range[1], false]] : stations(b.leaf, pl.range[0], pl.range[1], step).map((s) => [s, false] as [number, boolean]);
    const endsAtTip = pl.range[1] >= 1 - 1e-9;
    for (let j = LEVELS[scale] - 1; j >= 0; j--) {
      const rows = ss.map(([s, peak]) => {
        const lv = levels(b, scale >= 200 ? s : Math.min(s, 1), scale)[j]!;
        const q = station(b, s, scale === 50 && s <= 1 && j === 0 ? crenate(b.leaf, s, peak) : undefined);
        const hw = lv.k * (scale >= 200 ? Math.max(q.hw, 0.003) : q.hw), mid = (lv.lo + lv.hi) / 2;
        const at = (w: number, v: number) => loc(add(add(q.C, mul(q.S, w)), mul(q.N, v)));
        return { left: at(hw, mid), right: at(-hw, mid), low: at(0, lv.lo), high: at(0, lv.hi) };
      });
      const tipPt = scale === 50 && endsAtTip && j === 0 ? [loc(add(b.tip, mul(b.tangent(1), SPINE.length)))] : [];
      const ring = [...rows.map((m) => [r5(m.left[0]), r5(m.left[2])]), ...tipPt.map((p) => [r5(p[0]), r5(p[2])]), ...rows.map((m) => [r5(m.right[0]), r5(m.right[2])]).reverse()];
      const [lo, ko] = fitLine(rows.map((m) => [m.low[0], m.low[1]])), [hi, kh] = fitLine(rows.map((m) => [m.high[0], m.high[1]]));
      const thin = 0.0015;
      const [l0, l1] = [lo, lo + ko * Lb], [h0, h1] = [Math.max(hi, lo + thin), Math.max(hi + kh * Lb, lo + ko * Lb + thin)];
      const solid = { kind: "extruded", sweep: "v", z0: r5(l0), z1: r5(h0), zTip: [r5(l1), r5(h1)] };
      shapes.push({ group: pl.bone, gcentre: pl.gcentre, rank: pl.seg * 10 + LEVELS[scale] - 1 - j, shape: { id: j === 0 ? pl.id : `${pl.id}-k${j}`, bone: pl.bone, role: "body", filled: true, tone: pl.tone, region: pl.region, kind: "poly", points: ring, closed: true, solid } });
    }
    if (pl.groove) {
      const gs = ss.filter(([s], i) => s <= 0.93 && (i % 3 === 0 || i === ss.length - 1));
      const gp = gs.map(([s]) => { const q = station(b, s); return loc(add(q.C, mul(q.N, 0.25 * q.t))); });
      if (gp.length >= 2) {
        const [g0, kg] = fitLine(gp.map((q) => [q[0], q[1]]));
        const gsolid = { kind: "extruded", sweep: "v", z0: r5(g0 - 0.0006), z1: r5(g0 + 0.0006), zTip: [r5(g0 + kg * Lb - 0.0006), r5(g0 + kg * Lb + 0.0006)] };
        shapes.push({ group: pl.bone, gcentre: pl.gcentre, rank: 2000 + pl.seg, shape: { id: pl.id.replace(/-p(\d)$/, "-g$1"), bone: pl.bone, role: "detail", filled: false, tone: 0.25, kind: "poly", points: gp.map((q) => [r5(q[0]), r5(q[2])]), closed: false, solid: gsolid } });
      }
    }
  }
  for (const view of VIEWS) {
    const order = [...shapes].sort((x, y) => (x.group === y.group ? x.rank - y.rank : nearness(view, x.gcentre) - nearness(view, y.gcentre) || (x.group < y.group ? -1 : 1)));
    out[view] = order.map((x, i) => {
      const { shape } = x;
      const z = shape.id === "caudex" ? 1 : r4(2 + i / 1000);
      return { id: shape.id, bone: shape.bone, role: shape.role, z, filled: shape.filled, tone: shape.tone, ...(shape.region ? { region: shape.region } : {}), kind: shape.kind, points: shape.points, closed: shape.closed, solid: shape.solid };
    });
  }
  return out;
}

const cand = JSON.parse(JSON.stringify(asset));
for (const s of SCALES) {
  const rep = cand.representations[String(s)];
  rep.geometry = drawScale(s);
  rep.decisions = REP_DECISIONS[s];
}
TEXTS(cand);
cand.volumes = volumes;
const candDoc = { ...doc, asset: cand };
mkdirSync(join(GENERATED, "r3-agave"), { recursive: true });
writeFileSync(join(GENERATED, "r3-agave", "🔣️.json"), JSON.stringify(candDoc, null, 2) + "\n");
say(`[DEBUG] candidate ${join(GENERATED, "r3-agave", "🔣️.json")}`);
if (process.argv.includes("--source")) {
  writeFileSync(join(TICKET, "volumes", "vegetation", "agave.json"), JSON.stringify({ volumes }, null, 2) + "\n");
  say("[DEBUG] source volumes/vegetation/agave.json written");
}
for (const s of SCALES) say(`[DEBUG] 1:${s} body ${parts[s].body.length} thermal ${parts[s].thermal.length} structure ${parts[s].structure.length} shapes ${cand.representations[String(s)].geometry.elevation.length}`);
