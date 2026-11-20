/** 🦊️ Round 4 red fox at 1:500: the digital-only site scale (no print drawing exists, so no print floor thickens the parts). Its own hand tables: every mass at its real size with the fewest sections that still give the fox (level back, deep narrow chest, tucked flank, wedge head, leaf ears, slim legs, bushy brush), and the skeleton as real slender bones.
 * Usage: bun r4-red-fox-500.ts [--write]
 * @see ./r3-red-fox-coarse.ts
 * @see ./r3-animals-red-fox.md */
import { saveLists } from "./kit-lib.ts";
import { SPEC, Fox, side, sx, type Json, type Look, type V3 } from "./r3-red-fox-kit.ts";
import { BONE, DARK, FORE_Z, HIND_Z, JOINT, JOINT_Z, MASS, PALE, RED, SIDES, SPINE_C, SPINE_L, SPINE_T, STERNUM, REGION, ear, mem, ribPath } from "./r3-red-fox-50.ts";

type Rows = readonly (readonly number[])[];
const FLOOR = 0.004;
/** Trunk in planes x = const: x, centre height, half height, half width, exponent. */
const HIND: Rows = [[-0.052, 0.387, 0.024, 0.024, 2], [-0.015, 0.375, 0.061, 0.058, 2.2], [0.06, 0.364, 0.074, 0.066, 2.2], [0.14, 0.357, 0.081, 0.06, 2.2], [0.2125, 0.346, 0.091, 0.068, 2.3], [0.255, 0.341, 0.082, 0.058, 2.3]];
const CHEST: Rows = [[0.2125, 0.346, 0.091, 0.068, 2.3], [0.3, 0.335, 0.103, 0.075, 2.3], [0.385, 0.332, 0.108, 0.079, 2.3], [0.435, 0.341, 0.088, 0.058, 2.2], [0.468, 0.355, 0.032, 0.018, 2]];
/** Limb rows: distance along the bone, sagittal offset, sagittal half, lateral half, lateral position. */
const NECK: Rows = [[-0.052, -0.004, 0.06, 0.045], [0.0, -0.004, 0.069, 0.055], [0.08, -0.004, 0.048, 0.044], [0.158, 0.001, 0.031, 0.033]];
const HEAD: Rows = [[-0.012, 0.008, 0.02, 0.022], [0.022, 0.006, 0.04, 0.049], [0.068, 0.003, 0.031, 0.039], [0.1, -0.005, 0.021, 0.02], [0.154, -0.008, 0.009, 0.009]];
const EAR: Rows = [[0, 0.021, 0.007], [0.35, 0.027, 0.006], [1, 0.004, 0.004]];
const ROOT: Rows = [[0.0, -0.012, 0.022, 0.022], [0.07, -0.006, 0.043, 0.041], [0.15, -0.004, 0.055, 0.053], [0.172, -0.004, 0.045, 0.043]];
const MID: Rows = [[-0.02, -0.004, 0.036, 0.034], [0.01, -0.004, 0.057, 0.055], [0.08, -0.004, 0.059, 0.057], [0.162, -0.002, 0.044, 0.042]];
const TIP: Rows = [[-0.02, -0.002, 0.034, 0.032], [0.01, -0.002, 0.053, 0.051], [0.075, 0, 0.041, 0.039], [0.146, 0, 0.006, 0.006]];
const THIGH: Rows = [[-0.026, -0.012, 0.034, 0.02, 0.037], [0.02, -0.012, 0.055, 0.033, 0.042], [0.09, -0.006, 0.038, 0.027, 0.044], [0.144, 0.004, 0.018, 0.014, 0.045]];
const GASKIN: Rows = [[-0.02, -0.004, 0.026, 0.018, 0.046], [0.03, -0.008, 0.026, 0.019, 0.044], [0.095, -0.006, 0.016, 0.0125, 0.04], [0.146, -0.005, 0.0135, 0.0115, 0.038]];
const HIND_CANNON: Rows = [[-0.016, -0.008, 0.012, 0.01, 0.038], [0.02, -0.001, 0.012, 0.0105, 0.038], [0.094, 0.002, 0.0115, 0.0135, 0.036]];
const PAW: Rows = [[-0.008, -0.002, 0.012, 0.0115], [0.016, -0.006, 0.0185, 0.018], [0.05, 0.004, 0.006, 0.014]];
const FOREARM: Rows = [[-0.075, -0.006, 0.024, 0.016, 0.044], [-0.02, -0.008, 0.024, 0.017, 0.042], [0.005, -0.003, 0.02, 0.0145, 0.04], [0.06, 0, 0.014, 0.012, 0.036], [0.125, 0, 0.012, 0.0115, 0.033]];
const FORE_CANNON: Rows = [[-0.012, 0, 0.011, 0.0105, 0.033], [0.03, 0, 0.0115, 0.012, 0.032], [0.071, 0.002, 0.012, 0.015, 0.032]];

