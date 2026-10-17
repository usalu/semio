// 🧾️ Lists the source closure (canonical reader) of one crate entry: usage `bun closure.ts <entry.rs> <manifestDir> [test]`
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [entry, dir, mode] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
cfg.push("debug_assertions"); if (mode === "test") cfg.push("test");
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry!], ROOT, cfg, readFile, {manifestDirectory: dir!, generated: () => undefined});
let files = 0, bytes = 0, lines = 0;
for (const i of inputs as any[]) { files++; bytes += i.bytes; lines += i.activeLines; console.log([i.kind, i.bytes, i.activeLines, String(i.path ?? i.file ?? i.name ?? "").replace(ROOT + "/", "")].join("\t")); }
console.log(`TOTAL files ${files} bytes ${bytes} lines ${lines}`);
