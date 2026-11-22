/** 🎯️ The playing hold of the standing violinist: head on the chin rest, violin frame from the jaw, the left hand on the neck and the bow arm solved through the library's `evaluate`.
 * Hold after the common teaching posture (violin about 40 degrees left of ahead, scroll slightly raised, top tilted towards the E side, bow at right angles to the strings between bridge and fingerboard). */
import { add, at, cross, dot, evalOffsets, frameOf, len, mul, rad, restState, solvePose, sub, unit, type Any, type Frame, type V3 } from "./lib.ts";

export const HEAD = { neck: { angleDeg: -0.8, twistDeg: 0.6, abductionDeg: 0.4 }, head: { angleDeg: -0.6, twistDeg: 0.6, abductionDeg: 0.4 } };
export const JAW: V3 = [-0.09, -0.045, 0.035];
export const CHINREST: V3 = [0.035, 0.0445, 0.04];
export const AZIMUTH = 40, ELEVATION = 8, ROLL = 32;

/** 🎻️ The violin frame in the world of a state whose head is in the hold. */
export function violinFrame(state: Any): Frame {
  const J = at(frameOf(state, "head"), ...JAW);
  const u = unit([Math.cos(rad(ELEVATION)) * Math.cos(rad(AZIMUTH)), Math.sin(rad(ELEVATION)), Math.cos(rad(ELEVATION)) * Math.sin(rad(AZIMUTH))]);
  const up: V3 = [0, 1, 0];
  const bass = unit(cross(u, up));
  const n0 = unit(cross(bass, u));
  const v = unit(add(mul(n0, Math.cos(rad(ROLL))), mul(bass, -Math.sin(rad(ROLL)))));
  const w = cross(u, v);
  const o = sub(J, add(mul(u, CHINREST[0]), add(mul(v, CHINREST[1]), mul(w, CHINREST[2]))));
  return { o, u, v, w };
}

/** 🔁️ Carries a world frame given relative to `bone` in `state` to the rest pose of the asset. */
export function toRest(asset: Any, state: Any, bone: string, f: Frame): Frame {
  const P = frameOf(state, bone), R = frameOf(restState(asset), bone);
  const local = (p: V3): V3 => [dot(p, P.u), dot(p, P.v), dot(p, P.w)];
  const world = (l: V3): V3 => add(mul(R.u, l[0]), add(mul(R.v, l[1]), mul(R.w, l[2])));
  return { o: add(R.o, world(local(sub(f.o, P.o)))), u: world(local(f.u)), v: world(local(f.v)), w: world(local(f.w)) };
}

/** 📍️ The string contact of the bow hair on the violin (A string by default) and the hair point of the bow at distance `s` from the button. */
export const STRINGS = { g: 0.0135, d: 0.0045, a: -0.0045, e: -0.0135 } as const;
export const contactOf = (V: Frame, string: keyof typeof STRINGS): V3 => at(V, 0.215, 0.0475, STRINGS[string]);
export const hairV = (s: number): number => 0.0175 - ((s - 0.046) / 0.682) * 0.009;

const soft = (x: number, lo: number, hi: number): number => (x < lo ? (lo - x) ** 2 : x > hi ? (x - hi) ** 2 : 0);

/** ✋️ Left hand on the neck in first position: the crook of thumb and index under the neck, fingers over the fingerboard. */
export function solveLeftHand(asset: Any, base: Record<string, Any>, V: Frame, start?: number[]): { bones: Record<string, Any>; residual: number } {
  const target = at(V, 0.445, 0.006, -0.004);
  const fingers = V.v;
  const palm = V.w;
  return solvePose(asset, base, [["arm-l", "angleDeg"], ["arm-l", "abductionDeg"], ["arm-l", "twistDeg"], ["forearm-l", "angleDeg"], ["forearm-l", "twistDeg"], ["hand-l", "angleDeg"], ["hand-l", "abductionDeg"]], start ?? [60, -20, 30, 90, 60, 10, 0], (s, x) => {
    const H = frameOf(s, "hand-l");
    const g = at(H, 0.088, 0.022, -0.012);
    return 400 * len(sub(g, target)) ** 2 + 0.6 * (1 - dot(H.u, fingers)) + 0.3 * (1 - dot(mul(H.w, -1), palm)) + 1e-3 * (soft(x[0]!, -55, 165) + soft(x[3]!, 2, 148) + soft(x[5]!, -30, 18) + soft(x[6]!, -65, 65)) + 1e-6 * x.reduce((a, b) => a + b * b, 0);
  });
}

/** 🏹️ Bow arm: the hair at distance `s` from the button on the string contact, the bow at right angles to the strings, hair towards the strings. */
export function solveBowArm(asset: Any, base: Record<string, Any>, V: Frame, s: number, string: keyof typeof STRINGS, start?: number[]): { bones: Record<string, Any>; residual: number } {
  const P = contactOf(V, string);
  const tilt = { g: 14, d: 5, a: -5, e: -14 }[string];
  const dir = unit(add(mul(V.w, Math.cos(rad(tilt))), mul(V.v, Math.sin(rad(tilt)))));
  const hairTo = cross(V.u, dir);
  return solvePose(asset, base, [["arm-r", "angleDeg"], ["arm-r", "abductionDeg"], ["arm-r", "twistDeg"], ["forearm-r", "angleDeg"], ["forearm-r", "twistDeg"], ["hand-r", "angleDeg"], ["hand-r", "abductionDeg"], ["bow", "angleDeg"], ["bow", "abductionDeg"]], start ?? [50, -30, -30, 80, -40, 0, 0, 0, 0], (st, x) => {
    const Bf = frameOf(st, "bow");
    const h = at(Bf, s, hairV(s), 0);
    return 400 * len(sub(h, P)) ** 2 + 6 * (1 - dot(Bf.u, dir)) + 0.5 * (1 - dot(Bf.v, hairTo)) + 1e-3 * (soft(x[0]!, -55, 165) + soft(x[3]!, 2, 148) + soft(x[5]!, -30, 18) + soft(x[6]!, -65, 65) + soft(x[7]!, -25, 25) + soft(x[8]!, -25, 25)) + 1e-7 * x.reduce((a, b) => a + b * b, 0) + 1e-5 * (x[7]! ** 2 + x[8]! ** 2);
  });
}

/** 🔎️ How well a solved hold sits: left grip distance (mm), bow hair distance to the string (mm) and bow angle off the string normal (degrees). */
export function holdReport(asset: Any, bones: Record<string, Any>, V: Frame, s: number, string: keyof typeof STRINGS): string {
  const st = evalOffsets(asset, bones);
  const H = frameOf(st, "hand-l");
  const left = len(sub(at(H, 0.088, 0.022, -0.012), at(V, 0.445, 0.006, -0.004))) * 1000;
  const Bf = frameOf(st, "bow");
  const hair = len(sub(at(Bf, s, hairV(s), 0), contactOf(V, string))) * 1000;
  const angle = (Math.acos(Math.min(1, Math.abs(dot(Bf.u, V.u)))) * 180) / Math.PI;
  return `left ${left.toFixed(1)} mm, hair ${hair.toFixed(1)} mm, bow to strings ${angle.toFixed(1)} deg`;
}
