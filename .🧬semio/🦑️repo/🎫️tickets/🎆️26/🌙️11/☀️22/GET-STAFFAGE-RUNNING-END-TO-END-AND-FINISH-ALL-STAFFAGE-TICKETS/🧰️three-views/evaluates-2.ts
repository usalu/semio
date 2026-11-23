/** ⏱️ Scans a clip of one asset in fine time steps and lists the instants and scales at which `evaluate` throws, to measure how wide a failing band is.
 *
 * Usage: `bun evaluates-2.ts <id> <clip> <fromS> <toS> <stepS> [--file <json>]`
 * @see ./sweep-2.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { authoredScales, catalog } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

if (import.meta.main) {
  const args = process.argv.slice(2);
  const [id, clip, from, to, step] = args;
  const file = args.indexOf("--file") < 0 ? undefined : args[args.indexOf("--file") + 1];
  const asset = file === undefined ? catalog().find((candidate) => candidate.id === id)! : (JSON.parse(readFileSync(file, "utf8")) as { asset: ReturnType<typeof catalog>[number] }).asset;
  let failed = 0;
  let count = 0;
  for (let t = Number(from); t <= Number(to) + 1e-9; t += Number(step)) {
    count += 1;
    const bad = authoredScales(asset).filter((scale) => {
      try {
        evaluate(asset, { kind: "clip", clip: clip!, timeS: t }, { kind: "scale", scale: { denominator: scale } });
        return false;
      } catch {
        return true;
      }
    });
    if (bad.length > 0) {
      failed += 1;
      console.log(`${t.toFixed(4)} s: fails at 1:${bad.join(", 1:")}`);
    }
  }
  console.log(`${id} ${clip}: ${failed} of ${count} instants fail`);
}
