// 🗂️ Registers build-owner rows for hand-made crates through the generator's own inserter: usage `bun owner-row.ts <name>=<root> ...` from the repository root.
import {readFileSync, writeFileSync} from "node:fs";
import {insertBuildOwnerRows} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
const rows = process.argv.slice(2).map(arg => { const at = arg.indexOf("="); return {name: arg.slice(0, at), root: arg.slice(at + 1)}; });
const file = "/Users/ueli/Documents/semio/📋️build-owners.json";
const {text, added} = insertBuildOwnerRows(readFileSync(file, "utf8"), rows);
if (added.length) writeFileSync(file, text);
console.log("added", added);
