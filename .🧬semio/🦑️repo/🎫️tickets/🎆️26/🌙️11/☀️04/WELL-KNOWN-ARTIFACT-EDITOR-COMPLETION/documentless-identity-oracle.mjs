import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import protobuf from "protobufjs/minimal.js";
const channel = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/";
const law = JSON.parse(readFileSync(channel + "🧪️tests/🧫️fixtures/🪪️document-identity-wire/🔣️.json", "utf8"));
const schema = JSON.parse(readFileSync(channel + "🧬️schema/🪪️document-identity/🔣️.json", "utf8"));
const validate = new Ajv().compile(schema);
for (const row of law.cases) {
    assert.equal(validate(row.identity), true);
    const command = protobuf.Writer.create().uint32(law.commandTag).uint64(row.seq).finish();
    assert.deepEqual(Array.from(command), row.commandBytes);
    const value = row.identity;
    const frame = protobuf.Writer.create().uint32(law.frameTag).uint64(row.seq).uint32(value.appInstanceId).bool(value.parentDocumentId !== null);
    if (value.parentDocumentId !== null) frame.string(value.parentDocumentId);
    frame.uint64(value.baseRevision);
    assert.deepEqual(Array.from(frame.finish()), row.frameBytes);
}
console.log("[DEBUG] independent protobufjs + AJV reproduce all3 identity command/response vectors, including null identity revision0");
