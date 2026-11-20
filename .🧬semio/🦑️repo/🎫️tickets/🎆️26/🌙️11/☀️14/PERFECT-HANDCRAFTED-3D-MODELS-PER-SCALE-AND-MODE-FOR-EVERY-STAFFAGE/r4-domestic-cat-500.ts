/** 🐈️ Round 4 domestic cat at 1:500: the digital-only site scale (the print drawing of this scale is unavailable, so no print floor thickens the parts). Its own hand tables: every mass at its real size with the fewest sections that still give the cat (round head with pricked ears, short neck, long supple trunk with the deep flank, slim legs with small feet, long tail), and the skeleton as real slender bones.
 * Usage: bun r4-domestic-cat-500.ts [--write]
 * @see ./r3-domestic-cat-coarse.ts
 * @see ./r3-animals-domestic-cat.md */
import { saveLists } from "./kit-lib.ts";
import { BONE, COAT, EAR, EYE, FACE, JOINT, LIMB, MASS, NOSE, PAD, TONGUE, ear, mem, ribPath, thermal } from "./r3-domestic-cat-50.ts";
import { Cat, SIDES, side, sx, type Json, type Look, type PP, type Row, type V3 } from "./r3-domestic-cat-kit.ts";

/** Trunk in planes x = const: x, centre height, half height, half width, exponent. The two pieces cross at x = 0.19: each is full on its own side and continues 7 mm smaller inside the other. */
const HIND: readonly Row[] = [[-0.045, 0.225, 0.018, 0.02, 2], [-0.015, 0.218, 0.044, 0.043, 2.1], [0.05, 0.2065, 0.06, 0.052, 2.2], [0.13, 0.196, 0.07, 0.061, 2.2], [0.19, 0.1905, 0.0712, 0.062, 2.2], [0.215, 0.19, 0.064, 0.053, 2.2]];
const CHEST: readonly Row[] = [[0.165, 0.193, 0.064, 0.055, 2.2], [0.19, 0.1905, 0.0712, 0.062, 2.2], [0.225, 0.1885, 0.07, 0.057, 2.3], [0.268, 0.189, 0.063, 0.048, 2.3], [0.3, 0.198, 0.046, 0.036, 2.2], [0.317, 0.209, 0.022, 0.018, 2]];
/** Limb rows: distance along the bone, sagittal offset, sagittal half, lateral half, lateral position. */
const NECK: readonly Row[] = [[-0.03, 0.015, 0.024, 0.024], [0.005, 0.013, 0.045, 0.04], [0.05, 0.007, 0.04, 0.034], [0.086, 0.003, 0.014, 0.014]];
const HEAD: readonly Row[] = [[-0.011, 0.013, 0.012, 0.014], [0.01, 0.01, 0.035, 0.038], [0.038, 0.004, 0.04, 0.0405], [0.063, -0.004, 0.03, 0.031], [0.083, -0.01, 0.014, 0.018], [0.09, -0.011, 0.006, 0.009]];
const EAR_LEAF: readonly Row[] = [[0, 0.021, 0.0035], [0.55, 0.014, 0.003], [1, 0.003, 0.003]];
const THIGH: readonly Row[] = [[-0.024, -0.008, 0.012, 0.008, 0.031], [0.005, -0.009, 0.038, 0.018, 0.034], [0.05, -0.01, 0.04, 0.02, 0.038], [0.112, 0.002, 0.008, 0.008, 0.042]];
const SHANK: readonly Row[] = [[-0.012, 0.002, 0.01, 0.009, 0.042], [0.012, -0.004, 0.022, 0.0155, 0.042], [0.07, -0.006, 0.0145, 0.012, 0.037], [0.115, -0.004, 0.006, 0.006, 0.034]];
const FOREARM: readonly Row[] = [[-0.015, -0.01, 0.007, 0.008, 0.04], [0.008, -0.002, 0.017, 0.0135, 0.0395], [0.06, 0, 0.012, 0.012, 0.037], [0.094, -0.002, 0.006, 0.008, 0.034]];
const FOOT_R = 0.0105;

