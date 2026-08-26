//#region 🧬️Contract

export const SEQUENCE_MAX_REQUEST_BYTES = 1_048_576;
export const SEQUENCE_MAX_PAGE_BYTES = 65_536;
export const SEQUENCE_MAX_TRANSFER_BYTES = 16_777_216;
export const SEQUENCE_MAX_ENCODED_MESSAGE_BYTES = SEQUENCE_MAX_REQUEST_BYTES + 27;
export const SEQUENCE_INITIAL_POLL_BYTES = 1_024;
export const SEQUENCE_MAX_IN_FLIGHT = 256;

export const SequenceOperation = Object.freeze({
  open: 2300,
  loadFixtureJson: 2301,
  fixtureJson: 2302,
  catalogueJson: 2303,
  addStep: 2304,
  addStepDropped: 2305,
  addStepToSlot: 2306,
  setStepCollapsed: 2307,
  pickStepIdAtScreen: 2308,
  buildPathJson: 2309,
  removeStep: 2310,
  setStepParamsJson: 2311,
  connectSteps: 2312,
  disconnectSteps: 2313,
  compileText: 2314,
  compiledWireLiteral: 2315,
  run: 2316,
  attachSurface: 2317,
  gpuReady: 2318,
  setSize: 2319,
  renderFrame: 2320,
  worldFromScreen: 2321,
  pointerDownScreen: 2322,
  pointerMoveScreen: 2323,
  pointerUpScreen: 2324,
  wheelScreen: 2325,
  reorganize: 2326,
  lodScaleJson: 2327,
  setAutomaticLod: 2328,
  setForcedDrawLodLabel: 2329,
  drawLodLabel: 2330,
  setCanvasThemeJson: 2331,
  selectedNodeIds: 2332,
  setSelection: 2333,
  labelOverlayPaintStateJson: 2334,
  hoveredNodeId: 2335,
  preselectNodeIdsJson: 2336,
  selectionPreviewPointsJson: 2337,
  selectionPreviewCrossing: 2338,
  selectionPreviewMethod: 2339,
  selectionUnionBoundsScreenJson: 2340,
  setSelectionOptions: 2341,
  setGhostStep: 2342,
  clearGhostStep: 2343,
  play: 2344,
  pause: 2345,
  stop: 2346,
});

//#endregion 🧬️Contract

//#region 🌉️LinearMemoryHost

