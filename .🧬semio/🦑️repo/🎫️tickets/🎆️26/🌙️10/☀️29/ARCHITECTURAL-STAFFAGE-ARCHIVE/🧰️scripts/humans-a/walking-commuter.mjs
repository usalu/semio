// [DEBUG] design source: walking-commuter (rest-world coordinates f = forward, u = up, metres; ground u = 0).
import { readFileSync, existsSync } from "node:fs";
import { tx, refs, rev, FIXED, deg, rad, r4, smooth, sh, cap, ell, line, mass, STD_EXPORTS, ENV, mat, region } from "./common.mjs";
const DL = "de-leva-1996", WI = "winter-2009", ISO = "iso-9920-2007", GU = "gupta-2010";
const ADV = existsSync(new URL("./advance-walking-commuter.json", import.meta.url)) ? JSON.parse(readFileSync(new URL("./advance-walking-commuter.json", import.meta.url), "utf8")) : {};
const L1 = 0.435, L2 = 0.425, S = 1.2, T = 1.2, XH = 0.20, A_H = 0.075, BALL = [0.17, -0.075], HEEL = [-0.045, -0.075];
const sm5 = (w) => { const u = Math.min(1, Math.max(0, w)); return u * u * u * (u * (6 * u - 15) + 10); };
const R = (a, p) => { const c = Math.cos(rad(a)), s = Math.sin(rad(a)); return [p[0] * c - p[1] * s, p[0] * s + p[1] * c]; };
const leftFoot = (tau) => {
  const heel = [XH, 0], ball = [XH + 0.215, 0];
  let phi, ankle;
  if (tau <= 0.12) { phi = 12 * (1 - sm5(tau / 0.12)); const o = R(phi, [0.045, 0.075]); ankle = [heel[0] + o[0], heel[1] + o[1]]; }
  else if (tau <= 0.31) { phi = 0; ankle = [XH + 0.045, 0.075]; }
  else if (tau <= 0.62) { phi = -55 * sm5((tau - 0.31) / 0.31); const o = R(phi, BALL); ankle = [ball[0] - o[0], -o[1]]; }
  else {
    const o0 = R(-55, BALL), a0 = [ball[0] - o0[0], -o0[1]];
    const o1 = R(12, [0.045, 0.075]), a1 = [XH + S + o1[0], o1[1]];
    const w = (tau - 0.62) / 0.38, e = sm5(w);
    phi = -55 * (1 - e) + 12 * e;
    ankle = [a0[0] + (a1[0] - a0[0]) * e, a0[1] + (a1[1] - a0[1]) * e + 0.04 * Math.sin(Math.PI * w) ** 4];
  }
  const toe = tau < 0.62 ? Math.max(0, -phi) : Math.max(0, -phi) * (1 - sm5((tau - 0.62) / 0.38));
  return { ankle, phi, toe };
};
const footAt = (u, right) => {
  let tau = right ? u - 0.5 : u, shift = right ? S / 2 : 0;
  if (tau < 0) { tau += 1; shift -= S; }
  const f = leftFoot(tau);
  return { ankle: [f.ankle[0] + shift, f.ankle[1]], phi: f.phi, toe: f.toe };
};
const ik = (hip, foot) => {
  const dx = foot.ankle[0] - hip[0], dy = foot.ankle[1] - hip[1], d = Math.min(Math.hypot(dx, dy), L1 + L2 - 1e-4); if (Math.hypot(dx, dy) > L1 + L2 - 1e-4) console.error("[DEBUG] clamp", (Math.hypot(dx, dy) - L1 - L2).toFixed(4), foot.ankle.map((v) => v.toFixed(3)).join(","));
  if (Math.hypot(dx, dy) > L1 + L2 + 0.004) throw new Error(`leg out of reach ${Math.hypot(dx, dy)}`);
  const beta = Math.acos((L1 * L1 + d * d - L2 * L2) / (2 * L1 * d));
  const th = Math.atan2(dy, dx) + beta;
  const kx = hip[0] + L1 * Math.cos(th), ky = hip[1] + L1 * Math.sin(th);
  const sk = Math.atan2(foot.ankle[1] - ky, foot.ankle[0] - kx);
  return { thigh: deg(th) + 90, shank: deg(sk - th), foot: foot.phi - deg(sk) - 90, toe: foot.toe };
};
export const gait = (u) => {
  const hip = [S * u, 0.925 - 0.05 * (1 - Math.cos(4 * Math.PI * (u - 0.31))) / 2];
  const l = ik(hip, footAt(u, false)), r = ik(hip, footAt(u, true));
  const c = Math.cos(2 * Math.PI * u), s = Math.sin(2 * Math.PI * u);
  const swing = -26 * c;
  const a = (v) => ({ angleDeg: r4(v) });
  return {
    "thigh-l": a(l.thigh), "shank-l": a(l.shank), "foot-l": a(l.foot), "toe-l": a(l.toe), "thigh-r": a(r.thigh), "shank-r": a(r.shank), "foot-r": a(r.foot), "toe-r": a(r.toe),
    "arm-l": a(swing), "forearm-l": a(14 + 0.5 * (swing + 26)), "hand-l": a(4), "arm-r": a(0.75 * -swing), "forearm-r": a(12 + 0.45 * (-swing + 26)), "hand-r": a(-4),
    torso: a(-4 - 0.8 * Math.abs(s)), neck: a(3), head: a(1), bag: a(4 + 6 * Math.sin(2 * Math.PI * u + 0.5)),
  };
};
const poseAt = (id, en, de, descEn, descDe, u, contacts) => ({ id, title: tx(en, de), description: tx(descEn, descDe), bones: gait(u), ground: "auto", contacts, thermal: { activityMet: 2.8, emitters: { breath: 1 } }, loads: [BAG] });
const BAG = { id: "shoulder-bag", kind: "carried", title: tx("Shoulder bag", "Umhängetasche"), anchor: "bag-cog", fxN: 0, fyN: -34.32, source: "authored-estimate" };
const FEET4 = ["heel-l", "ball-l", "heel-r", "ball-r"];

