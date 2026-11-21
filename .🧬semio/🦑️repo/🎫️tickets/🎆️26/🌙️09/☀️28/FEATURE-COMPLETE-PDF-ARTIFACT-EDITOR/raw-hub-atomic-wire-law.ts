/** 🧪️ Checks actual client/server frame code against independent wire fixtures. */
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { encodeClientAtomicDocumentPart, decodeServerAtomicDocumentPart } from "./🗑️generated/raw-hub-atomic/hub-wire.ts";

const fixture = new URL("./🗑️generated/raw-hub-atomic/", import.meta.url);
for (let ordinal = 0; ordinal < 8; ordinal++) {
  const part = readFileSync(new URL(String(ordinal), fixture)).subarray(10);
  const expectedClient = readFileSync(new URL(String(ordinal), fixture));
  assert.deepEqual(Buffer.from(encodeClientAtomicDocumentPart(82n, part)), expectedClient);
  const expectedServer = readFileSync(new URL("server-" + ordinal, fixture));
  const decoded = decodeServerAtomicDocumentPart(expectedServer);
  assert.deepEqual(Buffer.from(decoded?.part ?? []), part);
  assert.equal(decoded?.frontier.length, ordinal === 0 ? 8 : 0);
  expectedServer[2] ^= 1;
  assert.throws(() => decodeServerAtomicDocumentPart(expectedServer), /atomic Hub frame/);
}
assert.equal(decodeServerAtomicDocumentPart(Uint8Array.of(0, 2)), undefined);
console.log("[DEBUG] Actual host codec matches independent client and server frames");
