// [DEBUG] design source: seated-reader (female 1.68 m, 61 kg) on a bench; rest = seated configuration; pattern space = commuter standing design.
import { tx, refs, rev, FIXED, deg, rad, r4, sm5, R, xf, xp, xpt, armIK, ik2, pt, sh, cap, ell, line, mass, STD_EXPORTS, ENV, mat, region } from "./common.mjs";
import C from "./walking-commuter.mjs";
import { PAT, FINGERS } from "./walking-commuter.mjs";
const DL = "de-leva-1996", WI = "winter-2009", ISO = "iso-9920-2007", GU = "gupta-2010";
const k = 0.944, H = [0, 0.5], TH = -3.9, L1 = 0.4106, L2 = 0.4012, ARM1 = 0.3115, ARM2 = 0.2454;
const KNEE = [L1 * Math.cos(rad(-TH)), 0.5 + L1 * Math.sin(rad(TH))], ANK = [KNEE[0], KNEE[1] - L2];
const U = (d) => xf(d, k, 0, [0, 0.935], H), Up = (p) => xpt(p, k, 0, [0, 0.935], H);
const LEGT = (d) => xf(d, k, 90 + TH, [0, 0.935], H), SHK = (d) => xf(d, k, 0, [0, 0.5], KNEE), FT = (d) => xf(d, k, 0, [0, 0.075], ANK), FTp = (p) => xpt(p, k, 0, [0, 0.075], ANK);
const ARMo = [0, 0.4 * 0 + 0.5 + (1.455 - 0.935) * k];
const ARMT = (d) => xf(d, k, 0, [0, 1.455], ARMo), ARMTp = (p) => xpt(p, k, 0, [0, 1.455], ARMo);
const LUMB = [0, 0.5 + 0.1 * k], SHL = 0.5 + (1.455 - 0.935) * k - LUMB[1];
const head = C.shapes.find((s) => s.id === "head").d, neckD = C.shapes.find((s) => s.id === "neck").d, earD = C.shapes.find((s) => s.id === "ear");
const FRAC = { head: 0.0668, trunk: 0.301, pelvis: 0.1247, uarm: 0.0255, farm: 0.0138, hand: 0.0056, thigh: 0.1478, shank: 0.0481, foot: 0.0129 };

const aq = (a) => ({ angleDeg: r4(a) });
const shoulderAt = (torso, pelvis = 0) => { const p = pt([LUMB[0], LUMB[1]], 0, [0, 0]); const q = R(torso + pelvis, [0, SHL]); return [LUMB[0] + q[0], LUMB[1] + q[1]]; };
const armTo = (side, shoulder, wrist, handWorld) => { const r = armIK(shoulder, wrist, ARM1, ARM2); const fw = -90 + r.arm + r.forearm; if (handWorld === null) handWorld = fw; return { [`arm-${side}`]: aq(r.arm), [`forearm-${side}`]: aq(r.forearm), [`hand-${side}`]: aq(handWorld - fw) }; };
const padPose = (handWorld, padWorld) => ({ pad: aq(padWorld - handWorld) });
const TIP = (wrist, handWorld) => [wrist[0] + 0.1794 * Math.cos(rad(handWorld)), wrist[1] + 0.1794 * Math.sin(rad(handWorld))];

const TOR_A = -4;
const base = { torso: aq(TOR_A), neck: aq(-8), head: aq(-14) };
const SH_A = shoulderAt(TOR_A);
const wristFar = [0.27, 0.69], handFar = 40, padW = 76;
const tipFar = TIP(wristFar, handFar);
const padTip = [tipFar[0] + 0.2 * Math.cos(rad(padW)), tipFar[1] + 0.2 * Math.sin(rad(padW))];
const near = (wrist, hw) => armTo("r", SH_A, wrist, hw);
const upright = { ...base, ...armTo("l", SH_A, wristFar, handFar), ...padPose(handFar, padW), ...near([0.25, 0.64], 50), page: aq(0) };
const SH_B = shoulderAt(14);
const wristB = [0.30, 0.93], tipB = TIP(wristB, 50), padWB = 70;
const reclined = { torso: aq(14), neck: aq(-6), head: aq(-14), ...armTo("l", SH_B, wristB, 50), ...padPose(50, padWB), ...armTo("r", SH_B, [wristB[0] - 0.03, wristB[1] - 0.07], 55), page: aq(0) };
const SH_C = shoulderAt(-6);
const cup = [-0.215, 0.495];
const reach = { torso: aq(-6), neck: aq(-6), head: aq(-12), ...armTo("r", SH_C, [cup[0] - 0.0, cup[1] + 0.05], -70), ...armTo("l", SH_C, [0.24, 0.60], 15), ...padPose(15, 8), page: aq(0) };
const SH_S = shoulderAt(-38);
const standingUp = (lean, lift, shift, ext) => ({ torso: aq(lean), neck: aq(-6), head: aq(-4), "hip-lift": { dy: r4(lift) }, "hip-shift": { dx: r4(shift) }, "thigh-l": aq(ext), "thigh-r": aq(ext), "shank-l": aq(-ext), "shank-r": aq(-ext), "arm-l": aq(30), "arm-r": aq(30), "forearm-l": aq(40), "forearm-r": aq(40), pad: aq(60), page: aq(0) });

