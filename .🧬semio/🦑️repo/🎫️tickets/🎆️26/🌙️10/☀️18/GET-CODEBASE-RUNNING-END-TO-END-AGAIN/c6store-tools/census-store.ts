// 🧮️ c6store own-closure census: canonical closure counter (`readRustSourceClosureV1`) over the lib root of each crate in census-crates.json (lib and cfg(test) closures); writes nothing. usage: bun census-store.ts [name-filter]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve, dirname} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = "/Users/ueli/Documents/semio";
const here = import.meta.dir;
const crates: {name: string; root: string; mdir: string; tags: string[]}[] = JSON.parse(readFileSync(resolve(here, "census-crates.json"), "utf8"));
const filter = process.argv[2];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const c of crates) {
  if (filter && !c.name.includes(filter) && !c.tags.includes(filter)) continue;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([resolve(REPO, c.root)], REPO, cfg, readFile, {manifestDirectory: resolve(REPO, c.mdir), generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({tags: c.tags.join(","), name: c.name, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({tags: c.tags.join(","), name: c.name, test, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