/** 🥾️ Lower end of a leg as one round piece at its real 21 mm: metapodium from the joint (with the heel `heel` behind it) down to the ground and forward to the toes. */
function foot(S: Cat, id: string, bone: string, pawBone: string, heel: readonly [number, number, number] | null): Json {
  const z = S.frame(bone).o[2]!;
  const len = S.bone(bone).lengthM as number;
  const tip = S.tipOf(pawBone);
  const mtp = S.frame(pawBone).o as V3;
  const r = FOOT_R;
  const pts: PP[] = [
    ...(heel === null ? [] : [{ p: S.at(bone, heel[0], heel[1], z), rl: heel[2], rs: heel[2] }]),
    { p: S.at(bone, -0.004, -0.003, z), rl: r * 0.75, rs: r * 0.75 },
    { p: S.at(bone, len * 0.5, 0, z), rl: r * 0.95, rs: r * 0.85 },
    { p: S.at(bone, len * 0.88, 0.003, z), rl: r, rs: r * 0.9 },
    { p: [mtp[0] + 0.01, r + 0.0005, z], rl: r * 1.1, rs: r * 0.95 },
    { p: [tip[0] - 0.008, r * 0.85, z], rl: r * 1.05, rs: r * 0.8 },
    { p: [tip[0] + 0.003, r * 0.6, z], rl: r * 0.4, rs: r * 0.35 },
  ];
  return S.run(id, bone, pts, LIMB, { floor: 0.002 });
}

/** 🪢️ Tail piece through world points, round and tapering, closed by a small section at each end. */
function tail(S: Cat, id: string, bone: string, pts: readonly V3[], r: readonly number[]): Json {
  const n = pts.length - 1;
  const out = (a: V3, b: V3, k: number): V3 => [a[0] + (a[0] - b[0]) * k, a[1] + (a[1] - b[1]) * k, 0];
  return S.run(id, bone, [{ p: out(pts[0]!, pts[1]!, 0.12), rl: r[0]! * 0.5, rs: r[0]! * 0.5, e: 2 }, ...pts.map((p, i) => ({ p, rl: r[i]!, rs: r[i]!, e: 2 })), { p: out(pts[n]!, pts[n - 1]!, 0.12), rl: r[n]! * 0.5, rs: r[n]! * 0.5, e: 2 }], COAT, { floor: 0.002 });
}

function body(S: Cat): void {
  S.hull("hindbody", "lumbar", HIND, COAT);
  S.hull("chest", "thorax-b", CHEST, COAT);
  S.limb("neck", "neck", NECK, COAT, { e: 2.1 });
  S.limb("head", "head", HEAD, FACE, { e: 2.5 });
  const o = (b: string): V3 => { const p = S.frame(b).o as V3; return [p[0], p[1], 0]; };
  const t1 = o("tail-1");
  tail(S, "tail-a", "tail-2", [[t1[0] + 0.014, t1[1] - 0.002, 0], o("tail-2"), o("tail-3"), o("tail-4")], [0.0135, 0.0125, 0.0115, 0.0105]);
  tail(S, "tail-b", "tail-4", [o("tail-4"), o("tail-5"), S.tipOf("tail-5")], [0.0105, 0.0095, 0.008]);
  for (const sd of SIDES) {
    const k = sx(sd);
    ear(S, `ear-${k}`, sd, EAR, { rows: EAR_LEAF, flat: true });
    S.limb(`thigh-h${k}`, `thigh-h${k}`, THIGH, LIMB, { sd });
    S.limb(`lower-h${k}`, `shank-h${k}`, SHANK, LIMB, { sd });
    foot(S, `foot-h${k}`, `metatarsus-h${k}`, `paw-h${k}`, [-0.019, -0.01, 0.005]);
    const sc = `scapula-f${k}`;
    const hu = `humerus-f${k}`;
    S.run(`upper-f${k}`, hu, [
      { p: S.at(sc, -0.004, 0, sd * 0.02), rl: 0.005, rs: 0.007 },
      { p: S.at(sc, 0.03, -0.002, sd * 0.029), rl: 0.014, rs: 0.025 },
      { p: S.at(hu, 0.02, -0.008, sd * 0.038), rl: 0.015, rs: 0.026 },
      { p: S.at(hu, 0.07, -0.006, sd * 0.039), rl: 0.014, rs: 0.02 },
      { p: S.at(hu, 0.1, -0.003, sd * 0.04), rl: 0.007, rs: 0.007 },
    ], LIMB, { floor: 0.002 });
    S.limb(`lower-f${k}`, `forearm-f${k}`, FOREARM, LIMB, { sd });
    foot(S, `foot-f${k}`, `metacarpus-f${k}`, `paw-f${k}`, null);
  }
}

