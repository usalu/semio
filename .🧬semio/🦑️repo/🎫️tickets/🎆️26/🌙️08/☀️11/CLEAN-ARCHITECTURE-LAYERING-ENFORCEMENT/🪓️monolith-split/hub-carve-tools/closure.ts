// 🧮️ Dumps the source closure (file, kind, bytes, activeLines) of one entry under a cfg selection, one row per file.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const lenient = process.env.LENIENT === "1";
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return lenient && !p.endsWith(".rs") ? new Uint8Array() : null; throw e; } };
const entry = resolve(process.argv[2]!);
const manifestDir = resolve(process.argv[3]!);
const extra = (process.argv[4] ?? "").split(",").filter(Boolean);
const inputs = await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: manifestDir, generated: () => undefined});
for (const i of inputs) console.log([i.kind, i.bytes, i.activeLines, (i as any).path ?? (i as any).file ?? JSON.stringify(Object.keys(i))].join("\t"));
