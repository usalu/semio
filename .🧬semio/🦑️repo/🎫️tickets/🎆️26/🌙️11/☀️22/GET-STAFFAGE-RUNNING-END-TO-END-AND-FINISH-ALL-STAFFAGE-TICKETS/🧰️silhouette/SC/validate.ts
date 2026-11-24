/** 🩺️ `validateAsset` of components or asset files: the issue count, the codes and every issue as one sorted line (so that two runs can be compared with diff).
 *  bun validate.ts <id|file.json>… */
import { S, fileOf } from "./lib.ts";
const { validateAsset } = await import(`${S}/📦️packages/🟦️typescript/🟦️.ts`);

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string } }[];
  const counts = new Map<string, number>();
  for (const i of issues) counts.set(i.code, (counts.get(i.code) ?? 0) + 1);
  console.log(`${asset.id}\t${issues.length} issues\t${[...counts].map(([c, n]) => `${c}×${n}`).join(", ")}`);
  for (const line of issues.map((i) => `   ${i.code} ${i.path ?? ""}: ${i.message.en}`.slice(0, Number(process.env.L ?? 300))).sort()) console.log(line);
}
