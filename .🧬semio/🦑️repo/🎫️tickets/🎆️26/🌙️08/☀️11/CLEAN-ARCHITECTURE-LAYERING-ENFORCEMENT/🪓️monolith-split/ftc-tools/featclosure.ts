// 📏 Source closure of a crate's lib under an explicit feature set (the set a companion requests), natively and for both wasm triples, through the canonical counter.
import {readFileSync, existsSync} from "node:fs";
import {join, resolve, dirname} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [manifest, featList = ""] = process.argv.slice(2);
const dir = dirname(join(ROOT, manifest)), parsed = Bun.TOML.parse(readFileSync(join(ROOT, manifest), "utf8")) as any;
const entry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
for (const [label, triple] of [["native", undefined], ["wasm32-unknown-unknown", "wasm32-unknown-unknown"], ["wasm32-wasip2", "wasm32-wasip2"]] as const) {
  const cfg = execFileSync(rustc, ["--print", "cfg", ...(triple ? ["--target", triple] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
  cfg.push("debug_assertions", ...featList.split(",").filter(Boolean).map(f => "feature=" + f));
  const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, async (p: string) => { try { return readFileSync(p); } catch { return null; } }, {manifestDirectory: dir, generated: () => undefined});
  console.log(`${parsed.package.name} [${featList || "default"}] ${label}: files ${inputs.length} bytes ${inputs.reduce((s: number, i: any) => s + i.bytes, 0)} lines ${inputs.reduce((s: number, i: any) => s + i.activeLines, 0)}`);
}
