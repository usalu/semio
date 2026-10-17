// 🧮️ Source-boundary census of the three flat test target roots (files / bytes / active lines of the closure that `#[path]` and `include!` reach). usage: bun census2.ts <root.rs>...
import {dirname, isAbsolute, join} from "node:path";
import {measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
for (const arg of process.argv.slice(2)) {
  const entry = isAbsolute(arg) ? arg.slice(root.length + 1) : arg;
  const m = await measureRustCrate(root, entry, false, new Map(), dirname(join(root, entry)));
  const hard = m.files <= 32 && m.bytes <= 262144 && m.lines <= 4000;
  console.log(JSON.stringify({entry: entry.split("/").slice(-3).join("/"), files: m.files, bytes: m.bytes, lines: m.lines, hard}));
}
