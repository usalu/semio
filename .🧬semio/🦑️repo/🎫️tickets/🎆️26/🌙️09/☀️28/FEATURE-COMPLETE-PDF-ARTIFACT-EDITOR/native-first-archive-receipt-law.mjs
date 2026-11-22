import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

const schema = JSON.parse(readFileSync(new URL("native-first-archive-receipt.schema.json", import.meta.url)));
const validate = new Ajv2020().compile(schema);
const seq = 0x0020000000000005n;
const scope = {
  seq: String(seq),
  loan: {
    operationId: "0020000000000002", generation: "0020000000000003", instanceId: 42, extent: 9437877,
    declaration: {formatKind: "s.stdio.pdf@1.7", extension: ".pdf", mediaType: "application/pdf", binary: true},
  },
  document: {appInstanceId: 42, baseRevision: String(0x0505050505050505n), parentDocumentId: "pdf-document"},
  storeGeneration: "7",
  storeRevision: Array(32).fill(5),
};
const offer = {scope, extent: 2130150};
const page = {offer, offset: 0, length: 4096};
const seal = {offer, sha256: Array(32).fill(0)};
for (const valid of [offer, page, seal]) assert(validate(valid), JSON.stringify(validate.errors));
for (const invalid of [
  {...offer, extent: 4194305},
  {...offer, scope: {...scope, seq: 9007199254740997}},
  {...offer, scope: {...scope, seq: "09007199254740997"}},
  {...page, offset: 1},
  {...page, length: 4097},
  {...page, destination: "foreign"},
  {...seal, sha256: Array(31).fill(0)},
]) assert(!validate(invalid));
assert.equal(BigInt(JSON.parse(JSON.stringify(offer)).scope.seq), seq);
console.log("Ajv checked archive offer, page and seal shapes and preserved the full sequence");
