// 🧮️ c4p44 own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each Package 44 crate, on the live tree or on an oracle sandbox copy.
// usage: bun census-own.ts <root> [only,tags]  ; root is the repo root (live) or a sandbox root that holds the module directories in the same relative layout
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const M = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const PK = "/📦️packages/🦀️rust";
const crates: [string, string, string, string][] = [
  ["pac", "semio-framework-os-mcp-plugin-runtime", `${M}/⚙️plugin-runtime${PK}`, `${M}/⚙️plugin-runtime/🦀️.rs`],
  ["gwb", "semio-framework-os-mcp-workspace", `${M}/🪟️workspace-host${PK}`, `${M}/🪟️workspace-host/🦀️.rs`],
  ["wsq", "semio-framework-os-mcp", `${M}${PK}`, `${M}${PK}/🦀️.rs`],
  ["inf", "semio-framework-os-mcp-tools-inference", `${M}/🔭️inference-tools${PK}`, `${M}/🔭️inference-tools/🦀️.rs`],
  ["cat", "semio-framework-os-mcp-catalog", `${M}/📚️catalog-compiler${PK}`, `${M}/📚️catalog-compiler/🦀️.rs`],
  ["sch", "semio-framework-os-mcp-schema", `${M}/📐️schema-contract${PK}`, `${M}/📐️schema-contract/🦀️.rs`],
  ["ses", "semio-framework-os-mcp-bridge", `${M}/🪢️bridge-session${PK}`, `${M}/🪢️bridge-session/🦀️.rs`],
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
