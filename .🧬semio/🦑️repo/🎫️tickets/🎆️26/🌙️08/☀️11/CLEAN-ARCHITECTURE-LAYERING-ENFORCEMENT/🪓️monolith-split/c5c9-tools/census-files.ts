// 🧮️ c5c per-file closure rows of one crate (test closure by default). usage: bun census-files.ts <tag> [lib|test]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const crates = JSON.parse(readFileSync(new URL("crates.json", import.meta.url), "utf8")) as {tag: string; name: string; pkg: string}[];
const c = crates.find(x => x.tag === process.argv[2])!;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const manifestDirectory = resolve(root, c.pkg);
const manifest = Bun.TOML.parse(readFileSync(resolve(manifestDirectory, "Cargo.toml"), "utf8")) as any;
const entry = resolve(manifestDirectory, manifest.lib?.path ?? "src/lib.rs");
const test = (process.argv[3] ?? "test") === "test";
const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
const readFile = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const inputs = await readRustSourceClosureV1([entry], root, cfg, readFile, {manifestDirectory, generated: () => undefined});
let tl = 0;
for (const i of inputs) { tl += i.activeLines; console.log(String(i.activeLines).padStart(6), String(i.bytes).padStart(8), i.path.replace(root + "/", "").split("/").slice(-4).join("/")); }
console.log("total", inputs.length, tl);