/** Regions the body of this scale does not show, as own small thermal solids at their real size: tongue, nose leather, eyes, one sole per foot. */
function extras(T: Cat): void {
  const hu = T.frame("head").u;
  const hv = T.frame("head").v;
  T.ball("tongue", "head", T.at("head", 0.047, -0.019, 0), [0.016, 0.0075, 0.004], TONGUE, [hu[0]!, hu[1]!, 0]);
  T.ball("nose", "head", T.at("head", 0.0865, -0.0105, 0), [0.0045, 0.006, 0.005], NOSE, [hu[0]!, hu[1]!, 0]);
  for (const sd of SIDES) {
    const k = sx(sd);
    T.ball(`eye-${k}`, "head", T.at("head", 0.064, 0.012, sd * 0.0195), [0.0035, 0.0048, 0.006], EYE, [hu[0]! * 0.62 + hv[0]! * 0.45, hu[1]! * 0.62 + hv[1]! * 0.45, sd * 0.64], [hv[0]!, hv[1]!, 0]);
    for (const [leg, bone] of [["h", `metatarsus-h${k}`], ["f", `metacarpus-f${k}`]] as const) {
      const tip = T.tipOf(`paw-${leg}${k}`);
      T.ball(`sole-${leg}${k}`, bone, [tip[0] - 0.014, 0.004, T.frame(bone).o[2]!], [0.014, 0.0105, 0.004], PAD, [1, 0, 0]);
    }
  }
}

const B: Record<string, readonly Row[]> = {
  cranium: [[-0.008, 0.016, 0.012, 0.014], [0.02, 0.015, 0.021, 0.025], [0.055, 0.005, 0.015, 0.019], [0.086, -0.006, 0.005, 0.006]],
  jaw: [[0.012, -0.008, 0.008, 0.02], [0.04, -0.022, 0.006, 0.015], [0.0765, -0.024, 0.003, 0.004]],
  scapula: [[-0.004, 0, 0.016, 0.003, 0.018], [0.03, 0, 0.015, 0.003, 0.028], [0.066, 0, 0.007, 0.006, 0.037]],
  humerus: [[0, 0, 0.0085, 0.008, 0.037], [0.045, 0, 0.0055, 0.0055, 0.039], [0.091, 0, 0.008, 0.0085, 0.04]],
  radius: [[-0.012, -0.008, 0.0045, 0.0045, 0.04], [0.002, -0.001, 0.0075, 0.007, 0.04], [0.045, 0, 0.0055, 0.006, 0.038], [0.085, 0, 0.0065, 0.008, 0.034]],
  metacarpals: [[-0.003, -0.001, 0.007, 0.009, 0.034], [0.041, 0, 0.005, 0.0115, 0.034]],
  femur: [[-0.004, 0, 0.008, 0.008, 0.03], [0.05, 0, 0.006, 0.006, 0.039], [0.099, 0.001, 0.009, 0.0095, 0.042]],
  tibia: [[0, 0, 0.009, 0.0095, 0.042], [0.055, 0, 0.0052, 0.0055, 0.038], [0.105, 0, 0.006, 0.007, 0.034]],
  metatarsals: [[-0.018, -0.01, 0.0042, 0.0042, 0.034], [0.002, -0.001, 0.0075, 0.008, 0.034], [0.07, 0, 0.0046, 0.0105, 0.034]],
  digits: [[0.0, 0, 0.004, 0.0115, 0.034], [0.027, 0, 0.003, 0.012, 0.034]],
};

