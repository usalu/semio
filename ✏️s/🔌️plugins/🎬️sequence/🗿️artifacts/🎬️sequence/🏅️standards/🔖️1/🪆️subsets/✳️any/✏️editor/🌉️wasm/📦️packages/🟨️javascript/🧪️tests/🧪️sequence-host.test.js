import {
  SEQUENCE_INITIAL_POLL_BYTES,
  SEQUENCE_MAX_ENCODED_MESSAGE_BYTES,
  SEQUENCE_MAX_IN_FLIGHT,
  SEQUENCE_MAX_PAGE_BYTES,
  SEQUENCE_MAX_REQUEST_BYTES,
  SequenceOperation,
  createSequenceFeatures,
  createSequenceHost,
  decodeMessage,
} from "../🖥️sequence-host.js";

const equal = (actual, expected, law) => {
  if (actual !== expected) throw new Error(`${law}: ${String(actual)} !== ${String(expected)}`);
};

const memory = new WebAssembly.Memory({ initial: 400 });
const bridge = new MockBridge(memory);
const canvas = new Canvas();
const renders = [];
const host = createSequenceHost({ exports: bridge.exports, memory, resolveCanvas: () => canvas, render: (_canvas, state) => renders.push(state) });
const features = await createSequenceFeatures(createSequenceHost({ exports: bridge.exports, memory, resolveCanvas: () => canvas, render: (_canvas, state) => renders.push(state) }));

equal(features.lifetime.session.slot, 1, "open-session");
equal(await result(features.editing.addStepDropped("math.add", 1, 2, undefined)), "step-101", "missing-optional-payload");
await result(features.document.loadFixtureJson('{"schema":"sequence.sequence","steps":[],"edges":[]}'));
equal(await result(features.document.fixtureJson()), '{"schema":"sequence.sequence","steps":[],"edges":[]}', "fixture-json-roundtrip");
let malformedJson = false;
try { await result(features.document.loadFixtureJson("[]")); } catch { malformedJson = true; }
equal(malformedJson, true, "malformed-json-owned-rejection");

let canvasMissing = false;
try {
  const missing = await createSequenceFeatures(createSequenceHost({ exports: bridge.exports, memory, resolveCanvas: () => undefined }));
  await result(missing.viewport.attach());
} catch { canvasMissing = true; }
equal(canvasMissing, true, "canvas-missing");

await result(features.viewport.attach(canvas));
await result(features.viewport.setSize(100, 50, 2));
equal(await result(features.viewport.gpuReady()), true, "surface-ready");
await result(features.viewport.renderFrame());
equal(renders.length, 1, "render-callback-once");
equal((await result(features.selection.selectedNodeIds())).length, 0, "selection-preserved");
await result(features.playback.play()); await result(features.playback.pause()); await result(features.playback.stop());

const largeTask = features.document.catalogueJson();
const progressEvents = [];
largeTask.subscribe((event) => progressEvents.push(event.event));
const large = await result(largeTask);
equal(large.length, SEQUENCE_MAX_PAGE_BYTES + 1, "zero-max-plus-one-pages");
equal(progressEvents.includes(2400) && progressEvents.includes(2406), true, "reactive-progress-events");
equal(bridge.shortPolls > 0, true, "undersized-poll-reported");
equal(bridge.exactRetries, bridge.shortPolls, "undersized-poll-exact-retry");
equal(bridge.pageAcknowledgements, 2, "page-ack-exact");

let callbackInterrupted = false;
const interruptedHost = createSequenceHost({
  exports: new MockBridge(memory).exports,
  memory,
  schedule: () => { callbackInterrupted = true; throw new Error("callback interrupted"); },
});
let interrupted = false;
try { await result(interruptedHost.start(SequenceOperation.open)); } catch { interrupted = true; }
equal(callbackInterrupted && interrupted, true, "callback-interruption");

const cappedBridge = new MockBridge(memory);
const capped = createSequenceHost({ exports: cappedBridge.exports, memory });
const admitted = Array.from({ length: SEQUENCE_MAX_IN_FLIGHT }, () => capped.start(SequenceOperation.open).result);
const cappedBefore = `${capped.state.nextRequest}:${credits(capped)}`;
const overflow = capped.start(SequenceOperation.open).result;
equal(`${capped.state.nextRequest}:${credits(capped)}`, cappedBefore, "in-flight-rejection-credit-stability");
let inFlightRejected = false;
try { await overflow; } catch { inFlightRejected = true; }
equal(inFlightRejected, true, "in-flight-max-plus-one");
await Promise.all(admitted);
await capped.close();

