import {readFileSync, writeFileSync, mkdirSync, existsSync} from "node:fs";
import {dirname, join} from "node:path";
import {fixturesToRuntime, resolveFixturePlaceholders} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [file, manifestDir, backup] = process.argv.slice(2);
const abs = join(ROOT, file!);
const text = readFileSync(abs, "utf8");
const out = fixturesToRuntime(text, abs, ROOT);
if (!out) { console.log("nothing to convert"); process.exit(0); }
const b = join(ROOT, backup!, file!);
if (!existsSync(b)) { mkdirSync(dirname(b), {recursive: true}); writeFileSync(b, text); }
writeFileSync(abs, resolveFixturePlaceholders(out, ROOT, join(ROOT, manifestDir!)));
console.log("converted", file);
