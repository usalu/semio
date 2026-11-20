/** 🐗️ Round 3 pig at 1:100, 1:200 and 1:500: each scale has its own hand tables (merged masses, fewer and simpler sections), never the finer list with parts removed.
 * Usage: bun r3-pig-coarse.ts <100|200|500> [--write]
 * @see ./r3-pig-50.ts
 * @see ./r3-animals-pig.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Pig, dense, side, sx, type Json, type Look, type V3 } from "./r3-pig-kit.ts";
import { DISC, EAR, FACE, FORE_Z, HIND_Z, HORN, LEG, SKIN, SPINE_C, SPINE_L, SPINE_T, TAIL, BONE, JOINT, MASS, LUMBAR_V, SPINOUS_T, STERNUM, ear, ham, hullRows, lumbarY, ribPath, ribX, spineY, spinous, thermal } from "./r3-pig-50.ts";

const scale = Number(process.argv[2]) as 100 | 200 | 500;
const SIDES = [-1, 1] as const;
const len = (S: Pig, bone: string): number => S.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
const toeOf = (S: Pig, hoof: string): V3 => { const f = S.frame(hoof); const l = len(S, hoof); return [f.o[0]! + f.u[0]! * l, f.o[1]! + f.u[1]! * l, 0]; };

/** Hand tables per scale. Trunk rows: x, top, underline, half width, exponent. Neck and head rows: s, v, half depth, half width. Leg rows: s, v, sagittal half, lateral half, lateral centre. Ham rows: world x, height, fore-aft half, lateral half, lateral centre. Ear rows: s, v, lateral centre, half width, half thickness. Hoof rows: x from the toe, centre height, half height, half width, exponent. */
const T = {
  100: {
    hind: [[-0.118, 0.612, 0.5, 0.06, 2], [-0.09, 0.642, 0.41, 0.118, 2.2], [-0.03, 0.664, 0.35, 0.15, 2.3], [0.1, 0.678, 0.318, 0.161, 2.5], [0.25, 0.683, 0.3, 0.164, 2.5], [0.4, 0.676, 0.298, 0.158, 2.5]],
    chest: [[0.34, 0.684, 0.294, 0.164, 2.5], [0.5, 0.675, 0.29, 0.162, 2.5], [0.62, 0.66, 0.292, 0.155, 2.4], [0.71, 0.642, 0.31, 0.138, 2.25], [0.8, 0.626, 0.37, 0.108, 2.05], [0.85, 0.57, 0.48, 0.035, 2]],
    neck: [[-0.04, -0.03, 0.13, 0.112], [0.08, -0.028, 0.122, 0.106], [0.19, -0.024, 0.084, 0.084]],
    head: [[-0.045, -0.005, 0.085, 0.074], [0.03, -0.012, 0.096, 0.084], [0.1, -0.012, 0.08, 0.075], [0.17, -0.008, 0.058, 0.052], [0.245, -0.002, 0.036, 0.035]],
    snout: [[0.24, 0.005, 0.027, 0.031], [0.266, 0.005, 0.03, 0.034]],
    ear: [[-0.01, 0, 0.06, 0.03, 0.0085], [0.05, 0.002, 0.086, 0.054, 0.0085], [0.12, -0.003, 0.092, 0.044, 0.0085], [0.166, -0.008, 0.088, 0.016, 0.0085]],
    tail: [[-0.098, 0.612, 0, 0.012], [-0.128, 0.626, 0, 0.0105], [-0.148, 0.618, 0.008, 0.0095], [-0.152, 0.598, 0.016, 0.009], [-0.138, 0.586, 0.014, 0.0085], [-0.128, 0.6, 0.004, 0.008]],
    ham: [[-0.012, 0.628, 0.04, 0.03, 0.07], [-0.025, 0.58, 0.08, 0.058, 0.08], [-0.037, 0.48, 0.112, 0.075, 0.09], [-0.015, 0.37, 0.095, 0.068, 0.093], [0.02, 0.27, 0.048, 0.046, 0.095]],
    gaskin: [[0.05, -0.006, 0.046, 0.041, 0.095], [0.12, -0.008, 0.035, 0.032, 0.095], [0.2, 0, 0.026, 0.025, 0.095]],
    hindCannon: [[-0.02, -0.012, 0.031, 0.025, 0.095], [0.06, 0, 0.021, 0.0215, 0.095], [0.13, 0, 0.0225, 0.023, 0.095]],
    forearm: [[-0.005, -0.004, 0.045, 0.04, 0.094], [0.07, 0.003, 0.034, 0.031, 0.094], [0.15, 0, 0.027, 0.026, 0.093]],
    foreCannon: [[-0.01, 0, 0.0265, 0.0255, 0.093], [0.05, 0, 0.022, 0.022, 0.093], [0.1, 0, 0.0225, 0.0235, 0.093]],
    pastern: [[-0.012, 0, 0.0215, 0.023], [0.045, 0.002, 0.0195, 0.025]],
    hoof: [[-0.052, 0.016, 0.016, 0.022, 3], [-0.03, 0.0175, 0.0175, 0.025, 3], [-0.012, 0.012, 0.012, 0.023, 3], [-0.001, 0.0075, 0.0075, 0.016, 3]],
  },
  200: {
    hind: [[-0.115, 0.61, 0.5, 0.06, 2], [-0.07, 0.65, 0.39, 0.13, 2.2], [0.05, 0.674, 0.325, 0.16, 2.4], [0.25, 0.683, 0.298, 0.164, 2.5], [0.4, 0.676, 0.298, 0.158, 2.5]],
    chest: [[0.34, 0.684, 0.294, 0.164, 2.5], [0.55, 0.67, 0.29, 0.16, 2.45], [0.72, 0.64, 0.315, 0.135, 2.2], [0.8, 0.626, 0.37, 0.108, 2.05], [0.85, 0.57, 0.48, 0.035, 2]],
    neck: [[-0.04, -0.03, 0.13, 0.112], [0.1, -0.027, 0.118, 0.103], [0.19, -0.024, 0.084, 0.084]],
    head: [[-0.045, -0.005, 0.085, 0.074], [0.05, -0.012, 0.092, 0.083], [0.16, -0.008, 0.062, 0.056], [0.25, 0, 0.035, 0.035]],
    snout: [[0.235, 0.005, 0.028, 0.032], [0.266, 0.005, 0.03, 0.034]],
    ear: [[0, 0, 0.066, 0.03, 0.011], [0.07, 0, 0.09, 0.05, 0.011], [0.16, -0.006, 0.09, 0.02, 0.011]],
    ham: [[-0.015, 0.63, 0.045, 0.036, 0.072], [-0.03, 0.57, 0.085, 0.06, 0.085], [-0.035, 0.45, 0.11, 0.075, 0.09], [0.015, 0.29, 0.06, 0.052, 0.094]],
    gaskin: [[0.06, -0.006, 0.045, 0.04, 0.095], [0.2, 0, 0.026, 0.025, 0.095]],
    hindCannon: [[-0.02, -0.01, 0.029, 0.025, 0.095], [0.13, 0, 0.0225, 0.023, 0.095]],
    forearm: [[-0.005, -0.004, 0.045, 0.04, 0.094], [0.15, 0, 0.027, 0.026, 0.093]],
    foreCannon: [[-0.01, 0, 0.027, 0.026, 0.093], [0.1, 0, 0.0225, 0.0235, 0.093]],
    hoof: [[-0.072, 0.052, 0.02, 0.023, 2.4], [-0.047, 0.029, 0.028, 0.025, 2.6], [-0.02, 0.017, 0.017, 0.024, 3], [-0.002, 0.0085, 0.0085, 0.017, 3]],
  },
  500: {
    hind: [[-0.145, 0.59, 0.46, 0.05, 2.1], [-0.125, 0.618, 0.4, 0.105, 2.2], [-0.07, 0.655, 0.325, 0.152, 2.4], [0.05, 0.676, 0.31, 0.163, 2.5], [0.2, 0.682, 0.302, 0.164, 2.5], [0.4, 0.676, 0.298, 0.158, 2.5]],
    chest: [[0.34, 0.684, 0.294, 0.164, 2.5], [0.5, 0.675, 0.29, 0.162, 2.5], [0.62, 0.66, 0.292, 0.155, 2.4], [0.72, 0.64, 0.315, 0.135, 2.2], [0.8, 0.626, 0.37, 0.108, 2.1], [0.85, 0.58, 0.45, 0.05, 2]],
    neck: [[-0.04, -0.03, 0.13, 0.112], [0.19, -0.02, 0.1, 0.095]],
    head: [[-0.045, -0.008, 0.094, 0.09], [0.04, -0.014, 0.098, 0.1], [0.12, -0.012, 0.078, 0.082], [0.2, -0.004, 0.05, 0.05], [0.268, 0.002, 0.042, 0.042]],
    ear: [[-0.005, 0, 0.066, 0.042, 0.0415], [0.07, 0, 0.09, 0.052, 0.0415], [0.15, -0.006, 0.09, 0.042, 0.0415]],
    gaskin: [[0.05, -0.006, 0.047, 0.046, 0.095], [0.2, 0, 0.046, 0.046, 0.095]],
    forearm: [[-0.005, 0, 0.047, 0.046, 0.094], [0.14, 0, 0.046, 0.046, 0.093]],
  },
} as const;

