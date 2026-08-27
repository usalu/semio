import { createSequenceBrowserFeatures } from "../📦️index.ts";

//#region 🌐️ProductionConsumerLaw

const memory = new WebAssembly.Memory({ initial: 2 });
const source = Uint8Array.of(0, 97, 115, 109);
const imports = { sequence: {} };
const bridge = new EntryBridge(memory);
let instantiated = 0;

const features = await createSequenceBrowserFeatures({
  source,
  imports,
  instantiate: async (actualSource, actualImports) => {
    if (actualSource !== source || actualImports !== imports) throw new Error("Sequence production entry changed its Wasm inputs");
    instantiated += 1;
    return { instance: { exports: { ...bridge.exports, memory } } };
  },
});

if (instantiated !== 1) throw new Error("Sequence production entry must instantiate exactly once");
if (features.lifetime.session.slot !== 1 || features.lifetime.session.generation !== 1) throw new Error("Sequence production entry did not create an owned session");
if (!features.document || !features.editing || !features.execution || !features.viewport || !features.input || !features.layout || !features.selection || !features.preview || !features.playback) throw new Error("Sequence production entry did not expose reactive features");
await features.lifetime.close();
if (!bridge.closed || !bridge.terminalEmpty) throw new Error("Sequence production entry did not close cleanly");

console.log(JSON.stringify({ publicEntry: "@semio-tech/sequence-js", instantiate: "exact", session: "owned", terminal: "empty" }));

//#endregion 🌐️ProductionConsumerLaw

//#region 🧪️Mock

function EntryBridge(targetMemory) {
  let cursor = 8;
  let outbound;
  this.closed = false;
  this.terminalEmpty = false;
  this.exports = {
    sequence_bridge_create() { return 1; },
    sequence_bridge_destroy() { return 1; },
    sequence_bridge_allocate(length) {
      const pointer = cursor;
      cursor += Math.max(1, length);
      return pointer;
    },
    sequence_bridge_release() {},
    sequence_bridge_send: (_owner, pointer, length) => {
      const request = new DataView(targetMemory.buffer, pointer, length);
      if (request.getUint8(0) !== 1) return -1;
      if (request.getUint8(1) === 5 && request.getUint8(2) === 2) return 1;
      if (request.getUint8(1) !== 1 || request.getUint16(2, true) !== 2300) return -1;
      outbound = reply(request.getBigUint64(4, true), request.getUint32(12, true));
      return 1;
    },
    sequence_bridge_poll: (_owner, pointer, capacity) => {
      if (!outbound) return this.closed ? -1 : 0;
      if (outbound.length > capacity) return outbound.length;
      new Uint8Array(targetMemory.buffer, pointer, outbound.length).set(outbound);
      const length = outbound.length;
      outbound = undefined;
      return length;
    },
    sequence_bridge_begin_close: () => {
      this.closed = true;
      this.terminalEmpty = true;
    },
    sequence_bridge_terminal_is_empty: () => Number(this.terminalEmpty),
  };
}

function reply(requestId, generation) {
  const bytes = new Uint8Array(34);
  const view = new DataView(bytes.buffer);
  view.setUint8(0, 1);
  view.setUint8(1, 2);
  view.setBigUint64(2, requestId, true);
  view.setUint32(10, generation, true);
  view.setUint16(14, 0, true);
  view.setUint8(16, 0);
  view.setUint32(17, 8, true);
  view.setUint32(21, 1, true);
  view.setUint32(25, 1, true);
  return bytes.subarray(0, 29);
}

//#endregion 🧪️Mock
