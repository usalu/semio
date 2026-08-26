import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const changed = execSync(`git -C "${root}" diff --name-only HEAD -- '*component.feature'`, { encoding: "utf8", maxBuffer: 1 << 26 }).trim().split("\n").filter(Boolean);
let before = 0, after = 0, lost: string[] = [], gained = 0;
for (const path of changed) {
  let head = "";
  try { head = execSync(`git -C "${root}" show HEAD:"${path}"`, { encoding: "utf8", maxBuffer: 1 << 26 }); } catch { head = ""; }
  const now = readFileSync(`${root}/${path}`, "utf8");
  const a = head ? parseFeature(head).scenarios.map((s) => s.id) : [];
  const b = parseFeature(now).scenarios.map((s) => s.id);
  before += a.length; after += b.length;
  for (const id of a) if (!b.includes(id)) lost.push(`${path}::${id}`);
  gained += b.filter((id) => !a.includes(id)).length;
}
console.log(JSON.stringify({ changedFeatures: changed.length, scenariosAtHead: before, scenariosNow: after, gained, lost }, null, 1));
