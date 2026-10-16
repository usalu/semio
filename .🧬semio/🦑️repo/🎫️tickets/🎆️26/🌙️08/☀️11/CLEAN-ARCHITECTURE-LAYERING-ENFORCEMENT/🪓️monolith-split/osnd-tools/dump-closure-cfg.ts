// 🧾️ Lists the canonical closure members for one entry under extra cfg tokens (argv: entry pkgDir cfg,cfg,...).
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [entry, pkgDir, extra = ""] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry!], ROOT, [...cfg, "debug_assertions", ...extra.split(",").filter(Boolean)], read, {manifestDirectory: pkgDir!, generated: () => undefined});
for (const i of inputs) console.log([i.path, i.kind, i.bytes, i.activeLines].join("\t"));
