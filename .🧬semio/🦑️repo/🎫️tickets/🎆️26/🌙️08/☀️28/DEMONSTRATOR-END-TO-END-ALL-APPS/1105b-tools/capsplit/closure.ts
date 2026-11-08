import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFileSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [manifest, featuresArg = "", platform = "", testArg = ""] = process.argv.slice(2);
const dir = dirname(join(ROOT, manifest));
const toml: any = Bun.TOML.parse(readFileSync(join(ROOT, manifest), "utf8"));
const args = ["--print", "cfg", "-C", "opt-level=0"];
if (platform) args.push("--target", platform);
if (testArg === "test") args.push("--test");
const base = Bun.spawnSync(["rustc", ...args]).stdout.toString().trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...base, ...featuresArg.split(",").filter(Boolean).map(f => `feature=${f}`)];
const entry = join(dir, toml.lib?.path ?? "src/lib.rs");
const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, async p => { try { return new Uint8Array(readFileSync(p)); } catch { return null; } }, { manifestDirectory: dir, generated: () => undefined });
let total = 0;
for (const i of [...inputs].sort((a, b) => b.bytes - a.bytes)) { total += i.bytes; console.log(String(i.bytes).padStart(7), String(i.activeLines).padStart(5), i.path.replace(ROOT + "/", "").slice(-110)); }
console.log("TOTAL", total, "files", inputs.length);
