import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { walkFiles, renamed, root } from "./lib.ts";
const dirs = new Set(renamed());
const files = walkFiles((n) => n.endsWith(".rs"));
console.error("rs files", files.length, "dirs", dirs.size);
const hits: Record<string, string[]> = {};
for (const f of files) {
  const t = readFileSync(f, "utf8");
  if (!t.includes("#[path")) continue;
  const re = /#\[path\s*=\s*"([^"]+)"\s*\]\s*((?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;)?/gu;
  for (const m of t.matchAll(re)) {
    const tgt = resolve(dirname(f), m[1]!);
    const d = dirname(tgt);
    if (dirs.has(d)) (hits[d] ??= []).push(`${f.slice(root.length + 1)} :: ${m[1]} :: ${m[3] ?? "(no mod)"}`);
  }
}
for (const d of dirs) console.log(d.slice(root.length + 1), "\n   ", (hits[d] ?? ["NO MOUNT"]).join("\n    "));