/** 🐖️ The body of the scale from its own tables. */
function body(S: Pig): void {
  const t = T[scale] as Record<string, readonly (readonly number[])[]>;
  S.hull("hindbody", "lumbar", dense(hullRows(t.hind!), 3), SKIN, { shape: "hindbody" });
  S.hull("chest", "thorax", dense(hullRows(t.chest!), 3), SKIN, { shape: "chest" });
  S.limb("neck", "neck", dense(t.neck!, 3), SKIN, { shape: "neck", e: 2.3 });
  S.limb("head", "head", dense(t.head!, 3), FACE, { shape: "head", e: 2.4 });
  if (scale !== 500) S.limb("snout", "head", t.snout!, DISC, { shape: scale === 100 ? "snout" : "head", e: 2.3 });
  if (scale === 100) S.run("tail", "tail-1", t.tail!.map(([x, y, z, r]) => ({ p: [x!, y!, z!] as V3, rl: r!, rs: r! })), TAIL, { shape: "tail", floor: 0.008 });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    if (scale !== 500) {
      ear(S, `${n}-ear`, sd, EAR, t.ear!, scale === 100 ? `ear-${k}` : sd < 0 ? "ear" : "ear-far");
      ham(S, `${n}-ham`, sd, SKIN, dense(t.ham!, 3), scale === 100 && sd < 0 ? "ham-n|hindbody" : "hindbody");
    }
    if (scale === 500) {
      ear(S, `${n}-ear`, sd, EAR, t.ear!, "head");
      S.limb(`${n}-hind-upper`, `shank-h${k}`, t.gaskin!, LEG, { sd, shape: `leg-hind-upper-${k}` });
      S.limb(`${n}-fore-upper`, `forearm-f${k}`, t.forearm!, LEG, { sd, shape: `leg-fore-upper-${k}` });
      for (const [leg, bone, hoof, z] of [["hind", `metatarsus-h${k}`, `hoof-h${k}`, HIND_Z], ["fore", `metacarpus-f${k}`, `hoof-f${k}`, FORE_Z]] as const) {
        const toe = toeOf(S, hoof);
        const f = S.frame(hoof).o;
        S.run(`${n}-${leg}-lower`, bone, [{ p: S.at(bone, -0.01, 0, sd * z), rl: 0.046, rs: 0.046 }, { p: [f[0]!, f[1]!, sd * z], rl: 0.046, rs: 0.046 }, { p: [f[0]! + 0.012, 0.0065, sd * z], rl: 0.046, rs: 0.046 }], LEG, { shape: `leg-${leg}-lower-${k}` });
      }
      continue;
    }
    S.limb(`${n}-gaskin`, `shank-h${k}`, t.gaskin!, LEG, { sd, shape: `gaskin-${k}` });
    S.limb(`${n}-hind-cannon`, `metatarsus-h${k}`, t.hindCannon!, LEG, { sd, shape: `cannon-h-${k}` });
    S.limb(`${n}-forearm`, `forearm-f${k}`, t.forearm!, LEG, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, t.foreCannon!, LEG, { sd, shape: `cannon-f-${k}` });
    for (const [leg, h, z] of [["hind", "h", HIND_Z], ["fore", "f", FORE_Z]] as const) {
      const hoof = `hoof-${h}${k}`;
      const toe = toeOf(S, hoof);
      if (scale === 100) S.limb(`${n}-${leg}-pastern`, hoof, t.pastern!.map((r) => [...r, z]), LEG, { sd, shape: `hoof-${h}-${k}` });
      S.hull(`${n}-${leg}-hoof`, hoof, t.hoof!.map((r) => [r[0]!, r[1]! + toe[1], r[2]!, r[3]!, r[4]!, sd * z]), HORN, { x0: toe[0], sd: 1, shape: `hoof-${h}-${k}`, floor: 0.006 });
    }
  }
}

