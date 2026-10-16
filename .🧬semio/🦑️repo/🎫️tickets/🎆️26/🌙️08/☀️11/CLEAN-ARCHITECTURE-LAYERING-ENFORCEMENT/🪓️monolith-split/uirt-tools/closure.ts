// 🧮️ Lists the canonical source-closure members of one lib root for the lib or test cfg (cfg flags after the root).
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [entry, pkgDir, ...extra] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry!], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: pkgDir!, generated: () => undefined});
let bytes = 0, lines = 0;
for (const i of inputs) { bytes += i.bytes; lines += i.activeLines; if (!process.env.TOTAL) console.log([i.path.replace(ROOT + "/", "").replace(/^.*🧠️runtime\//, ""), i.kind, i.bytes, i.activeLines].join("\t")); }
console.log(["TOTAL", "files", inputs.length, "bytes", bytes, "lines", lines].join("\t"));
