/** 🧊️ The sweep and the consistency rule of `🧪️tests/🧊️three-views` (every pose and clip instant at every authored scale) for people or asset files: the number of findings per check and the first ones.
 *  bun sweep.ts <slug|file.json>… */
import { authoredScales, consistency, sweep } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { fileOf } from "./plants.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const findings = authoredScales(asset).flatMap((scale: number) => [...sweep(asset, scale), ...consistency(asset, scale)]);
  const counts = new Map<string, number>();
  for (const f of findings) counts.set(f.check, (counts.get(f.check) ?? 0) + 1);
  console.log(`${arg.split("/").slice(-1)[0]}: ${findings.length} findings ${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const f of findings.slice(0, Number(process.env.N ?? 2))) console.log(`   [${f.check}] ${f.message}`.slice(0, 260));
}