export function createSequenceHost({ exports, memory, resolveCanvas, render = defaultRender, schedule = queueMicrotask, maximumInFlight = SEQUENCE_MAX_IN_FLIGHT } = {}) {
  if (!exports || !memory) throw new Error("Sequence Wasm exports and memory are required");
  const state = {
    nextRequest: 1n,
    generation: 1,
    nextCanvas: 1,
    nextSurface: 1,
    pending: new Map(),
    pages: new Map(),
    canvases: new Map(),
    surfaces: new Map(),
    blockedInbound: undefined,
    pumping: false,
    closing: false,
    closePromise: undefined,
    closed: false,
  };

  const transfer = (bytes, credit = Math.max(bytes.length, 1)) => {
    if (bytes.length > SEQUENCE_MAX_ENCODED_MESSAGE_BYTES) throw new Error("Sequence message exceeds the bounded envelope");
    const pointer = exports.sequence_bridge_allocate(bytes.length);
    if (!pointer) throw new Error("Sequence allocation failed");
    try {
      new Uint8Array(memory.buffer, pointer, bytes.length).set(bytes);
      if (exports.sequence_bridge_send(pointer, bytes.length, credit) !== 1) throw new Error("Sequence message rejected");
    } finally {
      exports.sequence_bridge_release(pointer, bytes.length);
    }
  };

  const pollExact = () => {
    let capacity = SEQUENCE_INITIAL_POLL_BYTES;
    let pointer = exports.sequence_bridge_allocate(capacity);
    if (!pointer) throw new Error("Sequence poll allocation failed");
    try {
      let length = exports.sequence_bridge_poll(pointer, capacity, 4_096);
      if (length <= 0) return { length };
      if (length > SEQUENCE_MAX_ENCODED_MESSAGE_BYTES) throw new Error("Sequence poll length exceeds the bounded envelope");
      if (length > capacity) {
        exports.sequence_bridge_release(pointer, capacity);
        capacity = length;
        pointer = exports.sequence_bridge_allocate(capacity);
        if (!pointer) throw new Error("Sequence exact retry allocation failed");
        length = exports.sequence_bridge_poll(pointer, capacity, 4_096);
        if (length !== capacity) throw new Error("Sequence retained poll changed length");
      }
      return { length, bytes: new Uint8Array(memory.buffer, pointer, length).slice() };
    } finally {
      exports.sequence_bridge_release(pointer, capacity);
    }
  };

  const transferControl = (bytes, commit) => {
    try {
      transfer(bytes);
    } catch {
      return false;
    }
    commit();
    return true;
  };

  const accept = (message) => {
    if (message.tag === 3) {
      const origin = message.requestId ^ (BigInt(message.sequence) << 32n);
      let operationHandle;
      let output;
      if (message.event === 2400) operationHandle = readHandle(new Reader(message.body));
      else if (message.event === 2406) {
        const reader = new Reader(message.body);
        const handle = readHandle(reader);
        const total = Number(reader.u64());
        reader.finish();
        if (total > SEQUENCE_MAX_TRANSFER_BYTES) throw new Error("Sequence transfer exceeds its admitted length");
        output = { handle, total };
      }
      return transferControl(encodeReply(message.requestId, message.generation, 0, new Uint8Array()), () => {
        if (operationHandle) {
          const pending = state.pending.get(origin);
          if (pending) pending.operationHandle = operationHandle;
        }
        if (output) state.pages.set(handleKey(output.handle), { total: output.total, chunks: [], length: 0 });
        const pending = state.pending.get(origin);
        if (pending) for (const observer of pending.observers) try { observer(message); } catch {}
      });
    }
    if (message.tag === 4) {
      if (message.body.length > SEQUENCE_MAX_PAGE_BYTES) throw new Error("Sequence page exceeds its bound");
      const key = handleKey(message.handle);
      const transferState = state.pages.get(key);
      if (!transferState) throw new Error("Sequence page has no admitted output");
      if (message.index !== transferState.chunks.length) throw new Error("Sequence page out of order");
      const nextLength = transferState.length + message.body.length;
      if (nextLength > transferState.total || nextLength > SEQUENCE_MAX_TRANSFER_BYTES) throw new Error("Sequence transfer exceeds its admitted length");
      return transferControl(encodeAcknowledge(message.handle, message.index), () => {
        transferState.chunks.push(message.body);
        transferState.length = nextLength;
      });
    }
    if (message.tag !== 2) throw new Error("Unexpected Sequence message");
    const pending = state.pending.get(message.requestId);
    if (!pending) return true;
    state.pending.delete(message.requestId);
    if (message.status !== 0) {
      pending.reject(new Error(message.errorMessage || `Sequence status ${message.status}`));
      return true;
    }
    let body = message.body;
    if (pending.operationHandle) {
      const paged = state.pages.get(handleKey(pending.operationHandle));
      if (paged) {
        if (paged.length !== paged.total) throw new Error("Sequence transfer completed before its exact byte count");
        body = joinBytes(paged.chunks, paged.total);
        state.pages.delete(handleKey(pending.operationHandle));
      }
    }
    pending.resolve(body);
    return true;
  };

  const pump = () => {
    if (state.pumping) return;
    state.pumping = true;
    const step = () => {
      try {
        let advanced = 0;
        while (advanced < 64) {
          const retained = state.blockedInbound;
          if (retained) {
            if (!accept(retained)) break;
            state.blockedInbound = undefined;
            advanced += 1;
            continue;
          }
          const result = pollExact();
          if (result.length < 0) {
            const error = new Error("Sequence bridge closed while requests were pending");
            for (const pending of state.pending.values()) pending.reject(error);
            state.pending.clear();
            state.closed = true;
            break;
          }
          if (result.length === 0) break;
          const message = decodeMessage(result.bytes);
          if (!accept(message)) {
            state.blockedInbound = message;
            break;
          }
          advanced += 1;
        }
      } catch (error) {
        for (const pending of state.pending.values()) pending.reject(error);
        state.pending.clear();
        state.closed = true;
      } finally {
        if (!state.closed && !state.closing && (state.pending.size > 0 || state.blockedInbound)) schedule(step);
        else state.pumping = false;
      }
    };
    schedule(step);
  };

  const start = (operation, payload = new Uint8Array(), session) => {
    if (state.closed || state.closing) return rejectedTask(new Error("Sequence host is closed"));
    if (state.pending.size >= maximumInFlight) return rejectedTask(new Error("Sequence in-flight limit"));
    const requestId = state.nextRequest;
    let message;
    try {
      const body = session ? concat(encodeHandle(session), payload) : payload;
      if (body.length > SEQUENCE_MAX_REQUEST_BYTES) throw new Error("Sequence request exceeds its bound");
      message = encodeRequest(operation, requestId, state.generation, body);
      transfer(message);
    } catch (error) {
      return rejectedTask(error);
    }
    state.nextRequest += 1n;
    let settle;
    const result = new Promise((resolve, reject) => { settle = { resolve, reject, operationHandle: undefined, observers: new Set() }; });
    state.pending.set(requestId, settle);
    try {
      pump();
    } catch (error) {
      state.pending.delete(requestId);
      settle.reject(error);
      try { transfer(encodeCancel(requestId, state.generation)); } catch {}
    }
    return {
      requestId,
      result,
      cancel: () => cancel(requestId),
      subscribe(observer) {
        settle.observers.add(observer);
        return () => settle.observers.delete(observer);
      },
    };
  };

  const cancel = (requestId) => {
    const pending = state.pending.get(requestId);
    if (!pending) return false;
    try {
      transfer(encodeCancel(requestId, state.generation));
    } catch (error) { throw error; }
    pump();
    return true;
  };

  const closeHandle = (handle) => {
    transfer(encodeClose(handle));
    return true;
  };

  const registerCanvas = (canvas) => {
    if (!canvas) throw new Error("Sequence canvas is unavailable");
    const canvasId = state.nextCanvas++;
    const surfaceId = state.nextSurface++;
    state.canvases.set(canvasId, canvas);
    state.surfaces.set(surfaceId, { canvasId, canvas });
    return { canvasId, surfaceId };
  };

  const close = () => {
    if (state.closePromise) return state.closePromise;
    if (state.closed) return Promise.resolve();
    exports.sequence_bridge_begin_close();
    state.closing = true;
    state.canvases.clear();
    state.surfaces.clear();
    for (const pending of state.pending.values()) pending.reject(new Error("Sequence host closed"));
    state.pending.clear();
    state.closePromise = new Promise((resolve, reject) => {
      const drain = () => {
        try {
          for (let advanced = 0; advanced < 64; advanced += 1) {
            const retained = state.blockedInbound;
            if (retained) {
              if (!accept(retained)) break;
              state.blockedInbound = undefined;
              continue;
            }
            if (exports.sequence_bridge_terminal_is_empty() === 1) {
              state.pages.clear();
              state.blockedInbound = undefined;
              state.closed = true;
              state.closing = false;
              resolve();
              return;
            }
            const result = pollExact();
            if (result.length > 0) {
              const message = decodeMessage(result.bytes);
              if (!accept(message)) {
                state.blockedInbound = message;
                break;
              }
            }
            else if (result.length < 0) throw new Error("Sequence bridge closed before terminal-empty");
            else break;
          }
          schedule(drain);
        } catch (error) {
          state.closed = true;
          state.closing = false;
          reject(error);
        }
      };
      schedule(drain);
    });
    return state.closePromise;
  };

  return { state, start, cancel, closeHandle, registerCanvas, resolveCanvas, render, close, terminalIsEmpty: () => exports.sequence_bridge_terminal_is_empty() === 1 };
}

