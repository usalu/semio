import { appendFileSync, writeFileSync } from "node:fs";
import { authoredScales, catalog, queries } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const out = process.argv[2]!;
writeFileSync(out, "");
const only = new Set(process.argv.slice(3));
for (const asset of catalog()) {
  if (only.size > 0 && !only.has(asset.id)) continue;
  let worst = 0;
  let where = "";
  for (const scale of authoredScales(asset))
    for (const { name, query } of queries(asset)) {
      appendFileSync(out, `START ${asset.id} 1:${scale} ${name}\n`);
      const t = performance.now();
      evaluate(asset, query, { kind: "scale", scale: { denominator: scale } });
      const d = performance.now() - t;
      if (d > worst) [worst, where] = [d, `1:${scale} ${name}`];
    }
  appendFileSync(out, `PERF ${asset.id} worst ${worst.toFixed(0)} ms at ${where}\n`);
}
appendFileSync(out, "DONE\n");