/** 🦴️ Skeleton at 1:500: every bone one member at its real thickness, the vertebral column under the topline, two ribs per side, sternum, joints at head, hips, stifles, shoulders and elbows, the four trunk masses. */
function structure(K: Cat): Json[] {
  const o = (b: string): V3 => K.frame(b).o as V3;
  const col = (id: string, bone: string, pts: readonly (readonly [number, number, number, number])[]): Json => mem(K.run(id, bone, pts.map(([x, y, rl, rs]) => ({ p: [x, y, 0] as V3, rl, rs })), BONE, { floor: 0.002 }), bone);
  mem(K.limb("skull", "head", B.cranium!, BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", B.jaw!, BONE, { frame: "head", e: 2.3 }), "jaw");
  col("cervical-spine", "neck", [[0.305, 0.208, 0.01, 0.008], [0.326, 0.228, 0.01, 0.008], [0.3365, 0.27, 0.011, 0.008]]);
  col("thoracic-spine-b", "thorax-b", [[0.245, 0.236, 0.008, 0.008], [0.278, 0.2285, 0.006, 0.013], [0.305, 0.208, 0.008, 0.008]]);
  col("thoracic-spine-a", "thorax-a", [[0.178, 0.246, 0.008, 0.008], [0.245, 0.236, 0.008, 0.008]]);
  col("lumbar-spine", "lumbar", [[0.045, 0.25, 0.011, 0.008], [0.11, 0.2535, 0.013, 0.008], [0.178, 0.246, 0.011, 0.008]]);
  col("pelvis", "pelvis", [[0.05, 0.254, 0.026, 0.008], [0.0, 0.237, 0.033, 0.009], [-0.034, 0.23, 0.029, 0.007]]);
  col("sternum", "thorax-b", [[0.304, 0.168, 0.006, 0.006], [0.262, 0.142, 0.007, 0.005], [0.212, 0.136, 0.005, 0.004]]);
  ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"].forEach((b, i) => mem(K.tubeDome(`caudal-${b.slice(5)}`, b, [o(b)[0], o(b)[1], 0], K.tipOf(b), 0.0048 - i * 0.0004, 0.0044 - i * 0.0004, BONE, { n: 1 }), b));
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    for (const i of [3, 8]) mem(K.run(`${n}-rib-${i + 1}`, i < 5 ? "thorax-b" : "thorax-a", ribPath(i, sd, 0.002), BONE, { floor: 0.001 }), i < 5 ? "thorax-b" : "thorax-a").slender = true;
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, { len: 0.048, rows: [[0.04, 0.014, 0.003], [1, 0.003, 0.003]], flat: true }), `ear-${k}`);
    for (const [name, bone, rows, e] of [["scapula", `scapula-f${k}`, B.scapula!, 2.6], ["humerus", `humerus-f${k}`, B.humerus!, 2.2], ["radius-ulna", `forearm-f${k}`, B.radius!, 2.2], ["metacarpals", `metacarpus-f${k}`, B.metacarpals!, 2.6], ["fore-digits", `paw-f${k}`, B.digits!, 2.8], ["femur", `thigh-h${k}`, B.femur!, 2.2], ["tibia", `shank-h${k}`, B.tibia!, 2.2], ["metatarsals", `metatarsus-h${k}`, B.metatarsals!, 2.6], ["hind-digits", `paw-h${k}`, B.digits!, 2.8]] as const) mem(K.limb(`${n}-${name}`, bone, rows, BONE, { sd, e }), bone);
  }
  const R: Record<string, number> = { head: 0.006, thigh: 0.0075, shank: 0.008, humerus: 0.0075, forearm: 0.007 };
  for (const b of K.c.raw.rig.bones as Json[]) {
    const r = R[(b.id as string).split("-")[0]!];
    if (r !== undefined) mem(K.ball(`joint-${b.id}`, b.id, [...o(b.id)] as V3, [r, r, r], JOINT), b.id, "joint");
  }
  for (const [b, at] of [["pelvis", [0.005, 0.228, 0]], ["lumbar", [0.112, 0.212, 0]], ["thorax-a", [0.21, 0.198, 0]], ["thorax-b", [0.272, 0.193, 0]]] as const) mem(K.ball(`mass-${b}`, b, [...at] as V3, [0.012, 0.012, 0.012], MASS), b, "mass");
  return K.body;
}

