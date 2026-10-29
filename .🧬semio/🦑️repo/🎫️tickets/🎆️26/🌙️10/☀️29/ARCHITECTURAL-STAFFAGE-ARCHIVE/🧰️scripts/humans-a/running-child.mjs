// [DEBUG] design source: running-child (7-year-old, 1.15 m, ~5.8 heads, 22 kg).
import { readFileSync, existsSync } from "node:fs";
import { tx, refs, rev, FIXED, deg, rad, r4, sm5, R, xf, xp, xpt, ik2, makeGait, sh, cap, ell, line, mass, STD_EXPORTS, ENV, mat, region } from "./common.mjs";
import C, { PAT, FINGERS } from "./walking-commuter.mjs";
const DL = "de-leva-1996", JE = "jensen-1986", ISO = "iso-9920-2007", GU = "gupta-2010";
const ADV = existsSync(new URL("./advance-running-child.json", import.meta.url)) ? JSON.parse(readFileSync(new URL("./advance-running-child.json", import.meta.url), "utf8")) : {};
const kL = 0.545, kT = 0.7, kN = 0.8, kH = 0.85, kA = 0.56, T = 0.52, S = 1.0;
const P = { k: kL, S, T, L1: 0.435 * kL, L2: 0.425 * kL, XH: 0.02, hipBase: 0.935 * kL - 0.03, hipAmp: 0.05, hipMinAt: 0.42, flatTo: 0.07, heelOff: 0.15, to: 0.34, phi0: 4, phiTO: -52, bump: 0.12, swingPow: 0.55 };
const G = makeGait(P);
const H = [0, 0.935 * kL], PEL = 0.1 * kT, TOR = 0.465 * kT, NECK = 0.1 * kN, HEAD = 0.18 * kH;
const LUMB = [0, H[1] + PEL], C7 = [0, LUMB[1] + TOR], NTOP = [C7[0] + NECK * Math.cos(rad(82)), C7[1] + NECK * Math.sin(rad(82))], SHO = [0, LUMB[1] + 0.42 * kT];
const UB = (d) => xf(d, kT, 0, [0, 0.935], H), UBp = (p) => xpt(p, kT, 0, [0, 0.935], H);
const NK = (d) => xf(d, kN, 0, [0, 1.5], C7), HD = (d) => xf(d, kH, 0, [0.0139, 1.599], NTOP), HDp = (p) => xpt(p, kH, 0, [0.0139, 1.599], NTOP);
const LEG = (d) => xf(d, kL, 0, [0, 0.935], H), SHK = (d) => xf(d, kL, 0, [0, 0.5], [0, 0.5 * kL]), FT = (d) => xf(d, kL, 0, [0, 0.075], [0, 0.075 * kL]), FTp = (p) => xpt(p, kL, 0, [0, 0.075], [0, 0.075 * kL]);
const ARM = (d) => xf(d, kA, 0, [0, 1.455], SHO), ARMp = (p) => xpt(p, kA, 0, [0, 1.455], SHO);
const A1 = 0.33 * kA, A2 = 0.26 * kA, HL = 0.19 * kA, WRIST0 = [SHO[0], SHO[1] - A1 - A2];
const aq = (a) => ({ angleDeg: r4(a) });
const hip0y = H[1];

export const gait = (u) => {
  const c = Math.cos(2 * Math.PI * u), sw = 42 * c;
  return { ...G.legs(u), "arm-l": aq(-sw), "forearm-l": aq(88 - 14 * c), "hand-l": aq(0), "arm-r": aq(sw), "forearm-r": aq(88 + 14 * c), "hand-r": aq(0), torso: aq(-9), neck: aq(5), head: aq(5), pack: aq(0) };
};
const rootY = (u) => r4(G.hipAt(u)[1] - hip0y);
const FEET = ["heel-l", "ball-l", "heel-r", "ball-r"];
const mkPose = (id, en, de, dEn, dDe, bones, contacts, met, breath, root, ground = "auto") => ({ id, title: tx(en, de), description: tx(dEn, dDe), bones, ...(root === undefined ? {} : { root: { y: root } }), ground, contacts, thermal: { activityMet: met, emitters: { breath } }, loads: [] });
//#region jump
const BALLO = [0.17 * kL, -0.075 * kL], XB = 0.115;
const ankleFor = (phi) => { const o = R(phi, BALLO); return [XB - o[0], -o[1]]; };
const jl = (q) => { const [hx, hy, phi, torso, a0, a1] = q; const af = ankleFor(phi), r = ik2([0, hy], [af[0] - hx, af[1]], P.L1, P.L2, phi, 0.012); const l = { thigh: aq(r.thigh), shank: aq(r.shank), foot: aq(r.foot), toe: aq(Math.max(0, -phi)) };
  return { "thigh-l": l.thigh, "thigh-r": l.thigh, "shank-l": l.shank, "shank-r": l.shank, "foot-l": l.foot, "foot-r": l.foot, "toe-l": l.toe, "toe-r": l.toe, torso: aq(torso), neck: aq(-torso * 0.3), head: aq(-torso * 0.3), "arm-l": aq(Math.min(a0, 168)), "arm-r": aq(Math.min(a0 + 4, 170)), "forearm-l": aq(a1), "forearm-r": aq(a1), "hand-l": aq(0), "hand-r": aq(0), pack: aq(0) }; };
