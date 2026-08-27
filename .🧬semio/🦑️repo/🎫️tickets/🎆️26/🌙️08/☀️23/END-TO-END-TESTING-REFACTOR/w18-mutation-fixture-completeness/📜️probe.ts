import { isExcludedTestPath } from "/Users/ueli/Documents/semio/./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = process.cwd();
const p = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧪️oracle";
const segs = p.split("/");
for (let i = 1; i <= segs.length; i++) { const sub = segs.slice(0, i).join("/"); console.log(isExcludedTestPath(root, sub) ? "EXCLUDED" : "ok      ", sub); }
