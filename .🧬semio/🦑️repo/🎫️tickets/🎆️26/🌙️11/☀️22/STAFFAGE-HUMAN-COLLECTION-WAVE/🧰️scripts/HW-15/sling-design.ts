/** 🩹️ Design inputs of `person-with-arm-sling`: the sling posture of the right arm, the revolved drawings of the arm chain, the fibreglass cast, the pouch and the neck strap with their volumes, regions and masses.
 * Cast after common short-arm casts (below the elbow to the knuckles, 3 to 4 layers of fibreglass over padding); pouch sling after common pouch arm slings (forearm about horizontal, hand slightly above the elbow).
 * @see ./gen-arm-sling.ts
 */
import { SCALES, VIEWS, frames, restFrames, toLocal, toWorld, clone, type V3 } from "./lib.ts";

/** 🦾️ The right arm in the sling: upper arm 20° forward, rotated 62° inwards, elbow at 92°, wrist straight in the cast. */
export const SLING = {
  offsets: { "arm-r": { angleDeg: 20, abductionDeg: -4, twistDeg: -62 }, "forearm-r": { angleDeg: 92 }, "hand-r": { angleDeg: 0 } } as Record<string, any>,
  pouch: { u0: -0.055, u1: 0.245, r: 0.062 },
  castForearm: { u0: 0.2, u1: 0.262, r: 0.042 },
  castHand: { u0: -0.012, u1: 0.07, r0: 0.042, r1: 0.037 },
  strapR: 0.011,
};

const FINGERS = new Set(["hand-r", "thumb-metacarpal-r", "thumb-proximal-r", "thumb-distal-r", "index-proximal-r", "index-middle-r", "index-distal-r", "middle-proximal-r", "middle-middle-r", "middle-distal-r", "ring-proximal-r", "ring-middle-r", "ring-distal-r", "little-proximal-r", "little-middle-r", "little-distal-r"]);
const CHAIN = new Set(["arm-r", "forearm-r", "hand-r", "thumb-metacarpal-r", "thumb-proximal-r", "thumb-distal-r", "index-proximal-r", "index-middle-r", "index-distal-r", "middle-proximal-r", "middle-middle-r", "middle-distal-r", "ring-proximal-r", "ring-middle-r", "ring-distal-r", "little-proximal-r", "little-middle-r", "little-distal-r"]);

/** 💊️ A closed side profile symmetric about the bone axis: from u0 (radius r0, rounded when `round0`) to u1 (radius r1, rounded when `round1`). */
export function profile(u0: number, u1: number, r0: number, r1: number, round0: boolean, round1: boolean, n = 6): number[][] {
  const upper: number[][] = [];
  if (round0) for (let i = 0; i <= n; i++) { const f = Math.PI / 2 * (i / n); upper.push([u0 + 0.8 * r0 * (1 - Math.cos(f)), r0 * Math.sin(f)]); }
  else upper.push([u0, 0], [u0, r0]);
  if (round1) for (let i = n; i >= 0; i--) { const f = Math.PI / 2 * (i / n); upper.push([u1 - 0.8 * r1 * (1 - Math.cos(f)), r1 * Math.sin(f)]); }
  else upper.push([u1, r1], [u1, 0]);
  const top = upper.map(([u, v]) => [round(u!), round(v!)]);
  const lower = top.slice(1, -1).reverse().map(([u, v]) => [u!, -v!]);
  return top.concat(lower);
}

/** 🧮️ The open revolve profile [axial, radial] of the same body, closed through the axis by the kernel. */
function revolveProfile(points: number[][]): number[][] {
  const upper = points.filter((p) => p[1]! >= 0).map(([u, v]) => [u!, Math.abs(v!)]);
  upper.sort((p, q) => p[0]! - q[0]! || 0);
  const out: number[][] = [];
  for (const p of upper) if (!out.some((q) => Math.abs(q[0]! - p[0]!) < 1e-9 && Math.abs(q[1]! - p[1]!) < 1e-9)) out.push(p);
  if (out[0]![1] !== 0) out.unshift([out[0]![0]!, 0]);
  if (out[out.length - 1]![1] !== 0) out.push([out[out.length - 1]![0]!, 0]);
  return out;
}

const round = (x: number) => Math.round(x * 1e5) / 1e5;

