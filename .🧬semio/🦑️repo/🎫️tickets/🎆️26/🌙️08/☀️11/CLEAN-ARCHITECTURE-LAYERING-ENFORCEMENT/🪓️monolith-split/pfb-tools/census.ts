// 🧮️ Source-boundary census (files / bytes / active lines, lib and test cfg) of the crate behind each manifest. usage: bun census.ts <Cargo.toml>...
import {dirname, isAbsolute, join} from "node:path";
import {measureRustCrate, readCargoLib} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
for (const arg of process.argv.slice(2)) {
  const manifest = isAbsolute(arg) ? arg : join(root, arg);
  const lib = readCargoLib(manifest);
  const dir = dirname(manifest);
  const l = await measureRustCrate(root, lib.entry, false, new Map(), dir);
  const t = await measureRustCrate(root, lib.entry, true, new Map(), dir);
  const within = (x: any, b: any) => x.files <= b.files && x.bytes <= b.bytes && x.lines <= b.lines;
  console.log(JSON.stringify({name: lib.name, lib: `${l.files}/${l.bytes}/${l.lines}`, test: `${t.files}/${t.bytes}/${t.lines}`, target: within(l, {files: 28, bytes: 230000, lines: 3500}) && within(t, {files: 28, bytes: 230000, lines: 3500}), hard: within(l, {files: 32, bytes: 262144, lines: 4000}) && within(t, {files: 32, bytes: 262144, lines: 4000})}));
}
