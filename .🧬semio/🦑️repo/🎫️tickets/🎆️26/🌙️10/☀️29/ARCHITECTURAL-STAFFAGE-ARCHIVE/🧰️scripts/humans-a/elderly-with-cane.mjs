// [DEBUG] design source: elderly-with-cane (male, 74 y, 1.70 m straight / 1.62 m stooped, 70 kg).
import { readFileSync, existsSync } from "node:fs";
import { tx, refs, rev, FIXED, deg, rad, r4, sm5, R, xf, xp, xpt, armIK, makeGait, pt, sh, cap, ell, line, mass, STD_EXPORTS, ENV, mat, region } from "./common.mjs";
import C, { PAT, FINGERS } from "./walking-commuter.mjs";
const DL = "de-leva-1996", WI = "winter-2009", ISO = "iso-9920-2007", GU = "gupta-2010";
const ADV = existsSync(new URL("./advance-elderly-with-cane.json", import.meta.url)) ? JSON.parse(readFileSync(new URL("./advance-elderly-with-cane.json", import.meta.url), "utf8")) : {};
const k = 0.93, T = 1.7, S = 0.95, CANE_L = 0.87;
const P = { k, S, T, L1: 0.435 * k, L2: 0.425 * k, XH: 0.15, hipBase: 0.935 * k - 0.015, hipAmp: 0.025, hipMinAt: 0.31, flatTo: 0.14, heelOff: 0.34, to: 0.64, phi0: 6, phiTO: -32, bump: 0.03 };
const G = makeGait(P);
const hip0 = [0, 0.935 * k], pel = 0.1 * k, torL = 0.465 * k, neckL = 0.1 * k, headL = 0.18 * k;
const WA = { torso: 74, neck: 56, head: 76 };
const dir = (a) => [Math.cos(rad(a)), Math.sin(rad(a))];
const add = (p, v, s = 1) => [p[0] + v[0] * s, p[1] + v[1] * s];
const LUMB = add(hip0, [0, 1], pel), C7 = add(LUMB, dir(WA.torso), torL), NTOP = add(C7, dir(WA.neck), neckL), SHO = add(LUMB, dir(WA.torso), 0.42 * k);
const UB = (d) => xf(d, k, WA.torso - 90, [0, 0.935], hip0), UBp = (p) => xpt(p, k, WA.torso - 90, [0, 0.935], hip0);
const NK = (d) => xf(d, k, WA.neck - 82, [0, 1.5], C7), HD = (d) => xf(d, k, WA.head - 90, [0.0139, 1.599], NTOP);
const HDp = (p) => xpt(p, k, WA.head - 90, [0.0139, 1.599], NTOP);
const LEG = (d) => xf(d, k, 0, [0, 0.935], hip0), SHK = (d) => xf(d, k, 0, [0, 0.5], [0, 0.5 * k]), FT = (d) => xf(d, k, 0, [0, 0.075], [0, 0.075 * k]), FTp = (p) => xpt(p, k, 0, [0, 0.075], [0, 0.075 * k]);
const ARM = (d) => xf(d, k, 0, [0, 1.455], SHO), ARMp = (p) => xpt(p, k, 0, [0, 1.455], SHO);
const A1 = 0.33 * k, A2 = 0.26 * k, HL = 0.19 * k, WRIST0 = [SHO[0], SHO[1] - A1 - A2];
const aq = (a) => ({ angleDeg: r4(a) });
const handMid0 = [WRIST0[0], WRIST0[1] - HL / 2], TIP0 = [handMid0[0], handMid0[1] - CANE_L];

