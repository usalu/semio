// 🧮️ c6plug own-closure census (crates from files.tsv): runs the canonical closure counter (`readRustSourceClosureV1`) over the lib root of each CENSUS5-A crate (lib and cfg(test) closures); writes nothing.
// usage: bun census-own.ts <root> [only,tags]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const PK = "/📦️packages/🦀️rust";
const crates: [string, string, string][] = readFileSync(process.env.C6_FILES ?? new URL("./files.tsv", import.meta.url), "utf8").split("\n").filter(Boolean).map((l) => {
  const [tag, rel] = l.split("\t");
  const dir = rel!.slice(0, rel!.length - "/🦀️.rs".length);
  let name = "?";
  try { name = /^name\s*=\s*"([^"]+)"/m.exec(readFileSync(resolve("/Users/ueli/Documents/semio", dir + PK + "/Cargo.toml"), "utf8"))![1]!; } catch { }
  return [tag!, name, dir];
});
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
