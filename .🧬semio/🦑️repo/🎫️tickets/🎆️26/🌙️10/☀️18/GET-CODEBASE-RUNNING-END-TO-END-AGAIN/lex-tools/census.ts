// 🧮️ LEX census: the canonical closure counter (`readRustSourceClosureV1`) over the lib root of the named crates in lib and cfg(test) mode; writes nothing.
// usage: bun census.ts
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const A = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor";
const PK = "/📦️packages/🦀️rust";
const crates: [string, string][] = [
  ["semio-s-artifact-layout-layout-engine-export", `${A}/⚙️engine/📤️export`],
  ["semio-s-artifact-layout-layout-engine-export-unit-tests", `${A}/⚙️engine/📤️export/🧪️tests/🔬️unit`],
  ["semio-s-artifact-layout-layout-engine-json", `${A}/⚙️engine/📤️export/🔣️json`],
  ["semio-s-artifact-layout-layout-engine", `${A}/⚙️engine`],
  ["semio-s-artifact-layout-layout-editor", `${A}`],
  ["semio-s-artifact-layout-layout-editor-commands-tests", `${A}/🎮️commands/🧪️tests/🥣️companion`],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [name, dir] of crates) {
  const lib = name.endsWith("unit-tests") ? `${dir}/🦀️.rs` : (name.endsWith("-export") ? `${dir}/🦀️.rs` : `${dir}/🦀️.rs`);
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([resolve(root, lib)], root, cfg, readFile, {manifestDirectory: resolve(root, `${dir}${PK}`), generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({name, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({name, test, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
