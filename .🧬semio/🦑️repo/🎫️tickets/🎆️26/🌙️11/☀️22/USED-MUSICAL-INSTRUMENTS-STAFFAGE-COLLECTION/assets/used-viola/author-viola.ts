/** 🎻️ The one authoritative script of the used 16-inch viola in its dart case: states every hand-chosen number (through `author-geometry.ts`), solves the bow placement of the playing pose through the product's own rig chain and writes the asset document `🔣️.json` next to it.
 *
 * Run: `$BUN author-viola.ts` in this folder; prints `[DEBUG] ` lines with the measured numbers.
 *
 * @see ./author-geometry.ts — outlines, arching caps and lofts
 * @see ../../brief.md — the contract of the wave
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — Pipeline, Conventions, Front and plan views, Volume */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { restFrames, sinCosDeg, solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { BOW, CASE, VIOLA, area, capMesh, centroid, clean, dart, dartHalf, envelope, hinge, hull, loft, loftFront, loftPlan, loftSide, q, qp, samples, simplify, superellipse, topHeight, violaOutline, type P, type Section, type V } from "./author-geometry.ts";

const HERE = import.meta.dir;
const T = (en: string, de: string) => ({ en, de });
const dot = (a: V, b: V) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const sub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const add = (a: V, b: V): V => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const mul = (a: V, s: number): V => [a[0] * s, a[1] * s, a[2] * s];
const cross = (a: V, b: V): V => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V): V => mul(a, 1 / Math.hypot(...a));
const deg = (r: number) => (r * 180) / Math.PI;
const rad = (d: number) => (d * Math.PI) / 180;
const debug = (...a: unknown[]) => console.log("[DEBUG]", ...a);
const poly = (pts: P[], d = 5) => pts.map((p) => qp(p, d));

//#region 🦴️ Rig
const H = hinge();
const dartCentroid = centroid(dart(0));
const lidOrigin: V = add(H.origin, mul(H.dir, dot(sub([dartCentroid[0], CASE.parting, dartCentroid[1]], H.origin), H.dir)));
const abduction = -(90 + H.deltaDeg);
const VO = VIOLA.origin;
const LIFT_AXIS = 100;
const LIFT = 1.02;
const PLAY_TILT = 12;

/** 🧭️ The axes turned like the product's rig: angle about w, abduction about the turned v, twist about the turned u. */
function turned(ax: { u: V; v: V; w: V }, a: number, b: number, c: number) {
  const [ts, tc] = [Math.sin(rad(a)), Math.cos(rad(a))];
  const u1 = add(mul(ax.u, tc), mul(ax.v, ts)), v1 = add(mul(ax.u, -ts), mul(ax.v, tc));
  const [ps, pc] = [Math.sin(rad(b)), Math.cos(rad(b))];
  const u2 = add(mul(u1, pc), mul(ax.w, ps)), w2 = add(mul(u1, -ps), mul(ax.w, pc));
  const [ws, wc] = [Math.sin(rad(c)), Math.cos(rad(c))];
  return { u: u2, v: add(mul(v1, wc), mul(w2, -ws)), w: add(mul(v1, ws), mul(w2, wc)), u1, v1, w2 };
}

/** 🔙️ The angles (angle, abduction, twist) that turn the axes `ax` into the target axes. */
function inverse(ax: { u: V; v: V; w: V }, target: { u: V; v: V }): [number, number, number] {
  const pw = dot(target.u, ax.w), pu = dot(target.u, ax.u), pv = dot(target.u, ax.v);
  const b = Math.asin(Math.max(-1, Math.min(1, pw)));
  const a = Math.atan2(pv, pu);
  const t = turned(ax, deg(a), deg(b), 0);
  const c = Math.atan2(-dot(target.v, t.w2), dot(target.v, t.v1));
  return [deg(a), deg(b), deg(c)];
}

const bowU = unit(BOW.dir);
const bowRest = { u: bowU, v: [0, 1, 0] as V, w: cross(bowU, [0, 1, 0]) };
const bones0: any[] = [
  { id: "case", parent: null, title: T("Case base with bed and handle", "Kofferunterteil mit Bett und Griff"), x: 0, y: 0, angleDeg: 0, lengthM: CASE.length, joint: { type: "fixed" } },
  { id: "lid", parent: "case", title: T("Case lid on its fabric hinge along the long side", "Kofferdeckel am Stoffscharnier der Längsseite"), x: q(lidOrigin[0]), y: q(lidOrigin[1]), z: q(lidOrigin[2]), angleDeg: 0, abductionDeg: q(abduction, 4), lengthM: 0.27, joint: { type: "revolute", minDeg: 0, maxDeg: 100 } },
  { id: "viola-lift", parent: "case", title: T("Lift of the viola out of its bed to the shoulder", "Hub der Bratsche aus dem Bett an die Schulter"), x: VO[0], y: VO[1], angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: LIFT_AXIS, minM: 0, maxM: 1.1 } },
  { id: "viola", parent: "viola-lift", title: T("Viola from the end button to the scroll", "Bratsche vom Endknopf bis zur Schnecke"), x: 0, y: 0, angleDeg: 0, lengthM: 0.677, joint: { type: "revolute", minDeg: -10, maxDeg: 25 } },
];
const lidRestAxes = turned({ u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] }, 0, abduction, 0);
const frogLocal: V = [dot(sub(BOW.frog, lidOrigin), lidRestAxes.u), dot(sub(BOW.frog, lidOrigin), lidRestAxes.v), dot(sub(BOW.frog, lidOrigin), lidRestAxes.w)];
const bowRestAngles = inverse(lidRestAxes, bowRest);
//#endregion

//#region 🏗️ Played pose: viola at the shoulder, bow on the D string
const stringZ = { C: 0.0172, G: 0.0057, D: -0.0057, A: -0.0172 };
const yStringAt = (x: number) => VIOLA.stringBridgeY + ((VIOLA.stringNutY - VIOLA.stringBridgeY) * (x - VIOLA.bridgeX)) / (VIOLA.nutX - VIOLA.bridgeX);
const contactRest: V = [VIOLA.bridgeX + 0.035, yStringAt(VIOLA.bridgeX + 0.035) + 0.0004, stringZ.D];

function rigWith(alpha: number, bowAngles: [number, number, number]) {
  return {
    kind: "mechanical",
    root: "case",
    bones: [
      ...bones0,
      { id: "bow-reach", parent: "lid", title: T("Reach of the bow out of the lid to the strings", "Weg des Bogens aus dem Deckel an die Saiten"), x: q(frogLocal[0]), y: q(frogLocal[1]), z: q(frogLocal[2]), angleDeg: 0, lengthM: 0.01, joint: { type: "prismatic", axisDeg: q(alpha, 4), minM: 0, maxM: 1.6 } },
      { id: "bow", parent: "bow-reach", title: T("Bow turn at the frog", "Drehung des Bogens am Frosch"), x: 0, y: 0, angleDeg: q(bowAngles[0], 4), abductionDeg: q(bowAngles[1], 4), twistDeg: q(bowAngles[2], 4), lengthM: 0.01, joint: { type: "revolute", minDeg: -180, maxDeg: 180 } },
      { id: "bow-stroke", parent: "bow", title: T("Bow from the frog ferrule to the head", "Bogen von der Frosch-Zwinge bis zum Kopf"), x: 0, y: 0, angleDeg: 0, lengthM: 0.675, joint: { type: "prismatic", axisDeg: 0, minM: -0.3, maxM: 0.3 } },
    ],
  };
}

const along = (s: number, axis: number) => { const { sin, cos } = sinCosDeg(axis); return { dx: s * cos, dy: s * sin }; };
const playedBase: Record<string, any> = { lid: { angleDeg: CASE.openDeg }, "viola-lift": along(LIFT, LIFT_AXIS), viola: { angleDeg: PLAY_TILT } };
const rig0 = rigWith(0, bowRestAngles);
const f0 = solveChain(rig0 as any, playedBase, { x: 0, y: 0, angleDeg: 0 });
const vf = f0["viola"]!.frame;
const worldOf = (frame: any, p: V): V => add(frame.o as V, add(add(mul(frame.u as V, p[0]), mul(frame.v as V, p[1])), mul(frame.w as V, p[2])));
const contact = worldOf(vf, sub(contactRest, VO));
const stringDir = unit(sub(worldOf(vf, sub([VIOLA.nutX, VIOLA.stringNutY, 0], VO)), worldOf(vf, sub([VIOLA.bridgeX, VIOLA.stringBridgeY, 0], VO))));
const violaUp = vf.v as V;
const bowDir = unit(cross(stringDir, violaUp));
const bowDirOriented: V = bowDir[2] > 0 ? bowDir : mul(bowDir, -1);
const lidOpen = f0["lid"]!.frame;
const frogOpen = f0["bow-reach"]!.frame.o as V;
const hingeW = lidOpen.w as V;
const sC = (dot(sub(contact, frogOpen), hingeW)) / dot(bowDirOriented, hingeW);
const frogPlayed = sub(contact, mul(bowDirOriented, sC));
const reachVec = sub(frogPlayed, frogOpen);
const alpha = deg(Math.atan2(dot(reachVec, lidOpen.v as V), dot(reachVec, lidOpen.u as V)));
const reach = Math.hypot(...reachVec);
const rig1 = rigWith(alpha, bowRestAngles);
const playedReach = { ...playedBase, "bow-reach": along(reach, q(alpha, 4)) };
const f1 = solveChain(rig1 as any, playedReach, { x: 0, y: 0, angleDeg: 0 });
const bowParent = f1["bow"]!.frame;
const bowTarget = { u: bowDirOriented, v: unit(sub(violaUp, mul(bowDirOriented, dot(violaUp, bowDirOriented)))) };
const bowOffset = inverse({ u: bowParent.u as V, v: bowParent.v as V, w: bowParent.w as V }, bowTarget);
const RIG = rig1;
const playedBones: Record<string, any> = { ...playedReach, bow: { angleDeg: q(bowOffset[0], 3), abductionDeg: q(bowOffset[1], 3), twistDeg: q(bowOffset[2], 3) } };

const f2 = solveChain(RIG as any, playedBones, { x: 0, y: 0, angleDeg: 0 });
const bowNow = f2["bow-stroke"]!.frame;
debug("lid origin", qp(lidOrigin), "abduction", q(abduction, 3), "frog local", qp(frogLocal), "bow rest angles", bowRestAngles.map((x) => q(x, 3)));
debug("played contact", qp(contact), "s_c", q(sC, 4), "alpha", q(alpha, 3), "reach", q(reach, 4), "bow offset", bowOffset.map((x) => q(x, 3)));
debug("check frog", qp(bowNow.o as number[]), "want", qp(frogPlayed), "u", qp(bowNow.u as number[], 4), "want", qp(bowDirOriented, 4));
const hairAt = (s: number) => add(bowNow.o as V, mul(bowNow.u as V, s));
debug("hair point at s_c", qp(hairAt(sC)), "contact", qp(contact));
//#endregion

//#region 📐️ Local frames
const rest = restFrames(RIG as any);
const local = (bone: string, p: V): V => {
  const f = rest[bone]!.frame;
  const d = sub(p, f.o as V);
  return [dot(d, f.u as V), dot(d, f.v as V), dot(d, f.w as V)];
};
const lidUW = (ring: P[]): P[] => ring.map(([x, z]) => { const l = local("lid", [x, CASE.parting, z]); return [l[0], l[2]] as P; });
const lidWU = (ring: P[]): P[] => lidUW(ring).map(([u, w]) => [w, u] as P);
const vl = (x: number, y: number): P => [x - VO[0], y - VO[1]];
const vlx = (x: number) => x - VO[0];
const vly = (y: number) => y - VO[1];
const bowZ = rest["bow-stroke"]!.frame.o[2] as number;
//#endregion

