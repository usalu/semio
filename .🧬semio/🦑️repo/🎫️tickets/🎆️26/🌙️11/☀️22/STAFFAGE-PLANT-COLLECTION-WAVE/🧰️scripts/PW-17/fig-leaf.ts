/** 🎻️ PW-17 the leaf of Ficus lyrata in [u, w] of its blade bone (u from the petiole tip at the cordate base to the apex, w across): the fiddle (panduriform) outline from a hand-drawn control table (broad apex, waist at 0.38 of the length, small basal lobes), the emarginate apex notch at 1:50, simplified per scale; at 1:200 and 1:500 the blade runs back as a stalk over its petiole to the trunk.
 * @see ./fiddle-leaf-fig.ts */
import type { P, Scale } from "./lib.ts";

/** ✏️ Control table of the right half: (u / L, w / width), drawn by hand from adult leaves (cordate base, waist, broadest at 0.75 of the length, rounded apex). */
const HALF: [number, number][] = [
  [0, 0], [-0.03, -0.1], [-0.02, -0.25], [0.04, -0.37], [0.12, -0.43], [0.25, -0.41], [0.38, -0.37], [0.52, -0.43], [0.65, -0.49], [0.76, -0.51], [0.86, -0.47], [0.93, -0.36], [0.975, -0.2], [0.995, -0.07], [1, 0],
];

/** 🍃️ The outline of a fig blade of length L and width W at a scale; `stalk` (m) adds the petiole as a 14 mm strip back to the trunk at the coarse scales. */
export function figBlade(L: number, W: number, scale: Scale, stalk = 0): P[] {
  const keep = scale === 50 ? HALF : scale === 100 ? HALF.filter((_, i) => i !== 2 && i !== 13) : scale === 200 ? HALF.filter((_, i) => [0, 1, 3, 4, 6, 8, 9, 11, 14].includes(i)) : HALF.filter((_, i) => [0, 3, 6, 9, 11, 14].includes(i));
  const right = keep.map(([u, w]) => [u * L, w * W] as P);
  const apex: P[] = scale === 50 ? [[0.99 * L, -0.035 * W], [0.975 * L, 0], [0.99 * L, 0.035 * W]] : [[L, 0]];
  const body = [...right.slice(1, -1), ...apex, ...right.slice(1, -1).map(([u, w]) => [u, -w] as P).reverse()];
  if (stalk <= 0) return [[0, 0], ...body.slice(0, body.length)];
  const s = 0.007;
  return [[-stalk, -s], [0.02 * L, -s], ...body, [0.02 * L, s], [-stalk, s]];
}
