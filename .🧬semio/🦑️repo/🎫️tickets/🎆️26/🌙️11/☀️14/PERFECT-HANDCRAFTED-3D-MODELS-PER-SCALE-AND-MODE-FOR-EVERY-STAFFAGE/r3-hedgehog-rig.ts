/** 🦔️ Round 3 root fix of animals/hedgehog: the rig from the skeletal landmarks of a 1.0 kg European hedgehog standing in the foraging stance (trunk frame: origin on the sacrum above the hip joint, x ahead, y up, metres), the anchors on it and every pose re-solved on the new chain (three-segment plantigrade limbs, spine coat bowls that turn about the centres of the coat ends).
 * Usage: bun r3-hedgehog-rig.ts   (reads r3-hedgehog-baseline.json, the asset before round 3, writes 🗑️generated/r3-hedgehog/rig-members.json)
 * @see ./r3-animals-hedgehog.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type P = [number, number];
type Json = any;
export const OUT = join(import.meta.dir, "🗑️generated", "r3-hedgehog");
const R = (d: number): number => (d * Math.PI) / 180;
const D = (r: number): number => (r * 180) / Math.PI;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4 + 0;
const r2 = (x: number): number => Math.round(x * 100) / 100 + 0;
const step = (p: P, l: number, a: number): P => [p[0] + l * Math.cos(R(a)), p[1] + l * Math.sin(R(a))];
const ang = (a: P, b: P): number => D(Math.atan2(b[1] - a[1], b[0] - a[0]));
const dist = (a: P, b: P): number => Math.hypot(b[0] - a[0], b[1] - a[1]);
const norm = (a: number): number => { let x = a; while (x > 180) x -= 360; while (x <= -180) x += 360; return x; };

/** Height of the trunk origin (sacrum) above the ground in the standing stance. */
export const HT = 0.066;
export const LEN = { trunk: 0.0696, chest: 0.0772, neck: 0.025, head: 0.066, coat: 0.058, fore: 0.038, forearm: 0.036, forepaw: 0.028, hind: 0.04, shank: 0.044, hindpaw: 0.04 } as const;
/** Joints in the frame of the straight trunk (origin on the sacrum); `back` is the mid-back joint the spine flexes about when the animal rolls up. */
export const JOINT = { back: [0.066, 0.022] as P, hip: [0, -0.01] as P, shoulder: [0.118, -0.008] as P, neck: [0.14, 0] as P, coatFront: [0.116, -0.022] as P, coatRear: [0, -0.022] as P };
export const LATERAL = { hind: 0.024, shank: 0.012, fore: 0.028, forearm: 0.006, coat: 0.014 } as const;
/** Rest world angles of the coat bowls (pole of each bowl) and of the axial bones. */
export const REST = { trunk: 18.43, chest: -16.56, coatFront: 100, coatRear: 101, neck: -20, head: -20, paw: -4 } as const;
/** Sole point of each paw in its bone frame (the contact anchor). */
export const SOLE = { fore: [0.016, -0.0045] as P, hind: [0.022, -0.005] as P };
/** Standing targets: wrist and ankle ahead of the trunk origin (x), the paw flat on the ground. */
export const STAND = { wristX: 0.106, ankleX: -0.01 } as const;

const soleDrop = (sole: P, pawAngle: number): number => sole[0] * Math.sin(R(pawAngle)) + sole[1] * Math.cos(R(pawAngle));

/** 🦵️ Two-bone solution from `p` to `q`: world angles of both bones; `bend` +1 puts the middle joint to the left of the line p -> q. */
function twoBone(p: P, q: P, l1: number, l2: number, bend: 1 | -1): { a1: number; a2: number; far: number } {
  const far = dist(p, q) - (l1 + l2);
  const d = Math.min(Math.max(dist(p, q), Math.abs(l1 - l2) + 1e-6), l1 + l2 - 1e-6);
  const a = D(Math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)));
  const a1 = ang(p, q) + bend * a;
  const mid = step(p, l1, a1);
  return { a1, a2: ang(mid, step(p, d, ang(p, q))), far };
}

