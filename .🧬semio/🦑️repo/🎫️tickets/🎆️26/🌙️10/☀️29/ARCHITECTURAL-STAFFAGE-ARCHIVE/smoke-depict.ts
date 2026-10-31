/** 🧍️ Depicts one authored asset per category at 1:100 in every view, the same path the website uses. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { defaultDepictionOptions, depict, evaluate, solveStructure, solveThermal, validateAsset, VIEWS, type AssetDocument, type View } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const skip = new Set(["🌤️environments", "🖨️print-profiles"]);

const samples = readdirSync(catalog, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !skip.has(entry.name)).flatMap((category) => {
  const folder = readdirSync(join(catalog, category.name), { withFileTypes: true }).find((entry) => entry.isDirectory());
  if (folder === undefined) return [];
  const file = readdirSync(join(catalog, category.name, folder.name)).find((name) => name.endsWith(".json"));
  return file === undefined ? [] : [join(catalog, category.name, folder.name, file)];
});

let failed = 0;
for (const path of samples) {
  const document = JSON.parse(readFileSync(path, "utf8")) as AssetDocument;
  const asset = document.asset;
  const issues = validateAsset(asset);
  const pose = asset.poses[0];
  if (pose === undefined) {
    console.log(`FAIL ${asset.id} no pose`);
    failed += 1;
    continue;
  }
  try {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: 100 } });
    const drawn: View[] = [];
    for (const view of VIEWS) {
      const options = defaultDepictionOptions(view, "en", 100);
      const thermal = view === "thermal" ? solveThermal(state) : undefined;
      const structure = view === "forces" || view === "moments" ? solveStructure(state) : undefined;
      const scene = depict(state, options, { ...(thermal === undefined ? {} : { thermal }), ...(structure === undefined ? {} : { structure }) });
      if (scene.layers.length === 0) throw new Error(`${view} drew no layers`);
      drawn.push(view);
    }
    console.log(`OK ${asset.id} issues=${issues.length} layers-views=${drawn.join(",")}`);
  } catch (error) {
    failed += 1;
    console.log(`FAIL ${asset.id} issues=${issues.length} ${(error instanceof Error ? error.message : String(error)).split("\n")[0]}`);
  }
}
if (failed > 0) process.exitCode = 1;