const stale = bridge.rejectHandle({ slot: 1, generation: 2 });
equal(stale, "stale", "stale-handle");
equal(bridge.rejectHandle({ slot: 1, generation: 0 }), "aba", "aba-handle");
equal(bridge.rejectHandle({ slot: 9, generation: 1 }), "lost", "lost-handle");
equal(bridge.rejectPage(SEQUENCE_MAX_PAGE_BYTES + 1), "pre-admission", "page-max-plus-one");
equal(bridge.rejectMessage(SEQUENCE_MAX_ENCODED_MESSAGE_BYTES + 1), "pre-admission", "message-max-plus-one");
let malformedPage = false;
try { decodeMessage(Uint8Array.of(1, 4, 1)); } catch { malformedPage = true; }
equal(malformedPage, true, "malformed-page");
equal(SEQUENCE_MAX_IN_FLIGHT, 256, "request-bound");

const cancelledBridge = new MockBridge(memory, { holdRun: true });
const cancelledHost = createSequenceHost({ exports: cancelledBridge.exports, memory });
const cancelledFeatures = await createSequenceFeatures(cancelledHost);
const runTask = cancelledFeatures.execution.run();
const run = runTask.result;
await Promise.resolve();
runTask.cancel();
cancelledBridge.cancelHeld();
let cancelled = false;
try { await run; } catch { cancelled = true; }
equal(cancelledBridge.cancelControls, 1, "cancel-control-exact");
equal(cancelled, true, "cancel-during-compute");

const hostileCancelBridge = new MockBridge(memory, { holdRun: true, rejectControls: SEQUENCE_MAX_IN_FLIGHT });
const hostileCancelHost = createSequenceHost({ exports: hostileCancelBridge.exports, memory });
const hostileCancelFeatures = await createSequenceFeatures(hostileCancelHost);
const heldRun = hostileCancelFeatures.execution.run();
const heldRejected = heldRun.result.then(() => false, () => true);
const hostileBefore = credits(hostileCancelHost);
for (let attempt = 0; attempt < SEQUENCE_MAX_IN_FLIGHT; attempt += 1) {
  let controlRejected = false;
  try { heldRun.cancel(); } catch { controlRejected = true; }
  equal(controlRejected, true, "hostile-cancel-control-rejected");
  equal(credits(hostileCancelHost), hostileBefore, "hostile-cancel-credit-stability");
}
equal(hostileCancelBridge.rejectedControls, SEQUENCE_MAX_IN_FLIGHT, "cancel-rejections-exact");
equal(new Set(hostileCancelBridge.rejectedControlFrames).size, 1, "cancel-rejection-frame-retained");
equal(heldRun.cancel(), true, "valid-cancel-after-hostile-rejections");
hostileCancelBridge.cancelHeld();
equal(await heldRejected, true, "valid-cancel-terminal-reply");
equal(credits(hostileCancelHost), "0:0:empty", "valid-cancel-credit-handback");
await hostileCancelHost.close();

const hostileAckBridge = new MockBridge(memory, { rejectControls: 6 });
const hostileAckHost = createSequenceHost({ exports: hostileAckBridge.exports, memory });
const hostileAckFeatures = await createSequenceFeatures(hostileAckHost);
equal((await result(hostileAckFeatures.document.catalogueJson())).length, SEQUENCE_MAX_PAGE_BYTES + 1, "ack-retry-output");
equal(hostileAckBridge.rejectedControls, 6, "ack-rejections-exact");
equal(new Set(hostileAckBridge.rejectedControlFrames).size, 1, "ack-rejection-frame-retained");
equal(hostileAckBridge.pageAcknowledgements, 2, "ack-retry-page-handback");
equal(credits(hostileAckHost), "0:0:empty", "ack-retry-credit-handback");
await hostileAckHost.close();

