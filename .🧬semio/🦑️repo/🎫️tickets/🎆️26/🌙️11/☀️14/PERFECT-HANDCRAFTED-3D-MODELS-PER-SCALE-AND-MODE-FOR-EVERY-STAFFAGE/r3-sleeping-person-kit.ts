/** 🧺️ Constructors of the round 3 sleeping person on top of the seated office worker's kit: every solid can be stated in world coordinates of the supine pose (x along the bed, y up, z to her left) and is carried into the frame of its bone through the solved supine frames, so the bedding and the body land exactly where the reference puts them while they still ride on their bones in every other pose. Adds the half revolve that models a duvet mound with its flat face on the mattress (blended between its bone and the duvet), world lofts with sections across a world axis, and world bars.
 * @see ./r3-seated-office-worker-kit.ts
 * @see ./r3-sleeping-person-roots.ts */
import { Kit as BaseKit, type Json, type Opts, type Row, type V3 } from "./r3-seated-office-worker-kit.ts";
import { framesOf, local, rooted, type Frame } from "./r3-sleeping-person-roots.ts";

export { ACROSS_U, ACROSS_V, dome, inset, mirror, parts, thermalOf, aboutU, aboutV, aboutW, quat, type Json, type Row, type V3 } from "./r3-seated-office-worker-kit.ts";

const DOCUMENT = rooted([]);
export const ASSET_ROOTED: Json = DOCUMENT.asset;
const SUPINE = (ASSET_ROOTED.poses as Json[]).find((p) => p.id === "supine")!;
/** 🛌️ The world frames of every bone in the supine pose (the frames the solids are authored in). */
export const POSE: Record<string, Frame> = framesOf(ASSET_ROOTED.rig, SUPINE.bones);

const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
/** 📍️ A world point in the frame of `bone`. */
export const W = (bone: string, p: V3): V3 => local(POSE[bone]!, p).map(r5) as unknown as V3;
/** ➡️ A world direction in the frame of `bone`. */
export const D = (bone: string, d: V3): V3 => {
  const f = POSE[bone]!;
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
};
/** 🌍️ A point of the frame of `bone` in the world. */
export const Wx = (bone: string, p: V3): V3 => {
  const f = POSE[bone]!;
  return [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as unknown as V3;
};

/** 🧮️ The unit quaternion `[x, y, z, w]` of the rotation whose matrix has the columns `a`, `b`, `c` (orthonormal, right-handed). */
export function quatOf(a: V3, b: V3, c: V3): number[] {
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]];
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
  const n = Math.hypot(...q);
  return q.map((x) => r5(x / n));
}
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V3): V3 => {
  const n = Math.hypot(...a);
  return [a[0] / n, a[1] / n, a[2] / n];
};
/** 🧭️ The section rotation, in the frame of `bone`, of a section that lies across the world direction `along` with its x axis (rx) as close as possible to the world direction `xHint`. */
export function across(bone: string, along: V3, xHint: V3 = [0, 0, 1]): number[] {
  const n = unit(D(bone, along));
  const h = D(bone, xHint);
  const k = dot(h, n);
  const x = unit([h[0] - k * n[0], h[1] - k * n[1], h[2] - k * n[2]]);
  return quatOf(x, cross(n, x), n);
}

/** 🌐️ A world row of a loft: centre `[x, y, z]`, `rx` along the section's x axis, `ry` along its y axis, exponent. */
export type WRow = readonly [x: number, y: number, z: number, rx: number, ry: number, e?: number];

/** 🪢️ How much of its bone's motion a duvet mound follows; the rest stays with the duvet on the bed, so the cloth stays on top when she rolls and rises over the knees when she draws them up. */
const MOUND_FOLLOW: Record<string, number> = { torso: 0.3, "thigh-r": 0.5, "shank-r": 0.5 };

/** 🗂️ One list under construction, with the world constructors. */
export class Kit extends BaseKit {
  /** 🌍️ A loft of world rows whose sections lie across the world direction `along` (rx along `xHint`). */
  wloft(id: string, bone: string, material: string, tone: number, rows: readonly WRow[], region?: string, o: Opts & { along?: V3; xHint?: V3 } = {}): Json {
    const rotation = across(bone, o.along ?? [1, 0, 0], o.xHint ?? [0, 0, 1]);
    const solid = this.loft(id, bone, material, tone, [], region, o);
    solid.sections = rows.map(([x, y, z, rx, ry, e]) => ({ at: W(bone, [x, y, z]), rx: r5(rx), ry: r5(ry), exponent: e ?? o.e ?? 2, rotation }));
    return solid;
  }
  /** 🧷️ A loft through world rows whose sections stand perpendicular to the path (bars, hems, rolls): `[x, y, z, rx, ry?]`. */
  wbar(id: string, bone: string, material: string, tone: number, points: readonly (readonly number[])[], region?: string, o: Opts = {}): Json {
    return this.bar(id, bone, material, tone, points.map(([x, y, z, rx, ry]) => [...W(bone, [x!, y!, z!]), rx!, ry ?? rx!]), region, o);
  }
  /** 🥖️ A half revolve whose flat face lies in the horizontal plane `y0` (the mattress): its axis runs along world x at lateral `zc`, `profile` is `[world x, half-width]`, the height is `lateral` times the half-width. */
  mound(id: string, bone: string, material: string, tone: number, profile: readonly (readonly [number, number])[], y0: number, zc: number, lateral: number, region?: string, o: Opts = {}, follow: number | undefined = MOUND_FOLLOW[bone]): Json {
    const rotation = quatOf(unit(D(bone, [1, 0, 0])), unit(D(bone, [0, -1, 0])), unit(D(bone, [0, 0, -1])));
    const solid = this.rev(id, bone, material, tone, profile.map(([x, r]) => [x, r * lateral] as const), [1, 0, 0], [0, 0, 0], region, { ...o, lateral: 1 / lateral, rotation, translation: W(bone, [0, y0, zc]) });
    solid.sweepRad = Math.PI;
    if (follow !== undefined) solid.weights = [{ bone, weight: follow }, { bone: "duvet", weight: Math.round((1 - follow) * 100) / 100 }];
    return solid;
  }
  /** 🌍️ An ellipsoid at a world point with semi-axes along the world axes (or turned by `rotation` given in the frame of `bone`). */
  well(id: string, bone: string, material: string, tone: number, p: V3, radii: V3, region?: string, o: Opts & { worldAxes?: boolean } = {}): Json {
    const solid = this.ell(id, bone, material, tone, W(bone, p), radii, region, o);
    if (o.rotation === undefined && o.worldAxes !== false) solid.transform.rotation = quatOf(unit(D(bone, [1, 0, 0])), unit(D(bone, [0, 1, 0])), unit(D(bone, [0, 0, 1])));
    return solid;
  }
  /** 💊️ A capsule between two world points. */
  wcap(id: string, bone: string, material: string, tone: number, p0: V3, p1: V3, r0: number, r1: number, region?: string, o: Opts = {}): Json {
    return this.cap(id, bone, material, tone, W(bone, p0), W(bone, p1), r0, r1, region, o);
  }
}
export type { Opts };
