/** 🦊️ Round 3 red fox at 1:100 and 1:200: each scale has its own hand tables (merged masses, fewer and simpler sections, parts at the size floor of the scale), never the 1:50 list with parts removed. The 1:500 print representation is unavailable (its representation says why); the digital 1:500 model is in r4-red-fox-500.ts.
 * Usage: bun r3-red-fox-coarse.ts <100|200> [--write]   (1:500: r4-red-fox-500.ts)
 * @see ./r3-red-fox-50.ts
 * @see ./r3-animals-red-fox.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Fox, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-red-fox-kit.ts";
import { BONE, DARK, FORE_Z, HIND_Z, PALE, RED, SIDES, SPINE_C, SPINE_L, SPINE_T, STERNUM, ear, jointsAndMasses, mem, ribPath, spineY, thermal } from "./r3-red-fox-50.ts";

type Rows = readonly (readonly number[])[];
type Tables = {
  trunk: Rows; split: number; tuck: Rows; neck: Rows; head: Rows; ear: Rows; root: Rows; mid: Rows; tip: Rows;
  thigh?: Rows; gaskin: Rows; hindCannon: Rows; paw: Rows; forearm: Rows; foreCannon: Rows; shoulder?: boolean; brush: readonly [string, string, string]; ears: readonly [string, string];
};

const T100: Tables = {
  trunk: [[-0.052, 0.388, 0.02, 0.02, 2], [-0.03, 0.381, 0.052, 0.05, 2.1], [0.0, 0.372, 0.064, 0.066, 2.2], [0.06, 0.364, 0.074, 0.066, 2.2], [0.14, 0.357, 0.081, 0.06, 2.2], [0.2125, 0.346, 0.091, 0.068, 2.3], [0.29, 0.336, 0.101, 0.075, 2.3], [0.37, 0.332, 0.108, 0.071, 2.3], [0.42, 0.337, 0.097, 0.058, 2.2], [0.455, 0.348, 0.066, 0.036, 2.1], [0.47, 0.356, 0.03, 0.017, 2]],
  split: 0.2125,
  tuck: [[0.252, 0.341, 0.084, 0.06, 2.3]],
  neck: [[-0.05, -0.004, 0.062, 0.046], [0.0, -0.004, 0.069, 0.055], [0.06, -0.004, 0.052, 0.046], [0.12, -0.002, 0.041, 0.04], [0.158, 0.002, 0.03, 0.033]],
  head: [[-0.012, 0.008, 0.022, 0.024], [0.01, 0.006, 0.038, 0.046], [0.04, 0.005, 0.04, 0.05], [0.068, 0.003, 0.031, 0.04], [0.09, -0.003, 0.0235, 0.026], [0.125, -0.007, 0.019, 0.0175], [0.15, -0.008, 0.0135, 0.013], [0.156, -0.008, 0.0085, 0.0085]],
  ear: [[0, 0.02, 0.0085], [0.3, 0.027, 0.0083], [0.7, 0.016, 0.0083], [1, 0.0083, 0.0083]],
  root: [[0.0, -0.012, 0.022, 0.022], [0.05, -0.007, 0.038, 0.037], [0.15, -0.004, 0.055, 0.053], [0.172, -0.004, 0.046, 0.044]],
  mid: [[-0.02, -0.004, 0.036, 0.034], [0.01, -0.004, 0.057, 0.055], [0.07, -0.004, 0.059, 0.057], [0.14, -0.002, 0.054, 0.052], [0.162, -0.002, 0.045, 0.043]],
  tip: [[-0.02, -0.002, 0.034, 0.032], [0.01, -0.002, 0.053, 0.051], [0.06, 0, 0.046, 0.044], [0.11, 0, 0.028, 0.027], [0.14, 0, 0.0105, 0.0105]],
  thigh: [[-0.025, -0.012, 0.035, 0.02, 0.038], [0.02, -0.012, 0.055, 0.033, 0.042], [0.075, -0.008, 0.045, 0.03, 0.044], [0.12, -0.001, 0.028, 0.021, 0.045], [0.145, 0.004, 0.016, 0.013, 0.045]],
  gaskin: [[-0.02, -0.004, 0.026, 0.018, 0.046], [0.02, -0.008, 0.028, 0.02, 0.045], [0.08, -0.006, 0.018, 0.0135, 0.041], [0.145, -0.005, 0.0145, 0.012, 0.038]],
  hindCannon: [[-0.02, -0.012, 0.01, 0.0085, 0.038], [0.0, -0.004, 0.015, 0.011, 0.038], [0.03, 0, 0.011, 0.0105, 0.037], [0.092, 0.002, 0.012, 0.014, 0.036]],
  paw: [[-0.008, -0.002, 0.012, 0.0115], [0.012, -0.0055, 0.0185, 0.017], [0.03, -0.003, 0.0145, 0.019], [0.046, 0.0055, 0.0105, 0.016], [0.054, 0.0105, 0.0083, 0.011]],
  forearm: [[-0.022, -0.011, 0.014, 0.0095, 0.04], [0.0, -0.004, 0.021, 0.0145, 0.04], [0.05, 0, 0.0155, 0.0125, 0.037], [0.124, 0, 0.0125, 0.012, 0.033]],
  foreCannon: [[-0.012, 0, 0.011, 0.0105, 0.033], [0.0, -0.002, 0.0125, 0.0115, 0.033], [0.035, 0, 0.0115, 0.0125, 0.032], [0.07, 0.002, 0.012, 0.015, 0.032]],
  shoulder: true,
  brush: ["brush", "brush-2", "brush-tip"],
  ears: ["ear-n", "ear-f"],
};

const T200: Tables = {
  trunk: [[-0.05, 0.386, 0.03, 0.03, 2], [-0.02, 0.376, 0.058, 0.056, 2.2], [0.03, 0.366, 0.07, 0.066, 2.2], [0.14, 0.357, 0.081, 0.061, 2.2], [0.2125, 0.346, 0.091, 0.069, 2.3], [0.3, 0.335, 0.103, 0.075, 2.3], [0.38, 0.332, 0.108, 0.071, 2.3], [0.43, 0.34, 0.09, 0.055, 2.2], [0.465, 0.353, 0.04, 0.022, 2]],
  split: 0.2125,
  tuck: [[0.255, 0.34, 0.082, 0.058, 2.3]],
  neck: [[-0.05, -0.004, 0.062, 0.048], [0.0, -0.004, 0.068, 0.055], [0.07, -0.004, 0.05, 0.045], [0.155, 0.0, 0.033, 0.035]],
  head: [[-0.01, 0.007, 0.024, 0.026], [0.02, 0.006, 0.039, 0.048], [0.06, 0.003, 0.034, 0.043], [0.1, -0.005, 0.022, 0.022], [0.152, -0.008, 0.0165, 0.0165]],
  ear: [[0, 0.024, 0.0165], [0.35, 0.027, 0.0165], [0.75, 0.016, 0.011], [1, 0.005, 0.005]],
  root: [[0.0, -0.012, 0.024, 0.024], [0.15, -0.004, 0.055, 0.053], [0.17, -0.004, 0.046, 0.044]],
  mid: [[-0.02, -0.004, 0.036, 0.034], [0.01, -0.004, 0.057, 0.055], [0.07, -0.004, 0.059, 0.057], [0.14, -0.002, 0.054, 0.052], [0.16, -0.002, 0.045, 0.043]],
  tip: [[-0.02, -0.002, 0.034, 0.032], [0.01, -0.002, 0.053, 0.051], [0.07, 0, 0.042, 0.04], [0.14, 0, 0.008, 0.008]],
  thigh: [[-0.022, -0.012, 0.038, 0.02, 0.04], [0.03, -0.012, 0.054, 0.032, 0.043], [0.09, -0.006, 0.038, 0.027, 0.044], [0.14, 0.003, 0.02, 0.0165, 0.045]],
  gaskin: [[-0.02, -0.006, 0.028, 0.019, 0.046], [0.03, -0.008, 0.026, 0.019, 0.045], [0.09, -0.006, 0.0175, 0.0165, 0.041], [0.145, -0.005, 0.0165, 0.0165, 0.038]],
  hindCannon: [[-0.015, -0.006, 0.0165, 0.0165, 0.038], [0.09, 0.002, 0.0165, 0.0165, 0.036]],
  paw: [[-0.006, -0.002, 0.0165, 0.0165], [0.02, -0.003, 0.02, 0.02], [0.05, 0.015, 0.0165, 0.0165]],
  forearm: [[-0.02, -0.008, 0.022, 0.0165, 0.04], [0.03, -0.002, 0.021, 0.0165, 0.038], [0.124, 0, 0.0185, 0.0165, 0.034]],
  foreCannon: [[-0.01, 0, 0.0165, 0.0165, 0.034], [0.068, 0.002, 0.0165, 0.0165, 0.033]],
  shoulder: true,
  brush: ["brush", "brush-2", "brush-tip"],
  ears: ["ear", "ear-far"],
};

function body(S: Fox, t: Tables, smooth: boolean): void {
  const sm = (rows: Rows): Rows => (smooth ? dense(rows, 2) : rows);
  const trunk = sm(t.trunk);
  const hind = [...trunk.filter((r) => r[0]! <= t.split + 1e-9), ...t.tuck];
  const chest = trunk.filter((r) => r[0]! >= t.split - 1e-9);
  S.hull("hindbody", "lumbar", hind, RED, { shape: "hindbody" });
  S.hull("chest", "thorax", chest, RED, { shape: "chest" });
  S.limb("neck", "neck", sm(t.neck), RED, { shape: "neck", e: 2.2 });
  S.limb("head", "head", sm(t.head), RED, { shape: "head", e: 2.4 });
  S.limb("brush-root", "tail-1", t.root, RED, { shape: t.brush[0], e: 2.1 });
  S.limb("brush-mid", "tail-2", t.mid, RED, { shape: t.brush[1], e: 2.1 });
  S.limb("brush-tip", "tail-3", t.tip, PALE, { shape: t.brush[2], e: 2.1 });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    ear(S, `${n}-ear`, sd, DARK, 0.092, t.ear, t.ears[sd < 0 ? 0 : 1]);
    if (t.thigh !== undefined) S.limb(`${n}-thigh`, `thigh-h${k}`, t.thigh, RED, { sd, shape: sd < 0 ? "thigh-n" : "hindbody" });
    S.limb(`${n}-gaskin`, `shank-h${k}`, t.gaskin, DARK, { sd, shape: `gaskin-${k}` });
    S.limb(`${n}-hind-cannon`, `metatarsus-h${k}`, t.hindCannon, DARK, { sd, shape: `cannon-h-${k}` });
    S.limb(`${n}-hind-paw`, `paw-h${k}`, t.paw.map((r) => [...r, HIND_Z]), DARK, { sd, shape: `paw-h-${k}`, e: 2.5 });
    if (t.shoulder === true) {
      const sc = `scapula-f${k}`;
      const hu = `humerus-f${k}`;
      const pts: PP[] = [
        { p: S.at(sc, 0.005, -0.004, sd * 0.034), rl: 0.013, rs: 0.02 },
        { p: S.at(sc, 0.05, -0.002, sd * 0.045), rl: 0.023, rs: 0.035 },
        { p: S.at(hu, 0.01, -0.004, sd * 0.047), rl: 0.023, rs: 0.035 },
        { p: S.at(hu, 0.065, -0.01, sd * 0.045), rl: 0.021, rs: 0.033 },
        { p: S.at(hu, 0.118, -0.006, sd * 0.042), rl: 0.015, rs: 0.021 },
      ];
      S.run(`${n}-shoulder`, hu, pts, RED, { shape: "chest" });
    }
    S.limb(`${n}-forearm`, `forearm-f${k}`, t.forearm, DARK, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, t.foreCannon, DARK, { sd, shape: `cannon-f-${k}` });
    S.limb(`${n}-fore-paw`, `paw-f${k}`, t.paw.map((r) => [r[0], r[1], r[2], r[3] * 1.06, FORE_Z]), DARK, { sd, shape: `paw-f-${k}`, e: 2.5 });
  }
}

/** 🦴️ Skeleton of a coarse scale: `ribs` lists the rib numbers kept per side (slender, at their real size), every other bone is one hand-stated member at no less than the size floor `fl`. */
export function structure(K: Fox, fl: number, ribs: readonly number[], fine: boolean): Json[] {
  const r = (x: number): number => Math.max(fl, x);
  const bar = (id: string, bone: string, len: number, z: number, sd: number, r0: number, r1: number, v0 = 0): Json => mem(K.run(id, bone, [{ p: K.at(bone, 0, v0, sd * z), rl: r(r0), rs: r(r0) }, { p: K.at(bone, len * 0.5, 0, sd * z), rl: r(Math.min(r0, r1) * 0.8), rs: r(Math.min(r0, r1) * 0.8) }, { p: K.at(bone, len, 0, sd * z), rl: r(r1), rs: r(r1) }], BONE), bone);
  const len = (bone: string): number => K.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
  mem(K.limb("skull", "head", fine ? [[-0.004, 0.004, 0.02, 0.016], [0.02, 0.004, 0.032, 0.026], [0.055, 0.002, 0.03, 0.03], [0.09, -0.004, 0.02, 0.016], [0.146, -0.007, 0.0115, 0.0095]] : [[0.0, 0.004, 0.024, 0.02], [0.04, 0.003, 0.033, 0.03], [0.09, -0.004, 0.022, 0.0175], [0.145, -0.007, 0.0165, 0.0165]], BONE, { e: 2.3, floor: fl }), "head");
  mem(K.run("cervical-spine", "neck", [SPINE_C[0]!, SPINE_C[2]!, SPINE_C[4]!, SPINE_C[5]!].map((p) => ({ p, rl: r(0.011), rs: r(0.0085) })), BONE), "neck");
  mem(K.run("thoracic-spine", "thorax", SPINE_T.map((p) => ({ p, rl: r(0.008), rs: r(0.007) })), BONE), "thorax");
  if (fine) mem(K.run("thoracic-spinous", "thorax", ([[0.4, 0.012], [0.38, 0.019], [0.32, 0.013], [0.23, 0.0085]] as const).map(([x, h]) => ({ p: [x, spineY(x) + 0.006 + h, 0] as V3, rl: fl, rs: h })), BONE), "thorax");
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: r(fine ? 0.02 : 0.022), rs: r(0.008) })), BONE), "lumbar");
  mem(K.run("pelvis-girdle", "pelvis", [{ p: [0.072, 0.408, 0], rl: 0.038, rs: r(0.014) }, { p: [0.03, 0.398, 0], rl: 0.032, rs: r(0.012) }, { p: [0.0, 0.378, 0], rl: 0.036, rs: r(0.011) }, { p: [-0.048, 0.374, 0], rl: 0.038, rs: r(0.011) }], BONE), "pelvis");
  if (fine) mem(K.run("sternum", "thorax", STERNUM.filter((_, i) => i !== 3).map((q) => ({ p: q.p, rl: r(q.rl), rs: r(q.rs) })), BONE), "thorax");
  else mem(Object.assign(K.run("sternum", "thorax", STERNUM.filter((_, i) => i !== 3).map((q) => ({ p: q.p, rl: 0.0035, rs: 0.0035 })), BONE, { floor: 0.0025 }), { slender: true }), "thorax");
  for (const [i, b] of ["tail-1", "tail-2", "tail-3"].entries()) mem(K.run(`caudal-${i + 1}`, b, [{ p: K.at(b, -0.004, 0), rl: r(0.006), rs: r(0.006) }, { p: K.at(b, len(b), 0), rl: r(0.0045), rs: r(0.0045) }], BONE), b);
  mem(K.limb("mandible", "jaw", fine ? [[0.03, -0.018, 0.0083, 0.026], [0.07, -0.022, 0.0083, 0.02], [0.136, -0.019, 0.0083, 0.0083]] : [[0.03, -0.02, 0.0165, 0.024], [0.13, -0.02, 0.0165, 0.0165]], BONE, { frame: "head", floor: fl }), "jaw");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.086, [[0.02, r(0.014), fl], [0.4, r(0.02), fl], [0.78, r(0.011), 0.65 * fl], [1, 0.005, 0.005]]), `ear-${k}`);
    for (const i of ribs) mem(Object.assign(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, i === 0 ? 0.0018 : 0.0025), BONE, { floor: 0.0018 }), { slender: true }), "thorax");
    bar(`${n}-femur`, `thigh-h${k}`, len(`thigh-h${k}`), 0.04, sd, 0.0095, 0.0095);
    bar(`${n}-tibia`, `shank-h${k}`, len(`shank-h${k}`), 0.042, sd, 0.0095, 0.007);
    bar(`${n}-metatarsals`, `metatarsus-h${k}`, len(`metatarsus-h${k}`), 0.037, sd, 0.0075, 0.007, fine ? -0.006 : 0);
    bar(`${n}-hind-digits`, `paw-h${k}`, len(`paw-h${k}`) - (fine ? 0.004 : 0.012), HIND_Z, sd, 0.006, 0.0045, fl > 0.01 ? 0.004 : 0);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, [[-0.002, 0, 0.022, fl, 0.03], [0.045, 0, 0.018, fl, 0.039], [0.094, 0, r(0.0095), fl, 0.045]], BONE, { sd, e: 2.6, floor: fl }), `scapula-f${k}`);
    bar(`${n}-humerus`, `humerus-f${k}`, len(`humerus-f${k}`), 0.044, sd, 0.0095, 0.009);
    bar(`${n}-radius-ulna`, `forearm-f${k}`, len(`forearm-f${k}`), 0.037, sd, 0.008, 0.0075);
    bar(`${n}-metacarpals`, `metacarpus-f${k}`, len(`metacarpus-f${k}`), 0.033, sd, 0.007, 0.0065);
    bar(`${n}-fore-digits`, `paw-f${k}`, len(`paw-f${k}`) - (fine ? 0.004 : 0.012), FORE_Z, sd, 0.006, 0.0045, fl > 0.01 ? 0.004 : 0);
  }
  jointsAndMasses(K, 1, fine ? [0.027, 0.021] : [0.026, 0.02]);
  return K.body;
}