//#region 🎻️ Viola geometry
const outline = (per: number, d = 0) => violaOutline(per, d);
const C0: P = [VIOLA.tailX + VIOLA.centreS, 0];
function caps(per: number, K: number) {
  const ring = outline(per);
  const top = capMesh(ring, C0, VIOLA.ribTop, VIOLA.plateEdge, VIOLA.topArch, 1, K);
  const back = capMesh(ring, C0, VIOLA.backEdge, VIOLA.plateEdge, VIOLA.backArch, -1, K);
  return { ring, top, back };
}
const toLocalMesh = (m: { vertices: V[]; triangles: [number, number, number][] }) => ({ kind: "polyhedron", vertices: m.vertices.map((v) => qp(sub(v, VO), 6)), triangles: m.triangles });
function capViews(m: { vertices: V[]; triangles: [number, number, number][] }, dir: 1 | -1, flat: number) {
  const pts = samples(m);
  const side = envelope(pts.map((v) => [v[0], v[1]] as P), 0.002, dir, flat);
  const front = envelope(pts.map((v) => [v[2], v[1]] as P), 0.002, dir, flat);
  return { side: clean(side), front: clean(front) };
}
const fbTop = (x: number) => 0.0696 + (VIOLA.nutX - x) * 0.1495;
const fbThick = (x: number) => 0.006 + ((VIOLA.nutX - x) / 0.29) * 0.003;
const fbHalf = (x: number) => 0.012 + ((VIOLA.nutX - x) / 0.29) * 0.01;
const fbSections = (xs: number[]): Section[] => xs.map((x) => ({ x, y: fbTop(x) - fbThick(x) / 2, rx: fbThick(x) / 2, ry: fbHalf(x), n: 5 }));
const neckBack = (x: number) => 0.0486 + ((VIOLA.nutX - x) / 0.132) * (0.0629 - 0.0486);
const neckSections = (detail: boolean): Section[] => {
  const xs = detail ? [0.452, 0.458, 0.466, 0.478, 0.52, 0.56, 0.602] : [0.452, 0.47, 0.53, 0.602];
  return xs.map((x) => {
    const top = fbTop(x) - fbThick(x);
    const bottom = x < 0.476 ? 0.0629 - ((0.476 - x) / 0.024) ** 1.6 * (0.0629 - 0.036) : neckBack(x);
    const half = x < 0.47 ? 0.017 : 0.012 + ((0.602 - Math.min(x, 0.602)) / 0.132) * 0.0045;
    return { x, y: (top + bottom) / 2, rx: (top - bottom) / 2, ry: half, n: 2.6 };
  });
};
const pegboxSections = (): Section[] => [
  { x: 0.6, y: 0.0562, rx: 0.0082, ry: 0.0128, n: 4 },
  { x: 0.645, y: 0.0518, rx: 0.0105, ry: 0.0136, n: 4 },
  { x: 0.684, y: 0.0465, rx: 0.0135, ry: 0.0145, n: 4 },
];
const tailSections = (): Section[] => [
  { x: 0.047, y: 0.0818, rx: 0.0024, ry: 0.0075, n: 3 },
  { x: 0.08, y: 0.0872, rx: 0.003, ry: 0.0125, n: 3 },
  { x: 0.13, y: 0.0952, rx: 0.0034, ry: 0.0178, n: 3 },
  { x: 0.18, y: 0.1012, rx: 0.0036, ry: 0.0205, n: 3 },
  { x: 0.188, y: 0.1018, rx: 0.003, ry: 0.0185, n: 3 },
];
const SCROLL = { c: [0.6985, 0.05, 0] as V, r: [0.0185, 0.022, 0.0135] as V };
const PEGS = [
  { x: 0.617, side: 1 }, { x: 0.636, side: -1 }, { x: 0.655, side: 1 }, { x: 0.672, side: -1 },
].map((p) => ({ ...p, y: 0.0565 - (p.x - 0.617) * 0.12 }));
const chinOutline = (): P[] => {
  const pts: P[] = [];
  for (let i = 0; i < 28; i++) {
    const t = (2 * Math.PI * i) / 28;
    const rx = 0.044, rz = 0.04 - 0.008 * Math.cos(t) ** 2 * (Math.sin(t) < 0 ? 1 : 0);
    pts.push([0.08 + rx * Math.cos(t), 0.062 + rz * Math.sin(t) * (Math.sin(t) > 0 ? 1.05 : 0.95)]);
  }
  return pts;
};
const bridgeFront = (): P[] => {
  const x = VIOLA.bridgeX;
  const footY = (z: number) => topHeight(x, z, outline(3));
  const pts: P[] = [];
  pts.push([-0.0245, footY(-0.0245)], [-0.0135, footY(-0.0135)], [-0.0105, 0.1035], [0.0105, 0.1035], [0.0135, footY(0.0135)], [0.0245, footY(0.0245)], [0.0215, 0.114], [0.0205, 0.1215]);
  for (let i = 0; i <= 8; i++) {
    const z = 0.0205 - (0.041 * i) / 8;
    pts.push([z, VIOLA.bridgeTop - (z / 0.0205) ** 2 * 0.004]);
  }
  pts.push([-0.0215, 0.114]);
  return pts;
};
const stringsAt = (x: number, which: "bridge" | "nut" | "tail") => {
  const spread = which === "bridge" ? 1 : which === "nut" ? 0.48 : 0.78;
  return (["C", "G", "D", "A"] as const).map((k) => ({ k, z: stringZ[k] * spread, r: { C: 0.00062, G: 0.00048, D: 0.0004, A: 0.00035 }[k] }));
};
const fHole = (side: 1 | -1): P[] => {
  const pts: P[] = [];
  const zc = side * 0.041;
  for (let i = 0; i <= 12; i++) {
    const t = i / 12;
    const x = 0.192 + t * 0.09;
    const z = zc + side * (0.012 * Math.sin(Math.PI * (t - 0.5)) - 0.004);
    pts.push([x, z + 0.0028]);
  }
  for (let i = 12; i >= 0; i--) {
    const t = i / 12;
    const x = 0.192 + t * 0.09;
    const z = zc + side * (0.012 * Math.sin(Math.PI * (t - 0.5)) - 0.004);
    pts.push([x, z - 0.0028]);
  }
  return pts;
};
//#endregion

//#region 🧰️ Shapes and solids
let ids = new Set<string>();
const shapeBase = (id: string, bone: string, role: string, z: number, filled: boolean, tone: number, extra: any = {}) => ({ id, bone, role, z, filled, tone, ...extra });
const polyShape = (id: string, bone: string, role: string, z: number, filled: boolean, tone: number, pts: P[], extra: any = {}, closed = true) => ({ ...shapeBase(id, bone, role, z, filled, tone, extra), kind: "poly", points: poly(pts), closed });
const sweepU = (u0: number, u1: number) => ({ kind: "extruded", sweep: "u", z0: q(u0), z1: q(u1) });
const sweepV = (v0: number, v1: number) => ({ kind: "extruded", sweep: "v", z0: q(v0), z1: q(v1) });
const sweepW = (z0: number, z1: number) => ({ kind: "extruded", sweep: "w", z0: q(z0), z1: q(z1) });
const solidPoly = (id: string, bone: string, role: string, z: number, tone: number, pts: P[], solid: any, extra: any = {}) => ({ ...shapeBase(id, bone, role, z, true, tone, extra), kind: "poly", points: poly(pts), closed: true, solid });
const solidPath = (id: string, bone: string, role: string, z: number, tone: number, rings: P[][], solid: any, extra: any = {}) => ({ ...shapeBase(id, bone, role, z, true, tone, extra), kind: "path", d: rings.map((r) => "M " + poly(r).map((p: number[]) => `${p[0]} ${p[1]}`).join(" L ") + " Z").join(" "), solid });
const ext = (id: string, bone: string, material: string, tone: number, plane: string, profile: P[], from: number, to: number, extra: any = {}) => ({ id, kind: "extrude", bone, material, role: "body", tone, profile: poly(profile, 6), plane, from: q(from, 6), to: q(to, 6), ...extra });
const box = (id: string, bone: string, material: string, tone: number, size: V, at: V, extra: any = {}) => ({ id, kind: "box", bone, material, role: "body", tone, size: qp(size, 6), transform: { translation: qp(at, 6) }, ...extra });
const caps3 = (id: string, bone: string, material: string, tone: number, p0: V, p1: V, r0: number, r1: number, extra: any = {}) => ({ id, kind: "capsule", bone, material, role: "body", tone, p0: qp(p0, 6), p1: qp(p1, 6), r0: q(r0, 6), r1: q(r1, 6), ...extra });
const ell = (id: string, bone: string, material: string, tone: number, radii: V, at: V, extra: any = {}) => ({ id, kind: "ellipsoid", bone, material, role: "body", tone, radii: qp(radii, 6), transform: { translation: qp(at, 6) }, ...extra });
//#endregion

//#region 🧳️ Case parts per scale
const dartN = (s: number) => (s === 50 ? [36, 20] : s === 100 ? [20, 12] : [12, 8]) as [number, number];
const dartXZ = (d: number, s: number) => dart(d, ...dartN(s));
const zx = (ring: P[]) => ring.map(([x, z]) => [z, x] as P);
const floorTaper = 0.97;
function caseSide(s: number): P[] {
  const c = centroid(dart(0));
  const x0 = c[0] - c[0] * floorTaper, x1 = c[0] + (CASE.length - c[0]) * floorTaper;
  const r = s === 50 ? 0.006 : 0.0;
  return [[x0, 0], [x1, 0], [CASE.length, CASE.floorTop], [CASE.length, CASE.parting - r], [CASE.length - r, CASE.parting], [r, CASE.parting], [0, CASE.parting - r], [0, CASE.floorTop]];
}
function caseFront(): P[] {
  const hw = CASE.wideR, hb = CASE.wideR * floorTaper;
  return [[0, -hb], [0, hb], [CASE.floorTop, hw], [CASE.parting, hw], [CASE.parting, -hw], [CASE.floorTop, -hw]];
}
const handleX = [0.335, 0.465];
const handleY = 0.074;
const handleWallZ = -dartHalf(0.4);
const handleZ = handleWallZ - 0.016;
//#endregion

//#region 🖼️ Representations
const REGIONS = ["case-cover", "case-lining", "viola-top", "viola-body", "fittings", "strings", "bow"];

function representation(s: 50 | 100 | 200) {
  const elevation: any[] = [], front: any[] = [], plan: any[] = [];
  const fine = s === 50, mid = s === 100;
  const lidRing = lidUW(dartXZ(0, s)), lidInner = lidUW(dartXZ(CASE.wall, s));
  elevation.push(solidPoly("lid-shell", "lid", "body", 1, 0.32, lidRing, sweepV(0, CASE.lidTop - CASE.parting), s === 200 ? { regions: ["case-cover", "case-lining"], material: "case-fabric" } : { region: "case-cover", material: "case-fabric" }));
  if (s !== 200) elevation.push(solidPoly("lid-lining", "lid", "body", 1.1, 0.82, lidInner, sweepV(CASE.lidTop - CASE.parting - CASE.lidSkin - CASE.liningLid, CASE.lidTop - CASE.parting - CASE.lidSkin), { region: "case-lining", material: "velour" }));
  const bowEnv = bowEnvelope(s);
  elevation.push(solidPoly("bow", "bow-stroke", "body", 1.2, 0.42, bowEnv, sweepW(bowZ - 0.0055, bowZ + 0.0055), { region: "bow", material: "pernambuco" }));
  if (s === 50) elevation.push(solidPoly("bow-frog", "bow-stroke", "detail", 1.25, 0.12, bowFrogProfile(), sweepW(bowZ - 0.0085, bowZ + 0.0085), { region: "bow", material: "ebony" }));
  violaElevation(s, elevation);
  elevation.push(solidPoly("lid-front-wall", "lid", "body", 3, 0.3, frontWall(s), sweepV(0.01, CASE.lidTop - CASE.parting - CASE.lidSkin), { region: "case-cover", material: "case-fabric" }));
  elevation.push(polyShape("case-base", "case", "body", 4, true, 0.3, caseSide(s), { region: "case-cover", material: "case-fabric" }));
  if (s !== 200) elevation.push({ ...shapeBase("case-handle", "case", "body", 4.2, true, 0.18, { region: "case-cover", material: "leather" }), kind: "capsule", x0: handleX[0] + 0.012, y0: handleY, x1: handleX[1] - 0.012, y1: handleY, r0: 0.0105, r1: 0.0105 });
  if (fine) {
    elevation.push(polyShape("case-zip", "case", "detail", 4.1, false, 0.1, [[0.006, CASE.parting - 0.003], [CASE.length - 0.006, CASE.parting - 0.003]], {}, false));
    elevation.push(polyShape("case-scuff-wide", "case", "detail", 4.3, true, 0.62, [[0.011, 0.0005], [0.06, 0.0005], [0.05, 0.016], [0.0005, 0.022], [0.0005, 0.013]]));
    elevation.push(polyShape("case-scuff-narrow", "case", "detail", 4.3, true, 0.62, [[0.745, 0.0005], [0.789, 0.0005], [0.7995, 0.013], [0.7995, 0.02], [0.755, 0.014]]));
    elevation.push(polyShape("case-sticker", "case", "detail", 4.3, true, 0.86, [[0.55, 0.03], [0.615, 0.03], [0.615, 0.066], [0.55, 0.066]]));
  }

  if (s !== 200) front.push(solidPoly("lid-lining", "lid", "body", 1.1, 0.82, lidInner, sweepV(CASE.lidTop - CASE.parting - CASE.lidSkin - CASE.liningLid, CASE.lidTop - CASE.parting - CASE.lidSkin), { region: "case-lining" }));
  front.push(solidPoly("bow", "bow-stroke", "body", 1.2, 0.42, bowEnv, sweepW(bowZ - 0.0055, bowZ + 0.0055), { region: "bow" }));
  violaFront(s, front);
  front.push(solidPoly("lid-shell", "lid", "body", 3, 0.32, lidRing, sweepV(0, CASE.lidTop - CASE.parting), { region: "case-cover" }));
  front.push(solidPoly("case-base", "case", "body", 4, 0.3, dartXZ(0, s), sweepV(0, CASE.parting), { region: "case-cover" }));
  if (s !== 200) front.push({ ...shapeBase("case-handle", "case", "body", 4.2, true, 0.18, { region: "case-cover" }), kind: "ellipse", cx: handleY, cy: q(handleZ), rx: 0.0105, ry: 0.0105, solid: sweepU(handleX[0], handleX[1]) });

  plan.push(solidPoly("case-base", "case", "body", 0.5, 0.3, dartXZ(0, s), sweepV(0, 0.05), { region: "case-cover" }));
  if (s !== 200) plan.push(solidPoly("case-handle", "case", "body", 0.55, 0.18, [[handleX[0], handleWallZ + 0.004], [handleX[1], handleWallZ + 0.004], [handleX[1], handleZ - 0.0105], [handleX[0], handleZ - 0.0105]], sweepV(handleY - 0.0105, handleY + 0.0105), { region: "case-cover" }));
  plan.push(solidPoly("case-bed", "case", "body", 0.6, 0.85, dartXZ(CASE.wall, s), sweepV(CASE.floorTop, CASE.bedTop), { region: "case-lining" }));
  if (s !== 200) plan.push(solidPoly("neck-rest", "case", "body", 0.65, 0.78, [[0.5, -0.03], [0.59, -0.03], [0.59, 0.03], [0.5, 0.03]], sweepV(CASE.bedTop, 0.05), { region: "case-lining" }));
  violaPlan(s, plan);
  plan.push(solidPoly("lid-shell", "lid", "body", 2.9, 0.3, lidRing, sweepV(0, CASE.lidTop - CASE.parting - CASE.lidSkin), { region: "case-cover" }));
  plan.push(solidPoly("lid-lining", "lid", "body", 3, 0.82, lidInner, sweepV(CASE.lidTop - CASE.parting - CASE.lidSkin - CASE.liningLid, CASE.lidTop - CASE.parting - CASE.lidSkin), { region: "case-lining" }));
  plan.push(solidPoly("bow", "bow-stroke", "body", 3.2, 0.42, bowEnv, sweepW(bowZ - 0.0055, bowZ + 0.0055), { region: "bow" }));
  if (s === 50) plan.push(solidPoly("bow-frog", "bow-stroke", "detail", 3.25, 0.12, bowFrogProfile(), sweepW(bowZ - 0.0085, bowZ + 0.0085), { region: "bow" }));
  plan.push(solidPoly("lid-skin", "lid", "body", 3.5, 0.32, lidRing, sweepV(CASE.lidTop - CASE.parting - CASE.lidSkin, CASE.lidTop - CASE.parting), { region: "case-cover" }));

  const pens = s === 50 ? { body: 0.25, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 } : s === 100 ? { body: 0.25, detail: 0.13, texture: null, glazing: null, ground: 0.35 } : { body: 0.18, detail: 0.1, texture: null, glazing: null, ground: 0.25 };
  const validRange = s === 50 ? { minDenominator: 20, maxDenominator: 75 } : s === 100 ? { minDenominator: 75, maxDenominator: 150 } : { minDenominator: 150, maxDenominator: 350 };
  return { status: "authored", denominator: s, validRange, pens, hatches: [], geometry: { elevation, front, plan }, decisions: repDecisions(s) };
}