type LegWant = { x: number; lift?: number; paw?: number } | { world: [number, number, number] };
type Want = { trunk?: number; chest?: number; neck: number; head: number; coatFront?: number; coatRear?: number; legs: Record<"hn" | "hf" | "fn" | "ff", LegWant>; ground?: number };
const LEGS = {
  hn: ["hind-n", "shank-n", "hindpaw-n"], hf: ["hind-f", "shank-f", "hindpaw-f"],
  fn: ["fore-n", "forearm-n", "forepaw-n"], ff: ["fore-f", "forearm-f", "forepaw-f"],
} as const;

/** 🧮️ World angles of every bone for a wanted posture (trunk origin at (0, 0), ground `ground` below it). */
function solve(id: string, w: Want, warn: string[]): Record<string, number> {
  const trunk = w.trunk ?? 0;
  const ground = w.ground ?? -HT;
  const at = (p: P): P => [p[0] * Math.cos(R(trunk)) - p[1] * Math.sin(R(trunk)), p[0] * Math.sin(R(trunk)) + p[1] * Math.cos(R(trunk))];
  const chest = w.chest ?? trunk;
  const back = at(JOINT.back);
  const atChest = (p: P): P => { const d: P = [p[0] - JOINT.back[0], p[1] - JOINT.back[1]]; return [back[0] + d[0] * Math.cos(R(chest)) - d[1] * Math.sin(R(chest)), back[1] + d[0] * Math.sin(R(chest)) + d[1] * Math.cos(R(chest))]; };
  const out: Record<string, number> = { trunk: REST.trunk + trunk, chest: REST.chest + chest, neck: w.neck, head: w.head };
  for (const s of ["n", "f"]) { out[`coat-f${s}`] = chest + (w.coatFront ?? REST.coatFront); out[`coat-r${s}`] = trunk + (w.coatRear ?? REST.coatRear); }
  for (const [leg, want] of Object.entries(w.legs) as [keyof typeof LEGS, LegWant][]) {
    const [up, low, paw] = LEGS[leg];
    const hind = leg[0] === "h";
    if ("world" in want) { [out[up], out[low], out[paw]] = want.world; continue; }
    const pawAngle = REST.paw + (want.paw ?? 0);
    const sole = hind ? SOLE.hind : SOLE.fore;
    const joint: P = [want.x, ground + (want.lift ?? 0) - soleDrop(sole, REST.paw)];
    const k = twoBone(hind ? at(JOINT.hip) : atChest(JOINT.shoulder), joint, hind ? LEN.hind : LEN.fore, hind ? LEN.shank : LEN.forearm, hind ? 1 : -1);
    if (k.far > 0) warn.push(`${id} ${leg} unreachable by ${k.far.toFixed(4)}`);
    out[up] = k.a1; out[low] = k.a2; out[paw] = pawAngle;
  }
  return out;
}

const STANDING: Want = { neck: REST.neck, head: REST.head, legs: { hn: { x: STAND.ankleX }, hf: { x: STAND.ankleX }, fn: { x: STAND.wristX }, ff: { x: STAND.wristX } } };
const warn: string[] = [];
const REST_WORLD = solve("rest", STANDING, warn);