/** 🔁️ Turns the shapes of the right arm chain into revolved solids of their side profile in every view, so they follow the rotated arm. */
function revolveArm(a: any): void {
  for (const scale of SCALES) {
    const g = a.representations[scale].geometry;
    const elevation: any[] = g.elevation.filter((s: any) => CHAIN.has(s.bone));
    for (const s of elevation) if (s.bone === "hand-r" && s.closed && s.filled) s.points = profile(0.05, 0.112, 0.034, 0.024, false, true, 4);
    for (const s of elevation) if (s.closed && s.filled) s.solid = { kind: "revolved", axisBone: s.bone, axis: "u", axisX: 0, axisZ: 0, mirror: true };
    g.elevation = g.elevation.filter((s: any) => !CHAIN.has(s.bone) || s.solid);
    const vol = a.volumes[scale];
    for (const s of elevation) {
      const i = vol.body.findIndex((x: any) => x.id === s.id && x.bone === s.bone && s.solid);
      if (i < 0) continue;
      const { id, bone, material, role, tone } = vol.body[i];
      vol.body[i] = { id, kind: "revolve", bone, material, role, tone, profile: revolveProfile(s.points), axis: [1, 0, 0], origin: [0, 0, 0] };
    }
    for (const view of ["front", "plan"]) {
      const old = new Map(g[view].filter((s: any) => CHAIN.has(s.bone)).map((s: any) => [s.id, s.z]));
      g[view] = g[view].filter((s: any) => !CHAIN.has(s.bone)).concat(elevation.filter((s) => s.solid).map((s) => ({ ...clone(s), z: old.get(s.id) ?? s.z })));
    }
  }
}

/** 🧵️ A strip polygon of half width r around a polyline (mitred). */
function strip(line: number[][], r: number): number[][] {
  const n = line.length, left: number[][] = [], right: number[][] = [];
  for (let i = 0; i < n; i++) {
    const p = line[i]!, a = line[Math.max(0, i - 1)]!, b = line[Math.min(n - 1, i + 1)]!;
    let dx = b[0]! - a[0]!, dy = b[1]! - a[1]!;
    const l = Math.hypot(dx, dy) || 1;
    dx /= l; dy /= l;
    left.push([round(p[0]! - dy * r), round(p[1]! + dx * r)]);
    right.push([round(p[0]! + dy * r), round(p[1]! - dx * r)]);
  }
  return left.concat(right.reverse());
}

/** 🎗️ The strap path in torso-local coordinates: two strands from the pouch ends up the chest and one round the back of the neck. */
function strapPaths(a: any): { id: string; points: V3[] }[] {
  const pose = a.poses.find((p: any) => p.id === a.presentation.defaultPose).bones;
  const f = frames(a, pose);
  const torso = f["torso"].frame, fore = f["forearm-r"].frame;
  const local = (p: V3) => toLocal(torso, p);
  const R = SLING.pouch.r + 0.004;
  const wristEnd = local(toWorld(fore, [0.21, R, 0]));
  const elbowEnd = local(toWorld(fore, [-0.03, R, 0]));
  const neck = f["neck"].frame;
  const n = (p: V3) => local(toWorld(neck, p));
  const leftNeck = n([0.03, 0.005, 0.085]), back = n([0.05, -0.085, 0]), rightNeck = n([0.03, 0.005, -0.085]);
  const mid = (p: V3, q: V3, lift: V3): V3 => [(p[0] + q[0]) / 2 + lift[0], (p[1] + q[1]) / 2 + lift[1], (p[2] + q[2]) / 2 + lift[2]];
  const chestL = mid(wristEnd, leftNeck, [0, 0.05, 0.01]);
  const chestR = mid(elbowEnd, rightNeck, [0, 0.06, -0.01]);
  const backL = mid(leftNeck, back, [0.01, -0.03, 0.03]);
  const backR = mid(rightNeck, back, [0.01, -0.03, -0.03]);
  return [
    { id: "sling-strap-front-l", points: [wristEnd, chestL, leftNeck] },
    { id: "sling-strap-neck", points: [leftNeck, backL, back, backR, rightNeck] },
    { id: "sling-strap-front-r", points: [rightNeck, chestR, elbowEnd] },
  ];
}

const STYLE = { pouch: { tone: 0.6, region: "sling", material: "sling-fabric" }, cast: { tone: 0.92, region: "cast", material: "fibreglass-cast" } };

