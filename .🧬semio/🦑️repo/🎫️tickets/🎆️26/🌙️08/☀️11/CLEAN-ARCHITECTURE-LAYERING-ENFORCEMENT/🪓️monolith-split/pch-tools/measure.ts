// 📏️ Read-only: census rows (files / bytes / active lines, lib and test closure, the same counter the gate uses) for the given crate manifests.
import {join, dirname} from "node:path";
import {readFileSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const {measureRustCrate, readCargoLib} = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts"));
for (const manifest of process.argv.slice(2)) {
  const name = (Bun.TOML.parse(readFileSync(join(ROOT, manifest), "utf8")) as any).package.name;
  const entry = readCargoLib(join(ROOT, manifest)).entry;
  const row = async (test: boolean) => { try { const m = await measureRustCrate(ROOT, entry, test, new Map(), join(ROOT, dirname(manifest))); return `${m.files}/${m.bytes}/${m.lines}`; } catch (e) { return "ERR " + String((e as Error).message).slice(0, 60); } };
  console.log([name, await row(false), await row(true)].join("\t"));
}