const PARENT: Record<string, string | null> = { trunk: null, chest: "trunk", neck: "chest", head: "neck" };
for (const s of ["n", "f"]) Object.assign(PARENT, { [`coat-r${s}`]: "trunk", [`coat-f${s}`]: "chest", [`fore-${s}`]: "chest", [`forearm-${s}`]: `fore-${s}`, [`forepaw-${s}`]: `forearm-${s}`, [`hind-${s}`]: "trunk", [`shank-${s}`]: `hind-${s}`, [`hindpaw-${s}`]: `shank-${s}` });
const ORDER = ["trunk", "coat-rn", "coat-rf", "chest", "coat-fn", "coat-ff", "neck", "head", "fore-n", "forearm-n", "forepaw-n", "hind-n", "shank-n", "hindpaw-n", "fore-f", "forearm-f", "forepaw-f", "hind-f", "shank-f", "hindpaw-f"];
const stem = (id: string): string => id.replace(/-[nf]$/, "").replace(/^coat-[rf]$/, "coat");
const lengthOf = (id: string): number => (LEN as Record<string, number>)[stem(id).replace(/^coat.*/, "coat")]!;
const turn = (p: P, a: number): P => [r4(p[0] * Math.cos(R(a)) + p[1] * Math.sin(R(a))), r4(-p[0] * Math.sin(R(a)) + p[1] * Math.cos(R(a)))];
const originOf = (id: string): P => {
  const s = stem(id);
  if (id.startsWith("coat-r")) return turn(JOINT.coatRear, REST.trunk);
  const inChest = (p: P): P => turn([p[0] - JOINT.back[0], p[1] - JOINT.back[1]], REST.chest);
  if (id.startsWith("coat-f")) return inChest(JOINT.coatFront);
  if (s === "neck") return inChest(JOINT.neck);
  if (s === "fore") return inChest(JOINT.shoulder);
  if (s === "chest") return [LEN.trunk, 0];
  if (s === "hind") return turn(JOINT.hip, REST.trunk);
  if (s === "trunk") return [0, 0];
  return [lengthOf(PARENT[id]!), 0];
};
const lateralOf = (id: string): number | undefined => {
  const k = id.startsWith("coat") ? LATERAL.coat : (LATERAL as Record<string, number>)[stem(id)];
  return k === undefined ? undefined : id.endsWith("f") ? k : -k;
};
const TITLE: Record<string, [string, string]> = {
  trunk: ["Loin: lumbar spine, sacrum and pelvis under the spiny dome", "Lende: Lendenwirbelsäule, Kreuzbein und Becken unter der Stachelkuppel"],
  chest: ["Chest: thoracic spine and ribcage under the spiny dome, flexing against the loin when the animal rolls up", "Brust: Brustwirbelsäule und Brustkorb unter der Stachelkuppel, beim Einrollen gegen die Lende gebeugt"],
  "coat-r": ["Rear spine coat bowl, {s} half (drawn over the rump by the caudodorsalis and orbicularis muscles)", "Hintere Stachelkleid-Schale, {s} Hälfte (von Caudodorsalis und Orbicularis über den Rumpf gezogen)"],
  "coat-f": ["Front spine coat bowl, {s} half (drawn over the head by the frontodorsalis and orbicularis muscles)", "Vordere Stachelkleid-Schale, {s} Hälfte (von Frontodorsalis und Orbicularis über den Kopf gezogen)"],
  neck: ["Neck (seven short cervical vertebrae)", "Hals (sieben kurze Halswirbel)"],
  head: ["Head: skull from the occiput to the tip of the nose", "Kopf: Schädel vom Hinterhaupt bis zur Nasenspitze"],
  fore: ["Upper arm (humerus), {s}", "Oberarm (Humerus), {s}"], forearm: ["Forearm (radius and ulna), {s}", "Unterarm (Radius und Ulna), {s}"], forepaw: ["Fore paw (carpus to toe tips), {s}", "Vorderpfote (Handwurzel bis Zehenspitzen), {s}"],
  hind: ["Thigh (femur), {s}", "Oberschenkel (Femur), {s}"], shank: ["Shank (tibia and fibula), {s}", "Unterschenkel (Tibia und Fibula), {s}"], hindpaw: ["Hind foot (heel to toe tips, plantigrade), {s}", "Hinterfuß (Ferse bis Zehenspitzen, Sohlengänger), {s}"],
};
const MASS: Record<string, [number, number, number]> = { trunk: [0.34, 0.035, -0.02], chest: [0.306, 0.035, -0.022], coat: [0.02, 0.04, 0], neck: [0.02, 0.0125, 0], head: [0.08, 0.026, 0], fore: [0.018, 0.017, 0], forearm: [0.012, 0.016, 0], forepaw: [0.01, 0.013, 0], hind: [0.022, 0.018, 0], shank: [0.013, 0.02, 0], hindpaw: [0.012, 0.018, 0] };
const LIMIT: Record<string, [number, number]> = { trunk: [-60, 60], chest: [-90, 30], coat: [-175, 175], neck: [-110, 110], head: [-110, 110] };

