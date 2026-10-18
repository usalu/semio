// 🧮️ c6plug2 own-closure census: canonical `readRustSourceClosureV1` over the real lib root of each crate named in crates.json (lib and cfg(test) closures); writes nothing.
// usage: bun census-mine.ts [only,tags]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = "/Users/ueli/Documents/semio";
const only = process.argv[2] ? new Set(process.argv[2].split(",")) : undefined;
const rows = JSON.parse(readFileSync(new URL("./crates.json", import.meta.url), "utf8")) as [string, string, string][];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [tag, name, manifest] of rows) {
  if (only && !only.has(tag)) continue;
  const manifestDirectory = resolve(REPO, dirname(manifest));
  const toml = readFileSync(resolve(REPO, manifest), "utf8");
  const libPath = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/ms.exec(toml)?.[1] ?? "🦀️.rs";
  const rootFile = resolve(manifestDirectory, libPath);
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test", "feature=testkit"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([rootFile], REPO, cfg, readFile, {manifestDirectory, generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({tag, name, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({tag, name, test, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
