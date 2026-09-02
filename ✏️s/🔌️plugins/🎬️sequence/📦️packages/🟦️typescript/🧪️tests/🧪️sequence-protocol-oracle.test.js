import Ajv2020 from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createSequenceFeatures, createSequenceHost } from "../../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🟨️sequence-host.js";

//#region 🔮️ThirdPartyOracle

const root = new URL("../../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/", import.meta.url);
const schema = JSON.parse(readFileSync(fileURLToPath(new URL("🧬️schema/🔣️.json", root)), "utf8"));
const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("🧪️fixtures/🔣️.json", root)), "utf8"));
const oracle = createOraclePort(new Ajv2020({ strict: true }).compile(schema));
const expected = oracle.emit(fixture);

//#endregion 🔮️ThirdPartyOracle

//#region 🧩️OwnedFeature

const memory = new WebAssembly.Memory({ initial: 2 });
const bridge = new CaptureBridge(memory);
const features = await createSequenceFeatures(createSequenceHost({ exports: bridge.exports, memory }));
const task = features.editing.addStepDropped(fixture.input.kind, fixture.input.x, fixture.input.y, fixture.input.picked);
const semantic = await task.result;

if (bridge.frameHex !== expected.frameHex) throw new Error("Sequence owned feature and third-party oracle protocol frames differ");
if (semantic !== expected.semantic) throw new Error("Sequence owned feature and third-party oracle semantic results differ");
await features.lifetime.close();

console.log(JSON.stringify({ oracle: "ajv-test-only", interface: "owned", feature: expected.feature, protocol: "equal", semantic: "equal" }));

//#endregion 🧩️OwnedFeature

//#region 🧪️OraclePort

function createOraclePort(validate) {
  return {
    emit(value) {
      if (!validate(value)) throw new Error("Sequence oracle fixture rejected");
      return { feature: value.feature, frameHex: encodeOracleRequest(value), semantic: value.semantic };
    },
  };
}

function encodeOracleRequest(value) {
  const text = new TextEncoder().encode(value.input.kind);
  const payloadLength = 4 + text.length + 8 + 8 + 1;
  const bodyLength = 8 + payloadLength;
  const bytes = new Uint8Array(20 + bodyLength);
  const view = new DataView(bytes.buffer);
  view.setUint8(0, 1);
  view.setUint8(1, 1);
  view.setUint16(2, value.operation, true);
  view.setBigUint64(4, BigInt(value.requestId), true);
  view.setUint32(12, value.generation, true);
  view.setUint32(16, bodyLength, true);
  view.setUint32(20, value.session.slot, true);
  view.setUint32(24, value.session.generation, true);
  view.setUint32(28, text.length, true);
  bytes.set(text, 32);
  let cursor = 32 + text.length;
  view.setFloat64(cursor, value.input.x, true);
  cursor += 8;
  view.setFloat64(cursor, value.input.y, true);
  cursor += 8;
  view.setUint8(cursor, 0);
  return hex(bytes);
}

//#endregion 🧪️OraclePort

//#region 🌉️CaptureBridge

function CaptureBridge(targetMemory) {
  let cursor = 8;
  let outbound;
  let terminal = false;
  this.frameHex = undefined;
  this.exports = {
    sequence_bridge_create() { return 1; },
    sequence_bridge_destroy() { return 1; },
    sequence_bridge_allocate(length) { const pointer = cursor; cursor += Math.max(1, length); return pointer; },
    sequence_bridge_release() {},
    sequence_bridge_send: (_owner, pointer, length) => {
      const bytes = new Uint8Array(targetMemory.buffer, pointer, length).slice();
      const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
      if (view.getUint8(0) !== 1) return -1;
      if (view.getUint8(1) === 5) return 1;
      if (view.getUint8(1) !== 1) return -1;
      const operation = view.getUint16(2, true);
      const requestId = view.getBigUint64(4, true);
      const generation = view.getUint32(12, true);
      if (operation === 2300) outbound = reply(requestId, generation, handle(1, 1));
      else if (operation === 2305) {
        this.frameHex = hex(bytes);
        outbound = reply(requestId, generation, new TextEncoder().encode("step-101"));
      } else return -1;
      return 1;
    },
    sequence_bridge_poll(_owner, pointer, capacity) {
      if (!outbound) return terminal ? -1 : 0;
      if (outbound.length > capacity) return outbound.length;
      new Uint8Array(targetMemory.buffer, pointer, outbound.length).set(outbound);
      const length = outbound.length;
      outbound = undefined;
      return length;
    },
    sequence_bridge_begin_close() { terminal = true; },
    sequence_bridge_terminal_is_empty() { return Number(terminal); },
  };
}

function reply(requestId, generation, body) {
  const bytes = new Uint8Array(21 + body.length);
  const view = new DataView(bytes.buffer);
  view.setUint8(0, 1);
  view.setUint8(1, 2);
  view.setBigUint64(2, requestId, true);
  view.setUint32(10, generation, true);
  view.setUint16(14, 0, true);
  view.setUint8(16, 0);
  view.setUint32(17, body.length, true);
  bytes.set(body, 21);
  return bytes;
}

function handle(slot, generation) {
  const bytes = new Uint8Array(8);
  const view = new DataView(bytes.buffer);
  view.setUint32(0, slot, true);
  view.setUint32(4, generation, true);
  return bytes;
}

function hex(bytes) { return [...bytes].map((value) => value.toString(16).padStart(2, "0")).join(""); }

//#endregion 🌉️CaptureBridge
