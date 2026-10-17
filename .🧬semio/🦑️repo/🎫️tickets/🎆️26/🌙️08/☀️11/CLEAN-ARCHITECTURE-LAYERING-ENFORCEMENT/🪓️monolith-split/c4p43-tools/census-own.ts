// 🧮️ c4p43 own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each Package 43 crate (lib and cfg(test)), on a given root (live repo, or a sandbox copy that holds the same relative layout).
// usage: bun census-own.ts <root> [tag...]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = new Set(process.argv.slice(3));
const O = "🧰️framework/🛍️products/💻️os/🔨️modules";
const crates: [string, string, string, string][] = [
  ["inf", "semio-framework-os-inference", `${O}/💡️inference/📦️packages/🦀️rust`, `${O}/💡️inference/🦀️.rs`],
  ["dir", "semio-framework-os-directory", `${O}/📇️directory/📦️packages/🦀️rust`, `${O}/📇️directory/🦀️.rs`],
  ["hcd", "semio-framework-os-history-codec", `${O}/📡️spr/📜️history/⚙️codec/📦️packages/🦀️rust`, `${O}/📡️spr/📜️history/⚙️codec/🦀️.rs`],
  ["plw", "semio-framework-os-spr", `${O}/📡️spr/📦️packages/🦀️rust`, `${O}/📡️spr/🦀️.rs`],
  ["chn", "semio-framework-os-channel", `${O}/📡️spr/🧵️channel/📦️packages/🦀️rust`, `${O}/📡️spr/🧵️channel/🦀️.rs`],
  ["cut", "semio-framework-os-channel-tests", `${O}/📡️spr/🧵️channel/🧪️tests/🪤️companion/📦️packages/🦀️rust`, `${O}/📡️spr/🧵️channel/🧪️tests/🪤️companion/🦀️.rs`],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [tag, name, manifestDir, entry] of crates) {
  if (only.size && !only.has(tag)) continue;
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