const bones = [
  { id: "pelvis", parent: null, title: tx("Pelvis", "Becken"), at: [0, 0.935], worldAngleDeg: 90, lengthM: 0.1, joint: rev(-15, 15), mass: mass(8.24, 0.06, DL) },
  { id: "torso", parent: "pelvis", title: tx("Torso with coat", "Rumpf mit Mantel"), at: [0, 1.035], worldAngleDeg: 90, lengthM: 0.465, joint: rev(-40, 25), mass: mass(24.45, 0.215, DL) },
  { id: "neck", parent: "torso", title: tx("Neck", "Hals"), at: [0, 1.5], worldAngleDeg: 82, lengthM: 0.1, joint: rev(-35, 30), mass: mass(0.7, 0.05, DL, { source: "authored-estimate" }) },
  { id: "head", parent: "neck", title: tx("Head", "Kopf"), at: [0.0139, 1.599], worldAngleDeg: 90, lengthM: 0.18, joint: rev(-30, 25), mass: mass(4.3, 0.075, DL) },
  ...["l", "r"].flatMap((s) => {
    const S2 = s === "l" ? ["Left", "Linker", "Linke", "Linkes"] : ["Right", "Rechter", "Rechte", "Rechtes"];
    return [
      { id: `arm-${s}`, parent: "torso", title: tx(`${S2[0]} upper arm`, `${S2[1]} Oberarm`), at: [0, 1.455], worldAngleDeg: -90, lengthM: 0.33, joint: rev(-60, 170), mass: mass(2.05, 0.19, DL) },
      { id: `forearm-${s}`, parent: `arm-${s}`, title: tx(`${S2[0]} forearm`, `${S2[1]} Unterarm`), at: [0, 1.125], worldAngleDeg: -90, lengthM: 0.26, joint: rev(0, 150), mass: mass(1.27, 0.119, DL) },
      { id: `hand-${s}`, parent: `forearm-${s}`, title: tx(`${S2[0]} hand`, `${S2[1]} Hand`), at: [0, 0.865], worldAngleDeg: -90, lengthM: 0.19, joint: rev(-60, 60), mass: mass(0.44, 0.096, WI) },
      { id: `thigh-${s}`, parent: "pelvis", title: tx(`${S2[0]} thigh`, `${S2[1]} Oberschenkel`), at: [0, 0.935], worldAngleDeg: -90, lengthM: L1, joint: rev(-30, 110), mass: mass(10.2, 0.178, DL) },
      { id: `shank-${s}`, parent: `thigh-${s}`, title: tx(`${S2[0]} shank`, `${S2[1]} Unterschenkel`), at: [0, 0.5], worldAngleDeg: -90, lengthM: L2, joint: rev(-150, 0), mass: mass(3.12, 0.1895, DL) },
      { id: `foot-${s}`, parent: `shank-${s}`, title: tx(`${S2[0]} foot and shoe`, `${S2[1]} Fuß mit Schuh`), at: [0, 0.075], worldAngleDeg: -23.8, lengthM: 0.1858, joint: rev(-60, 30), mass: mass(1.32, [0.071, 0.04], DL) },
      { id: `toe-${s}`, parent: `foot-${s}`, title: tx(`${S2[0]} forefoot`, `${S2[1]} Vorfuß`), at: [0.17, 0], worldAngleDeg: 0, lengthM: 0.065, joint: rev(0, 65), mass: mass(0.12, 0.03, DL, { source: "authored-estimate" }) },
    ];
  }),
  { id: "bag", parent: "torso", title: tx("Shoulder bag on strap", "Umhängetasche am Riemen"), at: [0, 1.455], worldAngleDeg: -90, lengthM: 0.55, joint: rev(-40, 40) },
];
const anchors = [
  ...["l", "r"].flatMap((s) => {
    const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"];
    return [
      { id: `heel-${s}`, bone: `foot-${s}`, at: [-0.045, 0], kind: "contact", title: tx(`${n[0]} heel`, `${n[1]} Ferse`), support: "contact", normalDeg: 90, footprintLengthM: 0.07, footprintWidthM: 0.065, frictionCoefficient: 0.9 },
      { id: `ball-${s}`, bone: `foot-${s}`, at: [0.17, 0], kind: "contact", title: tx(`${n[0]} ball of foot`, `${n[1]} Fußballen`), support: "contact", normalDeg: 90, footprintLengthM: 0.1, footprintWidthM: 0.1, frictionCoefficient: 0.9 },
    ];
  }),
  { id: "mouth", bone: "head", at: [0.094, 1.594], kind: "emitter", title: tx("Mouth and nose", "Mund und Nase") },
  { id: "bag-cog", bone: "bag", at: [0, 0.92], kind: "load", title: tx("Bag centre of gravity", "Schwerpunkt der Tasche") },
  { id: "strap-grip", bone: "hand-r", at: [0.01, 0.78], kind: "attachment", title: tx("Grip on the bag strap", "Griff am Taschenriemen") },
];

