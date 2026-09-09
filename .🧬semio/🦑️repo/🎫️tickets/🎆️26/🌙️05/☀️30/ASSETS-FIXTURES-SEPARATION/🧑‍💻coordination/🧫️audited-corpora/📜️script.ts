import assert from "node:assert/strict";
import { mkdirSync, readFileSync, renameSync, writeFileSync, existsSync, readdirSync, rmdirSync, appendFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createHash } from "node:crypto";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir));
const plugins = "✏️s/🔌️plugins/", sub = "/🏅️standards/🔖️1/🪆️subsets/✳️any";
const flow = plugins + "🌊️flow", cad = plugins + "📐️cad/🗿️artifacts/📐️cad" + sub + "/✏️editor/👥️presence";
const writer = plugins + "✒️writer/🗿️artifacts/✒️writer" + sub + "/✏️editor", sourcing = plugins + "🪵️sourcing/🗿️artifacts/🗂️curation" + sub;
const pairs = [
  [flow + "/🎬️action-cohort/🔣️.json", flow + "/🧫️fixtures/🎬️action-cohort/🔣️.json"],
  [cad + "/🧪️retirement.json", cad + "/🧫️fixtures/♻️retirement/🔣️.json"],
  [writer + "/📚️examples/🎬️demo-session/🔣️.json", writer + "/🧫️fixtures/🎬️writer-migration/🔣️.json"],
  [sourcing + "/📚️examples/🎬️demo/📦️expected-stock.json", sourcing + "/🧫️fixtures/📦️expected-stock.json"],
];
const edits: [string, string, string][] = [
 [flow + "/📦️packages/🟦️typescript/📜️script.ts", '"🎬️action-cohort/🔣️.json"', '"🧫️fixtures/🎬️action-cohort/🔣️.json"'],
 [flow + "/📦️packages/🟦️typescript/📜️script.ts", '"🧪️action-cohort/🔣️.json"', '"🧫️fixtures/🧪️action-cohort/🔣️.json"'],
 [flow + "/🗿️artifacts/🌊️flow" + sub + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", '/🎬️action-cohort/🔣️.json"', '/🧫️fixtures/🎬️action-cohort/🔣️.json"'],
 [flow + "/🗿️artifacts/🌊️flow" + sub + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", '/🗒️note/🧪️action-cohort/🔣️.json"', '/🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json"'],
 [cad + "/🧪️tests/🔬️cad-presence-retirement/🟦️.ts", '"🧪️retirement.json"', '"🧫️fixtures/♻️retirement/🔣️.json"'],
 [cad + "/♻️retirement/🧪️tests/🔬️unit/🦀️.rs", '/🧪️retirement.json"', '/🧫️fixtures/♻️retirement/🔣️.json"'],
 [dirname(cad) + "/🧪️tests/🔬️unit/🦀️.rs", '/👥️presence/🧪️retirement.json"', '/👥️presence/🧫️fixtures/♻️retirement/🔣️.json"'],
 [writer + "/🧪️tests/🔬️unit/🦀️.rs", '../../📚️examples/🎬️demo-session/🔣️.json', '../../🧫️fixtures/🎬️writer-migration/🔣️.json'],
 [sourcing + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs", '/📚️examples/🎬️demo/📦️expected-stock.json', '/🧫️fixtures/📦️expected-stock.json'],
 [sourcing + "/🚪️io/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs", '/📚️examples/🎬️demo/📦️expected-stock.json', '/🧫️fixtures/📦️expected-stock.json'],
];
for (const [path, before] of edits) assert(readFileSync(join(root, path), "utf8").includes(before), path + ": " + before);
const moves = pairs.map(([old, next]) => { assert(!existsSync(join(root, next!)), next); return { old, new: next, sha256: createHash("sha256").update(readFileSync(join(root, old!))).digest("hex") }; });
for (const row of moves) { mkdirSync(dirname(join(root, row.new!)), { recursive: true }); renameSync(join(root, row.old!), join(root, row.new!)); assert.equal(createHash("sha256").update(readFileSync(join(root, row.new!))).digest("hex"), row.sha256); const parent = dirname(join(root, row.old!)); if (!readdirSync(parent).length) rmdirSync(parent); }
for (const [path, before, after] of edits) { const absolute = join(root, path); writeFileSync(absolute, readFileSync(absolute, "utf8").replaceAll(before, after)); }
const paths = [...new Set([...pairs.flat(), ...edits.map(([path]) => path)])].sort();
appendFileSync(join(ticket, "📓️plugin-test-corpus-followup-2026-09-09.md"), "\n## Additional Audited Corpora\n\nFlow action-cohort, CAD presence retirement, Writer migration, and Sourcing expected-stock JSONs are testing-only examples. Their direct readers now use canonical fixtures. Flow's action-cohort schema remains a declaration, and its cross-plugin Note readers use Note's relocated fixture.\n\n### Preserved Moves\n\n```json\n" + JSON.stringify(moves, null, 2) + "\n```\n\n### Authored Paths\n\n```json\n" + JSON.stringify(paths, null, 2) + "\n```\n");
console.log("[DEBUG] " + JSON.stringify({ moved: moves.length, readers: new Set(edits.map(row => row[0])).size, byteChanges: 0 }));

