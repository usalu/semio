import { isExcludedTestPath, testTaxonomy } from "/Users/ueli/Documents/semio/./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { readdirSync, lstatSync, existsSync } from "node:fs";
import { join, relative, basename } from "node:path";
const root = process.cwd();
const tx = testTaxonomy(root);
const SKIP = new Set(["node_modules", "target", ".git", "temp"]);
let seen = 0, excluded = 0, withManifest = 0;
const excludedSamples: string[] = [], noManifest: string[] = [];
const stack = [root];
while (stack.length) {
  const dir = stack.pop()!;
  let entries: string[]; try { entries = readdirSync(dir); } catch { continue; }
  for (const name of entries) {
    const abs = join(dir, name);
    let st; try { st = lstatSync(abs); } catch { continue; }
    if (!st.isDirectory() || st.isSymbolicLink()) continue;
    if (SKIP.has(name)) continue;
    const rel = relative(root, abs).split("/").join("/");
    if (isExcludedTestPath(root, rel)) { if (basename(abs) === tx.testContributionDirName) { excluded++; if (excludedSamples.length < 5) excludedSamples.push(rel); } continue; }
    if (basename(abs) === tx.testContributionDirName) {
      seen++;
      if (existsSync(join(abs, "🔣️component.json"))) withManifest++; else if (noManifest.length < 5) noManifest.push(rel);
      continue;
    }
    stack.push(abs);
  }
}
console.log({ seen, excluded, withManifest, tx: tx.testContributionDirName });
console.log("excludedSamples", excludedSamples);
console.log("noManifest", noManifest);
