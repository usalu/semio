/** 🪞️ The derived-view distances of assets, flagged or not: `bun derived.ts <tests-dir> <id[,id]> [file.json]` prints per authored scale the best transform and the normalised Hausdorff distance of front against elevation, plan against elevation and plan against front in the default pose (the matrix flags below 0.03), and the closest same view of another band (flagged below 0.005). A file replaces the catalog document of the single id. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const tests = resolve(process.argv[2]!);
const ids = process.argv[3]!.split(",");
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { bestTransform, hausdorff, pointSet } = await import(join(tests, "🔭️multiview-coverage", "🪞️derived.ts"));
const { evaluate } = await import(join(tests, "..", "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const replaced = process.argv[4] ? JSON.parse(readFileSync(resolve(process.argv[4]), "utf8")).asset : undefined;
for (const listed of engine.loadCatalog()) {
  if (!ids.includes(listed.id)) continue;
  const asset = replaced ?? listed;
  const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
  const sets: Record<string, any> = {};
  for (const key of authored) {
    const shapes = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key))).shapes;
    sets[key] = { elevation: pointSet(shapes.elevation), front: pointSet(shapes.front), plan: pointSet(shapes.plan) };
  }
  for (const key of authored) {
    const s = sets[key];
    const pair = (a: string, b: string): string => {
      const best = bestTransform(s[a], s[b]);
      return `${a}/${b} ${best.distance.toFixed(4)} ${best.name}${best.distance < 0.03 ? " FLAG" : ""}`;
    };
    const copies = authored.filter((other) => other !== key).flatMap((other) => (["front", "plan"] as const).map((view) => ({ other, view, d: hausdorff(s[view], sets[other][view]) }))).sort((a, b) => a.d - b.d)[0];
    console.log(`${asset.id} 1:${key} | ${pair("front", "elevation")} | ${pair("plan", "elevation")} | ${pair("plan", "front")} | nearest band copy ${copies ? `${copies.view} 1:${copies.other} ${copies.d.toFixed(4)}${copies.d < 0.005 ? " FLAG" : ""}` : "-"}`);
  }
}
