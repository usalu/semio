/** 🎸️ Authoritative chisel of the used classical guitar: states every hand-chosen number of the guitar, its folding A-frame stand, its worn gig bag and the guitarist's footstool, and writes `🔣️.json` next to it.
 *
 * Every solid is authored once per scale (bone-local, or in world rest coordinates and carried into its bone); the three 2D views of each scale are the kernel tessellation of those very solids projected into the paper frames of the product (elevation bone-local, front lateral by height, plan ahead by lateral), unioned per drawn part and simplified within a twentieth of a millimetre of paper, plus hand-drawn detail strokes. Masses are section by density of the 1:50 solids plus the listed hidden parts; thermal areas are the measured outer areas of the 1:50 thermal lists.
 *
 * Run: `$BUN author-guitar.ts` in this folder.
 * @see ../../brief.md
 * @see ./author-lib.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { about, bounds, ccw, circle, frames, inset, meshOf, meshStats, nearness, outline, p2, p3, qmul, qrot, r5, roundRect, shapeGeometry, symmetricOutline, toLocal, type Frame, type Json, type Mesh, type Q, type V2, type V3, type View } from "./author-lib.ts";

const HERE = import.meta.dir;
const t = (en: string, de: string) => ({ en, de });
const SCALES = [50, 100, 200, 500] as const;
type Scale = (typeof SCALES)[number];
const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };

//#region numbers
const LEAN = 15;
const G_ANGLE = 90 + LEAN;
const G_O: V3 = [0, 0.07, 0];
const DEPTH = 0.095;
const TOP_T = 0.0025;
const BACK_T = 0.0025;
const RIB_T = 0.002;
const VT = -DEPTH / 2;
const VB = DEPTH / 2;
const SCALE_LEN = 0.65;
const BODY_LEN = 0.485;
const U_NUT = BODY_LEN + SCALE_LEN / 2;
const U_SADDLE = U_NUT - SCALE_LEN;
const HOLE_U = 0.33;
const HOLE_R = 0.0425;
const FB_END = 0.372;
const FB_T = 0.006;
const HEAD_DEG = 13;
const HEAD_LEN = 0.18;
const HEAD_T = 0.019;
const BODY_TABLE: V2[] = [[0, 0], [0.003, 0.05], [0.012, 0.1], [0.03, 0.14], [0.06, 0.167], [0.1, 0.18], [0.135, 0.1825], [0.17, 0.178], [0.205, 0.163], [0.24, 0.14], [0.27, 0.122], [0.292, 0.1175], [0.318, 0.121], [0.35, 0.131], [0.385, 0.139], [0.415, 0.137], [0.443, 0.126], [0.463, 0.106], [0.476, 0.078], [0.483, 0.045], [0.485, 0]];
const BAG_TABLE: V2[] = [[0, 0], [0.003, 0.06], [0.012, 0.115], [0.028, 0.158], [0.055, 0.187], [0.095, 0.2015], [0.15, 0.2045], [0.195, 0.196], [0.235, 0.177], [0.27, 0.153], [0.3, 0.14], [0.33, 0.143], [0.37, 0.155], [0.41, 0.161], [0.445, 0.155], [0.475, 0.135], [0.5, 0.1], [0.52, 0.072], [0.545, 0.064], [0.6, 0.062], [0.95, 0.062], [1.015, 0.06], [1.04, 0.045], [1.05, 0]];
const BAG_LEN = 1.05;
const BAG_PANEL = 0.012;
const STRAP_T = 0.008;
const BAG_V = { strap: 0, back: -STRAP_T, gusset0: -STRAP_T - BAG_PANEL, gusset1: -0.136, lid: -0.148, pocket: -0.178 };
const G_IN_BAG: V3 = [0.016, -0.0685, 0];
const TUBE_R = 0.008;
const TUBE_WALL = 0.0008;
const TUBE_FACTOR = 1 - ((TUBE_R - TUBE_WALL) / TUBE_R) ** 2;
const LEG_FACTOR = 1 - (0.004 / 0.005) ** 2;
const FRONT_FOOT: V2 = [-0.09, 0.012];
const HINGE: V2 = [-0.178, 0.49];
const REAR_FOOT: V2 = [-0.42, 0.012];
const FRONT_Z = 0.14;
const REAR_Z = 0.122;
const TOP_FOAM_R = 0.0185;
const ARM_FOAM_R = 0.0145;
const ARM_Z = 0.1;
const FS = { heel: 0.12, toe: 0.39, z: 0, y: 0.1355, w: 0.095 };
const PLAYED = { abduction: 48, angle: 0, waist: [0.1, 0.63] as V2 };
//#endregion

//#region rotation chain
type M3 = number[][];
const mm = (a: M3, b: M3): M3 => a.map((row) => [0, 1, 2].map((j) => row[0]! * b[0]![j]! + row[1]! * b[1]![j]! + row[2]! * b[2]![j]!));
const rw = (d: number): M3 => {
  const [c, s] = [Math.cos((d * Math.PI) / 180), Math.sin((d * Math.PI) / 180)];
  return [[c, -s, 0], [s, c, 0], [0, 0, 1]];
};
const rv = (d: number): M3 => {
  const [c, s] = [Math.cos((d * Math.PI) / 180), Math.sin((d * Math.PI) / 180)];
  return [[c, 0, -s], [0, 1, 0], [s, 0, c]];
};
const col = (m: M3, j: number): V3 => [m[0]![j]!, m[1]![j]!, m[2]![j]!];
const apply = (m: M3, p: V3): V3 => [0, 1, 2].map((i) => m[i]![0]! * p[0] + m[i]![1]! * p[1] + m[i]![2]! * p[2]) as V3;
/** 🧭️ The world rotation of the guitar for an angle and a lateral spread offset from its rest. */
const guitarR = (dAngle: number, abd: number): M3 => mm(mm(rw(G_ANGLE), rw(dAngle)), rv(abd));
const bagR = (dAngle: number): M3 => rw(180 + dAngle);
//#endregion

//#region outlines
const PER: Record<Scale, number> = { 50: 5, 100: 3, 200: 2, 500: 1 };
const bodyRing = (s: Scale): V2[] => ccw(symmetricOutline(BODY_TABLE, PER[s]));
const bagRing = (s: Scale): V2[] => ccw(symmetricOutline(BAG_TABLE, PER[s]));
/** 📏️ The half width of the guitar outline at `u` (linear in the table). */
function halfWidth(table: readonly V2[], u: number): number {
  for (let i = 1; i < table.length; i++) if (u <= table[i]![0]) return table[i - 1]![1] + ((table[i]![1] - table[i - 1]![1]) * (u - table[i - 1]![0])) / (table[i]![0] - table[i - 1]![0]);
  return 0;
}
/** 🔎️ The `u` of the tail half of the smoothed outline at half width `w`. */
function tailU(ring: readonly V2[], w: number): number {
  let best = 1;
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!;
    const b = ring[(i + 1) % ring.length]!;
    if (a[1] > 0.2 || b[1] > 0.2) continue;
    if ((a[0] - w) * (b[0] - w) <= 0 && a[0] !== b[0]) best = Math.min(best, a[1] + ((b[1] - a[1]) * (w - a[0])) / (b[0] - a[0]));
  }
  return best;
}
const HEAD_RING: V2[] = ccw([[-0.026, 0], [0.026, 0], [0.031, 0.01], [0.0355, 0.022], [0.0375, 0.035], [0.0375, 0.16], [0.0355, 0.17], [0.03, 0.176], [0.018, 0.179], [0, 0.181], [-0.018, 0.179], [-0.03, 0.176], [-0.0355, 0.17], [-0.0375, 0.16], [-0.0375, 0.035], [-0.0355, 0.022], [-0.031, 0.01]]);
const SLOTS: V2[][] = [-1, 1].map((side) => ccw(roundRect(side * 0.0185, 0.1, 0.017, 0.12, 0.0085, 4)));
//#endregion

//#region geometry of the stand and the guitar placement
const U_DIR: V2 = [Math.cos((G_ANGLE * Math.PI) / 180), Math.sin((G_ANGLE * Math.PI) / 180)];
const V_DIR: V2 = [-U_DIR[1], U_DIR[0]];
const gw = (u: number, v: number): V2 => [G_O[0] + u * U_DIR[0] + v * V_DIR[0], G_O[1] + u * U_DIR[1] + v * V_DIR[1]];
const CRADLE_U = tailU(bodyRing(50), ARM_Z);
const CRADLE_CORNER = gw(CRADLE_U, VB);
const ARM_Y = CRADLE_CORNER[1] - ARM_FOAM_R;
/** 📐️ The top bar: the point where the outward normal of the guitar back, a foam radius off, meets the front tube line. */
const BAR = (() => {
  const off = VB + TOP_FOAM_R;
  const dir = [HINGE[0] - FRONT_FOOT[0], HINGE[1] - FRONT_FOOT[1]];
  const base = gw(0, off);
  const a = [[U_DIR[0], -dir[0]!], [U_DIR[1], -dir[1]!]];
  const b = [FRONT_FOOT[0] - base[0], FRONT_FOOT[1] - base[1]];
  const det = a[0]![0]! * a[1]![1]! - a[0]![1]! * a[1]![0]!;
  const u = (b[0]! * a[1]![1]! - a[0]![1]! * b[1]!) / det;
  const p = gw(u, off);
  return { u, x: p[0], y: p[1] };
})();
const onFront = (y: number): number => FRONT_FOOT[0] + ((HINGE[0] - FRONT_FOOT[0]) * (y - FRONT_FOOT[1])) / (HINGE[1] - FRONT_FOOT[1]);
const onRear = (y: number): number => REAR_FOOT[0] + ((HINGE[0] - REAR_FOOT[0]) * (y - REAR_FOOT[1])) / (HINGE[1] - REAR_FOOT[1]);
const FRONT_CROSS: V2 = [onFront(ARM_Y), ARM_Y];
const REAR_CROSS: V2 = [onRear(0.03), 0.03];
const BAG_O: V3 = [REAR_CROSS[0] - TUBE_R, 0, 0];
const PLAYED_FRAME = guitarR(PLAYED.angle, PLAYED.abduction);
const WAIST: V3 = [0.292, 0, halfWidth(BODY_TABLE, 0.292)];
const WAIST_W = apply(PLAYED_FRAME, WAIST);
const PLAYED_O: V3 = [PLAYED.waist[0] - WAIST_W[0], PLAYED.waist[1] - WAIST_W[1], 0];
FS.z = r5(WAIST_W[2]);
//#endregion

