import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const { Reader, Writer } = require('protobufjs');
const Ajv = require('ajv');
const cases = JSON.parse(readFileSync(new URL('./layout-native-binding-cases.json', import.meta.url), 'utf8'));
const dictionary = JSON.parse(readFileSync('🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🧪️tests/🧫️fixtures/🔣️.json', 'utf8'));
const protocol = readFileSync('✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio', 'utf8');
const tags = new Map(Array.from(protocol.matchAll(/^record ([\w.-]+) tag=(\d+)\r?$/gm), ([, name, tag]) => [name, Number(tag)]));
for (const c of Object.values(cases)) {
  const keyword = c.kind.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase();
  assert.equal(tags.get(keyword), c.tag);
}

const c = cases.largeMutation;
const text = c.text.repeat(c.repeat);
assert(Buffer.byteLength(text, 'utf8') > c.maximumCopyBytes);
const bytes = Writer.create().uint32(1).uint32(c.tag)
  .uint32(0).uint32(1).uint32(1).uint32(0x11)
  .uint32(0x10).uint32(1).uint32(0x07).string(c.payloadKey)
  .uint32(0x07).string(text).finish();
const reader = Reader.create(bytes);
assert.deepEqual(Array.from(bytes.subarray(0, 9)), [1, c.tag, 0, 1, 1, 0x11, 0x10, 1, 0x07]);
reader.pos = 9;
assert.equal(reader.string(), c.payloadKey);
assert.equal(reader.uint32(), 0x07);
assert.equal(reader.string(), text);
assert.equal(reader.pos, reader.len);

const string = { type: 'string' };
const literal = { $ref: '#/definitions/literal' };
const list = items => ({ type: 'array', items });
const record = properties => ({
  type: 'object', properties, required: Object.keys(properties), additionalProperties: false,
});
const variant = (kind, fields = {}) => record({ kind: { const: kind }, ...fields });
const limb = { type: 'integer', minimum: 0, maximum: 0xffffffff };
const word = { high: limb, low: limb };
const schema = {
  ...record({ entries: list(record({ questionId: string, value: literal })) }),
  definitions: {
    literal: {
      oneOf: [
        variant('null'),
        variant('boolean', { value: { type: 'boolean' } }),
        variant('unsigned', word),
        variant('signed', word),
        variant('float', word),
        variant('text', { value: string }),
        variant('bytes', { value: list({ type: 'integer', minimum: 0, maximum: 255 }) }),
        variant('array', { items: list(literal) }),
        variant('object', { members: list(record({ name: string, value: literal })) }),
      ],
    },
  },
};
const validate = new Ajv().compile(schema);
const unique = entries => new Set(entries.map(entry => entry.questionId)).size === entries.length;
for (const row of dictionary.valid) assert(validate(row) && unique(row.entries));
for (const row of dictionary.structurallyInvalid) assert(!validate(row));
for (const row of dictionary.semanticallyInvalid) assert(validate(row) && !unique(row.entries));
console.log(`Validated the ${bytes.length}-byte operation and ${dictionary.valid.length + dictionary.structurallyInvalid.length + dictionary.semanticallyInvalid.length} dictionary cases.`);
