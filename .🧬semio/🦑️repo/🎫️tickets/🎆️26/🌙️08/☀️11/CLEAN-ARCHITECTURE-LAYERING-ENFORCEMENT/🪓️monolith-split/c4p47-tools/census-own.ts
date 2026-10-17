// 🧮️ c4p47 own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each Package 47 crate (lib and cfg(test) closures), on the live tree or on an oracle sandbox copy; writes nothing.
// usage: bun census-own.ts <root> [only,tags]  ; root is the repo root (live) or a sandbox root that holds the module directories in the same relative layout
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const O = "🧰️framework/🛍️products/💻️os";
const M = `${O}/🔨️modules`;
const PK = "/📦️packages/🦀️rust";
const GR = `${M}/🔁️workflow/🗿️artifacts/🔁️workflow/🕸️graph`;
const crates: [string, string, string, string][] = [
  ["gr", "semio-framework-artifact-workflow-workflow-graph", `${GR}${PK}`, `${GR}/🦀️.rs`],
  ["shs", "semio-framework-os-shell", `${M}/🖥️shell${PK}`, `${M}/🖥️shell${PK}/🦀️.rs`],
  ["dsu", "semio-framework-os-dsl-tests", `${M}/🗣️dsl/🧪️tests/🥪️companion${PK}`, `${M}/🗣️dsl/🧪️tests/🥪️companion/🦀️.rs`],
  ["neu", "semio-framework-os-kernel-neural-engine", `${M}/🧠️neural/⚙️engine${PK}`, `${M}/🧠️neural/⚙️engine${PK}/🦀️.rs`],
  ["cab", "semio-framework-os-host-codec-abi", `${O}/🖥️host/📡️codec-abi${PK}`, `${O}/🖥️host/📡️codec-abi/🦀️.rs`],
  ["host", "semio-framework-os", `${O}/🖥️host${PK}`, `${O}/🖥️host${PK}/🦀️.rs`],
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
      console.log(JSON.stringify({tag, name, test, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