function body(S: Fox): void {
  const o = { floor: FLOOR };
  S.hull("hindbody", "lumbar", HIND, RED, o);
  S.hull("chest", "thorax", CHEST, RED, o);
  S.limb("neck", "neck", NECK, RED, { ...o, e: 2.2 });
  S.limb("head", "head", HEAD, RED, { ...o, e: 2.4 });
  S.limb("brush-root", "tail-1", ROOT, RED, { ...o, e: 2.1 });
  S.limb("brush-mid", "tail-2", MID, RED, { ...o, e: 2.1 });
  S.limb("brush-tip", "tail-3", TIP, PALE, { ...o, e: 2.1 });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    ear(S, `${n}-ear`, sd, DARK, 0.092, EAR);
    S.limb(`${n}-thigh`, `thigh-h${k}`, THIGH, RED, { ...o, sd });
    S.limb(`${n}-gaskin`, `shank-h${k}`, GASKIN, DARK, { ...o, sd });
    S.limb(`${n}-hind-cannon`, `metatarsus-h${k}`, HIND_CANNON, DARK, { ...o, sd });
    S.limb(`${n}-hind-paw`, `paw-h${k}`, PAW.map((r) => [...r, HIND_Z]), DARK, { ...o, sd, e: 2.5 });
    S.limb(`${n}-forearm`, `forearm-f${k}`, FOREARM, DARK, { ...o, sd });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, FORE_CANNON, DARK, { ...o, sd });
    S.limb(`${n}-fore-paw`, `paw-f${k}`, PAW.map((r) => [r[0], r[1], r[2], r[3] * 1.06, FORE_Z]), DARK, { ...o, sd, e: 2.5 });
  }
}

/** 🌡️ Thermal list at 1:500: every body solid as the fur surface of its region, and the three outlets as small patches at their real size on the surface (nostrils, lips, back strip). */
function thermal(S: Fox, T: Fox): Json[] {
  const out: Json[] = S.body.map((b) => ({ id: `th-${b.id}`, of: b.id, region: REGION.find(([re]) => re.test(b.id))![1], layer: "fur" }));
  const E: Look = { material: "dark-fur", tone: 0.95, role: "detail" };
  const patch = (id: string, bone: string, rows: readonly (readonly [number, number, number, number])[], look: Look, region: string, tag: string): void => {
    out.push({ ...T.run(id, bone, rows.map(([s, v, rl, rs]) => ({ p: T.at(bone, s, v, 0), rl, rs, e: 2 })), look, { floor: 0.002 }), region, tags: ["emitter", tag] });
  };
  patch("th-breath-nose", "head", [[0.15, -0.007, 0.004, 0.003], [0.154, -0.007, 0.0075, 0.006], [0.158, -0.007, 0.004, 0.003]], E, "face", "breath-nose");
  patch("th-breath-mouth", "head", [[0.118, -0.018, 0.006, 0.002], [0.132, -0.017, 0.011, 0.003], [0.146, -0.015, 0.005, 0.002]], E, "face", "breath-mouth");
  patch("th-plume-back", "thorax", [[-0.02, 0.029, 0.012, 0.003], [0.03, 0.0405, 0.028, 0.005], [0.09, 0.054, 0.028, 0.005], [0.14, 0.0655, 0.012, 0.003]], { material: "red-fur", tone: 0.5 }, "coat-trunk", "plume-back");
  return out;
}

