import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';

const { Reader, Writer } = createRequire(import.meta.url)('protobufjs');
const testRoot = '🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🧫️fixtures';
const fixture = JSON.parse(readFileSync(`${testRoot}/🏷️tagged.json`, 'utf8'));
const protocol = readFileSync(`${testRoot}/🏷️protocol.semio`, 'utf8');
const layout = readFileSync('✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio', 'utf8');

function records(source) {
  return Array.from(source.matchAll(/^record ([\w.-]+) tag=(\d+)\r?$/gm), ([, name, tag]) => [name, Number(tag)]);
}

const fixtureTags = new Map(records(protocol));
assert.equal(fixtureTags.get('rename'), fixture.tag);
assert.equal(fixtureTags.get('remove'), fixture.firstTag);
function encodeTaggedText(tag, text) {
  return Writer.create()
    .uint32(1).uint32(tag)
    .uint32(1).string(text)
    .uint32(1).uint32(1).uint32(0x11)
    .uint32(0x10).uint32(1).uint32(0x07).string('text').uint32(0x06).uint32(0)
    .finish();
}

const wire = encodeTaggedText(fixture.tag, fixture.text);
assert.deepEqual(Array.from(wire), fixture.packWire);
const reader = Reader.create(wire);
assert.equal(reader.uint32(), 1);
const selectedTag = reader.uint32();
assert.equal(selectedTag, fixture.tag);
const selected = Array.from(fixtureTags).find(([, tag]) => tag === selectedTag)?.[0];
assert.equal(selected[0].toUpperCase() + selected.slice(1), fixture.selectedVariant);
assert.notEqual(fixtureTags.get('remove'), selectedTag);
assert.equal(reader.uint32(), 1);
assert.equal(reader.string(), fixture.text);
const payloadTail = [1, 1, 0x11, 0x10, 1, 0x07, 4, 116, 101, 120, 116, 0x06, 0];
assert.deepEqual(Array.from(wire.subarray(reader.pos)), payloadTail);

const longText = fixture.text.repeat(fixture.large.repetitions);
const longWire = Writer.create()
  .uint32(1).uint32(fixture.tag).uint32(0)
  .uint32(1).uint32(1).uint32(0x11)
  .uint32(0x10).uint32(1).uint32(0x07).string('text').uint32(0x07).string(longText)
  .finish();
assert.equal(longWire.length, fixture.large.wireBytes);
assert.deepEqual(Array.from(longWire.subarray(0, fixture.large.inlinePrefix.length)), fixture.large.inlinePrefix);
assert(longWire.length > fixture.large.refusedCopyBytes);
assert(fixture.large.refusedCopyBytes > fixture.large.bodyBytes);
const longReader = Reader.create(longWire);
assert.equal(longReader.uint32(), 1);
assert.equal(longReader.uint32(), fixture.tag);
assert.equal(longReader.uint32(), 0);
longReader.skip(fixture.large.inlinePrefix.length - longReader.pos);
assert.equal(longReader.string(), longText);
assert.equal(longReader.pos, longWire.length);

const layoutRecords = records(layout);
assert.equal(layoutRecords.length, 58);
assert.equal(new Set(layoutRecords.map(([, tag]) => tag)).size, layoutRecords.length);
assert.equal(new Map(layoutRecords).get('set-page-background-drawing'), 59);
assert.equal(layoutRecords.some(([, tag]) => tag === 44), false);
console.log(`Validated both tagged Pack fixtures (${wire.length} and ${longWire.length} bytes) and all 58 Layout protocol tags.`);
