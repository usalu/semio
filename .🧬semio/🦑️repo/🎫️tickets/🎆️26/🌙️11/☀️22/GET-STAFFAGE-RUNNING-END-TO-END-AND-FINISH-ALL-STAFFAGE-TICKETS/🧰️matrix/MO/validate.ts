/** 🩺️ `validateAsset` of assets of batch MO: the issue count, the codes and the issues, one per line.
 *  bun validate.ts <id|file.json>… */
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { fileOf } from "./files.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string } }[];
  const counts = new Map<string, number>();
  for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
  console.log(`${asset.id}\t${issues.length} issues\t${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const issue of issues.slice(0, Number(process.env.N ?? 40))) console.log(`   ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 330));
}