const hostileCloseBridge = new MockBridge(memory, { rejectCloses: 3 });
const hostileCloseHost = createSequenceHost({ exports: hostileCloseBridge.exports, memory });
const hostileCloseFeatures = await createSequenceFeatures(hostileCloseHost);
const hostileCloseBefore = `${credits(hostileCloseHost)}:${hostileCloseHost.state.closing}:${hostileCloseHost.state.closed}`;
for (let attempt = 0; attempt < 3; attempt += 1) {
  let rejected = false;
  try { await hostileCloseFeatures.lifetime.close(); } catch { rejected = true; }
  equal(rejected, true, "hostile-close-control-rejected");
  equal(`${credits(hostileCloseHost)}:${hostileCloseHost.state.closing}:${hostileCloseHost.state.closed}`, hostileCloseBefore, "hostile-close-credit-stability");
}
equal(new Set(hostileCloseBridge.rejectedCloseFrames).size, 1, "close-rejection-frame-retained");
await hostileCloseFeatures.lifetime.close();
equal(hostileCloseHost.terminalIsEmpty(), true, "valid-close-after-hostile-rejections");

const preflightBridge = new MockBridge(memory, { rejectInvalidHandle: true });
const preflightHost = createSequenceHost({ exports: preflightBridge.exports, memory });
const preflightFeatures = await createSequenceFeatures(preflightHost);
for (const invalid of [{ slot: 9, generation: 1 }, { slot: 1, generation: 0 }, { slot: 1, generation: 2 }]) {
  const before = `${preflightHost.state.nextRequest}:${credits(preflightHost)}`;
  let rejected = false;
  try { await result(preflightHost.start(SequenceOperation.run, new Uint8Array(), invalid)); } catch { rejected = true; }
  equal(rejected, true, "invalid-handle-rejected");
  equal(`${preflightHost.state.nextRequest}:${credits(preflightHost)}`, before, "invalid-handle-preflight-stability");
}
const oversizedBefore = `${preflightHost.state.nextRequest}:${credits(preflightHost)}`;
let oversizedRejected = false;
try { await result(preflightHost.start(SequenceOperation.run, new Uint8Array(SEQUENCE_MAX_REQUEST_BYTES + 1), preflightFeatures.lifetime.session)); } catch { oversizedRejected = true; }
equal(oversizedRejected, true, "request-max-plus-one-rejected");
equal(`${preflightHost.state.nextRequest}:${credits(preflightHost)}`, oversizedBefore, "request-max-plus-one-preflight-stability");
equal(preflightHost.cancel(999n), false, "unknown-control-rejected");
equal(`${preflightHost.state.nextRequest}:${credits(preflightHost)}`, oversizedBefore, "unknown-control-credit-stability");
await preflightHost.close();
const closedBefore = `${preflightHost.state.nextRequest}:${credits(preflightHost)}`;
let closedRejected = false;
try { await result(preflightHost.start(SequenceOperation.open)); } catch { closedRejected = true; }
equal(closedRejected, true, "closed-request-rejected");
equal(`${preflightHost.state.nextRequest}:${credits(preflightHost)}`, closedBefore, "closed-request-credit-stability");

await features.lifetime.close();
await host.close();
equal(bridge.closed, true, "close-called");
equal(bridge.terminalEmpty, true, "terminal-empty");
equal(bridge.outputDigest, (SEQUENCE_MAX_PAGE_BYTES + 1) * 120, "deterministic-output");

console.log(JSON.stringify({
  commands: "valid",
  optional: "missing",
  malformedJson: "owned",
  callback: "interrupted",
  cancel: "during-compute",
  hostileControls: "rejected-256-then-valid",
  handles: "lost-stale-aba",
  canvas: "missing",
  pages: "zero-max-plus-one",
  malformedPage: "rejected",
  bytes: "zero-max-plus-one",
  events: "bounded",
  inFlight: "max-plus-one",
  retainedRetry: "exact",
  acknowledgement: "exact",
  acknowledgementRetry: "rejected-then-valid",
  playback: "closed",
  output: "deterministic",
  terminal: "empty",
}));

class Canvas {
  constructor() { this.clientWidth = 100; this.clientHeight = 50; }
  getContext() { return undefined; }
}