function frontWall(s: number): P[] {
  const outer = lidUW(dartXZ(0, s)), inner = lidUW(dartXZ(CASE.wall, s));
  const umax = (w: number, ring: P[]) => Math.max(...ring.filter(([, ww]) => Math.abs(ww - w) < 0.03).map(([u]) => u), 0);
  const keep = (ring: P[]) => ring.filter(([u, w]) => u >= 0.6 * umax(w, ring));
  const o = keep(outer), i = keep(inner);
  return [...o.sort((a, b) => a[1] - b[1]), ...i.sort((a, b) => b[1] - a[1])];
}

function bowEnvelope(s: number): P[] {
  const top: P[] = [[BOW.button, 0.0255], [-0.05, 0.0252], [0, 0.0238], [0.1, 0.0198], [0.25, 0.0148], [0.4, 0.0132], [0.55, 0.0144], [0.63, 0.0172], [0.655, 0.0215], [0.672, 0.0225]];
  const bottom: P[] = [[0.675, 0.011], [0.664, 0.0008], [0.64, -0.0006], [0, -0.0006], [-0.004, -0.004], [-0.03, -0.0125], [-0.05, -0.009], [BOW.button, 0.017]];
  const ring = [...top, ...bottom];
  return s === 200 ? [[BOW.button, 0.024], [0.0, 0.022], [0.4, 0.013], [0.672, 0.02], [0.675, 0.0], [0, -0.006], [BOW.button, 0.012]] : s === 100 ? simplify(ring, 0.0015) : ring;
}
function bowFrogProfile(): P[] {
  return [[-0.052, 0.017], [0.002, 0.017], [0.004, 0.006], [0.0, -0.002], [-0.01, -0.0105], [-0.03, -0.013], [-0.046, -0.0105], [-0.052, 0.0]];
}

const capCache = new Map<number, ReturnType<typeof caps>>();
const capsAt = (s: number) => {
  if (!capCache.has(s)) capCache.set(s, s === 50 ? caps(2, 7) : s === 100 ? caps(1, 4) : caps(1, 2));
  return capCache.get(s)!;
};

function violaElevation(s: number, out: any[]) {
  const fine = s === 50, coarse = s === 200;
  const { top, back } = capsAt(s);
  const tv = capViews(top, 1, VIOLA.ribTop), bv = capViews(back, -1, VIOLA.backEdge);
  const vv = (pts: P[]) => pts.map(([x, y]) => vl(x, y));
  out.push(polyShape("viola-back", "viola", "body", 2, true, 0.42, vv(fine ? bv.side : simplify(bv.side, 0.001)), { region: "viola-body", material: "maple" }));
  const ribX0 = VIOLA.tailX + VIOLA.overhang, ribX1 = VIOLA.tailX + VIOLA.body - VIOLA.overhang;
  out.push(polyShape("viola-ribs", "viola", "body", 2.05, true, 0.4, vv([[ribX0, VIOLA.backEdge], [ribX1, VIOLA.backEdge], [ribX1, VIOLA.ribTop], [ribX0, VIOLA.ribTop]]), { region: "viola-body", material: "maple" }));
  out.push(polyShape("viola-top", "viola", "body", 2.1, true, 0.66, vv(fine ? tv.side : simplify(tv.side, 0.001)), { region: "viola-top", material: "spruce" }));
  const neck = neckSections(fine);
  if (coarse) {
    out.push(polyShape("viola-neck", "viola", "body", 2.2, true, 0.4, vv(clean([...loftSide(neck).slice(0, neck.length), ...loftSide(pegboxSections()).slice(0, 3), [0.717, 0.05], ...loftSide(pegboxSections()).slice(3), ...loftSide(neck).slice(neck.length)])), { region: "viola-body", material: "maple" }));
  } else {
    out.push(polyShape("viola-neck", "viola", "body", 2.2, true, 0.4, vv(loftSide(neck)), { region: "viola-body", material: "maple" }));
    out.push(polyShape("viola-pegbox", "viola", "body", 2.25, true, 0.4, vv(loftSide(pegboxSections())), { region: "viola-body", material: "maple" }));
    out.push({ ...shapeBase("viola-scroll", "viola", "body", 2.3, true, 0.38, { region: "viola-body", material: "maple" }), kind: "ellipse", cx: q(vlx(SCROLL.c[0])), cy: q(vly(SCROLL.c[1])), rx: SCROLL.r[0], ry: SCROLL.r[1] });
  }
  if (fine) {
    out.push(polyShape("scroll-volute", "viola", "detail", 2.32, false, 0.2, vv(spiral()), {}, false));
    PEGS.filter((p) => p.side < 0).forEach((p, i) => out.push({ ...shapeBase(`peg-${i + 1}`, "viola", "body", 2.35, true, 0.12, { region: "fittings", material: "ebony" }), kind: "ellipse", cx: q(vlx(p.x)), cy: q(vly(p.y)), rx: 0.00375, ry: 0.012 }));
    out.push({ ...shapeBase("end-button", "viola", "body", 2.15, true, 0.12, { region: "fittings", material: "ebony" }), kind: "rect", x: q(vlx(0.04)), y: q(vly(0.0503)), w: 0.007, h: 0.0124 });
  }
  const fb = fbSections(fine ? [0.312, 0.4, 0.5, 0.607] : [0.312, 0.607]);
  out.push(polyShape("fingerboard", "viola", "body", 2.5, true, 0.1, vv(loftSide(fb)), coarse || s === 100 ? { regions: ["fittings", "strings"], material: "ebony" } : { region: "fittings", material: "ebony" }));
  if (!coarse) {
    out.push(polyShape("tailpiece", "viola", "body", 2.45, true, 0.12, vv(loftSide(tailSections())), { region: "fittings", material: "ebony" }));
    out.push(polyShape("chinrest", "viola", "body", 2.6, true, 0.16, vv([[0.036, 0.0795], [0.124, 0.0815], [0.124, 0.098], [0.11, 0.1038], [0.06, 0.1042], [0.04, 0.1005], [0.036, 0.094]]), { region: "fittings", material: "ebony" }));
    const bx = VIOLA.bridgeX, foot = topHeight(bx, 0.0245, outline(3)) - 0.0015;
    out.push(polyShape("bridge", "viola", "body", 2.55, true, 0.7, vv([[bx - 0.002, foot], [bx + 0.002, foot], [bx + 0.0008, VIOLA.bridgeTop], [bx - 0.0008, VIOLA.bridgeTop]]), { region: "viola-body", material: "maple" }));
  }
  if (fine) {
    out.push(polyShape("strings", "viola", "detail", 2.7, false, 0.05, vv([[0.183, 0.1035], [VIOLA.bridgeX, VIOLA.stringBridgeY], [VIOLA.nutX, VIOLA.stringNutY], [0.614, 0.062]]), { region: "strings", material: "string-steel" }, false));
  }
}

function spiral(): P[] {
  const pts: P[] = [];
  for (let i = 0; i <= 24; i++) {
    const t = (i / 24) * 2.2 * Math.PI;
    const r = 0.017 * (1 - i / 30);
    pts.push([SCROLL.c[0] + r * Math.cos(t), SCROLL.c[1] + r * Math.sin(t) * 1.15]);
  }
  return pts;
}

function violaFront(s: number, out: any[]) {
  const fine = s === 50, coarse = s === 200;
  const { top, back } = capsAt(s);
  const tv = capViews(top, 1, VIOLA.ribTop), bv = capViews(back, -1, VIOLA.backEdge);
  const vw = (pts: P[]) => pts.map(([z, y]) => [vly(y), z] as P);
  const u0 = vlx(VIOLA.tailX), u1 = vlx(VIOLA.tailX + VIOLA.body);
  const hw = 0.1175 - VIOLA.overhang;
  out.push(solidPoly("viola-back", "viola", "body", 2, 0.42, vw(fine ? bv.front : simplify(bv.front, 0.001)), sweepU(u0, u1), { region: "viola-body" }));
  out.push(solidPoly("viola-ribs", "viola", "body", 2.05, 0.4, vw([[-hw, VIOLA.backEdge], [hw, VIOLA.backEdge], [hw, VIOLA.ribTop], [-hw, VIOLA.ribTop]]), sweepU(u0 + VIOLA.overhang, u1 - VIOLA.overhang), { region: "viola-body" }));
  out.push(solidPoly("viola-top", "viola", "body", 2.1, 0.66, vw(fine ? tv.front : simplify(tv.front, 0.001)), sweepU(u0, u1), { region: "viola-top" }));
  if (!coarse) {
    out.push(solidPoly("tailpiece", "viola", "body", 2.45, 0.12, vw(loftFront(tailSections())), sweepU(vlx(0.047), vlx(0.188)), { region: "fittings" }));
    out.push(solidPoly("chinrest", "viola", "body", 2.6, 0.16, vw([[0.02, 0.08], [0.104, 0.082], [0.106, 0.098], [0.095, 0.1042], [0.03, 0.1042], [0.02, 0.098]]), sweepU(vlx(0.036), vlx(0.124)), { region: "fittings" }));
    out.push(solidPoly("bridge", "viola", "body", 2.55, 0.7, vw(bridgeFront()), sweepU(vlx(VIOLA.bridgeX - 0.002), vlx(VIOLA.bridgeX + 0.002)), { region: "viola-body" }));
  }
  const fb = fbSections(fine ? [0.312, 0.4, 0.5, 0.607] : [0.312, 0.607]);
  out.push(solidPoly("fingerboard", "viola", "body", 2.5, 0.1, vw(loftFront(fb)), sweepU(vlx(0.312), vlx(0.607)), { region: "fittings" }));
  const neck = neckSections(fine);
  out.push(solidPoly("viola-neck", "viola", "body", 2.2, 0.4, vw(loftFront(coarse ? [...neck, ...pegboxSections()] : neck)), sweepU(vlx(0.452), vlx(coarse ? 0.717 : 0.602)), { region: "viola-body" }));
  if (!coarse) {
    out.push(solidPoly("viola-pegbox", "viola", "body", 2.25, 0.4, vw(loftFront(pegboxSections())), sweepU(vlx(0.6), vlx(0.684)), { region: "viola-body" }));
    out.push({ ...shapeBase("viola-scroll", "viola", "body", 2.3, true, 0.38, { region: "viola-body" }), kind: "ellipse", cx: q(vly(SCROLL.c[1])), cy: 0, rx: SCROLL.r[1], ry: SCROLL.r[2], solid: sweepU(vlx(SCROLL.c[0] - SCROLL.r[0]), vlx(SCROLL.c[0] + SCROLL.r[0])) });
  }
  if (fine) {
    PEGS.forEach((p, i) => out.push({ ...shapeBase(`peg-${i + 1}`, "viola", "body", 2.35, true, 0.12, { region: "fittings" }), kind: "rect", x: q(vly(p.y) - 0.012), y: q(p.side > 0 ? 0.0135 : -0.052), w: 0.024, h: 0.0385, solid: sweepU(vlx(p.x - 0.00375), vlx(p.x + 0.00375)) }));
  }
}

