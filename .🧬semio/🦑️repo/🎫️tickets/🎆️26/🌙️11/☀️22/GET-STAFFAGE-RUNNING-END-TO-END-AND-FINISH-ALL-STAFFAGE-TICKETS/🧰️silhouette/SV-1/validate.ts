/** 🩺️ `validateAsset` of a plant or an asset file: the issue count per code on one line and every issue as one sorted line in `<out>` (code, path, data), so a before and an after file can be compared line by line.
 *  bun validate.ts <id|file.json> <out.txt> */
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { fileOf } from "./plants.ts";

const [arg, out] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(arg!)).json()).asset;
const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string }; data?: any }[];
const counts = new Map<string, number>();
for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
await Bun.write(out!, issues.map((i) => `${i.code}\t${i.path ?? ""}\t${i.message.en.replace(/-?\d+(\.\d+)?/gu, "#").slice(0, 200)}`).sort().join("\n") + "\n");
console.log(`${asset.id}\t${issues.length} issues\t${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
for (const issue of issues.slice(0, Number(process.env.N ?? 4))) console.log(`   ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 300));
