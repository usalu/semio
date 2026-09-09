import { readFileSync } from "node:fs";
import { policyReadRustPolicySource, INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_GEOMETRY_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_ACTION_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE } from "/Users/ueli/Documents/semio/📜️script.ts";
const root = "/Users/ueli/Documents/semio";
const sources = [INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_GEOMETRY_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_ACTION_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE].map((p) => policyReadRustPolicySource(root, p));
for (const file of ["✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts", "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts"]) {
  const text = readFileSync(`${root}/${file}`, "utf8");
  console.log("==", file);
  for (const line of text.split("\n")) {
    const m = line.match(/^\s*\["([^"]+)",/);
    if (!m) continue;
    for (const call of line.matchAll(/\.replace\((("(?:[^"\\]|\\.)*")|('(?:[^'\\]|\\.)*')),/g)) {
      const literal = JSON.parse(call[1]!.startsWith("'") ? `"${call[1]!.slice(1, -1).replace(/"/g, '\\"')}"` : call[1]!);
      if (!sources.some((s) => s.includes(literal))) console.log("  DEAD MUTATION", m[1], "->", JSON.stringify(literal).slice(0, 110));
    }
  }
}
console.log("probe done");
