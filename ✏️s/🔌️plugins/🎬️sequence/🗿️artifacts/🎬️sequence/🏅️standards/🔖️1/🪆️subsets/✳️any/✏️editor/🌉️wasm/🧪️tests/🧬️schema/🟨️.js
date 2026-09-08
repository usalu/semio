import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import {
  SEQUENCE_MAX_PAGE_BYTES,
  SEQUENCE_MAX_REQUEST_BYTES,
  SEQUENCE_MAX_TRANSFER_BYTES,
  SequenceOperation,
} from "../../📦️packages/🟨️javascript/🖥️sequence-host.js";

//#region 🧬️SchemaLaws

/** 🥄️ Reads one `enum <name> { … }` body out of the interface facet without a WIT parser. */
const witEnum = (text, name) => {
  const body = new RegExp(`enum ${name} \\{([^}]*)\\}`, "u").exec(text)?.[1];
  if (body === undefined) throw new Error(`Sequence interface facet declares no enum ${name}`);
  return body.split("\n").map((line) => line.replace(/\/\/.*$/u, "").trim().replace(/,$/u, "")).filter(Boolean);
};

const kebab = (name) => name.replace(/(?<!^)(?=[A-Z])/gu, "-").toLowerCase();

const wit = readFileSync(fileURLToPath(new URL("../../🧬️schema/📜️.wit", import.meta.url)), "utf8");
const operations = witEnum(wit, "operation");
const events = witEnum(wit, "event");
const features = witEnum(wit, "feature");
const operationCodes = Object.values(SequenceOperation);

if (!wit.startsWith("package semio:sequence-browser-abi@1.0.0;")) throw new Error("Sequence interface facet package identity drift");
if (!/^world sequence-browser \{$/mu.test(wit) || !/^\s+export abi;$/mu.test(wit)) throw new Error("Sequence interface facet world drift");
if (operations.length !== 47 || new Set(operations).size !== 47) throw new Error("Sequence operation surface must be unique and complete");
if (JSON.stringify(operations) !== JSON.stringify(Object.keys(SequenceOperation).map(kebab))) throw new Error("Sequence JS operation ledger drift");
if (operationCodes.length !== 47 || new Set(operationCodes).size !== 47) throw new Error("Sequence operation ledger must be unique and complete");
if (Math.min(...operationCodes) !== 2300 || Math.max(...operationCodes) !== 2346) throw new Error("Sequence operation range drift");
if (features.length !== 10 || new Set(features).size !== 10) throw new Error("Sequence feature taxonomy must remain small");
const grouped = [...wit.matchAll(/^\s+\/\/ ([a-z-]+)\n((?:\s+[a-z0-9-]+,\n)+)/gmu)].map(([, feature, block]) => [feature, block.trim().split("\n").map((line) => line.trim().replace(/,$/u, ""))]);
if (JSON.stringify(grouped.map(([feature]) => feature)) !== JSON.stringify(features)) throw new Error("Sequence feature taxonomy drift");
if (grouped.flatMap(([, members]) => members).join(",") !== operations.join(",")) throw new Error("Sequence feature taxonomy must own every operation exactly once");
if (Math.max(...grouped.map(([, members]) => members.length)) > 10) throw new Error("Sequence feature groups must remain small");
if (events.length !== 8 || new Set(events).size !== 8) throw new Error("Sequence event surface must be unique and complete");
if (!/record limits \{[^}]*request-bytes: u32,[^}]*page-bytes: u32,[^}]*transfer-bytes: u32,[^}]*\}/su.test(wit)) throw new Error("Sequence limits record drift");
if (SEQUENCE_MAX_REQUEST_BYTES !== 1_048_576 || SEQUENCE_MAX_PAGE_BYTES !== 65_536 || SEQUENCE_MAX_TRANSFER_BYTES !== 16_777_216) throw new Error("Sequence host bound drift");
if (!/record identity \{\s+slot: u32,\s+generation: u32,\s+\}/su.test(wit)) throw new Error("Sequence identity record drift");
if (!/type surface-handle = u32;/u.test(wit) || !/type canvas-handle = u32;/u.test(wit)) throw new Error("Sequence browser handle drift");
if (!wit.includes("A1-AbiMessage-v1")) throw new Error("Sequence framing drift");

console.log(JSON.stringify({ facet: "📜️interface", features: features.length, operations: operations.length, events: events.length, framing: "A1-AbiMessage-v1" }));

//#endregion 🧬️SchemaLaws
