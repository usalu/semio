// 🧮️ Lists or sums the canonical source closure of one lib root under extra cfg flags (argv: entry pkgDir [list] cfg...).
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.OVERLAY ?? "/Users/ueli/Documents/semio";
const [entry, pkgDir, mode, ...extra] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry!], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: pkgDir!, generated: () => undefined});
const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
if (mode === "list") for (const i of inputs) console.log([i.path.replace(ROOT + "/", ""), i.kind, i.bytes, i.activeLines].join("\t"));
console.log(["TOTAL files", inputs.length, "bytes", bytes, "lines", lines, (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : "ok"].join("\t"));
