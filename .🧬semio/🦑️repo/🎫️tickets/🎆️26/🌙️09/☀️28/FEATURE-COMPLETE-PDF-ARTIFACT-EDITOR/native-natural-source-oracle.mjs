import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const load = (name) => JSON.parse(readFileSync(resolve(ticket, name)));
const schema = load("native-natural-source.schema.json");
const fixture = load("native-natural-source.json");
const source = load("🔣️segmented-pdf-source.json");
const validate = new Ajv2020().compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
assert.equal(Math.ceil(source.sourceBytes / fixture.pageBytes), fixture.pageCount);
assert.equal(source.sourceBytes - (fixture.pageCount - 1) * fixture.pageBytes, fixture.tailBytes);
for (const file of process.argv.slice(2)) {
  const hash = createHash("sha256");
  let length = 0;
  for await (const chunk of createReadStream(file, { highWaterMark: fixture.pageBytes })) {
    hash.update(chunk);
    length += chunk.length;
  }
  assert.equal(length, source.sourceBytes);
  assert.equal(hash.digest("hex"), source.sourceSha256);
}
process.stdout.write(`Ajv geometry and independent SHA-256 agree for ${source.sourceBytes} bytes\n`);
