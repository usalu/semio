/** 🧭️ Rest frames of the round 3 seated office worker (the rig with the root fixes) and the conversion of a world point into the frame of a bone, so that furniture parts can be stated in world coordinates (x ahead, y up, z to the left) and still ride on their bones.
 * @see ./r3-seated-office-worker-roots.ts */
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { rooted, type Frame } from "./r3-seated-office-worker-roots.ts";
import type { V3 } from "./r3-seated-office-worker-ref.ts";

export const REST: Record<string, Frame> = Object.fromEntries(Object.entries(restFrames(rooted([]).asset.rig) as Record<string, { frame: Frame }>).map(([id, b]) => [id, b.frame]));

/** 📍️ A world point in the rest frame of `bone` as `[u, v, w]`. */
export function local(bone: string, p: V3): [number, number, number] {
  const f = REST[bone]!;
  const d = [p[0] - f.o[0], p[1] - f.o[1], p[2] - f.o[2]];
  const dot = (a: V3): number => d[0]! * a[0] + d[1]! * a[1] + d[2]! * a[2];
  return [dot(f.u), dot(f.v), dot(f.w)];
}
/** ➡️ A world direction in the rest frame of `bone`. */
export function along(bone: string, d: V3): [number, number, number] {
  const f = REST[bone]!;
  const dot = (a: V3): number => d[0] * a[0] + d[1] * a[1] + d[2] * a[2];
  return [dot(f.u), dot(f.v), dot(f.w)];
}

/** 🎯️ The rotation `[x, y, z, w]` that lays the local u axis of a solid on bone `bone` along the world direction `dir`, keeping its local v as close to world up as possible. */
export function aim(bone: string, dir: V3): number[] {
  const n = (a: readonly number[]): number[] => {
    const l = Math.hypot(...a);
    return a.map((x) => x / l);
  };
  const u = n(along(bone, dir));
  const up = along(bone, [0, 1, 0]);
  const k = up[0] * u[0]! + up[1] * u[1]! + up[2] * u[2]!;
  const v = n([up[0] - k * u[0]!, up[1] - k * u[1]!, up[2] - k * u[2]!]);
  const w = [u[1]! * v[2]! - u[2]! * v[1]!, u[2]! * v[0]! - u[0]! * v[2]!, u[0]! * v[1]! - u[1]! * v[0]!];
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [u[0]!, u[1]!, u[2]!, v[0]!, v[1]!, v[2]!, w[0]!, w[1]!, w[2]!];
  const tr = m00 + m11 + m22;
  let q: number[];
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4];
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s];
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s];
  }
  return q.map((x) => Math.round(x * 1e5) / 1e5);
}
