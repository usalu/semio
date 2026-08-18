#!/usr/bin/env bun
/** [DEBUG] Undoes the unwanted `namedInputs.default` array reflow that
 * `d0-add-describe-command.ts`'s JSON.stringify round-trip introduced — collapses the exploded
 * 2-line array literal back to the original single-line form, leaving the new `describe` target
 * (this script's only intended change) as the sole real diff. Ticket-folder scratch, not repo code. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const pluginsDir = join(repoRoot, "✏️s/🔌️plugins");
const names = readdirSync(pluginsDir, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name);

let fixed = 0;
for (const name of names) {
  const projectPath = join(pluginsDir, name, "📦️packages/🦀️rust/📋️project.json");
  let src: string;
  try {
    src = readFileSync(projectPath, "utf8");
  } catch {
    continue;
  }
  const re = /"default": \[\n\s+("(?:[^"\\]|\\.)*"),\n\s+("(?:[^"\\]|\\.)*")\n\s+\]/;
  const m = src.match(re);
  if (!m) continue;
  const collapsed = `"default": [${m[1]}, ${m[2]}]`;
  const next = src.replace(re, collapsed);
  if (next !== src) {
    writeFileSync(projectPath, next);
    fixed++;
    console.log(`[DEBUG] collapsed namedInputs.default in ${name}`);
  }
}
console.log(`[DEBUG] done: ${fixed} files fixed`);