function violaPlan(s: number, out: any[]) {
  const fine = s === 50, coarse = s === 200;
  const ring = s === 50 ? outline(3) : s === 100 ? simplify(outline(2), 0.0012) : simplify(outline(1), 0.004);
  const uw = (pts: P[]) => pts.map(([x, z]) => [vlx(x), z] as P);
  const wz = (pts: P[]) => pts;
  out.push(solidPoly("viola-top", "viola", "body", 1, 0.66, uw(ring), sweepV(vly(VIOLA.ribTop), vly(VIOLA.ribTop + VIOLA.plateEdge + VIOLA.topArch)), { region: "viola-top" }));
  const neck = neckSections(fine);
  out.push(solidPoly("viola-neck", "viola", "body", 1.1, 0.4, uw(loftPlan(coarse ? [...neck, ...pegboxSections(), { x: 0.717, y: 0.05, rx: 0.02, ry: 0.0115, n: 2 }] : neck)), sweepV(vly(0.036), vly(0.083)), { region: "viola-body" }));
  if (!coarse) {
    out.push(solidPoly("viola-pegbox", "viola", "body", 1.15, 0.4, uw(loftPlan(pegboxSections())), sweepV(vly(0.033), vly(0.065)), { region: "viola-body" }));
    out.push({ ...shapeBase("viola-scroll", "viola", "body", 1.2, true, 0.38, { region: "viola-body" }), kind: "ellipse", cx: q(vlx(SCROLL.c[0])), cy: 0, rx: SCROLL.r[0], ry: SCROLL.r[2], solid: sweepV(vly(SCROLL.c[1] - SCROLL.r[1]), vly(SCROLL.c[1] + SCROLL.r[1])) });
  }
  if (fine) {
    PEGS.forEach((p, i) => out.push(solidPoly(`peg-${i + 1}`, "viola", "body", 1.25, 0.12, uw(p.side > 0 ? [[p.x - 0.004, -0.013], [p.x + 0.004, -0.013], [p.x + 0.004, 0.028], [p.x + 0.012, 0.03], [p.x + 0.012, 0.052], [p.x - 0.012, 0.052], [p.x - 0.012, 0.03], [p.x - 0.004, 0.028]] : [[p.x - 0.004, 0.013], [p.x - 0.004, -0.028], [p.x - 0.012, -0.03], [p.x - 0.012, -0.052], [p.x + 0.012, -0.052], [p.x + 0.012, -0.03], [p.x + 0.004, -0.028], [p.x + 0.004, 0.013]]), sweepV(vly(p.y - 0.012), vly(p.y + 0.012)), { region: "fittings" })));
    out.push(polyShape("purfling", "viola", "detail", 1.02, false, 0.25, wz(outline(3, 0.0045)), {}, true));
    out.push(polyShape("f-hole-bass", "viola", "detail", 1.05, true, 0.05, wz(fHole(1)), { penMm: 0.2 }));
    out.push(polyShape("f-hole-treble", "viola", "detail", 1.05, true, 0.05, wz(fHole(-1)), { penMm: 0.2 }));
    out.push(polyShape("crack", "viola", "detail", 1.06, false, 0.08, wz([[0.104, -0.1135], [0.13, -0.1095], [0.158, -0.1065], [0.186, -0.1045]]), {}, false));
    [0.112, 0.136, 0.16].forEach((x, i) => out.push(polyShape(`cleat-${i + 1}`, "viola", "texture", 1.07, false, 0.3, wz([[x - 0.005, -0.1155], [x + 0.005, -0.1155], [x + 0.005, -0.1015], [x - 0.005, -0.1015]]))));
    out.push(polyShape("rosin-dust", "viola", "detail", 1.08, true, 0.92, wz([[0.243, -0.032], [0.3, -0.036], [0.304, 0.034], [0.246, 0.03]]), { penMm: 0.2 }));
    out.push(polyShape("varnish-wear-shoulder", "viola", "detail", 1.08, true, 0.8, wz([[0.355, 0.068], [0.39, 0.058], [0.408, 0.047], [0.414, 0.06], [0.396, 0.078], [0.368, 0.086]]), { penMm: 0.2 }));
  }
  const fb = fbSections(fine ? [0.312, 0.4, 0.5, 0.607] : [0.312, 0.607]);
  out.push(solidPoly("fingerboard", "viola", "body", 1.3, 0.1, uw(loftPlan(fb)), sweepV(vly(0.086), vly(0.113)), { region: "fittings" }));
  if (!coarse) {
    out.push(solidPoly("tailpiece", "viola", "body", 1.3, 0.12, uw(loftPlan(tailSections())), sweepV(vly(0.079), vly(0.105)), { region: "fittings" }));
    out.push(solidPoly("chinrest", "viola", "body", 1.4, 0.16, uw(fine ? chinOutline() : simplify(chinOutline(), 0.002)), sweepV(vly(0.079), vly(0.1042)), { region: "fittings" }));
    out.push(solidPoly("bridge", "viola", "body", 1.35, 0.7, uw([[VIOLA.bridgeX - 0.002, -0.0245], [VIOLA.bridgeX + 0.002, -0.0245], [VIOLA.bridgeX + 0.002, 0.0245], [VIOLA.bridgeX - 0.002, 0.0245]]), sweepV(vly(0.092), vly(VIOLA.bridgeTop)), { region: "viola-body" }));
  }
  if (fine) {
    const b = stringsAt(0, "bridge"), n = stringsAt(0, "nut"), t = stringsAt(0, "tail");
    b.forEach((st, i) => out.push(polyShape(`string-${st.k.toLowerCase()}`, "viola", "detail", 1.5, false, 0.05, wz([[0.183, t[i]!.z], [VIOLA.bridgeX, st.z], [VIOLA.nutX, n[i]!.z]]), { region: "strings" }, false)));
  }
}
//#endregion

//#region 🧊️ Volumes
const MAT = {
  fabric: "case-fabric", shell: "case-shell", velour: "velour", leather: "leather", spruce: "spruce", maple: "maple", ebony: "ebony", steel: "string-steel", pern: "pernambuco", hair: "horsehair", rosin: "rosin",
};

function volume(s: 50 | 100 | 200) {
  const fine = s === 50, mid = s === 100, coarse = s === 200;
  const body: any[] = [], thermal: any[] = [], structure: any[] = [];
  const region: Record<string, string> = {};
  const add = (solid: any, reg?: string) => { body.push(solid); if (reg) region[solid.id] = reg; };
  const outer = dartXZ(0, s), inner = dartXZ(CASE.wall, s);
  add(ext("base-floor", "case", MAT.fabric, 0.3, "zx", zx(outer), 0, CASE.floorTop, { taperTo: [floorTaper, 1], ...(fine ? {} : { sheet: true }) }), "case-cover");
  add(ext("base-wall", "case", MAT.fabric, 0.3, "zx", zx(outer), CASE.floorTop - 0.0005, CASE.parting, { holes: [poly(zx(inner), 6)] }), "case-cover");
  add(ext("bed", "case", MAT.velour, 0.85, "zx", zx(dartXZ(CASE.wall + 0.0005, s)), CASE.floorTop - 0.0005, CASE.bedTop, fine ? {} : { sheet: true }), "case-lining");
  if (!coarse) {
    add(box("neck-rest", "case", MAT.velour, 0.78, [0.09, 0.05 - CASE.bedTop + 0.001, 0.06], [0.545, (0.05 + CASE.bedTop - 0.001) / 2, 0]), "case-lining");
    add(caps3("handle-grip", "case", MAT.leather, 0.18, [handleX[0] + 0.012, handleY, handleZ], [handleX[1] - 0.012, handleY, handleZ], 0.0105, 0.0105), "case-cover");
  }
  if (fine) {
    add(box("scroll-rest", "case", MAT.velour, 0.78, [0.04, SCROLL.c[1] - SCROLL.r[1] - CASE.bedTop + 0.0015, 0.03], [0.6985, (SCROLL.c[1] - SCROLL.r[1] + CASE.bedTop + 0.0005) / 2, 0]), "case-lining");
    for (const [k, x] of [["a", handleX[0] + 0.008], ["b", handleX[1] - 0.008]] as const) add(box(`handle-tab-${k}`, "case", MAT.leather, 0.18, [0.018, 0.022, handleWallZ - handleZ + 0.004], [x, handleY, (handleWallZ + handleZ) / 2 + 0.002]), "case-cover");
    const scuff = (id: string, keep: (p: P) => boolean, h: number) => {
      const o = dart(-0.0006, 72, 40).filter(keep), i = dart(0.0012, 72, 40).filter(keep);
      body.push({ ...ext(id, "case", MAT.shell, 0.62, "zx", zx([...o, ...i.reverse()]), 0.004, h), texture: true });
    };
    scuff("case-scuff-wide", (p) => p[0] < 0.045, 0.022);
    scuff("case-scuff-narrow", (p) => p[0] > 0.772, 0.02);
    body.push({ ...box("case-sticker", "case", MAT.rosin, 0.86, [0.065, 0.036, 0.0008], [0.5825, 0.048, -dartHalf(0.5825) - 0.0003]), texture: true });
  }
  const lidOuter = lidWU(outer), lidInner = lidWU(inner);
  const lidH = CASE.lidTop - CASE.parting;
  add(ext("lid-wall", "lid", MAT.fabric, 0.3, "zx", lidOuter, 0, lidH - CASE.lidSkin + 0.0005, { holes: [poly(lidInner, 6)] }), "case-cover");
  add(ext("lid-skin", "lid", MAT.fabric, 0.32, "zx", lidOuter, lidH - CASE.lidSkin, lidH, { taperTo: [1, 0.97], ...(fine ? {} : { sheet: true }) }), "case-cover");
  if (!coarse) add(ext("lid-lining", "lid", MAT.velour, 0.82, "zx", lidWU(dartXZ(CASE.wall + 0.0005, s)), lidH - CASE.lidSkin - CASE.liningLid, lidH - CASE.lidSkin + 0.0005, fine ? {} : { sheet: true }), "case-lining");
  const hingeLen = 0.58;
  if (!coarse) body.push({ ...caps3("lid-hinge", "lid", MAT.fabric, 0.25, [0, 0.001, -hingeLen / 2], [0, 0.001, hingeLen / 2], 0.004, 0.004), slender: true });
  if (fine) {
    region["lid-hinge"] = "case-cover";
  }
  const holder = (idx: string, sAlong: number) => {
    const p = add3(BOW.frog, mul(bowRest.u, sAlong));
    const stickY = p[1] + (sAlong < 0.3 ? 0.0238 : 0.0172) - (fine ? 0.002 : 0.008);
    const r = fine ? 0.006 : mid ? 0.0075 : 0.015;
    const a = local("lid", [p[0], stickY, p[2]]), b = local("lid", [p[0], CASE.lidTop - 0.004 - (coarse ? 0.012 : 0), p[2]]);
    return caps3(`bow-holder-${idx}`, "lid", MAT.ebony, 0.2, a, b, r, r);
  };
  add(holder("frog", 0.03), "case-lining");
  add(holder("tip", 0.6), "case-lining");
  const stickSections = (fine ? [BOW.button, -0.05, 0, 0.1, 0.25, 0.4, 0.55, 0.63, 0.655] : mid ? [BOW.button, 0, 0.25, 0.45, 0.655] : [BOW.button, 0.2, 0.45, 0.66]).map((u) => {
    const cv = u < -0.04 ? 0.0213 : u < 0.05 ? 0.0201 - u * 0.03 : u < 0.4 ? 0.0186 - (u - 0.05) * 0.024 : u < 0.6 ? 0.0102 + (u - 0.4) * 0.012 : 0.0126 + (u - 0.6) * 0.06;
    const r = 0.0042 - Math.max(0, u) * 0.0024;
    return { x: u, y: cv, rx: r, ry: r, n: 2 };
  });
  if (!fine) {
    add({ ...loft(stickSections.map((x) => ({ ...x, rx: x.rx + 0.003, y: x.y - 0.003 })), [0, 0, 0]), id: "bow-stick", bone: "bow-stroke", material: MAT.pern, role: "body", tone: 0.42, slender: true }, "bow");
  } else {
    add({ ...loft(stickSections, [0, 0, 0]), id: "bow-stick", bone: "bow-stroke", material: MAT.pern, role: "body", tone: 0.42, slender: true }, "bow");
    add({ ...box("bow-hair", "bow-stroke", MAT.hair, 0.9, [BOW.hair, 0.0012, 0.011], [BOW.hair / 2, 0, 0]), slender: true }, "bow");
  }
  if (!coarse) add(ext("bow-frog", "bow-stroke", MAT.ebony, 0.12, "xy", bowFrogProfile(), -0.0085, 0.0085), "bow");
  if (fine) add(ext("bow-head", "bow-stroke", MAT.pern, 0.4, "xy", [[0.638, 0.0005], [0.664, 0.001], [0.674, 0.0115], [0.669, 0.0225], [0.652, 0.0205], [0.64, 0.0125]], -0.0045, 0.0045), "bow");
  violaVolume(s, add, body, region);
  for (const solid of body) {
    if (solid.texture || !region[solid.id]) continue;
    const faces: Record<string, string[]> = { "base-floor": ["+v"], bed: ["-v"], "lid-lining": ["+v"], "lid-skin": ["-v"] };
    thermal.push({ id: `th-${solid.id}`, of: solid.id, region: region[solid.id], ...(faces[solid.id] ? { sectionFaces: faces[solid.id] } : {}) });
  }
  if (!fine) {
    const n = stringsAt(0, "nut");
    stringsAt(0, "bridge").forEach((st, i) => {
      if (coarse && i % 3 !== 0) return;
      thermal.push({ ...caps3(`th-string-${st.k.toLowerCase()}`, "viola", MAT.steel, 0.6, [vlx(VIOLA.bridgeX), vly(VIOLA.stringBridgeY - 0.0004), st.z], [vlx(VIOLA.nutX), vly(VIOLA.stringNutY - 0.0004), n[i]!.z], coarse ? 0.0009 : st.r, coarse ? 0.0009 : st.r), slender: true, region: "strings" });
    });
  }
  structureList(s, structure, body);
  return { status: "authored", denominator: s, body, thermal, structure, decisions: volumeDecisions(s) };
}

const add3 = (a: V, b: V): V => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];