//#endregion 🌉️LinearMemoryHost

//#region 🧩️ReactiveFeatures

export async function createSequenceFeatures(host) {
  if (!host) throw new Error("Sequence host is required");
  const opened = host.start(SequenceOperation.open);
  const session = readHandle(new Reader(await opened.result));
  const task = (operation, payload, decode = identity) => mapTask(host.start(operation, payload, session), decode);
  let surface;

  const document = {
    loadFixtureJson: (json) => task(SequenceOperation.loadFixtureJson, encoder.encode(json)),
    fixtureJson: () => task(SequenceOperation.fixtureJson, undefined, text),
    catalogueJson: () => task(SequenceOperation.catalogueJson, undefined, text),
  };
  const editing = {
    addStep: (kind, x, y) => task(SequenceOperation.addStep, fields((w) => { w.text(kind); w.f64(x); w.f64(y); }), text),
    addStepDropped: (kind, x, y, picked) => task(SequenceOperation.addStepDropped, fields((w) => { w.text(kind); w.f64(x); w.f64(y); w.optionalText(picked); }), text),
    addStepToSlot: (kind, x, y, owner, name) => task(SequenceOperation.addStepToSlot, fields((w) => { w.text(kind); w.f64(x); w.f64(y); w.text(owner); w.text(name); }), text),
    setStepCollapsed: (id, value) => task(SequenceOperation.setStepCollapsed, fields((w) => { w.text(id); w.u8(value ? 1 : 0); }), boolean),
    pickStepIdAtScreen: (x, y) => task(SequenceOperation.pickStepIdAtScreen, pointFields(x, y), optionalString),
    buildPathJson: () => task(SequenceOperation.buildPathJson, undefined, text),
    removeStep: (id) => task(SequenceOperation.removeStep, textField(id), boolean),
    setStepParamsJson: (id, json) => task(SequenceOperation.setStepParamsJson, twoTextFields(id, json)),
    connectSteps: (from, to) => task(SequenceOperation.connectSteps, twoTextFields(from, to), text),
    disconnectSteps: (from, to) => task(SequenceOperation.disconnectSteps, twoTextFields(from, to), boolean),
  };
  const execution = {
    compileText: () => task(SequenceOperation.compileText, undefined, text),
    compiledWireLiteral: () => task(SequenceOperation.compiledWireLiteral, undefined, text),
    run: () => task(SequenceOperation.run, undefined, text),
  };
  const viewport = {
    attach(canvas) {
      const owned = host.registerCanvas(canvas ?? host.resolveCanvas?.());
      const attached = task(SequenceOperation.attachSurface, fields((w) => { w.u32(owned.surfaceId); w.u32(owned.canvasId); }));
      attached.result = attached.result.then((value) => { surface = owned; return value; });
      return attached;
    },
    gpuReady: () => task(SequenceOperation.gpuReady, undefined, boolean),
    setSize: (width, height, dpr) => task(SequenceOperation.setSize, fields((w) => { w.u32(Math.max(1, width)); w.u32(Math.max(1, height)); w.f64(Math.max(1, dpr)); })),
    renderFrame() {
      if (!surface) return rejectedTask(new Error("Sequence canvas is unavailable"));
      return task(SequenceOperation.renderFrame, undefined, (bytes) => {
        const state = JSON.parse(text(bytes));
        host.render(host.state.canvases.get(surface.canvasId), state);
        return state;
      });
    },
  };
  const input = {
    worldFromScreen: (x, y) => task(SequenceOperation.worldFromScreen, pointFields(x, y), text),
    pointerDownScreen: (x, y, button, shift, ctrl, alt) => task(SequenceOperation.pointerDownScreen, fields((w) => { w.f64(x); w.f64(y); w.u8(button); w.bool(shift); w.bool(ctrl); w.bool(alt); })),
    pointerMoveScreen: (x, y, shift, ctrl, alt) => task(SequenceOperation.pointerMoveScreen, pointerFields(x, y, shift, ctrl, alt)),
    pointerUpScreen: (x, y, shift, ctrl, alt) => task(SequenceOperation.pointerUpScreen, pointerFields(x, y, shift, ctrl, alt)),
    wheelScreen: (x, y, deltaY) => task(SequenceOperation.wheelScreen, fields((w) => { w.f64(x); w.f64(y); w.f64(deltaY); })),
  };
  const layout = {
    reorganize: (json) => task(SequenceOperation.reorganize, encoder.encode(json)),
    lodScaleJson: () => task(SequenceOperation.lodScaleJson, undefined, text),
    setAutomaticLod: (value) => task(SequenceOperation.setAutomaticLod, Uint8Array.of(value ? 1 : 0)),
    setForcedDrawLodLabel: (value) => task(SequenceOperation.setForcedDrawLodLabel, textField(value)),
    drawLodLabel: () => task(SequenceOperation.drawLodLabel, undefined, text),
    setCanvasThemeJson: (json) => task(SequenceOperation.setCanvasThemeJson, encoder.encode(json)),
  };
  const selection = {
    selectedNodeIds: () => task(SequenceOperation.selectedNodeIds, undefined, json),
    setSelection: (ids) => task(SequenceOperation.setSelection, fields((w) => { w.u32(ids.length); for (const id of ids) w.text(id); })),
    labelOverlayPaintStateJson: () => task(SequenceOperation.labelOverlayPaintStateJson, undefined, text),
    hoveredNodeId: () => task(SequenceOperation.hoveredNodeId, undefined, optionalString),
  };
  const preview = {
    preselectNodeIdsJson: () => task(SequenceOperation.preselectNodeIdsJson, undefined, text),
    selectionPreviewPointsJson: () => task(SequenceOperation.selectionPreviewPointsJson, undefined, text),
    selectionPreviewCrossing: () => task(SequenceOperation.selectionPreviewCrossing, undefined, boolean),
    selectionPreviewMethod: () => task(SequenceOperation.selectionPreviewMethod, undefined, text),
    selectionUnionBoundsScreenJson: () => task(SequenceOperation.selectionUnionBoundsScreenJson, undefined, text),
    setSelectionOptions: (method, mode) => task(SequenceOperation.setSelectionOptions, twoTextFields(method, mode)),
    setGhostStep: (kind, x, y) => task(SequenceOperation.setGhostStep, fields((w) => { w.text(kind); w.f64(x); w.f64(y); })),
    clearGhostStep: () => task(SequenceOperation.clearGhostStep),
  };
  const playback = {
    play: () => task(SequenceOperation.play),
    pause: () => task(SequenceOperation.pause),
    stop: () => task(SequenceOperation.stop),
  };
  const lifetime = {
    session,
    async close() {
      host.closeHandle(session);
      await host.close();
    },
    terminalIsEmpty: host.terminalIsEmpty,
  };
  return { document, editing, execution, viewport, input, layout, selection, preview, playback, lifetime };
}