/** 🦴️ Skeleton at 1:500: every bone one member at its real thickness (no print floor), three ribs per side, joints at the rig joints, the two trunk masses. */
function structure(K: Fox): Json[] {
  const o = { floor: 0.003 };
  const len = (bone: string): number => K.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
  const bar = (id: string, bone: string, z: number, sd: number, r0: number, rm: number, r1: number, short = 0): Json => mem(K.run(id, bone, [{ p: K.at(bone, 0, 0, sd * z), rl: r0, rs: r0 }, { p: K.at(bone, len(bone) * 0.5, 0, sd * z), rl: rm, rs: rm }, { p: K.at(bone, len(bone) - short, 0, sd * z), rl: r1, rs: r1 }], BONE, o), bone);
  const knob = (id: string, bone: string, c: V3, r: number, look: Look, part: string): Json => mem(K.run(id, bone, ([[-1, 0.3], [-0.7, 0.72], [0, 1], [0.7, 0.72], [1, 0.3]] as const).map(([t, k]) => ({ p: [c[0] + t * r, c[1], c[2]] as V3, rl: k * r, rs: k * r, e: 2 })), look, { floor: 0.002 }), bone, part);
  mem(K.limb("skull", "head", [[-0.002, 0.012, 0.016, 0.018], [0.03, 0.015, 0.0225, 0.0245], [0.075, 0.008, 0.016, 0.016], [0.148, -0.001, 0.007, 0.008]], BONE, { e: 2.3, ...o }), "head");
  mem(K.limb("mandible", "jaw", [[0.03, -0.012, 0.008, 0.026], [0.075, -0.021, 0.006, 0.019], [0.136, -0.018, 0.004, 0.006]], BONE, { frame: "head", ...o }), "jaw");
  mem(K.run("cervical-spine", "neck", [SPINE_C[0]!, SPINE_C[2]!, SPINE_C[4]!, SPINE_C[5]!].map((p) => ({ p, rl: 0.011, rs: 0.0085 })), BONE, o), "neck");
  mem(K.run("thoracic-spine", "thorax", [{ p: SPINE_T[0]!, rl: 0.008, rs: 0.009 }, { p: [0.38, 0.384, 0] as V3, rl: 0.006, rs: 0.019 }, { p: [0.3, 0.398, 0] as V3, rl: 0.006, rs: 0.013 }, { p: SPINE_T[3]!, rl: 0.008, rs: 0.009 }], BONE, o), "thorax");
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: 0.02, rs: 0.008 })), BONE, o), "lumbar");
  mem(K.run("pelvis-girdle", "pelvis", [{ p: [0.072, 0.408, 0], rl: 0.037, rs: 0.013 }, { p: [0.03, 0.398, 0], rl: 0.031, rs: 0.011 }, { p: [0.0, 0.378, 0], rl: 0.035, rs: 0.01 }, { p: [-0.048, 0.374, 0], rl: 0.037, rs: 0.009 }], BONE, o), "pelvis");
  mem(K.run("sternum", "thorax", [STERNUM[0]!, STERNUM[1]!, STERNUM[2]!, STERNUM[4]!], BONE, o), "thorax");
  for (const [i, b] of ["tail-1", "tail-2", "tail-3"].entries()) mem(K.run(`caudal-${i + 1}`, b, [{ p: K.at(b, -0.004, 0), rl: 0.006 - i * 0.001, rs: 0.006 - i * 0.001 }, { p: K.at(b, len(b), 0), rl: 0.005 - i * 0.001, rs: 0.005 - i * 0.001 }], BONE, o), b);
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.086, [[0.02, 0.014, 0.004], [0.35, 0.02, 0.004], [1, 0.004, 0.004]]), `ear-${k}`);
    for (const i of [1, 5, 9]) mem(Object.assign(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, 0.003), BONE, { floor: 0.003 }), { slender: true }), "thorax");
    bar(`${n}-femur`, `thigh-h${k}`, 0.04, sd, 0.01, 0.0065, 0.0095);
    bar(`${n}-tibia`, `shank-h${k}`, 0.042, sd, 0.0095, 0.0055, 0.007);
    bar(`${n}-metatarsals`, `metatarsus-h${k}`, 0.037, sd, 0.0075, 0.006, 0.007);
    bar(`${n}-hind-digits`, `paw-h${k}`, HIND_Z, sd, 0.006, 0.005, 0.004, 0.004);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, [[-0.003, 0, 0.024, 0.004, 0.03], [0.05, 0, 0.017, 0.004, 0.04], [0.094, 0, 0.009, 0.006, 0.045]], BONE, { sd, e: 2.6, ...o }), `scapula-f${k}`);
    bar(`${n}-humerus`, `humerus-f${k}`, 0.044, sd, 0.0095, 0.006, 0.009);
    bar(`${n}-radius-ulna`, `forearm-f${k}`, 0.037, sd, 0.008, 0.0058, 0.0072);
    bar(`${n}-metacarpals`, `metacarpus-f${k}`, 0.033, sd, 0.007, 0.0055, 0.0062);
    bar(`${n}-fore-digits`, `paw-f${k}`, FORE_Z, sd, 0.006, 0.005, 0.004, 0.004);
  }
  const R: Record<string, number> = { head: 0.008, thigh: 0.009, shank: 0.0095, metatarsus: 0.0075, humerus: 0.009, forearm: 0.008, metacarpus: 0.007 };
  for (const b of K.c.raw.rig.bones as Json[]) {
    const id = b.id as string;
    const stem = id.replace(/-[hf]?[nf]$/, "");
    if (R[stem] === undefined) continue;
    const sd = /-(h|f)?n$/.test(id) ? -1 : /-(h|f)?f$/.test(id) ? 1 : 0;
    const at = K.frame(id).o;
    knob(`joint-${id}`, id, [at[0]!, at[1]!, sd * (JOINT_Z[stem] ?? 0)], R[stem]!, JOINT, "joint");
  }
  knob("mass-thorax", "thorax", [0.33, 0.325, 0], 0.024, MASS, "mass");
  knob("mass-lumbar", "lumbar", [0.14, 0.355, 0], 0.019, MASS, "mass");
  return K.body;
}

