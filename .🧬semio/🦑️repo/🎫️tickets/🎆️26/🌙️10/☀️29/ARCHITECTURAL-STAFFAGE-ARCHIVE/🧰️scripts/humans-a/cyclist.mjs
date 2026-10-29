// [DEBUG] design source: cyclist (woman 1.72 m, 66 kg, on a 28 inch city bicycle; rider and bicycle in one rig).
import { tx, refs, rev, FIXED, deg, rad, r4, sm5, R, xf, xp, xpt, armIK, ik2, circ, ring, arcBand, sh, cap, ell, line, mass, STD_EXPORTS, ENV, mat, region } from "./common.mjs";
import C, { PAT } from "./walking-commuter.mjs";
const DL = "de-leva-1996", WI = "winter-2009", ISO = "iso-9920-2007", GU = "gupta-2010", BIKE = "bicycle-geometry";
const k = 0.966, RW = 0.34, RX = [0, RW], FX = [1.05, RW], BB = [0.45, 0.30], CRK = 0.17, GEAR = 2, HIP = [0.255, 1.0];
const A1 = 0.33 * k, A2 = 0.26 * k, HL = 0.19 * k, L1 = 0.435 * k, L2 = 0.425 * k, PEL = 0.1 * k, TOR = 0.465 * k, NECK = 0.1 * k, HEAD = 0.18 * k;
const WA = { pelvis: 80, torso: 68, neck: 50, head: 78 };
const dir = (a) => [Math.cos(rad(a)), Math.sin(rad(a))], add = (p, v, s = 1) => [p[0] + v[0] * s, p[1] + v[1] * s];
const LUMB = add(HIP, dir(WA.pelvis), PEL), C7 = add(LUMB, dir(WA.torso), TOR), NTOP = add(C7, dir(WA.neck), NECK), SHO = add(LUMB, dir(WA.torso), 0.42 * k);
const PU = (d) => xf(d, k, WA.pelvis - 90, [0, 0.935], HIP), TU = (d) => xf(d, k, WA.torso - 90, [0, 1.035], LUMB), TUp = (p) => xpt(p, k, WA.torso - 90, [0, 1.035], LUMB);
const NK = (d) => xf(d, k, WA.neck - 82, [0, 1.5], C7), HD = (d) => xf(d, k, WA.head - 90, [0.0139, 1.599], NTOP), HDp = (p) => xpt(p, k, WA.head - 90, [0.0139, 1.599], NTOP);
const LEG = (d) => xf(d, k, 0, [0, 0.935], HIP), KNEE0 = add(HIP, [0, -1], L1), ANK0 = add(KNEE0, [0, -1], L2);
const SHK = (d) => xf(d, k, 0, [0, 0.5], KNEE0), FT = (d) => xf(d, k, 0, [0, 0.075], ANK0), FTp = (p) => xpt(p, k, 0, [0, 0.075], ANK0);
const ARM = (d) => xf(d, k, WA.torso - 90 + 0, [0, 1.455], SHO), WRIST0 = [SHO[0], SHO[1] - A1 - A2];
const ARMd = (d) => xf(d, k, 0, [0, 1.455], SHO);
const aq = (a) => ({ angleDeg: r4(a) });
const GRIP = [0.74, 1.06], BALLO = [0.17 * k, -0.075 * k];
const crank = (th) => add(BB, dir(th), CRK);
const shAt = (th, hipD) => { const q = R(th, [SHO[0] - LUMB[0], SHO[1] - LUMB[1]]); return [LUMB[0] + q[0] + hipD[0], LUMB[1] + q[1] + hipD[1]]; };
const rider = ({ th, hipD = [0, 0], thetaR, phi = -22, torso, neck = 0, head = 0, grip = GRIP, feet = null }) => {
  const hip = add(HIP, hipD), out = {};
  const sTorso = th;
  const s = shAt(sTorso, hipD), dx = grip[0] - s[0], dy = grip[1] - s[1], d = Math.hypot(dx, dy), wr = [grip[0] - (HL * 0.5) * dx / d, grip[1] - (HL * 0.5) * dy / d];
  const a = armIK(s, wr, A1, A2);
  for (const sd of ["l", "r"]) { out[`arm-${sd}`] = aq(a.arm - sTorso); out[`forearm-${sd}`] = aq(a.forearm); out[`hand-${sd}`] = aq(0); }
  const sn = Math.sin(rad(thetaR)), targets = feet ?? { r: { ball: add(crank(thetaR), [0, 0.012]), phi: phi - 10 * sn }, l: { ball: add(crank(thetaR + 180), [0, 0.012]), phi: phi + 10 * sn } };
  for (const sd of ["l", "r"]) { const t = targets[sd], o = R(t.phi, BALLO), ank = [t.ball[0] - o[0], t.ball[1] - o[1]], q = ik2(hip, ank, L1, L2, t.phi, 0.012);
    out[`thigh-${sd}`] = aq(q.thigh); out[`shank-${sd}`] = aq(q.shank); out[`foot-${sd}`] = aq(q.foot); out[`toe-${sd}`] = aq(t.toe ?? 0); }
  return { ...out, torso: aq(torso ?? th), neck: aq(neck), head: aq(head), "hip-lift": { dy: r4(hipD[1]) }, "hip-shift": { dx: r4(hipD[0]) }, pelvis: aq(0) };
};
const bikeBones = (thetaR, spinDeg) => ({ "crank-r": aq(thetaR + 90), "crank-l": aq(thetaR + 90), "pedal-r": aq(-(thetaR + 90)), "pedal-l": aq(-(thetaR + 90)), "rear-wheel": aq(-spinDeg), "front-wheel": aq(-spinDeg) });
const RIDE = (thetaR, spin) => ({ ...rider({ th: 2 * Math.cos(rad(2 * thetaR)) * 0.5 - 0, thetaR, torso: -2 + 1.2 * Math.sin(rad(2 * thetaR)), neck: 4, head: 3 }), ...bikeBones(thetaR, spin) });
const RIDE_STAND = (thetaR, spin) => ({ ...rider({ th: -18, hipD: [0.1, 0.13], thetaR, phi: -12, torso: -18 + 4 * Math.sin(rad(thetaR)), neck: 14, head: 6, grip: [0.78, 1.08] }), ...bikeBones(thetaR, spin) });
const PED0 = -60;
const mkPose = (id, en, de, dEn, dDe, bones, met, breath, contacts = ["patch-rear", "patch-front"]) => ({ id, title: tx(en, de), description: tx(dEn, dDe), bones, ground: "auto", contacts, thermal: { activityMet: met, emitters: { breath } }, loads: [] });
const DISM = () => { const f = (x) => ({ ball: [x, 0.0], phi: 0, toe: 0 }); return { ...rider({ th: 4, hipD: [0.17, -0.13], thetaR: 0, torso: 4, neck: 8, head: 4, grip: [0.76, 1.08], feet: { r: { ball: [0.5, 0.0], phi: 0 }, l: { ball: [0.38, 0.0], phi: 0 } } }), ...bikeBones(0, 0) }; };
const poses = [
  mkPose("riding-upright", "Riding upright", "Aufrecht fahrend", "Seated on the saddle with straight arms on the swept-back bars, the feet on the pedals with the right crank at 4 o'clock.", "Aufrecht im Sattel, mit gestreckten Armen am geschwungenen Lenker, die Füße auf den Pedalen, die rechte Kurbel auf vier Uhr.", RIDE(PED0, 0), 4, 1),
  mkPose("standing-on-pedals", "Standing on the pedals", "Stehend auf den Pedalen", "Out of the saddle with level cranks, the weight on both pedals and the trunk leaning forward over the bars.", "Aus dem Sattel gehoben bei waagerechten Kurbeln, das Gewicht auf beiden Pedalen und der Rumpf über dem Lenker nach vorn geneigt.", RIDE_STAND(0, 0), 7, 1),
  mkPose("coasting", "Coasting with level cranks", "Rollen mit waagerechten Kurbeln", "Seated, not pedalling, the cranks horizontal and the weight on the saddle.", "Sitzend ohne zu treten, die Kurbeln waagerecht und das Gewicht auf dem Sattel.", RIDE(0, 0), 2.2, 0.6),
  mkPose("dismounted-astride", "Standing astride the frame", "Stehend über dem Rahmen", "Both feet on the ground either side of the frame, hands on the bars, wheels at rest.", "Beide Füße beidseits des Rahmens auf dem Boden, die Hände am Lenker, die Räder in Ruhe.", DISM(), 1.8, 0.7, ["patch-rear", "patch-front", "heel-l", "ball-l", "heel-r", "ball-r"]),
];
const KS = 40, NC = 4, DUR = NC, ev = (t, kind, en, de, target) => ({ t, kind, title: tx(en, de), ...(target ? { target } : {}) });
const pedalKeys = Array.from({ length: NC * KS }, (_, i) => { const t = i / KS, th = PED0 - 360 * t; return { t: r4(t), pose: "riding-upright", ease: "smooth", bones: RIDE(th, 720 * t) }; });
// dismount: braking over 0..1.6 s, feet down, hip forward
const brakeSpin = (t) => (t < 1.6 ? 360 * (4.27 / (2 * Math.PI * RW)) * (t - (t * t) / (2 * 1.6)) : 360 * (4.27 / (2 * Math.PI * RW)) * 0.8);
const crankAt = (t) => (t < 1.0 ? PED0 - 360 * t + 360 * 0 : PED0 - 360 * 1.0 - 0);
const dismountKeys = (() => {
  const keys = [];
  for (let i = 0; i <= 16; i++) { const t = (i / 16) * 1.6, th = PED0 - 300 * (1 - Math.cos(Math.PI * Math.min(1, t / 1.6))) / 2 * 1; const spin = brakeSpin(t); keys.push({ t: r4(t), pose: "riding-upright", ease: "smooth", bones: { ...RIDE(th, spin), ...rider({ th: -2 - 8 * sm5(t / 1.6), thetaR: th, torso: -2 - 8 * sm5(t / 1.6), neck: 4, head: 3 }) } }); }
  const lastTheta = PED0 - 300 * 1;
  return keys;
})();
const bones = [
  { id: "frame", parent: null, title: tx("Bicycle frame, fork and handlebar", "Fahrradrahmen mit Gabel und Lenker"), at: [0, 0], worldAngleDeg: 0, lengthM: 1.05, joint: FIXED, mass: mass(8.2, [0.5, 0.65], BIKE, { source: "authored-estimate" }) },
  { id: "rear-wheel", parent: "frame", title: tx("Rear wheel with cog", "Hinterrad mit Ritzel"), at: RX, worldAngleDeg: 0, lengthM: RW, joint: { type: "revolute" }, mass: mass(2.4, RX, BIKE, { source: "authored-estimate" }) },
  { id: "front-wheel", parent: "frame", title: tx("Front wheel", "Vorderrad"), at: FX, worldAngleDeg: 0, lengthM: RW, joint: { type: "revolute" }, mass: mass(2.1, FX, BIKE, { source: "authored-estimate" }) },
  { id: "crank-r", parent: "frame", title: tx("Right crank with chainring", "Rechte Kurbel mit Kettenblatt"), at: BB, worldAngleDeg: -90, lengthM: CRK, joint: { type: "revolute" }, mass: mass(0.65, 0.07, BIKE, { source: "authored-estimate" }) },
  { id: "crank-l", parent: "frame", title: tx("Left crank", "Linke Kurbel"), at: BB, worldAngleDeg: 90, lengthM: CRK, joint: { type: "revolute" }, mass: mass(0.35, 0.085, BIKE, { source: "authored-estimate" }) },
  { id: "pedal-r", parent: "crank-r", title: tx("Right pedal", "Rechtes Pedal"), at: add(BB, [0, -1], CRK), worldAngleDeg: 0, lengthM: 0.05, joint: { type: "revolute" }, mass: mass(0.25, 0, BIKE, { source: "authored-estimate" }) },
  { id: "pedal-l", parent: "crank-l", title: tx("Left pedal", "Linkes Pedal"), at: add(BB, [0, 1], CRK), worldAngleDeg: 0, lengthM: 0.05, joint: { type: "revolute" }, mass: mass(0.25, 0, BIKE, { source: "authored-estimate" }) },
  { id: "hip-lift", parent: "frame", title: tx("Pelvis vertical travel", "Vertikale Beckenverschiebung"), at: HIP, worldAngleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 90, minM: -0.15, maxM: 0.3 } },
  { id: "hip-shift", parent: "hip-lift", title: tx("Pelvis horizontal travel", "Horizontale Beckenverschiebung"), at: HIP, worldAngleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: 0, minM: -0.2, maxM: 0.3 } },
  { id: "pelvis", parent: "hip-shift", title: tx("Pelvis on the saddle", "Becken auf dem Sattel"), at: HIP, worldAngleDeg: WA.pelvis, lengthM: PEL, joint: rev(-25, 25), mass: mass(8.53, 0.06, DL) },
  { id: "torso", parent: "pelvis", title: tx("Torso with jacket", "Rumpf mit Jacke"), at: LUMB, worldAngleDeg: WA.torso, lengthM: TOR, joint: rev(-60, 30), mass: mass(20.7, 0.215, DL) },
  { id: "neck", parent: "torso", title: tx("Neck", "Hals"), at: C7, worldAngleDeg: WA.neck, lengthM: NECK, joint: rev(-35, 40), mass: mass(0.55, 0.05, DL, { source: "authored-estimate" }) },
  { id: "head", parent: "neck", title: tx("Head with helmet", "Kopf mit Helm"), at: NTOP, worldAngleDeg: WA.head, lengthM: HEAD, joint: rev(-30, 30), mass: mass(4.16, 0.075, DL) },
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `arm-${s}`, parent: "torso", title: tx(`${n[0]} upper arm`, `${n[1]} Oberarm`), at: SHO, worldAngleDeg: -90, lengthM: A1, joint: rev(-60, 170), mass: mass(1.78, 0.18 * k, DL) },
    { id: `forearm-${s}`, parent: `arm-${s}`, title: tx(`${n[0]} forearm`, `${n[1]} Unterarm`), at: [SHO[0], SHO[1] - A1], worldAngleDeg: -90, lengthM: A2, joint: rev(0, 150), mass: mass(0.96, 0.112 * k, DL) },
    { id: `hand-${s}`, parent: `forearm-${s}`, title: tx(`${n[0]} gloved hand`, `${n[1]} behandschuhte Hand`), at: WRIST0, worldAngleDeg: -90, lengthM: HL, joint: rev(-60, 60), mass: mass(0.42, 0.09 * k, WI) },
    { id: `thigh-${s}`, parent: "pelvis", title: tx(`${n[0]} thigh`, `${n[1]} Oberschenkel`), at: HIP, worldAngleDeg: -90, lengthM: L1, joint: rev(-40, 125), mass: mass(9.91, 0.168 * k, DL) },
    { id: `shank-${s}`, parent: `thigh-${s}`, title: tx(`${n[0]} shank`, `${n[1]} Unterschenkel`), at: KNEE0, worldAngleDeg: -90, lengthM: L2, joint: rev(-150, 0), mass: mass(3.27, 0.179 * k, DL) },
    { id: `foot-${s}`, parent: `shank-${s}`, title: tx(`${n[0]} foot on the pedal`, `${n[1]} Fuß auf dem Pedal`), at: ANK0, worldAngleDeg: -23.8, lengthM: 0.1858 * k, joint: rev(-60, 45), mass: mass(1.12, FTp([0.071, 0.04]), DL) },
    { id: `toe-${s}`, parent: `foot-${s}`, title: tx(`${n[0]} forefoot`, `${n[1]} Vorfuß`), at: FTp([0.17, 0]), worldAngleDeg: 0, lengthM: 0.065 * k, joint: rev(0, 65), mass: mass(0.08, FTp([0.2, 0.03]), DL, { source: "authored-estimate" }) },
  ]; }),
];
const anchors = [
  { id: "patch-rear", bone: "frame", at: [0, 0], kind: "contact", title: tx("Rear tyre contact patch", "Aufstandsfläche des Hinterreifens"), support: "contact", normalDeg: 90, footprintLengthM: 0.08, footprintWidthM: 0.04, frictionCoefficient: 0.8 },
  { id: "patch-front", bone: "frame", at: [1.05, 0], kind: "contact", title: tx("Front tyre contact patch", "Aufstandsfläche des Vorderreifens"), support: "contact", normalDeg: 90, footprintLengthM: 0.08, footprintWidthM: 0.04, frictionCoefficient: 0.8 },
  { id: "pedal-spindle-r", bone: "pedal-r", at: add(BB, [0, -1], CRK), kind: "attachment", title: tx("Right pedal spindle", "Rechte Pedalachse") },
  { id: "pedal-spindle-l", bone: "pedal-l", at: add(BB, [0, 1], CRK), kind: "attachment", title: tx("Left pedal spindle", "Linke Pedalachse") },
  { id: "saddle", bone: "frame", at: [0.25, 0.92], kind: "attachment", title: tx("Saddle seat point", "Sattelauflage") },
  ...["l", "r"].flatMap((s) => { const n = s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]; return [
    { id: `heel-${s}`, bone: `foot-${s}`, at: FTp([-0.045, 0]), kind: "contact", title: tx(`${n[0]} heel`, `${n[1]} Ferse`), support: "contact", normalDeg: 90, footprintLengthM: 0.065, footprintWidthM: 0.06, frictionCoefficient: 0.9 },
    { id: `ball-${s}`, bone: `foot-${s}`, at: FTp([0.17, 0]), kind: "contact", title: tx(`${n[0]} ball of foot`, `${n[1]} Fußballen`), support: "contact", normalDeg: 90, footprintLengthM: 0.095, footprintWidthM: 0.09, frictionCoefficient: 0.9 }]; }),
  { id: "mouth", bone: "head", at: HDp([0.094, 1.594]), kind: "emitter", title: tx("Mouth and nose", "Mund und Nase") },
  { id: "grip-r", bone: "hand-r", at: add(WRIST0, [0, -1], HL / 2), kind: "attachment", title: tx("Grip on the handlebar", "Griff am Lenker") },
];
const diam = (wheel, c, r, id, lod) => line(id, wheel, "detail", lod, 7.1, 0.5, "bike-metal", "steel", []);
const spokes = (wheel, c, pre) => Array.from({ length: 18 }, (_, i) => { const a = rad(i * 10); return line(`${pre}-spoke-${i}`, wheel, "texture", i % 3 === 0 ? 1 : 2, 7.1, 0.55, "bike-metal", "steel", [[c[0] - 0.315 * Math.cos(a), c[1] - 0.315 * Math.sin(a)], [c[0] + 0.315 * Math.cos(a), c[1] + 0.315 * Math.sin(a)]]); });
const JACKET = "M -0.03 1.50 C -0.085 1.49 -0.115 1.44 -0.118 1.38 C -0.12 1.30 -0.108 1.20 -0.115 1.10 C -0.122 1.03 -0.125 0.95 -0.12 0.92 L 0.13 0.92 C 0.128 0.98 0.126 1.05 0.126 1.10 C 0.124 1.2 0.135 1.25 0.145 1.31 C 0.135 1.39 0.108 1.45 0.062 1.482 C 0.04 1.495 0.01 1.505 -0.03 1.5 Z";
const shapes = [
  line("ground", "frame", "ground", 0, 0, 0.5, undefined, undefined, [[-0.9, 0], [2.0, 0]]),
  sh("rear-tyre", "rear-wheel", "body", 0, 1.4, 0.12, "tyres", "rubber", ring(RX[0], RX[1], 0.34, 0.322)),
  sh("rear-rim", "rear-wheel", "detail", 1, 1.45, 0.75, "bike-metal", "steel", ring(RX[0], RX[1], 0.322, 0.312)),
  ell("rear-hub", "rear-wheel", "body", 1, 1.6, 0.6, "bike-metal", "steel", RX, 0.025, 0.025),
  ell("rear-cog", "rear-wheel", "detail", 1, 1.55, 0.5, "bike-metal", "steel", RX, 0.0406, 0.0406),
  ...spokes("rear-wheel", RX, "rear"),
  sh("front-tyre", "front-wheel", "body", 0, 1.4, 0.12, "tyres", "rubber", ring(FX[0], FX[1], 0.34, 0.322)),
  sh("front-rim", "front-wheel", "detail", 1, 1.45, 0.75, "bike-metal", "steel", ring(FX[0], FX[1], 0.322, 0.312)),
  ell("front-hub", "front-wheel", "body", 1, 1.6, 0.6, "bike-metal", "steel", FX, 0.025, 0.025),
  ...spokes("front-wheel", FX, "front"),
  sh("rear-fender", "frame", "body", 1, 1.7, 0.3, "bike-metal", "steel", arcBand(RX[0], RX[1], 0.375, 0.362, 10, 150)),
  sh("front-fender", "frame", "body", 1, 1.7, 0.3, "bike-metal", "steel", arcBand(FX[0], FX[1], 0.375, 0.362, 20, 160)),
  cap("tube-seat", "frame", "body", 0, 1.8, 0.2, "bike-metal", "steel", BB, [0.275, 0.874], 0.016),
  cap("tube-top", "frame", "body", 0, 1.8, 0.2, "bike-metal", "steel", [0.275, 0.874], [0.955, 0.86], 0.016),
  cap("tube-down", "frame", "body", 0, 1.8, 0.2, "bike-metal", "steel", BB, [0.99, 0.76], 0.018),
  cap("tube-head", "frame", "body", 0, 1.85, 0.2, "bike-metal", "steel", [0.99, 0.76], [0.955, 0.875], 0.022),
  cap("tube-chainstay", "frame", "body", 0, 1.8, 0.2, "bike-metal", "steel", BB, RX, 0.011),
  cap("tube-seatstay", "frame", "body", 0, 1.8, 0.2, "bike-metal", "steel", [0.275, 0.874], RX, 0.01),
  cap("fork", "frame", "body", 0, 1.8, 0.2, "bike-metal", "steel", [0.99, 0.76], FX, 0.012),
  cap("stem", "frame", "body", 1, 1.9, 0.2, "bike-metal", "steel", [0.955, 0.875], [0.915, 1.0], 0.014),
  sh("handlebar", "frame", "body", 0, 1.95, 0.2, "bike-metal", "steel", "M 0.915 1.0 C 0.88 1.05 0.82 1.075 0.74 1.06 L 0.74 1.036 C 0.81 1.045 0.855 1.025 0.89 0.99 Z"),
  sh("grip", "frame", "body", 1, 1.96, 0.1, "bike-metal", "steel", "M 0.80 1.07 L 0.70 1.06 L 0.70 1.03 L 0.80 1.04 Z"),
  cap("seatpost", "frame", "body", 1, 1.8, 0.2, "bike-metal", "steel", [0.275, 0.874], [0.255, 0.925], 0.011),
  sh("saddle", "frame", "body", 0, 1.9, 0.15, "saddle", "leather-saddle", "M 0.13 0.925 C 0.14 0.95 0.2 0.955 0.27 0.945 C 0.32 0.94 0.37 0.935 0.39 0.915 C 0.37 0.905 0.3 0.905 0.2 0.905 C 0.15 0.905 0.13 0.91 0.13 0.925 Z"),
  sh("headlamp", "frame", "detail", 2, 1.95, 0.7, "bike-metal", "steel", "M 0.99 0.93 L 1.03 0.93 L 1.03 0.89 L 0.99 0.89 Z"),
  line("chain-top", "frame", "detail", 1, 1.7, 0.2, "bike-metal", "steel", [[0.45, 0.381], [0.0, 0.381]]),
  line("chain-bottom", "frame", "detail", 1, 1.7, 0.2, "bike-metal", "steel", [[0.45, 0.219], [0.0, 0.299]]),
  ell("chainring", "crank-r", "body", 0, 2.2, 0.55, "bike-metal", "steel", BB, 0.081, 0.081),
  line("chainring-mark", "crank-r", "detail", 1, 2.25, 0.2, "bike-metal", "steel", [add(BB, [0, -1], 0.03), add(BB, [0, -1], 0.075)]),
  cap("crank-arm-r", "crank-r", "body", 0, 2.3, 0.35, "bike-metal", "steel", BB, add(BB, [0, -1], CRK), 0.014, 0.011),
  cap("crank-arm-l", "crank-l", "body", 0, 0.9, 0.3, "bike-metal", "steel", BB, add(BB, [0, 1], CRK), 0.014, 0.011),
  sh("pedal-r", "pedal-r", "body", 0, 2.35, 0.15, "bike-metal", "steel", `M ${BB[0] - 0.052} ${BB[1] - CRK - 0.008} L ${BB[0] + 0.052} ${BB[1] - CRK - 0.008} L ${BB[0] + 0.052} ${BB[1] - CRK + 0.008} L ${BB[0] - 0.052} ${BB[1] - CRK + 0.008} Z`),
  sh("pedal-l", "pedal-l", "body", 0, 0.95, 0.15, "bike-metal", "steel", `M ${BB[0] - 0.052} ${BB[1] + CRK - 0.008} L ${BB[0] + 0.052} ${BB[1] + CRK - 0.008} L ${BB[0] + 0.052} ${BB[1] + CRK + 0.008} L ${BB[0] - 0.052} ${BB[1] + CRK + 0.008} Z`),
  ...["l", "r"].flatMap((s) => { const far = s === "l"; const tn = far ? -0.08 : 0; const z0 = far ? 0 : 3; return [
    sh(`sleeve-${s}`, `arm-${s}`, "body", 0, far ? 0.5 : 6, 0.3 + tn, "jacket", "softshell", ARMd(PAT.SLEEVE)),
    sh(`forearm-${s}`, `forearm-${s}`, "body", 0, far ? 0.5 : 6, 0.3 + tn, "jacket", "softshell", ARMd(PAT.FOREARM)),
    sh(`glove-${s}`, `hand-${s}`, "body", 0, far ? 0.6 : 6.2, 0.15 + tn, "gloves", "leather-glove", ARMd(PAT.HAND)),
    sh(`glove-thumb-${s}`, `hand-${s}`, "detail", 1, far ? 0.62 : 6.25, 0.15 + tn, "gloves", "leather-glove", ARMd(PAT.THUMB)),
    sh(`thigh-${s}`, `thigh-${s}`, "body", 0, z0 + 1, 0.2 + tn, "trousers", "stretch-trouser", LEG(PAT.TROUSER)),
    sh(`shank-${s}`, `shank-${s}`, "body", 0, z0 + 1.02, 0.2 + tn, "trousers", "stretch-trouser", SHK(PAT.SHANK)),
    sh(`trouser-clip-${s}`, `shank-${s}`, "detail", 2, z0 + 1.03, 0.75, "trousers", "stretch-trouser", SHK("M -0.05 0.17 L 0.046 0.17 L 0.044 0.15 L -0.049 0.15 Z")),
    sh(`shoe-rear-${s}`, `foot-${s}`, "body", 0, z0 + 1.1, 0.82 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_REAR)),
    sh(`shoe-toe-${s}`, `toe-${s}`, "body", 0, z0 + 1.1, 0.82 + tn, "shoes", "canvas-sneaker", FT(PAT.SHOE_FRONT)),
    line(`shoe-sole-${s}`, `foot-${s}`, "detail", 1, z0 + 1.15, 0.1, "shoes", "canvas-sneaker", xp([[-0.05, 0.022], [0.17, 0.022]], k, 0, [0, 0.075], ANK0)),
  ]; }),
  sh("pelvis-trousers", "pelvis", "body", 0, 3, 0.2, "trousers", "stretch-trouser", PU("M -0.115 1.04 C -0.138 0.99 -0.135 0.93 -0.110 0.86 L 0.100 0.86 C 0.130 0.93 0.132 0.99 0.118 1.04 Z")),
  sh("jacket", "torso", "body", 0, 4, 0.3, "jacket", "softshell", TU(JACKET)),
  line("jacket-reflective", "torso", "detail", 2, 4.1, 0.9, "jacket", "softshell", xp([[-0.12, 1.2], [0.14, 1.2]], k, WA.torso - 90, [0, 1.035], LUMB)),
  line("jacket-zip", "torso", "detail", 2, 4.1, 0.1, "jacket", "softshell", xp([[0.062, 1.482], [0.11, 1.2], [0.128, 0.93]], k, WA.torso - 90, [0, 1.035], LUMB)),
  line("jacket-fold", "torso", "texture", 3, 4.05, 0.2, "jacket", "softshell", xp([[-0.1, 1.28], [-0.05, 1.22], [-0.09, 1.16]], k, WA.torso - 90, [0, 1.035], LUMB)),
  sh("neck", "neck", "body", 0, 3.8, 0.84, "face-skin", "skin", NK(C.shapes.find((q) => q.id === "neck").d)),
  sh("collar", "neck", "body", 1, 4.5, 0.3, "jacket", "softshell", NK("M -0.075 1.535 C -0.088 1.505 -0.078 1.478 -0.04 1.468 L 0.062 1.468 C 0.090 1.478 0.098 1.51 0.086 1.540 C 0.07 1.556 0.02 1.552 -0.02 1.55 C -0.05 1.55 -0.07 1.545 -0.075 1.535 Z")),
  sh("head", "head", "body", 0, 5, 0.86, "face-skin", "skin", HD(C.shapes.find((q) => q.id === "head").d)),
  sh("helmet", "head", "body", 0, 5.5, 0.82, "helmet", "eps-helmet", HD("M -0.09 1.64 C -0.125 1.73 -0.08 1.815 0.02 1.82 C 0.09 1.82 0.125 1.77 0.115 1.715 C 0.07 1.73 -0.04 1.72 -0.09 1.64 Z")),
  line("helmet-vent-1", "head", "detail", 2, 5.6, 0.2, "helmet", "eps-helmet", xp([[-0.05, 1.77], [0.05, 1.79]], k, WA.head - 90, [0.0139, 1.599], NTOP)),
  line("helmet-strap", "head", "detail", 3, 5.6, 0.2, "helmet", "eps-helmet", xp([[-0.03, 1.66], [-0.0, 1.6], [0.04, 1.575]], k, WA.head - 90, [0.0139, 1.599], NTOP)),
  sh("hair-tail", "head", "body", 1, 5.4, 0.25, "hair", "hair", HD("M -0.085 1.66 C -0.15 1.62 -0.165 1.54 -0.14 1.47 C -0.11 1.5 -0.1 1.58 -0.075 1.625 Z")),
  ell("ear", "head", "detail", 2, 5.2, 0.8, "face-skin", "skin", HDp([-0.012, 1.645]), 0.012 * k, 0.02 * k),
  ell("eye", "head", "detail", 3, 5.7, 0.05, "face-skin", "skin", HDp([0.075, 1.667]), 0.007 * k, 0.004 * k),
  line("mouth-line", "head", "detail", 3, 5.7, 0.3, "face-skin", "skin", xp([[0.083, 1.598], [0.096, 1.598]], k, WA.head - 90, [0.0139, 1.599], NTOP)),
];
const clips = [
  {
    id: "pedal-cycle", title: tx("Pedalling at cruising speed", "Treten in Reisegeschwindigkeit"), description: tx("Four crank revolutions at 60 rpm in a 2:1 gear: each revolution turns the wheels twice and advances the ground by 4.27 m (15 km/h, treadmill frame, the bicycle stays in place). The pedals stay level, the feet follow the cranks by inverse kinematics and the trunk rocks slightly with the pedal stroke. Crank and spoke symmetry make a restart seamless.", "Vier Kurbelumdrehungen mit 60 U/min in einer Übersetzung 2:1: Jede Umdrehung dreht die Räder zweimal und lässt den Boden um 4,27 m weiterlaufen (15 km/h, Laufbandsystem, das Fahrrad bleibt stehen). Die Pedale bleiben waagerecht, die Füße folgen den Kurbeln über inverse Kinematik und der Rumpf wiegt leicht mit dem Tritt. Kurbel- und Speichensymmetrie machen den Neustart nahtlos."),
    durationS: DUR, fps: 30, loop: false, rootMotion: "none", advanceM: r4(GEAR * 2 * Math.PI * RW * NC), keys: pedalKeys,
    contacts: [{ anchor: "patch-rear", fromS: 0, toS: DUR }, { anchor: "patch-front", fromS: 0, toS: DUR }],
    phases: Array.from({ length: NC }, (_, i) => ({ id: `revolution-${i + 1}`, fromS: i, toS: i + 1, title: tx(`Crank revolution ${i + 1}`, `Kurbelumdrehung ${i + 1}`) })),
    events: Array.from({ length: NC }, (_, i) => [ev(i, "motion", "Right pedal at 4 o'clock", "Rechtes Pedal auf vier Uhr"), ev(i + 0.4, i % 2 ? "inhale" : "exhale", i % 2 ? "Breath in" : "Breath out", i % 2 ? "Einatmen" : "Ausatmen")]).flat(),
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 3.8 }, { t: DUR, v: 4 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.9 }, { t: DUR, v: 1 }] }],
  },
  {
    id: "brake-and-stop", title: tx("Braking to a halt", "Abbremsen bis zum Stillstand"), description: tx("The rider stops pedalling, brakes over 1.6 s from 15 km/h so that the wheels spin down to rest, and leans slightly forward; the cranks end up near the 4 o'clock position.", "Die fahrende Person hört auf zu treten, bremst in 1,6 s von 15 km/h bis zum Stillstand der Räder und lehnt sich leicht nach vorn; die Kurbeln stehen am Ende nahe der Vier-Uhr-Stellung."),
    durationS: 1.6, fps: 30, loop: false, rootMotion: "none", keys: dismountKeys,
    contacts: [{ anchor: "patch-rear", fromS: 0, toS: 1.6 }, { anchor: "patch-front", fromS: 0, toS: 1.6 }],
    phases: [{ id: "braking", fromS: 0, toS: 1.6, title: tx("Braking", "Bremsen") }],
    events: [ev(0, "motion", "Pedalling stops", "Treten endet"), ev(1.6, "motion", "Wheels at rest", "Räder stehen")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 4 }, { t: 1.6, v: 2 }] }, { target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 1.6, v: 0.8 }] }],
  },
  {
    id: "dismount", title: tx("Stepping down to stand astride", "Absteigen und über dem Rahmen stehen"), description: tx("From the stopped bicycle the rider slides forward off the saddle and puts both feet on the ground either side of the frame while the hands stay on the bars.", "Vom stehenden Fahrrad rutscht die Person nach vorn vom Sattel und stellt beide Füße beidseits des Rahmens auf den Boden, die Hände bleiben am Lenker."),
    durationS: 2.4, fps: 30, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "coasting", ease: "smooth" }, { t: 1.0, pose: "standing-on-pedals", ease: "smooth" },
      { t: 1.5, pose: "dismounted-astride", ease: "smooth" },
      { t: 2.4, pose: "dismounted-astride", ease: "smooth" }],
    contacts: [{ anchor: "patch-rear", fromS: 0, toS: 2.4 }, { anchor: "patch-front", fromS: 0, toS: 2.4 }, { anchor: "heel-r", fromS: 1.5, toS: 2.4 }, { anchor: "ball-r", fromS: 1.5, toS: 2.4 }, { anchor: "heel-l", fromS: 1.5, toS: 2.4 }, { anchor: "ball-l", fromS: 1.5, toS: 2.4 }],
    phases: [{ id: "rise", fromS: 0, toS: 1, title: tx("Rising off the saddle", "Aus dem Sattel steigen") }, { id: "step-down", fromS: 1, toS: 2.4, title: tx("Feet to the ground", "Füße auf den Boden") }],
    events: [ev(1.5, "contact-on", "Right foot touches the ground", "Rechter Fuß berührt den Boden", "ball-r"), ev(1.5, "contact-on", "Left foot touches the ground", "Linker Fuß berührt den Boden", "ball-l")],
    tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 2.2 }, { t: 1, v: 3 }, { t: 2.4, v: 1.8 }] }],
  },
];
export default {
  id: "cyclist",
  title: tx("Cyclist", "Radfahrende Person"),
  summary: tx("A woman in a helmet and softshell jacket on a 28 inch city bicycle, modelled as one mechanical-anatomical system: frame, wheels, crank, pedals and chain run with the rider; the feet follow the pedals, the wheels turn twice per crank revolution, and the rider can stand on the pedals or step down astride the frame.", "Eine Frau mit Helm und Softshelljacke auf einem 28-Zoll-Citybike, als ein mechanisch-anatomisches System: Rahmen, Räder, Kurbel, Pedale und Kette laufen mit der Fahrerin; die Füße folgen den Pedalen, die Räder drehen sich zweimal pro Kurbelumdrehung, und die Fahrerin kann auf den Pedalen stehen oder über dem Rahmen absteigen."),
  tags: ["human", "cyclist", "bicycle", "mechanical", "pedal-cycle", "wheels"],
  provenance: {
    author: "Semio staffage content worker humans-a", method: "handcrafted", created: "2026-10-29",
    references: [...refs(DL, WI, "drillis-contini-1966", "dubois-1916", ISO, "iso-8996-2021", "iso-7730-2005", "ashrae-2021", GU), { id: BIKE, citation: "Wilson, D. G. and Papadopoulos, J. (2004). Bicycling Science, 3rd ed., MIT Press; geometry of upright city bicycles (700C wheels, 170 mm cranks, 73 degree seat tube, saddle height from leg length)." }, { id: "iso-8090-2007", citation: "ISO 8098:2014 Cycles, safety requirements for bicycles for young children, and EN 14764 city and trekking bicycles; typical masses of 14 kg." }],
    notes: tx("A 1.72 m, 66 kg woman (de Leva female fractions plus 2.4 kg of clothing and helmet) on a 14 kg city bicycle with 700C wheels (outer radius 0.34 m), 170 mm cranks and a 2:1 gear (40 to 20 teeth). The pedal positions drive the legs by two-link inverse kinematics; the feet stay attached to the pedals. The spin offsets accumulate over the clip, so the cycling clips do not loop; with two wheel turns per crank turn and 36-spoke symmetry the restart is seamless.", "Eine 1,72 m große, 66 kg schwere Frau (weibliche Anteile nach de Leva plus 2,4 kg Kleidung und Helm) auf einem 14 kg schweren Citybike mit 700C-Rädern (Außenradius 0,34 m), 170-mm-Kurbeln und Übersetzung 2:1 (40 zu 20 Zähne). Die Pedalstellungen steuern die Beine über zweigliedrige inverse Kinematik; die Füße bleiben an den Pedalen. Die Drehversätze summieren sich über den Clip, daher laufen die Fahr-Clips nicht in Schleife; mit zwei Radumdrehungen pro Kurbelumdrehung und 36-Speichen-Symmetrie ist der Neustart nahtlos."),
  },
  dimensions: { heightM: 1.79, widthM: 1.74, depthM: 0.65, massKg: 83 },
  materials: [
    mat("skin", "Skin", "Haut", 0.98, 0.6, "literature", "ashrae-2021"), mat("hair", "Hair", "Haar", 0.98, 0.85, "authored-estimate"),
    mat("softshell", "Softshell fabric", "Softshellgewebe", 0.92, 0.7, "literature", "ashrae-2021"), mat("stretch-trouser", "Stretch trousers", "Stretchhose", 0.92, 0.75, "literature", "ashrae-2021"), mat("leather-glove", "Leather glove", "Lederhandschuh", 0.9, 0.75, "authored-estimate"),
    mat("canvas-sneaker", "Canvas sneaker", "Stoffturnschuh", 0.9, 0.5, "authored-estimate"), mat("eps-helmet", "Polystyrene helmet", "Polystyrolhelm", 0.9, 0.5, "authored-estimate"),
    mat("steel", "Painted steel and aluminium", "Lackierter Stahl und Aluminium", 0.85, 0.5, "literature", "ashrae-2021"), mat("rubber", "Rubber tyre", "Gummireifen", 0.93, 0.9, "literature", "ashrae-2021"), mat("leather-saddle", "Leather saddle", "Ledersattel", 0.9, 0.7, "authored-estimate"),
  ],
  bones, anchors, shapes, poses, clips,
  thermal: {
    environment: ENV(14, 15, 0.65, 4.3, 200), baseMet: 1.2,
    regions: [
      region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", "core-resistance", 0.085, 0.2, 0.08, { coreTempC: 36.9, insulationM2KW: 0.035, wettedness: 0.15 }, "dubois-1916"),
      region("hair", "Hair tail", "Haarzopf", "hair", "hair", "core-resistance", 0.02, 0.1, 0.01, { coreTempC: 36.9, insulationM2KW: 0.07 }, "dubois-1916", "authored-estimate"),
      region("helmet", "Helmet", "Helm", "clothing", "eps-helmet", "core-resistance", 0.06, 0.2, 0.02, { coreTempC: 36.9, insulationM2KW: 0.1 }, ISO, "authored-estimate"),
      region("gloves", "Cycling gloves", "Radhandschuhe", "clothing", "leather-glove", "core-resistance", 0.08, 0.1, 0.04, { coreTempC: 35, insulationM2KW: 0.07 }, ISO),
      region("jacket", "Softshell jacket", "Softshelljacke", "clothing", "softshell", "core-resistance", 0.82, 0.3, 0.5, { coreTempC: 36.9, insulationM2KW: 0.15, wettedness: 0.1 }, ISO),
      region("trousers", "Stretch trousers", "Stretchhose", "clothing", "stretch-trouser", "core-resistance", 0.5, 0.12, 0.29, { coreTempC: 36.9, insulationM2KW: 0.09 }, ISO),
      region("shoes", "Sneakers", "Turnschuhe", "surface", "canvas-sneaker", "core-resistance", 0.1, 0.1, 0.06, { coreTempC: 34, insulationM2KW: 0.1 }, ISO, "authored-estimate"),
      region("bike-metal", "Frame, rims and drivetrain", "Rahmen, Felgen und Antrieb", "surface", "steel", "passive", 0.9, 0.4, undefined, {}, ISO, "authored-estimate"),
      region("tyres", "Tyres", "Reifen", "tyre", "rubber", "passive", 0.6, 0.5, undefined, {}, ISO, "authored-estimate"),
      region("saddle", "Saddle", "Sattel", "surface", "leather-saddle", "passive", 0.07, 0.2, undefined, {}, ISO, "authored-estimate"),
    ],
    emitters: [{ id: "breath", kind: "breath", title: tx("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: -10, carries: "vapour", temperatureC: 34, velocityMS: 1.8, diameterM: 0.022, vapourKgPerKg: 0.0345, periodS: 2.6, dutyFrom: 0.4, dutyTo: 1, source: "literature", reference: GU }],
    assumptions: [tx("Body regions sum to the DuBois area of 1.76 m² (66 kg, 172 cm); the bicycle surfaces are additional passive areas. The relative wind of 4.3 m/s is the riding speed; a fan-like cooling of the exposed surfaces follows from the wind in the environment.", "Die Körperregionen ergeben die DuBois-Fläche von 1,76 m² (66 kg, 172 cm); die Fahrradflächen sind zusätzliche passive Flächen. Der Fahrtwind von 4,3 m/s entspricht der Fahrgeschwindigkeit."), tx("Cycling at 15 km/h is about 4 met (ISO 8996: 5.5 to 6.0 W/kg of rider mass at 16 km/h is roughly 4 met); breathing every 2.6 s.", "Radfahren mit 15 km/h entspricht etwa 4 met (ISO 8996); Atmung alle 2,6 s.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [tx("The two tyre patches carry bicycle and rider; the rider is rigidly attached to the frame through the saddle, the grips and the pedals, and pedalling forces are internal. In the dismounted pose the feet are additional ground contacts.", "Die beiden Reifenaufstandsflächen tragen Fahrrad und Fahrerin; die Fahrerin ist über Sattel, Griffe und Pedale starr mit dem Rahmen verbunden, Tretkräfte sind innere Kräfte. In der abgestiegenen Pose sind die Füße zusätzliche Bodenkontakte."), tx("The bicycle stays in place and the ground advances (treadmill frame).", "Das Fahrrad bleibt stehen und der Boden läuft vorbei (Laufbandsystem).")], clinical: false },
  presentation: { defaultPose: "riding-upright", defaultClip: "pedal-cycle", viewpoints: ["elevation"] },
  exports: STD_EXPORTS,
};