const TROUSER = "M -0.095 0.95 C -0.105 0.85 -0.088 0.62 -0.062 0.52 C -0.060 0.495 -0.035 0.485 0.0 0.485 C 0.040 0.485 0.066 0.495 0.066 0.52 C 0.085 0.62 0.100 0.82 0.088 0.96 Z";
const SHANK = "M -0.062 0.50 C -0.064 0.532 -0.035 0.545 0.0 0.545 C 0.040 0.545 0.064 0.532 0.064 0.50 C 0.062 0.46 0.056 0.40 0.052 0.30 C 0.050 0.22 0.045 0.14 0.041 0.085 L -0.043 0.085 C -0.047 0.12 -0.052 0.19 -0.060 0.25 C -0.078 0.34 -0.076 0.42 -0.062 0.50 Z";
const SHOE_REAR = "M -0.052 0.065 C -0.056 0.04 -0.055 0.015 -0.050 0.005 L -0.040 0 L 0.171 0 L 0.171 0.068 C 0.12 0.082 0.08 0.097 0.05 0.105 L 0.035 0.110 C 0.015 0.100 -0.020 0.100 -0.035 0.095 C -0.048 0.090 -0.052 0.080 -0.052 0.065 Z";
const SHOE_FRONT = "M 0.169 0 L 0.215 0 C 0.238 0 0.246 0.012 0.240 0.026 C 0.232 0.045 0.200 0.062 0.169 0.068 Z";
const SLEEVE = "M -0.058 1.462 C -0.060 1.40 -0.055 1.25 -0.046 1.13 C -0.046 1.10 -0.025 1.092 0.0 1.092 C 0.025 1.092 0.050 1.10 0.050 1.13 C 0.056 1.25 0.064 1.38 0.062 1.452 C 0.040 1.488 -0.030 1.488 -0.058 1.462 Z";
const FOREARM = "M -0.047 1.135 C -0.048 1.155 -0.026 1.165 0.0 1.165 C 0.027 1.165 0.050 1.155 0.050 1.135 C 0.050 1.05 0.045 0.95 0.040 0.875 L -0.039 0.875 C -0.040 0.95 -0.046 1.05 -0.047 1.135 Z";
const HAND = "M -0.031 0.88 C -0.037 0.83 -0.036 0.78 -0.032 0.73 C -0.030 0.70 -0.024 0.682 -0.014 0.677 C -0.004 0.672 0.006 0.672 0.014 0.677 C 0.024 0.684 0.030 0.70 0.030 0.73 C 0.032 0.78 0.036 0.84 0.040 0.88 Z";
const THUMB = "M 0.034 0.865 C 0.052 0.84 0.058 0.80 0.050 0.775 C 0.044 0.77 0.038 0.79 0.034 0.82 Z";
export const FINGERS = [[[-0.016, 0.69], [-0.012, 0.79]], [[0.0, 0.675], [0.0, 0.79]], [[0.015, 0.685], [0.013, 0.79]]];
export const PAT = { TROUSER, SHANK, SHOE_REAR, SHOE_FRONT, SLEEVE, FOREARM, HAND, THUMB };
const limbs = ["l", "r"].flatMap((s) => {
  const far = s === "l"; const tn = far ? -0.07 : 0; const z0 = far ? 0 : 2;
  return [
    sh(`sleeve-${s}`, `arm-${s}`, "body", 0, far ? 0.5 : 6, 0.3 + tn, "coat", "wool-coat", SLEEVE),
    sh(`sleeve-cuff-${s}`, `forearm-${s}`, "detail", 2, far ? 0.52 : 6.05, 0.2 + tn, "coat", "wool-coat", "M -0.043 0.895 L 0.043 0.895 L 0.043 0.875 L -0.042 0.875 Z"),
    sh(`forearm-${s}`, `forearm-${s}`, "body", 0, far ? 0.5 : 6, 0.3 + tn, "coat", "wool-coat", FOREARM),
    sh(`hand-${s}`, `hand-${s}`, "body", 0, far ? 0.6 : 6.2, 0.82 + tn / 2, "hand-skin", "skin", HAND),
    ...FINGERS.map((f, i) => line(`fingers-${s}-${i}`, `hand-${s}`, "detail", 2, far ? 0.63 : 6.26, 0.5, "hand-skin", "skin", f)),
    sh(`thumb-${s}`, `hand-${s}`, "detail", 1, far ? 0.62 : 6.25, 0.8 + tn / 2, "hand-skin", "skin", THUMB),
    sh(`thigh-${s}`, `thigh-${s}`, "body", 0, z0 + 1, 0.3 + tn, "trousers", "wool-trouser", TROUSER),
    sh(`shank-${s}`, `shank-${s}`, "body", 0, z0 + 1.02, 0.3 + tn, "trousers", "wool-trouser", SHANK),
    line(`trouser-crease-${s}`, `shank-${s}`, "texture", 3, z0 + 1.02, 0.15, "trousers", "wool-trouser", [[0.0, 0.48], [0.004, 0.2], [0.0, 0.1]]),
    sh(`shoe-rear-${s}`, `foot-${s}`, "body", 0, z0 + 1.1, 0.12 + tn / 2, "shoes", "leather-shoe", SHOE_REAR),
    sh(`shoe-toe-${s}`, `toe-${s}`, "body", 0, z0 + 1.1, 0.12 + tn / 2, "shoes", "leather-shoe", SHOE_FRONT),
    line(`shoe-sole-${s}`, `foot-${s}`, "detail", 1, z0 + 1.15, 0.05, "shoes", "leather-shoe", [[-0.050, 0.02], [0.17, 0.02]]),
    line(`shoe-laces-${s}`, `foot-${s}`, "texture", 3, z0 + 1.15, 0.2, "shoes", "leather-shoe", [[0.05, 0.103], [0.085, 0.088], [0.12, 0.078]]),
    line(`shoe-cuff-${s}`, `shank-${s}`, "detail", 2, z0 + 1.05, 0.15, "trousers", "wool-trouser", [[-0.043, 0.098], [0.043, 0.098]]),
  ];
});