const CHILD: Record<string, string> = { thigh: "shank", shank: "metatarsus", metatarsus: "hoof", scapula: "humerus", humerus: "forearm", forearm: "metacarpus", metacarpus: "hoof" };
const JZ: Record<string, number> = { thigh: 0.078, shank: 0.1, metatarsus: 0.096, "hoof-h": 0.095, scapula: 0.07, humerus: 0.1, forearm: 0.096, metacarpus: 0.093, "hoof-f": 0.093 };

/** 🦴️ The skeleton generalised for the scale: spine, pelvis, skull, ribs as hoops (100, 200), one bar per long bone, joints (100, 200), masses. */
function structure(K: Pig): Json[] {
  const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });
  const r = 0.55 * K.feature;
  const big = scale === 100 ? 1 : 0;
  const tube = (id: string, bone: string, pts: readonly V3[], rl = r, rs = r): Json => mem(K.run(id, bone, pts.map((p) => ({ p, rl: Math.max(r, rl), rs: Math.max(r, rs) })), BONE), bone);
  tube("lumbar-spine", "lumbar", scale === 500 ? [SPINE_L[0]!, SPINE_L[2]!] : [...SPINE_L], 0.024 * big, 0.018 * big);
  tube("thoracic-spine", "thorax", scale === 500 ? [SPINE_T[0]!, SPINE_T[4]!] : scale === 200 ? [SPINE_T[0]!, SPINE_T[2]!, SPINE_T[4]!] : [...SPINE_T], 0.017 * big, 0.016 * big);
  if (scale === 100) {
    for (const i of [0, 2, 4, 6, 8, 10, 12]) mem(spinous(K, `thoracic-spinous-${i + 1}`, "thorax", ribX(i), spineY(ribX(i)) + 0.01, SPINOUS_T[i]![0], SPINOUS_T[i]![1], r, r), "thorax");
    for (const i of [0, 2, 4]) mem(spinous(K, `lumbar-spinous-${i + 1}`, "lumbar", LUMBAR_V[i]![0], lumbarY(LUMBAR_V[i]![0]) + 0.012, LUMBAR_V[i]![1], -0.008, r, r), "lumbar");
  }

  tube("cervical-spine", "neck", scale === 500 ? [SPINE_C[0]!, SPINE_C[4]!] : [SPINE_C[0]!, SPINE_C[2]!, SPINE_C[4]!], 0.026 * big, 0.02 * big);
  if (scale === 500) tube("pelvis-bone", "pelvis", [[-0.09, 0.56, 0], [0.11, 0.61, 0]], 0.075, r);
  else {
    tube("sacrum", "pelvis", [[0.1, 0.6, 0], [0.0, 0.614, 0], [-0.098, 0.612, 0]], 0.026 * big, 0.015 * big);
    for (const sd of SIDES) mem(K.run(`${side(sd)}-hip-bone`, "pelvis", [{ p: [0.128, 0.622, sd * 0.09], rl: r, rs: scale === 100 ? 0.03 : r }, { p: [0.0, 0.508, sd * 0.076], rl: r, rs: scale === 100 ? 0.02 : r }, { p: [-0.1, 0.526, sd * 0.066], rl: r, rs: scale === 100 ? 0.018 : r }], BONE), "pelvis");
  }
  if (scale === 100) {
    mem(K.limb("skull", "head", [[-0.012, 0.03, 0.04, 0.035], [0.04, 0.025, 0.055, 0.062], [0.12, 0.015, 0.042, 0.052], [0.2, 0.01, 0.026, 0.028], [0.255, 0.006, 0.017, 0.02]], BONE, { e: 2.4 }), "head");
    mem(K.limb("mandible", "jaw", [[0.03, -0.022, 0.03, 0.056], [0.06, -0.05, 0.04, 0.06], [0.15, -0.048, 0.02, 0.036], [0.235, -0.036, 0.01, 0.016]], BONE, { frame: "head" }), "jaw");
  } else {
    mem(K.limb("skull", "head", [[-0.005, 0.028, 0.045, 0.045], [0.06, 0.022, 0.05, 0.06], [0.25, 0.008, 0.0165, 0.02]], BONE), "head");
    mem(K.run("mandible", "jaw", [{ p: K.at("head", 0.04, -0.045), rl: Math.max(r, 0.05), rs: r }, { p: K.at("head", 0.235, -0.036), rl: r, rs: r }], BONE), "jaw");
  }
  for (const [i, b] of ["tail-1", "tail-2"].entries()) { if (scale === 500) mem(K.ball(`caudal-${i + 1}`, b, [-0.1 - 0.012 * i, 0.6 - 0.012 * i, 0], [r, r, r], BONE), b); else if (scale === 200) tube(`caudal-${i + 1}`, b, i === 0 ? [[-0.065, 0.608, 0], [-0.1, 0.61, 0]] : [[-0.082, 0.597, 0], [-0.116, 0.586, 0]]); else tube(`caudal-${i + 1}`, b, [K.at(b, 0, 0), K.at(b, len(K, b), 0)]); }
  if (scale !== 500) mem(K.run("sternum", "thorax", STERNUM.map((p, i) => ({ p, rl: i === 1 || i === 2 ? Math.max(r, 0.02) : r, rs: r })), BONE), "thorax");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    for (const i of scale === 100 ? [0, 2, 4, 6, 8, 10, 12] : scale === 200 ? [0, 3, 6, 9, 12] : []) mem(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, r), BONE), "thorax");
    const eb = K.at(`ear-${k}`, 0, 0, sd * 0.06);
    const et = K.at(`ear-${k}`, 0.14, -0.005, sd * 0.088);
    if (scale === 500) mem(K.ball(`${n}-ear-cartilage`, `ear-${k}`, [eb[0], eb[1], sd * 0.06], [r, r, r], BONE), `ear-${k}`);
    else tube(`${n}-ear-cartilage`, `ear-${k}`, [eb, et]);
    for (const [stem, h] of [["thigh", "h"], ["shank", "h"], ["metatarsus", "h"], ["scapula", "f"], ["humerus", "f"], ["forearm", "f"], ["metacarpus", "f"]] as const) {
      const bone = `${stem}-${h}${k}`;
      const child = CHILD[stem]!;
      const z1 = JZ[child === "hoof" ? `hoof-${h}` : child]!;
      const ends = stem === "scapula" && scale === 100 ? 0.035 : 0;
      mem(K.run(`${n}-${h === "h" ? "hind" : "fore"}-${stem}`, bone, [{ p: K.at(bone, 0, 0, sd * JZ[stem]!), rl: r, rs: Math.max(r, ends) }, { p: K.at(bone, len(K, bone), 0, sd * z1), rl: r, rs: r }], BONE), bone);
    }
    for (const [leg, h, z] of [["hind", "h", HIND_Z], ["fore", "f", FORE_Z]] as const) {
      const hoof = `hoof-${h}${k}`;
      const toe = toeOf(K, hoof);
      const f = K.frame(hoof).o;
      if (scale === 500) mem(K.ball(`${n}-${leg}-digits`, hoof, [toe[0] - 0.045, r, sd * z], [r, r, r], BONE), hoof);
      else tube(`${n}-${leg}-digits`, hoof, [[f[0]!, f[1]!, sd * z], [toe[0] - 0.012 - r * 0.6, Math.max(r, toe[1] + 0.01), sd * z]]);
    }
    if (scale !== 500) for (const [stem, h, rr] of [["thigh", "h", 0.022], ["shank", "h", 0.025], ["metatarsus", "h", 0.02], ["humerus", "f", 0.024], ["forearm", "f", 0.022], ["metacarpus", "f", 0.02]] as const) {
      const bone = `${stem}-${h}${k}`;
      const o = K.frame(bone).o;
      const q = Math.max(rr, r * 1.25);
      mem(K.ball(`joint-${bone}`, bone, [o[0]!, o[1]!, sd * JZ[stem]!], [q, q, q], JOINT), bone, "joint");
    }
  }
  if (scale === 100) for (const b of ["lumbar", "thorax", "neck", "head"]) { const o = K.frame(b).o; mem(K.ball(`joint-${b}`, b, [o[0]!, o[1]!, 0], [0.021, 0.021, 0.021], JOINT), b, "joint"); }
  const m = Math.max(0.04, r);
  mem(K.ball("mass-thorax", "thorax", [0.5, 0.46, 0], [m, m, m], MASS), "thorax", "mass");
  mem(K.ball("mass-lumbar", "lumbar", [0.22, 0.49, 0], [m * 0.85, m * 0.85, m * 0.85], MASS), "lumbar", "mass");
  return K.body;
}

