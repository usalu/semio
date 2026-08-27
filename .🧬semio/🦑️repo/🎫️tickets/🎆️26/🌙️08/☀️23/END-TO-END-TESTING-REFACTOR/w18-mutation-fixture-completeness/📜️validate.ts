/** 🧾️ Runs the platform's own catalog validator over every manifest in the tree. */
import { readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { mutationCatalogProblems } from "/Users/ueli/Documents/semio/./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const root = process.cwd();
const skip = new Set(["node_modules", "target", ".git", "temp", "storybook-static"]);
const manifests: string[] = [];
const walk = (p: string, d: number): void => {
  if (d > 14) return;
  let es; try { es = readdirSync(p, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const f = join(p, e.name);
    if (e.isDirectory()) { if (!f.includes("⚡️cache") && !f.includes("🎫️tickets")) walk(f, d + 1); }
    else if (e.name.endsWith("component.json") && f.includes("🧪️oracle")) manifests.push(f);
  }
};
walk(root, 0);
let catalogs = 0, bad = 0;
for (const m of manifests) {
  let j: any; try { j = JSON.parse(readFileSync(m, "utf8")); } catch { continue; }
  const owner = relative(root, m).split("/").slice(0, -2).join("/");
  for (const c of j.mutationCatalogs ?? []) {
    catalogs++;
    const problems = mutationCatalogProblems(c, owner);
    if (problems.length) { bad++; console.log("INVALID", c.id, owner, problems.join("; ")); }
  }
}
console.log(`${catalogs} catalog(s) validated, ${bad} invalid`);