const shapes = [
  ...limbs,
  sh("pelvis-trousers", "pelvis", "body", 0, 3, 0.3, "trousers", "wool-trouser", "M -0.115 1.04 C -0.138 0.99 -0.135 0.93 -0.110 0.86 L 0.100 0.86 C 0.130 0.93 0.132 0.99 0.118 1.04 Z"),
  sh("neck", "neck", "body", 0, 3.8, 0.82, "face-skin", "skin", "M -0.045 1.60 C -0.050 1.56 -0.055 1.53 -0.060 1.495 L 0.050 1.490 C 0.048 1.52 0.040 1.55 0.032 1.557 Z"),
  sh("scarf", "neck", "body", 1, 4.5, 0.55, "scarf", "wool-scarf", "M -0.075 1.535 C -0.088 1.505 -0.078 1.478 -0.040 1.468 L 0.062 1.468 C 0.090 1.478 0.098 1.51 0.086 1.540 C 0.070 1.556 0.02 1.552 -0.02 1.55 C -0.05 1.55 -0.07 1.545 -0.075 1.535 Z"),
  line("scarf-fold-1", "neck", "texture", 2, 4.6, 0.3, "scarf", "wool-scarf", [[-0.07, 1.515], [-0.02, 1.505], [0.05, 1.51], [0.085, 1.52]]),
  line("scarf-fold-2", "neck", "texture", 3, 4.6, 0.3, "scarf", "wool-scarf", [[-0.06, 1.49], [0.0, 1.485], [0.06, 1.488]]),
  sh("head", "head", "body", 0, 5, 0.85, "face-skin", "skin", "M -0.060 1.610 C -0.095 1.64 -0.098 1.71 -0.055 1.755 C -0.02 1.774 0.045 1.772 0.075 1.738 C 0.088 1.722 0.093 1.700 0.092 1.682 C 0.094 1.672 0.098 1.664 0.108 1.652 C 0.122 1.636 0.121 1.628 0.112 1.622 C 0.100 1.617 0.098 1.612 0.098 1.607 C 0.100 1.600 0.098 1.595 0.094 1.592 C 0.096 1.580 0.092 1.568 0.084 1.560 C 0.070 1.547 0.045 1.548 0.030 1.556 C 0.012 1.570 -0.020 1.585 -0.045 1.590 C -0.055 1.595 -0.058 1.602 -0.060 1.610 Z"),
  sh("hair", "head", "body", 1, 5.5, 0.12, "hair", "hair", "M 0.076 1.738 C 0.062 1.772 -0.020 1.784 -0.058 1.760 C -0.100 1.735 -0.108 1.670 -0.082 1.620 C -0.072 1.612 -0.062 1.610 -0.058 1.612 C -0.062 1.650 -0.050 1.700 -0.010 1.722 C 0.030 1.738 0.060 1.736 0.076 1.738 Z"),
  ell("ear", "head", "detail", 2, 5.2, 0.8, "face-skin", "skin", [-0.012, 1.645], 0.012, 0.02),
  ell("eye", "head", "detail", 3, 5.6, 0.05, "face-skin", "skin", [0.075, 1.667], 0.007, 0.004),
  line("brow", "head", "detail", 3, 5.6, 0.1, "hair", "hair", [[0.063, 1.680], [0.090, 1.683]]),
  line("mouth-line", "head", "detail", 3, 5.6, 0.3, "face-skin", "skin", [[0.083, 1.598], [0.096, 1.598]]),
  sh("coat", "torso", "body", 0, 4, 0.3, "coat", "wool-coat", "M -0.030 1.500 C -0.085 1.490 -0.118 1.440 -0.120 1.380 C -0.122 1.300 -0.108 1.200 -0.118 1.100 C -0.135 1.020 -0.145 0.860 -0.152 0.700 L -0.150 0.640 C -0.04 0.630 0.07 0.632 0.158 0.642 L 0.152 0.700 C 0.140 0.800 0.132 0.940 0.128 1.030 C 0.126 1.120 0.134 1.220 0.138 1.300 C 0.136 1.390 0.110 1.450 0.062 1.482 C 0.040 1.495 0.010 1.505 -0.030 1.500 Z"),
  sh("coat-lapel", "torso", "detail", 1, 4.1, 0.2, "coat", "wool-coat", "M 0.062 1.482 L 0.098 1.40 L 0.062 1.35 L 0.030 1.46 Z"),
  line("coat-front-edge", "torso", "detail", 1, 4.1, 0.1, "coat", "wool-coat", [[0.062, 1.35], [0.100, 1.10], [0.136, 0.645]]),
  line("coat-back-vent", "torso", "detail", 2, 4.1, 0.1, "coat", "wool-coat", [[-0.152, 0.645], [-0.142, 0.88]]),
  line("coat-hem", "torso", "detail", 2, 4.1, 0.1, "coat", "wool-coat", [[-0.148, 0.668], [0.150, 0.670]]),
  sh("coat-pocket", "torso", "detail", 2, 4.1, 0.2, "coat", "wool-coat", "M -0.005 0.90 L 0.105 0.90 L 0.105 0.855 L -0.005 0.855 Z"),
  ell("coat-button-1", "torso", "detail", 3, 4.2, 0.1, "coat", "wool-coat", [0.118, 1.28], 0.008, 0.008),
  ell("coat-button-2", "torso", "detail", 3, 4.2, 0.1, "coat", "wool-coat", [0.122, 1.17], 0.008, 0.008),
  ell("coat-button-3", "torso", "detail", 3, 4.2, 0.1, "coat", "wool-coat", [0.126, 1.06], 0.008, 0.008),
  line("coat-weave-1", "torso", "texture", 3, 4.05, 0.2, "coat", "wool-coat", [[-0.10, 1.38], [-0.05, 1.33], [0.0, 1.38], [0.05, 1.33]]),
  line("coat-weave-2", "torso", "texture", 3, 4.05, 0.2, "coat", "wool-coat", [[-0.12, 1.15], [-0.07, 1.10], [-0.02, 1.15], [0.03, 1.10]]),
  cap("bag-strap-front", "bag", "detail", 1, 5.4, 0.2, "bag-leather", "leather-bag", [0.005, 1.462], [0.12, 1.030], 0.010),
  cap("bag-strap-back", "bag", "detail", 1, 5.4, 0.2, "bag-leather", "leather-bag", [-0.005, 1.462], [-0.12, 1.030], 0.010),
  sh("bag-body", "bag", "body", 0, 5.5, 0.42, "bag-leather", "leather-bag", "M -0.145 1.02 C -0.150 0.95 -0.148 0.88 -0.140 0.83 C -0.08 0.815 0.07 0.815 0.135 0.83 C 0.145 0.88 0.148 0.95 0.145 1.02 C 0.07 1.03 -0.07 1.03 -0.145 1.02 Z"),
  sh("bag-flap", "bag", "detail", 1, 5.55, 0.34, "bag-leather", "leather-bag", "M -0.147 1.02 L 0.147 1.02 L 0.140 0.93 C 0.05 0.915 -0.05 0.915 -0.14 0.93 Z"),
  sh("bag-buckle", "bag", "detail", 3, 5.6, 0.7, "bag-leather", "leather-bag", "M -0.012 0.955 L 0.012 0.955 L 0.012 0.925 L -0.012 0.925 Z"),
  line("bag-stitching", "bag", "texture", 3, 5.6, 0.2, "bag-leather", "leather-bag", [[-0.135, 0.845], [0.128, 0.845]]),
];

