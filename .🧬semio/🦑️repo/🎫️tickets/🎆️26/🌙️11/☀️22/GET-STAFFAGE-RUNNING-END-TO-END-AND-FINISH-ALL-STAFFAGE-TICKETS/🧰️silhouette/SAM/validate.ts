/** 🩺️ `validateAsset` of assets or asset files, optional ops applied first: the issue count, the codes and the first issues.
 *  bun validate.ts <id|file.json>… [--ops ops.json] */
import { parseArgs } from "node:util";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { fileOf } from "./files.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" } } });
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  if (values.ops !== undefined) (await import("./ops.ts")).applyOps(asset, JSON.parse(await Bun.file(values.ops).text()));
  const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string } }[];
  const counts = new Map<string, number>();
  for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
  console.log(`${asset.id}\t${issues.length} issues\t${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const issue of issues.slice(0, Number(process.env.N ?? 6))) console.log(`   ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 400));
}
