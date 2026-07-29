/** @generated semio program host shim */

export function log(level, message) {
  if (level === "error") console.error(`[plugin] ${message}`);
  else console.log(`[plugin] ${message}`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function readDocument(handle) {
  throw `read-document unsupported: ${handle}`;
}

export function writeDocument(handle, payloadJson) {
  throw `write-document unsupported: ${handle}`;
}

export function openWindow(kind, paramsJson) {
  throw `open-window unsupported: ${kind}`;
}

export function invokeAction(target, invocationJson) {
  throw `invoke-action unsupported: ${target}`;
}

export function readAsset(handle) {
  throw `read-asset unsupported: ${handle}`;
}

export function networkFetch(origin, path) {
  throw `network-fetch unsupported: ${origin}${path}`;
}

// 📦 Must match `framework/product/os/core/js/index.ts`'s `BLOB_ENDPOINT_PATH`.
const BLOB_ENDPOINT_PATH = "/semio-blob";

/** @emoji 📦 Persists `data` to the dev server's content-addressed blob store, returning its hash.
 * `write-blob`/`read-blob` are declared synchronous in the WIT world (no `async` on the host import),
 * so this can't use `fetch` — a dedicated worker (unlike the main thread) still permits synchronous
 * `XMLHttpRequest`, which is the standard sync-bridge trick for exactly this constraint. */
export function writeBlob(data, mediaType) {
  const xhr = new XMLHttpRequest();
  xhr.open("PUT", `${BLOB_ENDPOINT_PATH}?mediaType=${encodeURIComponent(mediaType)}`, false);
  xhr.send(new Uint8Array(data));
  if (xhr.status < 200 || xhr.status >= 300) throw `write-blob failed (${xhr.status})`;
  return JSON.parse(xhr.responseText).hash;
}

/** @emoji 📦 Fetches a previously written blob's bytes by hash. See `writeBlob` for why this is a
 * synchronous XHR rather than `fetch`. */
export function readBlob(hash) {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", `${BLOB_ENDPOINT_PATH}/${encodeURIComponent(hash)}`, false);
  xhr.responseType = "arraybuffer";
  xhr.send();
  if (xhr.status === 404) throw `blob not found: ${hash}`;
  if (xhr.status < 200 || xhr.status >= 300) throw `read-blob failed (${xhr.status})`;
  return new Uint8Array(xhr.response);
}

// 🔗 Per-uri inbound queues (serialized `BackboneMessage`s), shared on the worker global so the program
// worker's `backboneInbound` relay (see pluginWorkerSource) can fill them while this shim drains them —
// the two scripts live in the same worker realm but are separate modules.
function backboneInboundQueues() {
  return (globalThis.__semioBackboneInbound ??= new Map());
}
const backboneAttached = new Set();

/** @emoji 📤 Enqueues an outbound message to the main thread, which relays it into `backbone-worker.ts`
 * (the sync actor). Inside a dedicated worker this is postMessage-only (a worker can't own the
 * socket/fetch itself); when this component is instead loaded directly on the main thread (the
 * no-`Worker`/component-model-load fallback in `framework/core/js/index.ts`), it reaches the same
 * relay through the well-known `__semioMainThreadPluginBackboneOutbound` global instead. */
export function backboneSend(uri, messageJson) {
  backboneAttached.add(uri);
  if (typeof WorkerGlobalScope !== "undefined" && typeof self !== "undefined" && typeof self.postMessage === "function") {
    self.postMessage({ type: "backboneOutbound", uri, message: messageJson });
  } else if (typeof globalThis.__semioMainThreadPluginBackboneOutbound === "function") {
    globalThis.__semioMainThreadPluginBackboneOutbound(uri, messageJson);
  }
}

/** @emoji 📥 Drains the inbound queue the worker filled from `backboneInbound` postMessages. Returns
 * serialized `BackboneMessage`s (never blocks — an empty queue yields `[]`). */
export function backbonePoll(uri) {
  backboneAttached.add(uri);
  const queues = backboneInboundQueues();
  const queue = queues.get(uri);
  if (!queue || queue.length === 0) return [];
  queues.set(uri, []);
  return queue;
}

/** @emoji 📶 Reports whether this shim has seen traffic for a uri (the real transport health lives in
 * `backbone-worker.ts`; the sandboxed program only needs attached/detached). */
export function backboneStatus(uri) {
  return backboneAttached.has(uri) ? "attached" : "detached";
}
