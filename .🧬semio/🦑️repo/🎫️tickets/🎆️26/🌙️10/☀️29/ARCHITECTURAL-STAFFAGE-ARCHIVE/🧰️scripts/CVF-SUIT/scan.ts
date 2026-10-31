/** [DEBUG] Scan checkAssetDetailed for the six staffage categories. */
import { mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { checkAssetDetailed } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { PRINT_PROFILES } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = (process.argv[2] ?? "vegetation,furniture,objects,vehicles,machinery,components").split(",");
const out = process.argv[3]!;
const profile = PRINT_PROFILES.profiles.find((p) => p.id === "office-laser")!;
const known = new Set<string>();
const files: { id: string; path: string }[] = [];
for (const dir of await readdir(root, { withFileTypes: true })) {
  if (!dir.isDirectory()) continue;
  if (!cats.some((c) => dir.name.includes(c))) continue;
  const cat = resolve(root, dir.name);
  for (const folder of await readdir(cat, { withFileTypes: true })) {
    if (!folder.isDirectory()) continue;
    const path = resolve(cat, folder.name);
    const names = await readdir(path);
    const docName = names.find((f) => f.endsWith(".json"));
    if (!docName) continue;
    const doc = JSON.parse(await readFile(resolve(path, docName), "utf8"));
    known.add(doc.asset.id);
    files.push({ id: doc.asset.id, path: resolve(path, docName) });
  }
}
const report: Record<string, { problems: string[]; lines: string[] }> = {};
let failed = 0;
for (const [index, file] of files.entries()) {
  const t0 = Date.now();
  const doc = JSON.parse(await readFile(file.path, "utf8"));
  const result = checkAssetDetailed(doc.asset, { samples: 4, profile, assets: known, physics: true });
  const ms = Date.now() - t0;
  console.log(`[DEBUG] ${result.problems.length ? "FAIL" : "ok"} ${file.id} ${index + 1}/${files.length} ${ms}ms ${result.problems.length}`);
  if (result.problems.length) {
    failed++;
    for (const problem of result.problems.slice(0, 12)) console.log(`  ${problem}`);
    report[file.id] = { problems: [...result.problems], lines: [...result.lines] };
  }
}
await mkdir(out.slice(0, out.lastIndexOf("/")), { recursive: true });
await writeFile(out, JSON.stringify(report, null, 1));
console.log(`[DEBUG] failed ${failed}/${files.length}`);
process.exitCode = failed ? 1 : 0;