const D: Record<number, { en: string; de: string }[]> = {
  100: [
    { en: "Body at 1:100: drawn anew for a 15 mm feature. The barrel keeps its two lofts with rounded rump, level back, straight underline and brisket in five or six sections each; the hams are two hanging masses of four sections, the shoulder and upper arm are merged into the chest; the head is one wedge with the jowl in it, the rooting disc and two lop ears over the eyes, the corkscrew becomes a short hooked tail; each leg is a gaskin or forearm, a cannon, a pastern and one hoof for both claws; the eyes and dewclaws are below the scale.", de: "Körper bei 1:100: neu gezeichnet für 15 mm kleinste Abmessung. Die Tonne behält ihre zwei Lofts mit runder Kruppe, ebenem Rücken, gerader Unterlinie und Vorderbrust in je fünf oder sechs Schnitten; die Schinken sind zwei hängende Massen aus vier Schnitten, Schulter und Oberarm gehen in der Brust auf; der Kopf ist ein Keil mit der Backe darin, die Rüsselscheibe und zwei Schlappohren über den Augen, der Ringel wird ein kurzer gebogener Schwanz; jedes Bein besteht aus Unterschenkel oder Unterarm, Röhre, Fessel und einer Klaue für beide Klauen; Augen und Afterklauen liegen unter dem Maßstab." },
    { en: "Thermal at 1:100: the body surfaces per region as at 1:50, the merged parts in the region of the mass that took them (the jowl in the head skin, the shoulder in the trunk skin); two nostrils in the rooting disc, the lips below it and a patch of the back are the outlets on the surface.", de: "Thermik bei 1:100: die Körperoberflächen je Region wie bei 1:50, die zusammengefassten Teile in der Region der Masse, die sie aufnahm (die Backe in der Kopfhaut, die Schulter in der Rumpfhaut); zwei Nasenlöcher in der Rüsselscheibe, die Lippen darunter und ein Rückenfeld sind die Auslässe auf der Oberfläche." },
    { en: "Structure at 1:100: the skeleton generalised: skull and mandible, cervical, thoracic and lumbar spine with a spinous process on every second vertebra (the long ones carry the withers), sacrum and one hip bone per side, seven rib hoops per side for the fourteen ribs, joined to the sternum,, one bar per long bone from joint to joint with the scapula as a blade, one bar for the two digits of a foot, caudal vertebrae and ear cartilages; joints at the limb joints and along the spine, the masses of thorax and loin at their centres of mass.", de: "Tragwerk bei 1:100: das Skelett verallgemeinert: Schädel und Unterkiefer, Hals-, Brust- und Lendenwirbelsäule mit einem Dornfortsatz an jedem zweiten Wirbel (die langen tragen den Widerrist), Kreuzbein und ein Hüftbein je Seite, sieben Rippenbögen je Seite für die vierzehn Rippen, am Brustbein verbunden, ein Stab je Röhrenknochen von Gelenk zu Gelenk mit dem Schulterblatt als Platte, ein Stab für die zwei Zehen eines Fußes, Schwanzwirbel und Ohrknorpel; Gelenke an den Gliedmaßengelenken und entlang der Wirbelsäule, die Massen von Brust und Lende in ihren Schwerpunkten." },
  ],
  200: [
    { en: "Body at 1:200: drawn anew for a 30 mm feature. The barrel is two lofts of five and four sections, the neck three, the head a wedge of four sections with the jowl and the rooting disc; the two hams stay as hanging masses of three sections because they make the pig at this size, the ears are thick leaves over the face; each leg is an upper mass, a cannon and one wedge for pastern and claws. Tail, eyes and dewclaws are below the scale.", de: "Körper bei 1:200: neu gezeichnet für 30 mm kleinste Abmessung. Die Tonne besteht aus zwei Lofts mit fünf und vier Schnitten, der Hals aus drei, der Kopf ist ein Keil aus vier Schnitten mit Backe und Rüsselscheibe; die zwei Schinken bleiben als hängende Massen aus drei Schnitten, weil sie das Schwein in dieser Größe ausmachen, die Ohren sind dicke Blätter über dem Gesicht; jedes Bein besteht aus einer oberen Masse, einer Röhre und einem Keil für Fessel und Klauen. Schwanz, Augen und Afterklauen liegen unter dem Maßstab." },
    { en: "Thermal at 1:200: the body surfaces per region; the tail, which the body leaves out, is a small solid of its own at the rump so that the region stays present; one outlet at the nose, one at the mouth and the plume patch on the back.", de: "Thermik bei 1:200: die Körperoberflächen je Region; der Schwanz, den der Körper weglässt, ist ein eigener kleiner Körper an der Kruppe, damit die Region vorhanden bleibt; ein Auslass an der Nase, einer am Maul und das Rückenfeld für die Warmluftfahne." },
    { en: "Structure at 1:200: the load path as bars of the smallest drawable section: spine from skull to tail, sacrum with one hip bone per side, five rib hoops per side with the sternum, one bar per limb bone joined at hip, stifle, hock, shoulder, elbow and carpus, the two trunk masses.", de: "Tragwerk bei 1:200: der Lastpfad als Stäbe mit dem kleinsten zeichenbaren Querschnitt: Wirbelsäule vom Schädel bis zum Schwanz, Kreuzbein mit einem Hüftbein je Seite, fünf Rippenbögen je Seite mit dem Brustbein, ein Stab je Gliedmaßenknochen, verbunden an Hüfte, Knie, Sprunggelenk, Schulter, Ellbogen und Vorderfußwurzel, die zwei Rumpfmassen." },
  ],
  500: [
    { en: "Body at 1:500: 14 solids for a 75 mm feature; the model is minimal. The barrel is two lofts with the hams in the rump, a neck stub, the head as one wedge with jowl and disc in it, both lop ears as thick leaves over the face, and per leg an upper mass and a lower post down to the ground at the smallest drawable thickness.", de: "Körper bei 1:500: 14 Körper für 75 mm kleinste Abmessung; das Modell ist minimal. Die Tonne besteht aus zwei Lofts mit den Schinken in der Kruppe, einem Halsstummel, dem Kopf als ein Keil mit Backe und Scheibe darin, beiden Schlappohren als dicke Blätter über dem Gesicht, und je Bein einer oberen Masse und einem unteren Pfosten bis zum Boden in der kleinsten zeichenbaren Dicke." },
    { en: "Thermal at 1:500: the body surfaces per region; snout, tail and hooves, which the body merges, are small solids of their own at the size floor so that every region stays present; the outlets sit on the snout and on the back.", de: "Thermik bei 1:500: die Körperoberflächen je Region; Rüsselscheibe, Schwanz und Klauen, die der Körper zusammenfasst, sind eigene kleine Körper an der Größengrenze, damit jede Region vorhanden bleibt; die Auslässe sitzen an der Rüsselscheibe und auf dem Rücken." },
    { en: "Structure at 1:500: one bar per bone of the rig at the smallest drawable section along the real skeleton line (spine, pelvis, skull, limb bones, tail, ears) and the two trunk masses; ribs and joints are below the scale.", de: "Tragwerk bei 1:500: ein Stab je Knochen des Skeletts mit dem kleinsten zeichenbaren Querschnitt entlang der wirklichen Skelettlinie (Wirbelsäule, Becken, Schädel, Gliedmaßenknochen, Schwanz, Ohren) und die zwei Rumpfmassen; Rippen und Gelenke liegen unter dem Maßstab." },
  ],
};

