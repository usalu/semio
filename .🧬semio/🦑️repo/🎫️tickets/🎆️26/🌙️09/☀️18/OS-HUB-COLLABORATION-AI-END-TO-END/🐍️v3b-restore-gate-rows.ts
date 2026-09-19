/** ⚖️ Restores the six `4_gate` launch rows that commit `6f33e313da` deleted alongside
 * `INTERACTIVITY_ALL_APP_REQUIRED_GATES`, in the shape and order they carried at `6f33e313da^`.
 *
 * The seed is edited as TEXT, never re-serialized: `🚀️launch/🟦️.ts` `readSeed()` splits it on an exact
 * generator-authored marker line and peers (Z1/K2/V3a) are editing other parts of the same file. The
 * file is re-read immediately before writing, each row is inserted only if its name is absent, and
 * the insertion point is the end of the `configurations` array. Idempotent.
 *
 * Usage: `bun 🐍️v3b-restore-gate-rows.ts <repoRoot> [--apply]`
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = process.argv[2]!;
const apply = process.argv.includes("--apply");
const seedPath = join(repoRoot, ".vscode/🧩️launch.seed.jsonc");

const ROWS: readonly { name: string; command: string; order: number }[] = [
  { name: "⚖️gate⚡️interactivity", command: "bun nx run workspace:verify -- interactivity", order: 411.43 },
  { name: "⚖️gate⚡️interactivity🎯️tool-jobs", command: "bun nx run workspace:verify -- interactivity tool-jobs", order: 411.44 },
  { name: "⚖️gate⚡️interactivity🧭️apps", command: "bun nx run workspace:verify -- interactivity apps", order: 411.45 },
  { name: "⚖️gate⚡️interactivity🧭️apps🎛️actions", command: "bun nx run workspace:verify -- interactivity apps --actions", order: 411.46 },
  { name: "⚖️gate📦️dependencies", command: "bun nx run workspace:verify -- dependencies", order: 411.47 },
  { name: "⚖️gate📦️dependencies0️⃣", command: "bun nx run workspace:verify -- dependencies literal-external", order: 411.48 },
];

const raw = readFileSync(seedPath, "utf8");
const missing = ROWS.filter((row) => !raw.includes(`"${row.name}"`));
console.log(`gate rows: ${ROWS.length} present=${ROWS.length - missing.length} missing=${missing.length}`);
if (missing.length === 0) process.exit(0);

// 🎯️ The seed holds four top-level arrays (`configurations`, `compounds`, `inputs`, plus the split-off
// `devLaunchers` table). Only `configurations` may carry a launch row, so the anchor is that array's
// own terminator — the `],` that precedes `"compounds"` — never the last `],` in the file.
const marker = '\n  ],\n  "compounds": [';
const markerIndex = raw.indexOf(marker);
if (markerIndex < 0) throw new Error("seed: could not locate the end of the configurations array (the `],` before \"compounds\")");
if (raw.indexOf(marker, markerIndex + 1) >= 0) throw new Error("seed: the configurations terminator is ambiguous");

const block = missing
  .map((row) =>
    [
      "    {",
      `      "name": ${JSON.stringify(row.name)},`,
      '      "type": "node-terminal",',
      '      "request": "launch",',
      `      "command": ${JSON.stringify(row.command)},`,
      '      "cwd": "${workspaceFolder}",',
      '      "presentation": {',
      '        "group": "4_gate",',
      `        "order": ${row.order}`,
      "      }",
      "    }",
    ].join("\n"),
  )
  .join(",\n");

const updated = `${raw.slice(0, markerIndex)},\n${block}${raw.slice(markerIndex)}`;
for (const row of missing) console.log(`${apply ? "insert" : "would insert"} ${row.name}`);
if (!apply) process.exit(0);
writeFileSync(seedPath, updated);
console.log("seed updated");
