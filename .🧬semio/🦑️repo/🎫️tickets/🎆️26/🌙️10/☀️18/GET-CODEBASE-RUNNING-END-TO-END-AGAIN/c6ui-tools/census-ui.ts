// 🧮️ c6ui own-closure census: canonical closure counter over the lib root of each owning crate (lib and cfg(test) closures, plus declared features); writes nothing.
// usage: bun census-ui.ts <crates.json> [crate,...]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve, dirname} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const crates: Record<string, string> = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const PK = "/📦️packages/🦀️rust";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [name, dir] of Object.entries(crates)) {
  if (only && !only.has(name)) continue;
  const toml = readFileSync(resolve(root, `${dir}${PK}/Cargo.toml`), "utf8");
  const lib = toml.match(/^\[lib\][^\[]*?path\s*=\s*"([^"]+)"/ms)?.[1] ?? "src/lib.rs";
  const libRoot = resolve(root, `${dir}${PK}`, lib);
  const limits = {bytes: +(toml.match(/maxSourceBytes\s*=\s*(\d+)/)?.[1] ?? 262144), files: +(toml.match(/maxSourceFiles\s*=\s*(\d+)/)?.[1] ?? 32), lines: +(toml.match(/maxActiveLines\s*=\s*(\d+)/)?.[1] ?? 4000)};
  const features = [...toml.matchAll(/^([A-Za-z0-9_-]+)\s*=\s*\[/gm)].map(m => m[1]!);
  const variants: [string, string[]][] = [["lib", []], ["test", ["test"]], ["all", ["test", ...features.map(f => `feature=${f}`)]]];
  for (const [label, extra] of variants) {
    const cfg = [...hostCfg, "debug_assertions", ...extra];
    try {
      const inputs = await readRustSourceClosureV1([libRoot], root, cfg, readFile, {manifestDirectory: resolve(root, `${dir}${PK}`), generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({name, cfg: label, files, bytes, activeLines: lines, limits, ratio: +Math.max(files / limits.files, bytes / limits.bytes, lines / limits.lines).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({name, cfg: label, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
