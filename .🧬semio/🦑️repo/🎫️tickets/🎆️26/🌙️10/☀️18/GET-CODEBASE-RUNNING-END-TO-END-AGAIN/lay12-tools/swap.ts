import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const agg = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const dry = process.argv[3] === "--dry";
const FW = "🧰️framework/🔨️modules/";
const aliases: Record<string, string> = {
  [FW + "📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"]: "@semio-tech/framework-compiler-ts/syntax/module-tree",
  [FW + "🧬️schema/✅️validator/🟦️.ts"]: "@semio-tech/schema-validator-ts",
  [FW + "📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"]: "@semio-tech/framework/filesystem/observation/descriptor",
  [FW + "📁️filesystem/🧾️observation/📁️current/🟦️.ts"]: "@semio-tech/framework/filesystem/observation/current",
  [FW + "🧪️test/📖️source-text/🟦️.ts"]: "@semio-tech/framework/test/source-text",
  [FW + "🧵️job/⚠️failure/🟦️.ts"]: "@semio-tech/framework/job/failure",
};
const pairs = new Set<string>();
for (const r of agg.rows) if (r.rule === "no-cross-package-relative" && aliases[r.to]) pairs.add(r.from + "\t" + r.to);
let changed = 0;
for (const pair of pairs) {
  const [from, to] = pair.split("\t") as [string, string];
  const abs = resolve(root, from);
  const text = readFileSync(abs, "utf8");
  const base = dirname(abs);
  const next = text.replace(/(["'])(\.{1,2}\/[^"']*)\1/gu, (m, q: string, spec: string) => {
    const rel = relative(root, resolve(base, spec));
    return rel === to ? q + aliases[to] + q : m;
  });
  if (next !== text) { changed++; if (!dry) writeFileSync(abs, next); console.log((dry ? "would " : "") + "swap " + from.replace(/️/gu, "").slice(-90)); }
  else console.log("NO MATCH " + from);
}
console.log("files changed", changed, "of", pairs.size);
