import { createRequire } from 'node:module';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const require = createRequire(resolve('package.json'));
const Ajv = require('ajv');
const isEqual = require('lodash/isEqual');
const base = '🧰️framework/🔨️modules/🌱️value/📋️list/🎟️reservation/🔎️selection/';
const fixture = JSON.parse(readFileSync(base + '🧪️tests/🔣️.json', 'utf8'));
const schema = JSON.parse(readFileSync(base + '🧬️schema/🔣️.json', 'utf8'));
const validate = new Ajv().compile(schema);
if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));

export function select(state, limit) {
  if (limit <= state.capacity) return { kind: 'none', depth: 0, selectionWork: 1, stagedSelectionWork: 1, reserveWork: 0 };
  const page = state.capacity;
  let height = 0;
  for (let n = page; n > 0; n = Math.floor(n / 16)) height++;
  const rootHeight = state.root?.height ?? 0;
  const depth = Math.max(rootHeight, height) + 3;
  if (height > rootHeight) return { kind: 'metadata', depth, grow: true, selectionWork: 3 + height, stagedSelectionWork: 2, reserveWork: 22 };
  let node = state.root, branches = 0;
  for (let h = rootHeight; h >= 0; h--) {
    if (!node) return { kind: 'metadata', depth, selectionWork: 4 + height + 2 * branches, stagedSelectionWork: 3 + 2 * branches, reserveWork: 2 * branches + (h === 0 ? 5 : 21) };
    if (node.height === 0) return { kind: 'payload', depth, selectionWork: 4 + height + 2 * branches, stagedSelectionWork: 3 + 2 * branches, reserveWork: 7 + 2 * branches };
    branches++;
    node = node.children[Math.floor(page / 16 ** (h - 1)) % 16];
  }
  throw new Error('invalid tree');
}
export function reserve(state, limit) {
  const choice = select(state, limit);
  if (choice.kind === 'none') return;
  if (choice.grow) {
    const children = Array(16).fill(null);
    children[0] = state.root;
    state.root = { height: (state.root?.height ?? 0) + 1, children };
    return;
  }
  const page = state.capacity;
  let parent = null, index = 0, node = state.root;
  const rootHeight = node?.height ?? 0;
  for (let h = rootHeight; h >= 0; h--) {
    if (!node) {
      node = { height: h, children: h ? Array(16).fill(null) : undefined };
      if (parent) parent.children[index] = node; else state.root = node;
      return;
    }
    if (!node.height) { state.capacity++; return; }
    parent = node;
    index = Math.floor(page / 16 ** (h - 1)) % 16;
    node = parent.children[index];
  }
}
for (const row of fixture.cases) {
  const state = { root: null, capacity: 0 };
  for (let i = 0; i < row.rows; i++) {
    while (state.capacity <= i) reserve(state, i + 1);
  }
  for (let i = 0; i < row.extraReserveSteps; i++) reserve(state, row.limit);
  const actual = select(state, row.limit);
  if (!isEqual({ kind: actual.kind, depth: actual.depth, selectionWork: actual.selectionWork, stagedSelectionWork: actual.stagedSelectionWork, reserveWork: actual.reserveWork }, { kind: row.kind, depth: row.depth, selectionWork: row.selectionWork, stagedSelectionWork: row.stagedSelectionWork, reserveWork: row.reserveWork })) {
    throw new Error(`${row.id}: ${JSON.stringify(actual)}`);
  }
}
console.log(`page selection fixture oracle passed ${fixture.cases.length} cases`);

const releaseBase = '🧰️framework/🔨️modules/🌱️value/📋️list/🎟️reservation/♻️release/';
const releaseFixture = JSON.parse(readFileSync(releaseBase + '🧪️tests/🔣️.json', 'utf8'));
const releaseSchema = JSON.parse(readFileSync(releaseBase + '🧬️schema/🔣️.json', 'utf8'));
const validateRelease = new Ajv().compile(releaseSchema);
if (!validateRelease(releaseFixture)) throw new Error(JSON.stringify(validateRelease.errors));
function releaseSelection(state) {
  if (!state.root) return { kind: 'none', depth: 0, selectionWork: 1, releaseWork: 0 };
  let node = state.root, parent = null, index = 0, links = 0, inspections = 1;
  while (true) {
    inspections++;
    if (!node.height) {
      const payload = !node.empty && node.payload !== false;
      return { kind: payload ? 'payload' : 'metadata', depth: links + (payload ? 3 : 2), selectionWork: inspections + 1, releaseWork: 2 + 2 * links + (payload ? 5 : 3), node, parent, index };
    }
    let i = 15;
    while (i >= 0) { inspections++; if (node.children[i]) break; i--; }
    if (i < 0) return { kind: 'metadata', depth: links + 2, selectionWork: inspections + 1, releaseWork: 2 + 2 * links + 19, node, parent, index };
    inspections++; links++; parent = node; index = i; node = node.children[i];
  }
}
for (const row of releaseFixture.cases) {
  const state = { root: null, capacity: 0 };
  for (let i = 0; i < row.rows; i++) while (state.capacity <= i) reserve(state, i + 1);
  for (let i = 0; i < row.extraReleaseSteps; i++) {
    const selected = releaseSelection(state);
    if (selected.kind === 'none') throw new Error(`${row.id}: release past empty`);
    if (selected.kind === 'payload') selected.node.payload = false;
    else if (selected.parent) selected.parent.children[selected.index] = null;
    else state.root = null;
  }
  if (row.retainedEmptyMetadata) {
    const selected = releaseSelection(state);
    if (selected.kind !== 'metadata' || selected.node.height) throw new Error(`${row.id}: expected leaf metadata`);
    const empty = { empty: true, height: 0 };
    if (selected.parent) selected.parent.children[selected.index] = empty;
    else state.root = empty;
  }
  const actual = releaseSelection(state);
  const fields = ({ kind, depth, selectionWork, releaseWork }) => ({ kind, depth, selectionWork, releaseWork });
  if (!isEqual(fields(actual), fields(row))) throw new Error(`${row.id}: ${JSON.stringify(fields(actual))} != ${JSON.stringify(fields(row))}`);
}
console.log(`page release fixture oracle passed ${releaseFixture.cases.length} cases`);
