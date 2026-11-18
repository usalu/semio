import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const all = new Map<string, any>();
const files: string[] = [];
for (const group of readdirSync(root, { withFileTypes: true })) {
  if (!group.isDirectory()) continue;
  for (const dir of readdirSync(join(root, group.name), { withFileTypes: true })) {
    const file = join(root, group.name, dir.name, "🔣️.json");
    if (!dir.isDirectory() || !existsSync(file)) continue;
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    if (asset.rig === undefined || asset.thermal === undefined) continue;
    all.set(asset.id, asset);
    files.push(file);
  }
}
const args = process.argv.slice(2);
const targets = args.length > 0 ? args : files;
let failing = 0;
for (const file of targets) {
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  const issues = validateAsset(asset, { assets: all });
  if (issues.length > 0) failing++;
  console.log(`${issues.length === 0 ? "OK  " : "FAIL"} ${asset.id} (${file.split("/").slice(-3, -1).join("/")})`);
  for (const issue of issues) console.log(`  [${issue.code}] ${issue.path}: ${(issue.message as any).en ?? issue.message}`);
}
console.log(`${targets.length} checked, ${failing} failing`);
process.exitCode = failing > 0 ? 1 : 0;
