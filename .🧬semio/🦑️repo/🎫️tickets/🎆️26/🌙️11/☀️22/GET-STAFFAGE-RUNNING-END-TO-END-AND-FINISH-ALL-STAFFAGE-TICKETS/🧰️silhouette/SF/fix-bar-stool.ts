/** 🪑️ Repair of `bar-stool`: the plan of the back pad is the plan of its solid. The pad is a loft bent on a 0.30 m radius (bone `backrest`), the plan drew it as the straight prism of its elevation profile (`solid` extruded ±0.150 m), so the two openings between pad and cushion and the bowed rear edge were missing: 58.9 mm largest boundary distance at 1:50 (limit 15 mm). The plan shape `back-pad` of 1:50, 1:100 and 1:200 becomes the outline of the loft of that scale (numbers of `outline.ts bar-stool <scale> plan back-pad`).
 *  bun fix-bar-stool.ts [--file <asset.json>] [--dry] */
import { asPoly, run, shapeOf } from "./lib.ts";

const mirror = (half: readonly (readonly [number, number])[]): [number, number][] => [...half.map(([x, y]) => [x, y] as [number, number]), ...[...half].reverse().filter(([, y]) => y !== 0).map(([x, y]) => [x, -y] as [number, number])];
const PAD: Record<number, readonly (readonly [number, number])[]> = {
  50: [[-0.212, 0], [-0.20993, -0.03591], [-0.20376, -0.07135], [-0.19355, -0.10586], [-0.17974, -0.13837], [-0.17609, -0.14316], [-0.17027, -0.14562], [-0.16507, -0.14514], [-0.15965, -0.1422], [-0.15635, -0.13822], [-0.15497, -0.13226], [-0.1559, -0.12799], [-0.16909, -0.09706], [-0.17844, -0.06543], [-0.18411, -0.03293], [-0.186, 0]],
  100: [[-0.212, 0], [-0.20376, -0.07135], [-0.18002, -0.13768], [-0.17609, -0.14316], [-0.17027, -0.14562], [-0.16507, -0.14514], [-0.15965, -0.1422], [-0.15635, -0.13822], [-0.15497, -0.13226], [-0.15562, -0.12868], [-0.17844, -0.06543], [-0.186, 0]],
  200: [[-0.2103, 0.04811], [-0.2103, -0.04811], [-0.18208, -0.13783], [-0.17786, -0.14407], [-0.17203, -0.14656], [-0.16682, -0.14609], [-0.1579, -0.14125], [-0.15459, -0.13728], [-0.1532, -0.13135], [-0.15356, -0.12853], [-0.18066, -0.04353], [-0.18066, 0.04353]],
};

await run("bar-stool", (asset) => {
  for (const scale of [50, 100]) asPoly(shapeOf(asset, scale, "plan", "back-pad"), mirror(PAD[scale]!));
  asPoly(shapeOf(asset, 200, "plan", "back-pad"), [...PAD[200]!, ...[...PAD[200]!].slice(2, -2).reverse().map(([x, y]) => [x, -y] as [number, number])]);
});
