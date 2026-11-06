import { createRequire } from 'node:module';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { select, reserve } from './🐍page-oracle.mjs';
const require = createRequire(resolve('package.json'));
const equal = require('lodash/isEqual');
const base = '🧰️framework/🔨️modules/🌱️value/📋️list/🎟️reservation/🔎️selection/';
const corpus = JSON.parse(readFileSync(base + '🧪️tests/🔣️.json'));
const schema = JSON.parse(readFileSync(base + '🧬️schema/🔣️.json'));
const valid = new (require('ajv'))().compile(schema);
if (!valid(corpus)) throw new Error(JSON.stringify(valid.errors));
const ticket = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/';
const expected = JSON.parse(readFileSync(ticket + '🧫page-parent.json'));
const checkExpected = new (require('ajv'))().compile(JSON.parse(readFileSync(ticket + '🧫page-parent.schema.json')));
if (!checkExpected(expected)) throw new Error(JSON.stringify(checkExpected.errors));
const permanentBase = '🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🎟️reservation/🧪️tests/';
const permanent = JSON.parse(readFileSync(permanentBase + '🔣️.json'));
const checkPermanent = new (require('ajv'))().compile(JSON.parse(readFileSync(permanentBase + '🔣️.schema.json')));
if (!checkPermanent(permanent)) throw new Error(JSON.stringify(checkPermanent.errors));
for (const row of corpus.cases) {
  const state = { root: null, capacity: 0 };
  for (let i = 0; i < row.rows; i++) while (state.capacity <= i) reserve(state, i + 1);
  for (let i = 0; i < row.extraReserveSteps; i++) reserve(state, row.limit);
  const first = select(state, row.limit);
  if (!equal([first.kind, first.stagedSelectionWork, first.reserveWork], [row.kind, row.stagedSelectionWork, row.reserveWork])) throw new Error(`${row.id}: first frontier mismatch`);
  const phases = [], context = [row.rows, row.limit];
  let allocations = 0;
  for (;;) {
    phases.push(['HeaderRead', 4]);
    const selected = select(state, context[1]);
    phases.push(['PathSelect', selected.stagedSelectionWork + 4]);
    if (selected.kind === 'none') { phases.push(['Done', 0]); break; }
    phases.push(['Allocate', selected.reserveWork + 1], ['Clear', 2]);
    reserve(state, context[1]);
    if (++allocations > 64) throw new Error(`${row.id}: frontier did not settle`);
  }
  if (!phases.every(([, work]) => Number.isSafeInteger(work) && work >= 0)) throw new Error(`${row.id}: invalid phase work`);
  const wanted = expected.walks.find(entry => entry.id === row.id);
  if (!wanted || !equal(phases.filter(([phase]) => phase !== 'Done').map(([, work]) => work), wanted.work)) throw new Error(`${row.id}: parent work walk differs`);
  const nativeRow = permanent.cases.find(entry => entry.id === row.id);
  if (!nativeRow || !equal(nativeRow.work, wanted.work) || !equal(Object.fromEntries(Object.entries(nativeRow).filter(([key]) => key !== 'work')), row)) throw new Error(`${row.id}: native fixture differs`);
  console.log(`${row.id}: ${phases.map(([phase, work]) => `${phase}:${work}`).join(' ')}`);
}
const context = [1, 2];
if (!equal(context, [1, 2]) || equal(context, [2, 2])) throw new Error('context equality');
if (expected.cache.sameContextStorageReads !== 0 || expected.cache.activeMismatchStorageReads !== 0 || expected.cache.changedIndexFirstPhase !== 'HeaderRead' || expected.cache.closeWork !== 5) throw new Error('context/close law');
console.log(`paid page parent real-frontier model passed ${corpus.cases.length} cases`);