const GRIPX = 0.2, TIPLEAD = 0.31;
const gripY = (tipx, tipy, gx) => tipy + Math.sqrt(CANE_L ** 2 - (gx - tipx) ** 2);
const caneState = (u) => {
  const hip = G.hipAt(u), gx = hip[0] + GRIPX; let tx_, ty;
  if (u <= P.to) { tx_ = GRIPX + TIPLEAD; ty = 0; }
  else { const w = (u - P.to) / (1 - P.to), e = sm5(w); tx_ = GRIPX + TIPLEAD + S * e; ty = 0.045 * Math.sin(Math.PI * w) ** 4; }
  const gy = gripY(tx_, ty, gx);
  return { grip: [gx, gy], tip: [tx_, ty] };
};
const shAt = (th, hipx = 0, dy = 0) => { const q = R(th, [SHO[0] - LUMB[0], SHO[1] - LUMB[1]]); return [hipx + LUMB[0] + q[0], dy + LUMB[1] + q[1]]; };
const armSolve = (side, shoulder, grip, tip, th = 0, extra = {}) => {
  const dx = grip[0] - shoulder[0], dy = grip[1] - shoulder[1], d = Math.hypot(dx, dy);
  const wr = [grip[0] - HL / 2 * dx / d, grip[1] - HL / 2 * dy / d];
  const r = armIK(shoulder, wr, A1, A2), fw = -90 + r.arm + r.forearm;
  const caneW = deg(Math.atan2(tip[1] - grip[1], tip[0] - grip[0]));
  return { [`arm-${side}`]: aq(r.arm - th), [`forearm-${side}`]: aq(r.forearm), [`hand-${side}`]: aq(0), cane: aq(caneW - fw), ...extra };
};
const shoulderW = (hipx) => [hipx + SHO[0], SHO[1] - (hip0[1] - (P.hipBase))];
export const gait = (u) => {
  const hip = G.hipAt(u), st = caneState(u), c = Math.cos(2 * Math.PI * u), s = Math.sin(2 * Math.PI * u);
  const th = -1 + 0.8 * s, sh2 = shAt(th, hip[0], hip[1] - hip0[1]);
  return { ...G.legs(u), ...armSolve("r", sh2, st.grip, st.tip, th), "arm-l": aq(-10 * c), "forearm-l": aq(18 + 4 * c), "hand-l": aq(3), torso: aq(th), neck: aq(2), head: aq(0) };
};
const FEET = ["heel-l", "ball-l", "heel-r", "ball-r"];
const TIPX = 0.34;
const gripOn = (gx) => [gx, Math.sqrt(CANE_L ** 2 - (gx - TIPX) ** 2)];
const standBones = (extra = {}) => ({ ...armSolve("r", SHO, gripOn(0.2), [TIPX, 0], 0), "arm-l": aq(6), "forearm-l": aq(16), "hand-l": aq(3), ...extra });
const leanBones = () => ({ ...armSolve("r", shAt(-8), gripOn(0.31), [TIPX, 0], -8), "arm-l": aq(14), "forearm-l": aq(22), "hand-l": aq(3), torso: aq(-8), neck: aq(5), head: aq(3) });
const restBones = () => ({ ...armSolve("r", shAt(5), gripOn(0.34), [TIPX, 0], 5), "arm-l": aq(34), "forearm-l": aq(86), "hand-l": aq(-10), torso: aq(5), neck: aq(-6), head: aq(-6) });
const PAR = { stand: { th: 0, gx: 0.2, al: [6, 16, 3], neck: 3, head: 1 }, lean: { th: -8, gx: 0.31, al: [14, 22, 3], neck: 5, head: 3 }, rest: { th: 5, gx: 0.34, al: [34, 86, -10], neck: -6, head: -6 } };
const mixPar = (a, b, w) => ({ th: a.th + (b.th - a.th) * w, gx: a.gx + (b.gx - a.gx) * w, al: a.al.map((v, i) => v + (b.al[i] - v) * w), neck: a.neck + (b.neck - a.neck) * w, head: a.head + (b.head - a.head) * w });
const parBones = (q) => ({ ...armSolve("r", shAt(q.th), gripOn(q.gx), [TIPX, 0], q.th), "arm-l": aq(q.al[0]), "forearm-l": aq(q.al[1]), "hand-l": aq(q.al[2]), torso: aq(q.th), neck: aq(q.neck), head: aq(q.head) });
const sub = (t0, t1, a, b, pose, n = 6) => Array.from({ length: n }, (_, i) => ({ t: r4(t0 + ((t1 - t0) * i) / n), pose, ease: "smooth", bones: parBones(mixPar(a, b, sm5(i / n))) }));
const pose = (id, en, de, dEn, dDe, bones, contacts, met, breath) => ({ id, title: tx(en, de), description: tx(dEn, dDe), bones, ground: "auto", contacts, thermal: { activityMet: met, emitters: { breath } }, loads: [] });
const gaitPose = (id, en, de, dEn, dDe, u, contacts) => pose(id, en, de, dEn, dDe, gait(u), contacts, 2, 0.8);
const poses = [
  pose("stooped-standing", "Stooped standing", "Gebeugtes Stehen", "Standing on both feet with a rounded upper back, the cane planted beside the foot and lightly loaded.", "Stand auf beiden Füßen mit rundem Rücken, der Stock neben dem Fuß aufgesetzt und leicht belastet.", standBones(), [...FEET, "cane-tip"], 1.2, 0.5),
  pose("cane-planted", "Leaning on the planted cane", "Auf den aufgesetzten Stock gestützt", "The cane is set one step ahead and the trunk leans onto it before the weak foot follows.", "Der Stock steht einen Schritt voraus und der Rumpf lehnt sich auf ihn, bevor der schwache Fuß folgt.", leanBones(), [...FEET, "cane-tip"], 1.5, 0.7),
  pose("resting-on-cane", "Resting with the hand on the crook", "Rast mit der Hand auf dem Griff", "Standing still, the near hand resting on the crook handle, the far hand on the near wrist, head up.", "Ruhiger Stand, die nahe Hand auf dem Krückgriff, die ferne Hand am nahen Handgelenk, Kopf erhoben.", restBones(), [...FEET, "cane-tip"], 1.1, 0.4),
  gaitPose("contact-left", "Left heel strike with the cane", "Linker Fersenkontakt mit Stock", "Cane tip and left heel land together one step ahead of the body.", "Stockspitze und linke Ferse setzen gemeinsam einen Schritt vor dem Körper auf.", 0, ["heel-l", "ball-r", "cane-tip"]),
  gaitPose("loading-left", "Left foot flat on the cane side", "Linker Fuß flach", "Left foot flat, right toe-off, the body moving over the planted cane.", "Linker Fuß flach, rechter Abdruck, der Körper bewegt sich über den aufgesetzten Stock.", 0.14, ["heel-l", "ball-l", "ball-r", "cane-tip"]),
  gaitPose("passing-left", "Left mid-stance over the cane", "Linke Standbeinmitte über dem Stock", "Body above the left foot and the cane, the right leg swinging through.", "Körper über linkem Fuß und Stock, das rechte Bein schwingt durch.", 0.31, ["heel-l", "ball-l", "cane-tip"]),
  gaitPose("contact-right", "Right heel strike", "Rechter Fersenkontakt", "Right heel strike while the left foot and the cane still carry.", "Rechter Fersenkontakt, während linker Fuß und Stock noch tragen.", 0.5, ["heel-r", "ball-l", "cane-tip"]),
  gaitPose("loading-right", "Right foot flat, cane lifting", "Rechter Fuß flach, Stock hebt ab", "Right foot flat, left toe-off, the cane lifts for the next step.", "Rechter Fuß flach, linker Abdruck, der Stock hebt für den nächsten Schritt ab.", 0.64, ["heel-r", "ball-r"]),
  gaitPose("passing-right", "Right mid-stance, cane swinging", "Rechte Standbeinmitte, Stock schwingt", "Body above the right foot while the cane and left leg swing forward together.", "Körper über dem rechten Fuß, während Stock und linkes Bein gemeinsam nach vorn schwingen.", 0.82, ["heel-r", "ball-r"]),
];
const N = 60;
const ev = (t, kind, en, de, target) => ({ t, kind, title: tx(en, de), ...(target ? { target } : {}) });
const sec = (f) => r4(f * T);
const bones = [
  { id: "pelvis", parent: null, title: tx("Pelvis", "Becken"), at: hip0, worldAngleDeg: 90, lengthM: pel, joint: rev(-15, 15), mass: mass(8, 0.057, DL) },
  { id: "torso", parent: "pelvis", title: tx("Stooped torso with jacket", "Gebeugter Rumpf mit Jacke"), at: LUMB, worldAngleDeg: WA.torso, lengthM: torL, joint: rev(-35, 30), mass: mass(23.9, 0.2, DL) },
  { id: "neck", parent: "torso", title: tx("Neck", "Hals"), at: C7, worldAngleDeg: WA.neck, lengthM: neckL, joint: rev(-30, 40), mass: mass(0.7, 0.047, DL, { source: "authored-estimate" }) },
  { id: "head", parent: "neck", title: tx("Head with cap", "Kopf mit Schiebermütze"), at: NTOP, worldAngleDeg: WA.head, lengthM: headL, joint: rev(-30, 30), mass: mass(4.45, 0.075, DL) },
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `arm-${s}`, parent: "torso", title: tx(`${n[0]} upper arm`, `${n[1]} Oberarm`), at: SHO, worldAngleDeg: -90, lengthM: A1, joint: rev(-60, 170), mass: mass(2, 0.19 * k, DL) },
    { id: `forearm-${s}`, parent: `arm-${s}`, title: tx(`${n[0]} forearm`, `${n[1]} Unterarm`), at: [SHO[0], SHO[1] - A1], worldAngleDeg: -90, lengthM: A2, joint: rev(0, 150), mass: mass(1.18, 0.119 * k, DL) },
    { id: `hand-${s}`, parent: `forearm-${s}`, title: tx(`${n[0]} hand`, `${n[1]} Hand`), at: WRIST0, worldAngleDeg: -90, lengthM: HL, joint: rev(-60, 60), mass: mass(0.43, 0.096 * k, WI) },
    { id: `thigh-${s}`, parent: "pelvis", title: tx(`${n[0]} thigh`, `${n[1]} Oberschenkel`), at: hip0, worldAngleDeg: -90, lengthM: P.L1, joint: rev(-30, 110), mass: mass(10.06, 0.178 * k, DL) },
    { id: `shank-${s}`, parent: `thigh-${s}`, title: tx(`${n[0]} shank`, `${n[1]} Unterschenkel`), at: [0, 0.5 * k], worldAngleDeg: -90, lengthM: P.L2, joint: rev(-150, 0), mass: mass(3.13, 0.1895 * k, DL) },
    { id: `foot-${s}`, parent: `shank-${s}`, title: tx(`${n[0]} foot and shoe`, `${n[1]} Fuß mit Schuh`), at: [0, 0.075 * k], worldAngleDeg: -23.8, lengthM: 0.1858 * k, joint: rev(-60, 30), mass: mass(1.3, FTp([0.071, 0.04]), DL) },
    { id: `toe-${s}`, parent: `foot-${s}`, title: tx(`${n[0]} forefoot`, `${n[1]} Vorfuß`), at: FTp([0.17, 0]), worldAngleDeg: 0, lengthM: 0.065 * k, joint: rev(0, 65), mass: mass(0.1, FTp([0.2, 0.03]), DL, { source: "authored-estimate" }) },
  ]; }),
  { id: "cane", parent: "hand-r", title: tx("Walking cane", "Gehstock"), at: handMid0, worldAngleDeg: -90, lengthM: CANE_L, joint: rev(-130, 130), mass: mass(0.35, CANE_L * 0.45, DL, { source: "authored-estimate", reference: undefined }) },
];
const anchors = [
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `heel-${s}`, bone: `foot-${s}`, at: FTp([-0.045, 0]), kind: "contact", title: tx(`${n[0]} heel`, `${n[1]} Ferse`), support: "contact", normalDeg: 90, footprintLengthM: 0.065, footprintWidthM: 0.06, frictionCoefficient: 0.9 },
    { id: `ball-${s}`, bone: `foot-${s}`, at: FTp([0.17, 0]), kind: "contact", title: tx(`${n[0]} ball of foot`, `${n[1]} Fußballen`), support: "contact", normalDeg: 90, footprintLengthM: 0.095, footprintWidthM: 0.09, frictionCoefficient: 0.9 }]; }),
  { id: "cane-tip", bone: "cane", at: TIP0, kind: "contact", title: tx("Rubber cane ferrule", "Gummi-Zwinge des Stocks"), support: "contact", normalDeg: 90, footprintLengthM: 0.03, footprintWidthM: 0.03, frictionCoefficient: 0.8 },
  { id: "mouth", bone: "head", at: HDp([0.094, 1.594]), kind: "emitter", title: tx("Mouth and nose", "Mund und Nase") },
  { id: "cane-grip", bone: "hand-r", at: add(WRIST0, [0, -1], HL / 2), kind: "attachment", title: tx("Hand on the crook handle", "Hand am Krückgriff") },
];
const jacket = "M -0.03 1.50 C -0.09 1.49 -0.135 1.44 -0.15 1.37 C -0.158 1.30 -0.12 1.22 -0.115 1.12 C -0.12 1.04 -0.125 0.96 -0.125 0.86 L 0.13 0.86 C 0.135 0.95 0.135 1.03 0.132 1.10 C 0.13 1.2 0.138 1.26 0.14 1.31 C 0.135 1.39 0.108 1.45 0.062 1.482 C 0.04 1.495 0.01 1.505 -0.03 1.5 Z";
const shapes = [
  ...["l", "r"].flatMap((s) => { const far = s === "l"; const tn = far ? -0.07 : 0; const z0 = far ? 0 : 2; return [
    sh(`sleeve-${s}`, `arm-${s}`, "body", 0, far ? 0.5 : 6, 0.5 + tn, "jacket", "tweed", ARM(PAT.SLEEVE)),
    sh(`forearm-${s}`, `forearm-${s}`, "body", 0, far ? 0.5 : 6, 0.5 + tn, "jacket", "tweed", ARM(PAT.FOREARM)),
    sh(`hand-${s}`, `hand-${s}`, "body", 0, far ? 0.6 : 6.2, 0.78 + tn, "hand-skin", "skin", ARM(PAT.HAND)),
    ...FINGERS.map((p, i) => line(`fingers-${s}-${i}`, `hand-${s}`, "detail", 2, far ? 0.63 : 6.26, 0.5, "hand-skin", "skin", xp(p, k, 0, [0, 1.455], SHO))),
    sh(`thumb-${s}`, `hand-${s}`, "detail", 1, far ? 0.62 : 6.25, 0.76 + tn, "hand-skin", "skin", ARM(PAT.THUMB)),
    sh(`thigh-${s}`, `thigh-${s}`, "body", 0, z0 + 1, 0.4 + tn, "trousers", "wool-trouser", LEG(PAT.TROUSER)),
    sh(`shank-${s}`, `shank-${s}`, "body", 0, z0 + 1.02, 0.4 + tn, "trousers", "wool-trouser", SHK(PAT.SHANK)),
    sh(`shoe-rear-${s}`, `foot-${s}`, "body", 0, z0 + 1.1, 0.3 + tn, "shoes", "leather-shoe", FT(PAT.SHOE_REAR)),
    sh(`shoe-toe-${s}`, `toe-${s}`, "body", 0, z0 + 1.1, 0.3 + tn, "shoes", "leather-shoe", FT(PAT.SHOE_FRONT)),
    line(`shoe-sole-${s}`, `foot-${s}`, "detail", 1, z0 + 1.15, 0.05, "shoes", "leather-shoe", xp([[-0.05, 0.022], [0.17, 0.022]], k, 0, [0, 0.075], [0, 0.075 * k])),
  ]; }),
  sh("pelvis-trousers", "pelvis", "body", 0, 3, 0.4, "trousers", "wool-trouser", UB("M -0.115 1.04 C -0.138 0.99 -0.135 0.93 -0.110 0.86 L 0.100 0.86 C 0.130 0.93 0.132 0.99 0.118 1.04 Z")),
  sh("jacket", "torso", "body", 0, 4, 0.5, "jacket", "tweed", UB(jacket)),
  sh("jacket-hump", "torso", "detail", 1, 4.05, 0.42, "jacket", "tweed", UB("M -0.14 1.43 C -0.19 1.38 -0.19 1.30 -0.14 1.22 C -0.145 1.30 -0.135 1.38 -0.12 1.44 Z")),
  line("jacket-front", "torso", "detail", 1, 4.1, 0.15, "jacket", "tweed", xp([[0.062, 1.482], [0.09, 1.30], [0.12, 0.88]], k, WA.torso - 90, [0, 0.935], hip0)),
  sh("jacket-pocket", "torso", "detail", 2, 4.1, 0.38, "jacket", "tweed", UB("M -0.005 0.98 L 0.105 0.98 L 0.105 0.93 L -0.005 0.93 Z")),
  line("jacket-check-1", "torso", "texture", 3, 4.05, 0.3, "jacket", "tweed", xp([[-0.1, 1.38], [0.11, 1.38]], k, WA.torso - 90, [0, 0.935], hip0)),
  line("jacket-check-2", "torso", "texture", 3, 4.05, 0.3, "jacket", "tweed", xp([[-0.1, 1.15], [0.11, 1.15]], k, WA.torso - 90, [0, 0.935], hip0)),
  line("jacket-check-3", "torso", "texture", 3, 4.05, 0.3, "jacket", "tweed", xp([[0.0, 1.45], [0.0, 0.88]], k, WA.torso - 90, [0, 0.935], hip0)),
  ell("jacket-button", "torso", "detail", 3, 4.2, 0.1, "jacket", "tweed", UBp([0.12, 1.12]), 0.008, 0.008),
  sh("neck", "neck", "body", 0, 3.8, 0.78, "face-skin", "skin", NK(C.shapes.find((q) => q.id === "neck").d)),
  sh("collar", "neck", "body", 1, 4.5, 0.45, "jacket", "tweed", NK("M -0.075 1.535 C -0.088 1.505 -0.078 1.478 -0.04 1.468 L 0.062 1.468 C 0.090 1.478 0.098 1.51 0.086 1.540 C 0.07 1.556 0.02 1.552 -0.02 1.55 C -0.05 1.55 -0.07 1.545 -0.075 1.535 Z")),
  sh("head", "head", "body", 0, 5, 0.8, "face-skin", "skin", HD(C.shapes.find((q) => q.id === "head").d)),
  sh("hair-white", "head", "body", 1, 5.4, 0.92, "hair", "hair", HD("M 0.0 1.745 C -0.05 1.752 -0.1 1.72 -0.105 1.67 C -0.108 1.63 -0.09 1.61 -0.07 1.606 C -0.068 1.64 -0.06 1.70 -0.03 1.725 Z")),
  sh("cap", "head", "body", 1, 5.6, 0.32, "cap", "tweed", HD("M -0.075 1.735 C -0.08 1.775 -0.01 1.795 0.045 1.775 C 0.085 1.76 0.115 1.735 0.13 1.72 C 0.135 1.712 0.12 1.708 0.105 1.711 C 0.075 1.715 0.02 1.722 -0.02 1.722 C -0.05 1.722 -0.07 1.725 -0.075 1.735 Z")),
  ell("ear", "head", "detail", 2, 5.2, 0.76, "face-skin", "skin", HDp([-0.012, 1.645]), 0.013 * k, 0.022 * k),
  ell("eye", "head", "detail", 3, 5.7, 0.05, "face-skin", "skin", HDp([0.075, 1.667]), 0.007 * k, 0.004 * k),
  line("glasses", "head", "detail", 3, 5.7, 0.1, "face-skin", "skin", xp([[0.06, 1.668], [0.09, 1.668], [0.0, 1.67]], k, WA.head - 90, [0.0139, 1.599], NTOP)),
  line("wrinkle", "head", "texture", 3, 5.7, 0.4, "face-skin", "skin", xp([[0.07, 1.65], [0.09, 1.648]], k, WA.head - 90, [0.0139, 1.599], NTOP)),
  cap("cane-shaft", "cane", "body", 0, 6.3, 0.45, "cane-wood", "ash-wood", [handMid0[0], handMid0[1] - 0.05], [TIP0[0], TIP0[1] + 0.03], 0.0105, 0.0095),
  sh("cane-crook", "cane", "body", 0, 6.35, 0.4, "cane-wood", "ash-wood", `M ${handMid0[0] - 0.012} ${handMid0[1] + 0.035} C ${handMid0[0] - 0.012} ${handMid0[1] + 0.085} ${handMid0[0] + 0.07} ${handMid0[1] + 0.09} ${handMid0[0] + 0.075} ${handMid0[1] + 0.045} C ${handMid0[0] + 0.077} ${handMid0[1] + 0.03} ${handMid0[0] + 0.065} ${handMid0[1] + 0.022} ${handMid0[0] + 0.055} ${handMid0[1] + 0.026} C ${handMid0[0] + 0.06} ${handMid0[1] + 0.045} ${handMid0[0] + 0.02} ${handMid0[1] + 0.06} ${handMid0[0] + 0.012} ${handMid0[1] + 0.03} Z`),
  cap("cane-ferrule", "cane", "body", 0, 6.4, 0.05, "cane-wood", "rubber", [TIP0[0], TIP0[1] + 0.05], [TIP0[0], TIP0[1] + 0.0125], 0.0125, 0.012),
  line("cane-grain", "cane", "texture", 3, 6.32, 0.2, "cane-wood", "ash-wood", [[TIP0[0] + 0.004, TIP0[1] + 0.2], [TIP0[0] + 0.004, TIP0[1] + 0.55]]),
];
const walkKeys = Array.from({ length: N }, (_, i) => ({ t: r4((i / N) * T), pose: "contact-left", ease: "smooth", bones: gait(i / N) }));
const clips = [
  {
    id: "cane-gait", title: tx("Three-point cane gait", "Dreipunktgang mit Stock"), description: tx("The cane tip lands together with the weak (left) foot and carries a share of the weight through the whole left stance; then the cane and the left leg swing forward together while the right foot carries alone. Slow walking at about 0.56 m/s.", "Die Stockspitze setzt gemeinsam mit dem schwachen (linken) Fuß auf und trägt während der ganzen linken Standphase einen Teil des Gewichts; dann schwingen Stock und linkes Bein gemeinsam nach vorn, während der rechte Fuß allein trägt. Langsames Gehen mit etwa 0,56 m/s."),
    durationS: T, fps: 30, loop: true, rootMotion: "plant", advanceM: ADV["cane-gait"] ?? S, keys: walkKeys,
    contacts: [{ anchor: "heel-l", fromS: 0, toS: sec(P.heelOff) }, { anchor: "ball-l", fromS: sec(P.flatTo), toS: sec(P.to) }, { anchor: "cane-tip", fromS: 0, toS: sec(P.to) }, { anchor: "heel-r", fromS: sec(0.5), toS: sec(0.5 + P.heelOff) }, { anchor: "ball-r", fromS: sec(0.5 + P.flatTo), toS: T }, { anchor: "ball-r", fromS: 0, toS: sec(P.to - 0.5) }],
    phases: [{ id: "left-stance-with-cane", fromS: 0, toS: sec(P.to), title: tx("Left stance with the cane", "Linke Standphase mit Stock") }, { id: "cane-and-left-swing", fromS: sec(P.to), toS: T, title: tx("Cane and left leg swing", "Stock und linkes Bein schwingen") }],
    events: [ev(0, "contact-on", "Cane tip and left heel land", "Stockspitze und linke Ferse landen", "cane-tip"), ev(0, "contact-on", "Left heel strike", "Linker Fersenkontakt", "heel-l"), ev(sec(P.to - 0.5), "contact-off", "Right toe-off", "Rechter Abdruck", "ball-r"), ev(sec(0.5), "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(sec(P.to), "contact-off", "Cane lifts, left toe-off", "Stock hebt ab, linker Abdruck", "cane-tip"), ev(0.5, "inhale", "Inhalation", "Einatmen"), ev(1.2, "exhale", "Exhalation", "Ausatmen")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.8 }, { t: 0.85, v: 0.6 }, { t: T, v: 0.8 }] }],
  },
  {
    id: "rest-on-cane", title: tx("Stopping to rest on the cane", "Rast auf den Stock gestützt"), description: tx("From stooped standing the person leans onto the planted cane, then straightens the head and lets the hand settle on the crook while breathing deeper.", "Aus gebeugtem Stand lehnt sich die Person auf den aufgesetzten Stock, richtet dann den Kopf auf und lässt die Hand auf dem Griff ruhen, während sie tiefer atmet."),
    durationS: 6, fps: 30, loop: false, rootMotion: "none",
    keys: [...sub(0, 1.6, PAR.stand, PAR.lean, "stooped-standing"), ...sub(1.6, 3, PAR.lean, PAR.lean, "stooped-standing", 2), ...sub(3, 4.8, PAR.lean, PAR.rest, "stooped-standing"), { t: 4.8, pose: "resting-on-cane", ease: "smooth" }, { t: 6, pose: "resting-on-cane", ease: "smooth" }],
    contacts: [...FEET, "cane-tip"].map((anchor) => ({ anchor, fromS: 0, toS: 6 })),
    phases: [{ id: "lean", fromS: 0, toS: 1.6, title: tx("Leaning onto the cane", "Auf den Stock lehnen") }, { id: "hold", fromS: 1.6, toS: 3, title: tx("Holding the lean", "Lehnen halten") }, { id: "settle", fromS: 3, toS: 6, title: tx("Settling on the crook", "Auf den Griff stützen") }],
    events: [ev(1.6, "load-on", "Cane carries the lean", "Stock trägt die Neigung"), ev(2.2, "inhale", "Deep breath", "Tiefes Einatmen"), ev(3.6, "exhale", "Long exhalation", "Langes Ausatmen")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.2 }, { t: 1.6, v: 1.5 }, { t: 3, v: 1.4 }, { t: 6, v: 1.1 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.5 }, { t: 3, v: 0.8 }, { t: 6, v: 0.4 }] }],
  },
];
export default {
  id: "elderly-with-cane",
  title: tx("Elderly Person with Cane", "Ältere Person mit Gehstock"),
  summary: tx("A 74-year-old man with a rounded upper back, flat cap and tweed jacket walking with a wooden crook cane in three-point gait: the cane tip is a third ground contact that shares the weight through the weak-side stance.", "Ein 74-jähriger Mann mit rundem Rücken, Schiebermütze und Tweedjacke geht mit einem Krückstock im Dreipunktgang: Die Stockspitze ist ein dritter Bodenkontakt, der in der Standphase der schwachen Seite einen Teil des Gewichts trägt."),
  tags: ["human", "elderly", "cane", "three-point-gait", "kyphosis"],
  provenance: {
    author: "Semio staffage content worker humans-a", method: "handcrafted", created: "2026-10-29",
    references: refs(DL, WI, "drillis-contini-1966", "perry-burnfield-2010", "dubois-1916", ISO, "iso-8996-2021", "iso-7730-2005", "ashrae-2021", GU),
    notes: tx("A 70 kg man, stature 1.70 m straight and about 1.62 m with the authored 16 degree forward torso inclination and flexed neck; segment masses from the de Leva male fractions plus 2.3 kg of clothing, lengths scaled by 0.93. Gait key frames from inverse kinematics of smooth foot paths with shortened strides; the cane tip path and the arm that holds it are solved so that the tip stays fixed in the stance.", "Ein 70 kg schwerer Mann, Körpergröße 1,70 m aufrecht und etwa 1,62 m mit der gezeichneten Rumpfneigung von 16 Grad und gebeugtem Hals; Segmentmassen nach den männlichen Anteilen von de Leva plus 2,3 kg Kleidung, Längen mit 0,93 skaliert. Ganghaltungen aus inverser Kinematik glatter Fußbahnen mit verkürzten Schritten; die Bahn der Stockspitze und der haltende Arm sind so gelöst, dass die Spitze in der Standphase fest bleibt."),
  },
  dimensions: { heightM: 1.65, widthM: 0.54, depthM: 0.48, massKg: 73.8 },
  materials: [
    mat("skin", "Aged skin", "Gealterte Haut", 0.98, 0.6, "literature", "ashrae-2021"), mat("hair", "White hair", "Weißes Haar", 0.98, 0.5, "authored-estimate"),
    mat("tweed", "Tweed wool", "Tweedwolle", 0.92, 0.7, "literature", "ashrae-2021"), mat("wool-trouser", "Wool trousers", "Wollhose", 0.92, 0.7, "literature", "ashrae-2021"),
    mat("leather-shoe", "Leather walking shoe", "Leder-Wanderschuh", 0.9, 0.75, "authored-estimate"), mat("ash-wood", "Ash wood", "Eschenholz", 0.9, 0.55, "literature", "ashrae-2021"), mat("rubber", "Rubber", "Gummi", 0.93, 0.9, "literature", "ashrae-2021"),
  ],
  bones, anchors, shapes, poses, clips,
  thermal: {
    environment: ENV(10, 10, 0.8, 1.5, 0), baseMet: 1.2,
    regions: [
      region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", "core-resistance", 0.1, 0.2, 0.07, { coreTempC: 36.4, insulationM2KW: 0.05 }, "dubois-1916"),
      region("hair", "Hair", "Haar", "hair", "hair", "core-resistance", 0.03, 0.15, 0.02, { coreTempC: 36.4, insulationM2KW: 0.07 }, "dubois-1916", "authored-estimate"),
      region("cap", "Flat cap", "Schiebermütze", "clothing", "tweed", "core-resistance", 0.03, 0.15, 0.01, { coreTempC: 36.4, insulationM2KW: 0.1 }, ISO),
      region("hand-skin", "Bare hands", "Unbedeckte Hände", "skin", "skin", "core-resistance", 0.09, 0.1, 0.04, { coreTempC: 33, insulationM2KW: 0.06 }, "dubois-1916"),
      region("jacket", "Tweed jacket on torso and arms", "Tweedjacke an Rumpf und Armen", "clothing", "tweed", "core-resistance", 0.97, 0.3, 0.5, { coreTempC: 36.4, insulationM2KW: 0.24 }, ISO),
      region("trousers", "Trousers", "Hose", "clothing", "wool-trouser", "core-resistance", 0.5, 0.13, 0.3, { coreTempC: 36.4, insulationM2KW: 0.15 }, ISO),
      region("shoes", "Shoes", "Schuhe", "surface", "leather-shoe", "core-resistance", 0.11, 0.1, 0.06, { coreTempC: 33, insulationM2KW: 0.12 }, ISO, "authored-estimate"),
      region("cane-wood", "Cane", "Gehstock", "surface", "ash-wood", "passive", 0.07, 0.05, undefined, {}, ISO, "authored-estimate"),
    ],
    emitters: [{ id: "breath", kind: "breath", title: tx("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: -20, carries: "vapour", temperatureC: 33.5, velocityMS: 1.1, diameterM: 0.018, vapourKgPerKg: 0.033, periodS: 3.5, dutyFrom: 0.4, dutyTo: 1, source: "literature", reference: GU }],
    assumptions: [tx("Body regions sum to the DuBois area of 1.80 m² (70 kg, 165 cm); the cane is an additional passive surface.", "Die Körperregionen ergeben die DuBois-Fläche von 1,80 m² (70 kg, 165 cm); der Stock ist eine zusätzliche passive Fläche."), tx("Slow walking at 2 km/h is 2.0 met (ISO 8996); core temperature 36.4 °C and cooler hands reflect reduced perfusion at higher age.", "Langsames Gehen mit 2 km/h entspricht 2,0 met (ISO 8996); Kerntemperatur 36,4 °C und kühlere Hände bilden die geringere Durchblutung im Alter ab.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [tx("The cane tip is a unilateral contact with rubber friction; the share it carries follows the elastic-support model of the solver and is not a clinical load measurement.", "Die Stockspitze ist ein einseitiger Kontakt mit Gummireibung; ihr Lastanteil folgt dem Elastizitätsmodell des Lösers und ist keine klinische Lastmessung."), tx("Segments are point masses at the de Leva centres.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva.")], clinical: false },
  presentation: { defaultPose: "stooped-standing", defaultClip: "cane-gait", viewpoints: ["elevation"] },
  exports: STD_EXPORTS,
};
