import { createRequire } from 'node:module';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const require = createRequire(resolve('package.json'));
const Ajv = require('ajv');
const isEqual = require('lodash/isEqual');
const base = '🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🎟️work/';
const schema = JSON.parse(readFileSync(base + '🧬️byte-work.schema.json', 'utf8'));
const fixture = JSON.parse(readFileSync(base + '🧫️byte-work.json', 'utf8'));
const validate = new Ajv().compile(schema);
if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
for (const row of fixture.cases) {
  if (isEqual(Array.from(Buffer.from(row.key)), Array.from(Buffer.from(row.name))) !== row.equal) throw new Error(row.id);
  if (row.maximumBytesPerTurn !== 1 || row.deniedWork !== 0) throw new Error(row.id);
}
console.log('matcher fixture oracle passed ' + fixture.cases.length + ' cases');
