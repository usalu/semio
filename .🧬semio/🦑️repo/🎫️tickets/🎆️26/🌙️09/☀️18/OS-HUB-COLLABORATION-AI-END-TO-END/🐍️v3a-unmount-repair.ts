#!/usr/bin/env bun
/** 🩹️ V3a repair: removes every mount block the FIRST (windowed-guard) projection run inserted, so the
 * fixed `mountProjectedRustLeaf` can re-insert them correctly. Matches only the exact five-line shape
 * that run emitted, whose `#[path]` targets a surface state lane's `🧬️schema/🦀️.rs`. */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const apply = process.argv.includes("--apply");

function rustFiles(root: string, out: string[] = []): string[] {
  if (!existsSync(root)) return out;
  for (const name of readdirSync(root).sort()) {
    if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
    const path = join(root, name);
    if (statSync(path).isDirectory()) rustFiles(path, out);
    else if (name.endsWith(".rs")) out.push(path);
  }
  return out;
}

let removed = 0;
const touched: string[] = [];
for (const file of rustFiles(join(repoRoot, "✏️s/🔌️plugins"))) {
  const lines = readFileSync(file, "utf8").split("\n");
  const keep: string[] = [];
  let index = 0;
  let fileRemoved = 0;
  while (index < lines.length) {
    const indent = /^(\s*)#\[path = "\."\]\s*$/u.exec(lines[index] ?? "")?.[1];
    const opens = indent !== undefined && new RegExp(`^${indent}pub mod (config|presence) \\{$`, "u").exec(lines[index + 1] ?? "");
    const mount = opens && new RegExp(`^${indent}    #\\[path = "[^"]*(🎚️config|👥️presence)/🧬️schema/🦀️\\.rs"\\]$`, "u").test(lines[index + 2] ?? "");
    const closes = mount && lines[index + 3] === `${indent}    pub mod schema;` && lines[index + 4] === `${indent}}`;
    if (closes) {
      index += 5;
      fileRemoved += 1;
      continue;
    }
    keep.push(lines[index]!);
    index += 1;
  }
  if (!fileRemoved) continue;
  removed += fileRemoved;
  touched.push(file.slice(repoRoot.length + 1));
  if (apply) writeFileSync(file, keep.join("\n"));
}
console.log(`${apply ? "removed" : "would remove"} ${removed} mount block(s) across ${touched.length} wiring file(s)`);
for (const file of touched.slice(0, 5)) console.log(`  ${file}`);
