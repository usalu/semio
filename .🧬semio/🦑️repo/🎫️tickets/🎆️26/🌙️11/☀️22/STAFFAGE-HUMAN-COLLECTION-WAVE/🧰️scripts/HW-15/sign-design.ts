/** 🤟️ Design inputs of `person-signing`: hand shapes (relaxed, flat B, index point) as finger offsets, and the wrist, finger and palm targets of the sign poses in world metres of the relaxed stance (x ahead, y up, z left), solved by arm inverse kinematics.
 * Signs: HOUSE (DGS HAUS, ASL HOUSE: two flat hands form the roof, then the walls), THANK YOU (DGS DANKE, ASL THANK-YOU: flat hand from the chin forward and down), YOU (index points at the addressee). Signing space after Klima and Bellugi (1979) and Boyes Braem (1995): from the waist to the top of the head, about a forearm ahead of the trunk.
 * @see ./gen-signing.ts
 */
import type { ArmTarget } from "./ik.ts";

const FINGERS = ["index", "middle", "ring", "little"] as const;

/** ✋️ Finger offsets of one hand: per finger the flexion of the proximal (as `abductionDeg` under the twist of ±90 that sets its flexion axis), middle and distal phalanx, and the thumb. */
export function hand(side: "l" | "r", shape: "relaxed" | "flat" | "point" | "fist"): Record<string, any> {
  const m = side === "r" ? 1 : -1, tw = -90 * m;
  const flex: Record<string, [number, number, number]> =
    shape === "flat" ? { index: [0, 2, 1], middle: [0, 2, 1], ring: [1, 3, 1], little: [2, 4, 2] }
    : shape === "point" ? { index: [-2, 3, 1], middle: [62, 76, 38], ring: [68, 82, 43], little: [73, 85, 47] }
    : shape === "fist" ? { index: [70, 88, 45], middle: [74, 90, 46], ring: [76, 90, 46], little: [78, 88, 45] }
    : { index: [7, 15, 10], middle: [8, 17, 11], ring: [9, 19, 12], little: [10, 21, 12] };
  const o: Record<string, any> = {};
  for (const f of FINGERS) {
    const [p, mid, d] = flex[f]!;
    o[`${f}-proximal-${side}`] = { angleDeg: 0, abductionDeg: p * m, twistDeg: tw };
    o[`${f}-middle-${side}`] = { angleDeg: mid };
    o[`${f}-distal-${side}`] = { angleDeg: d };
  }
  const thumb = shape === "flat" ? [[4, 4, 0], [-2, 6], [-1, 3]] : shape === "point" || shape === "fist" ? [[-18, 32, 0], [28, 16], [32, 6]] : [[0, 9.8, 0], [-1, 16], [-1.6, 6]];
  o[`thumb-metacarpal-${side}`] = { angleDeg: thumb[0]![0], abductionDeg: thumb[0]![1]! * m, twistDeg: tw };
  o[`thumb-proximal-${side}`] = { angleDeg: thumb[1]![0], abductionDeg: thumb[1]![1]! * m };
  o[`thumb-distal-${side}`] = { angleDeg: thumb[2]![0], abductionDeg: thumb[2]![1]! * m };
  return o;
}

const mirror = (t: ArmTarget): ArmTarget => ({ wrist: [t.wrist[0], t.wrist[1], -t.wrist[2]], fingers: t.fingers && [t.fingers[0], t.fingers[1], -t.fingers[2]], palm: t.palm && [t.palm[0], t.palm[1], -t.palm[2]] });

/** 🎯️ Right-hand targets; the left hand mirrors them unless stated. */
const READY: ArmTarget = { wrist: [0.25, 1.15, -0.16], fingers: [0.82, 0.3, 0.48], palm: [-0.1, 0.45, 1] };
const ROOF: ArmTarget = { wrist: [0.31, 1.27, -0.135], fingers: [0.1, 0.7, 0.7], palm: [0.05, -0.75, 0.6] };
const WALLS: ArmTarget = { wrist: [0.31, 1.09, -0.13], fingers: [0.15, 0.98, 0.05], palm: [0, 0, 1] };
const CHIN: ArmTarget = { wrist: [0.2, 1.39, -0.05], fingers: [-0.18, 0.96, 0.2], palm: [-1, -0.2, 0.1] };
const OUT: ArmTarget = { wrist: [0.38, 1.17, -0.11], fingers: [0.85, 0.35, 0.15], palm: [0, 1, 0.25] };
const YOU: ArmTarget = { wrist: [0.32, 1.24, -0.13], fingers: [1, 0.06, 0.08], palm: [0, -0.5, 0.85] };

export type SignPose = { id: string; key?: boolean; right: ArmTarget; left: ArmTarget; hands: [string, string]; head?: number; neck?: number; torso?: number; met: number };

/** 📜️ The named and key poses of the signing person (head, neck and torso `angleDeg` offsets added to the relaxed stance). */
export const SIGN_POSES: SignPose[] = [
  { id: "signing-ready", right: READY, left: mirror(READY), hands: ["relaxed", "relaxed"], head: 2, met: 1.5 },
  { id: "sign-house", right: ROOF, left: mirror(ROOF), hands: ["flat", "flat"], head: -2, neck: 1, met: 1.6 },
  { id: "sign-thank-you", right: CHIN, left: mirror(READY), hands: ["flat", "relaxed"], head: 3, met: 1.5 },
  { id: "sign-you", right: YOU, left: mirror(READY), hands: ["point", "relaxed"], head: 1, torso: 1, met: 1.5 },
  { id: "sign-house-walls", key: true, right: WALLS, left: mirror(WALLS), hands: ["flat", "flat"], head: 0, met: 1.6 },
  { id: "sign-thank-you-out", key: true, right: OUT, left: mirror(READY), hands: ["flat", "relaxed"], head: 4, torso: 0.5, met: 1.5 },
];

/** 🎞️ The looping phrase HOUSE - THANK YOU - YOU: key times in seconds and their poses. */
export const PHRASE: [number, string][] = [
  [0, "signing-ready"], [0.7, "sign-house"], [1.25, "sign-house"], [1.75, "sign-house-walls"], [2.35, "signing-ready"],
  [2.95, "sign-thank-you"], [3.35, "sign-thank-you"], [3.85, "sign-thank-you-out"], [4.45, "sign-you"], [4.95, "sign-you"],
];
export const PHRASE_S = 5.6;