//#region materials
const MAT: Json[] = [
  { id: "spruce-top", title: t("Spruce soundboard under aged, worn shellac", "Fichtendecke unter gealterter, abgespielter Schellackpolitur"), emissivity: 0.9, solarAbsorptivity: 0.52, densityKgM3: 430, conductivityWmK: 0.11, specificHeatJKgK: 1600, elasticModulusPa: 1.1e10, source: "literature", reference: "wood-handbook-2010" },
  { id: "rosewood-laminate", title: t("Rosewood-faced laminate of back and ribs", "Palisander-Sperrholz von Boden und Zargen"), emissivity: 0.9, solarAbsorptivity: 0.78, densityKgM3: 650, conductivityWmK: 0.15, specificHeatJKgK: 1500, elasticModulusPa: 9e9, source: "authored-estimate", reference: "wood-handbook-2010" },
  { id: "cedar-neck", title: t("Spanish cedar neck and head", "Hals und Kopf aus Zedrele"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 470, conductivityWmK: 0.12, specificHeatJKgK: 1600, elasticModulusPa: 9e9, source: "literature", reference: "wood-handbook-2010" },
  { id: "rosewood", title: t("Rosewood fingerboard and bridge, finger-polished", "Griffbrett und Steg aus Palisander, fingerpoliert"), emissivity: 0.9, solarAbsorptivity: 0.82, densityKgM3: 850, conductivityWmK: 0.17, specificHeatJKgK: 1500, elasticModulusPa: 1.2e10, source: "literature", reference: "wood-handbook-2010" },
  { id: "bone", title: t("Cattle bone nut and saddle, yellowed", "Sattel und Stegeinlage aus Rinderknochen, vergilbt"), emissivity: 0.92, solarAbsorptivity: 0.35, densityKgM3: 1900, conductivityWmK: 0.4, specificHeatJKgK: 1300, source: "authored-estimate", reference: "incropera-2018" },
  { id: "nylon-string", title: t("Nylon and silver-wound strings", "Nylon- und versilberte Umspinnungssaiten"), emissivity: 0.9, solarAbsorptivity: 0.3, densityKgM3: 1140, conductivityWmK: 0.25, specificHeatJKgK: 1670, elasticModulusPa: 4.5e9, source: "literature", reference: "french-2009" },
  { id: "nickel-brass", title: t("Nickel-plated brass tuner plates, tarnished", "Vernickelte Messing-Mechanikplatten, angelaufen"), emissivity: 0.12, solarAbsorptivity: 0.4, densityKgM3: 8500, conductivityWmK: 110, specificHeatJKgK: 380, elasticModulusPa: 1.0e11, source: "literature", reference: "incropera-2018" },
  { id: "tuner-plastic", title: t("Cream cellulose tuner buttons and rollers, yellowed", "Cremefarbene Mechanikknöpfe und -wellen aus Zelluloid, vergilbt"), emissivity: 0.9, solarAbsorptivity: 0.4, densityKgM3: 1300, conductivityWmK: 0.2, specificHeatJKgK: 1500, source: "authored-estimate", reference: "incropera-2018" },
  { id: "nickel-silver", title: t("Nickel-silver fret wire, worn flat under the first frets", "Neusilber-Bundstäbchen, unter den ersten Bünden flach gespielt"), emissivity: 0.15, solarAbsorptivity: 0.4, densityKgM3: 8700, conductivityWmK: 30, specificHeatJKgK: 400, source: "literature", reference: "incropera-2018" },
  { id: "steel-powder", title: t("Black powder-coated steel tube 16 x 0.8 mm, chipped", "Schwarz pulverbeschichtetes Stahlrohr 16 x 0,8 mm, abgeplatzt"), emissivity: 0.9, solarAbsorptivity: 0.92, densityKgM3: 7850, conductivityWmK: 50, specificHeatJKgK: 460, elasticModulusPa: 2.1e11, strengthPa: 2.35e8, source: "literature", reference: "iso-10456-2007" },
  { id: "pe-foam", title: t("Closed-cell polyethylene foam, compressed and torn", "Geschlossenzelliger Polyethylenschaum, gestaucht und eingerissen"), emissivity: 0.9, solarAbsorptivity: 0.9, densityKgM3: 45, conductivityWmK: 0.04, specificHeatJKgK: 2300, source: "literature", reference: "iso-10456-2007" },
  { id: "rubber", title: t("Black rubber caps and pads, hardened", "Schwarze Gummikappen und -auflagen, verhärtet"), emissivity: 0.93, solarAbsorptivity: 0.92, densityKgM3: 1200, conductivityWmK: 0.17, specificHeatJKgK: 1900, source: "literature", reference: "iso-10456-2007" },
  { id: "padded-nylon", title: t("Faded black polyester shell over 10 mm foam padding", "Ausgeblichene schwarze Polyesterhülle über 10 mm Schaumpolster"), emissivity: 0.88, solarAbsorptivity: 0.85, densityKgM3: 85, conductivityWmK: 0.045, specificHeatJKgK: 1700, source: "authored-estimate", reference: "iso-10456-2007" },
  { id: "webbing", title: t("Padded polypropylene webbing straps and handle", "Gepolsterte Polypropylen-Gurtbänder und Griff"), emissivity: 0.88, solarAbsorptivity: 0.85, densityKgM3: 300, conductivityWmK: 0.06, specificHeatJKgK: 1800, source: "authored-estimate", reference: "iso-10456-2007" },
  { id: "steel-black", title: t("Black-lacquered sheet and tube steel of the footstool", "Schwarz lackierter Blech- und Rohrstahl der Fußbank"), emissivity: 0.9, solarAbsorptivity: 0.9, densityKgM3: 7850, conductivityWmK: 50, specificHeatJKgK: 460, elasticModulusPa: 2.1e11, strengthPa: 2.35e8, source: "literature", reference: "iso-10456-2007" },
];
const DENSITY = Object.fromEntries(MAT.map((m) => [m.id, m.densityKgM3 as number]));
//#endregion

//#region rig
const bone = (id: string, parent: string | null, en: string, de: string, x: number, y: number, angleDeg: number, lengthM: number, joint: Json, extra: Json = {}): Json => ({ id, parent, title: t(en, de), x: r5(x), y: r5(y), angleDeg, lengthM, joint, ...extra });
const hingeLocal = [HINGE[0] * -1, HINGE[1] * -1];
const rearAngleWorld = (Math.atan2(REAR_FOOT[1] - HINGE[1], REAR_FOOT[0] - HINGE[0]) * 180) / Math.PI;
const rearLen = Math.hypot(REAR_FOOT[1] - HINGE[1], REAR_FOOT[0] - HINGE[0]);
const FRONT_LEG = { hinge: [FS.toe - 0.02, 0.14] as V2, foot: [FS.toe + 0.01, 0.009] as V2, z: 0.0405 };
const REAR_LEG = { hinge: [FS.heel + 0.04, 0.14] as V2, foot: [FS.heel - 0.02, 0.009] as V2, z: 0.033 };
const legAngle = (l: { hinge: V2; foot: V2 }) => r5((Math.atan2(l.foot[1] - l.hinge[1], l.foot[0] - l.hinge[0]) * 180) / Math.PI);
const legLen = (l: { hinge: V2; foot: V2 }) => r5(Math.hypot(l.foot[1] - l.hinge[1], l.foot[0] - l.hinge[0]));
const BONES: Json[] = [
  bone("stand", null, "Folding A-frame stand: front frame with cradle and padded backrest", "Klappbarer A-Ständer: Vorderrahmen mit Auflage und gepolsterter Rückenlehne", 0, 0, 180, 0.43, { type: "fixed" }),
  bone("stand-rear", "stand", "Rear frame of the stand on the head hinge", "Hinterrahmen des Ständers am Kopfgelenk", hingeLocal[0]!, hingeLocal[1]!, r5(rearAngleWorld - 180), r5(rearLen), { type: "revolute", minDeg: 0, maxDeg: 40 }),
  bone("guitar-slide-x", "stand", "Guitar carriage, forward travel", "Gitarrenführung, Längsweg", -G_O[0], -G_O[1], 180, 0.01, { type: "prismatic", axisDeg: 0, minM: -1.5, maxM: 1.5 }),
  bone("guitar-slide-y", "guitar-slide-x", "Guitar carriage, lift", "Gitarrenführung, Hub", 0, 0, 0, 0.01, { type: "prismatic", axisDeg: 90, minM: -0.2, maxM: 1.5 }),
  bone("guitar", "guitar-slide-y", "Classical guitar from the tail block to the head", "Konzertgitarre vom Unterklotz bis zum Kopf", 0, 0, G_ANGLE, 0.99, { type: "revolute", minDeg: -20, maxDeg: 90 }),
  bone("bag-slide-x", "stand", "Gig bag carriage, forward travel", "Taschenführung, Längsweg", -BAG_O[0], 0, 180, 0.01, { type: "prismatic", axisDeg: 0, minM: -1.5, maxM: 1.5 }),
  bone("bag-slide-y", "bag-slide-x", "Gig bag carriage, lift", "Taschenführung, Hub", 0, 0, 0, 0.01, { type: "prismatic", axisDeg: 90, minM: -0.2, maxM: 1.5 }),
  bone("bag", "bag-slide-y", "Gig bag body from the tail to the head pocket", "Gigbag vom Boden bis zur Kopftasche", 0, 0, 180, BAG_LEN, { type: "revolute", minDeg: -100, maxDeg: 10 }),
  bone("bag-lid", "bag", "Front panel of the gig bag, folding back at the head end", "Vorderteil des Gigbags, am Kopfende aufklappend", BAG_LEN, BAG_V.lid, 180, BAG_LEN, { type: "revolute", minDeg: 0, maxDeg: 110 }),
  { ...bone("footstool", "stand", "Adjustable guitar footstool: top plate", "Verstellbare Gitarrenfußbank: Trittplatte", -FS.heel, -FS.y, -180, r5(FS.toe - FS.heel), { type: "fixed" }), z: FS.z },
  bone("footstool-front-leg", "footstool", "Front leg frame of the footstool", "Vorderer Beinbügel der Fußbank", FRONT_LEG.hinge[0] - FS.heel, FRONT_LEG.hinge[1] - FS.y, legAngle(FRONT_LEG), legLen(FRONT_LEG), { type: "revolute", minDeg: 0, maxDeg: 85 }),
  bone("footstool-rear-leg", "footstool", "Rear leg frame of the footstool in its height notch", "Hinterer Beinbügel der Fußbank in seiner Höhenraste", REAR_LEG.hinge[0] - FS.heel, REAR_LEG.hinge[1] - FS.y, legAngle(REAR_LEG), legLen(REAR_LEG), { type: "revolute", minDeg: -85, maxDeg: 0 }),
];
const RIG = { kind: "mechanical", root: "stand", bones: BONES };
//#endregion

//#region parts
type Part = { id: string; bone: string; solid: Json; material: string; tone: number; region: string | null; role?: string; draw?: Partial<Record<View, string | false>>; drawRegion?: Partial<Record<View, string>>; stroke?: "body" | "texture"; massFactor?: number };
let FR: Record<string, Frame> = {};
/** 🌐️ A solid authored in world rest coordinates, carried into the frame of its bone. */
function worldSolid(solid: Json, boneId: string): Json {
  const f = FR[boneId]!;
  const ang = Math.atan2(f.u[1], f.u[0]) * (180 / Math.PI);
  const qb = about("w", ang);
  const qinv: Q = [-qb[0], -qb[1], -qb[2], qb[3]];
  const out = { ...solid };
  if (solid.kind === "capsule") {
    out.p0 = p3(toLocal(f, solid.p0));
    out.p1 = p3(toLocal(f, solid.p1));
    return out;
  }
  if (solid.kind === "revolve") {
    out.origin = p3(toLocal(f, solid.origin));
    out.axis = p3(qrot(qinv, solid.axis));
    return out;
  }
  const own = solid.transform ?? {};
  const rq: Q = own.rotation ?? [0, 0, 0, 1];
  const tw: V3 = own.translation ?? [0, 0, 0];
  out.transform = { rotation: qmul(qinv, rq), translation: p3(toLocal(f, tw)) };
  return out;
}
const P = (id: string, bone: string, solid: Json, material: string, tone: number, region: string | null, extra: Partial<Part> = {}): Part => ({ id, bone, solid: { id, bone, ...solid }, material, tone, region, ...extra });
const W = (id: string, bone: string, solid: Json, material: string, tone: number, region: string | null, extra: Partial<Part> = {}): Part => P(id, bone, worldSolid(solid, bone), material, tone, region, extra);
const zx = (profile: V2[], from: number, to: number, holes?: V2[][]): Json => ({ kind: "extrude", plane: "zx", profile: profile.map(p2), ...(holes ? { holes: holes.map((h) => h.map(p2)) } : {}), from: r5(from), to: r5(to) });
const xy = (profile: V2[], from: number, to: number): Json => ({ kind: "extrude", plane: "xy", profile: profile.map(p2), from: r5(from), to: r5(to) });
const box = (c: V3, s: V3, extra: Json = {}): Json => ({ kind: "box", size: p3(s), transform: { translation: p3(c) }, ...extra });
const cap = (a: V3, b: V3, r: number): Json => ({ kind: "capsule", p0: p3(a), p1: p3(b), r0: r, r1: r });
const ell = (c: V3, r: V3): Json => ({ kind: "ellipsoid", radii: p3(r), transform: { translation: p3(c) } });
const rev = (origin: V3, axis: V3, profile: V2[]): Json => ({ kind: "revolve", origin: p3(origin), axis: p3(axis), profile: profile.map(p2) });
const sleeve = (a0: number, a1: number, rin: number, rout: number): V2[] => [[a0, rin], [a1, rin], [a1, rout], [a0, rout], [a0, rin]];
const HEAD_Q = about("w", HEAD_DEG);
const HEAD_T0: V3 = [U_NUT, VT, 0];
const inHead = (s: number, n: number, w: number): V3 => {
  const r = qrot(HEAD_Q, [s, n, w]);
  return [HEAD_T0[0] + r[0], HEAD_T0[1] + r[1], r[2]];
};
const fretU = (n: number) => U_NUT - SCALE_LEN * (1 - 2 ** (-n / 12));
const fbHalf = (u: number) => 0.026 + ((0.0325 - 0.026) * (U_NUT - u)) / (U_NUT - FB_END);
const NECK_D: V2[] = (() => {
  const pts: V2[] = [[-0.0485, 0.03]];
  for (let k = 0; k <= 12; k++) {
    const a = (Math.PI * k) / 12;
    pts.push([-0.0485 + 0.019 * Math.sin(a), 0.03 * Math.cos(a)]);
  }
  return ccw(pts.slice(1));
})();
const HEEL: V2[] = ccw([[BODY_LEN, VT], [BODY_LEN, VB], [0.492, VB], [0.503, 0.036], [0.517, 0.014], [0.532, -0.008], [0.55, -0.021], [0.572, -0.028], [0.585, -0.0295], [0.585, VT]]);
/** 🪵️ The side profile of the whole neck at the coarse scales: heel, shaft and the fingerboard tongue over the top, in (u, v). */
const NECK_SIDE: V2[] = ccw([[FB_END, VT - FB_T], [U_NUT, VT - FB_T], [U_NUT, -0.032], [0.7, -0.0305], [0.6, -0.029], [0.555, -0.022], [0.53, -0.006], [0.51, 0.02], [0.492, VB], [BODY_LEN, VB], [BODY_LEN, VT], [FB_END, VT]]);
const HEAD_SIDE = (thick: number): V2[] => {
  const c = [inHead(0, 0, 0), inHead(HEAD_LEN, 0, 0), inHead(HEAD_LEN, thick, 0), inHead(0, thick, 0)];
  return ccw(c.map((p) => [p[0], p[1]] as V2));
};
const STRINGS = [-0.029, -0.0174, -0.0058, 0.0058, 0.0174, 0.029].map((ws, i) => ({ ws, wn: ws * (0.0215 / 0.029), r: [0.00055, 0.00045, 0.00052, 0.00041, 0.00036, 0.00035][i]!, wound: i < 3 }));

/** 🎸️ The guitar at one scale, in its own frame. */
function guitarParts(s: Scale): Part[] {
  const ring = bodyRing(s);
  const hole = circle(0, HOLE_U, HOLE_R, s === 50 ? 40 : 24).map(([a, b]) => [a, b] as V2);
  const parts: Part[] = [];
  const g = "guitar";
  const fb: V2[] = ccw([[-fbHalf(U_NUT), U_NUT], [fbHalf(U_NUT), U_NUT], [fbHalf(FB_END), FB_END], [-fbHalf(FB_END), FB_END]]);
  if (s === 50 || s === 100) {
    parts.push(P("top", g, { ...zx(ring, VT, VT + TOP_T, [hole]), sheet: true }, "spruce-top", 0.8, "top", { draw: { elevation: "body" } }));
    parts.push(P("ribs", g, zx(ring, VT + TOP_T, VB - BACK_T, [inset(ring, RIB_T)]), "rosewood-laminate", 0.38, "back-ribs", { draw: { elevation: "body" }, drawRegion: { elevation: "back-ribs" } }));
    parts.push(P("back", g, { ...zx(ring, VB - BACK_T, VB), sheet: true }, "rosewood-laminate", 0.36, "back-ribs", { draw: { elevation: "body" } }));
  } else {
    parts.push(P("top", g, { ...zx(ring, VT, VT + TOP_T), sheet: true }, "spruce-top", 0.8, "top", { draw: { elevation: "body" } }));
    parts.push(P("ribs", g, zx(ring, VT + TOP_T, VB), "rosewood-laminate", 0.38, "back-ribs", { draw: { elevation: "body" }, drawRegion: { elevation: "back-ribs" } }));
  }
  if (s === 50) {
    parts.push(P("heel", g, xy(HEEL, -0.03, 0.03), "cedar-neck", 0.5, "neck"));
    parts.push(P("neck-shaft", g, { kind: "extrude", plane: "yz", profile: NECK_D.map(p2), from: 0.56, to: U_NUT + 0.002, taperTo: [1, 0.87] }, "cedar-neck", 0.5, "neck", { draw: { elevation: "neck-shaft", front: "neck-shaft", plan: "neck-shaft" } }));
    parts.push(P("fretboard", g, zx(fb, VT - FB_T, VT), "rosewood", 0.18, "neck"));
    parts.push(P("nut", g, box([U_NUT - 0.003, VT - 0.0055, 0], [0.006, 0.011, 0.052]), "bone", 0.88, "neck"));
  } else {
    parts.push(P("neck-shaft", g, xy(NECK_SIDE, -0.03, 0.03), "cedar-neck", 0.45, "neck"));
  }
  if (s === 50 || s === 100) {
    parts.push(P("head", g, { ...zx(HEAD_RING, 0, HEAD_T, SLOTS), transform: { rotation: HEAD_Q, translation: p3(HEAD_T0) } }, "cedar-neck", 0.42, "neck"));
  } else {
    parts.push(P("head", g, xy(HEAD_SIDE(HEAD_T), -0.0375, 0.0375), "cedar-neck", 0.42, "neck"));
  }
  if (s === 50) {
    for (const side of [-1, 1]) {
      const c = inHead(0.1, 0.0095, side * 0.0415);
      parts.push(P(`tuner-plate-${side < 0 ? "bass" : "treble"}`, g, { ...box([0, 0, 0], [0.11, 0.015, 0.008]), transform: { rotation: HEAD_Q, translation: p3(c) } }, "nickel-brass", 0.62, "tuners", { massFactor: 0.3 }));
    }
    [0.06, 0.1, 0.14].forEach((sp, i) => {
      parts.push(P(`roller-${i + 1}`, g, rev(inHead(sp, 0.0095, 0), [0, 0, 1], [[-0.0375, 0], [-0.0375, 0.005], [0.0375, 0.005], [0.0375, 0]]), "tuner-plastic", 0.86, "tuners"));
      for (const side of [-1, 1]) parts.push(P(`button-${side < 0 ? "bass" : "treble"}-${i + 1}`, g, { ...ell([0, 0, 0], [0.0105, 0.016, 0.0045]), transform: { rotation: HEAD_Q, translation: p3(inHead(sp, 0.03, side * 0.043)) } }, "tuner-plastic", 0.9, "tuners"));
    });
  } else if (s === 100) {
    for (const side of [-1, 1]) parts.push(P(`tuners-${side < 0 ? "bass" : "treble"}`, g, { ...box([0, 0, 0], [0.11, 0.046, 0.015]), transform: { rotation: HEAD_Q, translation: p3(inHead(0.1, 0.023, side * 0.045)) } }, "nickel-brass", 0.7, "tuners", { massFactor: 0.15 }));
  } else {
    parts.push(P("tuners", g, { ...box([0, 0, 0], [0.11, 0.048, 0.105]), transform: { rotation: HEAD_Q, translation: p3(inHead(0.1, 0.022, 0)) } }, "nickel-brass", 0.7, "tuners", { massFactor: 0.03 }));
  }
  if (s === 50) {
    parts.push(P("bridge", g, zx(roundRect(0, (U_SADDLE - 0.024 + U_SADDLE + 0.004) / 2, 0.185, 0.028, 0.006, 3), VT - 0.005, VT), "rosewood", 0.22, "top"));
    parts.push(P("tie-block", g, box([U_SADDLE - 0.013, VT - 0.007, 0], [0.018, 0.004, 0.115]), "rosewood", 0.2, "top"));
    parts.push(P("saddle", g, box([U_SADDLE, VT - 0.0085, 0], [0.003, 0.007, 0.078]), "bone", 0.88, "top"));
  } else if (s === 100) {
    parts.push(P("bridge", g, box([U_SADDLE - 0.01, VT - 0.0045, 0], [0.028, 0.009, 0.185]), "rosewood", 0.22, "top"));
  }
  if (s === 50) {
    STRINGS.forEach((st, i) => {
      const tie: V3 = [U_SADDLE - 0.023, VT - 0.009 - st.r, st.ws * 0.98];
      const sad: V3 = [U_SADDLE, VT - 0.012 - st.r, st.ws];
      const nut: V3 = [U_NUT - 0.003, VT - 0.011 - st.r, st.wn];
      const slot = st.ws < 0 ? -0.0185 : 0.0185;
      const order = [0.06, 0.1, 0.14][Math.abs(i - (st.ws < 0 ? 0 : 5))]!;
      const roll = inHead(order, 0.0045 - st.r, slot);
      const mat = "nylon-string";
      const tone = st.wound ? 0.62 : 0.92;
      parts.push(P(`string-${i + 1}-tie`, g, { ...cap(tie, sad, st.r), slender: true }, mat, tone, "neck", { stroke: "texture", draw: { elevation: false, plan: false, front: `string-${i + 1}` } }));
      parts.push(P(`string-${i + 1}`, g, { ...cap(sad, nut, st.r), slender: true }, mat, tone, "neck", { stroke: "texture", draw: { elevation: false, plan: false } }));
      parts.push(P(`string-${i + 1}-head`, g, { ...cap(nut, roll, st.r), slender: true }, mat, tone, "neck", { stroke: "texture", draw: { elevation: false, plan: false, front: `string-${i + 1}` } }));
    });
    for (let n = 1; n <= 19; n++) {
      const u = fretU(n);
      parts.push(P(`fret-${n}`, g, { ...box([u, VT - FB_T - 0.0006, 0], [0.0025, 0.0012, 2 * fbHalf(u)]), texture: true }, "nickel-silver", 0.72, null, { role: "detail", draw: { elevation: false, plan: false, front: false } }));
    }
  }
  if (s === 50 || s === 100) {
    parts.push(P("rosette", g, { ...zx(circle(0, HOLE_U, 0.062, 36), VT - 0.0002, VT, [circle(0, HOLE_U, HOLE_R + 0.0005, 36)]), texture: true }, "rosewood", 0.3, null, { role: "detail", draw: { elevation: false, plan: false, front: false } }));
    parts.push(P("wear-soundhole", g, { ...zx(ccw([[0.004, 0.255], [0.06, 0.262], [0.085, 0.29], [0.072, 0.31], [0.04, 0.296], [0.006, 0.283]]), VT - 0.0002, VT), texture: true }, "spruce-top", 0.95, null, { role: "detail", draw: { elevation: false, plan: false, front: false } }));
  }
  return parts;
}

/** 🛠️ The folding A-frame stand at one scale, in world rest coordinates. */
function standParts(s: Scale): Part[] {
  const parts: Part[] = [];
  const st = "stand";
  const sr = "stand-rear";
  const slender = s === 500;
  const line: Partial<Part> = slender ? { stroke: "body" } : {};
  for (const side of [-1, 1]) {
    const n = side < 0 ? "r" : "l";
    parts.push(W(`front-tube-${n}`, st, { ...cap([FRONT_FOOT[0], FRONT_FOOT[1], side * FRONT_Z], [HINGE[0], HINGE[1], side * FRONT_Z], TUBE_R), ...(slender ? { slender: true } : {}) }, "steel-powder", 0.15, "stand-frame", line));
    parts.push(W(`rear-tube-${n}`, sr, { ...cap([HINGE[0], HINGE[1], side * REAR_Z], [REAR_FOOT[0], REAR_FOOT[1], side * REAR_Z], TUBE_R), ...(slender ? { slender: true } : {}) }, "steel-powder", 0.15, "stand-frame", line));
  }
  parts.push(W("rear-cross-bar", sr, cap([REAR_CROSS[0], REAR_CROSS[1], -REAR_Z], [REAR_CROSS[0], REAR_CROSS[1], REAR_Z], TUBE_R), "steel-powder", 0.15, "stand-frame"));
  if (s !== 500) {
    parts.push(W("top-bar", st, cap([BAR.x, BAR.y, -FRONT_Z], [BAR.x, BAR.y, FRONT_Z], TUBE_R), "steel-powder", 0.15, "stand-frame"));
    parts.push(W("top-foam", st, rev([BAR.x, BAR.y, 0], [0, 0, 1], sleeve(-0.125, 0.125, TUBE_R, TOP_FOAM_R)), "pe-foam", 0.1, "stand-padding"));
    parts.push(W("front-cross-bar", st, cap([FRONT_CROSS[0], FRONT_CROSS[1], -FRONT_Z], [FRONT_CROSS[0], FRONT_CROSS[1], FRONT_Z], TUBE_R), "steel-powder", 0.15, "stand-frame"));
    for (const side of [-1, 1]) {
      const n = side < 0 ? "r" : "l";
      parts.push(W(`arm-${n}`, st, cap([FRONT_CROSS[0], ARM_Y, side * ARM_Z], [0.058, ARM_Y, side * ARM_Z], TUBE_R), "steel-powder", 0.15, "stand-frame"));
      parts.push(W(`arm-tip-${n}`, st, cap([0.058, ARM_Y, side * ARM_Z], [0.07, 0.1, side * ARM_Z], TUBE_R), "steel-powder", 0.15, "stand-frame"));
      parts.push(W(`arm-foam-${n}`, st, rev([0, ARM_Y, side * ARM_Z], [1, 0, 0], sleeve(FRONT_CROSS[0] + 0.012, 0.05, TUBE_R, ARM_FOAM_R)), "pe-foam", 0.1, "stand-padding"));
    }
  } else {
    parts.push(W("top-foam", st, cap([BAR.x, BAR.y, -FRONT_Z], [BAR.x, BAR.y, FRONT_Z], TOP_FOAM_R), "pe-foam", 0.1, "stand-padding"));
    parts.push(W("cradle", st, box([(FRONT_CROSS[0] + 0.058) / 2 - 0.004, ARM_Y, 0], [0.058 - FRONT_CROSS[0] + 0.016, 2 * ARM_FOAM_R, 2 * FRONT_Z + 0.016]), "pe-foam", 0.12, "stand-padding"));
  }
  if (s === 50 || s === 100) {
    const fr = s === 50 ? [0.013, 0.012, 0.012] : [0.016, 0.012, 0.016];
    for (const side of [-1, 1]) {
      const n = side < 0 ? "r" : "l";
      parts.push(W(`foot-front-${n}`, st, ell([FRONT_FOOT[0], fr[1]!, side * FRONT_Z], fr as V3), "rubber", 0.08, "stand-padding"));
      parts.push(W(`foot-rear-${n}`, sr, ell([REAR_FOOT[0], fr[1]!, side * REAR_Z], fr as V3), "rubber", 0.08, "stand-padding"));
    }
  }
  if (s === 50) {
    for (const side of [-1, 1]) parts.push(W(`hinge-bolt-${side < 0 ? "r" : "l"}`, st, rev([HINGE[0], HINGE[1], side * 0.131], [0, 0, 1], [[-0.0185, 0], [-0.0185, 0.0045], [0.0185, 0.0045], [0.0185, 0]]), "steel-powder", 0.3, "stand-frame"));
    for (const side of [-1, 1]) parts.push(W(`tip-cap-${side < 0 ? "r" : "l"}`, st, ell([0.07, 0.1, side * ARM_Z], [0.011, 0.011, 0.011]), "rubber", 0.08, "stand-padding"));
  }
  for (const p of parts) if (p.material === "steel-powder" && p.solid.kind === "capsule") p.massFactor = TUBE_FACTOR;
  return parts;
}

/** 👜️ The worn gig bag at one scale, in its own frame (u from the tail to the head, v down at rest, w lateral). */
function bagParts(s: Scale): Part[] {
  const ring = bagRing(s);
  const parts: Part[] = [];
  const lidRing = ring.map(([w, u]) => [w, BAG_LEN - u] as V2);
  if (s === 50 || s === 100) {
    const back0 = s === 50 ? BAG_V.back : BAG_V.strap;
    parts.push(P("back-panel", "bag", zx(ring, BAG_V.gusset0, back0), "padded-nylon", 0.12, "bag-shell"));
    parts.push(P("gusset", "bag", zx(ring, BAG_V.gusset1, BAG_V.gusset0, [inset(ring, BAG_PANEL)]), "padded-nylon", 0.14, "bag-shell"));
    parts.push(P("lid-panel", "bag-lid", zx(ccw(lidRing), -BAG_PANEL, 0), "padded-nylon", 0.13, "bag-shell"));
  } else {
    parts.push(P("bag-body", "bag", zx(ring, BAG_V.gusset1, BAG_V.strap), "padded-nylon", 0.13, "bag-shell"));
    parts.push(P("lid-panel", "bag-lid", zx(ccw(lidRing), -BAG_PANEL, 0), "padded-nylon", 0.13, "bag-shell"));
  }
  if (s !== 500) parts.push(P("pocket", "bag-lid", zx(ccw(roundRect(0, BAG_LEN - 0.25, 0.24, 0.3, s === 200 ? 0.03 : 0.04, s === 50 ? 4 : 2)), 0, 0.03), "padded-nylon", 0.2, "bag-shell"));
  if (s === 50) {
    for (const side of [-1, 1]) parts.push(P(`strap-${side < 0 ? "r" : "l"}`, "bag", zx(ccw(roundRect(side * 0.075, 0.475, 0.05, 0.55, 0.012, 3)), BAG_V.back, BAG_V.strap), "webbing", 0.22, "bag-shell"));
    const hu = 0.32;
    const hw = halfWidth(BAG_TABLE, hu);
    parts.push(P("handle", "bag", cap([hu - 0.05, -0.078, -(hw + 0.008)], [hu + 0.05, -0.078, -(halfWidth(BAG_TABLE, hu + 0.05) + 0.008)], 0.009), "webbing", 0.25, "bag-shell"));
    parts.push(P("wear-tail", "bag", { ...zx(ccw([[-0.09, 0.0], [0.09, 0.0], [0.11, 0.03], [-0.1, 0.035]].map(([w, u]) => [w!, u!] as V2)), BAG_V.gusset1 - 0.0002, BAG_V.gusset1), texture: true }, "padded-nylon", 0.45, null, { role: "detail", draw: { elevation: false, front: false, plan: false } }));
  }
  return parts;
}

/** 🦶️ The guitarist's footstool at one scale, in world rest coordinates. */
function footstoolParts(s: Scale): Part[] {
  const parts: Part[] = [];
  const zf = FS.z;
  const plateTop = FS.y + 0.0115;
  if (s === 500) {
    parts.push(W("footstool-block", "footstool", box([(FS.heel + FS.toe) / 2, plateTop / 2 + 0.0015, zf], [FS.toe - FS.heel + 0.03, plateTop + 0.003, FS.w]), "steel-black", 0.2, "footstool-frame"));
    parts.push(W("pad", "footstool", { ...box([(FS.heel + FS.toe) / 2, plateTop + 0.0015 + 0.0015, zf], [FS.toe - FS.heel - 0.015, 0.003, FS.w - 0.01]), sheet: true }, "rubber", 0.06, "footstool-pad"));
    return parts;
  }
  const channel: V2[] = ccw([[0, 0.046], [0, 0.0475], [0.0115, 0.0475], [0.0115, -0.0475], [0, -0.0475], [0, -0.046], [0.01, -0.046], [0.01, 0.046]]);
  if (s === 200) parts.push(W("plate", "footstool", box([(FS.heel + FS.toe) / 2, FS.y + 0.00725, zf], [FS.toe - FS.heel, 0.0145, FS.w]), "steel-black", 0.2, "footstool-frame"));
  else parts.push(W("plate", "footstool", { kind: "extrude", plane: "yz", profile: channel.map(p2), from: 0, to: r5(FS.toe - FS.heel), transform: { rotation: [0, 0, 0, 1], translation: [FS.heel, FS.y, zf] } }, "steel-black", 0.2, "footstool-frame"));
  parts.push(W("pad", "footstool", { ...box([(FS.heel + FS.toe) / 2, plateTop + 0.0015, zf], [FS.toe - FS.heel - 0.015, 0.003, FS.w - 0.01]), sheet: true }, "rubber", 0.06, "footstool-pad"));
  for (const [legId, leg] of [["footstool-front-leg", FRONT_LEG], ["footstool-rear-leg", REAR_LEG]] as const) {
    const k = legId.includes("front") ? "front" : "rear";
    for (const side of [-1, 1]) parts.push(W(`${k}-leg-${side < 0 ? "r" : "l"}`, legId, cap([leg.hinge[0], leg.hinge[1], zf + side * leg.z], [leg.foot[0], leg.foot[1], zf + side * leg.z], 0.005), "steel-black", 0.22, "footstool-frame", { massFactor: LEG_FACTOR }));
    parts.push(W(`${k}-leg-bar`, legId, cap([leg.foot[0], leg.foot[1], zf - leg.z], [leg.foot[0], leg.foot[1], zf + leg.z], 0.005), "steel-black", 0.22, "footstool-frame", { massFactor: LEG_FACTOR }));
    parts.push(W(`${k}-leg-shoe`, legId, rev([leg.foot[0], leg.foot[1], zf], [0, 0, 1], sleeve(-leg.z + 0.004, leg.z - 0.004, 0.005, 0.009)), "rubber", 0.06, "footstool-pad"));
  }
  if (s === 50) {
    parts.push(W("rack", "footstool", box([FS.heel + 0.085, FS.y + 0.005, zf], [0.1, 0.01, 0.012]), "steel-black", 0.25, "footstool-frame", { massFactor: 0.35 }));
    for (const [k, leg] of [["front", FRONT_LEG], ["rear", REAR_LEG]] as const) parts.push(W(`${k}-hinge-pin`, "footstool", cap([leg.hinge[0], leg.hinge[1], zf - 0.046], [leg.hinge[0], leg.hinge[1], zf + 0.046], 0.004), "steel-black", 0.3, "footstool-frame"));
  }
  return parts;
}
//#endregion

//#region anchors
const A = (id: string, b: string, x: number, y: number, z: number, kind: string, en: string, de: string, extra: Json = {}): Json => ({ id, bone: b, x: r5(x), y: r5(y), ...(z !== 0 ? { z: r5(z) } : {}), kind, title: t(en, de), ...extra });
let ANCHORS: Json[] = [];
function buildAnchors(): Json[] {
  const st = FR["stand"]!;
  const sr = FR["stand-rear"]!;
  const fs = FR["footstool"]!;
  const fl = FR["footstool-front-leg"]!;
  const rl = FR["footstool-rear-leg"]!;
  const loc = (f: Frame, p: V3) => toLocal(f, p);
  const contact = { kind: "contact", support: "contact" };
  const out: Json[] = [];
  for (const side of [-1, 1]) {
    const n = side < 0 ? "right" : "left";
    const a = loc(st, [FRONT_FOOT[0], 0, side * FRONT_Z]);
    out.push(A(`stand-foot-front-${n}`, "stand", a[0], a[1], a[2], "contact", `Front ${n} rubber foot of the stand`, `${side < 0 ? "Rechter" : "Linker"} vorderer Gummifuß des Ständers`, { support: "contact", footprintLengthM: 0.026, footprintWidthM: 0.024, frictionCoefficient: 0.8 }));
    const b = loc(sr, [REAR_FOOT[0], 0, side * REAR_Z]);
    out.push(A(`stand-foot-rear-${n}`, "stand-rear", b[0], b[1], b[2], "contact", `Rear ${n} rubber foot of the stand`, `${side < 0 ? "Rechter" : "Linker"} hinterer Gummifuß des Ständers`, { support: "contact", footprintLengthM: 0.026, footprintWidthM: 0.024, frictionCoefficient: 0.8 }));
  }
  for (const [legId, leg, f] of [["footstool-front-leg", FRONT_LEG, fl], ["footstool-rear-leg", REAR_LEG, rl]] as const)
    for (const side of [-1, 1]) {
      const n = side < 0 ? "right" : "left";
      const k = legId.includes("front") ? "front" : "rear";
      const p = loc(f, [leg.foot[0], 0, FS.z + side * (leg.z - 0.01)]);
      out.push(A(`footstool-foot-${k}-${n}`, legId, p[0], p[1], p[2], "contact", `Footstool ${k} ${n} rubber shoe`, `Gummischuh der Fußbank ${k === "front" ? "vorn" : "hinten"} ${side < 0 ? "rechts" : "links"}`, { support: "contact", footprintLengthM: 0.018, footprintWidthM: 0.025, frictionCoefficient: 0.8 }));
    }
  void fs;
  out.push(A("bag-floor-tail", "bag", 0.22, 0, 0, "contact", "Bag on the floor under the lower strap ends", "Tasche auf dem Boden unter den unteren Gurtenden", { support: "contact", footprintLengthM: 0.1, footprintWidthM: 0.2, frictionCoefficient: 0.5 }));
  out.push(A("bag-floor-head", "bag", 0.73, 0, 0, "contact", "Bag on the floor under the upper strap ends", "Tasche auf dem Boden unter den oberen Gurtenden", { support: "contact", footprintLengthM: 0.1, footprintWidthM: 0.2, frictionCoefficient: 0.5 }));
  out.push(A("bag-tail-edge", "bag", 0, 0, 0, "contact", "Tail edge of the bag tipped on the floor", "Bodenkante der aufgekippten Tasche", { support: "contact", footprintLengthM: 0.02, footprintWidthM: 0.2, frictionCoefficient: 0.5 }));
  out.push(A("bag-grip-head", "bag", 1.02, BAG_V.gusset1 / 2, 0, "contact", "Hand at the head end lifting the bag", "Hand am Kopfende, die die Tasche anhebt", { support: "pin" }));
  out.push(A("strap-shoulders", "bag", 0.75, 0, 0, "contact", "Shoulder straps over the carrier's shoulders", "Schultergurte über den Schultern des Trägers", { support: "pin" }));
  out.push(A("strap-back", "bag", 0.25, 0, 0, "contact", "Lower bag end against the carrier's back", "Unteres Taschenende am Rücken des Trägers", { support: "contact", normalDeg: 0, footprintLengthM: 0.15, footprintWidthM: 0.25, frictionCoefficient: 0.5 }));
  out.push(A("guitar-grip-neck", "guitar", 0.65, -0.03, 0, "contact", "Hand round the neck lifting the guitar", "Hand um den Hals, die die Gitarre hebt", { support: "pin" }));
  out.push(A("guitar-grip-body", "guitar", 0.1, 0, -halfWidth(BODY_TABLE, 0.1), "contact", "Hand under the lower bout lifting the guitar", "Hand unter dem Unterbug, die die Gitarre hebt", { support: "pin" }));
  out.push(A("thigh-left", "guitar", WAIST[0], 0, WAIST[2], "contact", "Waist on the left thigh (raised by the footstool)", "Taille auf dem linken Oberschenkel (durch die Fußbank erhöht)", { support: "contact", footprintLengthM: 0.06, footprintWidthM: 0.09, frictionCoefficient: 0.6 }));
  out.push(A("thigh-right", "guitar", 0, 0, 0, "contact", "Tail of the lower bout against the right thigh", "Unterbug am rechten Oberschenkel", { support: "contact", tiltDeg: 30, footprintLengthM: 0.06, footprintWidthM: 0.09, frictionCoefficient: 0.6 }));
  out.push(A("chest", "guitar", 0.42, VB, 0.04, "contact", "Back of the upper bout against the chest", "Boden des Oberbugs an der Brust", { support: "contact", normalDeg: 0, footprintLengthM: 0.08, footprintWidthM: 0.12, frictionCoefficient: 0.5 }));
  out.push(A("saddle-load", "guitar", U_SADDLE, VT - 0.012, 0, "load", "Saddle: string tension on the bridge", "Stegeinlage: Saitenzug am Steg"));
  out.push(A("nut-load", "guitar", U_NUT - 0.003, VT - 0.011, 0, "load", "Nut: string tension on the head", "Sattel: Saitenzug am Kopf"));
  out.push(A("fret-hand", "guitar", 0.73, VT - FB_T, 0, "load", "Left-hand fingertips on the fingerboard", "Fingerkuppen der linken Hand auf dem Griffbrett"));
  out.push(A("forearm-rest", "guitar", 0.2, VT + 0.01, -0.17, "load", "Right forearm on the edge of the lower bout", "Rechter Unterarm auf der Kante des Unterbugs"));
  out.push(A("pluck-point", "guitar", 0.27, VT - 0.011, 0, "load", "Plucking point of the right hand fingers", "Anschlagstelle der rechten Hand"));
  return out;
}
const SUPPORTS = () => ANCHORS.filter((a) => a.id.startsWith("stand-foot") || a.id.startsWith("footstool-foot")).map((a) => a.id as string);
//#endregion

//#region poses
const REGIONS = ["top", "back-ribs", "neck", "tuners", "stand-frame", "stand-padding", "bag-shell", "footstool-frame", "footstool-pad"];
const STAND_FEET = ["stand-foot-front-left", "stand-foot-front-right", "stand-foot-rear-left", "stand-foot-rear-right"];
const STOOL_FEET = ["footstool-foot-front-left", "footstool-foot-front-right", "footstool-foot-rear-left", "footstool-foot-rear-right"];
const BAG_FLOOR = ["bag-floor-tail", "bag-floor-head"];
const TENSION_N = 360;
type GuitarPlace = { o: V2; dAngle: number; abd: number };
type BagPlace = { o: V2; dAngle: number; lid: number };
const STORED_G: GuitarPlace = { o: [G_O[0], G_O[1]], dAngle: 0, abd: 0 };
const STORED_B: BagPlace = { o: [BAG_O[0], 0], dAngle: 0, lid: 0 };
/** 🎒️ Where the guitar lies when it is inside the bag of a given placement. */
function inBag(b: BagPlace): GuitarPlace {
  const r = bagR(b.dAngle);
  const p = apply(r, G_IN_BAG);
  return { o: [b.o[0] + p[0], b.o[1] + p[1]], dAngle: 180 + b.dAngle - G_ANGLE, abd: 0 };
}
const CARRIED_B: BagPlace = { o: [-0.62, 0.8], dAngle: -82, lid: 0 };
const TILTED_B: BagPlace = { o: [BAG_O[0], 0], dAngle: -45, lid: 0 };
const PLAYED_G: GuitarPlace = { o: [PLAYED_O[0], PLAYED_O[1]], dAngle: PLAYED.angle, abd: PLAYED.abduction };
/** 🧩️ The bone offsets of a placement of guitar and bag. */
function offsets(g: GuitarPlace, b: BagPlace): Json {
  const out: Json = {};
  const gdx = -(g.o[0] - G_O[0]);
  const gdy = g.o[1] - G_O[1];
  if (Math.abs(gdx) > 1e-9) out["guitar-slide-x"] = { dx: r5(gdx) };
  if (Math.abs(gdy) > 1e-9) out["guitar-slide-y"] = { dy: r5(gdy) };
  if (g.dAngle !== 0 || g.abd !== 0) out["guitar"] = { ...(g.dAngle !== 0 ? { angleDeg: r5(g.dAngle) } : {}), ...(g.abd !== 0 ? { abductionDeg: g.abd } : {}) };
  const bdx = -(b.o[0] - BAG_O[0]);
  const bdy = b.o[1];
  if (Math.abs(bdx) > 1e-9) out["bag-slide-x"] = { dx: r5(bdx) };
  if (Math.abs(bdy) > 1e-9) out["bag-slide-y"] = { dy: r5(bdy) };
  if (b.dAngle !== 0) out["bag"] = { angleDeg: b.dAngle };
  if (b.lid !== 0) out["bag-lid"] = { angleDeg: b.lid };
  return out;
}
/** 🧵️ The string tension as the pair of loads on saddle and nut along the guitar axis of a placement. */
function tension(g: GuitarPlace): Json[] {
  const r = guitarR(g.dAngle, g.abd);
  const u = col(r, 0);
  const f = (k: number) => r5(TENSION_N * u[k]!);
  const z = Math.abs(u[2]) > 1e-9;
  return [
    { id: "string-tension-bridge", kind: "imposed", title: t("String tension pulling the bridge towards the head (six nylon strings at pitch, about 36 kgf)", "Saitenzug, der den Steg zum Kopf zieht (sechs gestimmte Nylonsaiten, etwa 36 kp)"), anchor: "saddle-load", fxN: f(0), fyN: f(1), ...(z ? { fzN: f(2) } : {}), source: "literature", reference: "french-2009" },
    { id: "string-tension-head", kind: "imposed", title: t("String tension pulling the head towards the bridge", "Saitenzug, der den Kopf zum Steg zieht"), anchor: "nut-load", fxN: r5(-TENSION_N * u[0]!), fyN: r5(-TENSION_N * u[1]!), ...(z ? { fzN: r5(-TENSION_N * u[2]!) } : {}), source: "literature", reference: "french-2009" },
  ];
}
const bind = (over: Partial<Record<string, Json>> = {}, guitarExposed = 1): Json => Object.fromEntries(REGIONS.map((r) => [r, over[r] ?? { exposedFraction: ["top", "back-ribs", "neck", "tuners"].includes(r) ? guitarExposed : 1, sunlitFraction: 0 }]));
const STORED_EXPOSED = { top: { exposedFraction: 0.75, sunlitFraction: 0 }, "back-ribs": { exposedFraction: 0.7, sunlitFraction: 0 } };
type PoseSpec = { id: string; title: Json; description: Json; key?: boolean; g: GuitarPlace; b: BagPlace; contacts: string[]; regions?: Json; loads?: Json[]; activity?: number };
function pose(p: PoseSpec): Json {
  return {
    id: p.id, title: p.title, description: p.description, ...(p.key ? { key: true } : {}),
    bones: offsets(p.g, p.b), ground: "auto", contacts: p.contacts,
    thermal: { regions: bind(p.regions ?? STORED_EXPOSED), emitters: {} },
    loads: [...tension(p.g), ...(p.loads ?? [])],
  };
}
function poseSpecs(): PoseSpec[] {
  const played = { ...PLAYED_G };
  const fretDir = col(guitarR(played.dAngle, played.abd), 1);
  const playedLoads = (fret = 10, abd = played.abd): Json[] => {
    const v = col(guitarR(played.dAngle, abd), 1);
    return [
      { id: "fretting", kind: "live", title: t("Left-hand fingers pressing the strings onto the fingerboard", "Finger der linken Hand drücken die Saiten aufs Griffbrett"), anchor: "fret-hand", fxN: r5(fret * v[0]!), fyN: r5(fret * v[1]!), fzN: r5(fret * v[2]!), source: "authored-estimate", reference: "fletcher-rossing-1998" },
      { id: "forearm", kind: "live", title: t("Weight of the right forearm resting on the lower bout", "Gewicht des rechten Unterarms auf dem Unterbug"), anchor: "forearm-rest", fxN: 0, fyN: -15, source: "authored-estimate", reference: "romanillos-1987" },
      { id: "pluck", kind: "live", title: t("Right-hand finger plucking a string towards the top", "Finger der rechten Hand schlägt eine Saite zur Decke hin an"), anchor: "pluck-point", fxN: r5(3 * v[0]!), fyN: r5(3 * v[1]!), fzN: r5(3 * v[2]!), source: "authored-estimate", reference: "fletcher-rossing-1998" },
    ];
  };
  void fretDir;
  const playRegions = { top: { exposedFraction: 0.75, sunlitFraction: 0, contact: { fraction: 0.04, tempC: 31 } }, "back-ribs": { exposedFraction: 0.65, sunlitFraction: 0, contact: { fraction: 0.12, tempC: 31 } }, neck: { exposedFraction: 1, sunlitFraction: 0, contact: { fraction: 0.12, tempC: 33.5 } } };
  const held = { neck: { exposedFraction: 1, sunlitFraction: 0, contact: { fraction: 0.08, tempC: 33.5 } }, ...STORED_EXPOSED };
  const bagged = { top: { exposedFraction: 0.2, sunlitFraction: 0 }, "back-ribs": { exposedFraction: 0.2, sunlitFraction: 0 }, neck: { exposedFraction: 0.2, sunlitFraction: 0 }, tuners: { exposedFraction: 0.2, sunlitFraction: 0 } };
  const LIFTED: GuitarPlace = { o: [0.06, 0.6], dAngle: 0, abd: 0 };
  const OVER: GuitarPlace = { o: [BAG_O[0] - G_IN_BAG[0], 0.42], dAngle: 75, abd: 0 };
  const OPEN: BagPlace = { ...STORED_B, lid: 105 };
  return [
    { id: "stored", title: t("Stored on its stand", "Auf dem Ständer gelagert"), description: t("As the used-instrument dealer offers it: the guitar leans 15 degrees back on the padded A-frame stand, its tail in the foam cradle; the empty gig bag lies zipped beside it on its backpack straps, the footstool stands set at 15 cm; strings tuned to pitch.", "Wie der Gebrauchtinstrumentenhandel sie anbietet: Die Gitarre lehnt 15 Grad zurück auf dem gepolsterten A-Ständer, der Boden in der Schaumauflage; der leere Gigbag liegt zugezogen daneben auf seinen Rucksackgurten, die Fußbank steht auf 15 cm eingestellt; Saiten gestimmt."), g: STORED_G, b: STORED_B, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR] },
    { id: "carried", title: t("Carried on the back in the gig bag", "Im Gigbag auf dem Rücken getragen"), description: t("The guitar zipped into its gig bag, carried on the back by the two shoulder straps: the bag hangs nearly upright, 8 degrees forward with the carrier, its tail at 0.8 m; stand and footstool stay behind in the shop.", "Die Gitarre im zugezogenen Gigbag an beiden Schultergurten auf dem Rücken getragen: Die Tasche hängt fast senkrecht, mit dem Träger 8 Grad vorgeneigt, der Boden auf 0,8 m; Ständer und Fußbank bleiben im Laden."), g: inBag(CARRIED_B), b: CARRIED_B, contacts: [...STAND_FEET, ...STOOL_FEET, "strap-shoulders", "strap-back"], regions: { ...bagged, "bag-shell": { exposedFraction: 0.92, sunlitFraction: 0, contact: { fraction: 0.08, tempC: 31 } } } },
    { id: "played", title: t("Played seated with the footstool", "Sitzend mit Fußbank gespielt"), description: t("Classical playing position of a seated guitarist (not part of the asset): the left foot on the footstool raises the left thigh, the waist of the guitar rests on it at 0.63 m, the neck rises 40 degrees to the left, the top tilted 15 degrees back; the lower bout lies against the right thigh, the back against the chest, the right forearm on the lower bout and the left hand on the fingerboard.", "Klassische Spielhaltung eines sitzenden Gitarristen (nicht Teil des Assets): Der linke Fuß auf der Fußbank hebt den linken Oberschenkel, die Taille der Gitarre liegt darauf auf 0,63 m, der Hals steigt 40 Grad nach links, die Decke 15 Grad zurückgeneigt; der Unterbug liegt am rechten Oberschenkel, der Boden an der Brust, der rechte Unterarm auf dem Unterbug und die linke Hand am Griffbrett."), g: played, b: STORED_B, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR, "thigh-left", "thigh-right", "chest"], regions: playRegions, loads: playedLoads() },
    { id: "played-reach", title: t("Played, reaching up the neck", "Gespielt, am Hals hinaufgreifend"), description: t("Key of the playing clip: the guitarist lifts the neck 3 degrees and shifts the left hand to the fifth position.", "Schlüssel des Spielclips: Der Gitarrist hebt den Hals um 3 Grad und versetzt die linke Hand in die fünfte Lage."), key: true, g: { ...played, abd: played.abd + 3 }, b: STORED_B, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR, "thigh-left", "thigh-right", "chest"], regions: playRegions, loads: playedLoads(12, played.abd + 3) },
    { id: "bag-opened", title: t("Gig bag opened", "Gigbag geöffnet"), description: t("Key of the carry clip: the zip is drawn round the tail and both sides, the front panel folds back over the head end.", "Schlüssel des Trageclips: Der Reißverschluss ist um den Boden und beide Seiten aufgezogen, das Vorderteil klappt über das Kopfende zurück."), key: true, g: STORED_G, b: OPEN, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR] },
    { id: "guitar-lifted", title: t("Guitar lifted off the stand", "Gitarre vom Ständer gehoben"), description: t("Key of the carry clip: two hands lift the guitar by the neck and the lower bout clear of the cradle and the backrest.", "Schlüssel des Trageclips: Zwei Hände heben die Gitarre an Hals und Unterbug aus der Auflage und von der Lehne."), key: true, g: LIFTED, b: OPEN, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR, "guitar-grip-neck", "guitar-grip-body"], regions: held },
    { id: "guitar-over-bag", title: t("Guitar turned face-up over the bag", "Gitarre mit der Decke nach oben über der Tasche"), description: t("Key of the carry clip: the guitar is turned to lie face-up and held over the open bag.", "Schlüssel des Trageclips: Die Gitarre wird mit der Decke nach oben gedreht und über die offene Tasche gehalten."), key: true, g: OVER, b: OPEN, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR, "guitar-grip-neck", "guitar-grip-body"], regions: held },
    { id: "guitar-bagged", title: t("Guitar laid into the open bag", "Gitarre in die offene Tasche gelegt"), description: t("Key of the carry clip: the guitar lies face-up on the padded back panel, the front panel still folded back.", "Schlüssel des Trageclips: Die Gitarre liegt mit der Decke nach oben auf dem gepolsterten Rückenteil, das Vorderteil noch zurückgeklappt."), key: true, g: inBag(OPEN), b: OPEN, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR] },
    { id: "bag-closed", title: t("Gig bag zipped with the guitar inside", "Gigbag mit der Gitarre zugezogen"), description: t("Key of the carry clip: the front panel is folded over the guitar and zipped.", "Schlüssel des Trageclips: Das Vorderteil ist über die Gitarre geklappt und zugezogen."), key: true, g: inBag(STORED_B), b: STORED_B, contacts: [...STAND_FEET, ...STOOL_FEET, ...BAG_FLOOR], regions: { ...bagged } },
    { id: "bag-tipped", title: t("Gig bag tipped up on its tail", "Gigbag auf den Boden aufgekippt"), description: t("Key of the carry clip: a hand at the head end tips the bag up on its tail edge before it is shouldered.", "Schlüssel des Trageclips: Eine Hand am Kopfende kippt die Tasche auf ihre Bodenkante, bevor sie geschultert wird."), key: true, g: inBag(TILTED_B), b: TILTED_B, contacts: [...STAND_FEET, ...STOOL_FEET, "bag-tail-edge", "bag-grip-head"], regions: { ...bagged } },
  ];
}
//#endregion