const DECISIONS = [
  { en: "Body at 1:500 (digital only, the print drawing of this scale is unavailable): 23 solids at their real size with the fewest hand sections that still give the fox. Trunk in two lofts of six and five sections with level back, deep narrow chest and tucked flank; one thigh mass per side on the trunk, the shoulders merged into the chest, which widens there, with the forearm reaching up into it; neck with ruff; head as one wedge with the pointed muzzle; ears as thin leaves; gaskin, forearm, cannon and paw of every leg at their real 22 to 30 mm; brush in three links that plug into each other, with the pale tip. No eyes, nose pad, toes or cheek ruffs.", de: "Körper bei 1:500 (nur digital, die Druckzeichnung dieses Maßstabs entfällt): 23 Körper in wirklicher Größe mit den wenigsten von Hand gesetzten Schnitten, die den Fuchs noch ergeben. Rumpf in zwei Lofts aus sechs und fünf Schnitten mit geradem Rücken, tiefer schmaler Brust und aufgezogener Flanke; je Seite eine Keule auf dem Rumpf, die Schultern in die Brust eingeschmolzen, die dort breiter wird und in die der Unterarm hinaufreicht; Hals mit Kragen; Kopf als ein Keil mit spitzem Fang; Ohren als dünne Blätter; Unterschenkel, Unterarm, Röhre und Pfote jedes Beins in ihren wirklichen 22 bis 30 mm; Rute in drei ineinander gesteckten Gliedern mit heller Spitze. Keine Augen, kein Nasenspiegel, keine Zehen, keine Backen." },
  { en: "Thermal at 1:500: each region is the coat surface of its body solids (single fur layer, references): trunk coat with neck and thighs; leg coat; face; ears; brush; paws. The outlets lie on the surface as one nostril patch, the lips and the back strip for the plume, each stated for this scale.", de: "Thermik bei 1:500: jede Region ist die Felloberfläche ihrer Körper (eine Fellschicht, Verweise): Rumpffell mit Hals und Keulen; Beinfell; Gesicht; Ohren; Rute; Pfoten. Die Auslässe liegen auf der Oberfläche als ein Nasenfleck, die Lippen und der Rückenstreifen für die Fahne, jeweils für diesen Maßstab gesetzt." },
  { en: "Structure at 1:500: the skeleton as real slender bones, since no print floor applies: skull, mandible, cervical spine entering the chest low, thoracic spine under the topline with the fin of the withers, lumbar spine, pelvic girdle, sternum, three ribs per side as slender solids, caudal vertebrae, ear cartilages, scapula plate and one tapered member per long bone, metapodial and digit bundle of every leg; joints at head, hip, stifle, hock, shoulder, elbow and carpus, the masses of thorax and loin at their centres. No supports are declared.", de: "Tragwerk bei 1:500: das Skelett als wirklich schlanke Knochen, da keine Druckgrenze gilt: Schädel, Unterkiefer, tief in die Brust eintretende Halswirbelsäule, Brustwirbelsäule unter der Rückenlinie mit dem Kamm des Widerrists, Lendenwirbelsäule, Beckengürtel, Brustbein, drei Rippen je Seite als schlanke Körper, Schwanzwirbel, Ohrknorpel, Schulterblattplatte und ein verjüngter Stab je Röhrenknochen, Mittelfuß- und Zehenbündel jedes Beins; Gelenke an Kopf, Hüfte, Knie, Sprunggelenk, Schulter, Ellbogen und Vorderfußwurzel, die Massen von Brust und Lende in ihren Schwerpunkten. Es sind keine Auflager deklariert." },
];

if (import.meta.main) {
  const S = new Fox(SPEC, 500);
  body(S);
  const th = thermal(S, new Fox(SPEC, 500));
  const st = structure(new Fox(SPEC, 500));
  console.log(`1:500 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS });
}