const standing = { "arm-r": { angleDeg: -2 }, "forearm-r": { angleDeg: 22 }, "hand-r": { angleDeg: -6 }, "arm-l": { angleDeg: 3 }, "forearm-l": { angleDeg: 10 }, "hand-l": { angleDeg: 4 }, bag: { angleDeg: 0 }, neck: { angleDeg: 3 }, head: { angleDeg: 1 }, torso: { angleDeg: -1 } };
const pos = [
  { id: "standing", title: tx("Standing with shoulder bag", "Stehend mit Umhängetasche"), description: tx("Relaxed upright stance, near hand on the bag strap, far arm hanging, both feet flat.", "Entspannter aufrechter Stand, nahe Hand am Taschenriemen, ferner Arm hängend, beide Füße flach."), bones: standing, ground: "auto", contacts: FEET4, thermal: { activityMet: 1.2, emitters: { breath: 0.5 } }, loads: [BAG] },
  { id: "check-watch", title: tx("Checking the watch", "Blick auf die Uhr"), description: tx("Standing still, near forearm raised in front of the chest, head and neck bowed towards the wrist.", "Ruhiger Stand, naher Unterarm vor der Brust angehoben, Kopf und Hals zum Handgelenk geneigt."), bones: { ...standing, "arm-r": { angleDeg: 22 }, "forearm-r": { angleDeg: 112 }, "hand-r": { angleDeg: -12 }, neck: { angleDeg: -8 }, head: { angleDeg: -10 }, torso: { angleDeg: -3 }, bag: { angleDeg: 6 } }, ground: "auto", contacts: FEET4, thermal: { activityMet: 1.3, emitters: { breath: 0.5 } }, loads: [BAG] },
  poseAt("contact-left", "Left heel strike", "Linker Fersenkontakt", "Left heel strike, right foot rolling off its forefoot, far arm swinging back.", "Linker Fersenkontakt, rechter Fuß rollt über den Vorfuß ab, ferner Arm schwingt zurück.", 0, ["heel-l", "ball-r"]),
  poseAt("loading-left", "Left loading response", "Linke Belastungsantwort", "Left foot flat, right toe-off, the far arm passing the hip.", "Linker Fuß flach, rechter Abdruck, ferner Arm am Becken vorbei.", 0.12, ["heel-l", "ball-l", "ball-r"]),
  poseAt("passing-left", "Left mid-stance", "Linke Standbeinmitte", "Body over the left foot, right leg swinging through.", "Körper über dem linken Fuß, rechtes Bein schwingt durch.", 0.31, ["heel-l", "ball-l"]),
  poseAt("contact-right", "Right heel strike", "Rechter Fersenkontakt", "Right heel strike, left heel rising off the ground.", "Rechter Fersenkontakt, linke Ferse hebt ab.", 0.5, ["heel-r", "ball-l"]),
  poseAt("loading-right", "Right loading response", "Rechte Belastungsantwort", "Right foot flat, left toe-off.", "Rechter Fuß flach, linker Abdruck.", 0.62, ["heel-r", "ball-r", "ball-l"]),
  poseAt("passing-right", "Right mid-stance", "Rechte Standbeinmitte", "Body over the right foot, left leg swinging through.", "Körper über dem rechten Fuß, linkes Bein schwingt durch.", 0.81, ["heel-r", "ball-r"]),
];

