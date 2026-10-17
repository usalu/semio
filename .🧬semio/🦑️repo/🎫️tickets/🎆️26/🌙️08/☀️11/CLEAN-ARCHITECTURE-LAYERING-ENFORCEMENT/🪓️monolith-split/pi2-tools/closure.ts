// 🧮️ pi2 closure lister: prints each file of a crate's lib (or test) closure with bytes and active lines; env PKGDIR (dir of Cargo.toml), CFG=lib|test, FEATURES=comma list.
import {readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const dir = resolve(ROOT, process.env.PKGDIR!);
const parsed = Bun.TOML.parse(readFileSync(join(dir, "Cargo.toml"), "utf8")) as any;
const entry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
const feats = (process.env.FEATURES ?? "").split(",").filter(Boolean).map(f => "feature=" + f);
const cfg = [...hostCfg, "debug_assertions", ...feats, ...(process.env.CFG === "test" ? ["test"] : [])];
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as any).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
let b = 0, l = 0;
for (const i of inputs.sort((a: any, c: any) => c.bytes - a.bytes)) { b += i.bytes; l += i.activeLines; console.log(`${i.bytes}\t${i.activeLines}\t${i.kind}\t${i.path.replace(ROOT + "/", "")}`); }
console.log(`TOTAL files=${inputs.length} bytes=${b} lines=${l}`);
