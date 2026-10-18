// 🧮️ c6fin closure census: runs the canonical closure counter (`readRustSourceClosureV1`) over the crate lib root of each c6fin file (lib and cfg(test) closures); writes nothing.
// usage: bun census-fin.ts <repo-root> [tag,tag]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve, dirname} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const only = process.argv[3] ? new Set(process.argv[3].split(",")) : undefined;
const rows: {tag: string; root: string; manifest: string | null; name: string | null}[] = JSON.parse(readFileSync(resolve(import.meta.dir, "crates.json"), "utf8"));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const row of rows) {
  if (only && !only.has(row.tag)) continue;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test", "feature=testkit", "feature=artifact-app-testing"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([resolve(root, row.root)], root, cfg, readFile, {manifestDirectory: resolve(root, row.manifest ?? dirname(row.root)), generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({tag: row.tag, name: row.name, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({tag: row.tag, name: row.name, test, error: String((error as Error).message ?? error).slice(0, 600)}));
    }
  }
}