const N = 48;
const walkKeys = Array.from({ length: N }, (_, k) => ({ t: r4((k / N) * T), pose: "contact-left", ease: "smooth", bones: gait(k / N) }));
const ev = (t, kind, en, de, target) => ({ t, kind, title: tx(en, de), ...(target ? { target } : {}) });
const clips = [
  {
    id: "walk-cycle", title: tx("Walking to work", "Gehen zur Arbeit"), description: tx("Level walking at about 1.1 m/s (4 km/h), 100 steps per minute; feet planted without sliding, heel strike to toe-off with double support, far arm swinging, bag swaying. Joint angles from two-link inverse kinematics of smooth foot paths.", "Gehen in der Ebene mit etwa 1,1 m/s (4 km/h), 100 Schritte pro Minute; Füße gleiten nicht, Fersenkontakt bis Abdruck mit Doppelstandphase, ferner Arm schwingt, Tasche pendelt. Gelenkwinkel aus zweigliedriger inverser Kinematik glatter Fußbahnen."),
    durationS: T, fps: 30, loop: true, rootMotion: "plant", advanceM: ADV["walk-cycle"] ?? 1.2, keys: walkKeys,
    contacts: [{ anchor: "heel-l", fromS: 0, toS: 0.372 }, { anchor: "ball-l", fromS: 0.144, toS: 0.744 }, { anchor: "heel-r", fromS: 0.6, toS: 0.972 }, { anchor: "ball-r", fromS: 0.744, toS: 1.2 }, { anchor: "ball-r", fromS: 0, toS: 0.144 }],
    phases: [{ id: "double-support-left-lead", fromS: 0, toS: 0.144, title: tx("Double support, left foot leading", "Doppelstandphase, linker Fuß vorn") }, { id: "left-single-support", fromS: 0.144, toS: 0.6, title: tx("Left single support", "Linke Einbeinstandphase") }, { id: "double-support-right-lead", fromS: 0.6, toS: 0.744, title: tx("Double support, right foot leading", "Doppelstandphase, rechter Fuß vorn") }, { id: "right-single-support", fromS: 0.744, toS: 1.2, title: tx("Right single support", "Rechte Einbeinstandphase") }],
    events: [ev(0, "contact-on", "Left heel strike", "Linker Fersenkontakt", "heel-l"), ev(0.144, "contact-off", "Right toe-off", "Rechter Abdruck", "ball-r"), ev(0.144, "contact-on", "Left foot flat", "Linker Fuß flach", "ball-l"), ev(0.372, "contact-off", "Left heel-off", "Linke Ferse hebt ab", "heel-l"), ev(0.6, "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(0.744, "contact-off", "Left toe-off", "Linker Abdruck", "ball-l"), ev(0.744, "contact-on", "Right foot flat", "Rechter Fuß flach", "ball-r"), ev(0.972, "contact-off", "Right heel-off", "Rechte Ferse hebt ab", "heel-r"), ev(0.9, "exhale", "Exhalation while walking", "Ausatmen beim Gehen"), ev(0.3, "inhale", "Inhalation while walking", "Einatmen beim Gehen")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 0.6, v: 0.8 }, { t: 1.2, v: 1 }] }],
  },
  {
    id: "glance-at-watch", title: tx("Pausing to glance at the watch", "Innehalten mit Blick auf die Uhr"), description: tx("Standing still with both feet flat: the near forearm rises in front of the chest to read the watch, holds, and is lowered again while the head follows.", "Ruhiger Stand mit beiden Füßen flach: Der nahe Unterarm hebt sich vor die Brust zum Lesen der Uhr, hält und wird wieder gesenkt, der Kopf folgt."),
    durationS: 4, fps: 30, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "standing", ease: "smooth" }, { t: 1.2, pose: "check-watch", ease: "smooth" }, { t: 2.6, pose: "check-watch", ease: "smooth" }, { t: 4, pose: "standing", ease: "smooth" }],
    contacts: FEET4.map((anchor) => ({ anchor, fromS: 0, toS: 4 })),
    phases: [{ id: "raise", fromS: 0, toS: 1.2, title: tx("Raising the arm", "Arm heben") }, { id: "read", fromS: 1.2, toS: 2.6, title: tx("Reading the watch", "Uhr ablesen") }, { id: "lower", fromS: 2.6, toS: 4, title: tx("Lowering the arm", "Arm senken") }],
    events: [ev(1.2, "motion", "Watch at eye level", "Uhr auf Augenhöhe"), ev(2.6, "motion", "Arm starts to lower", "Arm beginnt zu sinken"), ev(0.4, "inhale", "Breath in", "Einatmen"), ev(1.6, "exhale", "Breath out while reading", "Ausatmen beim Lesen")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.2 }, { t: 1.2, v: 1.3 }, { t: 4, v: 1.2 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.5 }, { t: 2, v: 0.6 }, { t: 4, v: 0.5 }] }],
  },
  {
    id: "idle-breathing", title: tx("Waiting at rest", "Ruhiges Warten"), description: tx("Standing at a crossing with a 4 s breathing cycle and slight chest motion.", "Warten an einer Kreuzung mit 4-s-Atemzyklus und leichter Brustbewegung."),
    durationS: 4, fps: 30, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "standing", ease: "smooth" }, { t: 2, pose: "standing", ease: "smooth", bones: { torso: { angleDeg: 0 }, head: { angleDeg: 0 } } }, { t: 4, pose: "standing", ease: "smooth" }],
    contacts: FEET4.map((anchor) => ({ anchor, fromS: 0, toS: 4 })),
    phases: [{ id: "inhale", fromS: 0, toS: 1.6, title: tx("Inhale", "Einatmen") }, { id: "exhale", fromS: 1.6, toS: 4, title: tx("Exhale", "Ausatmen") }],
    events: [ev(0, "inhale", "Start of inhalation", "Beginn des Einatmens"), ev(1.6, "exhale", "Start of exhalation", "Beginn des Ausatmens")],
    tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 1.6, v: 0 }, { t: 2.0, v: 0.6 }, { t: 3.2, v: 0.35 }, { t: 4, v: 0 }] }],
  },
];

