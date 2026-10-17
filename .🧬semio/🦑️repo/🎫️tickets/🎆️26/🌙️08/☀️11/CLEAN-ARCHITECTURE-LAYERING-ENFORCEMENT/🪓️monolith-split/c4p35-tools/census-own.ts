// 🧮️ c4p35 own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each Package 35 crate (lib and cfg(test)), on a given root (live repo, or a pre-edit sandbox copy that holds the same relative layout).
// usage: bun census-own.ts <root> [tag...]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = new Set(process.argv.slice(3));
const M = "🧰️framework/🔨️modules";
const crates: [string, string, string, string][] = [
  ["fmt", "semio-framework-pack-format", `${M}/🎒️pack/📐️format/📦️packages/🦀️rust`, `${M}/🎒️pack/📐️format/🦀️.rs`],
  ["jtu", "semio-framework-pack-json-tests", `${M}/🎒️pack/🔤️json/🧪️tests/🥣️companion/📦️packages/🦀️rust`, `${M}/🎒️pack/🔤️json/🧪️tests/🥣️companion/🦀️.rs`],
  ["onb", "semio-framework-onboarding", `${M}/🎓️onboarding/📦️packages/🦀️rust`, `${M}/🎓️onboarding/🦀️.rs`],
  ["mut", "semio-framework-replication-mutation", `${M}/📡️replication/🎮️mutation/📦️packages/🦀️rust`, `${M}/📡️replication/🎮️mutation/🦀️.rs`],
  ["ing", "semio-framework-replication-command-ingress", `${M}/📡️replication/📡️wire/🎮️command/📥️ingress/📦️packages/🦀️rust`, `${M}/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs`],
  ["prs", "semio-framework-replication-presence", `${M}/📡️replication/📡️wire/👥️presence/📦️packages/🦀️rust`, `${M}/📡️replication/📡️wire/👥️presence/🦀️.rs`],
  ["wire", "semio-framework-replication-frames", `${M}/📡️replication/📡️wire/📦️packages/🦀️rust`, `${M}/📡️replication/📡️wire/🦀️.rs`],
  ["exp", "semio-framework-machine-derive", `${M}/🔄️machine/✨️derive/📦️packages/🦀️rust`, `${M}/🔄️machine/✨️derive/🦀️.rs`],
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
