// 🧮️ c4p23 own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each Package 23 crate, once on the live tree and once on the pre-edit sandbox copy.
// usage: bun census.ts <root> ; root is the repo root (live) or a sandbox root that holds the artifact directories in the same relative layout
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const R = "✏️s/🔌️plugins";
const crates: [string, string, string][] = [
  ["semio-s-artifact-raster-raster", `${R}/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust`, `${R}/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs`],
  ["semio-s-artifact-stdio-contract-editing", `${R}/🗄️stdio/📇️registry/🧬️contract/✏️editing/📦️packages/🦀️rust`, `${R}/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`],
  ["semio-s-artifact-stdio-contract-editing-patch", `${R}/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/📦️packages/🦀️rust`, `${R}/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs`],
  ["semio-s-artifact-stdio-contract-editing-details", `${R}/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/📦️packages/🦀️rust`, `${R}/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [name, manifestDir, entry] of crates) {
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([resolve(root, entry)], root, cfg, readFile, {manifestDirectory: resolve(root, manifestDir), generated: () => undefined});
      console.log(JSON.stringify({name, test, files: inputs.length, bytes: inputs.reduce((s, i) => s + i.bytes, 0), activeLines: inputs.reduce((s, i) => s + i.activeLines, 0)}));
    } catch (error) {
      console.log(JSON.stringify({name, test, error: String((error as Error).message ?? error).slice(0, 160)}));
    }
  }
}
