/** 🌳️ Repair of `park-bench` (side elevation of 1:500 agrees with the body by 0.903, below 0.92): the middle and the top back slat were drawn 71 mm high and 53 mm apart, the solids `back-slat-2` and `back-slat-3` of 1:500 are 82 mm high and 28 mm apart. The two slats of the elevation path `three-back-slats` take the heights of their solids (0.6535 to 0.7359 m and 0.7644 to 0.8468 m, the bone `backrest` stands at 0.677 m).
 *  bun fix-park-bench.ts [--file <asset.json>] [--dry] */
import { run, shapeOf } from "./lib.ts";

await run("park-bench", (asset) => {
  shapeOf(asset, 500, "elevation", "three-back-slats").d = "M 0.0 -0.1878 L 1.8 -0.1878 L 1.8 -0.1037 L 0.0 -0.1037 Z M 0.0 -0.0235 L 1.8 -0.0235 L 1.8 0.0589 L 0.0 0.0589 Z M 0.0 0.0874 L 1.8 0.0874 L 1.8 0.1698 L 0.0 0.1698 Z";
});
