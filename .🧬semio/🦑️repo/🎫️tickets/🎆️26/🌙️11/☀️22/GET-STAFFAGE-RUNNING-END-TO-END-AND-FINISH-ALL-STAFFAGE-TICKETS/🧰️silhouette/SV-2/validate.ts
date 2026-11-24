/** 🩺️ `validateAsset` of plants or asset files: the issue count, the codes and the first issues; with `--against <dir>` also the issues the file has that `<dir>/<slug>.json` (the state before the repair) did not have.
 *  bun validate.ts <slug|file.json>… [--against <dir>] */
import { parseArgs } from "node:util";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { fileOf } from "./measure-lib.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { against: { type: "string" } } });
const keyOf = (i: any): string => JSON.stringify([i.code, i.path, i.data?.bone, [...(i.data?.shapes ?? [])].sort()]);
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const issues = validateAsset(asset) as any[];
  const counts = new Map<string, number>();
  for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
  let extra = "";
  if (values.against !== undefined) {
    const before = new Set((validateAsset((await Bun.file(`${values.against}/${asset.id}.json`).json()).asset) as any[]).map(keyOf));
    const fresh = issues.filter((i) => !before.has(keyOf(i)));
    extra = `\t${before.size} before, ${fresh.length} new`;
    for (const i of fresh.slice(0, 6)) console.log(`   NEW ${i.code} ${i.path ?? ""}: ${i.message.en}`.slice(0, 330));
  }
  console.log(`${asset.id}\t${issues.length} issues${extra}\t${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
}