//#region assembly
const qualityAsset = { dimensions: { heightM: 1.05, widthM: 1.9, depthM: 0.6 } };
type Built = { parts: Part[]; meshes: Map<string, Mesh> };
function buildScale(s: Scale): Built {
  const parts = [...guitarParts(s), ...standParts(s), ...bagParts(s), ...footstoolParts(s)];
  const q = qualityOf(qualityAsset as never, s);
  const meshes = new Map<string, Mesh>();
  for (const p of parts) meshes.set(p.id, meshOf({ ...p.solid, material: p.material, role: p.role ?? "body", tone: p.tone }, FR[p.bone]!, q));
  return { parts, meshes };
}
const PENS: Record<Scale, Json> = { 50: { body: 0.35, detail: 0.18, texture: 0.1, glazing: null, ground: 0.5 }, 100: { body: 0.25, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 }, 200: { body: 0.2, detail: 0.1, texture: null, glazing: null, ground: 0.25 }, 500: { body: 0.2, detail: null, texture: null, glazing: null, ground: 0.2 } };
const RANGE: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
/** ✏️ The body shapes of one scale and view: each drawn group of parts projected and unioned, or the centre line of a slender group drawn as a stroke. */
function bodyShapes(s: Scale, b: Built, view: View): Json[] {
  const groups = new Map<string, Part[]>();
  for (const p of b.parts) {
    if (p.solid.texture || p.bone === "guitar") continue;
    const g = p.draw?.[view];
    if (g === false) continue;
    const key = `${p.bone}|${g ?? p.id}`;
    groups.set(key, [...(groups.get(key) ?? []), p]);
  }
  const eps = (0.05 * s) / 1000;
  const out: Json[] = [];
  let k = 0;
  for (const [key, ps] of groups) {
    k++;
    const id = key.split("|")[1]!;
    const meshes = ps.map((p) => b.meshes.get(p.id)!);
    const f = FR[ps[0]!.bone]!;
    const regions = [...new Set(ps.map((p) => p.region).filter((r): r is string => r !== null))];
    const region = ps[0]!.drawRegion?.[view] ?? (regions.length === 1 ? regions[0] : undefined);
    const bb = bounds(meshes);
    const near = Math.max(...meshes.flatMap((m) => m.world.map((p) => nearness(view, p))));
    const base: Json = { id, bone: ps[0]!.bone, role: ps[0]!.role ?? "body", z: r5(2 + near + k * 1e-4), filled: true, tone: ps[0]!.tone, ...(region ? { region } : regions.length > 1 ? { regions } : {}), material: ps[0]!.material };
    const kind = ps[0]!.stroke;
    if (kind !== undefined) {
      const pts: V2[] = [];
      for (const p of ps)
        for (const end of [p.solid.p0, p.solid.p1] as V3[]) {
          const w = toWorldOf(p.bone, end);
          const q = view === "elevation" ? (toLocal(f, w).slice(0, 2) as V2) : view === "front" ? ([w[2], w[1]] as V2) : ([w[0], w[2]] as V2);
          if (!pts.some((u) => Math.hypot(u[0] - q[0], u[1] - q[1]) < 1e-6)) pts.push(p2(q));
        }
      out.push({ ...base, filled: false, role: kind === "body" ? "body" : "texture", penMm: kind === "body" ? PENS[s].body : PENS[s].texture, kind: "poly", points: pts, closed: false });
      delete out[out.length - 1].material;
      continue;
    }
    const polys = outline(meshes, view, f, eps, 1e-8);
    if (polys.length === 0) continue;
    const shape: Json = { ...base, ...shapeGeometry(polys) };
    if (view === "elevation" && ps[0]!.bone === "guitar") shape.solid = { kind: "extruded", sweep: "w", z0: r5(bb.lo[2] - 0.0005), z1: r5(bb.hi[2] + 0.0005) };
    out.push(shape);
  }
  return out;
}
const toWorldOf = (boneId: string, p: V3): V3 => {
  const f = FR[boneId]!;
  return [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3;
};
/** 🎸️ The guitar drawn as placed solids in all three views (its bone spreads laterally in the playing pose, so no flat drawing would stay right): face-outline parts as their (u, w) profile swept along the bone normal, side-profile parts as their (u, v) outline swept laterally, the details as thin sweeps on the soundboard. */
function guitarShapes(s: Scale, b: Built, view: View): Json[] {
  const f = FR["guitar"]!;
  const out: Json[] = [];
  const eps = (0.05 * s) / 1000;
  const geo = (rings: V2[][]): Json => (rings.length === 1 ? { kind: "poly", points: rings[0]!.map(p2), closed: true } : { kind: "path", d: rings.map((r) => `M ${r.map(([x, y]) => `${r5(x)} ${r5(y)}`).join(" L ")} Z`).join(" ") });
  const swap = (ring: V2[]): V2[] => ring.map(([w, u]) => p2([u, w]));
  const coarse = s === 200 || s === 500;
  let k = 0;
  const push = (id: string, role: string, tone: number, filled: boolean, region: Json, material: string | undefined, rings: V2[][], solid: Json, near: number, pen?: number) => {
    k++;
    out.push({ id, bone: "guitar", role, z: r5(2 + near + k * 1e-4), filled, tone, ...region, ...(material ? { material } : {}), ...(pen !== undefined ? { penMm: pen } : {}), ...geo(rings), solid });
  };
  for (const p of b.parts.filter((q) => q.bone === "guitar")) {
    if (p.draw?.[view] === false && !p.solid.texture) continue;
    if (p.solid.texture && view === "elevation") continue;
    if (coarse && view === "elevation" && p.id === "top") continue;
    const m = b.meshes.get(p.id)!;
    const near = Math.max(...m.world.map((q) => nearness(view, q)));
    const region: Json = p.region === null ? {} : coarse && view === "elevation" && p.id === "ribs" ? { regions: ["top", "back-ribs"] } : { region: p.region };
    const role = p.role ?? "body";
    const sd = p.solid;
    const tr = sd.transform ?? {};
    const rotated = tr.rotation !== undefined && Math.abs(tr.rotation[3] - 1) > 1e-9;
    if (sd.kind === "extrude" && sd.plane === "zx" && !rotated) {
      const keepHoles = view !== "elevation" && p.id !== "ribs";
      const rings = [ccw(swap(sd.profile)), ...(keepHoles ? ((sd.holes ?? []) as V2[][]).map(swap) : [])];
      const z0 = coarse && view === "elevation" && p.id === "ribs" ? VT : sd.from;
      push(p.id, role, p.tone, !sd.texture, sd.texture ? {} : region, p.material, rings, { kind: "extruded", sweep: "v", z0: r5(z0), z1: r5(sd.to) }, near, sd.texture ? PENS[s].texture ?? PENS[s].detail : undefined);
      continue;
    }
    if (sd.kind === "box" && !rotated) {
      const [cu, cv, cw] = tr.translation as V3;
      const [su, sv, sw] = sd.size as V3;
      const rect: V2[] = ccw([[cu - su / 2, cw - sw / 2], [cu + su / 2, cw - sw / 2], [cu + su / 2, cw + sw / 2], [cu - su / 2, cw + sw / 2]]);
      push(p.id, role, p.tone, !sd.texture, sd.texture ? {} : region, p.material, [rect], { kind: "extruded", sweep: "v", z0: r5(cv - sv / 2), z1: r5(cv + sv / 2) }, near, sd.texture ? PENS[s].texture ?? PENS[s].detail : undefined);
      continue;
    }
    if (sd.kind === "capsule") {
      if (view === "elevation") continue;
      const [a, c] = [sd.p0 as V3, sd.p1 as V3];
      const d = [c[0] - a[0], c[2] - a[2]];
      const l = Math.hypot(d[0]!, d[1]!) || 1;
      const n = [(-d[1]! / l) * sd.r0, (d[0]! / l) * sd.r0];
      const quad: V2[] = ccw([[a[0] + n[0]!, a[2] + n[1]!], [c[0] + n[0]!, c[2] + n[1]!], [c[0] - n[0]!, c[2] - n[1]!], [a[0] - n[0]!, a[2] - n[1]!]]);
      push(p.id, "texture", p.tone, false, {}, undefined, [quad], { kind: "extruded", sweep: "v", z0: r5(Math.min(a[1], c[1]) - sd.r0), z1: r5(Math.max(a[1], c[1]) + sd.r0) }, near, PENS[s].texture ?? PENS[s].detail);
      continue;
    }
    const polys = outline([m], "elevation", f, eps, 1e-8);
    const loc = m.world.map((q) => toLocal(f, q));
    const wlo = Math.min(...loc.map((q) => q[2]));
    const whi = Math.max(...loc.map((q) => q[2]));
    const solid: Json = { kind: "extruded", sweep: "w", z0: r5(wlo), z1: r5(whi) };
    if (p.id === "neck-shaft" && s === 50) {
      const [u0, u1, h0, h1] = [0.56, U_NUT + 0.002, 0.03, 0.03 * 0.87];
      const slope = (h1 - h0) / (u1 - u0);
      const at = (u: number) => h0 + slope * (u - u0);
      Object.assign(solid, { z0: r5(-at(0)), z1: r5(at(0)), zTip: [r5(-at(0.99)), r5(at(0.99))] });
    }
    push(p.id, role, p.tone, true, region, p.material, polys.flat(), solid, near);
  }
  if (view === "elevation") return out;
  const sweepOn = (id: string, role: string, tone: number, filled: boolean, region: Json, rings: V2[][], v0: number, v1: number, pen?: number) => push(id, role, tone, filled, region, undefined, rings.map(ccw), { kind: "extruded", sweep: "v", z0: r5(v0), z1: r5(v1) }, 9, pen);
  const thin = (a: V2, c: V2, w: number): V2[] => {
    const d = [c[0] - a[0], c[1] - a[1]];
    const l = Math.hypot(d[0]!, d[1]!) || 1;
    const n = [(-d[1]! / l) * w, (d[0]! / l) * w];
    return [[a[0] + n[0]!, a[1] + n[1]!], [c[0] + n[0]!, c[1] + n[1]!], [c[0] - n[0]!, c[1] - n[1]!], [a[0] - n[0]!, a[1] - n[1]!]];
  };
  const SURF = [VT - 0.0004, VT - 0.0002] as const;
  const det = PENS[s].detail;
  const tex = PENS[s].texture ?? undefined;
  const circ = (r: number, n: number) => circle(0, HOLE_U, r, n).map(([w, u]) => [u, w] as V2);
  if (s !== 500) sweepOn("soundhole", "detail", 0.05, true, { region: "top" }, [circ(HOLE_R, s === 50 ? 32 : 20)], SURF[0], SURF[1], s === 200 ? 0.2 : undefined);
  if (s !== 500) sweepOn("rosette-outer", "detail", 0.3, false, {}, [circ(0.062, s === 50 ? 36 : 20)], SURF[0], SURF[1], det ?? undefined);
  if (s === 100 || s === 200) sweepOn("fingerboard", "detail", 0.18, true, { region: "neck" }, [[[U_NUT, -fbHalf(U_NUT)], [U_NUT, fbHalf(U_NUT)], [FB_END, fbHalf(FB_END)], [FB_END, -fbHalf(FB_END)]]], VT - FB_T - 0.0004, VT - FB_T - 0.0002);
  if (s === 50 && tex !== undefined) {
    sweepOn("rosette-inner", "texture", 0.3, false, {}, [circ(0.05, 32)], SURF[0], SURF[1], tex);
    sweepOn("rosette-mosaic", "texture", 0.3, false, {}, [circ(0.056, 32)], SURF[0], SURF[1], tex);
    const cracks: [number, number, number, number][] = [[0.3, 0.052, 0.36, 0.058], [-0.35, 0.05, -0.28, 0.06], [1.1, 0.051, 1.2, 0.059], [2.3, 0.052, 2.42, 0.061], [3.6, 0.05, 3.7, 0.06], [4.5, 0.053, 4.62, 0.06], [5.4, 0.05, 5.5, 0.058]];
    cracks.forEach(([a0, r0, a1, r1], i) => sweepOn(`rosette-craze-${i + 1}`, "texture", 0.45, false, {}, [thin([HOLE_U + r0 * Math.sin(a0), r0 * Math.cos(a0)], [HOLE_U + r1 * Math.sin(a1), r1 * Math.cos(a1)], 0.0003)], SURF[0], SURF[1], tex));
    for (let n = 1; n <= 19; n++) {
      const u = fretU(n);
      sweepOn(`fret-line-${n}`, "texture", 0.7, false, {}, [thin([u, -fbHalf(u)], [u, fbHalf(u)], 0.00125)], VT - FB_T - 0.0013, VT - FB_T - 0.0011, tex);
    }
    sweepOn("finger-wear", "detail", 0.95, true, { region: "top" }, [[[0.255, 0.004], [0.262, 0.06], [0.29, 0.085], [0.31, 0.072], [0.296, 0.04], [0.283, 0.006]]], VT - 0.0006, VT - 0.0005);
    sweepOn("nail-scratch-1", "texture", 0.55, false, {}, [thin([0.262, 0.02], [0.287, 0.05], 0.0003)], VT - 0.0008, VT - 0.0007, tex);
    sweepOn("nail-scratch-2", "texture", 0.55, false, {}, [thin([0.27, 0.012], [0.296, 0.045], 0.0003)], VT - 0.0008, VT - 0.0007, tex);
    [[1, 0.012, -0.012, 0.006], [2, 0.012, -0.016, 0.01], [3, 0.01, -0.008, 0.012]].forEach(([n, du, w0, w1]) => sweepOn(`fret-divot-${n}`, "texture", 0.55, false, {}, [thin([fretU(n!) + du!, w0!], [fretU(n!) + du!, w1!], 0.0006)], VT - FB_T - 0.0004, VT - FB_T - 0.0002, tex));
    sweepOn("tail-ding", "texture", 0.2, false, {}, [thin([0.006, -0.07], [0.012, -0.058], 0.0015)], SURF[0], SURF[1], tex);
    sweepOn("binding", "texture", 0.5, false, {}, [swap(inset(bodyRing(50), 0.0015)).filter((_, i) => i % 2 === 0)], SURF[0], SURF[1], tex);
  }
  return out;
}

/** 〰️ Hand-drawn detail strokes and patches of one scale: rosette, soundhole, frets, wear, seams and zips. */
function details(s: Scale): Record<View, Json[]> {
  const E: Json[] = [];
  const F: Json[] = [];
  const Pl: Json[] = [];
  const plan = (pts: V3[]) => pts.map((p) => p2([p[0], p[2]]));
  const stroke = (id: string, bone: string, pts: V2[], z: number, role = "detail", tone = 0.3, closed = false, extra: Json = {}) => ({ id, bone, role, z, filled: false, tone, kind: "poly", points: pts, closed, ...extra });
  if (s === 50 || s === 100) {
    const bag = (u: number, v: number, w: number) => toWorldOf("bag", [u, v, w]);
    const lidTop = BAG_V.lid - 0.0002;
    Pl.push(stroke("bag-zip", "bag", plan(bagRing(s).filter((_, i) => i % (s === 50 ? 1 : 2) === 0).map(([w, u]) => bag(u, lidTop, w * 0.97))), 4, "detail", 0.45, true));
    Pl.push(stroke("pocket-zip", "bag-lid", plan([bag(0.12, BAG_V.pocket, -0.1), bag(0.12, BAG_V.pocket, 0.1)]), 4.1, "detail", 0.45));
    if (s === 50) {
      Pl.push(stroke("bag-scuff", "bag", plan([bag(0.03, BAG_V.pocket, -0.08), bag(0.07, BAG_V.pocket, -0.02), bag(0.04, BAG_V.pocket, 0.03), bag(0.08, BAG_V.pocket, 0.09)]), 4.2, "texture", 0.5));
      Pl.push(stroke("bag-tear", "bag", plan([bag(0.012, lidTop, -0.11), bag(0.02, lidTop, -0.135), bag(0.03, lidTop, -0.15)]), 4.2, "texture", 0.7));
      Pl.push(stroke("pocket-seam", "bag-lid", plan(roundRect(0, 0.25, 0.22, 0.28, 0.035, 3).map(([w, u]) => bag(u, BAG_V.pocket, w))), 4.15, "texture", 0.35, true));
      E.push(stroke("gusset-zip", "bag", [[0.0, BAG_V.gusset1 + 0.006], [BAG_LEN, BAG_V.gusset1 + 0.006]].map(p2), 3, "texture", 0.45));
      F.push(stroke("stand-foam-tear", "stand", [[-0.06, BAR.y + 0.012], [-0.03, BAR.y + 0.006], [-0.01, BAR.y + 0.014]].map(p2), 2.2, "texture", 0.6));
      Pl.push(stroke("pad-ribs", "footstool", Array.from({ length: 9 }, (_, i) => FS.heel + 0.02 + i * 0.029).flatMap((x, i) => (i % 2 === 0 ? [[x, FS.z - 0.04], [x, FS.z + 0.04]] : [[x, FS.z + 0.04], [x, FS.z - 0.04]])).map((p) => p2(p as V2)), 4.3, "texture", 0.3));
    }
  }
  return { elevation: E, front: F, plan: Pl };
}
//#endregion

//#region structure lists
function structureList(s: Scale, b: Built): Json[] {
  const out: Json[] = [];
  const has = (id: string) => b.parts.some((p) => p.id === id);
  const ofm = (id: string, member: string, part = "member", extra: Json = {}) => { if (has(id)) out.push({ id: `st-${id}`, of: id, member, part, ...extra }); };
  const F = FEATURE[s];
  const pad = Math.max(F, 0.01);
  const supBox = (a: Json, size: V3) => {
    const w = toWorldOf(a.bone, [a.x, a.y, a.z ?? 0]);
    return { ...worldSolid({ id: `support-${a.id}`, kind: "box", bone: a.bone, size: p3(size), transform: { translation: p3([w[0], w[1] + size[1] / 2, w[2]]) } }, a.bone), material: "rubber", role: "ground", tone: 0.15, member: a.bone, part: "support", anchor: a.id };
  };
  for (const id of ["front-tube-l", "front-tube-r", "top-bar", "front-cross-bar", "arm-l", "arm-r", "arm-tip-l", "arm-tip-r", "top-foam", "arm-foam-l", "arm-foam-r", "cradle"]) ofm(id, "stand");
  for (const id of ["rear-tube-l", "rear-tube-r", "rear-cross-bar"]) ofm(id, "stand-rear");
  for (const id of ["hinge-bolt-l", "hinge-bolt-r"]) ofm(id, "stand-rear", "joint");
  if (!has("hinge-bolt-l")) for (const side of [-1, 1]) out.push({ id: `joint-hinge-${side < 0 ? "r" : "l"}`, kind: "ellipsoid", bone: "stand", material: "steel-powder", role: "detail", tone: 0.3, radii: p3([0.012, 0.012, Math.max(0.012, F / 2)].map((r) => Math.max(r, F / 2)) as V3), transform: { translation: p3(toLocal(FR["stand"]!, [HINGE[0], HINGE[1], side * (FRONT_Z + REAR_Z) / 2])) }, member: "stand-rear", part: "joint" });
  for (const a of ANCHORS.filter((x) => SUPPORTS().includes(x.id))) {
    if (a.id.startsWith("stand-foot")) {
      const foot = a.id.replace("stand-foot-", "foot-").replace("-left", "-l").replace("-right", "-r");
      if (has(foot)) out.push({ id: `support-${a.id}`, of: foot, member: a.bone, part: "support", anchor: a.id });
      else out.push(supBox(a, [Math.max(0.026, F), Math.max(0.012, F / 2), Math.max(0.024, F)]));
    } else if (a.id.startsWith("footstool-foot")) out.push(supBox(a, [Math.max(0.018, F), Math.max(0.009, F / 2), Math.max(0.02, F)]));
    else if (a.bone === "bag") out.push(supBox(a, [Math.max(0.04, F), Math.max(0.008, F / 2), Math.max(0.06, F)]));
    else out.push({ id: `support-${a.id}`, kind: "box", bone: a.bone, material: "rubber", role: "ground", tone: 0.15, size: p3([pad, pad, pad]), transform: { translation: p3([a.x, a.y, a.z ?? 0]) }, member: "guitar", part: "support", anchor: a.id });
  }
  for (const id of ["ribs", "top", "back", "heel", "neck-shaft", "fretboard", "head"]) ofm(id, "guitar");
  const gj = (id: string, worldP: V3, r: number) => out.push({ id, kind: "ellipsoid", bone: "guitar", material: "rosewood-laminate", role: "detail", tone: 0.3, radii: p3([r, r, r]), transform: { translation: p3(toLocal(FR["guitar"]!, worldP)) }, member: "guitar", part: "joint" });
  const jr = Math.max(0.008, F / 2 + 0.001);
  gj("joint-neck-block", toWorldOf("guitar", [BODY_LEN, 0, 0]), Math.max(jr, 0.012));
  for (const side of [-1, 1]) {
    const c = toWorldOf("guitar", [CRADLE_U, VB, side * ARM_Z]);
    gj(`joint-cradle-${side < 0 ? "r" : "l"}`, [c[0], c[1] - jr * 0.6, c[2]], jr);
  }
  const barC = toWorldOf("guitar", [BAR.u, VB, 0]);
  gj("joint-backrest", [barC[0] - jr * 0.5, barC[1], barC[2]], jr);
  for (const id of ["back-panel", "gusset", "bag-body"]) ofm(id, "bag");
  for (const id of ["lid-panel", "pocket"]) ofm(id, "bag-lid");
  out.push({ id: "joint-lid-hinge", kind: "capsule", bone: "bag", material: "padded-nylon", role: "detail", tone: 0.3, p0: p3([BAG_LEN - 0.02, BAG_V.lid + jr, -0.02]), p1: p3([BAG_LEN - 0.02, BAG_V.lid + jr, 0.02]), r0: jr, r1: jr, member: "bag-lid", part: "joint" });
  for (const id of ["plate", "rack", "footstool-block", "pad"]) ofm(id, "footstool", "member", id === "footstool-block" ? { members: ["footstool-front-leg", "footstool-rear-leg"] } : {});
  for (const id of ["front-leg-l", "front-leg-r", "front-leg-bar", "front-leg-shoe"]) ofm(id, "footstool-front-leg");
  for (const id of ["rear-leg-l", "rear-leg-r", "rear-leg-bar", "rear-leg-shoe"]) ofm(id, "footstool-rear-leg");
  for (const id of ["front-hinge-pin", "rear-hinge-pin"]) ofm(id, id.startsWith("front") ? "footstool-front-leg" : "footstool-rear-leg", "joint");
  if (s !== 50 && s !== 500) for (const [k, leg] of [["front", FRONT_LEG], ["rear", REAR_LEG]] as const) out.push({ id: `joint-${k}-hinge`, kind: "ellipsoid", bone: "footstool", material: "steel-black", role: "detail", tone: 0.3, radii: p3([jr, jr, jr]), transform: { translation: p3(toLocal(FR["footstool"]!, [leg.hinge[0], leg.hinge[1], FS.z])) }, member: `footstool-${k}-leg`, part: "joint" });
  const mr = Math.max(F / 2 + 0.0005, 0.006);
  for (const m of MASSES) {
    const f = FR[m.bone]!;
    const verts = b.parts.filter((p) => p.bone === m.bone && !p.solid.texture).flatMap((p) => b.meshes.get(p.id)!.world.map((w) => toLocal(f, w)));
    const split = ["stand", "stand-rear", "footstool-front-leg", "footstool-rear-leg"].includes(m.bone);
    const sides = split ? [-1, 1] : [0];
    for (const side of sides) {
      const pool = verts.filter((q) => side === 0 || Math.sign(q[2] - m.com[2]) === side);
      let best: V3 = pool[0] ?? m.com;
      let bd = Infinity;
      for (const q of pool) {
        const d = Math.hypot(q[0] - m.com[0], q[1] - m.com[1], side === 0 ? q[2] - m.com[2] : 0);
        if (d < bd) [bd, best] = [d, q];
      }
      const at: V3 = pool.length === 0 || (bd < 0.004 && side === 0) ? m.com : best;
      out.push({ id: `mass-${m.bone}${side === 0 ? "" : side < 0 ? "-r" : "-l"}`, kind: "ellipsoid", bone: m.bone, material: m.material, role: "detail", tone: 0.5, radii: p3([mr, mr, mr]), transform: { translation: p3(at) }, member: m.bone, part: "mass" });
    }
  }
  return out;
}
//#endregion

//#region masses
type MassRow = { bone: string; kg: number; com: V3; material: string; rows: [string, number][] };
let MASSES: MassRow[] = [];
const HIDDEN: Record<string, [string, number, V3][]> = {
  guitar: [["fan braces, harmonic bars, linings and blocks (spruce, cedar)", 0.19, [0.24, 0, 0]], ["finish, glue and inlays", 0.06, [0.4, 0, 0]], ["tuner gears and screws (brass, steel)", 0.08, [U_NUT + 0.1, 0.01, 0]]],
  bag: [["zips, sliders and lining", 0.12, [0.5, -0.07, 0]]],
  stand: [["rivets, end plugs and hinge washers", 0.03, [0.15, -0.3, 0]]],
};
function computeMasses(b: Built): MassRow[] {
  const rows = new Map<string, MassRow>();
  for (const p of b.parts) {
    if (p.solid.texture) continue;
    const st = meshStats(b.meshes.get(p.id)!);
    const kg = st.volume * DENSITY[p.material]! * (p.massFactor ?? 1);
    const f = FR[p.bone]!;
    const c = toLocal(f, st.centroid);
    const r = rows.get(p.bone) ?? { bone: p.bone, kg: 0, com: [0, 0, 0] as V3, material: p.material, rows: [] };
    r.com = [0, 1, 2].map((k) => (r.com[k]! * r.kg + c[k]! * kg) / (r.kg + kg || 1)) as V3;
    r.kg += kg;
    r.rows.push([p.id, kg]);
    rows.set(p.bone, r);
  }
  for (const [boneId, items] of Object.entries(HIDDEN)) {
    const r = rows.get(boneId)!;
    for (const [name, kg, c] of items) {
      r.com = [0, 1, 2].map((k) => (r.com[k]! * r.kg + c[k]! * kg) / (r.kg + kg)) as V3;
      r.kg += kg;
      r.rows.push([name, kg]);
    }
  }
  return [...rows.values()].map((r) => ({ ...r, kg: Math.round(r.kg * 1000) / 1000, com: p3(r.com) }));
}
//#endregion

//#region thermal
/** 📐️ The outer areas of the regions as measured on the 1:50 thermal list (flood from outside, the open soundhole lets the inside of the box count); `GUITAR_AREAS` overrides them for a re-measurement. */
const AREA: Record<string, number> = { top: 0.273, "back-ribs": 0.543, neck: 0.128, tuners: 0.024, "stand-frame": 0.165, "stand-padding": 0.094, "bag-shell": 0.924, "footstool-frame": 0.092, "footstool-pad": 0.058, ...JSON.parse(process.env["GUITAR_AREAS"] ?? "{}") };
function region(id: string, en: string, de: string, material: string, areaM2: number, L: number, geometry: string, layer: Json, extra: Json = {}): Json {
  return { id, title: t(en, de), kind: "surface", model: "passive", material, areaM2: r5(AREA[id] ?? areaM2), characteristicLengthM: L, geometry, skyViewFactor: 0.5, layers: [layer], heatCapacityJM2K: layer.heatCapacityJM2K, source: "authored-estimate", reference: "incropera-2018", ...extra };
}
const layer = (id: string, en: string, de: string, kind: string, material: string, th: number, k: number, rho: number, cp: number, source = "literature", reference = "wood-handbook-2010"): Json => ({ id, title: t(en, de), kind, material, thicknessM: th, resistanceM2KW: r6(th / k), heatCapacityJM2K: Math.round(rho * cp * th), source, reference });
const r6 = (x: number) => Math.round(x * 1e6) / 1e6;
function thermalModel(area: Record<string, number>): Json {
  const passive = { ratedPowerW: 0, ratedPowerBasis: "passive" };
  return {
    environment: { airTempC: 20, meanRadiantTempC: 20, skyTempC: 20, groundTempC: 19, relativeHumidity: 0.45, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 },
    operating: [], passengers: [],
    regions: [
      region("top", "Spruce soundboard with bridge, inside and out (worn shellac, finger wear at the soundhole)", "Fichtendecke mit Steg, innen und außen (abgespielte Schellackpolitur, Fingerspuren am Schallloch)", "spruce-top", area["top"] ?? 0.27, 0.36, "vertical-plate", layer("soundboard", "Spruce 2.5 mm", "Fichte 2,5 mm", "timber", "spruce-top", 0.0025, 0.11, 430, 1600), { ...passive }),
      region("back-ribs", "Back and ribs, inside and out (rosewood laminate, buckle rash on the back)", "Boden und Zargen, innen und außen (Palisander-Sperrholz, Gürtelschnallen-Kratzer am Boden)", "rosewood-laminate", area["back-ribs"] ?? 0.55, 0.36, "vertical-plate", layer("laminate", "Rosewood laminate 2.5 mm", "Palisander-Sperrholz 2,5 mm", "timber", "rosewood-laminate", 0.0025, 0.15, 650, 1500, "authored-estimate")),
      region("neck", "Neck, heel, head, fingerboard, nut and strings", "Hals, Halsfuß, Kopf, Griffbrett, Sattel und Saiten", "cedar-neck", area["neck"] ?? 0.12, 0.05, "cylinder", layer("neck-wood", "Cedar and rosewood, 10 mm to the core", "Zedrele und Palisander, 10 mm bis zum Kern", "timber", "cedar-neck", 0.01, 0.12, 470, 1600)),
      region("tuners", "Tuning machines: tarnished nickel-plated plates, rollers and buttons", "Mechaniken: angelaufene vernickelte Platten, Wellen und Knöpfe", "nickel-brass", area["tuners"] ?? 0.02, 0.02, "cylinder", layer("plating", "Nickel-plated brass 1.5 mm", "Vernickeltes Messing 1,5 mm", "metal", "nickel-brass", 0.0015, 110, 8500, 380, "literature", "incropera-2018")),
      region("stand-frame", "Stand tubes and hinge (black powder coat, chipped at the feet)", "Ständerrohre und Gelenk (schwarze Pulverbeschichtung, an den Füßen abgeplatzt)", "steel-powder", area["stand-frame"] ?? 0.12, 0.016, "cylinder", layer("tube-wall", "Steel tube wall 1 mm", "Stahlrohrwand 1 mm", "metal", "steel-powder", 0.001, 50, 7850, 460, "literature", "iso-10456-2007"), { ...passive }),
      region("stand-padding", "Foam sleeves and rubber feet of the stand", "Schaumhüllen und Gummifüße des Ständers", "pe-foam", area["stand-padding"] ?? 0.06, 0.03, "cylinder", layer("foam", "PE foam 6 to 10 mm", "PE-Schaum 6 bis 10 mm", "polymer", "pe-foam", 0.008, 0.04, 45, 2300, "literature", "iso-10456-2007")),
      region("bag-shell", "Gig bag shell, pocket, straps and handle (faded, scuffed at the tail)", "Gigbag-Hülle, Tasche, Gurte und Griff (ausgeblichen, am Boden abgescheuert)", "padded-nylon", area["bag-shell"] ?? 1.1, 0.4, "horizontal-plate-up", layer("padding", "Polyester shell over foam, 12 mm", "Polyesterhülle über Schaum, 12 mm", "insulation", "padded-nylon", 0.012, 0.045, 85, 1700, "authored-estimate", "iso-10456-2007"), { ...passive }),
      region("footstool-frame", "Footstool plate, rack and leg frames (black lacquer)", "Trittplatte, Raste und Beinbügel der Fußbank (schwarzer Lack)", "steel-black", area["footstool-frame"] ?? 0.12, 0.05, "horizontal-plate-up", layer("sheet", "Steel sheet and tube 1.5 mm", "Stahlblech und -rohr 1,5 mm", "metal", "steel-black", 0.0015, 50, 7850, 460, "literature", "iso-10456-2007"), { ...passive }),
      region("footstool-pad", "Ribbed rubber pad and rubber shoes of the footstool", "Gerippte Gummiauflage und Gummischuhe der Fußbank", "rubber", area["footstool-pad"] ?? 0.03, 0.09, "horizontal-plate-up", layer("rubber", "Rubber 3 mm", "Gummi 3 mm", "polymer", "rubber", 0.003, 0.17, 1200, 1900, "literature", "iso-10456-2007")),
    ],
    emitters: [],
    assumptions: [
      t("An acoustic instrument with its accessories: nothing draws power, every element is passive at 0 W (stated on the soundboard, the stand frame, the bag shell and the footstool frame). The shop is indoors at 20 degrees C and 45 % relative humidity, the humidity at which a guitar dealer keeps solid tops from cracking; there is no sun.", "Ein akustisches Instrument mit Zubehör: Nichts nimmt Leistung auf, jedes Element ist passiv mit 0 W (angegeben an der Decke, am Ständerrahmen, an der Taschenhülle und am Fußbankrahmen). Der Laden ist innen bei 20 Grad C und 45 % relativer Feuchte, bei der ein Gitarrenhändler massive Decken vor Rissen schützt; es gibt keine Sonne."),
      t("The guitar body is open through its 85 mm soundhole, so the inner faces of top, back and ribs belong to the area of their regions; the still air inside exchanges heat slowly, which the stored and played poses express as an exposed fraction of 0.65 to 0.75.", "Der Gitarrenkorpus ist durch sein 85-mm-Schallloch offen, die Innenflächen von Decke, Boden und Zargen gehören zur Fläche ihrer Regionen; die ruhende Luft im Inneren tauscht Wärme nur langsam, was die Lager- und Spielposen mit einem Austauschanteil von 0,65 bis 0,75 ausdrücken."),
      t("Played: the left hand warms the neck (12 % of its area at 33.5 degrees C), chest, thighs and forearm touch back, ribs and top through clothing (31 degrees C). Carried, the guitar sits in the still air of the closed bag (exposed fraction 0.2).", "Gespielt: Die linke Hand wärmt den Hals (12 % seiner Fläche bei 33,5 Grad C), Brust, Oberschenkel und Unterarm berühren Boden, Zargen und Decke durch die Kleidung (31 Grad C). Getragen sitzt die Gitarre in der ruhenden Luft der geschlossenen Tasche (Austauschanteil 0,2)."),
      t("Aged surfaces: worn shellac and the bare spruce under the finger wear absorb more light than new finish (0.52 against about 0.45), tarnished nickel plating emits 0.12 against 0.05 when polished, the faded bag fabric absorbs 0.85 against 0.92 new (authored estimates from the tables of Incropera et al.).", "Gealterte Oberflächen: Abgespielte Schellackpolitur und die blanke Fichte unter den Fingerspuren absorbieren mehr Licht als neuer Lack (0,52 gegen etwa 0,45), angelaufene Vernickelung emittiert 0,12 gegen 0,05 poliert, der ausgeblichene Taschenstoff absorbiert 0,85 gegen 0,92 neu (Schätzwerte nach den Tabellen von Incropera et al.)."),
      t("Tuner buttons and rollers are plastic in reality (emissivity about 0.9); they are merged into the nickel-plated tuner region.", "Mechanikknöpfe und -wellen sind in Wirklichkeit aus Kunststoff (Emissionsgrad etwa 0,9); sie sind in die vernickelte Mechanikregion eingerechnet."),
    ],
  };
}
//#endregion

//#region texts per scale
const REP_DECISIONS: Record<Scale, Json> = {
  50: {
    silhouette: t("Side elevation 37 mm long: the stand's A of 16 mm tubes with the foam backrest and cradle, the guitar seen on its rib leaning 15 degrees back, top and back plates on the rosewood ribs, the Spanish heel, the neck tapering to the nut and the slotted head angled 13 degrees back with the tuner buttons behind it; the gig bag lying flat on its straps with the pocket on top; the footstool's A of two leg frames under the channel plate. Front view: the face of the guitar with soundboard, rosette, soundhole, bridge, fingerboard, frets, six strings and the slotted head, the stand's frames behind it, the bag end-on and the footstool. Plan: the guitar from above with its head and tuners, the stand's feet and cradle, the bag's outline with zip and pocket, the footstool's ribbed pad.", "Seitenansicht 37 mm lang: das A des Ständers aus 16-mm-Rohren mit Schaumlehne und Auflage, die Gitarre auf der Zarge gesehen, 15 Grad zurückgelehnt, Decke und Boden auf den Palisanderzargen, der spanische Halsfuß, der zum Sattel schlanker werdende Hals und der 13 Grad zurückgeneigte Schlitzkopf mit den Mechanikknöpfen dahinter; der Gigbag flach auf seinen Gurten mit der Tasche oben; das A der beiden Beinbügel der Fußbank unter der Profilplatte. Vorderansicht: das Gesicht der Gitarre mit Decke, Rosette, Schallloch, Steg, Griffbrett, Bünden, sechs Saiten und Schlitzkopf, dahinter die Rahmen des Ständers, die Tasche stirnseitig und die Fußbank. Draufsicht: die Gitarre von oben mit Kopf und Mechaniken, Füße und Auflage des Ständers, Umriss der Tasche mit Reißverschluss und Tasche, die gerippte Auflage der Fußbank."),
    retained: t("Every part of the 1:50 model at its true size and place: top, ribs and back as plates, the heel, the D-shaped neck, fingerboard with 19 frets, bone nut and saddle, bridge with tie block, six strings, slotted head with tuner plates, rollers and buttons; all stand tubes, foam sleeves, rubber feet, hinge bolts and tip caps; bag panels, gusset, straps, handle and pocket; footstool channel plate, pad, rack, hinge pins, two leg frames and their shoes.", "Jedes Teil des 1:50-Modells in wahrer Größe und Lage: Decke, Zargen und Boden als Platten, der Halsfuß, der D-förmige Hals, Griffbrett mit 19 Bünden, Knochensattel und Stegeinlage, Steg mit Saitenhalter, sechs Saiten, Schlitzkopf mit Mechanikplatten, Wellen und Knöpfen; alle Ständerrohre, Schaumhüllen, Gummifüße, Gelenkbolzen und Spitzenkappen; Taschenteile, Seitenwand, Gurte, Griff und Außentasche; Profilplatte, Auflage, Raste, Gelenkstifte, zwei Beinbügel und ihre Schuhe der Fußbank."),
    merged: t("Nothing is merged at this scale; the three plates of the guitar body are drawn as one silhouette in the side elevation, where they are seen together.", "In diesem Maßstab ist nichts zusammengefasst; die drei Platten des Korpus sind in der Seitenansicht als ein Umriss gezeichnet, weil sie dort zusammen gesehen werden."),
    omitted: t("Braces, linings and the inner blocks are hidden inside the body and carried as mass only; screw heads and the zip teeth are below the drawing.", "Balken, Reifchen und die inneren Klötze sind im Korpus verborgen und nur als Masse geführt; Schraubenköpfe und die Reißverschlusszähne liegen unter der Zeichnung."),
    thinElements: t("Strings (0.7 to 1.1 mm) and frets are texture strokes in the front view; the 2.5 mm plates and the 3 mm saddle are filled shapes that the 0.35 mm body pen keeps printable.", "Saiten (0,7 bis 1,1 mm) und Bünde sind Texturstriche in der Vorderansicht; die 2,5-mm-Platten und die 3-mm-Stegeinlage sind gefüllte Formen, die die 0,35-mm-Körperfeder druckbar hält."),
    hatching: t("No hatches: materials read through tones, light spruce against dark rosewood and black foam, the soundhole nearly black.", "Keine Schraffuren: Die Materialien lesen sich über Töne, helle Fichte gegen dunkles Palisander und schwarzen Schaum, das Schallloch fast schwarz."),
    styles: t("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; tones from 0.05 (soundhole) to 0.95 (worn spruce).", "Körperfeder 0,35 mm, Detail 0,18 mm, Textur 0,1 mm; Töne von 0,05 (Schallloch) bis 0,95 (abgespielte Fichte)."),
    overlays: t("Traces of use: the pale finger wear and nail scratches on the treble side below the soundhole, crazing strokes across the rosette lacquer, divots under the first three frets, a ding at the tail, the torn stand foam, the scuffed tail and torn piping of the bag.", "Gebrauchsspuren: die helle Fingerabnutzung und Nagelkratzer auf der Diskantseite unter dem Schallloch, Craquelé-Striche über den Rosettenlack, Kerben unter den ersten drei Bünden, eine Delle am Boden, der eingerissene Ständerschaum, der abgescheuerte Boden und die eingerissene Paspel der Tasche."),
  },
  100: {
    silhouette: t("Side elevation 19 mm long: the stand's A with the foam backrest and cradle, the guitar on its rib with top, ribs and back, the neck with heel and fingerboard in one side profile, the slotted head with the tuner banks; the bag on its back with the pocket; the footstool's A. Front view: soundboard, rosette ring, soundhole, bridge and fingerboard of the guitar, the slotted head; the stand behind; the bag end-on; the footstool. Plan: guitar, stand, bag with zip and pocket, footstool.", "Seitenansicht 19 mm lang: das A des Ständers mit Schaumlehne und Auflage, die Gitarre auf der Zarge mit Decke, Zargen und Boden, der Hals mit Halsfuß und Griffbrett in einem Seitenprofil, der Schlitzkopf mit den Mechanikblöcken; die Tasche auf dem Rücken mit Außentasche; das A der Fußbank. Vorderansicht: Decke, Rosettenring, Schallloch, Steg und Griffbrett der Gitarre, der Schlitzkopf; dahinter der Ständer; die Tasche stirnseitig; die Fußbank. Draufsicht: Gitarre, Ständer, Tasche mit Reißverschluss und Außentasche, Fußbank."),
    retained: t("Body plates with soundhole, neck, fingerboard, slotted head, one tuner bank per side, the bridge; all stand tubes with foam and feet; bag panels and pocket; footstool plate, pad and legs.", "Korpusplatten mit Schallloch, Hals, Griffbrett, Schlitzkopf, je Seite ein Mechanikblock, der Steg; alle Ständerrohre mit Schaum und Füßen; Taschenteile und Außentasche; Platte, Auflage und Beine der Fußbank."),
    merged: t("Heel, shaft and nut are one side profile; plates, rollers and buttons of the tuners are one bank per side; tie block and wings are one bridge; the straps are part of the 20 mm back panel.", "Halsfuß, Hals und Sattel sind ein Seitenprofil; Platten, Wellen und Knöpfe der Mechanik sind je Seite ein Block; Saitenhalter und Flügel sind ein Steg; die Gurte gehören zum 20-mm-Rückenteil."),
    omitted: t("Strings, frets, saddle, hinge bolts, tip caps, the bag handle and the footstool rack are below 15 mm and are left out.", "Saiten, Bünde, Stegeinlage, Gelenkbolzen, Spitzenkappen, der Taschengriff und die Raste der Fußbank liegen unter 15 mm und entfallen."),
    thinElements: t("The plates of body and bag are declared sheets at their true 2.5 and 12 mm; the 16 mm tubes keep their section.", "Die Platten von Korpus und Tasche sind als dünne Platten in wahrer Stärke von 2,5 und 12 mm erklärt; die 16-mm-Rohre behalten ihren Querschnitt."),
    hatching: t("No hatches; tones carry the materials.", "Keine Schraffuren; die Töne tragen die Materialien."),
    styles: t("Body pen 0.25 mm, detail 0.13 mm, texture 0.1 mm.", "Körperfeder 0,25 mm, Detail 0,13 mm, Textur 0,1 mm."),
    overlays: t("The rosette ring and the dark soundhole remain; the bag shows its zip and the pocket zip.", "Rosettenring und dunkles Schallloch bleiben; die Tasche zeigt ihren Reißverschluss und den Taschenreißverschluss."),
  },
  200: {
    silhouette: t("Side elevation 9 mm long: the stand's A, the guitar body as one block with its soundboard, the neck profile and the head with its tuner band; the bag as one slab with its front panel and pocket; the footstool as plate on two legs. Front view: the guitar face with the soundhole, the stand behind, the bag end-on, the footstool. Plan: the four objects from above.", "Seitenansicht 9 mm lang: das A des Ständers, der Gitarrenkorpus als ein Block mit seiner Decke, das Halsprofil und der Kopf mit seinem Mechanikband; die Tasche als eine Platte mit Vorderteil und Außentasche; die Fußbank als Platte auf zwei Beinen. Vorderansicht: das Gesicht der Gitarre mit Schallloch, dahinter der Ständer, die Tasche stirnseitig, die Fußbank. Draufsicht: die vier Gegenstände von oben."),
    retained: t("Soundboard, body, neck, head, tuner band; stand tubes, foam sleeves; bag body, front panel, pocket; footstool plate, pad, legs.", "Decke, Korpus, Hals, Kopf, Mechanikband; Ständerrohre, Schaumhüllen; Taschenkörper, Vorderteil, Außentasche; Platte, Auflage und Beine der Fußbank."),
    merged: t("Ribs and back are one solid body behind the soundboard; head slots, tuners and buttons are one band across the head; the bag is one padded block; the footstool plate is one 14.5 mm slab.", "Zargen und Boden sind ein Vollkörper hinter der Decke; Kopfschlitze, Mechaniken und Knöpfe sind ein Band quer über den Kopf; die Tasche ist ein gepolsterter Block; die Trittplatte ist eine 14,5-mm-Platte."),
    omitted: t("Bridge, frets, strings and the feet of stand and footstool are below 30 mm and left out; the rosette is not drawn.", "Steg, Bünde, Saiten und die Füße von Ständer und Fußbank liegen unter 30 mm und entfallen; die Rosette ist nicht gezeichnet."),
    thinElements: t("The 16 mm stand tubes and the 10 mm footstool legs are drawn as filled strips at their true width.", "Die 16-mm-Ständerrohre und die 10-mm-Fußbankbeine sind als gefüllte Streifen in wahrer Breite gezeichnet."),
    hatching: t("No hatches.", "Keine Schraffuren."),
    styles: t("Body pen 0.2 mm, detail 0.1 mm; the soundhole carries the body pen so that it prints where the bag covers it in the carried pose.", "Körperfeder 0,2 mm, Detail 0,1 mm; das Schallloch trägt die Körperfeder, damit es auch dort druckt, wo die Tasche es in der Tragepose verdeckt."),
    overlays: t("Only the dark soundhole and the rosette ring are kept as marks of the instrument.", "Nur das dunkle Schallloch und der Rosettenring bleiben als Zeichen des Instruments."),
  },
  500: {
    silhouette: t("Side elevation under 4 mm: the four stand legs as lines with the backrest and cradle blocks, the guitar as body, neck and head, the bag as one slab with its front panel, the footstool as one block. Front and plan: the same masses.", "Seitenansicht unter 4 mm: die vier Ständerbeine als Linien mit Lehnen- und Auflageblock, die Gitarre als Korpus, Hals und Kopf, die Tasche als eine Platte mit Vorderteil, die Fußbank als ein Block. Vorder- und Draufsicht: dieselben Massen."),
    retained: t("Guitar body with soundboard, neck, head, tuner band; four stand legs, backrest, cradle; bag body and front panel; footstool block with pad.", "Gitarrenkorpus mit Decke, Hals, Kopf, Mechanikband; vier Ständerbeine, Lehne, Auflage; Taschenkörper und Vorderteil; Fußbankblock mit Auflage."),
    merged: t("Cross bars, arms and their foam are one cradle block, the backrest tube and its foam one roll; plate, legs and shoes of the footstool are one block.", "Querrohre, Arme und ihr Schaum sind ein Auflageblock, Lehnenrohr und Schaum eine Rolle; Platte, Beine und Schuhe der Fußbank sind ein Block."),
    omitted: t("The soundhole, the pocket of the bag, the feet and every detail below 75 mm are left out.", "Das Schallloch, die Außentasche der Tasche, die Füße und jedes Detail unter 75 mm entfallen."),
    thinElements: t("The stand legs are slender tubes drawn as 0.2 mm lines; the soundboard and the bag's front panel are declared sheets.", "Die Ständerbeine sind schlanke Rohre, als 0,2-mm-Linien gezeichnet; die Decke und das Vorderteil der Tasche sind als dünne Platten erklärt."),
    hatching: t("No hatches.", "Keine Schraffuren."),
    styles: t("One body pen of 0.2 mm so that the 22 mm neck still prints at 1:1000.", "Eine Körperfeder von 0,2 mm, damit der 22-mm-Hals bei 1:1000 noch druckt."),
    overlays: t("None.", "Keine."),
  },
};
const VOL_DECISIONS: Record<Scale, Json[]> = {
  50: [
    t("Body: the guitar built as the real hollow box: 2.5 mm spruce top with the 85 mm soundhole, 2 mm rosewood ribs, 2.5 mm back, the Spanish heel, a D-section cedar neck tapering from 60 to 52 mm, the 6 mm rosewood fingerboard to the soundhole, 19 fret wires, bone nut and saddle, the bridge with tie block, six strings from the tie block over saddle and nut to the rollers, and the slotted head 13 degrees back with two plates, three rollers and six buttons.", "Körper: die Gitarre als echter Hohlkasten: 2,5-mm-Fichtendecke mit 85-mm-Schallloch, 2-mm-Palisanderzargen, 2,5-mm-Boden, spanischer Halsfuß, D-förmiger Zedrelenhals von 60 auf 52 mm, 6-mm-Palisandergriffbrett bis zum Schallloch, 19 Bundstäbchen, Knochensattel und Stegeinlage, Steg mit Saitenhalter, sechs Saiten vom Saitenhalter über Stegeinlage und Sattel zu den Wellen, und der Schlitzkopf 13 Grad zurück mit zwei Platten, drei Wellen und sechs Knöpfen."),
    t("Stand, bag and footstool: every 16 mm tube as a capsule, foam sleeves as closed-loop revolves round them, rubber feet; the bag as back panel, gusset ring, front panel on its own hinge bone, pocket, two straps and the side handle; the footstool's channel plate, rubber pad, rack, hinge pins and two U leg frames with rubber shoes.", "Ständer, Tasche und Fußbank: jedes 16-mm-Rohr als Kapsel, Schaumhüllen als geschlossene Drehprofile darum, Gummifüße; die Tasche als Rückenteil, Seitenwandring, Vorderteil an eigenem Gelenkknochen, Außentasche, zwei Gurte und Seitengriff; Profilplatte, Gummiauflage, Raste, Gelenkstifte und zwei U-Beinbügel mit Gummischuhen der Fußbank."),
    t("Texture: the fret wires, the rosette ring, the finger-wear patch on the top and the scuffed patch on the bag's tail are texture solids, as their 2D strokes and patches are.", "Textur: Bundstäbchen, Rosettenring, Fingerabnutzung auf der Decke und die abgescheuerte Stelle am Taschenboden sind Texturkörper, wie ihre 2D-Striche und -Flächen."),
    t("Thermal: every region is the skin of its body solids by reference; the open soundhole lets the inner faces of the box count.", "Thermik: Jede Region ist die Haut ihrer Körper als Verweis; das offene Schallloch lässt die Innenflächen des Kastens mitzählen."),
    t("Structure: tubes, plates, neck and panels are their own members; joints at the hinges, the neck block, the cradle and backrest contacts and the lid hinge; a support at every support anchor; a mass marker per bone at its centre of mass.", "Tragwerk: Rohre, Platten, Hals und Taschenteile sind selbst die Stäbe; Knoten an den Gelenken, am Halsklotz, an den Kontakten von Auflage und Lehne und am Deckelgelenk; ein Lager an jedem Lageranker; je Knochen eine Massenmarke im Schwerpunkt."),
  ],
  100: [
    t("Body: the guitar box stays hollow with its soundhole; neck, heel and nut are one side profile 60 mm wide, the fingerboard its own plate on it; the slotted head keeps its 17 mm slots; each tuner side is one bank of plate, gears and buttons; the bridge is one block.", "Körper: Der Gitarrenkasten bleibt hohl mit Schallloch; Hals, Halsfuß und Sattel sind ein Seitenprofil von 60 mm Breite, das Griffbrett eine eigene Platte darauf; der Schlitzkopf behält seine 17-mm-Schlitze; jede Mechanikseite ist ein Block aus Platte, Getriebe und Knöpfen; der Steg ist ein Block."),
    t("Stand: tubes, foam sleeves and simplified feet; the hinge bolts and tip caps are merged into the tubes. Bag: back panel 20 mm with the straps, gusset ring, front panel, pocket. Footstool: channel plate, pad, two leg frames with shoes.", "Ständer: Rohre, Schaumhüllen und vereinfachte Füße; Gelenkbolzen und Spitzenkappen gehen in den Rohren auf. Tasche: 20-mm-Rückenteil mit den Gurten, Seitenwandring, Vorderteil, Außentasche. Fußbank: Profilplatte, Auflage, zwei Beinbügel mit Schuhen."),
    t("Thermal and structure as at 1:50, on the solids of this scale.", "Thermik und Tragwerk wie bei 1:50, auf den Körpern dieses Maßstabs."),
  ],
  200: [
    t("Body: the soundboard as a declared sheet on a solid body of ribs and back (the cavity is not resolved at this scale); neck and head as side profiles 60 and 75 mm wide; one tuner band across the head; stand tubes at their true 16 mm; the bag one padded block with its front panel and pocket; the footstool plate a 14.5 mm slab on 10 mm legs.", "Körper: die Decke als erklärte dünne Platte auf einem Vollkörper aus Zargen und Boden (der Hohlraum wird in diesem Maßstab nicht aufgelöst); Hals und Kopf als Seitenprofile von 60 und 75 mm Breite; ein Mechanikband quer über den Kopf; Ständerrohre in wahren 16 mm; die Tasche ein gepolsterter Block mit Vorderteil und Außentasche; die Trittplatte eine 14,5-mm-Platte auf 10-mm-Beinen."),
    t("Thermal: references to the body solids; only the sum of the areas is compared at this scale. Structure: members by reference, joints and supports at the feature size.", "Thermik: Verweise auf die Körper; in diesem Maßstab wird nur die Flächensumme verglichen. Tragwerk: Stäbe als Verweise, Knoten und Lager in Merkmalgröße."),
  ],
  500: [
    t("Body: four slender 16 mm legs, the backrest roll and the cradle block of the stand; the guitar as soundboard sheet on the body, neck and head profiles and the tuner band; the bag as one block with its front panel; the footstool as one block with its pad.", "Körper: vier schlanke 16-mm-Beine, die Lehnenrolle und der Auflageblock des Ständers; die Gitarre als Deckenplatte auf dem Korpus, Hals- und Kopfprofil und Mechanikband; die Tasche als ein Block mit Vorderteil; die Fußbank als ein Block mit Auflage."),
    t("Structure: the footstool block is the member of the plate and both leg bones (members); supports and joints at the feature size.", "Tragwerk: Der Fußbankblock ist der Stab der Platte und beider Beinknochen (members); Lager und Knoten in Merkmalgröße."),
  ],
};
//#endregion

//#region main
function main(): void {
  FR = frames(RIG);
  ANCHORS = buildAnchors();
  const built = Object.fromEntries(SCALES.map((s) => [s, buildScale(s)])) as Record<Scale, Built>;
  MASSES = computeMasses(built[50]);
  for (const b of BONES) {
    const m = MASSES.find((x) => x.bone === b.id);
    if (m) b.mass = { kg: m.kg, comX: r5(m.com[0]), comY: r5(m.com[1]), distribution: "point", source: "authored-estimate", reference: b.id.startsWith("guitar") ? "romanillos-1987" : "iso-10456-2007" };
  }
  const massKg = Math.round(MASSES.reduce((a, m) => a + m.kg, 0) * 1000) / 1000;
  const bb = bounds([...built[50].parts.filter((p) => !p.solid.texture).map((p) => built[50].meshes.get(p.id)!)]);
  const dims = { heightM: r5(bb.hi[1]), widthM: r5(bb.hi[0] - bb.lo[0]), depthM: r5(bb.hi[2] - bb.lo[2]), massKg };
  const representations: Json = {};
  const volumes: Json = {};
  for (const s of SCALES) {
    const b = built[s];
    const d = details(s);
    const geometry = Object.fromEntries((["elevation", "front", "plan"] as View[]).map((v) => [v, [...bodyShapes(s, b, v), ...guitarShapes(s, b, v), ...d[v]]]));
    representations[String(s)] = { status: "authored", denominator: s, validRange: { minDenominator: RANGE[s][0], maxDenominator: RANGE[s][1] }, pens: PENS[s], hatches: [], geometry, decisions: REP_DECISIONS[s] };
    const body = b.parts.map((p) => ({ ...p.solid, material: p.material, role: p.role ?? "body", tone: p.tone }));
    const thermal = b.parts.filter((p) => !p.solid.texture && p.region !== null).map((p) => ({ id: `th-${p.id}`, of: p.id, region: p.region }));
    volumes[String(s)] = { status: "authored", denominator: s, body, thermal, structure: structureList(s, b), decisions: VOL_DECISIONS[s] };
  }
  const specs = poseSpecs();
  const poses = specs.map(pose);
  const asset = {
    id: "used-classical-guitar",
    category: "objects",
    title: t("Used Classical Guitar", "Gebrauchte Konzertgitarre"),
    summary: t(`A second-hand classical guitar of the standard student pattern, 0.99 m long with a 650 mm scale, spruce top, rosewood-laminate back and ribs and a slotted head, as a used-instrument dealer offers it: cleaned, restrung and tuned, leaning 15 degrees back on a folding A-frame stand, its worn gig bag lying beside it and the guitarist's footstool set at 15 cm. Finger wear and nail scratches below the soundhole, crazed lacquer over the rosette, divots under the first frets, a ding at the tail, torn stand foam and a scuffed bag tail are what its years of practice left. Stored, carried on the back in the bag and played seated with the left foot on the footstool, with a carry clip from the stand into the bag onto the back and a playing clip; ${massKg.toFixed(2)} kg in all, four individually drawn and modelled scales from 1:500 to 1:50.`, `Eine gebrauchte Konzertgitarre des üblichen Schülermodells, 0,99 m lang mit 650 mm Mensur, Fichtendecke, Boden und Zargen aus Palisander-Sperrholz und Schlitzkopf, wie sie ein Gebrauchtinstrumentenhandel anbietet: gereinigt, neu besaitet und gestimmt, 15 Grad zurückgelehnt auf einem klappbaren A-Ständer, daneben ihr abgenutzter Gigbag und die auf 15 cm eingestellte Gitarrenfußbank. Fingerabnutzung und Nagelkratzer unter dem Schallloch, Craquelé im Lack über der Rosette, Kerben unter den ersten Bünden, eine Delle am Boden, eingerissener Ständerschaum und ein abgescheuerter Taschenboden sind, was die Übungsjahre hinterlassen haben. Gelagert, im Gigbag auf dem Rücken getragen und sitzend mit dem linken Fuß auf der Fußbank gespielt, mit einem Trageclip vom Ständer in die Tasche auf den Rücken und einem Spielclip; insgesamt ${massKg.toFixed(2).replace(".", ",")} kg, vier einzeln gezeichnete und modellierte Maßstäbe von 1:500 bis 1:50.`),
    tags: ["used", "instrument", "music", "second-hand", "strings", "guitar", "classical-guitar", "nylon-strings", "spruce", "rosewood", "gig-bag", "guitar-stand", "footstool", "chordophone"],
    provenance: {
      author: "Semio staffage content (used instruments wave)",
      method: "handcrafted",
      created: "2026-11-22",
      references: [
        { id: "hornbostel-sachs-1914", citation: "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46(4-5), 553-590: class 321.322 composite chordophone, necked box lute (guitar), sounded by the fingers (-5)." },
        { id: "mimo-2011", citation: "MIMO Consortium (2011). Revision of the Hornbostel-Sachs Classification of Musical Instruments, class 321.322-5 (necked box lutes, plucked with the fingers)." },
        { id: "romanillos-1987", citation: "Romanillos, J. L. (1987). Antonio de Torres, Guitar Maker: His Life and Work. Element Books: the Torres pattern of the modern classical guitar, 650 mm scale, body about 485 mm long, bouts about 280 and 365 mm, rib depth about 95 mm." },
        { id: "courtnall-1993", citation: "Courtnall, R. (1993). Making Master Guitars. Robert Hale: classical guitar plans, neck 52 mm at the nut and 62 mm at the 12th fret, slotted head about 190 x 75 mm, head angle 12 to 15 degrees, fingerboard 6 mm." },
        { id: "french-2009", citation: "French, R. M. (2009). Engineering the Guitar: Theory and Practice. Springer, chapters 3 and 7: string tensions of nylon sets (about 340 to 380 N total for normal to hard tension) and instrument masses (classical guitar about 1.4 to 1.7 kg)." },
        { id: "fletcher-rossing-1998", citation: "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed. Springer, chapter 9 (the guitar): plucking forces of a few newtons, body and air modes." },
        { id: "wood-handbook-2010", citation: "Forest Products Laboratory (2010). Wood Handbook: Wood as an Engineering Material, General Technical Report FPL-GTR-190, USDA: densities, conductivities, specific heats and moduli of spruce, cedar and rosewood at 12 % moisture." },
        { id: "iso-10456-2007", citation: "ISO 10456:2007 Building materials and products, Hygrothermal properties, Tabulated design values (steel, polyethylene foam, rubber, textiles)." },
        { id: "incropera-2018", citation: "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: emissivities of nickel (polished 0.05, oxidised 0.1 to 0.4), wood, rubber and painted metal." },
        { id: "catalogue-accessories", citation: "Catalogue-typical data of guitar accessories as sold by music retailers: folding A-frame guitar stand of 16 mm steel tube with foam padding, about 0.45 to 0.5 m high and 0.9 to 1.3 kg; padded 4/4 classical gig bag about 1.05 x 0.41 x 0.14 m with 10 to 15 mm padding, backpack straps and front pocket, 1.0 to 1.4 kg; adjustable folding guitarist's footstool, plate about 270 x 95 mm, heights 12 to 23 cm in six notches, 0.5 to 0.8 kg." },
      ],
      notes: t(`World frame of the stored pose: x ahead (the face of the guitar looks ahead), y up, z to the left. The guitar is a standard student classical guitar of the Torres pattern as DDR and BRD workshops built it in series (no maker, no label). Its tail rests in the foam cradle of the stand at 0.07 m and its back leans on the foam backrest at ${r5(BAR.y).toFixed(3)} m; the empty bag lies behind the stand with its tail against the rear cross bar; the footstool stands to the left where the left foot will go. Masses are section by density of the 1:50 solids plus braces, linings, blocks, finish and tuner gears listed in the ticket report: guitar ${MASSES.find((m) => m.bone === "guitar")!.kg.toFixed(2)} kg. String tension 360 N acts as a pair of loads on saddle and nut. The seated guitarist is not part of the asset: his contacts are support anchors on the guitar and his hands are loads.`, `Weltrahmen der Lagerpose: x nach vorn (das Gesicht der Gitarre schaut nach vorn), y nach oben, z nach links. Die Gitarre ist eine übliche Schüler-Konzertgitarre nach Torres-Muster, wie DDR- und BRD-Werkstätten sie in Serie bauten (kein Hersteller, kein Etikett). Ihr Boden ruht in der Schaumauflage des Ständers auf 0,07 m, ihr Rücken lehnt an der Schaumlehne auf ${r5(BAR.y).toFixed(3).replace(".", ",")} m; die leere Tasche liegt hinter dem Ständer mit dem Boden am hinteren Querrohr; die Fußbank steht links, wo der linke Fuß hingehört. Massen sind Querschnitt mal Dichte der 1:50-Körper plus Balken, Reifchen, Klötze, Lack und Mechanikgetriebe aus dem Ticketbericht: Gitarre ${MASSES.find((m) => m.bone === "guitar")!.kg.toFixed(2).replace(".", ",")} kg. Der Saitenzug von 360 N wirkt als Lastpaar auf Stegeinlage und Sattel. Der sitzende Gitarrist ist nicht Teil des Assets: Seine Kontakte sind Lageranker an der Gitarre, seine Hände Lasten.`),
    },
    dimensions: dims,
    materials: MAT,
    rig: RIG,
    anchors: ANCHORS,
    representations,
    symbols: [],
    poses,
    clips: clips(),
    thermal: thermalModel(AREA),
    structure: {
      gravityMS2: 9.80665, dynamics: "quasi-static", supports: SUPPORTS(),
      assumptions: [
        t("All four objects are one rigid body on the active supports of each pose (elastic springs of equal stiffness): stored, the stand and footstool feet and the bag on its straps; carried, the shoulder straps and the lower bag end on the back replace the floor under the bag, and the stand and footstool feet stay; played, the left and right thighs and the chest carry the guitar.", "Alle vier Gegenstände sind ein starrer Körper auf den aktiven Lagern jeder Pose (elastische Federn gleicher Steifigkeit): gelagert die Füße von Ständer und Fußbank und die Tasche auf ihren Gurten; getragen ersetzen Schultergurte und das untere Taschenende am Rücken den Boden unter der Tasche, die Füße von Ständer und Fußbank bleiben; gespielt tragen linker und rechter Oberschenkel und die Brust die Gitarre."),
        t("The string tension of 360 N (six nylon strings at pitch, catalogue-typical normal to hard tension, French 2009) is a self-balancing pair of loads on saddle and nut: it compresses the neck and bends the top behind the bridge. The 0.7 kN of a steel-string guitar would be twice that.", "Der Saitenzug von 360 N (sechs gestimmte Nylonsaiten, katalogtypische normale bis harte Spannung, French 2009) ist ein selbstausgleichendes Lastpaar auf Stegeinlage und Sattel: Er drückt den Hals und biegt die Decke hinter dem Steg. Die 0,7 kN einer Stahlsaitengitarre wären das Doppelte."),
        t("The rig joint of the guitar sits at its tail in the cradle, so the cradle carries the weight and the moment that the foam backrest takes in reality; the joints at the cradle and the backrest contacts show where the load passes.", "Das Riggelenk der Gitarre sitzt am Boden in der Auflage, die Auflage trägt also Gewicht und Moment, die in Wirklichkeit die Schaumlehne aufnimmt; die Knoten an den Kontakten von Auflage und Lehne zeigen, wo die Last übergeht."),
      ],
      clinical: false,
    },
    volumes,
    presentation: { defaultPose: "stored", defaultClip: "carry", defaultScale: 50 },
    exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    completion: "authored",
  };
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  writeFileSync(join(HERE, "🔣️.json"), JSON.stringify(doc, null, 1) + "\n");
  const report = { dims, masses: MASSES.map((m) => ({ bone: m.bone, kg: m.kg, com: m.com, rows: m.rows.map(([n, kg]) => [n, Math.round(kg * 10000) / 10000]) })), bar: BAR, cradleU: CRADLE_U, armY: ARM_Y, footstoolZ: FS.z, playedO: PLAYED_O, counts: Object.fromEntries(SCALES.map((s) => [s, { body: volumes[String(s)].body.length, thermal: volumes[String(s)].thermal.length, structure: volumes[String(s)].structure.length, shapes: Object.fromEntries(Object.entries(representations[String(s)].geometry).map(([k, v]) => [k, (v as Json[]).length])) }])) };
  mkdirSync(join(HERE, "🗑️generated"), { recursive: true });
  writeFileSync(join(HERE, "🗑️generated", "author.json"), JSON.stringify(report, null, 1));
  console.log(JSON.stringify({ dims, guitarKg: MASSES.find((m) => m.bone === "guitar")!.kg, bar: BAR, armY: ARM_Y, fsz: FS.z, counts: report.counts }));
}

function clips(): Json[] {
  const all = [...STAND_FEET, ...STOOL_FEET];
  const span = (anchors: string[], fromS: number, toS: number) => anchors.map((anchor) => ({ anchor, fromS, toS }));
  return [
    {
      id: "carry", title: t("From the stand into the bag onto the back", "Vom Ständer in die Tasche auf den Rücken"),
      description: t("The bag is unzipped and its front panel folded back, the guitar lifted off the stand, turned face-up and laid into the bag, the panel closed, the bag tipped up on its tail and shouldered.", "Die Tasche wird aufgezogen und ihr Vorderteil zurückgeklappt, die Gitarre vom Ständer gehoben, mit der Decke nach oben gedreht und in die Tasche gelegt, das Vorderteil geschlossen, die Tasche auf den Boden gekippt und geschultert."),
      durationS: 10.5, fps: 10, loop: false, rootMotion: "none",
      keys: [{ t: 0, pose: "stored", ease: "smooth" }, { t: 1.5, pose: "bag-opened", ease: "smooth" }, { t: 3, pose: "guitar-lifted", ease: "smooth" }, { t: 4.5, pose: "guitar-over-bag", ease: "smooth" }, { t: 6, pose: "guitar-bagged", ease: "smooth" }, { t: 7.5, pose: "bag-closed", ease: "smooth" }, { t: 9, pose: "bag-tipped", ease: "smooth" }, { t: 10.5, pose: "carried", ease: "smooth" }],
      contacts: [...span(all, 0, 10.5), ...span(BAG_FLOOR, 0, 7.5), ...span(["guitar-grip-neck", "guitar-grip-body"], 2, 5.5), ...span(["bag-tail-edge", "bag-grip-head"], 7.5, 9.6), ...span(["strap-shoulders", "strap-back"], 9.6, 10.5)],
      phases: [{ id: "open", fromS: 0, toS: 1.5, title: t("Opening the bag", "Tasche öffnen") }, { id: "lift", fromS: 1.5, toS: 4.5, title: t("Lifting the guitar off the stand", "Gitarre vom Ständer heben") }, { id: "pack", fromS: 4.5, toS: 7.5, title: t("Packing and zipping", "Einpacken und zuziehen") }, { id: "shoulder", fromS: 7.5, toS: 10.5, title: t("Shouldering the bag", "Tasche schultern") }],
      events: [{ t: 2, kind: "contact-on", title: t("Hands take the guitar", "Hände nehmen die Gitarre"), target: "guitar-grip-neck" }, { t: 5.5, kind: "contact-off", title: t("Guitar released in the bag", "Gitarre in der Tasche losgelassen"), target: "guitar-grip-neck" }, { t: 7.5, kind: "contact-off", title: t("Bag leaves the floor", "Tasche verlässt den Boden"), target: "bag-floor-head" }, { t: 9.6, kind: "contact-on", title: t("Straps over the shoulders", "Gurte über den Schultern"), target: "strap-shoulders" }],
      tracks: [],
    },
    {
      id: "play", title: t("Playing a phrase", "Eine Phrase spielen"),
      description: t("Seated with the footstool: the right fingers pluck four times a second while the left hand shifts from the second to the fifth position and back, lifting the neck by three degrees.", "Sitzend mit Fußbank: Die Finger der rechten Hand schlagen viermal je Sekunde an, während die linke Hand von der zweiten in die fünfte Lage und zurück wechselt und den Hals um drei Grad hebt."),
      durationS: 2, fps: 20, loop: true, rootMotion: "none",
      keys: [{ t: 0, pose: "played", ease: "smooth" }, { t: 1, pose: "played-reach", ease: "smooth" }, { t: 2, pose: "played", ease: "smooth" }],
      contacts: span([...all, ...BAG_FLOOR, "thigh-left", "thigh-right", "chest"], 0, 2),
      phases: [{ id: "low", fromS: 0, toS: 1, title: t("Second position", "Zweite Lage") }, { id: "high", fromS: 1, toS: 2, title: t("Fifth position and back", "Fünfte Lage und zurück") }],
      events: [{ t: 0, kind: "motion", title: t("Phrase begins", "Phrase beginnt") }, { t: 1, kind: "motion", title: t("Shift to the fifth position", "Lagenwechsel in die fünfte Lage") }],
      tracks: [{ target: "load:pluck", ease: "linear", keys: Array.from({ length: 17 }, (_, i) => ({ t: i * 0.125, v: i % 2 === 0 ? 1 : 0 })) }, { target: "load:fretting", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 0.45, v: 1 }, { t: 0.55, v: 0.2 }, { t: 0.65, v: 1 }, { t: 1.45, v: 1 }, { t: 1.55, v: 0.2 }, { t: 1.65, v: 1 }, { t: 2, v: 1 }] }],
    },
  ];
}

main();
//#endregion
