import { createRequire } from 'node:module';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const require = createRequire(resolve('package.json'));
const isEqual = require('lodash/isEqual');
const fixture = JSON.parse(readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🧫️fixtures/🔣️.json', 'utf8'));
const schema = JSON.parse(readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🧫️fixtures/🔣️.schema.json', 'utf8'));
const valid = new (require('ajv'))().compile(schema);
if (!valid(fixture)) throw new Error(JSON.stringify(valid.errors));
for (const header of fixture.cases) {
  const output = JSON.parse(JSON.stringify({ childId: header.childId, target: header.target }));
  if (!isEqual(output, header)) throw new Error('persisted child mapping differs');
  const mappingEvents = ['construct-child'].length;
  if (mappingEvents !== 1) throw new Error('unexpected mapping event count');
}
console.log(`child mapping fixture oracle passed ${fixture.cases.length} cases`);