function violaVolume(s: number, add: (solid: any, reg?: string) => void, body: any[], region: Record<string, string>) {
  const fine = s === 50, coarse = s === 200;
  const { ring, top, back } = capsAt(s);
  add({ id: "back-cap", bone: "viola", material: MAT.maple, role: "body", tone: 0.42, ...toLocalMesh(back) }, "viola-body");
  const ribOuter = coarse ? simplify(violaOutline(1, VIOLA.overhang), 0.002) : violaOutline(fine ? 2 : 1, VIOLA.overhang);
  const ribInner = coarse ? simplify(violaOutline(1, VIOLA.overhang + 0.006), 0.002) : violaOutline(fine ? 2 : 1, VIOLA.overhang + VIOLA.ribWall);
  const lw = (pts: P[]) => pts.map(([x, z]) => [z, vlx(x)] as P);
  add(ext("rib-ring", "viola", MAT.maple, 0.4, "zx", lw(ribOuter), vly(VIOLA.backEdge) - 0.0003, vly(VIOLA.ribTop) + 0.0003, { holes: [poly(lw(ribInner), 6)] }), "viola-body");
  add({ id: "top-cap", bone: "viola", material: MAT.spruce, role: "body", tone: 0.66, ...toLocalMesh(top) }, "viola-top");
  const neck = neckSections(fine);
  if (coarse) {
    add({ ...loft([...neck, ...pegboxSections(), { x: 0.712, y: 0.05, rx: 0.02, ry: 0.0115, n: 2 }], VO), id: "neck-scroll", bone: "viola", material: MAT.maple, role: "body", tone: 0.4 }, "viola-body");
  } else {
    add({ ...loft(neck, VO), id: "neck", bone: "viola", material: MAT.maple, role: "body", tone: 0.4 }, "viola-body");
    add({ ...loft(pegboxSections(), VO), id: "pegbox", bone: "viola", material: MAT.maple, role: "body", tone: 0.4 }, "viola-body");
    add(ell("scroll", "viola", MAT.maple, 0.38, SCROLL.r, sub(SCROLL.c, VO)), "viola-body");
  }
  add({ ...loft(fbSections(fine ? [0.312, 0.4, 0.5, 0.607] : [0.312, 0.607]), VO), id: "fingerboard", bone: "viola", material: MAT.ebony, role: "body", tone: 0.1 }, "fittings");
  if (!coarse) {
    add({ ...loft(tailSections(), VO), id: "tailpiece", bone: "viola", material: MAT.ebony, role: "body", tone: 0.12 }, "fittings");
    const chin = fine ? chinOutline() : simplify(chinOutline(), 0.002);
    add(ext("chinrest", "viola", MAT.ebony, 0.16, "zx", lw(chin), vly(0.0785), vly(0.1042), { taperTo: [1, 0.93] }), "fittings");
    const bf = bridgeFront().map(([z, y]) => [vly(y), z] as P);
    add(ext("bridge", "viola", MAT.maple, 0.7, "yz", bf, vlx(VIOLA.bridgeX - 0.002), vlx(VIOLA.bridgeX + 0.002)), "viola-body");
  }
  if (fine) {
    add({ id: "end-button", kind: "revolve", bone: "viola", material: MAT.ebony, role: "body", tone: 0.12, profile: [[0, 0], [0, 0.0062], [0.006, 0.0062], [0.0065, 0.0045], [0.011, 0.0045], [0.011, 0]], axis: [1, 0, 0], origin: [0, q(vly(0.0565), 6), 0] }, "fittings");
    const heads = (sideId: string, list: typeof PEGS) => {
      const a = list[0]!, b = list[1]!;
      const zc = a.side * 0.04;
      add({ ...ell(`peg-heads-${sideId}`, "viola", MAT.ebony, 0.12, [0.00375, 0.012, 0.012], [vlx(a.x), vly(a.y), zc]), repeat: { count: 2, translation: qp([b.x - a.x, b.y - a.y, 0], 6) } }, "fittings");
      add({ ...caps3(`peg-shafts-${sideId}`, "viola", MAT.ebony, 0.12, [vlx(a.x), vly(a.y), a.side * 0.03], [vlx(a.x), vly(a.y), -a.side * 0.0125], 0.0042, 0.0035), repeat: { count: 2, translation: qp([b.x - a.x, b.y - a.y, 0], 6) } }, "fittings");
    };
    heads("bass", PEGS.filter((p) => p.side > 0));
    heads("treble", PEGS.filter((p) => p.side < 0));
    const t = stringsAt(0, "tail"), b = stringsAt(0, "bridge"), n = stringsAt(0, "nut");
    b.forEach((st, i) => {
      add({ ...caps3(`string-${st.k.toLowerCase()}-after`, "viola", MAT.steel, 0.6, [vlx(0.183), vly(0.1035), t[i]!.z], [vlx(VIOLA.bridgeX), vly(VIOLA.stringBridgeY - 0.0004), st.z], st.r, st.r), slender: true }, "strings");
      add({ ...caps3(`string-${st.k.toLowerCase()}`, "viola", MAT.steel, 0.6, [vlx(VIOLA.bridgeX), vly(VIOLA.stringBridgeY - 0.0004), st.z], [vlx(VIOLA.nutX + 0.002), vly(VIOLA.stringNutY - 0.0006), n[i]!.z], st.r, st.r), slender: true }, "strings");
    });
    const onTop = (x: number, z: number) => topHeight(x, z, ring);
    const tex = (id: string, mat: string, tone: number, pts: P[], thick: number) => {
      const ys = pts.map(([x, z]) => onTop(x, z));
      body.push({ ...ext(id, "viola", mat, tone, "zx", lw(pts), vly(Math.min(...ys)) - 0.0005, vly(Math.max(...ys)) + thick), texture: true });
    };
    tex("f-hole-bass", MAT.ebony, 0.05, fHole(1), 0.0006);
    tex("f-hole-treble", MAT.ebony, 0.05, fHole(-1), 0.0006);
    tex("crack-retouch", MAT.rosin, 0.3, [[0.104, -0.1145], [0.186, -0.1055], [0.186, -0.1035], [0.104, -0.1125]], 0.0004);
    tex("rosin-dust", MAT.rosin, 0.92, [[0.243, -0.032], [0.3, -0.036], [0.304, 0.034], [0.246, 0.03]], 0.0004);
    tex("varnish-wear-shoulder", MAT.spruce, 0.8, [[0.355, 0.068], [0.39, 0.058], [0.408, 0.047], [0.414, 0.06], [0.396, 0.078], [0.368, 0.086]], 0.0004);
    [0.112, 0.136, 0.16].forEach((x, i) => body.push({ ...box(`cleat-${i + 1}`, "viola", MAT.spruce, 0.6, [0.01, 0.0025, 0.014], [vlx(x), vly(VIOLA.ribTop) - 0.00115, -0.1085]), texture: true }));
  }
}

function structureList(s: number, out: any[], body: any[]) {
  const fine = s === 50, coarse = s === 200;
  const has = (id: string) => body.some((b) => b.id === id);
  const ofm = (id: string, member: string) => ({ id: `st-${id}`, of: id, member, part: "member" });
  out.push(ofm("base-floor", "case"), ofm("base-wall", "case"), ofm("bed", "case"));
  out.push({ ...ell("st-mass-case", "case", MAT.shell, 0.6, [0.03, coarse ? 0.016 : 0.012, 0.03], [0.36, 0.04, 0]), role: "detail", member: "case", part: "mass" });
  out.push(ofm("lid-wall", "lid"), ofm("lid-skin", "lid"));
  out.push({ ...caps3("st-hinge", "lid", MAT.fabric, 0.25, [0, 0.004, -0.29], [0, 0.004, 0.29], 0.006, 0.006), member: "lid", part: "joint", slender: true });
  out.push({ ...ell("st-mass-lid", "lid", MAT.shell, 0.6, [0.03, coarse ? 0.016 : 0.012, 0.03], [0.115, 0.035, 0]), role: "detail", member: "lid", part: "mass" });
  out.push(ofm("bow-stick", "bow-stroke"));
  for (const id of ["bow-hair", "bow-frog", "bow-head"]) if (has(id)) out.push({ ...ofm(id, "bow-stroke"), part: id === "bow-hair" ? "member" : "joint" });
  out.push({ id: "st-bow-holder-frog", of: "bow-holder-frog", member: "lid", part: "joint" }, { id: "st-bow-holder-tip", of: "bow-holder-tip", member: "lid", part: "joint" });
  out.push({ ...ell("st-mass-bow", "bow-stroke", MAT.pern, 0.6, [coarse ? 0.03 : 0.015, 0.006, 0.006].map((x) => Math.max(x, coarse ? 0.03 : 0.0075)) as V, [0.26, 0.012, 0]), role: "detail", member: "bow-stroke", part: "mass" });
  out.push(ofm("back-cap", "viola"), ofm("rib-ring", "viola"), ofm("top-cap", "viola"));
  out.push(ofm(coarse ? "neck-scroll" : "neck", "viola"));
  if (has("bridge")) out.push(ofm("bridge", "viola"));
  out.push({ ...ell("st-mass-viola", "viola", MAT.maple, 0.6, [0.02, 0.012, 0.02].map((x) => Math.max(x, coarse ? 0.03 : s === 100 ? 0.015 : 0.0075)) as V, [0.2, 0.004, 0]), role: "detail", member: "viola", part: "mass" });
}
//#endregion

//#region 📝️ Decisions
function repDecisions(s: number) {
  if (s === 50)
    return {
      silhouette: T("Side elevation 16 mm by 7.2 mm on paper: the dart case lying on its floor with the lid opened to 95 degrees on its fabric hinge behind it, its velour ceiling facing the viewer with the bow clipped in it, and in front of the base rim the viola on its back: bridge, fingerboard, chinrest, tailpiece and scroll show above the base wall, the ribs and arched plates hide behind it. Front view: the 0.27 m wide case section with the lid standing at its left side and the bridge, fingerboard and chinrest above the rim. Plan: the dart with the viola lying in its bed: outline with purfling, f-holes, bridge, fingerboard, tailpiece, chinrest, neck, pegbox with four pegs and scroll; the lid is a narrow band beyond the hinge.", "Seitenansicht 16 mm mal 7,2 mm auf Papier: der Formkoffer liegt auf dem Boden, der Deckel steht am Stoffscharnier um 95 Grad geöffnet dahinter, seine Velourdecke mit dem eingeklemmten Bogen zeigt zum Betrachter, und vor dem Unterteilrand liegt die Bratsche auf dem Rücken: Steg, Griffbrett, Kinnhalter, Saitenhalter und Schnecke ragen über die Wand des Unterteils, Zargen und gewölbte Platten liegen dahinter. Vorderansicht: der 0,27 m breite Kofferquerschnitt mit dem Deckel an seiner linken Seite und Steg, Griffbrett und Kinnhalter über dem Rand. Draufsicht: die Pfeilform mit der Bratsche im Bett: Umriss mit Einlage, F-Löchern, Steg, Griffbrett, Saitenhalter, Kinnhalter, Hals, Wirbelkasten mit vier Wirbeln und Schnecke; der Deckel ist ein schmales Band hinter dem Scharnier."),
      retained: T("True case section 0.80 by 0.27 by 0.15 m, the 406 mm body with 38 mm ribs and 15/16 mm arching, string length 364 mm, bridge 38 mm, bow 740 mm with frog and head, the handle on the front wall, the zip line, both worn corners and the school inventory sticker.", "Wahrer Kofferquerschnitt 0,80 mal 0,27 mal 0,15 m, der 406-mm-Korpus mit 38 mm Zargen und 15/16 mm Wölbung, Mensur 364 mm, Steg 38 mm, Bogen 740 mm mit Frosch und Kopf, der Griff an der Vorderwand, die Reißverschlusslinie, beide abgestoßenen Ecken und der Inventaraufkleber der Musikschule."),
      merged: T("Back, ribs and neck are one maple colour; tailpiece, chinrest, pegs and fingerboard are the ebony fittings; the four strings are one stroke in the side view.", "Boden, Zargen und Hals sind ein Ahornton; Saitenhalter, Kinnhalter, Wirbel und Griffbrett sind die Ebenholzgarnitur; die vier Saiten sind in der Seitenansicht ein Strich."),
      omitted: T("Fine tuner, tailgut, bridge cut-outs, soundpost and bass bar (inside), zip teeth and the shoulder straps folded into their pocket.", "Feinstimmer, Saitenhalterschlinge, Stegausschnitte, Stimmstock und Bassbalken (innen), Reißverschlusszähne und die in ihrer Tasche verstauten Schulterriemen."),
      thinElements: T("Strings, purfling and the crack are 0.13 mm detail strokes; the repair cleats under the crack are drawn through the top as texture outlines; the bow is drawn as its outline from button to head because stick and hair 6 mm apart would leave a white sliver.", "Saiten, Einlage und Riss sind Detailstriche von 0,13 mm; die Rissleisten unter dem Riss sind als Texturumrisse durch die Decke gezeichnet; der Bogen ist als Umriss vom Knopf bis zum Kopf gezeichnet, weil Stange und Haar 6 mm auseinander einen weißen Spalt ließen."),
      hatching: T("No hatches: tones separate spruce, maple, ebony, velour and fabric.", "Keine Schraffur: Tonwerte trennen Fichte, Ahorn, Ebenholz, Velours und Gewebe."),
      styles: T("Body pen 0.25 mm, detail 0.13 mm, texture 0.10 mm, ground 0.35 mm.", "Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,10 mm, Boden 0,35 mm."),
      overlays: T("Traces of use as tones: the scuffed corners, the rosin dust between bridge and fingerboard, the worn varnish on the upper bout shoulder and the crack with its retouch.", "Gebrauchsspuren als Tonwerte: die abgestoßenen Ecken, der Kolophoniumstaub zwischen Steg und Griffbrett, der abgegriffene Lack an der oberen Schulter und der Riss mit seiner Retusche."),
    };
  if (s === 100)
    return {
      silhouette: T("Side elevation 8 mm by 3.6 mm: the case with the open lid, the bow in the lid and the viola above the base rim as body, neck, pegbox, scroll, fingerboard, tailpiece, chinrest and bridge. Front view: the case section, the standing lid and the bridge with the fingerboard above the rim. Plan: the dart outline with the viola's outline, neck, pegbox, scroll, fingerboard, tailpiece, chinrest and bridge in its bed.", "Seitenansicht 8 mm mal 3,6 mm: der Koffer mit offenem Deckel, dem Bogen im Deckel und der Bratsche über dem Unterteilrand als Korpus, Hals, Wirbelkasten, Schnecke, Griffbrett, Saitenhalter, Kinnhalter und Steg. Vorderansicht: der Kofferquerschnitt, der stehende Deckel und der Steg mit dem Griffbrett über dem Rand. Draufsicht: die Pfeilform mit Umriss, Hals, Wirbelkasten, Schnecke, Griffbrett, Saitenhalter, Kinnhalter und Steg der Bratsche im Bett."),
      retained: T("All case and viola dimensions, the arching in four rings, the bridge and the handle.", "Alle Koffer- und Bratschenmaße, die Wölbung in vier Ringen, der Steg und der Griff."),
      merged: T("Strings into the fingerboard (they run 0.5 to 5 mm above it), pegs into the pegbox, frog and head into the bow outline.", "Saiten ins Griffbrett (sie laufen 0,5 bis 5 mm darüber), Wirbel in den Wirbelkasten, Frosch und Kopf in den Bogenumriss."),
      omitted: T("Purfling, f-holes, crack, cleats, rosin dust, scuffs, sticker and zip line, all below 0.15 mm on paper.", "Einlage, F-Löcher, Riss, Leisten, Kolophoniumstaub, Abschürfungen, Aufkleber und Reißverschlusslinie, alle unter 0,15 mm auf Papier."),
      thinElements: T("The bow is one filled outline; the bridge keeps its 4 mm wedge.", "Der Bogen ist ein gefüllter Umriss; der Steg behält seinen 4-mm-Keil."),
      hatching: T("No hatches.", "Keine Schraffur."),
      styles: T("Body pen 0.25 mm, detail 0.13 mm, ground 0.35 mm.", "Körperstift 0,25 mm, Detail 0,13 mm, Boden 0,35 mm."),
      overlays: T("None at this scale.", "Keine in diesem Maßstab."),
    };
  return {
    silhouette: T("Side elevation 4 mm by 1.8 mm: the case base, the open lid with the bow and the viola as body, neck with pegbox and scroll and fingerboard above the rim. Front view: the case section with the standing lid. Plan: the dart outline with the viola outline, the neck with scroll and the fingerboard.", "Seitenansicht 4 mm mal 1,8 mm: das Unterteil, der offene Deckel mit dem Bogen und die Bratsche als Korpus, Hals mit Wirbelkasten und Schnecke und Griffbrett über dem Rand. Vorderansicht: der Kofferquerschnitt mit dem stehenden Deckel. Draufsicht: die Pfeilform mit dem Bratschenumriss, dem Hals mit Schnecke und dem Griffbrett."),
    retained: T("Case outline and height, the open lid, the viola outline, arching in two rings and the bow length.", "Kofferumriss und -höhe, der offene Deckel, der Bratschenumriss, die Wölbung in zwei Ringen und die Bogenlänge."),
    merged: T("Neck, pegbox and scroll are one outline; the lid lining is drawn with the rim; strings belong to the fingerboard outline.", "Hals, Wirbelkasten und Schnecke sind ein Umriss; das Deckelfutter ist mit dem Rand gezeichnet; die Saiten gehören zum Griffbrettumriss."),
    omitted: T("Handle, bridge, tailpiece, chinrest, pegs, frog and every trace of use: each below 0.15 mm or within one 0.18 mm pen of its neighbour.", "Griff, Steg, Saitenhalter, Kinnhalter, Wirbel, Frosch und alle Gebrauchsspuren: jeweils unter 0,15 mm oder innerhalb eines 0,18-mm-Stifts vom Nachbarn."),
    thinElements: T("The bow is a filled strip of 11 mm (0.05 mm plus the pen).", "Der Bogen ist ein gefüllter Streifen von 11 mm (0,05 mm plus Stift)."),
    hatching: T("No hatches.", "Keine Schraffur."),
    styles: T("Body pen 0.18 mm, detail 0.10 mm, ground 0.25 mm.", "Körperstift 0,18 mm, Detail 0,10 mm, Boden 0,25 mm."),
    overlays: T("None.", "Keine."),
  };
}