//#endregion 🧩️ReactiveFeatures

//#region 🧱️Codec

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });

function encodeRequest(operation, requestId, generation, body) {
  return fields((writer) => { writer.u8(1); writer.u8(1); writer.u16(operation); writer.u64(requestId); writer.u32(generation); writer.bytes(body); });
}

function encodeReply(requestId, generation, status, body) {
  return fields((writer) => { writer.u8(1); writer.u8(2); writer.u64(requestId); writer.u32(generation); writer.u16(status); writer.u8(0); writer.bytes(body); });
}

function encodeCancel(requestId, generation) {
  return fields((writer) => { writer.u8(1); writer.u8(5); writer.u8(1); writer.u64(requestId); writer.u32(generation); });
}

function encodeAcknowledge(handle, index) {
  return fields((writer) => { writer.u8(1); writer.u8(5); writer.u8(3); writer.handle(handle); writer.u32(index); });
}

function encodeClose(handle) {
  return fields((writer) => { writer.u8(1); writer.u8(5); writer.u8(2); writer.handle(handle); });
}

export function decodeMessage(bytes) {
  const reader = new Reader(bytes);
  if (reader.u8() !== 1) throw new Error("Sequence ABI version");
  const tag = reader.u8();
  if (tag === 2) {
    const requestId = reader.u64();
    const generation = reader.u32();
    const status = reader.u16();
    const error = reader.u8();
    let errorMessage = "";
    if (error === 1) { reader.u16(); errorMessage = decoder.decode(reader.bytes()); }
    else if (error !== 0) throw new Error("Sequence reply error marker");
    const body = reader.bytes(); reader.finish(); return { tag, requestId, generation, status, errorMessage, body };
  }
  if (tag === 3) {
    const requestId = reader.u64();
    const generation = reader.u32();
    const sequence = reader.u32();
    const event = reader.u16();
    const status = reader.u16();
    const error = reader.u8();
    if (error === 1) { reader.u16(); reader.bytes(); } else if (error !== 0) throw new Error("Sequence event error marker");
    const body = reader.bytes(); reader.finish(); return { tag, requestId, generation, sequence, event, status, body };
  }
  if (tag === 4) {
    const handle = readHandle(reader);
    const index = reader.u32();
    const body = reader.bytes(); reader.finish(); return { tag, handle, index, body };
  }
  throw new Error("Unexpected Sequence ABI tag");
}

