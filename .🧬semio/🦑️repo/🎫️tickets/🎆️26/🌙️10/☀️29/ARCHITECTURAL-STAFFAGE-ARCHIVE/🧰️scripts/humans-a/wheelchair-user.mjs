// [DEBUG] design source: wheelchair-user (adult man 1.75 m, 75 kg, in a manual wheelchair; the chair frame is the rig root).
import { tx, refs, rev, FIXED, deg, rad, r4, sm5, R, xf, xp, xpt, armIK, circ, ring, sh, cap, ell, line, mass, STD_EXPORTS, ENV, mat, region } from "./common.mjs";
import C, { PAT, FINGERS } from "./walking-commuter.mjs";
const DL = "de-leva-1996", WI = "winter-2009", ISO = "iso-9920-2007", GU = "gupta-2010", CRP = "cooper-2008";
const k = 0.98, AX = [0, 0.305], RW = 0.305, RC = 0.0678, CAST = [0.44, RC], HIP = [0, 0.6], SPIN = 240, T1 = 1;
const A1 = 0.33 * k, A2 = 0.26 * k, HL = 0.19 * k, L1 = 0.435 * k, L2 = 0.425 * k, PEL = 0.1 * k, TOR = 0.465 * k, NECK = 0.1 * k, HEAD = 0.18 * k;
const dir = (a) => [Math.cos(rad(a)), Math.sin(rad(a))], add = (p, v, s = 1) => [p[0] + v[0] * s, p[1] + v[1] * s];
const LUMB = add(HIP, [0, 1], PEL), C7 = add(LUMB, [0, 1], TOR), NTOP = add(C7, dir(82), NECK), SHO = add(LUMB, [0, 1], 0.42 * k);
const KNEE = add(HIP, dir(-2), L1), ANK = add(KNEE, dir(-81), L2), BALL = [ANK[0] + 0.17 * k, ANK[1] - 0.075 * k];
const U = (d) => xf(d, k, 0, [0, 0.935], HIP), Up = (p) => xpt(p, k, 0, [0, 0.935], HIP);
const NK = (d) => xf(d, k, 0, [0, 1.5], C7), HD = (d) => xf(d, k, 0, [0.0139, 1.599], NTOP), HDp = (p) => xpt(p, k, 0, [0.0139, 1.599], NTOP);
const LEGT = (d) => xf(d, k, 88, [0, 0.935], HIP), SHK = (d) => xf(d, k, 9, [0, 0.5], KNEE), FT = (d) => xf(d, k, 0, [0, 0.075], ANK), FTp = (p) => xpt(p, k, 0, [0, 0.075], ANK);
const ARM = (d) => xf(d, k, 0, [0, 1.455], SHO), WRIST0 = [SHO[0], SHO[1] - A1 - A2];
const aq = (a) => ({ angleDeg: r4(a) });
const rimPoint = (a, r) => [AX[0] + r * Math.cos(rad(a)), AX[1] + r * Math.sin(rad(a))];
const shAt = (th) => { const q = R(th, [SHO[0] - LUMB[0], SHO[1] - LUMB[1]]); return [LUMB[0] + q[0], LUMB[1] + q[1]]; };
const handOn = (side, th, P, extra = {}) => { const s = shAt(th), dx = P[0] - s[0], dy = P[1] - s[1], d = Math.hypot(dx, dy), wr = [P[0] - (HL / 2) * dx / d, P[1] - (HL / 2) * dy / d], r = armIK(s, wr, A1, A2); return { [`arm-${side}`]: aq(r.arm - th), [`forearm-${side}`]: aq(r.forearm), [`hand-${side}`]: aq(0), ...extra }; };
const strokeState = (tau) => {
  if (tau <= 0.1875) return { a: 100 - SPIN * tau, r: 0.285 };
  const w = (tau - 0.1875) / 0.8125, m = (-SPIN * 0.8125) / 45, h = (w ** 3 - 2 * w ** 2 + w) * m + (-2 * w ** 3 + 3 * w ** 2) + (w ** 3 - w ** 2) * m;
  return { a: 55 + 45 * h, r: 0.285 + 0.075 * Math.sin(Math.PI * w) ** 2 };
};
export const pushBones = (tau, spinDeg, lean0 = -4, lean1 = -22) => {
  const q = strokeState(tau), th = lean0 + (lean1 - lean0) * ((100 - q.a) / 45), P = rimPoint(q.a, q.r);
  return { ...handOn("r", th, P), ...handOn("l", th, add(P, [0.0, 0.0])), torso: aq(th), neck: aq(-th * 0.45 + 2), head: aq(-th * 0.35), "rear-wheel": aq(-spinDeg), castor: aq(-spinDeg * (RW / RC)) };
};
const FEETUP = {}, LAP = { "arm-r": aq(14), "forearm-r": aq(58), "hand-r": aq(5), "arm-l": aq(16), "forearm-l": aq(60), "hand-l": aq(5), torso: aq(0), neck: aq(2), head: aq(0) };
const mkPose = (id, en, de, dEn, dDe, bones, met, breath, root) => ({ id, title: tx(en, de), description: tx(dEn, dDe), bones, ...(root ? { root } : {}), ground: "auto", contacts: ["patch-rear", "patch-front"], thermal: { activityMet: met, emitters: { breath } }, loads: [] });
const poses = [
  mkPose("seated-level", "Seated on level ground", "Sitzend auf ebenem Boden", "Hands resting in the lap, back against the backrest, castors and rear wheels on the ground.", "Hände im Schoß, Rücken an der Lehne, Lenkrollen und Hinterräder auf dem Boden.", LAP, 1.2, 0.4),
  mkPose("hands-on-rims", "Hands on the hand rims", "Hände an den Greifreifen", "The gloved hands grip the hand rims just behind the top of the wheel, trunk upright, ready to push.", "Die behandschuhten Hände greifen die Greifreifen kurz hinter dem Radscheitel, Rumpf aufrecht, bereit zum Antrieb.", pushBones(0, 0), 1.8, 0.6),
  mkPose("push-end", "End of the push stroke", "Ende des Antriebsschubs", "The arms are extended forward along the rim, the trunk leans forward at the end of the 45 degree push arc.", "Die Arme sind entlang des Reifens nach vorn gestreckt, der Rumpf neigt sich am Ende des 45-Grad-Schubbogens nach vorn.", pushBones(0.1875, 45), 2.6, 1),
  mkPose("ramp-ascent", "Pushing up a 1:12 ramp", "Antrieb auf einer Rampe 1:12", "The whole chair is tilted 4.8 degrees about the rear axle onto a ramp, the trunk leans forward and the hands push at the lower rim.", "Der ganze Rollstuhl ist um 4,8 Grad um die Hinterachse auf eine Rampe gekippt, der Rumpf neigt sich vor und die Hände schieben am unteren Reifenbereich.", pushBones(0.1, 24, -16, -30), 3.2, 1, { angleDeg: 4.8 }),
];
const KEYSN = 32;
const mkKeys = (n, pose, lean0, lean1, speed = 1) => Array.from({ length: n * KEYSN + 0 }, (_, i) => { const t = i / KEYSN, tau = t % 1; return { t: r4(t), pose, ease: "smooth", bones: pushBones(tau, SPIN * t * speed, lean0, lean1) }; });
const NC = 4, DUR = NC * T1;
const ev = (t, kind, en, de, target) => ({ t, kind, title: tx(en, de), ...(target ? { target } : {}) });
const bones = [
  { id: "frame", parent: null, title: tx("Wheelchair frame with seat and footplate", "Rollstuhlrahmen mit Sitz und Fußplatte"), at: AX, worldAngleDeg: 0, lengthM: 0.55, joint: FIXED, mass: mass(7.5, [0.2, 0.42], CRP, { source: "authored-estimate" }) },
  { id: "rear-wheel", parent: "frame", title: tx("Rear wheel pair with hand rims", "Hinterradpaar mit Greifreifen"), at: AX, worldAngleDeg: 0, lengthM: RW, joint: { type: "revolute" }, mass: mass(4.4, [AX[0], AX[1]], CRP, { source: "authored-estimate", distribution: "point" }) },
  { id: "castor", parent: "frame", title: tx("Front castor pair", "Vorderes Lenkrollenpaar"), at: CAST, worldAngleDeg: 0, lengthM: RC, joint: { type: "revolute" }, mass: mass(0.5, [CAST[0], CAST[1]], CRP, { source: "authored-estimate" }) },
  { id: "pelvis", parent: "frame", title: tx("Pelvis", "Becken"), at: HIP, worldAngleDeg: 90, lengthM: PEL, joint: rev(-15, 15), mass: mass(8.6, 0.06, DL) },
  { id: "torso", parent: "pelvis", title: tx("Torso with hoodie", "Rumpf mit Kapuzenpullover"), at: LUMB, worldAngleDeg: 90, lengthM: TOR, joint: rev(-50, 30), mass: mass(25, 0.215, DL) },
  { id: "neck", parent: "torso", title: tx("Neck", "Hals"), at: C7, worldAngleDeg: 82, lengthM: NECK, joint: rev(-35, 30), mass: mass(0.7, 0.05, DL, { source: "authored-estimate" }) },
  { id: "head", parent: "neck", title: tx("Head with beanie", "Kopf mit Mütze"), at: NTOP, worldAngleDeg: 90, lengthM: HEAD, joint: rev(-30, 25), mass: mass(4.6, 0.075, DL) },
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `arm-${s}`, parent: "torso", title: tx(`${n[0]} upper arm`, `${n[1]} Oberarm`), at: SHO, worldAngleDeg: -90, lengthM: A1, joint: rev(-60, 170), mass: mass(2.13, 0.19 * k, DL) },
    { id: `forearm-${s}`, parent: `arm-${s}`, title: tx(`${n[0]} forearm`, `${n[1]} Unterarm`), at: [SHO[0], SHO[1] - A1], worldAngleDeg: -90, lengthM: A2, joint: rev(0, 150), mass: mass(1.27, 0.119 * k, DL) },
    { id: `hand-${s}`, parent: `forearm-${s}`, title: tx(`${n[0]} gloved hand`, `${n[1]} behandschuhte Hand`), at: WRIST0, worldAngleDeg: -90, lengthM: HL, joint: rev(-70, 70), mass: mass(0.51, 0.096 * k, WI) },
    { id: `thigh-${s}`, parent: "pelvis", title: tx(`${n[0]} thigh`, `${n[1]} Oberschenkel`), at: HIP, worldAngleDeg: -2, lengthM: L1, joint: rev(-30, 30), mass: mass(10.77, 0.178 * k, DL) },
    { id: `shank-${s}`, parent: `thigh-${s}`, title: tx(`${n[0]} shank`, `${n[1]} Unterschenkel`), at: KNEE, worldAngleDeg: -81, lengthM: L2, joint: rev(-40, 40), mass: mass(3.35, 0.1895 * k, DL) },
    { id: `foot-${s}`, parent: `shank-${s}`, title: tx(`${n[0]} foot on the footplate`, `${n[1]} Fuß auf der Fußplatte`), at: ANK, worldAngleDeg: -23.8, lengthM: 0.1858 * k, joint: rev(-30, 30), mass: mass(1.35, FTp([0.071, 0.04]), DL) },
    { id: `toe-${s}`, parent: `foot-${s}`, title: tx(`${n[0]} forefoot`, `${n[1]} Vorfuß`), at: FTp([0.17, 0]), worldAngleDeg: 0, lengthM: 0.065 * k, joint: rev(0, 40), mass: mass(0.08, FTp([0.2, 0.03]), DL, { source: "authored-estimate" }) },
  ]; }),
];
const anchors = [
  { id: "patch-rear", bone: "frame", at: [0, 0], kind: "contact", title: tx("Rear tyre contact patch", "Aufstandsfläche der Hinterreifen"), support: "contact", normalDeg: 90, footprintLengthM: 0.05, footprintWidthM: 0.06, frictionCoefficient: 0.8 },
  { id: "patch-front", bone: "frame", at: [CAST[0], 0], kind: "contact", title: tx("Castor contact patch", "Aufstandsfläche der Lenkrollen"), support: "contact", normalDeg: 90, footprintLengthM: 0.025, footprintWidthM: 0.04, frictionCoefficient: 0.8 },
  { id: "rim-grip", bone: "hand-r", at: add(WRIST0, [0, -1], HL / 2), kind: "attachment", title: tx("Grip on the hand rim", "Griff am Greifreifen") },
  { id: "mouth", bone: "head", at: HDp([0.094, 1.594]), kind: "emitter", title: tx("Mouth and nose", "Mund und Nase") },
];
const spokes = Array.from({ length: 12 }, (_, i) => { const a = (i * 30 * Math.PI) / 180, p0 = [AX[0] + 0.035 * Math.cos(a), AX[1] + 0.035 * Math.sin(a)], p1 = [AX[0] + 0.285 * Math.cos(a), AX[1] + 0.285 * Math.sin(a)]; return line(`spoke-${i}`, "rear-wheel", "detail", 1, 7.1, 0.5, "frame-metal", "aluminium", [p0, p1]); });
const castorSpokes = [0, 60, 120].map((a, i) => { const r = rad(a); return line(`castor-spoke-${i}`, "castor", "detail", 1, 7.1, 0.4, "tyres", "rubber", [[CAST[0] - 0.05 * Math.cos(r), CAST[1] - 0.05 * Math.sin(r)], [CAST[0] + 0.05 * Math.cos(r), CAST[1] + 0.05 * Math.sin(r)]]); });
const SWEAT = "M -0.03 1.50 C -0.085 1.49 -0.115 1.44 -0.118 1.38 C -0.12 1.30 -0.108 1.20 -0.115 1.10 C -0.122 1.03 -0.125 0.95 -0.12 0.90 L 0.135 0.90 C 0.13 0.98 0.128 1.05 0.126 1.10 C 0.124 1.2 0.135 1.25 0.14 1.31 C 0.135 1.39 0.108 1.45 0.062 1.482 C 0.04 1.495 0.01 1.505 -0.03 1.5 Z";
const shapes = [
  line("ground", "frame", "ground", 0, 0, 0.5, undefined, undefined, [[-1.0, 0], [1.7, 0]]),
  cap("frame-seat-rail", "frame", "body", 0, 6.8, 0.35, "frame-metal", "aluminium", [-0.07, 0.47], [0.42, 0.495], 0.012),
  cap("frame-back-tube", "frame", "body", 0, 6.8, 0.35, "frame-metal", "aluminium", [-0.07, 0.47], [-0.115, 0.93], 0.012),
  cap("frame-push-handle", "frame", "body", 1, 6.8, 0.15, "frame-metal", "aluminium", [-0.115, 0.93], [-0.2, 0.91], 0.014),
  cap("frame-hanger", "frame", "body", 0, 6.8, 0.35, "frame-metal", "aluminium", [0.42, 0.495], [0.49, 0.11], 0.012),
  cap("frame-lower-rail", "frame", "body", 0, 6.7, 0.35, "frame-metal", "aluminium", [0.0, 0.305], [0.40, 0.30], 0.011),
  cap("frame-lower-front", "frame", "body", 1, 6.7, 0.35, "frame-metal", "aluminium", [0.4, 0.3], [0.49, 0.12], 0.011),
  cap("castor-fork", "frame", "body", 0, 6.7, 0.3, "frame-metal", "aluminium", [0.46, 0.215], [CAST[0], CAST[1] + 0.01], 0.01),
  sh("castor-housing", "frame", "body", 1, 6.75, 0.25, "frame-metal", "aluminium", "M 0.43 0.25 L 0.49 0.25 L 0.49 0.19 L 0.43 0.19 Z"),
  sh("footplate", "frame", "body", 0, 6.7, 0.3, "frame-metal", "aluminium", "M 0.46 0.10 L 0.78 0.10 L 0.78 0.075 L 0.46 0.075 Z"),
  sh("footplate-heel-loop", "frame", "detail", 2, 6.75, 0.3, "frame-metal", "aluminium", "M 0.46 0.10 L 0.46 0.145 L 0.475 0.145 L 0.475 0.10 Z"),
  sh("seat-cushion", "frame", "body", 0, 2.4, 0.22, "cushion", "foam", "M -0.07 0.47 L 0.42 0.495 L 0.42 0.532 C 0.3 0.542 0.0 0.538 -0.07 0.528 Z"),
  sh("back-sling", "frame", "body", 1, 2.3, 0.28, "cushion", "foam", "M -0.075 0.53 L -0.092 0.86 L -0.12 0.86 L -0.1 0.53 Z"),
  sh("brake-lever", "frame", "detail", 2, 6.9, 0.2, "frame-metal", "aluminium", "M 0.08 0.48 L 0.15 0.62 L 0.165 0.615 L 0.095 0.475 Z"),
  sh("rear-tyre", "rear-wheel", "body", 0, 7, 0.1, "tyres", "rubber", ring(AX[0], AX[1], RW, 0.282)),
  sh("rear-rim", "rear-wheel", "detail", 1, 7.05, 0.7, "frame-metal", "aluminium", ring(AX[0], AX[1], 0.282, 0.272)),
  sh("hand-rim", "rear-wheel", "detail", 0, 7.05, 0.45, "frame-metal", "aluminium", ring(AX[0], AX[1], 0.272 + 0.0, 0.258)),
  ell("rear-hub", "rear-wheel", "body", 1, 7.2, 0.55, "frame-metal", "aluminium", AX, 0.034, 0.034),
  ...spokes,
  sh("castor-tyre", "castor", "body", 0, 7, 0.15, "tyres", "rubber", circ(CAST[0], CAST[1], RC, true)),
  ell("castor-hub", "castor", "detail", 1, 7.2, 0.6, "frame-metal", "aluminium", CAST, 0.018, 0.018),
  ...castorSpokes,
  ...["l", "r"].flatMap((s) => { const far = s === "l"; const tn = far ? -0.08 : 0; const z0 = far ? 0 : 3; return [
    sh(`sleeve-${s}`, `arm-${s}`, "body", 0, far ? 0.5 : 6, 0.36 + tn, "hoodie", "fleece", ARM(PAT.SLEEVE)),
    sh(`forearm-${s}`, `forearm-${s}`, "body", 0, far ? 0.5 : 6, 0.36 + tn, "hoodie", "fleece", ARM(PAT.FOREARM)),
    sh(`cuff-${s}`, `forearm-${s}`, "detail", 2, far ? 0.52 : 6.05, 0.25 + tn, "hoodie", "fleece", ARM("M -0.043 0.895 L 0.043 0.895 L 0.043 0.875 L -0.042 0.875 Z")),
    sh(`glove-${s}`, `hand-${s}`, "body", 0, far ? 0.6 : 6.2, 0.18 + tn, "gloves", "leather-glove", ARM(PAT.HAND)),
    sh(`glove-thumb-${s}`, `hand-${s}`, "detail", 1, far ? 0.62 : 6.25, 0.18 + tn, "gloves", "leather-glove", ARM(PAT.THUMB)),
    sh(`finger-tips-${s}`, `hand-${s}`, "detail", 2, far ? 0.63 : 6.26, 0.8 + tn, "hand-skin", "skin", ARM("M -0.026 0.715 C -0.024 0.690 -0.014 0.678 -0.004 0.675 C 0.01 0.674 0.022 0.69 0.026 0.72 Z")),
    sh(`thigh-${s}`, `thigh-${s}`, "body", 0, z0 + 1, 0.4 + tn, "jeans", "denim", LEGT(PAT.TROUSER)),
    sh(`shank-${s}`, `shank-${s}`, "body", 0, z0 + 1.02, 0.4 + tn, "jeans", "denim", SHK(PAT.SHANK)),
    sh(`shoe-rear-${s}`, `foot-${s}`, "body", 0, z0 + 1.1, 0.2 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_REAR)),
    sh(`shoe-toe-${s}`, `toe-${s}`, "body", 0, z0 + 1.1, 0.2 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_FRONT)),
    line(`shoe-sole-${s}`, `foot-${s}`, "detail", 1, z0 + 1.15, 0.85, "shoes", "canvas-sneaker", xp([[-0.05, 0.022], [0.17, 0.022]], k, 0, [0, 0.075], ANK)),
  ]; }),
  sh("pelvis-jeans", "pelvis", "body", 0, 3, 0.4, "jeans", "denim", U("M -0.115 1.04 C -0.138 0.99 -0.135 0.93 -0.110 0.86 L 0.100 0.86 C 0.130 0.93 0.132 0.99 0.118 1.04 Z")),
  sh("hoodie", "torso", "body", 0, 4, 0.36, "hoodie", "fleece", U(SWEAT)),
  sh("hoodie-pocket", "torso", "detail", 2, 4.1, 0.3, "hoodie", "fleece", U("M 0.0 1.02 L 0.12 1.0 L 0.12 0.95 L 0.0 0.96 Z")),
  line("hoodie-seam", "torso", "texture", 3, 4.05, 0.25, "hoodie", "fleece", xp([[-0.1, 1.3], [-0.04, 1.12]], k, 0, [0, 0.935], HIP)),
  sh("neck", "neck", "body", 0, 3.8, 0.8, "face-skin", "skin", NK(C.shapes.find((q) => q.id === "neck").d)),
  sh("hood", "neck", "body", 1, 4.5, 0.32, "hoodie", "fleece", NK("M -0.095 1.54 C -0.115 1.50 -0.09 1.46 -0.04 1.468 L 0.062 1.468 C 0.09 1.48 0.098 1.51 0.086 1.540 C 0.07 1.55 0.0 1.55 -0.04 1.55 C -0.07 1.55 -0.09 1.55 -0.095 1.54 Z")),
  sh("head", "head", "body", 0, 5, 0.84, "face-skin", "skin", HD(C.shapes.find((q) => q.id === "head").d)),
  sh("beanie", "head", "body", 1, 5.5, 0.28, "beanie", "knit-wool", HD("M -0.082 1.665 C -0.112 1.74 -0.06 1.805 0.025 1.80 C 0.09 1.795 0.108 1.745 0.100 1.708 C 0.06 1.722 -0.035 1.716 -0.082 1.665 Z")),
  sh("beanie-cuff", "head", "detail", 2, 5.55, 0.2, "beanie", "knit-wool", HD("M -0.083 1.668 C -0.04 1.712 0.06 1.72 0.101 1.708 L 0.098 1.69 C 0.05 1.70 -0.04 1.695 -0.078 1.648 Z")),
  ell("ear", "head", "detail", 2, 5.2, 0.8, "face-skin", "skin", HDp([-0.012, 1.645]), 0.012 * k, 0.02 * k),
  ell("eye", "head", "detail", 3, 5.6, 0.05, "face-skin", "skin", HDp([0.075, 1.667]), 0.007 * k, 0.004 * k),
  line("mouth-line", "head", "detail", 3, 5.6, 0.3, "face-skin", "skin", xp([[0.083, 1.598], [0.096, 1.598]], k, 0, [0.0139, 1.599], NTOP)),
];
const clips = [
  {
    id: "push-cycle", title: tx("Pushing along level ground", "Antrieb auf ebenem Boden"), description: tx("Four push strokes at 1.28 m/s: the gloved hands grip the rims behind the top, push through a 45 degree arc with the trunk leaning forward, release and swing back over the rims. The rear wheels turn 240 degrees and the castors three full turns per stroke, so the ground advances 1.278 m per stroke (treadmill frame, the chair stays in place). Spoke symmetry makes a restart seamless.", "Vier Antriebsschübe mit 1,28 m/s: Die behandschuhten Hände greifen die Reifen hinter dem Scheitel, schieben durch einen 45-Grad-Bogen mit nach vorn geneigtem Rumpf, lösen und schwingen über den Reifen zurück. Die Hinterräder drehen sich pro Schub um 240 Grad und die Lenkrollen um drei volle Umdrehungen, der Boden läuft also 1,278 m pro Schub vorbei (Laufbandsystem, der Stuhl bleibt stehen). Die Speichensymmetrie macht den Neustart nahtlos."),
    durationS: DUR, fps: 30, loop: false, rootMotion: "none", advanceM: r4(2 * Math.PI * RW * (SPIN / 360) * NC), keys: mkKeys(NC, "hands-on-rims", -4, -22),
    contacts: [{ anchor: "patch-rear", fromS: 0, toS: DUR }, { anchor: "patch-front", fromS: 0, toS: DUR }],
    phases: Array.from({ length: NC }, (_, i) => [{ id: `push-${i + 1}`, fromS: i, toS: r4(i + 0.1875), title: tx(`Push stroke ${i + 1}`, `Antriebsschub ${i + 1}`) }, { id: `recovery-${i + 1}`, fromS: r4(i + 0.1875), toS: i + 1, title: tx(`Recovery ${i + 1}`, `Rückführung ${i + 1}`) }]).flat(),
    events: Array.from({ length: NC }, (_, i) => [ev(i, "load-on", `Hands grip the rims (stroke ${i + 1})`, `Hände greifen die Reifen (Schub ${i + 1})`), ev(r4(i + 0.1875), "load-off", `Hands release the rims`, `Hände lösen die Reifen`), ev(i + 0.3, i % 2 ? "inhale" : "exhale", i % 2 ? "Breath in" : "Breath out", i % 2 ? "Einatmen" : "Ausatmen")]).flat(),
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 2.2 }, { t: 0.1, v: 3 }, { t: 0.6, v: 2.4 }, { t: 1.1, v: 3 }, { t: DUR, v: 2.6 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.8 }, { t: DUR, v: 1 }] }],
  },
  {
    id: "ramp-roll", title: tx("Pushing up the ramp", "Antrieb die Rampe hinauf"), description: tx("The same push cycle on a 1:12 ramp: the chair is tilted 4.8 degrees about the rear axle, the trunk leans forward up to 30 degrees and the activity rises to 3.2 met; the drawn ground line follows the ramp.", "Derselbe Antriebszyklus auf einer Rampe 1:12: Der Stuhl ist um 4,8 Grad um die Hinterachse gekippt, der Rumpf neigt sich bis 30 Grad vor und die Aktivität steigt auf 3,2 met; die gezeichnete Bodenlinie folgt der Rampe."),
    durationS: 2 * T1, fps: 30, loop: false, rootMotion: "none", advanceM: r4(2 * Math.PI * RW * (SPIN / 360) * 2 * 0.8), keys: mkKeys(2, "ramp-ascent", -14, -30, 0.8),
    contacts: [{ anchor: "patch-rear", fromS: 0, toS: 2 * T1 }, { anchor: "patch-front", fromS: 0, toS: 2 * T1 }],
    phases: [{ id: "first-push", fromS: 0, toS: 1, title: tx("First push on the slope", "Erster Schub an der Steigung") }, { id: "second-push", fromS: 1, toS: 2, title: tx("Second push on the slope", "Zweiter Schub an der Steigung") }],
    events: [ev(0, "load-on", "Hands grip the rims on the slope", "Hände greifen die Reifen an der Steigung"), ev(1, "load-on", "Second grip", "Zweites Greifen")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 3 }, { t: 1, v: 3.2 }, { t: 2, v: 3 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 2, v: 1 }] }],
  },
  {
    id: "rest-in-chair", title: tx("Resting in the chair", "Rast im Stuhl"), description: tx("Sitting still with the hands in the lap; slow breathing with a slight rise of the shoulders.", "Ruhiges Sitzen mit den Händen im Schoß; langsames Atmen mit leichtem Heben der Schultern."),
    durationS: 5, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "seated-level", ease: "smooth" }, { t: 2, pose: "seated-level", ease: "smooth", bones: { torso: { angleDeg: 1 }, head: { angleDeg: -1 } } }, { t: 5 - 0.001, pose: "seated-level", ease: "smooth" }].slice(0, 2).concat([{ t: 3.5, pose: "seated-level", ease: "smooth" }]),
    contacts: [{ anchor: "patch-rear", fromS: 0, toS: 5 }, { anchor: "patch-front", fromS: 0, toS: 5 }],
    phases: [{ id: "inhale", fromS: 0, toS: 2, title: tx("Inhale", "Einatmen") }, { id: "exhale", fromS: 2, toS: 5, title: tx("Exhale", "Ausatmen") }],
    events: [ev(0, "inhale", "Start of inhalation", "Beginn des Einatmens"), ev(2, "exhale", "Start of exhalation", "Beginn des Ausatmens")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 2, v: 0 }, { t: 2.5, v: 0.6 }, { t: 4, v: 0.3 }, { t: 5, v: 0 }] }],
  },
];
export default {
  id: "wheelchair-user",
  title: tx("Wheelchair User", "Rollstuhlnutzende Person"),
  summary: tx("A man in a manual wheelchair, rigged as one mechanical-anatomical system: the frame is the root, the rear wheels and castors are free revolute bones that turn with the distance rolled, and the gloved hands drive the hand rims in a push cycle; the wheel and castor patches are the ground contacts.", "Ein Mann in einem Aktivrollstuhl, als ein mechanisch-anatomisches System aufgebaut: Der Rahmen ist die Wurzel, Hinterräder und Lenkrollen sind frei drehbare Knochen, die sich mit der Rollstrecke drehen, und die behandschuhten Hände treiben die Greifreifen im Antriebszyklus an; die Aufstandsflächen von Rädern und Rollen sind die Bodenkontakte."),
  tags: ["human", "wheelchair", "accessibility", "mechanical", "push-cycle", "ramp"],
  provenance: {
    author: "Semio staffage content worker humans-a", method: "handcrafted", created: "2026-10-29",
    references: [...refs(DL, WI, "drillis-contini-1966", "dubois-1916", ISO, "iso-8996-2021", "iso-7730-2005", "ashrae-2021", GU), { id: CRP, citation: "Cooper, R. A. and De Luigi, A. J. (2014). Adaptive sports technology and biomechanics: wheelchairs. PM&R 6(8 Suppl), S31-S39; typical mass and geometry of ultralight manual wheelchairs (24 inch wheels, 100 mm to 140 mm castors)." }, { id: "ada-2010", citation: "2010 ADA Standards for Accessible Design, section 405: ramp running slope not steeper than 1:12." }],
    notes: tx("A 1.75 m, 75 kg man in an ultralight manual chair with 24 inch rear wheels (radius 0.305 m), 135 mm castors and a 0.42 m seat height; frame and wheel masses are typical product values. The wheels, hand-rim geometry and push cycle (45 degree arc, 240 degree wheel rotation per push at 1.28 m/s) are authored from published propulsion studies in rounded form. The spin offsets accumulate over the clip, therefore the push clips do not loop; the 12-spoke symmetry makes the restart seamless.", "Ein 1,75 m großer, 75 kg schwerer Mann in einem Ultraleicht-Aktivrollstuhl mit 24-Zoll-Hinterrädern (Radius 0,305 m), 135-mm-Lenkrollen und 0,42 m Sitzhöhe; Rahmen- und Radmassen sind typische Produktwerte. Räder, Greifreifengeometrie und Antriebszyklus (45-Grad-Bogen, 240 Grad Raddrehung pro Schub bei 1,28 m/s) sind in gerundeter Form nach veröffentlichten Antriebsstudien gezeichnet. Die Drehversätze summieren sich über den Clip, daher laufen die Antriebs-Clips nicht in Schleife; die 12-Speichen-Symmetrie macht den Neustart nahtlos."),
  },
  dimensions: { heightM: 1.45, widthM: 1.09, depthM: 0.7, massKg: 90.2 },
  materials: [
    mat("skin", "Skin", "Haut", 0.98, 0.6, "literature", "ashrae-2021"), mat("hair", "Hair", "Haar", 0.98, 0.85, "authored-estimate"),
    mat("fleece", "Fleece", "Fleece", 0.93, 0.65, "literature", "ashrae-2021"), mat("knit-wool", "Knitted wool", "Wollstrick", 0.9, 0.6, "literature", "ashrae-2021"), mat("leather-glove", "Leather glove", "Lederhandschuh", 0.9, 0.75, "authored-estimate"),
    mat("denim", "Denim", "Denim", 0.92, 0.65, "literature", "ashrae-2021"), mat("canvas-sneaker", "Canvas sneaker", "Stoffturnschuh", 0.9, 0.5, "authored-estimate"),
    mat("aluminium", "Anodised aluminium", "Eloxiertes Aluminium", 0.15, 0.4, "literature", "ashrae-2021"), mat("rubber", "Rubber tyre", "Gummireifen", 0.93, 0.9, "literature", "ashrae-2021"), mat("foam", "Cushion foam with cover", "Polsterschaum mit Bezug", 0.9, 0.6, "authored-estimate"),
  ],
  bones, anchors, shapes, poses, clips,
  thermal: {
    environment: ENV(10, 10, 0.7, 1, 0), baseMet: 1.2,
    regions: [
      region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", "core-resistance", 0.09, 0.2, 0.07, { coreTempC: 36.8, insulationM2KW: 0.04 }, "dubois-1916"),
      region("beanie", "Beanie", "Mütze", "clothing", "knit-wool", "core-resistance", 0.05, 0.15, 0.03, { coreTempC: 36.8, insulationM2KW: 0.12 }, ISO),
      region("hand-skin", "Bare fingertips", "Freie Fingerspitzen", "skin", "skin", "core-resistance", 0.02, 0.05, 0.02, { coreTempC: 34, insulationM2KW: 0.05 }, "dubois-1916"),
      region("gloves", "Push gloves", "Rollstuhlhandschuhe", "clothing", "leather-glove", "core-resistance", 0.07, 0.1, 0.03, { coreTempC: 35, insulationM2KW: 0.08 }, ISO),
      region("hoodie", "Hooded sweatshirt", "Kapuzenpullover", "clothing", "fleece", "core-resistance", 1.02, 0.3, 0.5, { coreTempC: 36.8, insulationM2KW: 0.19 }, ISO),
      region("jeans", "Jeans", "Jeans", "clothing", "denim", "core-resistance", 0.5, 0.13, 0.29, { coreTempC: 36.8, insulationM2KW: 0.12 }, ISO),
      region("shoes", "Sneakers", "Turnschuhe", "surface", "canvas-sneaker", "core-resistance", 0.13, 0.1, 0.06, { coreTempC: 34, insulationM2KW: 0.1 }, ISO, "authored-estimate"),
      region("frame-metal", "Wheelchair frame and rims", "Rollstuhlrahmen und Reifen", "surface", "aluminium", "passive", 0.55, 0.3, undefined, {}, ISO, "authored-estimate"),
      region("tyres", "Tyres", "Reifen", "tyre", "rubber", "passive", 0.35, 0.4, undefined, {}, ISO, "authored-estimate"),
      region("cushion", "Seat cushion and back sling", "Sitzkissen und Rückenbespannung", "surface", "foam", "passive", 0.4, 0.4, undefined, {}, ISO, "authored-estimate"),
    ],
    emitters: [{ id: "breath", kind: "breath", title: tx("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: -15, carries: "vapour", temperatureC: 34, velocityMS: 1.2, diameterM: 0.02, vapourKgPerKg: 0.0345, periodS: 3.2, dutyFrom: 0.4, dutyTo: 1, source: "literature", reference: GU }],
    assumptions: [tx("Body regions sum to the DuBois area of 1.90 m² (75 kg, 175 cm); the chair surfaces are additional passive areas.", "Die Körperregionen ergeben die DuBois-Fläche von 1,90 m² (75 kg, 175 cm); die Stuhlflächen sind zusätzliche passive Flächen."), tx("Hand-rim propulsion at 1.1 to 1.3 m/s is about 2.5 to 3 met; the contact of the thighs with the cushion is not reduced because the cushion is a separate passive region.", "Antrieb über Greifreifen bei 1,1 bis 1,3 m/s entspricht etwa 2,5 bis 3 met; der Kontakt der Oberschenkel mit dem Kissen wird nicht vermindert, weil das Kissen eine eigene passive Region ist.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [tx("The tyre patches of the rear wheels and the castors are the ground contacts; the person is rigidly attached to the chair and the weight is shared according to the elastic-support model of the solver.", "Die Aufstandsflächen der Hinterreifen und der Lenkrollen sind die Bodenkontakte; die Person ist starr mit dem Stuhl verbunden und das Gewicht verteilt sich nach dem Elastizitätsmodell des Lösers."), tx("The chair stays in place while the ground advances (treadmill frame); the push force on the rims is internal and not modelled as an external load.", "Der Stuhl bleibt stehen, während der Boden vorbeiläuft (Laufbandsystem); die Antriebskraft an den Reifen ist innerlich und nicht als äußere Last modelliert.")], clinical: false },
  presentation: { defaultPose: "seated-level", defaultClip: "push-cycle", viewpoints: ["elevation"] },
  exports: STD_EXPORTS,
};
