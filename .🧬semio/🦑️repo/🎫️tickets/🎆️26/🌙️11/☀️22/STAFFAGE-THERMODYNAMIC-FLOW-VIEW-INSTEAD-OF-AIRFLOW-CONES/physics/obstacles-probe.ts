/** 🔎️ Prints the obstacles of catalog assets or fixtures at their default pose: kind, exponent, blockage, centre and radii, and the time the grouping and the posing take. Usage: bun obstacles-probe.ts <path under S>... */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const S = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
for (const path of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(join(S, path, "🔣️.json"), "utf8")).asset;
  let t = performance.now();
  const clusters = transport.bodyClusters(asset);
  const grouped = performance.now() - t;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
  t = performance.now();
  const obstacles = transport.obstaclesOf(state);
  console.log(path, `clusters ${clusters.length} in ${grouped.toFixed(0)} ms, posed in ${(performance.now() - t).toFixed(2)} ms`);
  for (const o of obstacles) console.log(`  ${o.id.padEnd(28)} ${o.kind.padEnd(8)} p=${o.exponent} phi=${o.blockage.toFixed(2)} c=(${o.centre.x.toFixed(2)}, ${o.centre.y.toFixed(2)}, ${o.centre.z.toFixed(2)}) r=(${o.radiiM.map((r: number) => r.toFixed(3)).join(", ")})`);
}
