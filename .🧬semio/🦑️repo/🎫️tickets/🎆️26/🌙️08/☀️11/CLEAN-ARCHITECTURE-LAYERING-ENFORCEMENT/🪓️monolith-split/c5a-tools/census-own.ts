// 🧮️ c5a own-closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each CENSUS5-A crate (lib and cfg(test) closures); writes nothing.
// usage: bun census-own.ts <root> [only,tags]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const PK = "/📦️packages/🦀️rust";
const comp = (n: string) => `${PL}/🧷️companions/${n}`;
const crates: [string, string, string][] = [
  ["t01", "semio-framework-plugin-tests-01", comp("🍎️tests-01")],
  ["t05", "semio-framework-plugin-tests-05", comp("🍌️tests-05")],
  ["t07-2", "semio-framework-plugin-tests-07-2", comp("🪡️tests-07-2")],
  ["t08", "semio-framework-plugin-tests-08", comp("🍓️tests-08")],
  ["t09-2", "semio-framework-plugin-tests-09-2", comp("🪛️tests-09-2")],
  ["t11-2", "semio-framework-plugin-tests-11-2", comp("🔧️tests-11-2")],
  ["t12-2", "semio-framework-plugin-tests-12-2", comp("🪝️tests-12-2")],
  ["t13", "semio-framework-plugin-tests-13", comp("🥭️tests-13")],
  ["t14-2", "semio-framework-plugin-tests-14-2", comp("🧵️tests-14-2")],
  ["t15", "semio-framework-plugin-tests-15", comp("🥥️tests-15")],
  ["t16", "semio-framework-plugin-tests-16", comp("🥝️tests-16")],
  ["tt", "semio-framework-plugin-time-travel", `${PL}/⏪️time-travel`],
  ["tti", "semio-framework-plugin-time-travel-turns", `${PL}/⏪️time-travel/🧵️instance`],
  ["tr", "semio-framework-plugin-tool-run", `${PL}/⏯️tool-run`],
  ["tri", "semio-framework-plugin-tool-run-turns", `${PL}/⏯️tool-run/🧵️instance`],
  ["rt", "semio-framework-plugin-reactor-turn", `${PL}/⚛️reactor/🔄️turn`],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const [tag, name, dir] of crates) {
  if (only && !only.has(tag)) continue;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([resolve(root, `${dir}/🦀️.rs`)], root, cfg, readFile, {manifestDirectory: resolve(root, `${dir}${PK}`), generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({tag, name, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({tag, name, test, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
