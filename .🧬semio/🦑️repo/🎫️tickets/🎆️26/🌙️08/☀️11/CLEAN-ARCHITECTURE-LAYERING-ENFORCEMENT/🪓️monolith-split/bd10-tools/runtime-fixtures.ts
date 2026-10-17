// 🧰 bd10: rewrites fixture `include_str!`/`include_bytes!` of the given test files into runtime reads (generator machinery), relative to the owning crate directory.
// usage: bun runtime-fixtures.ts <manifestDir> <file>... [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {fixturesToRuntime, resolveFixturePlaceholders} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const [manifestDir, ...rest] = process.argv.slice(2).filter((a) => a !== "--write");
const write = process.argv.includes("--write");
for (const file of rest) {
  const abs = resolve(root, file);
  const text = readFileSync(abs, "utf8");
  const out = fixturesToRuntime(text, abs, root);
  if (out === null) { console.log("unchanged", file.split("any/")[1]); continue; }
  const resolved = resolveFixturePlaceholders(out, root, join(root, manifestDir));
  console.log("rewrote", file.split("any/")[1], text.length, "->", resolved.length);
  if (write) writeFileSync(abs, resolved);
}
