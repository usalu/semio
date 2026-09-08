import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const require = createRequire(import.meta.url), here = dirname(fileURLToPath(import.meta.url));
const cases = JSON.parse(readFileSync(join(here, "🧫️cases.json"), "utf8"));
const runtime = resolve(process.argv[2] ?? "node_modules"), runtimeRequire = createRequire(join(runtime, "nx/package.json")), api = runtimeRequire("nx/src/utils/consume-messages-from-socket.js");
const version = require(join(runtime, "nx/package.json")).version;
const schema = JSON.parse(readFileSync(join(here, "🧬️schema.json"), "utf8"));
assert.equal(require("jsonschema").validate(cases, schema).valid, true);
for (const size of cases.chunkSizes) {
  const frames: Buffer[] = [];
  for (const message of cases.messages) {
    const payload = Buffer.from(JSON.stringify(message));
    if (api.writeMessage) api.writeMessage({ write: (chunk: Buffer) => frames.push(chunk) }, payload);
    else frames.push(payload, Buffer.from([4]));
  }
  const received: unknown[] = [], consume = api.consumeMessagesFromSocket((message: Buffer | string) => received.push(api.parseMessage ? api.parseMessage(message) : JSON.parse(String(message))));
  const bytes = Buffer.concat(frames);
  for (let offset = 0; offset < bytes.length; offset += size) consume(bytes.subarray(offset, offset + size));
  assert.deepEqual(received, cases.messages, `Nx ${version}, chunk size ${size}`);
}
console.log(`[DEBUG] Nx ${version}: Unicode paths, fragmented and coalesced socket frames PASS`);