const DECISIONS: Record<number, { en: string; de: string }[]> = {
  100: [
    { en: "Body at 1:100: 25 solids with their own sections for this scale (smallest feature 15 mm). Trunk in two lofts with level back, deep chest and tucked flank; thigh per side; shoulder blade and upper arm merged into one bent shoulder mass per side; neck with ruff; skull, cheeks and mandible merged into one wedge head, no eyes or nose pad; ears as leaves at the 15 mm floor; brush in three links with the pale tip; per leg gaskin or forearm, cannon with the point of hock merged in, and a paw as one pad mass without toes.", de: "Körper bei 1:100: 25 Körper mit eigenen Schnitten für diesen Maßstab (kleinstes Merkmal 15 mm). Rumpf in zwei Lofts mit geradem Rücken, tiefer Brust und aufgezogener Flanke; Keule je Seite; Schulterblatt und Oberarm zu einer geknickten Schultermasse je Seite zusammengefasst; Hals mit Krause; Schädel, Backen und Unterkiefer zu einem Keilkopf zusammengefasst, ohne Augen und Nasenspiegel; Ohren als Blätter an der 15-mm-Grenze; Rute in drei Gliedern mit heller Spitze; je Bein Unterschenkel oder Unterarm, Röhre mit eingeschmolzenem Fersenhöcker und eine Pfote als eine Ballenmasse ohne Zehen." },
    { en: "Thermal at 1:100: each region is the coat surface of its body solids (single fur layer, references): trunk coat with neck, thighs and shoulders; leg coat; face; ears; brush; paws. Outlets on the surface: one nostril patch on the nose, the lips, the back strip for the plume.", de: "Thermik bei 1:100: jede Region ist die Felloberfläche ihrer Körper (eine Fellschicht, Verweise): Rumpffell mit Hals, Keulen und Schultern; Beinfell; Gesicht; Ohren; Rute; Pfoten. Auslässe auf der Oberfläche: ein Nasenlochfeld an der Nase, die Lippen, der Rückenstreifen für die Warmluftfahne." },
    { en: "Structure at 1:100: the skeleton simplified: skull, mandible, cervical, thoracic and lumbar spine, the fin of the withers, one pelvic girdle, sternum, seven of the thirteen ribs per side as slender solids at real size, caudal vertebrae, ear cartilages, scapula plate and one member per long bone, metapodial bundle and digit bundle of every leg; joints at the rig joints, masses of thorax and loin at their centres of mass. No supports are declared.", de: "Tragwerk bei 1:100: das Skelett vereinfacht: Schädel, Unterkiefer, Hals-, Brust- und Lendenwirbelsäule, der Kamm des Widerrists, ein Beckengürtel, Brustbein, sieben der dreizehn Rippen je Seite als schlanke Körper in wirklicher Größe, Schwanzwirbel, Ohrknorpel, Schulterblattplatte und je ein Stab pro Röhrenknochen, Mittelfuß- und Zehenbündel jedes Beins; Gelenke an den Gelenken des Skeletts, Massen von Brust und Lende in ihren Schwerpunkten. Auflager sind nicht deklariert." },
  ],
  200: [
    { en: "Body at 1:200: 25 solids with their own sections (smallest feature 30 mm). Trunk in two lofts with level back, deep chest and tucked flank; one thigh mass and one bent shoulder mass per side lie on the trunk and turn with their bones, so the trot stays closed; gaskin, forearm, cannons and paws stand at the 30 mm floor, so the legs are drawn stouter than the real 22 mm; head as one wedge; ears as leaves that are 33 mm thick at the base and taper to the tip; brush in three links that plug into each other, with the pale tip.", de: "Körper bei 1:200: 25 Körper mit eigenen Schnitten (kleinstes Merkmal 30 mm). Rumpf in zwei Lofts mit geradem Rücken, tiefer Brust und aufgezogener Flanke; je Seite liegen eine Keule und eine geknickte Schultermasse auf dem Rumpf und drehen mit ihren Knochen, der Trab bleibt so geschlossen; Unterschenkel, Unterarm, Röhren und Pfoten stehen an der 30-mm-Grenze, die Beine sind daher kräftiger als die wirklichen 22 mm gezeichnet; Kopf als ein Keil; Ohren als Blätter, am Ansatz 33 mm dick und zur Spitze verjüngt; Rute in drei ineinander gesteckten Gliedern mit heller Spitze." },
    { en: "Thermal at 1:200: each region is the coat surface of its body solids (single fur layer, references); trunk coat is trunk and neck, leg coat is gaskins, forearms and cannons. Outlets on the surface at the size floor: nose, lips, back strip.", de: "Thermik bei 1:200: jede Region ist die Felloberfläche ihrer Körper (eine Fellschicht, Verweise); Rumpffell sind Rumpf und Hals, Beinfell sind Unterschenkel, Unterarme und Röhren. Auslässe auf der Oberfläche an der Größengrenze: Nase, Lippen, Rückenstreifen." },
    { en: "Structure at 1:200: the load path as beams at the 30 mm floor: skull, mandible, cervical, thoracic and lumbar spine, one pelvic girdle, the sternum and four ribs per side at their real size as slender solids, caudal vertebrae, ear cartilages, scapula and one bar per leg bone; joints and the two trunk masses. No supports are declared.", de: "Tragwerk bei 1:200: der Lastpfad als Stäbe an der 30-mm-Grenze: Schädel, Unterkiefer, Hals-, Brust- und Lendenwirbelsäule, ein Beckengürtel, Brustbein, vier Rippen je Seite als schlanke Körper, Schwanzwirbel, Ohrknorpel, Schulterblatt und je ein Stab pro Beinknochen; Gelenke und die beiden Rumpfmassen. Auflager sind nicht deklariert." },
  ],
};

if (import.meta.main) {
  const scale = Number(process.argv[2]);
  const t = scale === 100 ? T100 : T200;
  const S = new Fox(SPEC, scale);
  body(S, t, scale === 100);
  const fl = 0.56 * S.feature;
  const th = thermal(S, new Fox(SPEC, scale), { nostrilR: fl, mouthR: fl, plume: [[0.235, scale === 100 ? 0.432 : 0.428, 0], [0.1, 0.03, scale === 100 ? 0.009 : 0.0168]], pair: false });
  const st = structure(new Fox(SPEC, scale), fl, scale === 100 ? [0, 2, 4, 6, 8, 10, 12] : [1, 4, 7, 10], scale === 100);
  console.log(`1:${scale} body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS[scale]! });
    writeFileSync(`${OUT}/shapes.${scale}.json`, JSON.stringify(S.shapes));
  }
}
