/** 🏭️ Writes the PW-04 street trees (catalog asset, manifest row, build report) in one run: `repo tools exec bun -- build.ts [ginkgo] [honey-locust] [sweetgum]` (all three without arguments).
 *
 * @see ./tree-asset.ts */
import { writeAll } from "./tree-asset.ts";
import ginkgo from "./ginkgo.ts";
import honeyLocust from "./honey-locust.ts";
import sweetgum from "./sweetgum.ts";

const all = { ginkgo, "honey-locust": honeyLocust, sweetgum };
const wanted = process.argv.slice(2).length > 0 ? process.argv.slice(2) : Object.keys(all);
for (const id of wanted) {
  const sp = all[id as keyof typeof all];
  if (sp === undefined) throw new Error(`unknown species ${id}; known: ${Object.keys(all).join(", ")}`);
  const report = writeAll(sp);
  console.log(`${id}: ${JSON.stringify(report.dimensions)} bones ${report.bones} solids ${JSON.stringify(report.solids)}`);
}
