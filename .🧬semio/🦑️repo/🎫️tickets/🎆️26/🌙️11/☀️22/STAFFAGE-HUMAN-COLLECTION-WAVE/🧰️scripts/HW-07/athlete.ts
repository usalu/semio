/** 🦵️ Inverse-kinematics goal builders of the HW-07 people (all three share the rig of the catalog jogger): feet planted flat or on the balls,
 * arm variables, and the seed that bends knees and elbows so the solver never starts on a straight-limb singularity.
 * @see ./lib.ts — solvePose */
import type { Goal, Json, Offsets, Variable } from "./lib.ts";

export type V3 = [number, number, number];
export const anchorPoint = (a: Json, id: string): V3 => { const x = a.anchors.find((y: Json) => y.id === id); return [x.x, x.y, x.z ?? 0]; };

/** 🦶️ Both feet flat on the ground at `y`, `half` to each side, toes turned out by `toeOutDeg`, the foot centres at `footX` (left) and `footXr` (right). */
export function flatFeet(a: Json, half: number, toeOutDeg: number, footX: number, footXr = footX, y = 0, halfR = half): Goal[] {
  return (["l", "r"] as const).flatMap((side) => {
    const z = side === "l" ? half : -halfR, t = ((side === "l" ? toeOutDeg : -toeOutDeg) * Math.PI) / 180, x = side === "l" ? footX : footXr;
    const heel = anchorPoint(a, `heel-${side}`), ball = anchorPoint(a, `contact-${side}`), len = ball[0] - heel[0];
    return [
      { kind: "point", bone: `foot-${side}`, p: heel, target: [x - (len / 2) * Math.cos(t), y, z - (len / 2) * Math.sin(t)], weight: 4 },
      { kind: "point", bone: `foot-${side}`, p: ball, target: [x + (len / 2) * Math.cos(t), y, z + (len / 2) * Math.sin(t)], weight: 4 },
      { kind: "axis", bone: `foot-${side}`, axis: "v", dir: [0, 1, 0], weight: 2 },
    ] as Goal[];
  });
}
/** 🩰️ One foot on its ball: the ball of the foot at `at`, the forefoot flat, the heel raised so the foot slopes by `heelDeg`. */
export function onBall(a: Json, side: "l" | "r", at: V3, heelDeg: number, toeOutDeg = 0): Goal[] {
  const t = ((side === "l" ? toeOutDeg : -toeOutDeg) * Math.PI) / 180, h = (heelDeg * Math.PI) / 180;
  return [
    { kind: "point", bone: `foot-${side}`, p: anchorPoint(a, `contact-${side}`), target: at, weight: 4 },
    { kind: "axis", bone: `toe-${side}`, axis: "v", dir: [0, 1, 0], weight: 2 },
    { kind: "axis", bone: `foot-${side}`, axis: "u", dir: [Math.cos(h) * Math.cos(t), -Math.sin(h), Math.cos(h) * Math.sin(t)], weight: 2 },
  ];
}
export const legVars = (sides: readonly ("l" | "r")[] = ["l", "r"], toes = false): Variable[] => sides.flatMap((side) => [[`thigh-${side}`, "angleDeg"], [`thigh-${side}`, "abductionDeg"], [`thigh-${side}`, "twistDeg"], [`shank-${side}`, "angleDeg"], [`foot-${side}`, "angleDeg"], [`foot-${side}`, "abductionDeg"], [`foot-${side}`, "twistDeg"], ...(toes ? [[`toe-${side}`, "angleDeg"]] : [])] as Variable[]);
/** 💪️ The arm chain of one side; the clavicle is stiff (`clavicleStiffness` times the pull of the others), so the shoulder only shrugs where a reach needs it. */
export const armVars = (side: "l" | "r", wristTwist = false, clavicleStiffness = 25): Variable[] => [[`clavicle-${side}`, "angleDeg", clavicleStiffness], [`upper-arm-${side}`, "angleDeg"], [`upper-arm-${side}`, "abductionDeg"], [`upper-arm-${side}`, "twistDeg"], [`forearm-${side}`, "angleDeg"], [`forearm-${side}`, "twistDeg"], [`hand-${side}`, "angleDeg"], [`hand-${side}`, "abductionDeg"], ...(wristTwist ? [[`hand-${side}`, "twistDeg"]] : [])] as Variable[];
/** 🌱️ Bent knees and elbows as the start of every solve. */
export const SEED: Offsets = { "thigh-l": { angleDeg: 35 }, "shank-l": { angleDeg: -60 }, "foot-l": { angleDeg: 25 }, "thigh-r": { angleDeg: 35 }, "shank-r": { angleDeg: -60 }, "foot-r": { angleDeg: 25 }, "upper-arm-r": { angleDeg: 20 }, "forearm-r": { angleDeg: 80 }, "upper-arm-l": { angleDeg: 20 }, "forearm-l": { angleDeg: 80 } };