export default {
  id: "walking-commuter",
  title: tx("Walking Commuter", "Pendler zu Fuß"),
  summary: tx("An adult in a knee-length wool overcoat and scarf walking to work with a shoulder bag at the hip: heel-to-toe gait with double support, a free-swinging far arm and a swaying bag whose weight is carried by the near shoulder.", "Ein Erwachsener in knielangem Wollmantel mit Schal auf dem Weg zur Arbeit, Umhängetasche an der Hüfte: Gang von der Ferse zum Zeh mit Doppelstandphase, frei schwingender ferner Arm und pendelnde Tasche, deren Gewicht die nahe Schulter trägt."),
  tags: ["human", "adult", "walker", "commuter", "overcoat", "bag", "gait"],
  provenance: {
    author: "Semio staffage content worker humans-a", method: "handcrafted", created: "2026-10-29",
    references: refs(DL, WI, "drillis-contini-1966", "perry-burnfield-2010", "dubois-1916", ISO, "iso-8996-2021", "iso-7730-2005", "ashrae-2021", GU),
    notes: tx("Silhouette, clothing and rig are handcrafted for a 1.78 m, 72 kg man (7.5 heads); segment masses follow de Leva (1996) male fractions of 72 kg plus 2.7 kg of clothing and shoes, segment lengths follow Drillis and Contini. Gait key frames come from two-link inverse kinematics of smooth foot paths with phase timing after Perry and Burnfield. The 3.5 kg bag is an applied carried load, not a bone mass.", "Silhouette, Kleidung und Skelett sind von Hand für einen 1,78 m großen, 72 kg schweren Mann (7,5 Kopfhöhen) gezeichnet; Segmentmassen folgen den männlichen Anteilen nach de Leva (1996) für 72 kg plus 2,7 kg Kleidung und Schuhe, Segmentlängen Drillis und Contini. Die Ganghaltungen stammen aus zweigliedriger inverser Kinematik glatter Fußbahnen mit Phasenzeiten nach Perry und Burnfield. Die 3,5 kg schwere Tasche ist eine aufgebrachte Last, keine Knochenmasse."),
  },
  dimensions: { heightM: 1.78, widthM: 0.4, depthM: 0.5, massKg: 74.7 },
  materials: [
    mat("skin", "Skin", "Haut", 0.98, 0.6, "literature", "ashrae-2021"),
    mat("hair", "Hair", "Haar", 0.98, 0.85, "authored-estimate"),
    mat("wool-coat", "Wool overcoat", "Wollmantel", 0.92, 0.8, "literature", "ashrae-2021"),
    mat("wool-scarf", "Wool scarf", "Wollschal", 0.92, 0.65, "literature", "ashrae-2021"),
    mat("wool-trouser", "Wool trousers", "Wollhose", 0.92, 0.7, "literature", "ashrae-2021"),
    mat("leather-shoe", "Leather shoe", "Lederschuh", 0.9, 0.75, "authored-estimate"),
    mat("leather-bag", "Leather bag", "Ledertasche", 0.9, 0.6, "authored-estimate"),
  ],
  bones, anchors, shapes, poses: pos, clips,
  thermal: {
    environment: ENV(8, 8, 0.75, 1, 0), baseMet: 1.2,
    regions: [
      region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", "core-resistance", 0.095, 0.2, 0.07, { coreTempC: 36.8, insulationM2KW: 0.04 }, "dubois-1916"),
      region("hair", "Hair", "Haar", "hair", "hair", "core-resistance", 0.045, 0.18, 0.02, { coreTempC: 36.8, insulationM2KW: 0.08 }, "dubois-1916", "authored-estimate"),
      region("hand-skin", "Bare hands", "Unbedeckte Hände", "skin", "skin", "core-resistance", 0.095, 0.1, 0.05, { coreTempC: 35, insulationM2KW: 0.05 }, "dubois-1916"),
      region("coat", "Overcoat on torso and arms", "Mantel an Rumpf und Armen", "clothing", "wool-coat", "core-resistance", 1.0, 0.3, 0.5, { coreTempC: 36.8, insulationM2KW: 0.27 }, ISO),
      region("scarf", "Scarf", "Schal", "clothing", "wool-scarf", "core-resistance", 0.04, 0.1, 0.02, { coreTempC: 36.8, insulationM2KW: 0.2 }, ISO),
      region("trousers", "Trousers", "Hose", "clothing", "wool-trouser", "core-resistance", 0.5, 0.13, 0.29, { coreTempC: 36.8, insulationM2KW: 0.14 }, ISO),
      region("shoes", "Shoes", "Schuhe", "surface", "leather-shoe", "core-resistance", 0.115, 0.1, 0.05, { coreTempC: 34, insulationM2KW: 0.12 }, ISO, "authored-estimate"),
      region("bag-leather", "Leather bag", "Ledertasche", "surface", "leather-bag", "passive", 0.28, 0.25, undefined, {}, "iso-9920-2007", "authored-estimate"),
    ],
    emitters: [{ id: "breath", kind: "breath", title: tx("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: -15, carries: "vapour", temperatureC: 34, velocityMS: 1.3, diameterM: 0.02, vapourKgPerKg: 0.0345, periodS: 3, dutyFrom: 0.4, dutyTo: 1, source: "literature", reference: GU }],
    assumptions: [tx("Body regions sum to the DuBois area of 1.89 m² (72 kg, 178 cm); the bag is an additional passive surface.", "Die Körperregionen ergeben die DuBois-Fläche von 1,89 m² (72 kg, 178 cm); die Tasche ist eine zusätzliche passive Fläche."), tx("Insulation values are core-to-surface and include tissue (about 0.03 m²K/W); clothing follows ISO 9920 for a suit with overcoat (about 1.7 clo).", "Dämmwerte gelten vom Kern zur Oberfläche und enthalten das Gewebe (etwa 0,03 m²K/W); die Kleidung folgt ISO 9920 für Anzug mit Mantel (etwa 1,7 clo)."), tx("Walking at 4 km/h is 2.8 met (ISO 8996); breathing 20 per minute with 60 % exhalation.", "Gehen mit 4 km/h entspricht 2,8 met (ISO 8996); 20 Atemzüge pro Minute mit 60 % Ausatmung.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [tx("Segments are point masses at the de Leva centres; the 3.5 kg bag is an applied load at its centre of gravity.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; die 3,5 kg schwere Tasche ist eine Last in ihrem Schwerpunkt."), tx("Contacts are the heel and the ball of each foot; supports follow the clip contact spans.", "Kontakte sind Ferse und Fußballen jedes Fußes; Auflager folgen den Kontaktspannen des Clips.")], clinical: false },
  presentation: { defaultPose: "standing", defaultClip: "walk-cycle", viewpoints: ["elevation"] },
  exports: STD_EXPORTS,
};
