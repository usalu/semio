// 🧮️ Source-closure rows (lib, test, wasm32 browser, wasip2) of the surface crates via the canonical counter. usage: bun census.ts <entry> <pkgDir> [feature...]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const [entry, dir, ...features] = process.argv.slice(2);
const feats = features.map(f => `feature=${f}`);
const rows: [string, string[]][] = [
  ["lib", [...cfgOf(), "debug_assertions", ...feats]],
  ["test", [...cfgOf(), "debug_assertions", "test", ...feats]],
  ["wasm32", [...cfgOf("wasm32-unknown-unknown"), "debug_assertions", ...feats]],
  ["wasip2", [...cfgOf("wasm32-wasip2"), "debug_assertions", ...feats]],
];
for (const [label, cfg] of rows) {
  try {
    const inputs = await readRustSourceClosureV1([entry!], ROOT, cfg, read, {manifestDirectory: dir!, generated: () => undefined});
    const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
    console.log([label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : (inputs.length > 28 || bytes > 230000 || lines > 3500) ? "over-target" : "ok"].join("\t"));
    if (process.env.LIST) for (const i of inputs) console.log("   ", i.path.split("🗺️surface/").pop(), i.bytes, i.activeLines);
  } catch (e) { console.log([label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
}
