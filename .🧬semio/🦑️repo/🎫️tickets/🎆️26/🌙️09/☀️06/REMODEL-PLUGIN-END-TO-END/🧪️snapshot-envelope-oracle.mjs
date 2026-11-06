import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { SmartBuffer } from "smart-buffer";
import { NativeDecodeControl } from "@semio-tech/value-ts/decode";

const { unwrapBinaryControlled } = await import(pathToFileURL(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🟦️.ts")).href);
const generic = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🧫️fixtures/🚦️controlled/🔣️.json", "utf8"));
const snapshot = JSON.parse(readFileSync("✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/📸️snapshot-export/🔣️.json", "utf8"));
const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 13, 10, 26, 10]);
const vectors = [{ id: generic.id, version: generic.version, bodyBytes: generic.bodyBytes }, { id: snapshot.envelopeId, version: 1, bodyBytes: 127 }];
let cases = 0, valid = 0, refused = 0;
for (const vector of vectors) {
  const token = `${vector.id}.pack v${vector.version}`;
  const writer = new SmartBuffer();
  writer.writeBuffer(magic).writeUInt32LE(Buffer.byteLength(token)).writeString(token, "utf8").writeBuffer(Buffer.alloc(vector.bodyBytes, 127));
  const encoded = writer.toBuffer();
  for (const fault of ["none", "prefix", "short-prefix", "token-length", "identity", "component", "version"]) {
    let bytes = Buffer.from(encoded), id = vector.id, component = "pack", version = vector.version;
    if (fault === "prefix") bytes[0] = 0;
    if (fault === "short-prefix") bytes = bytes.subarray(0, 11);
    if (fault === "token-length") bytes.writeUInt32LE(0xffffffff, 8);
    if (fault === "identity") id = "foreign.owner";
    if (fault === "component") component = "spr";
    if (fault === "version") version += 1;
    let reference, actual;
    try {
      const reader = SmartBuffer.fromBuffer(bytes);
      assert.deepEqual(reader.readBuffer(8), magic);
      const length = reader.readUInt32LE();
      assert.equal(reader.readString(length, "utf8"), `${id}.${component} v${version}`);
      reference = { offset: reader.readOffset, body: reader.readBuffer() };
    } catch {}
    const control = new NativeDecodeControl(0, () => true);
    try { actual = await unwrapBinaryControlled(bytes, id, component, version, control); } catch {}
    assert.equal(actual !== undefined, reference !== undefined, `${vector.id}:${fault}`);
    assert.equal(actual !== undefined, fault === "none", `declared acceptance:${vector.id}:${fault}`);
    assert.equal(control.ownedBytes, 0);
    cases += 1;
    if (actual !== undefined) {
      valid += 1;
      assert.equal(actual.buffer, bytes.buffer);
      assert.equal(actual.byteOffset, bytes.byteOffset + reference.offset);
      assert.deepEqual(Buffer.from(actual), reference.body);
      assert.equal(actual.length, vector.bodyBytes);
    } else refused += 1;
  }
}
assert.deepEqual({ cases, valid, refused }, { cases: 14, valid: 2, refused: 12 });
console.log(JSON.stringify({ cases, valid, refused, thirdParty: "smart-buffer", firstParty: "unwrapBinaryControlled", ownedPayloadBytes: 0, innerPackValidated: false }));