const jumpLegs = (hx, hy, phi, torso, arms) => jl([hx, hy, phi, torso, arms[0], arms[1]]);
const air = (th, kn, ft, torso, arms) => ({ "thigh-l": aq(th), "thigh-r": aq(th + 6), "shank-l": aq(-kn), "shank-r": aq(-kn - 4), "foot-l": aq(ft), "foot-r": aq(ft), "toe-l": aq(10), "toe-r": aq(10), torso: aq(torso), neck: aq(2), head: aq(2), "arm-l": aq(arms[0]), "arm-r": aq(arms[0] + 4), "forearm-l": aq(arms[1]), "forearm-r": aq(arms[1]), "hand-l": aq(0), "hand-r": aq(0), pack: aq(0) });
const lerpQ = (a, b, w) => a.map((v, i) => v + (b[i] - v) * w);
const seg = (t0, t1, qa, qb, pose, n) => Array.from({ length: n }, (_, i) => ({ t: r4(t0 + ((t1 - t0) * i) / n), pose, ease: "smooth", bones: jl(lerpQ(qa, qb, sm5(i / n))) }));
const Q = { stand: [0.02, 0.502, 0, -2, 4, 14], crouch: [-0.1, 0.345, 0, -38, -48, 18], pre: [-0.06, 0.41, -8, -30, 20, 20], drive: [-0.015, 0.47, -22, -22, 60, 20], takeoff: [0.03, 0.553, -55, -6, 150, 12], land: [0.02, 0.5, -48, -14, 60, 24], absorb: [-0.09, 0.372, 0, -30, 20, 30], recover: [-0.02, 0.48, 0, -8, 10, 16] };
const JH = 0.28, TF = 2 * Math.sqrt((2 * JH) / 9.80665), T0 = 0.72, T1 = T0 + TF, HT = 0.553;
const standHip = [0.0, 0.5];
const jumpPoses = { stand: jl(Q.stand), crouch: jl(Q.crouch), drive: jl(Q.drive), takeoff: jl(Q.takeoff), up: air(24, 40, -34, -4, [165, 8]), apex: air(36, 60, -38, -2, [140, 14]), down: air(26, 44, -26, -6, [100, 20]), land: jl(Q.land), absorb: jl(Q.absorb), recover: jl(Q.recover) };
//#endregion
const rootT = HT - hip0y;
const poses = [
  mkPose("standing-ready", "Standing ready", "Startbereit stehend", "Standing on both feet, knees slightly flexed, arms bent for the start.", "Auf beiden Füßen, Knie leicht gebeugt, Arme zum Start angewinkelt.", jumpPoses.stand, FEET, 1.4, 0.6),
  mkPose("mid-stride", "Mid-stance of the run", "Standbeinmitte des Laufs", "Left foot carries the whole body over the planted forefoot, right leg folding forward, arms in opposition.", "Der linke Fuß trägt den ganzen Körper über dem aufgesetzten Vorfuß, das rechte Bein klappt nach vorn, Arme gegenläufig.", gait(0.17), ["heel-l", "ball-l"], 6, 1),
  mkPose("airborne", "Airborne at the apex", "Flugphase am Scheitelpunkt", "Neither foot touches the ground: the body is in free flight between two strides.", "Kein Fuß berührt den Boden: Der Körper fliegt frei zwischen zwei Schritten.", gait(0.42), [], 6, 1, rootY(0.42)),
  mkPose("toe-off", "Toe-off, last instant of contact", "Abdruck, letzter Bodenkontakt", "The left forefoot leaves the ground, hip and knee fully extended.", "Der linke Vorfuß verlässt den Boden, Hüfte und Knie sind gestreckt.", gait(0.34), [], 6, 1, rootY(0.34)),
  mkPose("landing", "Landing on the forefoot", "Landung auf dem Vorfuß", "The right foot lands ahead of the body, knee soft, the arms swinging through.", "Der rechte Fuß landet vor dem Körper, Knie weich, die Arme schwingen durch.", gait(0.5), ["heel-r"], 6, 1, rootY(0.5)),
  mkPose("crouch", "Crouch before the jump", "Hocke vor dem Sprung", "Both feet flat, hips sunk behind the heels, trunk inclined, arms swung back.", "Beide Füße flach, Hüfte hinter die Fersen abgesenkt, Rumpf geneigt, Arme nach hinten geschwungen.", jumpPoses.crouch, FEET, 3, 1),
  mkPose("jump-takeoff", "Take-off", "Absprung", "Legs fully extended on the balls of the feet, arms thrown up.", "Beine voll gestreckt auf den Fußballen, Arme nach oben geworfen.", jumpPoses.takeoff, ["ball-l", "ball-r"], 7, 1, r4(rootT)),
  mkPose("jump-apex", "Top of the jump", "Scheitel des Sprungs", "Highest point of the 0.28 m jump with the knees tucked.", "Höchster Punkt des 0,28-m-Sprungs mit angezogenen Knien.", jumpPoses.apex, [], 4, 0.8, r4(rootT + JH)),
];
const FLP = [[0.36, "flight-1", "Flight, rising"], [0.38, "flight-2", "Flight, rising"], [0.4, "flight-3", "Flight, near the apex"]].map(([u, id, en]) => mkPose(id, en, en.replace("Flight, rising", "Flugphase, steigend").replace("Flight, near the apex", "Flugphase, nahe dem Scheitel"), "Neither foot touches the ground; the hips follow the smooth flight arc.", "Kein Fuß berührt den Boden; die Hüfte folgt dem glatten Flugbogen.", gait(u), [], 6, 1, rootY(u)));
const half = [...Array.from({ length: 17 }, (_, i) => r4(i * 0.02)), ...Array.from({ length: 11 }, (_, i) => r4(0.34 + i * 0.01)), 0.46, 0.48];
const runKeysU = [...half, ...half.map((u) => r4(u + 0.5))];
const FLN = [[0.34, "toe-off"], [0.36, "flight-1"], [0.38, "flight-2"], [0.4, "flight-3"], [0.42, "airborne"]];
const poseForU = (u) => { const m = r4(u % 0.5); if (u === 0 || u === 0.5 || m === 0.5) return "landing"; if (m < 0.34) return "mid-stride"; const f = 0.42 - Math.abs(m - 0.42); return FLN.reduce((best, c) => (Math.abs(c[0] - f) < Math.abs(best[0] - f) ? c : best))[1]; };
const runKeys = runKeysU.map((u) => ({ t: r4(u * T), pose: poseForU(u), ease: "smooth", bones: gait(u) }));
const sec = (f) => r4(f * T), ev = (t, kind, en, de, target) => ({ t, kind, title: tx(en, de), ...(target ? { target } : {}) });
const jp = (t, pose, bones) => ({ t: r4(t), pose, ease: "smooth", bones });
const flightKey = (f, pose, b) => jp(T0 + f * TF, pose, b);
const jumpKeys = [
  ...seg(0, 0.2, Q.stand, Q.stand, "standing-ready", 1), ...seg(0.2, 0.5, Q.stand, Q.crouch, "crouch", 6), ...seg(0.5, 0.6, Q.crouch, Q.pre, "crouch", 2), ...seg(0.6, 0.66, Q.pre, Q.drive, "crouch", 2), ...seg(0.66, T0, Q.drive, Q.takeoff, "crouch", 2),
  jp(T0, "jump-takeoff", jumpPoses.takeoff), flightKey(0.25, "jump-apex", jumpPoses.up), flightKey(0.5, "jump-apex", jumpPoses.apex), flightKey(0.75, "jump-apex", jumpPoses.down), jp(T1, "jump-takeoff", jumpPoses.land),
  ...seg(T1 + 0.05, T1 + 0.3, Q.land, Q.absorb, "crouch", 5), ...seg(T1 + 0.3, T1 + 0.9, Q.absorb, Q.recover, "standing-ready", 6), jp(T1 + 0.9, "standing-ready", jl(Q.stand)),
];
const JD = r4(T1 + 0.9);
const poseRootFix = poses.find((p) => p.id === "jump-apex");
const bones = [
  { id: "pelvis", parent: null, title: tx("Pelvis", "Becken"), at: H, worldAngleDeg: 90, lengthM: PEL, joint: rev(-20, 20), mass: mass(2.4, 0.04, JE, { source: "literature" }) },
  { id: "torso", parent: "pelvis", title: tx("Trunk with T-shirt", "Rumpf mit T-Shirt"), at: LUMB, worldAngleDeg: 90, lengthM: TOR, joint: rev(-60, 30), mass: mass(7.2, 0.15, JE) },
  { id: "pack", parent: "torso", title: tx("Small backpack", "Kleiner Rucksack"), at: [LUMB[0] - 0.045, LUMB[1] + 0.1], worldAngleDeg: 90, lengthM: 0.2, joint: rev(-8, 8), mass: mass(1.4, 0.1, JE, { source: "authored-estimate", reference: undefined }) },
  { id: "neck", parent: "torso", title: tx("Neck", "Hals"), at: C7, worldAngleDeg: 82, lengthM: NECK, joint: rev(-35, 35), mass: mass(0.3, 0.04, JE, { source: "authored-estimate" }) },
  { id: "head", parent: "neck", title: tx("Head with short hair", "Kopf mit kurzem Haar"), at: NTOP, worldAngleDeg: 90, lengthM: HEAD, joint: rev(-35, 30), mass: mass(2.3, 0.07, JE) },
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `arm-${s}`, parent: "torso", title: tx(`${n[0]} upper arm`, `${n[1]} Oberarm`), at: SHO, worldAngleDeg: -90, lengthM: A1, joint: rev(-80, 170), mass: mass(0.58, 0.58 * A1, JE) },
    { id: `forearm-${s}`, parent: `arm-${s}`, title: tx(`${n[0]} forearm`, `${n[1]} Unterarm`), at: [SHO[0], SHO[1] - A1], worldAngleDeg: -90, lengthM: A2, joint: rev(0, 150), mass: mass(0.34, 0.46 * A2, JE) },
    { id: `hand-${s}`, parent: `forearm-${s}`, title: tx(`${n[0]} hand`, `${n[1]} Hand`), at: WRIST0, worldAngleDeg: -90, lengthM: HL, joint: rev(-60, 60), mass: mass(0.14, 0.5 * HL, JE) },
    { id: `thigh-${s}`, parent: "pelvis", title: tx(`${n[0]} thigh`, `${n[1]} Oberschenkel`), at: H, worldAngleDeg: -90, lengthM: P.L1, joint: rev(-40, 120), mass: mass(2.6, 0.41 * P.L1, JE) },
    { id: `shank-${s}`, parent: `thigh-${s}`, title: tx(`${n[0]} shank`, `${n[1]} Unterschenkel`), at: [0, 0.5 * kL], worldAngleDeg: -90, lengthM: P.L2, joint: rev(-150, 0), mass: mass(0.95, 0.446 * P.L2, JE) },
    { id: `foot-${s}`, parent: `shank-${s}`, title: tx(`${n[0]} foot and sneaker`, `${n[1]} Fuß mit Turnschuh`), at: [0, 0.075 * kL], worldAngleDeg: -23.8, lengthM: 0.1858 * kL, joint: rev(-65, 50), mass: mass(0.46, FTp([0.071, 0.04]), JE) },
    { id: `toe-${s}`, parent: `foot-${s}`, title: tx(`${n[0]} forefoot`, `${n[1]} Vorfuß`), at: FTp([0.17, 0]), worldAngleDeg: 0, lengthM: 0.065 * kL, joint: rev(0, 70), mass: mass(0.04, FTp([0.2, 0.03]), JE, { source: "authored-estimate" }) },
  ]; }),
];
const anchors = [
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `heel-${s}`, bone: `foot-${s}`, at: FTp([-0.045, 0]), kind: "contact", title: tx(`${n[0]} heel`, `${n[1]} Ferse`), support: "contact", normalDeg: 90, footprintLengthM: 0.05, footprintWidthM: 0.045, frictionCoefficient: 1 },
    { id: `ball-${s}`, bone: `foot-${s}`, at: FTp([0.17, 0]), kind: "contact", title: tx(`${n[0]} ball of foot`, `${n[1]} Fußballen`), support: "contact", normalDeg: 90, footprintLengthM: 0.07, footprintWidthM: 0.06, frictionCoefficient: 1 }]; }),
  { id: "mouth", bone: "head", at: HDp([0.094, 1.594]), kind: "emitter", title: tx("Mouth", "Mund") },
];
const TSHIRT = "M -0.03 1.50 C -0.085 1.49 -0.118 1.44 -0.12 1.38 C -0.122 1.30 -0.112 1.20 -0.118 1.10 C -0.122 1.04 -0.122 0.98 -0.12 0.95 L 0.124 0.95 C 0.126 1.00 0.126 1.05 0.128 1.10 C 0.132 1.2 0.142 1.26 0.146 1.31 C 0.138 1.39 0.108 1.45 0.062 1.482 C 0.04 1.495 0.01 1.505 -0.03 1.5 Z";
const SHORTS = "M -0.095 0.95 C -0.100 0.90 -0.098 0.82 -0.094 0.74 L 0.098 0.74 C 0.098 0.82 0.098 0.90 0.092 0.96 Z";
const SLEEVE_S = "M -0.058 1.462 C -0.060 1.42 -0.057 1.35 -0.054 1.30 L 0.060 1.30 C 0.062 1.35 0.066 1.42 0.062 1.452 C 0.040 1.488 -0.030 1.488 -0.058 1.462 Z";
const shapes = [
  ...["l", "r"].flatMap((s) => { const far = s === "l"; const tn = far ? -0.08 : 0; const z0 = far ? 0 : 2; return [
    sh(`arm-skin-${s}`, `arm-${s}`, "body", 0, far ? 0.5 : 6, 0.82 + tn, "limb-skin", "skin", ARM(PAT.SLEEVE)),
    sh(`sleeve-${s}`, `arm-${s}`, "body", 0, far ? 0.52 : 6.05, 0.7 + tn, "tshirt", "cotton-jersey", ARM(SLEEVE_S)),
    sh(`forearm-${s}`, `forearm-${s}`, "body", 0, far ? 0.5 : 6, 0.82 + tn, "limb-skin", "skin", ARM(PAT.FOREARM)),
    sh(`hand-${s}`, `hand-${s}`, "body", 0, far ? 0.6 : 6.2, 0.84 + tn, "limb-skin", "skin", ARM(PAT.HAND)),
    sh(`thumb-${s}`, `hand-${s}`, "detail", 1, far ? 0.62 : 6.25, 0.82 + tn, "limb-skin", "skin", ARM(PAT.THUMB)),
    sh(`thigh-skin-${s}`, `thigh-${s}`, "body", 0, z0 + 1, 0.84 + tn, "limb-skin", "skin", LEG(PAT.TROUSER)),
    sh(`shorts-${s}`, `thigh-${s}`, "body", 0, z0 + 1.01, 0.34 + tn, "shorts", "cotton-jersey", LEG("M -0.097 0.95 C -0.102 0.90 -0.098 0.82 -0.090 0.73 L 0.092 0.73 C 0.098 0.82 0.100 0.90 0.090 0.96 Z")),
    sh(`shank-${s}`, `shank-${s}`, "body", 0, z0 + 1.02, 0.84 + tn, "limb-skin", "skin", SHK(PAT.SHANK)),
    sh(`sock-${s}`, `shank-${s}`, "detail", 1, z0 + 1.03, 0.95 + tn, "socks", "cotton-jersey", SHK("M -0.046 0.19 C -0.050 0.15 -0.046 0.12 -0.043 0.085 L 0.041 0.085 C 0.045 0.12 0.049 0.17 0.050 0.20 C 0.02 0.205 -0.02 0.205 -0.046 0.19 Z")),
    sh(`shoe-rear-${s}`, `foot-${s}`, "body", 0, z0 + 1.1, 0.9 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_REAR)),
    sh(`shoe-toe-${s}`, `toe-${s}`, "body", 0, z0 + 1.1, 0.9 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_FRONT)),
    line(`shoe-sole-${s}`, `foot-${s}`, "detail", 1, z0 + 1.15, 0.1, "shoes", "canvas-sneaker", xp([[-0.05, 0.022], [0.17, 0.022]], kL, 0, [0, 0.075], [0, 0.075 * kL])),
    line(`shoe-stripe-${s}`, `foot-${s}`, "detail", 2, z0 + 1.16, 0.3, "shoes", "canvas-sneaker", xp([[0.0, 0.06], [0.1, 0.075]], kL, 0, [0, 0.075], [0, 0.075 * kL])),
  ]; }),
  sh("pelvis-shorts", "pelvis", "body", 0, 3, 0.34, "shorts", "cotton-jersey", UB("M -0.115 1.04 C -0.138 0.99 -0.135 0.93 -0.110 0.88 L 0.100 0.88 C 0.130 0.93 0.132 0.99 0.118 1.04 Z")),
  sh("pack-body", "pack", "body", 0, 3.5, 0.3, "backpack", "nylon-pack", UB("M -0.13 1.40 C -0.2 1.38 -0.215 1.22 -0.19 1.12 C -0.17 1.08 -0.13 1.08 -0.115 1.1 L -0.115 1.38 Z")),
  line("pack-strap", "pack", "detail", 2, 4.4, 0.2, "backpack", "nylon-pack", xp([[-0.115, 1.38], [0.0, 1.46], [0.07, 1.36], [0.02, 1.15]], kT, 0, [0, 0.935], H)),
  line("pack-pocket", "pack", "detail", 2, 3.6, 0.5, "backpack", "nylon-pack", xp([[-0.2, 1.2], [-0.13, 1.2]], kT, 0, [0, 0.935], H)),
  sh("tshirt", "torso", "body", 0, 4, 0.7, "tshirt", "cotton-jersey", UB(TSHIRT)),
  sh("tshirt-print", "torso", "detail", 2, 4.1, 0.25, "tshirt", "cotton-jersey", UB("M 0.02 1.30 L 0.115 1.30 L 0.115 1.20 L 0.02 1.20 Z")),
  line("tshirt-hem", "torso", "detail", 2, 4.1, 0.3, "tshirt", "cotton-jersey", xp([[-0.118, 0.975], [0.124, 0.975]], kT, 0, [0, 0.935], H)),
  sh("neck", "neck", "body", 0, 3.8, 0.84, "face-skin", "skin", NK(C.shapes.find((q) => q.id === "neck").d)),
  sh("collar", "neck", "body", 1, 4.5, 0.62, "tshirt", "cotton-jersey", NK("M -0.07 1.50 C -0.075 1.485 -0.06 1.47 -0.03 1.468 L 0.06 1.468 C 0.085 1.475 0.09 1.50 0.08 1.52 C 0.05 1.51 -0.03 1.51 -0.07 1.50 Z")),
  sh("head", "head", "body", 0, 5, 0.87, "face-skin", "skin", HD(C.shapes.find((q) => q.id === "head").d)),
  sh("hair", "head", "body", 1, 5.5, 0.3, "hair", "hair", HD("M 0.082 1.735 C 0.07 1.772 -0.02 1.786 -0.06 1.762 C -0.105 1.735 -0.108 1.675 -0.085 1.625 C -0.075 1.62 -0.065 1.622 -0.06 1.63 C -0.062 1.665 -0.04 1.70 -0.005 1.715 C 0.03 1.725 0.065 1.72 0.082 1.735 Z")),
  line("hair-fringe", "head", "texture", 3, 5.6, 0.3, "hair", "hair", xp([[0.03, 1.74], [0.07, 1.715]], kH, 0, [0.0139, 1.599], NTOP)),
  ell("ear", "head", "detail", 2, 5.2, 0.8, "face-skin", "skin", HDp([-0.012, 1.645]), 0.012 * kH, 0.02 * kH),
  ell("eye", "head", "detail", 3, 5.6, 0.05, "face-skin", "skin", HDp([0.075, 1.667]), 0.008 * kH, 0.005 * kH),
  line("mouth-line", "head", "detail", 3, 5.6, 0.3, "face-skin", "skin", xp([[0.083, 1.598], [0.096, 1.598]], kH, 0, [0.0139, 1.599], NTOP)),
];
const clips = [
  {
    id: "sprint-cycle", title: tx("Running with flight phases", "Rennen mit Flugphasen"), description: tx("A child running at about 1.9 m/s with 3.8 steps per second: each foot carries for 0.18 s on the forefoot, then both feet are in the air for 0.08 s before the other foot lands. The hips rise in the flight phases; the arms swing in opposition with the elbows at about 90 degrees.", "Ein laufendes Kind mit etwa 1,9 m/s und 3,8 Schritten pro Sekunde: Jeder Fuß trägt 0,18 s auf dem Vorfuß, danach sind beide Füße 0,08 s in der Luft, bevor der andere Fuß landet. Die Hüfte steigt in den Flugphasen; die Arme schwingen gegenläufig mit etwa 90 Grad Ellbogenwinkel."),
    durationS: T, fps: 30, loop: true, rootMotion: "plant", advanceM: ADV["sprint-cycle"] ?? S, keys: runKeys,
    contacts: [{ anchor: "heel-l", fromS: 0, toS: sec(P.heelOff) }, { anchor: "ball-l", fromS: sec(P.flatTo), toS: sec(P.to) }, { anchor: "heel-r", fromS: sec(0.5), toS: sec(0.5 + P.heelOff) }, { anchor: "ball-r", fromS: sec(0.5 + P.flatTo), toS: sec(0.5 + P.to) }],
    phases: [{ id: "left-stance", fromS: 0, toS: sec(P.to), title: tx("Left stance", "Linke Standphase") }, { id: "flight-1", fromS: sec(P.to), toS: sec(0.5), title: tx("Flight, no foot on the ground", "Flugphase, kein Fuß am Boden") }, { id: "right-stance", fromS: sec(0.5), toS: sec(0.5 + P.to), title: tx("Right stance", "Rechte Standphase") }, { id: "flight-2", fromS: sec(0.5 + P.to), toS: T, title: tx("Flight, no foot on the ground", "Flugphase, kein Fuß am Boden") }],
    events: [ev(0, "contact-on", "Left foot lands", "Linker Fuß landet", "heel-l"), ev(sec(P.to), "contact-off", "Left toe-off", "Linker Abdruck", "ball-l"), ev(sec(0.5), "contact-on", "Right foot lands", "Rechter Fuß landet", "heel-r"), ev(sec(0.5 + P.to), "contact-off", "Right toe-off", "Rechter Abdruck", "ball-r"), ev(0.1, "inhale", "Inhalation every second step", "Einatmen bei jedem zweiten Schritt"), ev(0.3, "exhale", "Exhalation", "Ausatmen")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 0.26, v: 0.7 }, { t: T, v: 1 }] }],
  },
  {
    id: "standing-jump", title: tx("Jump from a crouch", "Sprung aus der Hocke"), description: tx("A vertical jump of 0.28 m: crouch with the arms swung back, extension through the balls of the feet with the arms thrown up, 0.48 s of free flight, landing on the forefoot and absorbing the impact in a crouch before standing up.", "Ein senkrechter Sprung von 0,28 m: Hocke mit zurückgeschwungenen Armen, Streckung über die Fußballen mit hochgeworfenen Armen, 0,48 s freier Flug, Landung auf dem Vorfuß und Abfedern in der Hocke vor dem Aufrichten."),
    durationS: JD, fps: 30, loop: false, rootMotion: "plant", keys: jumpKeys,
    contacts: [{ anchor: "heel-l", fromS: 0, toS: 0.6 }, { anchor: "heel-r", fromS: 0, toS: 0.6 }, { anchor: "ball-l", fromS: 0, toS: r4(T0) }, { anchor: "ball-r", fromS: 0, toS: r4(T0) }, { anchor: "ball-l", fromS: r4(T1), toS: JD }, { anchor: "ball-r", fromS: r4(T1), toS: JD }, { anchor: "heel-l", fromS: r4(T1 + 0.12), toS: JD }, { anchor: "heel-r", fromS: r4(T1 + 0.12), toS: JD }],
    phases: [{ id: "crouch", fromS: 0, toS: 0.45, title: tx("Crouching", "In die Hocke gehen") }, { id: "drive", fromS: 0.45, toS: r4(T0), title: tx("Driving upwards", "Abdruck nach oben") }, { id: "flight", fromS: r4(T0), toS: r4(T1), title: tx("Flight", "Flugphase") }, { id: "landing", fromS: r4(T1), toS: JD, title: tx("Landing and standing up", "Landen und Aufrichten") }],
    events: [ev(0.6, "contact-off", "Heels lift", "Fersen heben ab", "heel-l"), ev(r4(T0), "contact-off", "Take-off", "Absprung", "ball-l"), ev(r4(T1), "contact-on", "Touch-down on the forefoot", "Aufsetzen auf dem Vorfuß", "ball-l"), ev(0.3, "inhale", "Breath in", "Einatmen"), ev(r4(T1 + 0.3), "exhale", "Breath out on landing", "Ausatmen bei der Landung")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.4 }, { t: r4(T0), v: 8 }, { t: r4(T1), v: 6 }, { t: JD, v: 2 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.6 }, { t: r4(T1), v: 1 }, { t: JD, v: 0.8 }] }],
  },
];
export default {
  id: "running-child",
  title: tx("Running Child", "Rennendes Kind"),
  summary: tx("A seven-year-old in a T-shirt, shorts and a small backpack sprinting with real flight phases in which neither foot touches the ground, and jumping from a crouch; child proportions with about 5.8 heads to the body.", "Ein Siebenjähriger in T-Shirt, Shorts und kleinem Rucksack sprintet mit echten Flugphasen, in denen kein Fuß den Boden berührt, und springt aus der Hocke; kindliche Proportionen mit etwa 5,8 Kopfhöhen."),
  tags: ["human", "child", "runner", "flight-phase", "jump", "backpack"],
  provenance: {
    author: "Semio staffage content worker humans-a", method: "handcrafted", created: "2026-10-29",
    references: [...refs(DL, "drillis-contini-1966", "dubois-1916", ISO, "iso-8996-2021", "iso-7730-2005", "ashrae-2021", GU), { id: JE, citation: "Jensen, R. K. (1986). Body segment mass, radius and radius of gyration proportions of children. Journal of Biomechanics 19(5), 359-368." }, { id: "schache-2011", citation: "Schache, A. G. et al. (2011). Mechanics of the human hamstring muscles during sprinting. Medicine and Science in Sports and Exercise 44(4), 647-658; contact and flight times of sprint running." }],
    notes: tx("A 22 kg child of 1.15 m with separate scale factors for legs, trunk, neck, head and arms so that the head is about a sixth of the height. Segment mass fractions follow the child proportions of Jensen (1986) in rounded form and are therefore tagged as literature-based estimates. Run key frames come from two-link inverse kinematics of smooth foot paths with 34 % stance, the flight arcs of the hips are a smooth sinusoid, not an exact parabola.", "Ein 22 kg schweres Kind von 1,15 m mit getrennten Skalierungen für Beine, Rumpf, Hals, Kopf und Arme, sodass der Kopf etwa ein Sechstel der Körpergröße misst. Die Segmentmassenanteile folgen gerundet den Kinderproportionen von Jensen (1986) und sind daher als literaturbasierte Schätzung gekennzeichnet. Die Laufschlüsselbilder stammen aus zweigliedriger inverser Kinematik glatter Fußbahnen mit 34 % Standphase; die Flugbögen der Hüfte sind eine glatte Sinuskurve, keine exakte Parabel."),
  },
  dimensions: { heightM: 1.15, widthM: 0.27, depthM: 0.28, massKg: 22.1 },
  materials: [
    mat("skin", "Child skin", "Kinderhaut", 0.98, 0.6, "literature", "ashrae-2021"), mat("hair", "Hair", "Haar", 0.98, 0.85, "authored-estimate"),
    mat("cotton-jersey", "Cotton jersey", "Baumwolljersey", 0.95, 0.5, "literature", "ashrae-2021"), mat("canvas-sneaker", "Canvas sneaker", "Stoffturnschuh", 0.9, 0.5, "authored-estimate"), mat("nylon-pack", "Nylon fabric", "Nylongewebe", 0.9, 0.6, "authored-estimate"),
  ],
  bones, anchors, shapes, poses: [...poses, ...FLP], clips,
  thermal: {
    environment: ENV(18, 20, 0.55, 0.5, 400), baseMet: 1.4,
    regions: [
      region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", "core-resistance", 0.075, 0.15, 0.09, { coreTempC: 37, insulationM2KW: 0.035 }, "dubois-1916"),
      region("hair", "Hair", "Haar", "hair", "hair", "core-resistance", 0.04, 0.14, 0.02, { coreTempC: 37, insulationM2KW: 0.06 }, "dubois-1916", "authored-estimate"),
      region("limb-skin", "Bare arms and legs", "Unbedeckte Arme und Beine", "skin", "skin", "core-resistance", 0.4, 0.07, 0.35, { coreTempC: 36.4, insulationM2KW: 0.04, wettedness: 0.2 }, "dubois-1916"),
      region("tshirt", "T-shirt", "T-Shirt", "clothing", "cotton-jersey", "core-resistance", 0.34, 0.2, 0.33, { coreTempC: 37, insulationM2KW: 0.08 }, ISO),
      region("shorts", "Shorts", "Shorts", "clothing", "cotton-jersey", "core-resistance", 0.14, 0.12, 0.11, { coreTempC: 37, insulationM2KW: 0.07 }, ISO),
      region("socks", "Socks", "Socken", "clothing", "cotton-jersey", "core-resistance", 0.05, 0.08, 0.04, { coreTempC: 35, insulationM2KW: 0.07 }, ISO),
      region("shoes", "Sneakers", "Turnschuhe", "surface", "canvas-sneaker", "core-resistance", 0.07, 0.08, 0.06, { coreTempC: 34, insulationM2KW: 0.08 }, ISO, "authored-estimate"),
      region("backpack", "Backpack", "Rucksack", "surface", "nylon-pack", "passive", 0.12, 0.2, undefined, {}, ISO, "authored-estimate"),
    ],
    emitters: [{ id: "breath", kind: "breath", title: tx("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: -10, carries: "vapour", temperatureC: 33, velocityMS: 2.2, diameterM: 0.016, vapourKgPerKg: 0.032, periodS: 1.3, dutyFrom: 0.45, dutyTo: 1, source: "authored-estimate", reference: GU }],
    assumptions: [tx("Body regions sum to the DuBois area of 0.86 m² (22 kg, 115 cm); the backpack is an additional passive surface.", "Die Körperregionen ergeben die DuBois-Fläche von 0,86 m² (22 kg, 115 cm); der Rucksack ist eine zusätzliche passive Fläche."), tx("Running is 6 met in this model (ISO 8996 level for fast running of a child, scaled by body surface); the breathing period of 1.3 s is an authored estimate.", "Rennen ist in diesem Modell 6 met (ISO-8996-Niveau für schnelles Laufen eines Kindes, nach Körperoberfläche skaliert); die Atemperiode von 1,3 s ist eine Schätzung.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: [], assumptions: [tx("Quasi-static: weights and loads only. In the flight phases no support is active and the residual is the full body weight, which is the truth of a body in free flight; the stride accelerations are not evaluated because the seam of the planted root motion would create spurious inertial spikes.", "Quasi-statisch: nur Gewichte und Lasten. In den Flugphasen ist kein Auflager aktiv und das Residuum ist das volle Körpergewicht, wie bei einem Körper im freien Flug; die Schrittbeschleunigungen werden nicht ausgewertet, da die Naht der geplanten Wurzelbewegung künstliche Trägheitsspitzen erzeugen würde."), tx("The backpack is a rigid 1.4 kg bone mass on the trunk.", "Der Rucksack ist eine starre Knochenmasse von 1,4 kg am Rumpf.")], clinical: false },
  presentation: { defaultPose: "standing-ready", defaultClip: "sprint-cycle", viewpoints: ["elevation"] },
  exports: STD_EXPORTS,
};
