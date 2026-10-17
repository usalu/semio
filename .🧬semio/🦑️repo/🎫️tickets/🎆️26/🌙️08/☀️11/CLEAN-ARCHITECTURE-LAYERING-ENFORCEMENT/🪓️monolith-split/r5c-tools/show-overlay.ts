// 🔍️ Prints the transformed text (test transform + lift) of one file of a package. Usage: <pkg> <lift> <file-substring>
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {transformTestClosure} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {liftModuleKids} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [pkg, lift, needle] = process.argv.slice(2);
const manifest = resolve(ROOT, pkg!, "Cargo.toml");
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const entry = resolve(dirname(manifest), parsed.lib?.path ?? "src/lib.rs");
const orig = readFileSync(entry, "utf8");
const names = new Set<string>(parseRustSourceTree(orig).modules.filter(n => !n.inline).map(n => n.name));
const tt = transformTestClosure(ROOT, entry, orig, names, {fixtures: true, aggregate: true, hoist: true, detach: true});
const r = (lift ?? "") ? liftModuleKids(tt, entry, orig, lift!.split(",")) : tt;
for (const [p, t] of [...r.edits, ...r.creates]) if (p.includes(needle!)) console.log("=== " + p + "\n" + t.slice(0, 2500));
