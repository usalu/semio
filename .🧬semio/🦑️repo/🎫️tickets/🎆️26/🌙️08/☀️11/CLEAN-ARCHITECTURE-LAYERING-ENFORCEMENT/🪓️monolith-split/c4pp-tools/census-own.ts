// 🧮️ c4p49 own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each Package 49 crate, on the live tree or on an oracle sandbox copy.
// usage: bun census-own.ts <root> [only,tags]  ; root is the repo root (live) or a sandbox root that holds the module directories in the same relative layout
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const R = "🧰️framework/🔨️modules";
const PK = "/📦️packages/🦀️rust";
const crates: [string, string, string, string][] = [
  ["tml", "semio-framework-surface-tiled-map", `${R}/🗺️surface/🗺️tiled-map${PK}`, `${R}/🗺️surface/🗺️tiled-map/🦀️.rs`],
  ["act", "semio-framework-manifest-actions", `${R}/🛂️manifest/🕹️actions${PK}`, `${R}/🛂️manifest/🕹️actions/🦀️.rs`],
  ["arg", "semio-framework-manifest-arg-schema", `${R}/🛂️manifest/🧮️arg-schema${PK}`, `${R}/🛂️manifest/🧮️arg-schema/🦀️.rs`],
  ["app", "semio-framework-manifest-app-surface", `${R}/🛂️manifest/🧱️app-surface${PK}`, `${R}/🛂️manifest/🧱️app-surface/🦀️.rs`],
  ["tmu", "semio-framework-tool-machine-tests", `${R}/🛠️tool-machine/🧪️tests/🥣️companion${PK}`, `${R}/🛠️tool-machine/🧪️tests/🥣️companion/🦀️.rs`],
  ["cfg", "semio-framework-math-sampling-config", `${R}/🧮️math/🎯️sampling/🎚️config${PK}`, `${R}/🧮️math/🎯️sampling/🎚️config/🦀️.rs`],
  ["voc", "semio-framework-math-sampling-vocabulary", `${R}/🧮️math/🎯️sampling/🔤️vocabulary${PK}`, `${R}/🧮️math/🎯️sampling/🔤️vocabulary/🦀️.rs`],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [tag, name, manifestDir, entry] of crates) {
  if (only && !only.has(tag)) continue;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([resolve(root, entry)], root, cfg, readFile, {manifestDirectory: resolve(root, manifestDir), generated: () => undefined});
      console.log(JSON.stringify({tag, name, test, files: inputs.length, bytes: inputs.reduce((s, i) => s + i.bytes, 0), activeLines: inputs.reduce((s, i) => s + i.activeLines, 0)}));
    } catch (error) {
      console.log(JSON.stringify({tag, name, test, error: String((error as Error).message ?? error).slice(0, 160)}));
    }
  }
}
