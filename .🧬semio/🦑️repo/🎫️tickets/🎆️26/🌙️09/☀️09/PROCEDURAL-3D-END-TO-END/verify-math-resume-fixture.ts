/** 🔎️ Compares the native law's fixture to the captured wire with the renderer's independent Pack implementation. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { decodePackValue, encodePackValue, isPackMap, packUInt, packUIntSafeOrNull } from "@semio-tech/framework/packs";

const [fixturePath, capturedAnswerPath, capturedRequestPath] = process.argv.slice(2);
const fixtureText = await readFile(fixturePath, "utf8");
const fixture = JSON.parse(fixtureText);
const capturedAnswer = await readFile(capturedAnswerPath);
const capturedRequest = await readFile(capturedRequestPath, "utf8");
assert.deepEqual(Buffer.from(fixture.wireHex, "hex"), capturedAnswer);
assert.equal(createHash("sha256").update(capturedAnswer).digest("hex"), "b1ddfe8470f5147ecf7c3a919efe7f4a09d87038e6df60c583ca6082cf22b396");
const losslessRequest = (json: string) => JSON.parse(json.replace(/"nodeHash":\s*(\d+)/, '"nodeHash":"$1"'));
assert.deepEqual(losslessRequest(fixtureText).request, losslessRequest(capturedRequest));
const { unitsDone, unitsTotal } = fixture.reply;
assert.deepEqual(Buffer.from(encodePackValue({ ...fixture.reply, unitsDone: packUInt(BigInt(unitsDone)), unitsTotal: packUInt(BigInt(unitsTotal)) })), capturedAnswer);
const decoded = decodePackValue(capturedAnswer);
assert(isPackMap(decoded));
assert.deepEqual({ ...decoded, unitsDone: packUIntSafeOrNull(decoded.unitsDone), unitsTotal: packUIntSafeOrNull(decoded.unitsTotal) }, fixture.reply);
console.log("Captured Math request, exact Pack answer, and independent renderer encoding agree");
