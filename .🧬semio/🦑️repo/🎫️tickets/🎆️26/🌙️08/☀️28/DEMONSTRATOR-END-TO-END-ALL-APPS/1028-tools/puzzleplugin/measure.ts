import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { execFileSync } from "node:child_process";
const entry = process.argv[2];
const cfg = [...execFileSync("rustc", ["--print", "cfg", ...(process.argv[3] ? ["--target", process.argv[3]] : [])], { encoding: "utf8" }).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q))), "debug_assertions"];
const read = async (p: string) => { try { return readFileSync(p); } catch (e: any) { if (e.code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry], "/", cfg, read, { manifestDirectory: dirname(entry), generated: () => undefined });
let b = 0, l = 0;
for (const i of inputs) { b += i.bytes; l += i.activeLines; console.log(i.bytes, i.activeLines, i.path.replace("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/", "")); }
console.log("TOTAL files", inputs.length, "bytes", b, "lines", l);