function volumeDecisions(s: number) {
  if (s === 50)
    return [
      T("Body: the case as a 12 mm floor (tapered 3 % for the rounded bottom edge), the 10 mm composite walls to the parting line, a 7.5 mm velour bed, the neck and scroll rests, the leather handle on two tabs; the lid as walls, a 10 mm skin with a rounded top edge, the 7.5 mm velour ceiling, the fabric hinge and two ebony bow holders. The viola is hollow: back and top are arched polyhedron plates (3.5 mm edge, 15 and 16 mm arching in seven rings) on a 1.2 mm maple rib ring, closed so that the inside is a sealed cavity; neck, pegbox, fingerboard and tailpiece are lofts, the scroll an ellipsoid, four pegs with shafts and heads, the chinrest, the 4 mm bridge standing on the arch, the end button and eight string segments. Traces of use are texture parts: f-holes, the crack retouch, three spruce cleats glued under it, the rosin dust, the worn varnish, both scuffed case corners and the inventory sticker.", "Körper: der Koffer als 12-mm-Boden (für die gerundete Unterkante um 3 % verjüngt), die 10-mm-Verbundwände bis zur Trennfuge, ein 7,5-mm-Velourbett, Hals- und Schneckenauflage, der Ledergriff an zwei Laschen; der Deckel als Wände, eine 10-mm-Haut mit gerundeter Oberkante, die 7,5-mm-Velourdecke, das Stoffscharnier und zwei Bogenhalter aus Ebenholz. Die Bratsche ist hohl: Boden und Decke sind gewölbte Polyeder-Platten (3,5 mm Rand, 15 und 16 mm Wölbung in sieben Ringen) auf einem 1,2-mm-Zargenkranz aus Ahorn, geschlossen, sodass das Innere ein dichter Hohlraum ist; Hals, Wirbelkasten, Griffbrett und Saitenhalter sind Lofts, die Schnecke ein Ellipsoid, vier Wirbel mit Schaft und Kopf, der Kinnhalter, der 4-mm-Steg auf der Wölbung, der Endknopf und acht Saitenabschnitte. Gebrauchsspuren sind Texturteile: F-Löcher, die Rissretusche, drei darunter geleimte Fichtenleisten, der Kolophoniumstaub, der abgegriffene Lack, beide abgestoßenen Kofferecken und der Inventaraufkleber."),
      T("Thermal: every body part is its own region surface (references); the sealed body cavity is not outer area, so the top and back count their outer faces only.", "Thermik: jedes Körperteil ist seine eigene Regionsoberfläche (Verweise); der dichte Korpushohlraum ist keine Außenfläche, daher zählen Decke und Boden nur ihre Außenseiten."),
      T("Structure: the case floor, walls and bed, the lid walls and skin on the hinge joint, the bow stick and the hair ribbon in its holders, and the viola box of back, ribs and top with the neck and the bridge, each with a mass marker at its centre.", "Struktur: Kofferboden, -wände und Bett, Deckelwände und -haut am Scharniergelenk, Bogenstange und Haarband in ihren Haltern und der Bratschenkasten aus Boden, Zargen und Decke mit Hals und Steg, jeweils mit einer Massenmarke im Schwerpunkt."),
    ];
  if (s === 100)
    return [
      T("Body: arching in four rings, ribs, neck, pegbox and scroll, fingerboard, tailpiece, chinrest and bridge; the case without tabs, scroll rest, stickers and scuffs; the bow as stick, hair and frog.", "Körper: Wölbung in vier Ringen, Zargen, Hals, Wirbelkasten und Schnecke, Griffbrett, Saitenhalter, Kinnhalter und Steg; der Koffer ohne Laschen, Schneckenauflage, Aufkleber und Abschürfungen; der Bogen als Stange, Haar und Frosch."),
      T("Pegs, end button, bow head, strings and all traces of use are below the 15 mm feature and left out of the body; the four strings stay as slender thermal solids of their true diameter.", "Wirbel, Endknopf, Bogenkopf, Saiten und alle Gebrauchsspuren liegen unter dem 15-mm-Merkmal und fehlen im Körper; die vier Saiten bleiben als schlanke thermische Körper ihres wahren Durchmessers."),
      T("Structure as at 1:50 with simple bow holders.", "Struktur wie 1:50 mit einfachen Bogenhaltern."),
    ];
  return [
    T("Body: case floor, walls and bed, the lid walls and skin, the viola as back, ribs and top in two rings, one loft of neck, pegbox and scroll, the fingerboard, and the bow as one slender stick that takes the hair in.", "Körper: Kofferboden, -wände und Bett, Deckelwände und -haut, die Bratsche als Boden, Zargen und Decke in zwei Ringen, ein Loft aus Hals, Wirbelkasten und Schnecke, das Griffbrett und der Bogen als eine schlanke Stange, die das Haar aufnimmt."),
    T("Handle, rests, lid lining, bridge, tailpiece, chinrest, pegs and frog are below the 30 mm feature and left out; two slender strings remain in the thermal list.", "Griff, Auflagen, Deckelfutter, Steg, Saitenhalter, Kinnhalter, Wirbel und Frosch liegen unter dem 30-mm-Merkmal und fehlen; zwei schlanke Saiten bleiben in der thermischen Liste."),
    T("Structure: case, lid, the viola box with the neck loft and the bow stick, with merged holders.", "Struktur: Koffer, Deckel, der Bratschenkasten mit dem Halsloft und die Bogenstange mit zusammengefassten Haltern."),
  ];
}
//#endregion

//#region 📍️ Anchors, materials, regions
const anchor = (id: string, bone: string, p: V, kind: string, en: string, de: string, extra: any = {}) => ({ id, bone, x: q(p[0]), y: q(p[1]), z: q(p[2]), kind, title: T(en, de), ...extra });
const vp = (x: number, y: number, z: number): V => [vlx(x), vly(y), z];
const ANCHORS = [
  anchor("floor-wide-left", "case", [0.1, 0, 0.08], "contact", "Floor under the wide end, left", "Boden unter dem breiten Ende, links", { support: "contact", footprintLengthM: 0.08, footprintWidthM: 0.06, frictionCoefficient: 0.5 }),
  anchor("floor-wide-right", "case", [0.1, 0, -0.08], "contact", "Floor under the wide end, right", "Boden unter dem breiten Ende, rechts", { support: "contact", footprintLengthM: 0.08, footprintWidthM: 0.06, frictionCoefficient: 0.5 }),
  anchor("floor-narrow", "case", [0.7, 0, 0], "contact", "Floor under the narrow end", "Boden unter dem schmalen Ende", { support: "contact", footprintLengthM: 0.06, footprintWidthM: 0.06, frictionCoefficient: 0.5 }),
  anchor("handle-grip", "case", [0.4, handleY, handleZ], "support", "Hand on the carrying handle", "Hand am Tragegriff", { support: "pin" }),
  anchor("strap-upper", "case", [0.62, 0, 0], "support", "Shoulder straps at their upper rings", "Schulterriemen an ihren oberen Ringen", { support: "pin" }),
  anchor("back-contact", "case", [0.16, 0, 0], "contact", "Case floor against the lower back", "Kofferboden am unteren Rücken", { support: "contact", normalDeg: 180, footprintLengthM: 0.15, footprintWidthM: 0.2, frictionCoefficient: 0.6 }),
  anchor("collarbone", "viola", vp(0.075, 0.03, 0.035), "contact", "Back of the viola on the collarbone", "Bratschenboden auf dem Schlüsselbein", { support: "contact", footprintLengthM: 0.04, footprintWidthM: 0.03, frictionCoefficient: 0.6 }),
  anchor("left-hand", "viola", vp(0.585, 0.052, 0), "support", "Left hand cradling the neck in first position", "Linke Hand hält den Hals in der ersten Lage", { support: "pin" }),
  anchor("chin", "viola", vp(0.08, 0.1042, 0.062), "load", "Jaw on the chinrest", "Kiefer auf dem Kinnhalter"),
  anchor("fingers", "viola", vp(0.52, fbTop(0.52), 0), "load", "Fingers stopping the strings", "Finger greifen die Saiten"),
  anchor("nut", "viola", vp(VIOLA.nutX, VIOLA.stringNutY, 0), "load", "Strings on the nut", "Saiten auf dem Sattel"),
  anchor("bridge-top", "viola", vp(VIOLA.bridgeX, VIOLA.stringBridgeY, 0), "load", "Strings on the bridge", "Saiten auf dem Steg"),
  anchor("tail-saddle", "viola", vp(0.047, 0.083, 0), "load", "Tailgut over the saddle", "Saitenhalterschlinge über dem Untersattel"),
  anchor("frog-grip", "bow-stroke", [-0.02, 0.02, 0], "support", "Right hand at the frog", "Rechte Hand am Frosch", { support: "pin" }),
  anchor("index-press", "bow-stroke", [0.045, 0.022, 0], "load", "Index finger pressing the stick", "Zeigefinger drückt auf die Stange"),
  anchor("hair-frog", "bow-stroke", [0, 0, 0], "load", "Hair at the frog ferrule", "Haar an der Froschzwinge"),
  anchor("hair-tip", "bow-stroke", [BOW.hair, 0.001, 0], "load", "Hair in the head mortise", "Haar im Kopfkeil"),
];

const mat = (id: string, en: string, de: string, emissivity: number, solar: number, rho: number, k: number, cp: number, E: number | null, source: string, reference: string) => ({ id, title: T(en, de), emissivity, solarAbsorptivity: solar, densityKgM3: rho, conductivityWmK: k, specificHeatJKgK: cp, ...(E ? { elasticModulusPa: E } : {}), source, reference });
const MATERIALS = [
  mat("case-fabric", "Nylon cover of the case, faded by years of sun", "Nylonbezug des Koffers, von Jahren Sonne verblichen", 0.88, 0.62, 1140, 0.25, 1700, 2.5e9, "authored-estimate", "incropera-2018"),
  mat("case-shell", "Plywood and EPS foam composite shell", "Verbundschale aus Sperrholz und EPS-Schaum", 0.9, 0.6, 350, 0.05, 1500, 3e9, "authored-estimate", "case-catalogue"),
  mat("velour", "Polyester velour lining on PE foam, flattened where the viola lies", "Polyester-Velourfutter auf PE-Schaum, wo die Bratsche liegt plattgedrückt", 0.95, 0.5, 60, 0.04, 1500, null, "authored-estimate", "case-catalogue"),
  mat("leather", "Leather of the carrying handle, worn shiny", "Leder des Tragegriffs, glänzend abgegriffen", 0.9, 0.7, 860, 0.16, 1500, null, "literature", "incropera-2018"),
  mat("spruce", "Spruce top under an aged oil varnish", "Fichtendecke unter gealtertem Öllack", 0.9, 0.62, 420, 0.11, 1300, 1e10, "literature", "wegst-2006"),
  mat("maple", "Flamed maple back, ribs and neck under spirit varnish, worn at the neck", "Geflammter Ahornboden, Zargen und Hals unter Spirituslack, am Hals abgegriffen", 0.9, 0.66, 650, 0.16, 1300, 1.2e10, "literature", "wegst-2006"),
  mat("ebony", "Ebony fittings, fingerboard scooped by playing", "Ebenholzgarnitur, Griffbrett vom Spiel ausgespielt", 0.92, 0.9, 1100, 0.2, 1300, 1.7e10, "literature", "wegst-2006"),
  mat("string-steel", "Wound strings on steel and synthetic cores", "Umsponnene Saiten mit Stahl- und Kunststoffkern", 0.25, 0.4, 7850, 50, 470, 2e11, "literature", "incropera-2018"),
  mat("pernambuco", "Pernambuco bow stick", "Fernambuk-Bogenstange", 0.9, 0.7, 1000, 0.2, 1300, 2.5e10, "literature", "wegst-2006"),
  mat("horsehair", "White horsehair, rosined and thinned", "Weißes Rosshaar, kolophoniert und ausgedünnt", 0.9, 0.3, 1300, 0.25, 1500, 7e9, "authored-estimate", "fletcher-rossing-1998"),
  mat("rosin", "Rosin dust and paper labels", "Kolophoniumstaub und Papieretiketten", 0.92, 0.4, 1070, 0.13, 1500, null, "authored-estimate", "incropera-2018"),
];