const pose = (id, en, de, dEn, dDe, bones, contacts, met, breath, exposed = {}) => ({ id, title: tx(en, de), description: tx(dEn, dDe), bones, ground: "authored", contacts, thermal: { activityMet: met, emitters: { breath }, ...(Object.keys(exposed).length ? { regions: exposed } : {}) }, loads: [] });
const SEATED = ["seat", "thigh-support", "heel-l", "ball-l", "heel-r", "ball-r"];

const bones = [
  { id: "seat", parent: null, title: tx("Bench seat", "Bankfläche"), at: [0, 0], worldAngleDeg: 0, lengthM: 0.48, joint: FIXED },
  { id: "hip-lift", parent: "seat", title: tx("Pelvis vertical travel", "Vertikale Beckenverschiebung"), at: [0, 0.5], worldAngleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: 0, maxM: 0.46 } },
  { id: "hip-shift", parent: "hip-lift", title: tx("Pelvis horizontal travel", "Horizontale Beckenverschiebung"), at: [0, 0.5], worldAngleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 0, minM: -0.1, maxM: 0.5 } },
  { id: "pelvis", parent: "hip-shift", title: tx("Pelvis", "Becken"), at: [0, 0.5], worldAngleDeg: 90, lengthM: 0.0944, joint: rev(-25, 25), mass: mass(7.8, 0.057, DL) },
  { id: "torso", parent: "pelvis", title: tx("Torso with sweater", "Rumpf mit Pullover"), at: LUMB, worldAngleDeg: 90, lengthM: 0.439, joint: rev(-65, 30), mass: mass(18.96, 0.2, DL) },
  { id: "neck", parent: "torso", title: tx("Neck", "Hals"), at: [0, 1.0334], worldAngleDeg: 82, lengthM: 0.0944, joint: rev(-35, 30), mass: mass(0.57, 0.047, DL, { source: "authored-estimate" }) },
  { id: "head", parent: "neck", title: tx("Head with bun", "Kopf mit Dutt"), at: [0.0131, 1.1269], worldAngleDeg: 90, lengthM: 0.17, joint: rev(-30, 25), mass: mass(3.6, 0.07, DL) },
  ...["l", "r"].flatMap((s) => {
    const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"];
    return [
      { id: `arm-${s}`, parent: "torso", title: tx(`${n[0]} upper arm`, `${n[1]} Oberarm`), at: ARMo, worldAngleDeg: -90, lengthM: ARM1, joint: rev(-60, 170), mass: mass(1.66, 0.18, DL) },
      { id: `forearm-${s}`, parent: `arm-${s}`, title: tx(`${n[0]} forearm`, `${n[1]} Unterarm`), at: [0, ARMo[1] - ARM1], worldAngleDeg: -90, lengthM: ARM2, joint: rev(0, 150), mass: mass(0.89, 0.112, DL) },
      { id: `hand-${s}`, parent: `forearm-${s}`, title: tx(`${n[0]} hand`, `${n[1]} Hand`), at: [0, ARMo[1] - ARM1 - ARM2], worldAngleDeg: -90, lengthM: 0.1794, joint: rev(-60, 60), mass: mass(0.34, 0.09, WI) },
      { id: `thigh-${s}`, parent: "pelvis", title: tx(`${n[0]} thigh`, `${n[1]} Oberschenkel`), at: H, worldAngleDeg: TH, lengthM: L1, joint: rev(-125, 25), mass: mass(9.32, 0.168, DL) },
      { id: `shank-${s}`, parent: `thigh-${s}`, title: tx(`${n[0]} shank`, `${n[1]} Unterschenkel`), at: KNEE, worldAngleDeg: -90, lengthM: L2, joint: rev(-65, 90), mass: mass(3.13, 0.179, DL) },
      { id: `foot-${s}`, parent: `shank-${s}`, title: tx(`${n[0]} foot and shoe`, `${n[1]} Fuß mit Schuh`), at: ANK, worldAngleDeg: -23.8, lengthM: 0.1754, joint: rev(-60, 30), mass: mass(1.06, FTp([0.071, 0.04]), DL) },
      { id: `toe-${s}`, parent: `foot-${s}`, title: tx(`${n[0]} forefoot`, `${n[1]} Vorfuß`), at: FTp([0.17, 0]), worldAngleDeg: 0, lengthM: 0.0614, joint: rev(0, 65), mass: mass(0.08, FTp([0.2, 0.03]), DL, { source: "authored-estimate" }) },
    ];
  }),
  { id: "pad", parent: "hand-l", title: tx("Top-bound notebook", "Oben gebundenes Notizbuch"), at: [0, ARMo[1] - ARM1 - ARM2 - 0.1794], worldAngleDeg: -90, lengthM: 0.2, joint: rev(-20, 120), mass: mass(0.6, 0.1, DL, { source: "authored-estimate", reference: undefined }) },
  { id: "page", parent: "pad", title: tx("Turning page", "Umblätternde Seite"), at: [0, ARMo[1] - ARM1 - ARM2 - 0.1794 - 0.2], worldAngleDeg: 90, lengthM: 0.19, joint: rev(-185, 5), mass: mass(0.01, 0.1, DL, { source: "authored-estimate" }) },
  { id: "cup", parent: "seat", title: tx("Paper cup on the seat", "Pappbecher auf der Sitzfläche"), at: [-0.215, 0.42], worldAngleDeg: 90, lengthM: 0.11, joint: FIXED, mass: mass(0, 0.05, DL, { source: "authored-estimate" }) },
];
const fa = (n, p) => ({ ...n, at: p });
const anchors = [
  { id: "seat", bone: "pelvis", at: [-0.03, 0.42], kind: "support", title: tx("Seat bones on the bench", "Sitzbeinhöcker auf der Bank"), support: "contact", normalDeg: 90, footprintLengthM: 0.16, footprintWidthM: 0.3, frictionCoefficient: 0.5 },
  { id: "thigh-support", bone: "thigh-r", at: [0.1, 0.42], kind: "support", title: tx("Thighs on the seat edge", "Oberschenkel auf der Sitzkante"), support: "contact", normalDeg: 90, footprintLengthM: 0.2, footprintWidthM: 0.3, frictionCoefficient: 0.5 },
  { id: "backrest", bone: "torso", at: [-0.12, 0.8], kind: "support", title: tx("Back against the backrest", "Rücken an der Lehne"), support: "contact", normalDeg: 0, footprintLengthM: 0.3, footprintWidthM: 0.35, frictionCoefficient: 0.5 },
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `heel-${s}`, bone: `foot-${s}`, at: FTp([-0.045, 0]), kind: "contact", title: tx(`${n[0]} heel`, `${n[1]} Ferse`), support: "contact", normalDeg: 90, footprintLengthM: 0.065, footprintWidthM: 0.06, frictionCoefficient: 0.9 },
    { id: `ball-${s}`, bone: `foot-${s}`, at: FTp([0.17, 0]), kind: "contact", title: tx(`${n[0]} ball of foot`, `${n[1]} Fußballen`), support: "contact", normalDeg: 90, footprintLengthM: 0.095, footprintWidthM: 0.09, frictionCoefficient: 0.9 }]; }),
  { id: "mouth", bone: "head", at: Up([0.094, 1.594]), kind: "emitter", title: tx("Mouth and nose", "Mund und Nase") },
  { id: "pad-grip", bone: "pad", at: [0, 0.1547], kind: "load", title: tx("Notebook centre of gravity", "Schwerpunkt des Notizbuchs") },
];
const woodD = "M -0.28 0.42 L 0.20 0.42 L 0.20 0.38 L -0.28 0.38 Z";
const shapes = [
  sh("bench-seat", "seat", "body", 0, 1.5, 0.62, undefined, "wood-bench", woodD),
  line("bench-slat-1", "seat", "detail", 1, 1.6, 0.3, undefined, "wood-bench", [[-0.28, 0.40], [0.20, 0.40]]),
  sh("bench-leg-front", "seat", "body", 0, 1.4, 0.45, undefined, "wood-bench", "M 0.12 0.38 L 0.17 0.38 L 0.17 0 L 0.12 0 Z"),
  sh("bench-leg-rear", "seat", "body", 0, 1.4, 0.45, undefined, "wood-bench", "M -0.25 0.38 L -0.20 0.38 L -0.20 0 L -0.25 0 Z"),
  sh("bench-back", "seat", "body", 0, 0.8, 0.58, undefined, "wood-bench", "M -0.30 0.42 L -0.265 0.42 L -0.345 0.86 L -0.38 0.86 Z"),
  line("bench-back-slat", "seat", "detail", 1, 0.9, 0.3, undefined, "wood-bench", [[-0.30, 0.62], [-0.355, 0.62]]),
  line("bench-back-slat-2", "seat", "detail", 2, 0.9, 0.3, undefined, "wood-bench", [[-0.32, 0.75], [-0.365, 0.75]]),
  sh("pelvis-jeans", "pelvis", "body", 0, 3, 0.33, "jeans", "denim", U("M -0.115 1.04 C -0.138 0.99 -0.135 0.93 -0.110 0.86 L 0.100 0.86 C 0.130 0.93 0.132 0.99 0.118 1.04 Z")),
  sh("sweater", "torso", "body", 0, 4, 0.62, "sweater", "knit-wool", U("M -0.03 1.50 C -0.085 1.49 -0.115 1.44 -0.118 1.38 C -0.12 1.30 -0.108 1.20 -0.115 1.10 C -0.122 1.03 -0.125 0.95 -0.12 0.90 L 0.135 0.90 C 0.13 0.98 0.128 1.05 0.126 1.10 C 0.124 1.2 0.135 1.25 0.145 1.31 C 0.135 1.39 0.108 1.45 0.062 1.482 C 0.04 1.495 0.01 1.505 -0.03 1.5 Z")),
  sh("sweater-hem", "torso", "detail", 1, 4.1, 0.5, "sweater", "knit-wool", U("M -0.12 0.93 L 0.134 0.93 L 0.135 0.90 L -0.12 0.90 Z")),
  line("sweater-rib-1", "torso", "texture", 3, 4.05, 0.4, "sweater", "knit-wool", xp([[-0.1, 1.38], [-0.1, 1.0]], k, 0, [0, 0.935], H)),
  line("sweater-rib-2", "torso", "texture", 3, 4.05, 0.4, "sweater", "knit-wool", xp([[0.0, 1.45], [0.0, 0.95]], k, 0, [0, 0.935], H)),
  line("sweater-rib-3", "torso", "texture", 3, 4.05, 0.4, "sweater", "knit-wool", xp([[0.08, 1.35], [0.09, 0.95]], k, 0, [0, 0.935], H)),
  sh("cowl", "neck", "body", 1, 4.5, 0.55, "sweater", "knit-wool", U("M -0.07 1.55 C -0.085 1.52 -0.08 1.485 -0.04 1.47 L 0.062 1.47 C 0.092 1.485 0.095 1.52 0.08 1.55 C 0.06 1.565 0.0 1.56 -0.03 1.56 Z")),
  sh("neck", "neck", "body", 0, 3.8, 0.84, "face-skin", "skin", U(neckD)),
  sh("head", "head", "body", 0, 5, 0.86, "face-skin", "skin", U(head)),
  sh("hair", "head", "body", 1, 5.5, 0.22, "hair", "hair", U("M 0.076 1.738 C 0.062 1.772 -0.020 1.784 -0.058 1.760 C -0.108 1.735 -0.115 1.66 -0.085 1.60 C -0.075 1.595 -0.062 1.60 -0.056 1.615 C -0.062 1.650 -0.050 1.700 -0.010 1.722 C 0.030 1.738 0.060 1.736 0.076 1.738 Z")),
  ell("hair-bun", "head", "body", 1, 5.6, 0.2, "hair", "hair", Up([-0.085, 1.765]), 0.04 * k, 0.037 * k),
  ell("ear", "head", "detail", 2, 5.2, 0.8, "face-skin", "skin", Up([-0.012, 1.645]), 0.012 * k, 0.02 * k),
  ell("eye", "head", "detail", 3, 5.6, 0.05, "face-skin", "skin", Up([0.075, 1.667]), 0.007 * k, 0.004 * k),
  line("mouth-line", "head", "detail", 3, 5.6, 0.3, "face-skin", "skin", xp([[0.083, 1.598], [0.096, 1.598]], k, 0, [0, 0.935], H)),
  ...["l", "r"].flatMap((s) => { const far = s === "l"; const tn = far ? -0.08 : 0; const z0 = far ? 0 : 2; return [
    sh(`sleeve-${s}`, `arm-${s}`, "body", 0, far ? 0.5 : 6, 0.62 + tn, "sweater", "knit-wool", ARMT(PAT.SLEEVE)),
    sh(`forearm-${s}`, `forearm-${s}`, "body", 0, far ? 0.5 : 6, 0.62 + tn, "sweater", "knit-wool", ARMT(PAT.FOREARM)),
    sh(`cuff-${s}`, `forearm-${s}`, "detail", 2, far ? 0.52 : 6.05, 0.5 + tn, "sweater", "knit-wool", ARMT("M -0.043 0.895 L 0.043 0.895 L 0.043 0.875 L -0.042 0.875 Z")),
    sh(`hand-${s}`, `hand-${s}`, "body", 0, far ? 0.6 : 6.2, 0.84 + tn, "hand-skin", "skin", ARMT(PAT.HAND)),
    ...FINGERS.map((p, i) => line(`fingers-${s}-${i}`, `hand-${s}`, "detail", 2, far ? 0.63 : 6.26, 0.5, "hand-skin", "skin", xp(p, k, 0, [0, 1.455], ARMo))),
    sh(`thumb-${s}`, `hand-${s}`, "detail", 1, far ? 0.62 : 6.25, 0.82 + tn, "hand-skin", "skin", ARMT(PAT.THUMB)),
    sh(`thigh-${s}`, `thigh-${s}`, "body", 0, z0 + 1, 0.35 + tn, "jeans", "denim", LEGT(PAT.TROUSER)),
    sh(`shank-${s}`, `shank-${s}`, "body", 0, z0 + 1.02, 0.35 + tn, "jeans", "denim", SHK(PAT.SHANK)),
    line(`jeans-seam-${s}`, `thigh-${s}`, "texture", 3, z0 + 1.02, 0.2, "jeans", "denim", xp([[-0.02, 0.92], [0.0, 0.65], [-0.01, 0.52]], k, 90 + TH, [0, 0.935], H)),
    sh(`shoe-rear-${s}`, `foot-${s}`, "body", 0, z0 + 1.1, 0.88 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_REAR)),
    sh(`shoe-toe-${s}`, `toe-${s}`, "body", 0, z0 + 1.1, 0.88 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_FRONT)),
    line(`shoe-sole-${s}`, `foot-${s}`, "detail", 1, z0 + 1.15, 0.1, "shoes", "canvas-sneaker", xp([[-0.050, 0.02], [0.17, 0.02]], k, 0, [0, 0.075], ANK)),
  ]; }),
  sh("pad-cover", "pad", "body", 0, 6.4, 0.3, "notebook", "paper", "M 0.0 0.0547 L 0.016 0.0547 L 0.016 0.2547 L 0.0 0.2547 Z"),
  sh("pad-pages", "pad", "detail", 1, 6.45, 0.95, "notebook", "paper", "M -0.012 0.0547 L 0.0 0.0547 L 0.0 0.2547 L -0.012 0.2547 Z"),
  line("pad-ruling", "pad", "texture", 3, 6.46, 0.4, "notebook", "paper", [[-0.012, 0.15], [0.0, 0.15]]),
  sh("page-sheet", "page", "detail", 1, 6.5, 0.98, "notebook", "paper", "M -0.016 0.0547 L -0.012 0.0547 L -0.012 0.2447 L -0.016 0.2447 Z"),
  sh("cup-body", "cup", "body", 0, 1.7, 0.9, undefined, "paper-cup", "M -0.245 0.42 L -0.185 0.42 L -0.19 0.525 L -0.24 0.525 Z"),
  sh("cup-lid", "cup", "detail", 1, 1.75, 0.2, undefined, "paper-cup", "M -0.243 0.525 L -0.187 0.525 L -0.188 0.54 L -0.242 0.54 Z"),
  line("cup-sleeve", "cup", "detail", 2, 1.75, 0.4, undefined, "paper-cup", [[-0.2395, 0.46], [-0.1905, 0.46]]),
];