class Writer {
  constructor() { this.values = []; }
  u8(value) { this.values.push(value & 255); }
  u16(value) { this.number(2, (view) => view.setUint16(0, value, true)); }
  u32(value) { this.number(4, (view) => view.setUint32(0, value, true)); }
  u64(value) { this.number(8, (view) => view.setBigUint64(0, BigInt(value), true)); }
  f64(value) { if (!Number.isFinite(value)) throw new Error("non-finite Sequence number"); this.number(8, (view) => view.setFloat64(0, value, true)); }
  bool(value) { this.u8(value ? 1 : 0); }
  bytes(value) { this.u32(value.length); this.raw(value); }
  text(value) { this.bytes(encoder.encode(value)); }
  optionalText(value) { if (value === undefined || value === null) this.u8(0); else { this.u8(1); this.text(value); } }
  handle(value) { this.u32(value.slot); this.u32(value.generation); }
  raw(value) { this.values.push(...value); }
  number(length, write) { const bytes = new Uint8Array(length); write(new DataView(bytes.buffer)); this.raw(bytes); }
  finish() { return Uint8Array.from(this.values); }
}

class Reader {
  constructor(bytes) { this.value = bytes; this.cursor = 0; }
  take(length) { const result = this.value.subarray(this.cursor, this.cursor + length); if (result.length !== length) throw new Error("Sequence malformed length"); this.cursor += length; return result; }
  view(length) { const value = this.take(length); return new DataView(value.buffer, value.byteOffset, value.byteLength); }
  u8() { return this.take(1)[0]; }
  u16() { return this.view(2).getUint16(0, true); }
  u32() { return this.view(4).getUint32(0, true); }
  u64() { return this.view(8).getBigUint64(0, true); }
  bytes() { return this.take(this.u32()); }
  finish() { if (this.cursor !== this.value.length) throw new Error("Sequence trailing bytes"); }
}