/** 🩹️ Adds the sling posture, cast, pouch and strap to every scale, view and list of the asset. */
export function slingParts(a: any): void {
  revolveArm(a);
  const P = SLING.pouch, CF = SLING.castForearm, CH = SLING.castHand;
  const pouch = profile(P.u0, P.u1, P.r, P.r, true, false);
  const castF = profile(CF.u0, CF.u1, CF.r, CF.r, false, false);
  const castH = profile(CH.u0, CH.u1, CH.r0, CH.r1, false, true, 4);
  const castF500 = profile(CF.u0, CF.u1 + 0.122, CF.r, CH.r1, false, true, 4);
  const rest = restFrames(a.rig);
  const torsoRest = rest["torso"].frame;
  const straps = strapPaths(a);
  for (const scale of SCALES) {
    const rep = a.representations[scale];
    const g = rep.geometry;
    const coarse = scale === "500";
    const parts: any[] = [
      { id: "sling-pouch", bone: "forearm-r", z: 8.9, ...STYLE.pouch, points: pouch },
      coarse ? { id: "cast", bone: "forearm-r", z: 8.5, ...STYLE.cast, points: castF500 } : { id: "cast-forearm", bone: "forearm-r", z: 8.5, ...STYLE.cast, points: castF },
      ...(coarse ? [] : [{ id: "cast-hand", bone: "hand-r", z: 8.45, ...STYLE.cast, points: castH }]),
    ].map((p) => ({ id: p.id, bone: p.bone, role: "body", z: p.z, filled: true, tone: p.tone, region: p.region, material: p.material, solid: { kind: "revolved", axisBone: p.bone, axis: "u", axisX: 0, axisZ: 0, mirror: true }, kind: "poly", points: p.points, closed: true }));
    if (scale === "50") parts.push({ id: "cast-edge", bone: "forearm-r", role: "detail", z: 8.55, filled: false, tone: 0, region: "cast", material: "fibreglass-cast", solid: { kind: "revolved", axisBone: "forearm-r", axis: "u", axisX: 0, axisZ: 0, mirror: true }, kind: "poly", points: profile(CF.u1 - 0.004, CF.u1, CF.r + 0.0015, CF.r + 0.0015, false, false), closed: true });
    for (const view of VIEWS) g[view].push(...parts.map((p) => clone(p)));
    if (scale === "50" || scale === "100") {
      const r = SLING.strapR;
      for (const s of straps) {
        const world = s.points.map((p) => toWorld(torsoRest, p));
        const base = { bone: "torso", role: "body", filled: true, tone: 0.6, region: "sling", material: "sling-fabric", kind: "poly", closed: true };
        g.elevation.push({ id: s.id, ...base, z: 8.8, points: strip(s.points.map((p) => [p[0], p[1]]), r) });
        g.front.push({ id: s.id, ...base, z: 9.5, points: strip(world.map((p) => [p[2], p[1]]), r) });
        g.plan.push({ id: s.id, ...base, z: 9.5, points: strip(world.map((p) => [p[0], p[2]]), r) });
      }
    }
    const vol = a.volumes[scale];
    const solid = (id: string, bone: string, material: string, tone: number, prof: number[][]) => ({ id, kind: "revolve", bone, material, role: "body", tone, profile: revolveProfile(prof), axis: [1, 0, 0], origin: [0, 0, 0] });
    const body = [solid("sling-pouch", "forearm-r", "sling-fabric", 0.6, pouch), coarse ? solid("cast", "forearm-r", "fibreglass-cast", 0.92, castF500) : solid("cast-forearm", "forearm-r", "fibreglass-cast", 0.92, castF)];
    if (!coarse) body.push(solid("cast-hand", "hand-r", "fibreglass-cast", 0.92, castH));
    if (scale === "50" || scale === "100") for (const s of straps) for (let i = 0; i + 1 < s.points.length; i++) body.push({ id: `${s.id}-${i + 1}`, kind: "capsule", bone: "torso", material: "sling-fabric", role: "body", tone: 0.6, p0: s.points[i]!.map(round), p1: s.points[i + 1]!.map(round), r0: SLING.strapR, r1: SLING.strapR } as any);
    const fist = new Set(vol.body.filter((x: any) => x.material === "skin" && FINGERS.has(x.bone)).map((x: any) => x.id));
    vol.body = vol.body.filter((x: any) => !fist.has(x.id));
    vol.thermal = vol.thermal.filter((x: any) => !fist.has(x.of));
    if (!coarse) body.push(solid("fingers-r", "hand-r", "skin", 0.71, profile(0.05, 0.112, 0.034, 0.024, false, true, 4)));
    vol.body.push(...body);
    vol.thermal.push(...body.map((s: any) => ({ id: `th-${s.id}`, of: s.id, region: s.material === "sling-fabric" ? "sling" : s.material === "skin" ? "hand-skin" : "cast" })));
  }
}
