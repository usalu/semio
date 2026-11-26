import assert from 'node:assert/strict';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import Ajv from 'ajv';

const packageRoot = resolve(process.argv[2]);
const puzzle = '✏️s/🔌️plugins/🧩️puzzle';
const artifacts = join(puzzle, '🗿️artifacts');
const exampleRoot = dimension => join(artifacts, dimension, '🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples');
const exampleDsl = (dimension, id) => {
  const root = exampleRoot(dimension);
  const directory = readdirSync(root).find(name => name.endsWith(id));
  assert(directory, id);
  const assets = join(root, directory, '🖼️assets');
  const model = readdirSync(assets, { withFileTypes: true }).find(entry => entry.isDirectory() && readdirSync(join(assets, entry.name)).includes('🗣️.dsl.semio'));
  assert(model, id);
  return readFileSync(join(assets, model.name, '🗣️.dsl.semio'), 'utf8');
};

const wasm = await import(pathToFileURL(join(packageRoot, 'semio_puzzle.js')).href);
await wasm.default({ module_or_path: readFileSync(join(packageRoot, 'semio_puzzle_bg.wasm')) });
console.log('[DEBUG] Initialized staged browser WASM');

const boardSnapshotJson = wasm.puzzle2dParseDslJson(exampleDsl('◻️2d', 'nakagin-capsule-tower'));
const boardSnapshot = JSON.parse(boardSnapshotJson);
assert.equal(boardSnapshot.nodes.length, 180);
assert.equal(boardSnapshot.edges.length, 179);
assert(boardSnapshot.nodes.some(node => 'anchor' in node));
assert(boardSnapshot.edges.some(edge => 'gap' in edge));
console.log('[DEBUG] Parsed the shipped Nakagin document with its domain fields');
if (process.argv[3] === '--emit-board-document') {
  writeFileSync(resolve(process.argv[4]), boardSnapshotJson);
  process.exit(0);
}
const projected = readFileSync(resolve(process.argv[3]), 'utf8');
const boardSchema = JSON.parse(readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/📸️snapshot/🔣️.json', 'utf8'));
const validate = new Ajv().compile(boardSchema);
const painted = JSON.parse(projected);
assert(validate(painted), JSON.stringify(validate.errors));
assert.deepEqual(painted.nodes.map(node => [node.id, node.x, node.y, node.handles]), boardSnapshot.nodes.map(node => [node.id, node.x, node.y, node.handles]));
assert.deepEqual(painted.edges.map(edge => [edge.id, edge.source, edge.target]), boardSnapshot.edges.map(edge => [edge.id, edge.source, edge.target]));
assert.deepEqual(painted.targetRegions, boardSnapshot.targetRegions);
assert.deepEqual(painted.meta, boardSnapshot.meta);
console.log('[DEBUG] Ajv validated the production board snapshot against the strict schema');
const board = new wasm.BoardSession();
try {
  assert.equal(board.loadBoardSnapshotJson(boardSnapshotJson), false);
  assert(board.loadBoardSnapshotJson(projected));
  console.log('[DEBUG] Loaded the unmodified production Nakagin board projection into the board session (180 nodes and 179 edges)');
  const source = { id: 'alpha', nodeKind: 'seed', x: 0, y: 0, shape: 'circle', radius: 44, handles: [] };
  assert(board.loadBoardSnapshotJson(JSON.stringify({ schema: 'board.ports.directed.v1', camera: { x: 0, y: 0, zoom: 1 }, nodes: [source], edges: [] })));
  board.setSize(800, 600, 1);
  board.setCameraSilent(source.x, source.y, 1);
  board.setActiveUtility('select');
  board.drainEventsJson();
  board.pointerDownScreen(400, 300, 0, false, false);
  board.pointerUpScreen(400, 300, false, false, false);
  const events = JSON.parse(board.drainEventsJson());
  assert(events.some(event => event.name === 'select' && event.payload.ids.includes(source.id)), JSON.stringify(events));
  board.pointerDownScreen(4, 4, 0, false, false);
  board.pointerUpScreen(4, 4, false, false, false);
  const cleared = JSON.parse(board.drainEventsJson());
  assert(cleared.some(event => event.name === 'select' && event.payload.ids.length === 0), JSON.stringify(cleared));
  console.log('[DEBUG] Selected a node and cleared selection on the board background');
} finally {
  board.free();
}

for (const [id, objects] of [['concrete-forest', 1], ['nakagin-capsule-tower', 180]]) {
  const world = JSON.parse(wasm.puzzle3dParseDslJson(exampleDsl('🧊️3d', id)));
  assert.equal(world.objects.length, objects);
  console.log(`[DEBUG] Parsed 3D ${id} with ${objects} objects`);
}

const exampleCounts = JSON.parse(readFileSync(join(exampleRoot('🖐️5d'), '../🧪️tests/🧫️fixtures/📚️example-counts/🔣️.json'), 'utf8'));
const worldCorpus = JSON.parse(readFileSync(join(puzzle, '🧪️tests/🧫️fixtures/🌐️world-session/🔣️.json'), 'utf8'));
for (const { id, parts, fasteners } of exampleCounts.examples) {
  const model = JSON.parse(wasm.puzzle5dParseDslJson(exampleDsl('🖐️5d', id)));
  assert.equal(model.parts.length, parts);
  assert.equal(model.fasteners.length, fasteners);
  if (id === 'nakagin-capsule-tower') assert.equal(model.parts[0].id, worldCorpus.nakaginTimeline.oldestPartId);
  console.log(`[DEBUG] Parsed 5D ${id} with ${parts} parts and ${fasteners} fasteners`);
}
