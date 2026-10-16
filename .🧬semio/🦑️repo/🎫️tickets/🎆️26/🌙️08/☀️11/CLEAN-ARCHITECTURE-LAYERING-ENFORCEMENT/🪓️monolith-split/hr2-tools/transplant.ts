/** 🔁️ hr2 digest transplant: replaces the sha256 of every rewritten file (old content) with the sha256 of its new content in every text file that pins it. Usage: bun transplant.ts <apply-result.json> [--write] */
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [resultPath, flag] = process.argv.slice(2);
const write = flag === "--write";
const result: { digests?: [string, string][] }[] = JSON.parse(readFileSync(resultPath!, "utf8"));
const map = new Map<string, string>();
for (const row of result) for (const [o, n] of row.digests ?? []) if (o !== n) map.set(o, n);
console.log(`digests=${map.size}`);
const patternFile = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/hr2-digest-patterns.txt");
writeFileSync(patternFile, [...map.keys()].join("\n") + "\n");
function git(args: string[]): string[] {
  try {
    return execFileSync("git", ["-c", "core.quotepath=false", ...args], { cwd: ROOT, maxBuffer: 1 << 30, stdio: ["ignore", "pipe", "ignore"] }).toString("utf8").split("\n").filter((x) => x);
  } catch (error) {
    return ((error as { stdout?: Buffer }).stdout?.toString("utf8") ?? "").split("\n").filter((x) => x);
  }
}
const tracked = git(["grep", "-l", "-F", "-f", patternFile, "--", ".", ":!.🧬semio"]);
const untracked = git(["ls-files", "-o", "--exclude-standard"]).filter((f) => !f.startsWith(".🧬semio/"));
const hits = new Set(tracked);
for (const f of untracked) { try { const text = readFileSync(join(ROOT, f), "utf8"); for (const k of map.keys()) if (text.includes(k)) { hits.add(f); break; } } catch {} }
let total = 0;
for (const file of [...hits].sort()) {
  const path = join(ROOT, file);
  const text = readFileSync(path, "utf8");
  let next = text;
  let count = 0;
  for (const [o, n] of map) { const parts = next.split(o); if (parts.length > 1) { count += parts.length - 1; next = parts.join(n); } }
  if (count === 0) continue;
  total += count;
  console.log(`${file}: ${count}`);
  if (write) { if (readFileSync(path, "utf8") !== text) throw new Error(`changed under us: ${file}`); writeFileSync(path, next); }
}
console.log(`files=${hits.size} replacements=${total}${write ? "" : " (dry run)"}`);