function MockBridge(targetMemory, options = {}) {
  let cursor = 8;
  let fixture = '{"schema":"sequence.sequence","steps":[],"edges":[]}';
  let retained;
  let held;
  const queue = [];
  let rejectedControlsRemaining = options.rejectControls ?? 0;
  let rejectedClosesRemaining = options.rejectCloses ?? 0;
  this.shortPolls = 0;
  this.exactRetries = 0;
  this.pageAcknowledgements = 0;
  this.cancelControls = 0;
  this.rejectedControls = 0;
  this.rejectedControlFrames = [];
  this.rejectedCloseFrames = [];
  this.closed = false;
  this.terminalEmpty = false;
  this.outputDigest = 0;
  this.rejectHandle = (handle) => handle.slot !== 1 ? "lost" : handle.generation < 1 ? "aba" : handle.generation > 1 ? "stale" : "ok";
  this.rejectPage = (length) => length > SEQUENCE_MAX_PAGE_BYTES ? "pre-admission" : "ok";
  this.rejectMessage = (length) => length > SEQUENCE_MAX_ENCODED_MESSAGE_BYTES ? "pre-admission" : "ok";
  this.cancelHeld = () => { if (held) { queue.push(reply(held.id, held.generation, 2, new Uint8Array(), 12, "cancelled")); held = undefined; } };
  this.exports = {
    sequence_bridge_create() { return 1; },
    sequence_bridge_destroy() { return 1; },
    sequence_bridge_allocate(length) { const pointer = cursor; cursor += Math.max(length, 1); return pointer; },
    sequence_bridge_release() {},
    sequence_bridge_send: (_owner, pointer, length) => {
      const bytes = new Uint8Array(targetMemory.buffer, pointer, length).slice();
      const reader = new Reader(bytes);
      equal(reader.u8(), 1, "mock-version");
      const tag = reader.u8();
      if (tag === 5 && bytes[2] === 2 && rejectedClosesRemaining > 0) {
        rejectedClosesRemaining -= 1;
        this.rejectedCloseFrames.push(hex(bytes));
        return -1;
      }
      if ((tag === 2 || tag === 5) && rejectedControlsRemaining > 0) {
        rejectedControlsRemaining -= 1;
        this.rejectedControls += 1;
        this.rejectedControlFrames.push(hex(bytes));
        return -1;
      }
      if (tag === 1) {
        const operation = reader.u16(); const id = reader.u64(); const generation = reader.u32(); const body = reader.bytes();
        if (operation === SequenceOperation.open) queue.push(reply(id, generation, 0, handle(1, 1)));
        else {
          const session = new Reader(body); const slot = session.u32(); const handleGeneration = session.u32(); const payload = body.subarray(8);
          if ((slot !== 1 || handleGeneration !== 1) && options.rejectInvalidHandle) return -1;
          if (slot !== 1 || handleGeneration !== 1) queue.push(reply(id, generation, 3, new Uint8Array(), 7, "handle"));
          else if (operation === SequenceOperation.loadFixtureJson) {
            const text = new TextDecoder().decode(payload);
            if (!text.startsWith("{")) queue.push(reply(id, generation, 3, new Uint8Array(), 1, "json"));
            else { fixture = text; queue.push(reply(id, generation, 0, new Uint8Array())); }
          } else if (operation === SequenceOperation.fixtureJson) queue.push(reply(id, generation, 0, new TextEncoder().encode(fixture)));
          else if (operation === SequenceOperation.addStepDropped) queue.push(reply(id, generation, 0, new TextEncoder().encode("step-101")));
          else if (operation === SequenceOperation.attachSurface || operation === SequenceOperation.setSize || operation >= SequenceOperation.play && operation <= SequenceOperation.stop) queue.push(reply(id, generation, 0, new Uint8Array()));
          else if (operation === SequenceOperation.gpuReady) queue.push(reply(id, generation, 0, Uint8Array.of(1)));
          else if (operation === SequenceOperation.renderFrame) queue.push(reply(id, generation, 0, new TextEncoder().encode(`{"fixture":${fixture},"labels":{"labels":[]}}`)));
          else if (operation === SequenceOperation.selectedNodeIds) queue.push(reply(id, generation, 0, new TextEncoder().encode("[]")));
          else if (operation === SequenceOperation.catalogueJson) {
            const operationHandle = { slot: 2, generation: 1 };
            queue.push(event(id, generation, 1, 2400, handle(2, 1)));
            const output = new Uint8Array(SEQUENCE_MAX_PAGE_BYTES + 1).fill(120);
            queue.push(event(id, generation, 2, 2406, concat(handle(2, 1), u64(output.length))));
            queue.push(page(operationHandle, 0, output.subarray(0, SEQUENCE_MAX_PAGE_BYTES)));
            queue.push(page(operationHandle, 1, output.subarray(SEQUENCE_MAX_PAGE_BYTES)));
            queue.push(reply(id, generation, 0, new Uint8Array()));
            this.outputDigest = output.reduce((sum, value) => (sum + value) >>> 0, 0);
          } else if (operation === SequenceOperation.run && options.holdRun) held = { id, generation };
          else queue.push(reply(id, generation, 0, new Uint8Array()));
        }
      } else if (tag === 2) {
        reader.u64(); reader.u32();
      } else if (tag === 5) {
        const control = reader.u8();
        if (control === 1) this.cancelControls += 1;
        if (control === 3) { reader.u32(); reader.u32(); reader.u32(); this.pageAcknowledgements += 1; }
      }
      return 1;
    },
    sequence_bridge_poll: (_owner, pointer, capacity) => {
      retained ??= queue[0];
      if (!retained) return this.closed ? -1 : 0;
      if (retained.length > capacity) { this.shortPolls += 1; return retained.length; }
      if (capacity === retained.length && retained.length > SEQUENCE_INITIAL_POLL_BYTES) this.exactRetries += 1;
      new Uint8Array(targetMemory.buffer, pointer, retained.length).set(retained);
      queue.shift();
      const length = retained.length;
      retained = undefined;
      return length;
    },
    sequence_bridge_begin_close: () => { this.closed = true; queue.length = 0; retained = undefined; this.terminalEmpty = true; },
    sequence_bridge_terminal_is_empty: () => this.terminalEmpty ? 1 : 0,
  };
}

