// 🧰 bd10: per-crate lib/test source closure of the raster crate roots with the canonical reader; prints JSON.
import {readFileSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, resolve, relative} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const art = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster";
const A = `${art}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
const crates: Record<string, string> = Object.fromEntries(Object.entries((JSON.parse(readFileSync(new URL("./crates.json", import.meta.url), "utf8")) as {crates: Record<string, {root: string}>}).crates).map(([k, v]) => [k, v.root]));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
const extra = process.argv.slice(3);
const out: Record<string, {root: string; lib: string[]; test: string[]; libEmbedded: string[]; testEmbedded: string[]; libStats: number[]; testStats: number[]; errors: string[]}> = {};
const onlyNames = process.env.ONLY?.split(",").filter(Boolean);
for (const [name, entry] of Object.entries(crates)) {
  if (onlyNames && !onlyNames.includes(name)) continue;
  const row = {root: entry, lib: [] as string[], test: [] as string[], libEmbedded: [] as string[], testEmbedded: [] as string[], libStats: [0, 0, 0], testStats: [0, 0, 0], errors: [] as string[]};
  for (const test of [false, true]) {
    try {
      const cfg = [...hostCfg, "debug_assertions", ...extra, ...(test ? ["test"] : [])];
      const inputs = await readRustSourceClosureV1([resolve(root, entry)], root, cfg, readFile, {manifestDirectory: dirname(resolve(root, entry)), generated: () => undefined}) as unknown as {path: string; kind: string; bytes: number; activeLines: number}[];
      (test ? row.test : row.lib).push(...inputs.filter(i => i.kind === "source").map(i => i.path));
      (test ? row.testEmbedded : row.libEmbedded).push(...inputs.filter(i => i.kind !== "source").map(i => i.path));
      (test ? row.testStats : row.libStats).splice(0, 3, inputs.length, inputs.reduce((a, i) => a + i.activeLines, 0), inputs.reduce((a, i) => a + i.bytes, 0));
    } catch (e) { row.errors.push((test ? "test: " : "lib: ") + String((e as Error).message ?? e).slice(0, 300)); }
  }
  out[name] = row;
}
writeFileSync(process.argv[2]!, JSON.stringify(out, null, 1));
for (const [n, r] of Object.entries(out)) console.log(n, "lib files/lines/bytes", r.libStats.join("/"), "test", r.testStats.join("/"), r.errors.join(" | "));
