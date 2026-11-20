/** 🐑️ Round 3 sheep at 1:100, 1:200 and 1:500: each scale has its own hand tables (merged masses, fewer and simpler sections), never the finer list with parts removed.
 * Usage: bun r3-sheep-coarse.ts <100|200|500> [--write]
 * @see ./r3-sheep-50.ts
 * @see ./r3-animals-sheep.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Sheep, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-sheep-kit.ts";
import { FORE_Z, HIND_Z, LUMBAR_V, SPINE_C, SPINE_T, SPINOUS_T, STERNUM, ear, lumbarY, ribPath, ribX, spineY, spinous, thermal } from "./r3-sheep-50.ts";

const scale = Number(process.argv[2]) as 100 | 200 | 500;
const FLEECE: Look = { material: "fleece", tone: 0.3 };
const WOOL: Look = { material: "fleece", tone: 0.36 };
const HAIR: Look = { material: "short-hair", tone: 0.46 };
const FACE: Look = { material: "short-hair", tone: 0.42 };
const EAR: Look = { material: "bare-skin", tone: 0.5 };
const HORN: Look = { material: "hoof-horn", tone: 0.86 };
const BONE: Look = { material: "bone", tone: 0.18 };
const JOINT: Look = { material: "bone", tone: 0.5 };
const MASS: Look = { material: "fleece", tone: 0.7 };
const SIDES = [-1, 1] as const;
const len = (S: Sheep, bone: string): number => S.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
const toeOf = (S: Sheep, hoof: string): V3 => { const f = S.frame(hoof); const l = len(S, hoof); return [f.o[0]! + f.u[0]! * l, f.o[1]! + f.u[1]! * l, 0]; };

const T = {
  100: {
    hind: [[-0.15, 0.6, 0.03, 0.03, 2], [-0.12, 0.59, 0.13, 0.12, 2.1], [-0.04, 0.573, 0.182, 0.184, 2.25], [0.1, 0.552, 0.206, 0.204, 2.3], [0.28, 0.54, 0.216, 0.22, 2.35], [0.42, 0.541, 0.204, 0.207, 2.35]],
    chest: [[0.28, 0.541, 0.22, 0.224, 2.35], [0.42, 0.541, 0.218, 0.221, 2.35], [0.54, 0.551, 0.206, 0.207, 2.3], [0.64, 0.564, 0.192, 0.188, 2.25], [0.73, 0.584, 0.156, 0.145, 2.1], [0.79, 0.6, 0.09, 0.085, 2]],
    neck: [[-0.03, 0.03, 0.11, 0.1], [0.08, 0.034, 0.113, 0.097], [0.2, 0.014, 0.078, 0.07], [0.3, -0.004, 0.045, 0.046]],
    head: [[-0.015, 0.012, 0.045, 0.036], [0.03, 0.005, 0.075, 0.054], [0.075, 0.0, 0.08, 0.061], [0.125, 0.0, 0.062, 0.046], [0.18, -0.002, 0.045, 0.033], [0.225, -0.006, 0.03, 0.026], [0.242, -0.008, 0.017, 0.018]],
    ear: [[0, 0.013, 0.0085], [0.4, 0.027, 0.0085], [0.85, 0.018, 0.0085], [1, 0.009, 0.0085]],
    tail: [[-0.095, 0.662, 0.028, 0.026], [-0.13, 0.6, 0.04, 0.034], [-0.148, 0.52, 0.036, 0.03], [-0.151, 0.47, 0.016, 0.014]],
    gaskin: [[-0.04, 0, 0.052, 0.042, 0.12], [0.02, -0.002, 0.055, 0.044, 0.116], [0.1, -0.01, 0.04, 0.031, 0.108], [0.19, -0.016, 0.029, 0.02, 0.099], [0.236, -0.012, 0.024, 0.018, 0.096]],
    hindCannon: [[-0.02, -0.006, 0.026, 0.019, 0.097], [0.04, 0, 0.017, 0.014, 0.098], [0.14, 0, 0.0165, 0.014, 0.102], [0.175, -0.002, 0.021, 0.0185, 0.103], [0.192, -0.002, 0.018, 0.017, 0.103]],
    forearm: [[-0.04, -0.012, 0.04, 0.031, 0.107], [0.01, -0.008, 0.042, 0.033, 0.104], [0.09, -0.001, 0.029, 0.024, 0.097], [0.19, 0, 0.0215, 0.0195, 0.088]],
    foreCannon: [[-0.018, 0.001, 0.022, 0.0205, 0.088], [0.035, 0, 0.0155, 0.014, 0.088], [0.11, 0, 0.0155, 0.0145, 0.09], [0.135, -0.002, 0.02, 0.0185, 0.09], [0.152, -0.002, 0.0175, 0.017, 0.09]],
    pastern: [[-0.01, 0, 0.018, 0.019], [0.04, 0.001, 0.017, 0.021], [0.07, 0.002, 0.016, 0.023]],
    hoof: [[-0.055, 0.011, 0.011, 0.019, 3], [-0.033, 0.017, 0.017, 0.021, 3], [-0.013, 0.0105, 0.0105, 0.019, 3], [-0.002, 0.0085, 0.0085, 0.013, 3]],
  },
  200: {
    hind: [[-0.15, 0.6, 0.04, 0.04, 2], [-0.1, 0.588, 0.15, 0.14, 2.1], [0.02, 0.562, 0.196, 0.196, 2.3], [0.22, 0.542, 0.216, 0.22, 2.35], [0.4, 0.541, 0.202, 0.206, 2.3]],
    chest: [[0.3, 0.541, 0.218, 0.222, 2.3], [0.4, 0.541, 0.216, 0.22, 2.3], [0.52, 0.549, 0.208, 0.209, 2.3], [0.67, 0.57, 0.185, 0.178, 2.2], [0.78, 0.598, 0.1, 0.095, 2]],
    neck: [[-0.02, 0.03, 0.115, 0.1], [0.14, 0.024, 0.097, 0.085], [0.3, -0.004, 0.048, 0.048]],
    head: [[-0.01, 0.01, 0.05, 0.04], [0.06, 0.0, 0.08, 0.06], [0.16, -0.002, 0.05, 0.036], [0.24, -0.007, 0.02, 0.02]],
    ear: [[0, 0.017, 0.0165], [0.5, 0.026, 0.0165], [1, 0.017, 0.0165]],
    tail: [[-0.1, 0.66, 0.03, 0.028], [-0.145, 0.56, 0.04, 0.034], [-0.15, 0.475, 0.02, 0.018]],
    gaskin: [[-0.03, 0, 0.052, 0.042, 0.118], [0.08, -0.008, 0.043, 0.034, 0.11], [0.235, -0.012, 0.028, 0.0225, 0.097]],
    hindCannon: [[-0.02, -0.004, 0.024, 0.019, 0.097], [0.09, 0, 0.0165, 0.0165, 0.1], [0.185, -0.002, 0.019, 0.018, 0.103]],
    forearm: [[-0.035, -0.01, 0.04, 0.031, 0.106], [0.07, -0.002, 0.032, 0.026, 0.099], [0.19, 0, 0.0255, 0.0235, 0.088]],
    foreCannon: [[-0.018, 0.001, 0.022, 0.02, 0.088], [0.07, 0, 0.0165, 0.0165, 0.089], [0.148, -0.002, 0.019, 0.018, 0.09]],
    hoof: [[-0.085, 0.07, 0.0165, 0.018, 2.2], [-0.06, 0.05, 0.03, 0.02, 2.4], [-0.03, 0.026, 0.026, 0.022, 3], [-0.004, 0.0165, 0.0165, 0.018, 3]],
  },
  500: {
    hind: [[-0.162, 0.59, 0.09, 0.09, 2.1], [-0.11, 0.584, 0.15, 0.148, 2.2], [0.0, 0.565, 0.19, 0.19, 2.3], [0.22, 0.542, 0.214, 0.218, 2.3], [0.4, 0.541, 0.2, 0.204, 2.3]],
    chest: [[0.3, 0.541, 0.218, 0.222, 2.3], [0.4, 0.541, 0.216, 0.22, 2.3], [0.55, 0.552, 0.202, 0.202, 2.3], [0.68, 0.572, 0.17, 0.165, 2.2], [0.76, 0.592, 0.11, 0.105, 2.1]],
    ear: [[0, 0.0415, 0.0415], [0.55, 0.0415, 0.0415], [1.1, 0.0415, 0.0415]],
    neck: [[-0.02, 0.03, 0.118, 0.1], [0.3, -0.004, 0.06, 0.055]],
    head: [[-0.01, 0.008, 0.055, 0.045], [0.08, 0, 0.075, 0.058], [0.235, -0.005, 0.042, 0.042]],
    gaskin: [[-0.02, 0, 0.052, 0.046, 0.118], [0.23, -0.01, 0.047, 0.047, 0.1]],
    forearm: [[-0.03, -0.008, 0.05, 0.046, 0.104], [0.19, 0, 0.047, 0.047, 0.09]],
  },
} as const;

function body(S: Sheep): void {
  const t = T[scale] as Record<string, readonly (readonly number[])[]>;
  S.hull("hindbody", "lumbar", dense(t.hind!, 3), FLEECE, { shape: "hindbody" });
  S.hull("chest", "thorax", dense(t.chest!, 3), FLEECE, { shape: "chest" });
  S.limb("neck", "neck", dense(t.neck!, 3), FLEECE, { shape: "neck" });
  S.limb("head", "head", dense(t.head!, 3), FACE, { shape: "head", e: 2.4 });
  if (scale !== 500) S.run("tail", "tail-1", t.tail!.map(([x, y, rl, rs]) => ({ p: [x!, y!, 0] as V3, rl: rl!, rs: rs! })), WOOL, { shape: "tail" });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    if (scale !== 500) ear(S, `${n}-ear`, sd, EAR, 0.105, t.ear!, scale === 100 ? `ear-${k}` : sd < 0 ? "ear" : "ear-far");
    if (scale === 500) {
      ear(S, `${n}-ear`, sd, EAR, 0.105, t.ear!, "head");
      S.limb(`${n}-hind-upper`, `shank-h${k}`, t.gaskin!, WOOL, { sd, shape: `leg-hind-upper-${k}` });
      S.limb(`${n}-fore-upper`, `forearm-f${k}`, t.forearm!, WOOL, { sd, shape: `leg-fore-upper-${k}` });
      for (const [leg, bone, hoof, z] of [["hind", `metatarsus-h${k}`, `hoof-h${k}`, HIND_Z], ["fore", `metacarpus-f${k}`, `hoof-f${k}`, FORE_Z]] as const) {
        const toe = toeOf(S, hoof);
        const f = S.frame(hoof).o;
        S.run(`${n}-${leg}-lower`, bone, [{ p: S.at(bone, -0.01, 0, sd * z), rl: 0.041, rs: 0.041 }, { p: [f[0]!, f[1]!, sd * z], rl: 0.041, rs: 0.041 }, { p: [toe[0] - 0.041, toe[1] + 0.05, sd * z], rl: 0.041, rs: 0.041 }, { p: [toe[0] - 0.041, toe[1], sd * z], rl: 0.041, rs: 0.041 }], HAIR, { shape: `leg-${leg}-lower-${k}` });
      }
      continue;
    }
    S.limb(`${n}-gaskin`, `shank-h${k}`, t.gaskin!, WOOL, { sd, shape: `gaskin-${k}` });
    S.limb(`${n}-hind-cannon`, `metatarsus-h${k}`, t.hindCannon!, HAIR, { sd, shape: `cannon-h-${k}` });
    S.limb(`${n}-forearm`, `forearm-f${k}`, t.forearm!, WOOL, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, t.foreCannon!, HAIR, { sd, shape: `cannon-f-${k}` });
    for (const [leg, h, z] of [["hind", "h", HIND_Z], ["fore", "f", FORE_Z]] as const) {
      const hoof = `hoof-${h}${k}`;
      if (scale === 100) {
        S.limb(`${n}-${leg}-pastern`, hoof, t.pastern!.map((r) => [...r, z]), HAIR, { sd, shape: `hoof-${h}-${k}` });
        const toe = toeOf(S, hoof);
        S.hull(`${n}-${leg}-hoof`, hoof, t.hoof!.map((r) => [r[0]!, r[1]! + toe[1], r[2]!, r[3]!, r[4]!, sd * z]), HORN, { x0: toe[0], sd: 1, shape: `hoof-${h}-${k}` });
      } else { const toe = toeOf(S, hoof); S.hull(`${n}-${leg}-hoof`, hoof, t.hoof!.map((r) => [r[0]!, r[1]! + toe[1], r[2]!, r[3]!, r[4]!, sd * z]), HORN, { x0: toe[0], sd: 1, shape: `hoof-${h}-${k}` }); }
    }
  }
}

const CHILD: Record<string, string> = { thigh: "shank", shank: "metatarsus", metatarsus: "hoof", scapula: "humerus", humerus: "forearm", forearm: "metacarpus", metacarpus: "hoof" };
const JZ: Record<string, number> = { thigh: 0.088, shank: 0.115, metatarsus: 0.097, "hoof-h": 0.103, scapula: 0.05, humerus: 0.095, forearm: 0.105, metacarpus: 0.088, "hoof-f": 0.09 };

function structure(K: Sheep): Json[] {
  const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });
  const r = 0.55 * K.feature;
  const big = scale === 100 ? 1 : 0;
  const tube = (id: string, bone: string, pts: readonly V3[], rl = r, rs = r): Json => mem(K.run(id, bone, pts.map((p) => ({ p, rl: Math.max(r, rl), rs: Math.max(r, rs) })), BONE), bone);
  tube("lumbar-spine", "lumbar", [[0.105, 0.655, 0], [0.225, 0.664, 0], [0.345, 0.66, 0]], 0.03 * big, 0.019 * big);
  tube("thoracic-spine", "thorax", scale === 500 ? [SPINE_T[0]!, SPINE_T[4]!] : scale === 200 ? [SPINE_T[0]!, SPINE_T[2]!, SPINE_T[4]!] : [...SPINE_T], 0.015 * big, 0.015 * big);
  if (scale === 100) {
    for (const i of [0, 2, 4, 6, 8, 10, 12]) mem(spinous(K, `thoracic-spinous-${i + 1}`, "thorax", ribX(i), spineY(ribX(i)) + 0.01, SPINOUS_T[i]![0], SPINOUS_T[i]![1], r, r), "thorax");
    for (const i of [0, 2, 4]) mem(spinous(K, `lumbar-spinous-${i + 1}`, "lumbar", LUMBAR_V[i]![0], lumbarY(LUMBAR_V[i]![0]) + 0.012, LUMBAR_V[i]![1], -0.006, r, r), "lumbar");
  }
  tube("cervical-spine", "neck", scale === 500 ? [SPINE_C[0]!, SPINE_C[5]!] : scale === 200 ? [SPINE_C[0]!, SPINE_C[2]!, SPINE_C[5]!] : [SPINE_C[0]!, SPINE_C[1]!, SPINE_C[2]!, SPINE_C[3]!, SPINE_C[5]!], 0.02 * big, 0.016 * big);
  if (scale === 500) tube("pelvis-bone", "pelvis", [[-0.07, 0.61, 0], [0.105, 0.655, 0]], 0.08, r);
  else {
    tube("sacrum", "pelvis", [[0.105, 0.655, 0], [0.0, 0.667, 0], [-0.078, 0.662, 0]], 0.025 * big, 0.015 * big);
    for (const sd of SIDES) mem(K.run(`${side(sd)}-hip-bone`, "pelvis", [{ p: [0.115, 0.672, sd * 0.1], rl: r, rs: scale === 100 ? 0.028 : r }, { p: [0.0, 0.592, sd * 0.078], rl: r, rs: scale === 100 ? 0.016 : r }, { p: [-0.09, 0.6, sd * 0.07], rl: r, rs: scale === 100 ? 0.014 : r }], BONE), "pelvis");
  }
  if (scale === 100) {
    mem(K.limb("skull", "head", [[-0.01, 0.03, 0.03, 0.03], [0.04, 0.032, 0.043, 0.047], [0.1, 0.03, 0.038, 0.044], [0.17, 0.021, 0.027, 0.027], [0.23, 0.006, 0.013, 0.016]], BONE, { e: 2.4 }), "head");
    mem(K.limb("mandible", "jaw", [[0.034, -0.014, 0.022, 0.04], [0.06, -0.038, 0.026, 0.04], [0.14, -0.029, 0.013, 0.024], [0.218, -0.019, 0.0085, 0.013]], BONE, { frame: "head" }), "jaw");
  } else {
    mem(K.limb("skull", "head", [[-0.005, 0.028, 0.035, 0.035], [0.07, 0.03, 0.042, 0.048], [0.228, 0.008, 0.0165, 0.018]], BONE), "head");
    mem(K.run("mandible", "jaw", [{ p: K.at("head", 0.04, -0.01), rl: Math.max(r, 0.03), rs: r }, { p: K.at("head", 0.215, -0.022), rl: r, rs: r }], BONE), "jaw");
  }
  for (const [i, b] of ["tail-1", "tail-2"].entries()) tube(`caudal-${i + 1}`, b, [K.at(b, 0, 0), K.at(b, len(K, b), 0)]);
  if (scale !== 500) mem(K.run("sternum", "thorax", STERNUM.map((p, i) => ({ p, rl: i === 1 || i === 2 ? Math.max(r, 0.017) : r, rs: r })), BONE), "thorax");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    for (const i of scale === 100 ? [0, 2, 4, 6, 8, 10, 12] : scale === 200 ? [0, 3, 6, 9, 12] : []) mem(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, r), BONE), "thorax");
    const eb = K.at("head", 0.022, 0.04, sd * 0.043);
    if (scale === 500) mem(K.ball(`${n}-ear-cartilage`, `ear-${k}`, [eb[0], eb[1], sd * 0.05], [r, r, r], BONE), `ear-${k}`);
    else tube(`${n}-ear-cartilage`, `ear-${k}`, [eb, [eb[0] - 0.026, eb[1] - 0.012, eb[2] + sd * 0.094]]);
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
      if (scale === 500) mem(K.ball(`${n}-${leg}-digits`, hoof, [toe[0] - 0.036, toe[1] + r, sd * z], [r, r, r], BONE), hoof);
      else tube(`${n}-${leg}-digits`, hoof, [[f[0]!, f[1]!, sd * z], [toe[0] - 0.02 - r * 0.6, Math.max(r, toe[1] + 0.012), sd * z]]);
    }
    if (scale !== 500) for (const [stem, h, rr] of [["thigh", "h", 0.02], ["shank", "h", 0.023], ["metatarsus", "h", 0.019], ["humerus", "f", 0.021], ["forearm", "f", 0.02], ["metacarpus", "f", 0.019]] as const) {
      const bone = `${stem}-${h}${k}`;
      const o = K.frame(bone).o;
      const q = Math.max(rr, r * 1.25);
      mem(K.ball(`joint-${bone}`, bone, [o[0]!, o[1]!, sd * JZ[stem]!], [q, q, q], JOINT), bone, "joint");
    }
  }
  if (scale === 100) for (const b of ["lumbar", "thorax", "neck", "head"]) { const o = K.frame(b).o; mem(K.ball(`joint-${b}`, b, [o[0]!, o[1]!, 0], [0.02, 0.02, 0.02], JOINT), b, "joint"); }
  const m = Math.max(0.034, r);
  mem(K.ball("mass-thorax", "thorax", [0.5, 0.535, 0], [m, m, m], MASS), "thorax", "mass");
  mem(K.ball("mass-lumbar", "lumbar", [0.225, 0.565, 0], [m * 0.85, m * 0.85, m * 0.85], MASS), "lumbar", "mass");
  return K.body;
}

const D: Record<number, { en: string; de: string }[]> = {
  100: [
    { en: "Body at 1:100: 23 solids drawn anew for a 15 mm feature. The fleece trunk keeps its two lofts with rump, belly and brisket in six sections each; ham, shoulder, topknot, eyes, dewclaws and the point of hock are merged into their neighbours; the head is one wedge with the lower jaw in it; each leg is a wool gaskin or forearm, a cannon with fetlock, a pastern and one hoof for both claws; the tail is one piece.", de: "Körper bei 1:100: 23 Körper, neu gezeichnet für 15 mm kleinste Abmessung. Der Vliesrumpf behält seine zwei Lofts mit Kruppe, Bauch und Vorderbrust in je sechs Schnitten; Keule, Schulter, Schopf, Augen, Afterklauen und Fersenhöcker gehen in ihren Nachbarn auf; der Kopf ist ein Keil mit dem Unterkiefer darin; jedes Bein besteht aus wolligem Unterschenkel oder Unterarm, Röhre mit Fesselkopf, Fessel und einer Klaue für beide Zehen; der Schwanz ist ein Stück." },
    { en: "Thermal at 1:100: the body surfaces per region as at 1:50, with the merged parts in the region of the mass that took them; the nostrils, the lips and the plume patch on the back are outlets on the surface.", de: "Thermik bei 1:100: die Körperoberflächen je Region wie bei 1:50, die zusammengefassten Teile in der Region der Masse, die sie aufnahm; Nüstern, Lippen und das Rückenfeld der Warmluftfahne sind Auslässe auf der Oberfläche." },
    { en: "Structure at 1:100: the skeleton generalised: skull and mandible, cervical, thoracic and lumbar spine with a spinous process on every second vertebra (the long ones carry the withers), sacrum and one hip bone per side, seven rib hoops per side for the thirteen ribs, joined to the sternum,, one bar per long bone from joint to joint with the scapula as a blade, one bar for the two digits of a foot, caudal vertebrae and ear cartilages; joints at the limb joints and along the spine, the two trunk masses.", de: "Tragwerk bei 1:100: das Skelett verallgemeinert: Schädel und Unterkiefer, Hals-, Brust- und Lendenwirbelsäule mit einem Dornfortsatz an jedem zweiten Wirbel (die langen tragen den Widerrist), Kreuzbein und ein Hüftbein je Seite, sieben Rippenbögen je Seite für die dreizehn Rippen, am Brustbein verbunden, ein Stab je Röhrenknochen von Gelenk zu Gelenk mit dem Schulterblatt als Platte, ein Stab für die zwei Zehen eines Fußes, Schwanzwirbel und Ohrknorpel; Gelenke an den Beingelenken und entlang der Wirbelsäule, die zwei Rumpfmassen." },
  ],
  200: [
    { en: "Body at 1:200: 19 solids for a 30 mm feature. Trunk fleece in two lofts of five and four sections, neck in three, the head as a short wedge, both ears as thick leaves, the tail as one piece; each leg is an upper mass, a cannon and one wedge for pastern and hoof.", de: "Körper bei 1:200: 19 Körper für 30 mm kleinste Abmessung. Rumpfvlies in zwei Lofts mit fünf und vier Schnitten, Hals in drei, der Kopf als kurzer Keil, beide Ohren als dicke Blätter, der Schwanz als ein Stück; jedes Bein besteht aus oberer Masse, Röhre und einem Keil für Fessel und Klaue." },
    { en: "Thermal at 1:200: the body surfaces per region; pastern and hoof are one body solid, so the thermal list splits them into a hair-covered pastern and the horn of the hoof; one outlet at the nose, one at the mouth and the plume patch on the back.", de: "Thermik bei 1:200: die Körperoberflächen je Region; Fessel und Klaue sind ein Körper, daher teilt die Thermikliste sie in behaarte Fessel und Klauenhorn; ein Auslass an der Nase, einer am Maul und das Rückenfeld der Warmluftfahne." },
    { en: "Structure at 1:200: the load path as bars of the smallest drawable section: spine from skull to tail, sacrum with one hip bone per side, five rib hoops per side with the sternum, one bar per limb bone joined at hip, stifle, hock, shoulder, elbow and knee, the two trunk masses.", de: "Tragwerk bei 1:200: der Lastpfad als Stäbe mit dem kleinsten zeichenbaren Querschnitt: Wirbelsäule vom Schädel bis zum Schwanz, Kreuzbein mit einem Hüftbein je Seite, fünf Rippenbögen je Seite mit dem Brustbein, ein Stab je Beinknochen, verbunden an Hüfte, Knie, Sprunggelenk, Schulter, Ellbogen und Vorderfußwurzel, die zwei Rumpfmassen." },
  ],
  500: [
    { en: "Body at 1:500: 14 solids for a 75 mm feature; the model is minimal. Trunk fleece in two lofts with one clean seam, a neck stub, the head as one wedge with the jaw in it, both ears as stubs at the smallest drawable thickness, the tail in the rump, and per leg an upper mass and a lower post down to the ground at the smallest drawable thickness.", de: "Körper bei 1:500: 14 Körper für 75 mm kleinste Abmessung; das Modell ist minimal. Rumpfvlies in zwei Lofts mit einer sauberen Naht, ein Halsstumpf, der Kopf als ein Keil mit dem Kiefer darin, beide Ohren als Stummel in der kleinsten zeichenbaren Dicke, der Schwanz in der Kruppe und je Bein eine obere Masse und ein unterer Pfosten bis zum Boden in der kleinsten zeichenbaren Dicke." },
    { en: "Thermal at 1:500: the body surfaces per region; tail and hooves, which the body merges, are small solids of their own at the size floor so that every region stays present; the outlets sit on the muzzle and on the back.", de: "Thermik bei 1:500: die Körperoberflächen je Region; Schwanz und Klauen, die der Körper zusammenfasst, sind eigene kleine Körper an der Größengrenze, damit jede Region vorhanden bleibt; die Auslässe sitzen am Maul und auf dem Rücken." },
    { en: "Structure at 1:500: one bar per bone of the rig at the smallest drawable section along the real skeleton line (spine, pelvis, skull, limb bones, tail, ears) and the two trunk masses; ribs and joints are below the scale.", de: "Tragwerk bei 1:500: ein Stab je Knochen des Skeletts mit dem kleinsten zeichenbaren Querschnitt entlang der wirklichen Skelettlinie (Wirbelsäule, Becken, Schädel, Beinknochen, Schwanz, Ohren) und die zwei Rumpfmassen; Rippen und Gelenke liegen unter dem Maßstab." },
  ],
};

const S = new Sheep(SPEC, scale);
body(S);
const X = new Sheep(SPEC, scale);
const th = thermal(S, X, { nostrilR: 0.0055, mouthR: 0.011, plume: [[0.44, 0.705, 0], [0.24, 0.15, scale === 500 ? 0.06 : 0.058]], pair: scale === 100, ...(scale === 200 ? { skip: /-hoof$/ } : {}) });
if (scale === 200) {
  const Y = new Sheep(SPEC, scale);
  for (const sd of SIDES) for (const [leg, h, z] of [["hind", "h", HIND_Z], ["fore", "f", FORE_Z]] as const) {
    const hoof = `hoof-${h}${sx(sd)}`;
    const toe = toeOf(Y, hoof);
    Y.run(`th-${side(sd)}-${leg}-pastern`, hoof, [{ p: Y.at(hoof, -0.012, 0, sd * z), rl: 0.018, rs: 0.018 }, { p: Y.at(hoof, 0.062, 0.002, sd * z), rl: 0.0185, rs: 0.0165 }], HAIR);
    Y.hull(`th-${side(sd)}-${leg}-hoof`, hoof, [[-0.057, 0.0165, 0.0165, 0.021, 3, sd * z], [-0.03, 0.018, 0.018, 0.022, 3, sd * z], [-0.004, 0.0165, 0.0165, 0.0175, 3, sd * z]].map((r) => [r[0]!, r[1]! + toe[1], r[2]!, r[3]!, r[4]!, r[5]!]), HORN, { x0: toe[0], sd: 1 });
  }
  for (const t of Y.body) th.push(/-pastern$/.test(t.id) ? { ...t, region: "lower-legs", layer: "hair" } : { ...t, region: "hooves" });
}
if (scale === 500) {
  const Y = new Sheep(SPEC, scale);
  const q = 0.0413;
  Y.run("th-tail", "tail-1", [{ p: [-0.122, 0.612, 0], rl: q, rs: q }, { p: [-0.138, 0.525, 0], rl: q, rs: q }], WOOL);
  for (const sd of SIDES) for (const h of ["h", "f"] as const) { const toe = toeOf(Y, `hoof-${h}${sx(sd)}`); const z = sd * (h === "h" ? HIND_Z : FORE_Z); Y.run(`th-${side(sd)}-${h === "h" ? "hind" : "fore"}-hoof`, `hoof-${h}${sx(sd)}`, [{ p: [toe[0] - 0.079, toe[1] + q, z], rl: q, rs: q }, { p: [toe[0] + 0.004, toe[1] + q, z], rl: q, rs: q }], HORN); }
  for (const t of Y.body) th.push({ ...t, region: t.id === "th-tail" ? "tail" : t.id === "th-ears" ? "ears" : "hooves", ...(t.id === "th-tail" ? { layer: "wool" } : {}) });
}
const st = structure(new Sheep(SPEC, scale));
console.log(`1:${scale} body ${S.body.length} thermal ${th.length} structure ${st.length}`);
if (process.argv.includes("--write")) {
  saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: D[scale]! });
  writeFileSync(`${OUT}/shapes.${scale}.json`, JSON.stringify(S.shapes));
}
