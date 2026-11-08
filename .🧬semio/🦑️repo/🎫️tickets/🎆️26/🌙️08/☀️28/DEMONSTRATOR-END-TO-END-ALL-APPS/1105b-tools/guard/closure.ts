import { readFileSync, existsSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const [entry, ...features] = process.argv.slice(2);
const root = "/Users/ueli/Documents/semio";
const args = ["--print", "cfg", "--test", ...features.flatMap(f => ["--cfg", `feature="${f}"`])];
const cfg = execFileSync("rustc", args, { encoding: "utf8" }).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const files = await readRustSourceClosureV1([entry], root, cfg, async p => existsSync(p) ? new Uint8Array(readFileSync(p)) : null);
let total = 0;
for (const f of [...files].sort((a, b) => b.bytes - a.bytes)) { total += f.bytes; }
console.log("files", files.length, "bytes", total, "limit 262144 / 32");
for (const f of [...files].sort((a, b) => b.bytes - a.bytes).slice(0, 40)) console.log(f.bytes, f.path);