const ik = (hip) => { const a = ik2(hip, ANK, L1, L2, 0); return { "thigh-l": aq(a.thigh - 90 - TH), "thigh-r": aq(a.thigh - 90 - TH), "shank-l": aq(a.shank + 90 + TH), "shank-r": aq(a.shank + 90 + TH), "foot-l": aq(a.foot), "foot-r": aq(a.foot) }; };
const seatedLegs = ik(H);
const withLegs = (b, hip = H) => ({ ...b, ...ik(hip), ...(hip === H ? {} : { "hip-lift": { dy: r4(hip[1] - 0.5) }, "hip-shift": { dx: r4(hip[0]) } }) });
const FEET = ["heel-l", "ball-l", "heel-r", "ball-r"];
const poses = [
  pose("seated-upright", "Reading upright", "Aufrecht lesend", "Upright on the bench, head bowed to the notebook held in both hands.", "Aufrecht auf der Bank, der Kopf zum Notizbuch in beiden Händen geneigt.", withLegs(upright), SEATED, 1, 0.4, { jeans: { exposedFraction: 0.85 } }),
  pose("reclined-reading", "Reclined reading", "Zurückgelehnt lesend", "Leaning back against the backrest with the notebook raised to eye level.", "An die Lehne zurückgelehnt, das Notizbuch auf Augenhöhe angehoben.", withLegs(reclined), [...SEATED, "backrest"], 1, 0.4, { jeans: { exposedFraction: 0.85 }, sweater: { exposedFraction: 0.86 } }),
  pose("reaching-for-cup", "Reaching for the cup", "Nach dem Becher greifen", "Notebook resting on the knee, the near hand reaching back for the paper cup on the seat.", "Notizbuch auf dem Knie, die nahe Hand greift nach hinten zum Pappbecher auf der Sitzfläche.", withLegs(reach), SEATED, 1.2, 0.5, { jeans: { exposedFraction: 0.85 } }),
  pose("leaning-forward", "Leaning forward", "Vorgeneigt", "Weight shifted forward over the feet, ready to stand up, hands gathering the notebook.", "Gewicht nach vorn über die Füße verlagert, bereit zum Aufstehen, die Hände nehmen das Notizbuch.", withLegs(standingUp(-38, 0, 0, 0), [0.08, 0.5]), SEATED, 1.6, 0.8, { jeans: { exposedFraction: 0.92 } }),
  pose("rising", "Rising", "Aufrichten", "Pelvis lifted off the seat, trunk still inclined, the legs extending.", "Becken von der Sitzfläche abgehoben, Rumpf noch geneigt, die Beine strecken sich.", withLegs(standingUp(-35, 0, 0, 0), [0.27, 0.66]), FEET, 2.2, 1, {}),
  pose("standing", "Standing up", "Aufgestanden", "Upright on both feet next to the bench, notebook held at waist height.", "Aufrecht auf beiden Füßen neben der Bank, das Notizbuch auf Hüfthöhe.", withLegs({ ...standingUp(-4, 0, 0, 0), "arm-l": aq(10), "arm-r": aq(10), "forearm-l": aq(75), "forearm-r": aq(75), pad: aq(80) }, [0.41, 0.8727]), FEET, 1.4, 0.7, {}),
];
const pageFree = (off) => { const w = padW + 180 + off, e = [padTip[0] + 0.19 * Math.cos(rad(w)), padTip[1] + 0.19 * Math.sin(rad(w))]; return e; };
const nearAtPage = (off) => { const e = pageFree(off), dx = e[0] - SH_A[0], dy = e[1] - SH_A[1], d = Math.hypot(dx, dy); let f = 0.75, out; for (; f > -0.4; f -= 0.1) { const wr = [e[0] - 0.1794 * f * dx / d, e[1] - 0.1794 * f * dy / d]; out = armTo("r", SH_A, wr, null); if (out["forearm-r"].angleDeg < 140 && Math.abs(out["hand-r"].angleDeg) < 50) break; } return { ...out, page: aq(off) }; };
const ev = (t, kind, en, de, target) => ({ t, kind, title: tx(en, de), ...(target ? { target } : {}) });
const keyAt = (t, pose, bones, ease = "smooth") => ({ t, pose, ease, ...(bones ? { bones } : {}) });
const clips = [
  {
    id: "page-turn", title: tx("Turning the page", "Seite umblättern"), description: tx("The near hand travels from the lap to the free edge of the top-bound page, lifts it over the spine and returns to the notebook while the far hand keeps holding it.", "Die nahe Hand wandert vom Schoß zur freien Kante der oben gebundenen Seite, hebt sie über die Bindung und kehrt zum Notizbuch zurück, während die ferne Hand es hält."),
    durationS: 3.6, fps: 30, loop: false, rootMotion: "none",
    keys: [keyAt(0, "seated-upright"), keyAt(0.7, "seated-upright", nearAtPage(0)), keyAt(1.2, "seated-upright", nearAtPage(-50)), keyAt(1.7, "seated-upright", nearAtPage(-110)), keyAt(2.3, "seated-upright", nearAtPage(-178)), keyAt(3.0, "seated-upright", { ...near([0.30, 0.66], 50), page: aq(-178) }), keyAt(3.6, "seated-upright", { ...near([0.30, 0.66], 50), page: aq(-178) })],
    contacts: SEATED.map((anchor) => ({ anchor, fromS: 0, toS: 3.6 })),
    phases: [{ id: "reach", fromS: 0, toS: 0.7, title: tx("Hand to the page edge", "Hand zur Seitenkante") }, { id: "turn", fromS: 0.7, toS: 2.3, title: tx("Lifting the page over", "Seite umlegen") }, { id: "settle", fromS: 2.3, toS: 3.6, title: tx("Back to reading", "Zurück zum Lesen") }],
    events: [ev(0.7, "motion", "Fingers at the page edge", "Finger an der Seitenkante"), ev(1.2, "motion", "Page passes the vertical", "Seite passiert die Senkrechte"), ev(2.3, "motion", "Page lies over", "Seite liegt um"), ev(1.0, "inhale", "Breath in", "Einatmen"), ev(2.0, "exhale", "Breath out", "Ausatmen")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.4 }, { t: 1.0, v: 0.2 }, { t: 2.0, v: 0.5 }, { t: 3.6, v: 0.4 }] }],
  },
  {
    id: "stand-up", title: tx("Standing up from the bench", "Von der Bank aufstehen"), description: tx("Sit-to-stand: the trunk leans forward over the feet, the pelvis leaves the seat and travels 0.4 m forward and 0.37 m up while both feet stay planted; the activity rises from 1 to about 2.2 met and falls again.", "Aufstehen aus dem Sitz: Der Rumpf neigt sich über die Füße, das Becken verlässt die Sitzfläche und bewegt sich 0,4 m nach vorn und 0,37 m nach oben, während beide Füße stehen bleiben; die Aktivität steigt von 1 auf etwa 2,2 met und fällt wieder."),
    durationS: 4, fps: 30, loop: false, rootMotion: "none",
    keys: [keyAt(0, "seated-upright"), keyAt(0.8, "leaning-forward"), keyAt(1.6, "rising"), keyAt(2.6, "standing"), keyAt(4, "standing")],
    contacts: [...SEATED.slice(0, 2).map((anchor) => ({ anchor, fromS: 0, toS: 1.3 })), ...FEET.map((anchor) => ({ anchor, fromS: 0, toS: 4 }))],
    phases: [{ id: "lean", fromS: 0, toS: 0.8, title: tx("Trunk leans forward", "Rumpf neigt sich vor") }, { id: "lift-off", fromS: 0.8, toS: 1.6, title: tx("Pelvis leaves the seat", "Becken verlässt die Sitzfläche") }, { id: "extend", fromS: 1.6, toS: 2.6, title: tx("Legs extend", "Beine strecken sich") }, { id: "stand", fromS: 2.6, toS: 4, title: tx("Standing", "Stehen") }],
    events: [ev(1.3, "contact-off", "Seat contact released", "Sitzkontakt gelöst", "seat"), ev(1.3, "contact-off", "Thigh support released", "Oberschenkelauflage gelöst", "thigh-support"), ev(0.5, "inhale", "Breath in before rising", "Einatmen vor dem Aufstehen"), ev(1.8, "exhale", "Breath out while rising", "Ausatmen beim Aufrichten")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 1.2, v: 2.2 }, { t: 2.8, v: 1.8 }, { t: 4, v: 1.4 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.4 }, { t: 1.8, v: 1 }, { t: 4, v: 0.7 }] }],
  },
  {
    id: "lean-back", title: tx("Settling back and reaching for the cup", "Zurücklehnen und Becher greifen"), description: tx("From upright reading she leans back against the backrest and then turns the near arm backwards to the paper cup on the seat.", "Aus aufrechtem Lesen lehnt sie sich an die Lehne und greift mit dem nahen Arm nach hinten zum Pappbecher auf der Sitzfläche."),
    durationS: 5, fps: 30, loop: false, rootMotion: "none",
    keys: [keyAt(0, "seated-upright"), keyAt(1.5, "reclined-reading"), keyAt(2.5, "reclined-reading"), keyAt(3.6, "reaching-for-cup"), keyAt(5, "reaching-for-cup")],
    contacts: [...SEATED.map((anchor) => ({ anchor, fromS: 0, toS: 5 })), { anchor: "backrest", fromS: 1.1, toS: 2.9 }],
    phases: [{ id: "recline", fromS: 0, toS: 1.5, title: tx("Leaning back", "Zurücklehnen") }, { id: "read", fromS: 1.5, toS: 2.5, title: tx("Reading reclined", "Zurückgelehnt lesen") }, { id: "reach", fromS: 2.5, toS: 5, title: tx("Reaching for the cup", "Nach dem Becher greifen") }],
    events: [ev(1.1, "contact-on", "Back touches the backrest", "Rücken berührt die Lehne", "backrest"), ev(2.9, "contact-off", "Back leaves the backrest", "Rücken löst sich von der Lehne", "backrest")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.4 }, { t: 5, v: 0.5 }] }],
  },
];
export default {
  id: "seated-reader",
  title: tx("Seated Reader", "Lesende Person im Sitzen"),
  summary: tx("A woman reading a top-bound notebook on the edge of a park bench: pelvis and thighs carried by the seat, feet flat, with a page-turn, a sit-to-stand transition and a reach for a paper cup on the seat.", "Eine Frau liest in einem oben gebundenen Notizbuch an der Kante einer Parkbank: Becken und Oberschenkel liegen auf der Sitzfläche, die Füße stehen flach; mit Umblättern, Aufstehen und Griff zum Pappbecher auf der Sitzfläche."),
  tags: ["human", "adult", "seated", "reader", "bench", "female"],
  provenance: {
    author: "Semio staffage content worker humans-a", method: "handcrafted", created: "2026-10-29",
    references: refs(DL, WI, "drillis-contini-1966", "dubois-1916", ISO, "iso-8996-2021", "iso-7730-2005", "ashrae-2021", GU),
    notes: tx("A 1.68 m, 61 kg woman: segment masses from the de Leva (1996) female fractions plus 2.9 kg of clothing and footwear, lengths scaled from the 1.78 m design by 0.944. The rest configuration is seated; the bench seat is a fixed root bone so that sitting and standing up are the same rig.", "Eine 1,68 m große, 61 kg schwere Frau: Segmentmassen nach den weiblichen Anteilen von de Leva (1996) plus 2,9 kg Kleidung und Schuhe, Längen mit 0,944 aus dem 1,78-m-Entwurf skaliert. Die Ruhekonfiguration ist sitzend; die Bankfläche ist ein fester Wurzelknochen, sodass Sitzen und Aufstehen dasselbe Skelett nutzen."),
  },
  dimensions: { heightM: 1.34, widthM: 1.02, depthM: 0.5, massKg: 64.5 },
  materials: [
    mat("skin", "Skin", "Haut", 0.98, 0.6, "literature", "ashrae-2021"), mat("hair", "Hair", "Haar", 0.98, 0.85, "authored-estimate"),
    mat("knit-wool", "Knitted wool sweater", "Wollstrickpullover", 0.9, 0.6, "literature", "ashrae-2021"), mat("denim", "Denim", "Denim", 0.92, 0.65, "literature", "ashrae-2021"),
    mat("canvas-sneaker", "Canvas sneaker", "Stoffturnschuh", 0.9, 0.5, "authored-estimate"), mat("paper", "Paper", "Papier", 0.93, 0.4, "literature", "ashrae-2021"),
    mat("paper-cup", "Paper cup", "Pappbecher", 0.9, 0.4, "authored-estimate"), mat("wood-bench", "Painted bench timber", "Gestrichenes Bankholz", 0.9, 0.6, "literature", "ashrae-2021"),
  ],
  bones, anchors, shapes, poses, clips,
  thermal: {
    environment: ENV(14, 16, 0.6, 0.3, 150), baseMet: 1,
    regions: [
      region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", "core-resistance", 0.09, 0.2, 0.08, { coreTempC: 36.8, insulationM2KW: 0.04 }, "dubois-1916"),
      region("hair", "Hair", "Haar", "hair", "hair", "core-resistance", 0.05, 0.18, 0.02, { coreTempC: 36.8, insulationM2KW: 0.08 }, "dubois-1916", "authored-estimate"),
      region("hand-skin", "Bare hands", "Unbedeckte Hände", "skin", "skin", "core-resistance", 0.08, 0.1, 0.05, { coreTempC: 35, insulationM2KW: 0.05 }, "dubois-1916"),
      region("sweater", "Sweater with cowl", "Pullover mit Rollkragen", "clothing", "knit-wool", "core-resistance", 0.81, 0.3, 0.52, { coreTempC: 36.8, insulationM2KW: 0.2 }, ISO),
      region("jeans", "Jeans", "Jeans", "clothing", "denim", "core-resistance", 0.56, 0.13, 0.28, { coreTempC: 36.8, insulationM2KW: 0.12 }, ISO),
      region("shoes", "Sneakers", "Turnschuhe", "surface", "canvas-sneaker", "core-resistance", 0.1, 0.1, 0.05, { coreTempC: 34, insulationM2KW: 0.1 }, ISO, "authored-estimate"),
      region("notebook", "Notebook", "Notizbuch", "surface", "paper", "passive", 0.12, 0.2, undefined, {}, ISO, "authored-estimate"),
    ],
    emitters: [{ id: "breath", kind: "breath", title: tx("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: -20, carries: "vapour", temperatureC: 34, velocityMS: 1, diameterM: 0.018, vapourKgPerKg: 0.0345, periodS: 5, dutyFrom: 0.4, dutyTo: 1, source: "literature", reference: GU }],
    assumptions: [tx("Body regions sum to the DuBois area of 1.69 m² (61 kg, 168 cm); contact with the seat and backrest is modelled as reduced exposed fractions of the jeans and the sweater.", "Die Körperregionen ergeben die DuBois-Fläche von 1,69 m² (61 kg, 168 cm); Kontakt mit Sitz und Lehne wird durch verminderte freie Anteile von Jeans und Pullover abgebildet."), tx("Seated quiet reading is 1.0 met (ISO 8996); breathing at 12 per minute with 60 % exhalation.", "Ruhiges Sitzen und Lesen entspricht 1,0 met (ISO 8996); 12 Atemzüge pro Minute mit 60 % Ausatmung."), tx("Sun-exposed autumn day with 150 W/m² of diffuse and direct radiation on the vertical surface.", "Sonniger Herbsttag mit 150 W/m² Strahlung auf der vertikalen Fläche.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [tx("Seat, thigh support and backrest are unilateral contacts; they release when they would have to pull. The bench is rigid and fixed.", "Sitz, Oberschenkelauflage und Lehne sind einseitige Kontakte; sie lösen sich, wenn sie ziehen müssten. Die Bank ist starr und fest."), tx("Segments are point masses at the de Leva centres; the notebook is a 0.6 kg bone mass on the far hand.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; das Notizbuch ist eine Knochenmasse von 0,6 kg an der fernen Hand.")], clinical: false },
  presentation: { defaultPose: "seated-upright", defaultClip: "page-turn", viewpoints: ["elevation"] },
  exports: STD_EXPORTS,
};
