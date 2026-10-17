// 🧪️ Applies the generator's include_str! -> runtime read rewrite to given test files, resolving the placeholders against one crate directory: usage `bun fixtures-runtime.ts <crateDir> <file>...` (paths relative to the repository root).
import {readFileSync, writeFileSync} from "node:fs";
import {resolve} from "node:path";
import {fixturesToRuntime, resolveFixturePlaceholders} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [crateDir, ...files] = process.argv.slice(2);
for (const file of files) {
  const abs = resolve(ROOT, file);
  const text = readFileSync(abs, "utf8");
  const out = fixturesToRuntime(text, abs, ROOT);
  if (out === null) { console.log("unchanged", file); continue; }
  writeFileSync(abs, resolveFixturePlaceholders(out, ROOT, resolve(ROOT, crateDir)));
  console.log("converted", file);
}
