import { readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { fixtureFacadeTargets } from "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/🧪️test/📖️source-text/🟦️.ts";
const root = "/Users/ueli/Documents/semio/🧰️framework";
const files = (d: string): string[] => readdirSync(d, { withFileTypes: true }).flatMap(e => e.name === "node_modules" ? [] : e.isDirectory() ? files(join(d, e.name)) : e.isFile() ? [join(d, e.name)] : []);
const banned = ["🧬️schema", "📦️packages", "🎯️targets", "🧩️implementations"];
function belongs(rel: string, skip: number): boolean { const p = rel.split("/"), f = p.indexOf("🧫️fixtures"), t = p.indexOf("🧪️tests"); return t >= 0 && t < f && !p.slice(skip, f).some(x => banned.includes(x)); }
const base = process.argv[2]!, skip = Number(process.argv[3] ?? 0);
for (const f of files(join(root, base)).filter(p => p.split("/").includes("🧫️fixtures"))) {
  const rel = relative(join(root, base), f);
  if (belongs(rel, skip)) continue;
  const t = fixtureFacadeTargets(f);
  if (t && t.every(x => belongs(relative(join(root, base), x), skip))) continue;
  console.log(relative(root, f), t ? "FACADE-BAD-TARGET" : "");
}
