// 🧮️ uicarve closure census of one crate per feature set: bun census.ts <crate-dir-rel-to-root> [--list=<cfgLabel>]
import {readFileSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const dir = resolve(root, process.argv[2]!);
const listLabel = process.argv.find(a => a.startsWith("--list="))?.slice(7);
const toml = readFileSync(resolve(dir, "Cargo.toml"), "utf8");
const parsed = Bun.TOML.parse(toml) as any;
const lib = parsed.lib?.path ?? "src/lib.rs";
const libRoot = resolve(dir, lib);
const limits = {bytes: +(toml.match(/maxSourceBytes\s*=\s*(\d+)/)?.[1] ?? 262144), files: +(toml.match(/maxSourceFiles\s*=\s*(\d+)/)?.[1] ?? 32), lines: +(toml.match(/maxActiveLines\s*=\s*(\d+)/)?.[1] ?? 4000)};
const table: Record<string, string[]> = parsed.features ?? {};
const expand = (names: string[]) => {
  const out = new Set<string>(), queue = [...names];
  while (queue.length) { const n = queue.pop()!; if (out.has(n)) continue; out.add(n); for (const t of table[n] ?? []) { if (t.startsWith("dep:")) continue; const h = t.split("/")[0]!.replace(/\?$/u, ""); if (table[h]) queue.push(h); } }
  return [...out];
};
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => { try { return readFileSync(path); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const all = Object.keys(table);
const extraSets = (process.argv.find(a => a.startsWith("--sets="))?.slice(7) ?? "").split(";").filter(Boolean).map(s => s.split(","));
const sets: [string, string[]][] = [["default", []], ...all.filter(f => f !== "default").map(f => [f, [f]] as [string, string[]]), ...extraSets.map(s => [s.join("+"), s] as [string, string[]]), ["ALL", all.filter(f => f !== "default")]];
for (const [label, feats] of sets) for (const test of [false, true]) {
  const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : []), ...expand(feats).filter(f => f !== "default").map(f => `feature=${f}`)];
  try {
    const inputs = await readRustSourceClosureV1([libRoot], root, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
    const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
    const ratio = +Math.max(files / limits.files, bytes / limits.bytes, lines / limits.lines).toFixed(3);
    console.log(JSON.stringify({label, test, files, bytes, lines, limits, ratio}));
    if (listLabel === label + (test ? ":test" : "")) writeFileSync(resolve(process.cwd(), `list-${label}${test ? "-test" : ""}.tsv`), inputs.map(i => `${i.activeLines}\t${i.bytes}\t${i.path}`).join("\n"));
  } catch (e) { console.log(JSON.stringify({label, test, error: String((e as Error).message ?? e).slice(0, 300)})); }
}
