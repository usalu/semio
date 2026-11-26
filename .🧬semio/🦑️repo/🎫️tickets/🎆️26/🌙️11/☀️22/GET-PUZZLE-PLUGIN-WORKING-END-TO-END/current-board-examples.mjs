import assert from 'node:assert/strict';
import { closeSync, openSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import Ajv from 'ajv';

const packageRoot = resolve(process.argv[2]);
const projection = process.argv[3];
const wasm = await import(pathToFileURL(join(packageRoot, 'semio_puzzle.js')).href);
await wasm.default({ module_or_path: readFileSync(join(packageRoot, 'semio_puzzle_bg.wasm')) });
console.log("[DEBUG] Browser module initialized");
const schema = JSON.parse(readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/📸️snapshot/🔣️.json', 'utf8'));
const validate = new Ajv().compile(schema);
const root = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples';
let count = 0;
for (const example of readdirSync(root)) {
  const assets = join(root, example, '🖼️assets');
  const doc = readdirSync(assets, { withFileTypes: true }).filter(entry => entry.isDirectory()).map(entry => join(assets, entry.name)).find(dir => readdirSync(dir).includes('🗣️.dsl.semio'));
  assert(doc, example);
  console.log("[DEBUG] Parsing", example);
  const input = wasm.puzzle2dParseDslJson(readFileSync(join(doc, '🗣️.dsl.semio'), 'utf8'));
  const snapshot = JSON.parse(input);
  console.log("[DEBUG] Projecting", example, input.length);
  const source = join(process.argv[4], example + '-source.json');
  const target = join(process.argv[4], example + '-painted.json');
  writeFileSync(source, input);
  const original = openSync(source, 'r');
  const output = openSync(target, 'w');
  const result = spawnSync(projection, [], { stdio: [original, output, 'pipe'], timeout: 15000 });
  closeSync(original);
  closeSync(output);
  assert.equal(result.status, 0, result.stderr);
  console.log("[DEBUG] Validating", example);
  const wire = readFileSync(target, 'utf8');
  const painted = JSON.parse(wire);
  assert(validate(painted), JSON.stringify(validate.errors));
  const board = new wasm.BoardSession();
  try {
    assert(board.loadBoardSnapshotJson(wire), example);
    assert.deepEqual(painted.nodes.map(row => row.id), snapshot.nodes.map(row => row.id));
    assert.deepEqual(painted.edges.map(row => row.id), snapshot.edges.map(row => row.id));
    console.log(example, painted.nodes.length, painted.edges.length);
    count++;
  } finally { board.free(); }
}
assert(count >= 2);