const before: Json = JSON.parse(readFileSync(join(import.meta.dir, "r3-hedgehog-baseline.json"), "utf8")).asset;
const bones: Json[] = ORDER.map((id) => {
  const s = stem(id);
  const key = id.startsWith("coat") ? id.slice(0, 6) : s;
  const side = id.endsWith("-n") || id.endsWith("n") && id.startsWith("coat") ? ["near", "nahe"] : ["far", "ferne"];
  const [en, de] = TITLE[key]!;
  const o = originOf(id);
  const p = PARENT[id]!;
  const z = lateralOf(id);
  const [kg, cx, cy] = MASS[id.startsWith("coat") ? "coat" : s]!;
  const [lo, hi] = LIMIT[id.startsWith("coat") ? "coat" : s] ?? [-170, 170];
  return {
    id, parent: p ?? null, title: { en: en.replace("{s}", side[0]!), de: de.replace("{s}", side[1]!) }, x: o[0], y: o[1],
    angleDeg: r2(p === undefined || p === null ? REST_WORLD[id]! : norm(REST_WORLD[id]! - REST_WORLD[p]!)), lengthM: lengthOf(id),
    joint: { type: "revolute", minDeg: lo, maxDeg: hi },
    mass: { kg, comX: cx, comY: cy, distribution: s === "trunk" || s === "chest" || s === "head" ? "point" : "uniform", source: "authored-estimate", reference: "reeve-1994" },
    ...(z === undefined ? {} : { z }),
  };
});
const restAngle: Record<string, number> = Object.fromEntries(bones.map((b) => [b.id, b.angleDeg]));

/** Offsets from rest for wanted world angles. */
function offsets(world: Record<string, number>, extra: Record<string, Json> = {}): Json {
  const out: Json = {};
  for (const id of ORDER) {
    const p = PARENT[id];
    out[id] = { angleDeg: r2(norm(world[id]! - (p ? world[p]! : 0) - restAngle[id]!)), ...(extra[id] ?? {}) };
  }
  return out;
}

const NECK_F = -35;
const HEAD_F = -32;
const forage = (trunk: number, legs: Want["legs"]): Want => ({ trunk, neck: trunk + NECK_F, head: trunk + HEAD_F, legs });
const standLegs = (dn = 0, df = 0): Want["legs"] => ({ hn: { x: STAND.ankleX + dn }, hf: { x: STAND.ankleX + df + 0.012 }, fn: { x: STAND.wristX + dn }, ff: { x: STAND.wristX + df - 0.01 } });
const CYCLE = 0.8;
const STANCE = 0.528;
const TRAVEL = 0.1127 * (STANCE / CYCLE);
const ON = { hn: 0, fn: 0.2, hf: 0.4, ff: 0.6 } as const;
function walkLegs(t: number): Want["legs"] {
  const out: Json = {};
  for (const leg of ["hn", "hf", "fn", "ff"] as const) {
    const x0 = leg[0] === "h" ? STAND.ankleX : STAND.wristX;
    const tau = (((t - ON[leg]) % CYCLE) + CYCLE) % CYCLE;
    if (tau < STANCE) out[leg] = { x: x0 + TRAVEL / 2 - TRAVEL * (tau / STANCE) };
    else {
      const u = (tau - STANCE) / (CYCLE - STANCE);
      const sm = u * u * (3 - 2 * u);
      out[leg] = { x: x0 - TRAVEL / 2 + TRAVEL * sm, lift: 0.008 * Math.sin(Math.PI * u), paw: -22 * Math.sin(Math.PI * u) * (1 - u) * 2 };
    }
  }
  return out;
}
const BOB = [0, 1.5, 0, -1.5, 0, 1.5, 0, -1.5];
/** Rolling up: the turn of each coat bowl against its spine half, and the flexion of each spine half against the horizontal. */
const ROLL = { front: -135, rear: 114 };
const FLEX = 35;
const WANT: Record<string, { want: Want; extra?: Record<string, Json> }> = {
  foraging: { want: forage(0, standLegs()) },
  alert: { want: { trunk: 4, neck: 12, head: -4, coatFront: REST.coatFront - 6, coatRear: REST.coatRear + 6, legs: standLegs() } },
  "rolling-half": { want: { trunk: FLEX / 2, chest: -FLEX / 2, neck: -90, head: -128, coatFront: REST.coatFront + ROLL.front / 2, coatRear: REST.coatRear + ROLL.rear / 2, legs: standLegs(), ground: -HT + 0.004 } },
  rolled: {
    want: {
      trunk: FLEX, chest: -FLEX, neck: -140, head: -186, coatFront: REST.coatFront + ROLL.front, coatRear: REST.coatRear + ROLL.rear,
      legs: { hn: { world: [15, -150, -5] }, hf: { world: [20, -146, 0] }, fn: { world: [-178, -30, -178] }, ff: { world: [-182, -34, -182] } },
    },
  },
};
for (let k = 0; k < 8; k++) WANT[`walk-${k}`] = { want: forage(BOB[k]!, walkLegs(k * 0.1)) };

