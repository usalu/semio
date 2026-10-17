// 🔍️ Lists every input of one Rust target closure measured by the canonical reader. usage: bun closure.ts <crate dir> <entry file relative to dir or abs> [lib]
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [dir, entryArg, mode] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...hostCfg, "debug_assertions", ...(mode === "lib" ? [] : ["test"]), ...(process.env.FEATURES ?? "").split(",").filter(Boolean).map(f => "feature=" + f)];
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const d = resolve(ROOT, dir);
const toml = Bun.TOML.parse(readFileSync(resolve(d, "Cargo.toml"), "utf8")) as any;
const entry = entryArg === "LIB" ? (toml.lib?.path ?? "src/lib.rs") : entryArg!.startsWith("test:") ? toml.test.find((t: any) => t.name === entryArg!.slice(5)).path : entryArg!;
const inputs = await readRustSourceClosureV1([resolve(d, entry)], ROOT, cfg, readFile, {manifestDirectory: d, generated: () => undefined});
let b = 0, l = 0;
for (const i of [...inputs].sort((x, y) => y.bytes - x.bytes)) { b += i.bytes; l += i.activeLines; console.log([i.kind, i.bytes, i.activeLines, i.path.replace(ROOT + "/", "")].join("\t")); }
console.log(`TOTAL files=${inputs.length} bytes=${b} lines=${l}`);
