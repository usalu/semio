// 🔎 Survey pass for lane R2 (norm plugin, 111 leaves): classifies every remaining 🔺️diff leaf as
// matching the uniform single-field scalar shape (`XDiff { field: Some(payload.field), ..Default::default() }`)
// vs needing hand conversion. Read-only — writes nothing.
import { readFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const LEAVES_FILE = "/private/tmp/claude-501/-Users-ueli-Documents-semio/5cc87afa-6e10-4e82-9978-b6cf7408b8ed/scratchpad/norm-leaves.txt";

const DIFF_FN_RE =
  /pub fn diff\(payload: &(\w+), _?base: &(\w+)\) -> (\w+) \{\n\s*\3 \{ (\w+): Some\(payload\.(\w+)(?:\.clone\(\))?\), \.\.Default::default\(\) \}\n\}/;

const lines = readFileSync(LEAVES_FILE, "utf8").trim().split("\n");
let matched = 0;
let unmatched: string[] = [];

for (const rel of lines) {
  const full = join(ROOT, rel);
  const content = readFileSync(full, "utf8");
  if (DIFF_FN_RE.test(content)) {
    matched++;
  } else {
    unmatched.push(rel);
  }
}

console.log(`Total: ${lines.length}`);
console.log(`Matched uniform scalar shape: ${matched}`);
console.log(`Unmatched (${unmatched.length}):`);
for (const u of unmatched) console.log(`  ${u}`);
