import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdirSync, renameSync, existsSync, readdirSync, rmdirSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir));
const plugins = "✏️s/🔌️plugins/";
const sub = "🏅️standards/🔖️1/🪆️subsets/✳️any";
const roots = [
  ["💠️lowpoly", "🧪️interactive-job"],
  ["🧱️block", "🧪️publication-authority"],
  ["🖍️draw", "🧪️publication-authority"],
  ["➗️mathematical", "📣️publication-authority"],
  ["🧩️puzzle", "🔏️publication-authority"],
  ["🖨️raster", "🔏️publication-authority"],
  ["📐️cad/🗿️artifacts/📐️cad/" + sub, "🗄️retained-jobs"],
  ...["◻️2d", "🧊️3d", "🖐️5d"].map(dimension => ["🧩️puzzle/🗿️artifacts/" + dimension + "/" + sub, "🗄️retained-jobs"]),
];
const moves = roots.map(([owner, name]) => ({ old: plugins + owner + "/" + name + "/🔣️.json", new: plugins + owner + "/🧫️fixtures/" + name + "/🔣️.json", sha256: "" }));
const edits = new Map<string, [string, string][]>();
const edit = (path: string, old: string, next: string) => { const rows = edits.get(path) ?? []; rows.push([old, next]); edits.set(path, rows); };
for (const [plugin, name] of roots.slice(0, 5)) edit(plugins + plugin + "/📦️packages/🟦️typescript/📜️script.ts", '"' + name + (plugin === "💠️lowpoly" || plugin === "🧩️puzzle" ? "/🔣️.json" : "") + '"', '"🧫️fixtures/' + name + (plugin === "💠️lowpoly" || plugin === "🧩️puzzle" ? "/🔣️.json" : "") + '"');
edit(plugins + "📐️cad/📦️packages/🟦️typescript/📜️script.ts", sub + "/🗄️retained-jobs/🔣️.json", sub + "/🧫️fixtures/🗄️retained-jobs/🔣️.json");
edit(plugins + "📐️cad/🗿️artifacts/📐️cad/" + sub + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", "../../../🗄️retained-jobs/🔣️.json", "../../../🧫️fixtures/🗄️retained-jobs/🔣️.json");
for (const dimension of ["◻️2d", "🧊️3d", "🖐️5d"]) edit(plugins + "🧩️puzzle/🗿️artifacts/" + dimension + "/🦀️.rs", sub + "/🗄️retained-jobs/🔣️.json", sub + "/🧫️fixtures/🗄️retained-jobs/🔣️.json");
edit(plugins + "🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs", sub + "/🗄️retained-jobs/🔣️.json", sub + "/🧫️fixtures/🗄️retained-jobs/🔣️.json");
edit(plugins + "🧩️puzzle/🗿️artifacts/🧊️3d/" + sub + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", "../../../🗄️retained-jobs/🔣️.json", "../../../🧫️fixtures/🗄️retained-jobs/🔣️.json");
for (const row of moves) { assert(existsSync(join(root, row.old)), row.old); assert(!existsSync(join(root, row.new)), row.new); row.sha256 = createHash("sha256").update(readFileSync(join(root, row.old))).digest("hex"); }
for (const [path, rows] of edits) { const source = readFileSync(join(root, path), "utf8"); for (const [old] of rows) assert(source.includes(old), path + ": " + old); }
for (const row of moves) { const destination = join(root, row.new); mkdirSync(dirname(destination), { recursive: true }); renameSync(join(root, row.old), destination); assert.equal(createHash("sha256").update(readFileSync(destination)).digest("hex"), row.sha256); const oldDirectory = dirname(join(root, row.old)); if (readdirSync(oldDirectory).length === 0) rmdirSync(oldDirectory); }
for (const [path, rows] of edits) { let source = readFileSync(join(root, path), "utf8"); for (const [old, next] of rows) source = source.replaceAll(old, next); writeFileSync(join(root, path), source); }
const authored = [...new Set([...moves.flatMap(row => [row.old, row.new]), ...edits.keys(), relative(root, import.meta.path)])].sort();
const report = "# Plugin Test Corpus Ownership Follow-up\n\nTen JSON test corpora now live under their semantic owner's fixtures folder. Six publication/interactive-job examples use the plugin shared by their readers and source oracles; four retained-job examples use their artifact subset shared by the editor and retained-command tests. Every move preserves bytes. Raster's retained publication example has no active reader in the bounded audit, but its schema and source references identify it as test evidence, so it is retained as a fixture. The three extra Puzzle retained-job examples are included only by canonical tests or cfg(all(test, feature = component-app-assembly)) functions.\n\n## Preserved Moves\n\n```json\n" + JSON.stringify(moves, null, 2) + "\n```\n\n## Authored Paths\n\n```json\n" + JSON.stringify(authored, null, 2) + "\n```\n";
writeFileSync(join(ticket, "📓️plugin-test-corpus-followup-2026-09-09.md"), report);
console.log("[DEBUG] " + JSON.stringify({ moved: moves.length, updatedConsumers: edits.size, byteChanges: 0 }));