const layer = (id: string, en: string, de: string, kind: string, material: string, t: number, k: number, rho: number, cp: number, source = "authored-estimate", reference = "incropera-2018") => ({ id, title: T(en, de), kind, material, thicknessM: t, resistanceM2KW: q(t / k, 5), heatCapacityJM2K: Math.round(rho * cp * t), source, reference });
const region = (id: string, en: string, de: string, kind: string, material: string, areaM2: number, L: number, geometry: string, layers: any[], extra: any = {}) => ({ id, title: T(en, de), kind, model: "passive", material, areaM2, characteristicLengthM: L, geometry, skyViewFactor: 0, layers, heatCapacityJM2K: layers.reduce((a, l) => a + l.heatCapacityJM2K, 0), source: "authored-estimate", reference: "incropera-2018", ...extra });
const AREA: Record<string, number> = JSON.parse(process.env.VIOLA_AREAS ?? "{}");
const REGION_LIST = [
  region("case-cover", "Case shell with its faded nylon cover", "Kofferschale mit verblichenem Nylonbezug", "surface", "case-fabric", AREA["case-cover"] ?? 0.85, 0.27, "horizontal-plate-up", [{ ...layer("shell", "Nylon-covered plywood and EPS shell 10 mm", "Nylonbezogene Schale aus Sperrholz und EPS 10 mm", "insulation", "case-shell", 0.01, 0.0532, 350, 1500, "authored-estimate", "case-catalogue"), heatCapacityJM2K: 6380 }], { ratedPowerW: 0, ratedPowerBasis: "passive" }),
  region("case-lining", "Velour lining of the bed and the lid", "Velourfutter von Bett und Deckel", "surface", "velour", AREA["case-lining"] ?? 0.292, 0.25, "horizontal-plate-up", [layer("foam-velour", "Velour on PE foam 7.5 mm", "Velours auf PE-Schaum 7,5 mm", "insulation", "velour", 0.0075, 0.04, 60, 1500, "authored-estimate", "case-catalogue")]),
  region("viola-top", "Spruce top under worn varnish, rosin dust and the cleated crack", "Fichtendecke unter abgegriffenem Lack, Kolophoniumstaub und dem geleimten Riss", "surface", "spruce", AREA["viola-top"] ?? 0.081, 0.235, "horizontal-plate-up", [layer("spruce-plate", "Spruce plate 3.5 mm", "Fichtenplatte 3,5 mm", "timber", "spruce", 0.0035, 0.11, 420, 1300, "literature", "fpl-2010")]),
  region("viola-body", "Maple back, ribs, neck, scroll and bridge", "Ahornboden, Zargen, Hals, Schnecke und Steg", "surface", "maple", AREA["viola-body"] ?? 0.151, 0.2, "vertical-plate", [layer("maple-plate", "Maple plate 3.5 mm", "Ahornplatte 3,5 mm", "timber", "maple", 0.0035, 0.16, 650, 1300, "literature", "fpl-2010")]),
  region("fittings", "Ebony fingerboard, tailpiece, chinrest, pegs and end button", "Griffbrett, Saitenhalter, Kinnhalter, Wirbel und Endknopf aus Ebenholz", "surface", "ebony", AREA["fittings"] ?? 0.0526, 0.04, "cylinder", [layer("ebony-wall", "Ebony 6 mm", "Ebenholz 6 mm", "timber", "ebony", 0.006, 0.2, 1100, 1300, "literature", "fpl-2010")]),
  region("strings", "Four wound strings C, G, D, A", "Vier umsponnene Saiten C, G, D, A", "surface", "string-steel", AREA["strings"] ?? 0.00503, 0.001, "cylinder", [layer("winding", "Metal winding 0.5 mm", "Metallumspinnung 0,5 mm", "metal", "string-steel", 0.0005, 50, 7850, 470, "literature", "incropera-2018")]),
  region("bow", "Bow: pernambuco stick, horsehair and ebony frog", "Bogen: Fernambukstange, Rosshaar und Ebenholzfrosch", "surface", "pernambuco", AREA["bow"] ?? 0.0378, 0.009, "cylinder", [layer("stick-wood", "Pernambuco 4 mm", "Fernambuk 4 mm", "timber", "pernambuco", 0.004, 0.2, 1000, 1300, "literature", "wegst-2006")]),
];
//#endregion

//#region 🧍️ Poses and clips
const STRING_T = 270;
const HAIR_T = 55;
type Off = Record<string, any>;
function loadsFor(bones: Off, root: { x: number; y: number; angleDeg: number }, played: boolean) {
  const f = solveChain(RIG as any, bones, root);
  const at = (id: string) => { const a = ANCHORS.find((x) => x.id === id)!; return worldOf(f[a.bone]!.frame, [a.x, a.y, a.z]); };
  const nut = at("nut"), br = at("bridge-top"), tail = at("tail-saddle");
  const fn = mul(unit(sub(br, nut)), STRING_T), ft = mul(unit(sub(br, tail)), STRING_T), fb = mul(add(fn, ft), -1);
  const bu = f["bow-stroke"]!.frame.u as V, bv = f["bow-stroke"]!.frame.v as V, vv = f["viola"]!.frame.v as V;
  const load = (id: string, kind: string, anchorId: string, F: V, en: string, de: string, source: string, reference: string) => ({ id, kind, title: T(en, de), anchor: anchorId, fxN: q(F[0], 2), fyN: q(F[1], 2), fzN: q(F[2], 2), source, reference });
  const out = [
    load("strings-on-nut", "imposed", "nut", fn, "Four strings pulling the nut towards the bridge (0.27 kN)", "Vier Saiten ziehen den Sattel zum Steg (0,27 kN)", "literature", "string-tension-catalogue"),
    load("strings-on-tailpiece", "imposed", "tail-saddle", ft, "Afterlength pulling the tailpiece towards the bridge", "Nachlänge zieht den Saitenhalter zum Steg", "literature", "string-tension-catalogue"),
    load("strings-on-bridge", "imposed", "bridge-top", fb, "Down-bearing of the strings on the bridge", "Druck der Saiten auf den Steg", "literature", "fletcher-rossing-1998"),
    load("hair-on-frog", "imposed", "hair-frog", mul(bu, HAIR_T), "Bow hair tension on the frog (55 N)", "Bogenhaarspannung am Frosch (55 N)", "literature", "fletcher-rossing-1998"),
    load("hair-on-head", "imposed", "hair-tip", mul(bu, -HAIR_T), "Bow hair tension on the head", "Bogenhaarspannung am Kopf", "literature", "fletcher-rossing-1998"),
  ];
  if (played)
    out.push(
      load("jaw", "occupant", "chin", [0, -15, 0], "Weight of the head on the chinrest (15 N)", "Gewicht des Kopfes auf dem Kinnhalter (15 N)", "authored-estimate", "fletcher-rossing-1998"),
      load("stopping-fingers", "occupant", "fingers", mul(vv, -5), "Fingers stopping the strings (5 N)", "Finger greifen die Saiten (5 N)", "authored-estimate", "fletcher-rossing-1998"),
      load("bow-pressure", "occupant", "index-press", mul(bv, -1.5), "Index finger pressure that sets the bow force", "Zeigefingerdruck, der die Bogenkraft einstellt", "literature", "fletcher-rossing-1998"),
    );
  return out;
}
const regions = (b: Record<string, [number, number?, number?]>) => Object.fromEntries(Object.entries(b).map(([k, [e, cf, ct]]) => [k, { exposedFraction: e, sunlitFraction: 0, ...(cf ? { contact: { fraction: cf, tempC: ct } } : {}) }]));
const storedBones = { lid: { angleDeg: CASE.openDeg } };
const closedBones = {};
const carriedRoot = { x: 0, y: 0.8, angleDeg: 90 };
const POSES = [
  { id: "stored", title: T("Stored open at the dealer", "Offen im Laden gelagert"), description: T("The case lies on the dealer's shelf with the lid open at 95 degrees: the viola rests on its back in the velour bed, the bow is clipped in the lid, strings tuned and the bridge upright, ready to be tried.", "Der Koffer liegt im Regal des Händlers, der Deckel steht um 95 Grad offen: die Bratsche liegt auf dem Rücken im Velourbett, der Bogen ist im Deckel eingeklemmt, die Saiten gestimmt und der Steg aufrecht, bereit zum Anspielen."), bones: storedBones, ground: "auto", contacts: ["floor-wide-left", "floor-wide-right", "floor-narrow"], thermal: { regions: regions({ "case-cover": [0.88, 0.12, 20], "case-lining": [0.89], "viola-top": [0.87], "viola-body": [1, 0.12, 20], fittings: [0.85], strings: [0.95], bow: [0.98] }), emitters: {} }, loads: loadsFor(storedBones, { x: 0, y: 0, angleDeg: 0 }, false) },
  { id: "closed", title: T("Closed and zipped", "Geschlossen"), description: T("Lid closed and zipped: viola and bow lie in the still air of the case.", "Deckel zu und Reißverschluss geschlossen: Bratsche und Bogen liegen in der ruhenden Luft des Koffers."), bones: closedBones, ground: "auto", contacts: ["floor-wide-left", "floor-wide-right", "floor-narrow"], thermal: { regions: regions({ "case-cover": [0.85, 0.12, 20], "case-lining": [0.1], "viola-top": [0.1], "viola-body": [0.1, 0.12, 20], fittings: [0.1], strings: [0.1], bow: [0.1] }), emitters: {} }, loads: loadsFor(closedBones, { x: 0, y: 0, angleDeg: 0 }, false) },
  { id: "carried", title: T("Carried on the back", "Auf dem Rücken getragen"), description: T("Closed and carried upright on the back by the shoulder straps, scroll end up, the case floor against the lower back at 0.8 to 1.6 m.", "Geschlossen und aufrecht an den Schulterriemen auf dem Rücken getragen, Schneckenende oben, der Kofferboden am unteren Rücken zwischen 0,8 und 1,6 m."), bones: closedBones, root: carriedRoot, ground: "authored", contacts: ["strap-upper", "back-contact"], thermal: { regions: regions({ "case-cover": [0.8, 0.15, 33], "case-lining": [0.1], "viola-top": [0.1], "viola-body": [0.1], fittings: [0.1], strings: [0.1], bow: [0.1] }), emitters: {} }, loads: loadsFor(closedBones, carriedRoot, false) },
  { id: "played", title: T("Played seated in the orchestra", "Im Orchester sitzend gespielt"), description: T("Seated on an orchestra chair: the viola lies on the left collarbone at 1.06 m with the neck raised by 12 degrees and the scroll ahead, the left hand cradles the neck, the bow crosses the D string 35 mm from the bridge; the open case lies on the floor beside the chair.", "Auf einem Orchesterstuhl sitzend: die Bratsche liegt in 1,06 m auf dem linken Schlüsselbein, der Hals um 12 Grad angehoben, die Schnecke vorn, die linke Hand hält den Hals, der Bogen streicht die D-Saite 35 mm vor dem Steg; der offene Koffer liegt neben dem Stuhl am Boden."), bones: playedBones, ground: "auto", contacts: ["floor-wide-left", "floor-wide-right", "floor-narrow", "collarbone", "left-hand", "frog-grip"], thermal: { regions: regions({ "case-cover": [0.85, 0.12, 20], "case-lining": [1], "viola-top": [1], "viola-body": [0.85, 0.08, 33], fittings: [0.75, 0.25, 34], strings: [1, 0.05, 32], bow: [0.95, 0.06, 32] }), emitters: {} }, loads: loadsFor(playedBones, { x: 0, y: 0, angleDeg: 0 }, true) },
];
const span = (anchorId: string, fromS: number, toS: number) => ({ anchor: anchorId, fromS, toS });
const CLIPS = [
  {
    id: "bowing", title: T("Bowing the D string", "Streichen der D-Saite"), description: T("A slow whole bow on the D string: two seconds up-bow from the tip half to the frog, two seconds down-bow back; the hair stays on the string 35 mm from the bridge.", "Ein langsamer ganzer Bogen auf der D-Saite: zwei Sekunden Aufstrich von der Spitzenhälfte zum Frosch, zwei Sekunden Abstrich zurück; das Haar bleibt 35 mm vor dem Steg auf der Saite."),
    durationS: 4, fps: 25, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "played", ease: "smooth", bones: { "bow-stroke": { dx: -0.2 } } }, { t: 2, pose: "played", ease: "smooth", bones: { "bow-stroke": { dx: 0.2 } } }, { t: 4, pose: "played", ease: "smooth", bones: { "bow-stroke": { dx: -0.2 } } }],
    contacts: ["floor-wide-left", "floor-wide-right", "floor-narrow", "collarbone", "left-hand", "frog-grip"].map((a) => span(a, 0, 4)),
    phases: [{ id: "up-bow", fromS: 0, toS: 2, title: T("Up-bow", "Aufstrich") }, { id: "down-bow", fromS: 2, toS: 4, title: T("Down-bow", "Abstrich") }],
    events: [{ t: 2, kind: "motion", title: T("Bow change at the frog", "Bogenwechsel am Frosch"), target: "bow-stroke" }],
    tracks: [],
  },
  {
    id: "carry", title: T("Close and carry", "Schließen und tragen"), description: T("The lid with the bow swings shut and is zipped, a hand takes the handle, lifts the case and swings it upright onto the back, where the straps take it.", "Der Deckel mit dem Bogen schwenkt zu und wird geschlossen, eine Hand nimmt den Griff, hebt den Koffer und schwingt ihn aufrecht auf den Rücken, wo die Riemen ihn übernehmen."),
    durationS: 4, fps: 15, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "stored", ease: "smooth" }, { t: 1.2, pose: "closed", ease: "smooth" }, { t: 1.6, pose: "closed", ease: "smooth" }, { t: 4, pose: "carried", ease: "smooth" }],
    contacts: [span("floor-wide-left", 0, 2), span("floor-wide-right", 0, 2), span("floor-narrow", 0, 2), span("handle-grip", 1.4, 3.6), span("strap-upper", 3.2, 4), span("back-contact", 3.4, 4)],
    phases: [{ id: "closing", fromS: 0, toS: 1.2, title: T("Closing the lid", "Deckel schließen") }, { id: "gripping", fromS: 1.2, toS: 1.6, title: T("Taking the handle", "Griff fassen") }, { id: "lifting", fromS: 1.6, toS: 3.4, title: T("Lifting onto the back", "Auf den Rücken heben") }, { id: "settling", fromS: 3.4, toS: 4, title: T("Straps take the load", "Riemen übernehmen") }],
    events: [{ t: 0.1, kind: "motion", title: T("The lid starts to close", "Der Deckel beginnt zu schließen"), target: "lid" }, { t: 1.4, kind: "contact-on", title: T("Hand on the handle", "Hand am Griff"), target: "handle-grip" }, { t: 2, kind: "contact-off", title: T("The case leaves the floor", "Der Koffer verlässt den Boden"), target: "floor-narrow" }, { t: 3.2, kind: "contact-on", title: T("Straps on the shoulders", "Riemen auf den Schultern"), target: "strap-upper" }],
    tracks: [],
  },
];
//#endregion