const poses: Json[] = before.poses.map((p: Json) => {
  const w = WANT[p.id];
  if (w === undefined) throw new Error(`no posture for pose ${p.id}`);
  return { ...p, bones: offsets(solve(p.id, w.want, warn), w.extra) };
});

const local = (bone: string, world: P, state: Record<string, { o: P; a: number }>): P => {
  const f = state[bone]!;
  const d: P = [world[0] - f.o[0], world[1] - f.o[1]];
  return [r4(d[0] * Math.cos(R(f.a)) + d[1] * Math.sin(R(f.a))), r4(-d[0] * Math.sin(R(f.a)) + d[1] * Math.cos(R(f.a)))];
};
export function fk(world: Record<string, number>): Record<string, { o: P; a: number }> {
  const st: Record<string, { o: P; a: number }> = {};
  for (const b of bones) {
    const p = b.parent === null ? undefined : st[b.parent];
    const o: P = p === undefined ? [0, 0] : [p.o[0] + b.x * Math.cos(R(p.a)) - b.y * Math.sin(R(p.a)), p.o[1] + b.x * Math.sin(R(p.a)) + b.y * Math.cos(R(p.a))];
    st[b.id] = { o, a: world[b.id]! };
  }
  return st;
}
const restState = fk(REST_WORLD);
const polar = (l: number, a: number): P => [r4(l * Math.cos(R(a))), r4(l * Math.sin(R(a)))];
const ANCHOR: Record<string, { bone: string; at: P }> = {
  "foot-fn": { bone: "forepaw-n", at: SOLE.fore }, "foot-ff": { bone: "forepaw-f", at: SOLE.fore }, "foot-hn": { bone: "hindpaw-n", at: SOLE.hind }, "foot-hf": { bone: "hindpaw-f", at: SOLE.hind },
  "spine-front": { bone: "coat-fn", at: polar(LEN.coat, -90 - (REST.coatFront + ROLL.front - FLEX)) }, "spine-rear": { bone: "coat-rn", at: polar(LEN.coat, 270 - (REST.coatRear + ROLL.rear + FLEX)) },
  nostrils: { bone: "head", at: [0.066, 0.0005] }, "snout-tip": { bone: "head", at: [0.0665, -0.001] }, eye: { bone: "head", at: [0.03, 0.0065] }, "back-plume": { bone: "trunk", at: local("trunk", [0.056, 0.051], restState) },
};
const anchors: Json[] = before.anchors.map((a: Json) => {
  const n = ANCHOR[a.id];
  if (n === undefined) throw new Error(`no anchor ${a.id}`);
  const out: Json = { ...a, bone: n.bone, x: n.at[0], y: n.at[1] };
  if (a.id.startsWith("foot-")) { out.footprintLengthM = a.id[5] === "h" ? 0.04 : 0.028; out.footprintWidthM = 0.018; }
  return out;
});
const materials: Json[] = [...before.materials.filter((m: Json) => m.id !== "bone"), { id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate", reference: "reeve-1994" }];

if (import.meta.main) {
  mkdirSync(OUT, { recursive: true });
  writeFileSync(join(OUT, "rig-members.json"), JSON.stringify({ materials, rig: { ...before.rig, bones }, anchors, poses }));
  for (const w of warn) console.log("WARN", w);
  const f = (p: P): string => `(${p[0].toFixed(4)}, ${(p[1] + HT).toFixed(4)})`;
  for (const b of bones) console.log(b.id.padEnd(12), "joint", f(restState[b.id]!.o), "tip", f(step(restState[b.id]!.o, b.lengthM, restState[b.id]!.a)), "world", REST_WORLD[b.id]!.toFixed(1), "rest", b.angleDeg);
  console.log("mass", bones.reduce((s, b) => s + b.mass.kg, 0).toFixed(3));
}