const DECISIONS = [
  { en: "Body at 1:500 (digital only, the print drawing of this scale is unavailable): 20 solids at their real size with the fewest hand sections that still give the cat. Trunk in two lofts of six sections each that cross at the deepest point of the flank; short neck; round head as one mass with the short muzzle, the mandible inside it; ears as thin pricked leaves; per hind leg thigh, gaskin and one foot piece from the heel to the toes; per foreleg one mass for shoulder blade and upper arm, the forearm and one foot piece; tail in two round tapering pieces over its five bones. No eyes, nose, toes or whisker pads.", de: "Körper bei 1:500 (nur digital, die Druckzeichnung dieses Maßstabs entfällt): 20 Körper in wirklicher Größe mit den wenigsten von Hand gesetzten Schnitten, die die Katze noch ergeben. Rumpf in zwei Lofts aus je sechs Schnitten, die sich am tiefsten Punkt der Flanke kreuzen; kurzer Hals; runder Kopf als eine Masse mit kurzem Fang, der Unterkiefer liegt darin; Ohren als dünne aufgestellte Blätter; je Hinterbein Oberschenkel, Unterschenkel und ein Fußstück von der Ferse bis zu den Zehen; je Vorderbein eine Masse für Schulterblatt und Oberarm, der Unterarm und ein Fußstück; Schwanz in zwei runden, sich verjüngenden Stücken über seine fünf Knochen. Keine Augen, keine Nase, keine Zehen, keine Schnurrhaarkissen." },
  { en: "Thermal at 1:500: each coat region is the fur surface of its body solids (references, layer fur): trunk with neck, head, ears, forelimbs, hindlimbs, tail. The bare regions the body does not show are own small solids at their real size: nose leather, the two eyes, the tongue in the mouth and one sole under every foot. The outlets lie on the surface as the nostril patch, the lip patch and the back patch, stated for this scale.", de: "Thermik bei 1:500: jede Fellregion ist die Felloberfläche ihrer Körper (Verweise, Schicht fur): Rumpf mit Hals, Kopf, Ohren, Vorderbeine, Hinterbeine, Schwanz. Die nackten Regionen, die der Körper nicht zeigt, sind eigene kleine Körper in wirklicher Größe: Nasenspiegel, die beiden Augen, die Zunge im Maul und eine Sohle unter jedem Fuß. Die Auslässe liegen auf der Oberfläche als Nasenfleck, Lippenfleck und Rückenfleck, für diesen Maßstab gesetzt." },
  { en: "Structure at 1:500: the skeleton as real slender bones, since no print floor applies: skull, mandible, cervical spine, thoracic spine in two members under the topline with the withers, lumbar spine, pelvis, sternum, two ribs per side as slender solids, five caudal vertebra runs, ear cartilages, scapula plate, one tapered member per long bone, metapodial and digit bundle of every leg; joints at head, hips, stifles, shoulders and elbows; the four trunk masses at their centres. No supports are declared.", de: "Tragwerk bei 1:500: das Skelett als wirklich schlanke Knochen, da keine Druckgrenze gilt: Schädel, Unterkiefer, Halswirbelsäule, Brustwirbelsäule in zwei Stäben unter der Rückenlinie mit dem Widerrist, Lendenwirbelsäule, Becken, Brustbein, zwei Rippen je Seite als schlanke Körper, fünf Schwanzwirbelzüge, Ohrknorpel, Schulterblattplatte, ein verjüngter Stab je Röhrenknochen, Mittelfuß- und Zehenbündel jedes Beins; Gelenke an Kopf, Hüften, Knien, Schultern und Ellbogen; die vier Rumpfmassen in ihren Schwerpunkten. Es sind keine Auflager deklariert." },
];

if (import.meta.main) {
  const S = new Cat(500);
  body(S);
  const th = thermal(S, new Cat(500), { extra: extras });
  const st = structure(new Cat(500));
  console.log(`1:500 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) saveLists(S.c, { body: S.body, thermal: th, structure: st, decisions: DECISIONS });
}
