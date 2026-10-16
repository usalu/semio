// 🔎 Read-only scan of plain literal include_str!/include_bytes!/include!/#[path] inputs of tracked Rust files; reports the ones that do not exist.
import {existsSync, readFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";

const ROOT = "/Users/ueli/Documents/semio";
const scope = process.argv.slice(2);
const files = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-z", "--", ...scope], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28}).split("\0").filter(f => f.endsWith(".rs"));
const re = /(include_str|include_bytes|include)!\(\s*"([^"]+)"\s*\)|#\[path\s*=\s*"([^"]+)"\s*\]/gu;
let checked = 0;
for (const file of files) {
  let text: string;
  try { text = readFileSync(resolve(ROOT, file), "utf8"); } catch { continue; }
  for (const match of text.matchAll(re)) {
    const literal = match[2] ?? match[3]!;
    checked++;
    const target = resolve(dirname(resolve(ROOT, file)), literal);
    if (!existsSync(target)) console.log(["MISSING", file, match[1] ?? "path", literal].join("\t"));
  }
}
console.error(`checked ${checked} literals in ${files.length} files`);
