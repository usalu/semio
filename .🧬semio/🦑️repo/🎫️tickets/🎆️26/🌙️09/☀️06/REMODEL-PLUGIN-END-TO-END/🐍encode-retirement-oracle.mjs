import { createRequire } from 'node:module';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const require = createRequire(resolve('package.json'));
const base = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/';
const input = JSON.parse(readFileSync(base + '🧫encode-retirement.json'));
const validate = new (require('ajv'))().compile(JSON.parse(readFileSync(base + '🧫encode-retirement.schema.json')));
if (!validate(input)) throw new Error(JSON.stringify(validate.errors));
const equal = require('lodash/isEqual');
const subtract = require('lodash/subtract');
const effect = JSON.parse(readFileSync('🧰️framework/🔨️modules/🌱️value/🛫️encode/🎟️effect/🧪️tests/🧫️.json'));
const settlement = JSON.parse(readFileSync('🧰️framework/🔨️modules/🌱️value/🛫️encode/🎟️action/🧪️tests/🧫️settlement.json'));
if (effect.find(row => row.name === 'independent refusals retained').expectedCauses !== 4 || settlement.cases.length !== 2) throw new Error('source corpus changed');
for (const row of input.cases) {
  if (row.kind === "settlement" && subtract(row.selectedWork,row.actualWork) !== row.unspentWork) throw new Error(row.id + " actual work");
  const original = Object.fromEntries(row.fields.map((field, index) => [field, `original-${index}`]));
  const retained = JSON.parse(JSON.stringify(original));
  if (!equal(original, retained) || row.closeAllowed === row.pending || row.birthHeap !== (row.kind === 'pauseRefused' ? null : 0)) throw new Error(row.id);
}
console.log(`encoding retirement fixture model passed ${input.cases.length} cases`);

const multiply = require('lodash/multiply');
const allocationHeaders = JSON.parse(readFileSync('🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🧫️fixtures/🔣️.json')).retainedCloseHeaders;
for (const header of allocationHeaders) {
 if (multiply(header.nativeWordBytes,2) !== header.ownerSlotHeaderBytes || multiply(header.ownerSlotHeaderBytes,1) !== header.ownerSlotCloseCopyBytes || multiply(header.nativeWordBytes,3) !== header.optionalByteHeaderBytes) throw new Error('original erased return-slot copy currency');
}
console.log('erased return-slot model passed ' + allocationHeaders.length + ' native-word cases');
