// 📃️ Per-file listing of one crate's test closure (canonical counter): bytes and lines, largest first.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const entry = process.env.ENTRY!, pkg = process.env.PKG!;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry], ROOT, [...cfg, "debug_assertions", ...(process.env.TEST ? ["test"] : [])], read, {manifestDirectory: pkg, generated: () => undefined});
for (const i of [...inputs].sort((a, b) => b.bytes - a.bytes)) console.log(i.kind, i.bytes, i.activeLines, ((i as any).path ?? (i as any).file ?? "").replace(ROOT + "/", "").replace("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/", "P/"));