//#region 🧾️ Asset
const masses: Record<string, { kg: number; comX: number; comY: number }> = { case: { kg: 1.48, comX: 0.36, comY: 0.04 }, lid: { kg: 0.72, comX: 0.115, comY: 0.035 }, viola: { kg: 0.6, comX: 0.2, comY: 0.004 }, "bow-stroke": { kg: 0.072, comX: 0.26, comY: 0.012 } };
const sections: Record<string, any> = {
  case: { areaM2: 0.0052, secondMomentM4: 1.1e-5, extremeFibreM: 0.075, material: "case-shell", title: T("Dart shell section 0.27 by 0.15 m, 10 mm composite wall", "Pfeilschalenquerschnitt 0,27 mal 0,15 m, 10-mm-Verbundwand") },
  lid: { areaM2: 0.0034, secondMomentM4: 1.5e-6, extremeFibreM: 0.03, material: "case-shell", title: T("Lid shell with 10 mm wall and skin", "Deckelschale mit 10-mm-Wand und -Haut") },
  viola: { areaM2: 0.00052, secondMomentM4: 2.6e-8, extremeFibreM: 0.0125, material: "maple", title: T("Neck at the heel, 34 by 25 mm D-section with the fingerboard", "Hals am Fuß, D-Querschnitt 34 mal 25 mm mit Griffbrett") },
  "bow-stroke": { areaM2: 4.5e-5, secondMomentM4: 1.6e-10, extremeFibreM: 0.0038, material: "pernambuco", title: T("Stick near the frog, 7.6 mm round", "Stange am Frosch, 7,6 mm rund") },
};
const rig = { ...RIG, bones: RIG.bones.map((b: any) => ({ ...b, ...(masses[b.id] ? { mass: { ...masses[b.id], distribution: "point", source: "authored-estimate", reference: b.id === "case" ? "case-catalogue" : b.id === "bow-stroke" ? "bow-catalogue" : b.id === "lid" ? "case-catalogue" : "riley-1980" } } : {}), ...(sections[b.id] ? { section: sections[b.id] } : {}) })) };
const massKg = Object.values(masses).reduce((a, m) => a + m.kg, 0);
const DIMS = JSON.parse(process.env.VIOLA_DIMS ?? "null") ?? { heightM: 0.361, widthM: 0.814, depthM: 0.357 };
const unavailable500 = { status: "unavailable", denominator: 500, cause: "below-printable-feature", reason: T("At 1:500 the whole case is 1.6 mm long and 0.7 mm high on paper, the viola inside it 1.4 mm: below the 3 mm under which an object is a dot, and every part of it (the 38 mm ribs, the 11 mm bow) is far below the 0.2 mm print minimum.", "Bei 1:500 ist der ganze Koffer auf Papier 1,6 mm lang und 0,7 mm hoch, die Bratsche darin 1,4 mm: unter den 3 mm, unter denen ein Gegenstand ein Punkt ist, und jedes Teil (die 38-mm-Zargen, der 11-mm-Bogen) liegt weit unter dem Druckminimum von 0,2 mm."), limitingFeature: T("The whole case, 0.80 m long", "Der ganze Koffer, 0,80 m lang"), limitingSizeM: 0.8, alternatives: [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }] };
const reps = { "50": representation(50), "100": representation(100), "200": representation(200), "500": unavailable500 };
const vols = { "50": volume(50), "100": volume(100), "200": volume(200), "500": { status: "unavailable", denominator: 500 } };
const asset = {
  id: "used-viola",
  category: "objects",
  title: T("Used Viola", "Gebrauchte Bratsche"),
  summary: T("A second-hand 16-inch orchestra viola (406 mm body, 364 mm string length) with its 740 mm bow in a faded dart-shaped case of 0.80 m, as a used-instrument dealer offers it: a cleated crack in the lower bout of the top, rosin dust under the strings, worn varnish on the shoulder and scuffed case corners, set up and tuned to be played again. Stored open, carried on the back and played seated in an orchestra chair.", "Eine gebrauchte 16-Zoll-Orchesterbratsche (406 mm Korpus, 364 mm Mensur) mit ihrem 740-mm-Bogen in einem verblichenen pfeilförmigen Formetui von 0,80 m, wie ein Gebrauchtinstrumentenhandel sie anbietet: ein mit Leisten geleimter Riss im unteren Deckenbogen, Kolophoniumstaub unter den Saiten, abgegriffener Lack an der Schulter und abgestoßene Kofferecken, eingerichtet und gestimmt zum Weiterspielen. Offen gelagert, auf dem Rücken getragen und im Orchesterstuhl sitzend gespielt."),
  tags: ["used", "instrument", "music", "second-hand", "strings", "viola", "bowed", "chordophone", "orchestra", "case", "dart-case", "bow", "spruce", "maple", "ebony", "music-school"],
  provenance: {
    author: "Semio staffage content (used instruments wave, row 05)",
    method: "handcrafted",
    created: "2026-11-22",
    references: [
      { id: "hornbostel-sachs-1914", citation: "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Zeitschrift für Ethnologie 46, 553-590 (viola: 321.322-71, composite chordophone, necked box lute sounded by a bow)." },
      { id: "riley-1980", citation: "Riley, M. W. (1980). The History of the Viola, Braun-Brumfield, Ann Arbor (orchestral viola body lengths 15 to 17 inch; 16 inch = 406 mm the common orchestra size)." },
      { id: "johnson-courtnall-1999", citation: "Johnson, C., Courtnall, R. (1999). The Art of Violin Making, Robert Hale, London (arching heights, rib heights, plate edges, neck overstand and projection, bridge height, fingerboard dimensions)." },
      { id: "fletcher-rossing-1998", citation: "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed., Springer, chapter 10 (bowed strings: bridge down-bearing, bow force 0.1 to 1.5 N, bow hair tension)." },
      { id: "string-tension-catalogue", citation: "Catalogue-typical tensions of synthetic-core viola strings for a 16-inch scale, C 59 N, G 61 N, D 70 N, A 80 N, about 0.27 kN together (string makers' tension charts)." },
      { id: "case-catalogue", citation: "Catalogue-typical shaped (dart) viola cases for 15.5 to 16.5 inch: 0.79 to 0.82 m long, 0.26 to 0.28 m wide, 0.14 to 0.16 m high, 1.9 to 2.6 kg, plywood or EPS shell with a nylon cover, velour lining, two bow holders in the lid, handle and backpack straps (dealer listings)." },
      { id: "bow-catalogue", citation: "Catalogue-typical viola bows: 740 to 745 mm long, 68 to 74 g, pernambuco or brazilwood stick, ebony frog, hair ribbon about 11 mm (bow makers' data)." },
      { id: "wegst-2006", citation: "Wegst, U. G. K. (2006). Wood for sound. American Journal of Botany 93(10), 1439-1448 (density and Young's modulus of spruce, maple, pernambuco and ebony)." },
      { id: "fpl-2010", citation: "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, FPL-GTR-190, USDA (thermal conductivity and specific heat of wood)." },
      { id: "incropera-2018", citation: "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities, conductivities of polymers and metals)." },
      { id: "iso-9886-2004", citation: "ISO 9886:2004 Ergonomics - Evaluation of thermal strain by physiological measurements (skin temperatures of 32 to 34 degC at the jaw and hands)." },
    ],
    notes: T("Coordinates: the case lies with its long axis ahead (x, scroll end ahead), its fabric hinge on the left long side (+z). Nothing is measured; dimensions are literature and catalogue-typical values, the traces of use authored estimates. The playing pose has no musician: the viola and the bow are held where a seated violist holds them, and the case lies open on the floor.", "Koordinaten: der Koffer liegt mit seiner Längsachse nach vorn (x, Schneckenende vorn), sein Stoffscharnier an der linken Längsseite (+z). Nichts ist gemessen; die Maße sind Literatur- und katalogtypische Werte, die Gebrauchsspuren geschätzt. Die Spielpose hat keinen Musiker: Bratsche und Bogen sind dort gehalten, wo ein sitzender Bratscher sie hält, und der Koffer liegt offen am Boden."),
  },
  dimensions: { ...DIMS, massKg: q(massKg, 3) },
  materials: MATERIALS,
  rig,
  anchors: ANCHORS,
  representations: reps,
  symbols: [],
  poses: POSES,
  clips: CLIPS,
  thermal: {
    environment: { airTempC: 20, meanRadiantTempC: 20, relativeHumidity: 0.45, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
    passengers: [],
    operating: [],
    regions: REGION_LIST,
    emitters: [],
    assumptions: [
      T("An acoustic instrument in its case: no power and no heat source of its own (passive, 0 W). In the shop at 20 degC everything stays at room temperature; the closed case keeps the viola in still air.", "Ein akustisches Instrument im Koffer: keine Leistung und keine eigene Wärmequelle (passiv, 0 W). Im Laden bei 20 degC bleibt alles bei Raumtemperatur; der geschlossene Koffer hält die Bratsche in ruhender Luft."),
      T("When played, the jaw (34 degC) warms a quarter of the ebony fittings through the chinrest, the collarbone and the left hand warm the back and neck (33 degC) and the right hand the bow frog; aged varnish absorbs more sunlight (0.62 to 0.66) than new varnish (about 0.5).", "Beim Spielen wärmt der Kiefer (34 degC) über den Kinnhalter ein Viertel der Ebenholzgarnitur, Schlüsselbein und linke Hand wärmen Boden und Hals (33 degC) und die rechte Hand den Bogenfrosch; gealterter Lack absorbiert mehr Sonnenlicht (0,62 bis 0,66) als neuer (etwa 0,5)."),
    ],
  },
  structure: {
    gravityMS2: 9.80665,
    dynamics: "quasi-static",
    supports: [],
    assumptions: [
      T("Stored and closed, the 2.9 kg case stands on three floor contacts; the viola lies on its back in the velour bed and on the neck rest, the bow in two holders of the lid. Carried, the straps hold the case at their upper rings and the case floor leans on the lower back.", "Gelagert und geschlossen steht der 2,9 kg schwere Koffer auf drei Bodenkontakten; die Bratsche liegt auf dem Rücken im Velourbett und auf der Halsauflage, der Bogen in zwei Haltern des Deckels. Getragen halten die Riemen den Koffer an ihren oberen Ringen und der Kofferboden lehnt am unteren Rücken."),
      T("In every pose the tuned strings load the viola: 0.27 kN pull the nut towards the bridge and the tailpiece towards the bridge, and press the bridge down with about 0.1 kN; the bow hair pulls frog and head together with 55 N, so neck and stick are in compression and bending.", "In jeder Pose belasten die gestimmten Saiten die Bratsche: 0,27 kN ziehen den Sattel und den Saitenhalter zum Steg und drücken den Steg mit etwa 0,1 kN nieder; das Bogenhaar zieht Frosch und Kopf mit 55 N zusammen, sodass Hals und Stange auf Druck und Biegung beansprucht sind."),
      T("Played, the viola lies on the collarbone and in the left hand and carries 15 N of the head on the chinrest and 5 N of the stopping fingers; the right hand holds the bow at the frog and sets 1.5 N of bow pressure. The asset is solved as one rigid body, so the floor under the open case and the hands share the reactions as the pseudo-inverse distributes them.", "Gespielt liegt die Bratsche auf dem Schlüsselbein und in der linken Hand und trägt 15 N des Kopfes auf dem Kinnhalter und 5 N der greifenden Finger; die rechte Hand hält den Bogen am Frosch und gibt 1,5 N Bogendruck. Das Asset wird als ein starrer Körper gelöst, daher teilen sich der Boden unter dem offenen Koffer und die Hände die Reaktionen, wie die Pseudoinverse sie verteilt."),
    ],
    clinical: false,
  },
  volumes: vols,
  presentation: { defaultPose: "stored", defaultClip: "bowing", defaultScale: 50 },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
  completion: "authored",
};
const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
writeFileSync(join(HERE, "🔣️.json"), JSON.stringify(doc, null, 1) + "\n");
for (const s of ["50", "100", "200"] as const) debug(`1:${s}`, "body", (vols as any)[s].body.length, "thermal", (vols as any)[s].thermal.length, "structure", (vols as any)[s].structure.length, "shapes", Object.fromEntries(Object.entries((reps as any)[s].geometry).map(([k, v]: any) => [k, v.length])));
debug("mass", q(massKg, 3), "written", join(HERE, "🔣️.json"));
//#endregion