function fields(build) { const writer = new Writer(); build(writer); return writer.finish(); }
function textField(value) { return fields((writer) => writer.text(value)); }
function twoTextFields(first, second) { return fields((writer) => { writer.text(first); writer.text(second); }); }
function pointFields(x, y) { return fields((writer) => { writer.f64(x); writer.f64(y); }); }
function pointerFields(x, y, shift, ctrl, alt) { return fields((writer) => { writer.f64(x); writer.f64(y); writer.bool(shift); writer.bool(ctrl); writer.bool(alt); }); }
function encodeHandle(handle) { return fields((writer) => writer.handle(handle)); }
function readHandle(reader) { return { slot: reader.u32(), generation: reader.u32() }; }
function handleKey(handle) { return `${handle.slot}:${handle.generation}`; }
function optionalString(bytes) { return bytes.length === 0 ? undefined : decoder.decode(bytes); }
function identity(value) { return value; }
function text(bytes) { return decoder.decode(bytes); }
function json(bytes) { return JSON.parse(text(bytes)); }
function boolean(bytes) { return bytes[0] === 1; }
function mapTask(task, decode) { return { ...task, result: task.result.then(decode) }; }
function rejectedTask(error) { return { requestId: undefined, result: Promise.reject(error), cancel: () => false, subscribe: () => () => {} }; }
function concat(...values) { const length = values.reduce((total, value) => total + value.length, 0); const bytes = new Uint8Array(length); let cursor = 0; for (const value of values) { bytes.set(value, cursor); cursor += value.length; } return bytes; }
function joinBytes(values, length) { const bytes = new Uint8Array(length); let cursor = 0; for (const value of values) { bytes.set(value, cursor); cursor += value.length; } return bytes; }

function defaultRender(canvas, state) {
  const context = canvas?.getContext?.("2d");
  if (!context) return;
  const ratio = globalThis.devicePixelRatio || 1;
  const width = Math.max(1, Math.round(canvas.clientWidth * ratio));
  const height = Math.max(1, Math.round(canvas.clientHeight * ratio));
  if (canvas.width !== width) canvas.width = width;
  if (canvas.height !== height) canvas.height = height;
  context.clearRect(0, 0, width, height);
  context.save();
  context.scale(ratio, ratio);
  context.fillStyle = "#20242b";
  context.strokeStyle = "#8ea1b5";
  for (const step of state.fixture?.steps ?? []) {
    context.fillRect(step.x - 60, step.y - 20, 120, 40);
    context.strokeRect(step.x - 60, step.y - 20, 120, 40);
  }
  context.restore();
}

//#endregion 🧱️Codec
