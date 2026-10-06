import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync, renameSync, existsSync, appendFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createHash } from "node:crypto";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const owner = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
const groups: [string, string[]][] = [
  [owner, ["📢️member-publication.json", "🎯️group-cursor.json", "📖️group-read.json", "🌱️runtime-seed.json"]],
  [owner + "/👥️presence", ["🧹️retirement.json", "📌️peer-commit.json", "🛂️peer-admission.json"]],
];
const moves = groups.flatMap(([base, names]) => names.map(name => ({ old: base + "/" + name, new: base + "/🧫️fixtures/" + name, sha256: "" })));
const cad = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts";
const edits: [string, string[]][] = [
 [owner + "/🧪️tests/🔬️unit/🦀️.rs", groups[0]![1]],
 [owner + "/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs", ["🧹️retirement.json", "📌️peer-commit.json"]],
 [owner + "/👥️presence/🚫️rejection/🧪️tests/🔬️unit/🦀️.rs", ["🛂️peer-admission.json"]],
 [owner + "/🧪️tests/👁️group-visibility/🟦️.ts", ["📖️group-read.json", "🎯️group-cursor.json"]],
 [cad, groups[1]![1]],
];
for (const row of moves) { assert(!existsSync(join(root, row.new)), row.new); row.sha256 = createHash("sha256").update(readFileSync(join(root, row.old))).digest("hex"); }
for (const [path, names] of edits) for (const name of names) assert(readFileSync(join(root, path), "utf8").includes(name), path + ": " + name);
for (const row of moves) { mkdirSync(dirname(join(root, row.new)), { recursive: true }); renameSync(join(root, row.old), join(root, row.new)); assert.equal(createHash("sha256").update(readFileSync(join(root, row.new))).digest("hex"), row.sha256); }
for (const [path, names] of edits) {
  const absolute = join(root, path); let source = readFileSync(absolute, "utf8");
  for (const name of names) source = source.replaceAll(name + '"', "🧫️fixtures/" + name + '"');
  writeFileSync(absolute, source);
}
appendFileSync(join(ticket, "📓️framework-store-corpus-followup-2026-09-09.md"), "# Framework Store Test Corpus Ownership\n\nSeven Store and Presence JSON corpora encode test inputs and expected lifecycle outcomes. They now belong to canonical fixture folders at their Store or Presence semantic scope. Their schema declarations remain unchanged. All Rust includes and the Group visibility and CAD presence TypeScript readers now use those current paths.\n\n## Preserved Moves\n\n```json\n" + JSON.stringify(moves, null, 2) + "\n```\n\n## Authored Paths\n\n```json\n" + JSON.stringify([...new Set([...moves.flatMap(row => [row.old, row.new]), ...edits.map(row => row[0])])].sort(), null, 2) + "\n```\n");
console.log("[DEBUG] " + JSON.stringify({ moved: moves.length, readers: edits.length, byteChanges: 0 }));

