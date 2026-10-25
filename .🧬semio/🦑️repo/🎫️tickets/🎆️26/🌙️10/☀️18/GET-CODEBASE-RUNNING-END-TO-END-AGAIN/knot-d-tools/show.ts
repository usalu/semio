import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const strip = (p: string) => p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "L/");
for (const [i, r] of rows.entries()) {
  console.log("SCC", i, r.size, r.edges.length);
  for (const f of r.files) console.log("  F", strip(f));
  for (const [a, b, k] of r.edges) console.log("  E", strip(a), "->", strip(b), k);
}