const S = new Pig(SPEC, scale);
body(S);
const X = new Pig(SPEC, scale);
const th = thermal(S, X, { nostrilR: scale === 100 ? 0.0055 : 0.009, mouthR: 0.014, plume: [[0.42, 0.663, 0], [0.22, 0.13, 0.02]], pair: scale === 100, discS: 0.27, ...(scale === 200 ? { skip: /-hoof$/ } : {}) });
const Y = new Pig(SPEC, scale);
const q = 0.55 * S.feature;
if (scale === 200) {
  Y.run("th-tail", "tail-1", [{ p: [-0.1, 0.612, 0], rl: q, rs: q }, { p: [-0.15, 0.622, 0], rl: q, rs: q }], TAIL);
  for (const sd of SIDES) for (const [leg, h, z] of [["hind", "h", HIND_Z], ["fore", "f", FORE_Z]] as const) {
    const hoof = `hoof-${h}${sx(sd)}`;
    const toe = toeOf(Y, hoof);
    Y.run(`th-${side(sd)}-${leg}-pastern`, hoof, [{ p: Y.at(hoof, -0.012, 0, sd * z), rl: 0.0215, rs: 0.0195 }, { p: Y.at(hoof, 0.045, 0.002, sd * z), rl: 0.024, rs: 0.0185 }], LEG);
    Y.hull(`th-${side(sd)}-${leg}-claws`, hoof, [[-0.055, 0.0165, 0.0165, 0.024, 3, sd * z], [-0.03, 0.0175, 0.0175, 0.025, 3, sd * z], [-0.004, 0.009, 0.009, 0.018, 3, sd * z]].map((r) => [r[0]!, r[1]! + toe[1], r[2]!, r[3]!, r[4]!, r[5]!]), HORN, { x0: toe[0], sd: 1, floor: 0.006 });
  }
}
if (scale === 500) {
  Y.run("th-tail", "tail-1", [{ p: [-0.085, 0.6, 0], rl: q, rs: q }, { p: [-0.165, 0.6, 0], rl: q, rs: q }], TAIL);
  Y.run("th-snout", "head", [{ p: Y.at("head", 0.19, 0.005, 0), rl: q, rs: q }, { p: Y.at("head", 0.27, 0.005, 0), rl: q, rs: q }], DISC);
  for (const sd of SIDES) for (const h of ["h", "f"] as const) { const toe = toeOf(Y, `hoof-${h}${sx(sd)}`); const z = sd * (h === "h" ? HIND_Z : FORE_Z); Y.run(`th-${side(sd)}-${h === "h" ? "hind" : "fore"}-hoof`, `hoof-${h}${sx(sd)}`, [{ p: [toe[0] - 0.08, q, z], rl: q, rs: q }, { p: [toe[0], q, z], rl: q, rs: q }], HORN); }
}
for (const t of Y.body) th.push({ ...t, region: t.id === "th-tail" ? "tail" : t.id === "th-ears" ? "ears" : t.id === "th-snout" ? "snout" : /-pastern$/.test(t.id) ? "skin-legs" : "hooves", ...(/-pastern$/.test(t.id) ? { layer: "bristles" } : {}) });
const st = structure(new Pig(SPEC, scale));
console.log(`1:${scale} body ${S.body.length} thermal ${th.length} structure ${st.length}`);
if (process.argv.includes("--write")) {
  saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: D[scale]! });
  writeFileSync(`${OUT}/shapes.${scale}.json`, JSON.stringify(S.shapes));
}
