import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import {
  SEQUENCE_MAX_PAGE_BYTES,
  SEQUENCE_MAX_REQUEST_BYTES,
  SEQUENCE_MAX_TRANSFER_BYTES,
  SequenceOperation,
} from "../🟨️sequence-host.js";

//#region 🧬️SchemaLaws

const schemaUrl = new URL("../../../🧬️schema/🔣️component.json", import.meta.url);
const schema = JSON.parse(readFileSync(fileURLToPath(schemaUrl), "utf8"));
const operationCodes = Object.values(schema.operations);
const eventCodes = Object.values(schema.events);
const featureOperations = Object.values(schema.features).flat();

if (schema.version !== 1) throw new Error("Sequence schema instance version drift");
if (schema.properties.version.const !== 1) throw new Error("Sequence ABI schema version drift");
if (JSON.stringify(schema.operations) !== JSON.stringify(SequenceOperation)) throw new Error("Sequence JS operation ledger drift");
if (operationCodes.length !== 47 || new Set(operationCodes).size !== 47) throw new Error("Sequence operation ledger must be unique and complete");
if (Math.min(...operationCodes) !== 2300 || Math.max(...operationCodes) !== 2346) throw new Error("Sequence operation range drift");
if (Object.keys(schema.features).length !== 10 || Math.max(...Object.values(schema.features).map((operations) => operations.length)) > 10) throw new Error("Sequence feature taxonomy must remain small");
if (featureOperations.length !== 47 || new Set(featureOperations).size !== 47) throw new Error("Sequence feature taxonomy must own every operation exactly once");
if (JSON.stringify([...featureOperations].sort()) !== JSON.stringify(Object.keys(schema.operations).sort())) throw new Error("Sequence feature taxonomy drift");
if (eventCodes.length !== 8 || new Set(eventCodes).size !== 8) throw new Error("Sequence event ledger must be unique and complete");
if (Math.min(...eventCodes) !== 2400 || Math.max(...eventCodes) !== 2407) throw new Error("Sequence event range drift");
if (schema.limits.requestBytes !== SEQUENCE_MAX_REQUEST_BYTES) throw new Error("Sequence request bound drift");
if (schema.limits.pageBytes !== SEQUENCE_MAX_PAGE_BYTES) throw new Error("Sequence page bound drift");
if (schema.limits.transferBytes !== SEQUENCE_MAX_TRANSFER_BYTES) throw new Error("Sequence transfer bound drift");
if (schema.identities.surface !== "nonzero-u32" || schema.identities.canvas !== "nonzero-u32") throw new Error("Sequence browser identity drift");
if (schema.framing.byteOrder !== "little-endian" || schema.framing.message !== "A1-AbiMessage-v1") throw new Error("Sequence framing drift");

console.log(JSON.stringify({ schema: "valid", features: 10, operations: 47, events: 8, framing: "A1-AbiMessage-v1" }));

//#endregion 🧬️SchemaLaws
