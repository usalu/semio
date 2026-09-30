/** 🧪️ Neutral IO descriptors agree with owned parsers and the independent Ajv schema oracle. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import schema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import { parseIoEntryDescriptor, parseIoRoute } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import fixture from "../../🧫️fixtures/📇️descriptor-parity.json" with { type: "json" };
import { ioEntries } from "../../🟦️.ts";

export function testArtifactIoDescriptorParity(): number {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-ui", valid: true });
  ajv.addSchema(schema);
  const descriptor = ajv.compile({ $ref: `${schema.$id}#/$defs/IoEntryDescriptor` });
  const route = ajv.compile({ $ref: `${schema.$id}#/$defs/IoRoute` });
  let checks = 0;
  assert.deepEqual(ioEntries, fixture.entries);
  checks++;
  for (const entry of fixture.entries) {
    assert.equal(descriptor(entry), true, JSON.stringify(descriptor.errors));
    assert.deepEqual(parseIoEntryDescriptor(entry), entry);
    checks += 2;
  }
  for (const entry of fixture.invalidEntries) {
    assert.equal(descriptor(entry), false, JSON.stringify(entry));
    assert.throws(() => parseIoEntryDescriptor(entry));
    checks += 2;
  }
  for (const value of fixture.routes) {
    assert.equal(route(value), true, JSON.stringify(route.errors));
    assert.deepEqual(parseIoRoute(value), value);
    checks += 2;
  }
  for (const value of fixture.invalidRoutes) {
    assert.equal(route(value), false, JSON.stringify(value));
    assert.throws(() => parseIoRoute(value));
    checks += 2;
  }
  for (const mismatch of fixture.mismatches) {
    const stale = { ...ioEntries[mismatch.index], [mismatch.field]: mismatch.value };
    assert.equal(descriptor(stale), true);
    assert.notDeepEqual(stale, fixture.entries[mismatch.index]);
    checks += 2;
  }
  console.log(`Artifact IO descriptor parity checks=${checks}, entries=${ioEntries.length}`);
  return checks;
}
