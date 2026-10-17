// 🧮️ c4p4 gate census: the source closure of one Cargo manifest lib/test target through the canonical counter; --base serves the pre-split text of the eight c4p4 files and hides their children.
import {readFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const manifest = resolve(ROOT, process.argv[2]!);
const withTest = process.argv.includes("--test");
const base = process.argv.includes("--base");
const overlay = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p4-tools/closure-overlay.json", "utf8")) as {orig: Record<string, string>; hide: string[]};
const hidden = new Set(overlay.hide);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...hostCfg, "debug_assertions", ...(withTest ? ["test"] : [])];
const readFile = async (p: string) => {
  if (base && hidden.has(p)) return null;
  const source = base ? overlay.orig[p] : undefined;
  try { return readFileSync(source ?? p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; }
};
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const dir = dirname(manifest);
const entry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
const files = inputs.length, bytes = inputs.reduce((s: number, i: any) => s + i.bytes, 0), lines = inputs.reduce((s: number, i: any) => s + i.activeLines, 0);
console.log(JSON.stringify({name: parsed.package?.name, test: withTest, base, files, bytes, lines, ok: files <= 32 && bytes <= 262144 && lines <= 4000}));