function credits(host) {
  const pages = [...host.state.pages.entries()].map(([key, value]) => `${key}:${value.length}:${value.chunks.length}`).join(",") || "0";
  const blocked = host.state.blockedInbound ? `${host.state.blockedInbound.tag}` : "empty";
  return `${host.state.pending.size}:${pages}:${blocked}`;
}

function result(task) { return task.result; }
function hex(bytes) { return [...bytes].map((value) => value.toString(16).padStart(2, "0")).join(""); }

function reply(id, generation, status, body, errorCode, errorMessage = "") {
  return write((w) => { w.u8(1); w.u8(2); w.u64(id); w.u32(generation); w.u16(status); if (errorCode) { w.u8(1); w.u16(errorCode); w.bytes(new TextEncoder().encode(errorMessage)); } else w.u8(0); w.bytes(body); });
}
function event(origin, generation, sequence, code, body) { return write((w) => { w.u8(1); w.u8(3); w.u64(origin ^ (BigInt(sequence) << 32n)); w.u32(generation); w.u32(sequence); w.u16(code); w.u16(0); w.u8(0); w.bytes(body); }); }
function page(value, index, body) { return write((w) => { w.u8(1); w.u8(4); w.u32(value.slot); w.u32(value.generation); w.u32(index); w.bytes(body); }); }
function handle(slot, generation) { return write((w) => { w.u32(slot); w.u32(generation); }); }
function u64(value) { return write((w) => w.u64(BigInt(value))); }
function concat(...values) { const output = new Uint8Array(values.reduce((n, value) => n + value.length, 0)); let cursor = 0; for (const value of values) { output.set(value, cursor); cursor += value.length; } return output; }
function write(build) { const writer = new Writer(); build(writer); return Uint8Array.from(writer.values); }

class Writer {
  constructor() { this.values = []; }
  u8(value) { this.values.push(value & 255); }
  u16(value) { this.number(2, (view) => view.setUint16(0, value, true)); }
  u32(value) { this.number(4, (view) => view.setUint32(0, value, true)); }
  u64(value) { this.number(8, (view) => view.setBigUint64(0, BigInt(value), true)); }
  bytes(value) { this.u32(value.length); this.values.push(...value); }
  number(length, put) { const value = new Uint8Array(length); put(new DataView(value.buffer)); this.values.push(...value); }
}
class Reader {
  constructor(bytes) { this.bytesValue = bytes; this.cursor = 0; }
  take(length) { const value = this.bytesValue.subarray(this.cursor, this.cursor + length); if (value.length !== length) throw new Error("mock length"); this.cursor += length; return value; }
  view(length) { const value = this.take(length); return new DataView(value.buffer, value.byteOffset, value.byteLength); }
  u8() { return this.take(1)[0]; }
  u16() { return this.view(2).getUint16(0, true); }
  u32() { return this.view(4).getUint32(0, true); }
  u64() { return this.view(8).getBigUint64(0, true); }
  bytes() { return this.take(this.u32()); }
}
