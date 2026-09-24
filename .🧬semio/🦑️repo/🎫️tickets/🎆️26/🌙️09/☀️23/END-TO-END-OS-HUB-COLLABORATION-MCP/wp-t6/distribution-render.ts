/** 🖨️ Compiles every catalogue variant of the eight charts-distribution families in render mode and reports the geometry record counts per kind. */
import { compileVizProbe, probeProjection } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts";
import { join } from "node:path";
const records = await compileVizProbe(join(import.meta.dir, "distribution-render.tex"), { workDir: join(import.meta.dir, "generated", "render-work"), caseName: "distribution-render", scenario: "all", keepWorkDir: true });
const projection = probeProjection(records, "all");
for (const [key, values] of Object.entries(projection)) console.log(key, values.length);
