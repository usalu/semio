/** @generated semio actor host shim — implements the pure AND host-async import interfaces
 * (component.wit ~:823 / ~:887). See plugin-web-materialize.ts's hostShimSource doc for the design
 * this file is generated from. */

//#region 🧬️pure
export function log(level, message) {
  if (level === "error") console.error(`[actor] ${message}`);
  else console.log(`[actor] ${message}`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function traceSpan(name) {
  if (typeof performance !== "undefined" && typeof performance.mark === "function") performance.mark(name);
}
//#endregion 🧬️pure

//#region 🌉️host-async
let boundActorId = null;
let effectSeq = 0;
const pendingEffects = new Map();

// 🌉️ Called once by `createActorApi(actorId)` (`pluginComponentBridgeSource`), right after this
// module is imported for that actor — every subsequent `effectRequest` in THIS module instance is
// tagged with `actorId`. Safe as module-scoped (not global) state ONLY because `🟨️shard-worker.js`
// dynamically `import()`s a distinct moduleUrl per actor (`loadActor`'s own doc), so each actor gets
// its own copy of this file's top-level state — never shared across actors, even under the
// cross-actor interleaving this worker's header doc describes.
export function __bindHostBridge(actorId) {
  boundActorId = actorId;
}

// 🌉️ Settles the Promise `effectRequest` handed back for `requestId` — called by `🟨️shard-worker.js`
// when an `effect-complete`/`effect-error` envelope for this actor arrives.
export function __resolveEffect(requestId, value) {
  const entry = pendingEffects.get(requestId);
  if (!entry) return;
  pendingEffects.delete(requestId);
  entry.resolve(value);
}

export function __rejectEffect(requestId, message) {
  const entry = pendingEffects.get(requestId);
  if (!entry) return;
  pendingEffects.delete(requestId);
  entry.reject(new Error(message));
}

// 🌊️ jco's proven shape for a guest-consumable `stream<u8>` is a plain async generator yielding ONE
// byte at a time — confirmed against a real component (jcoprobe's `fetchBody`/`read-body`, S4 in
// 📓️terra-jco-spike-report.md), NOT a `ReadableStream` directly. An `effect-complete` for
// `http-fetch`/`blob-read` is expected to carry a `ReadableStream` (structured-clone-transferable
// across `postMessage`); this adapts it into the proven per-byte generator shape. Also accepts an
// already-async-iterable value so a kernel handing back a plain byte array still works.
async function* streamToByteGenerator(body) {
  if (body == null) return;
  if (typeof body[Symbol.asyncIterator] === "function") {
    for await (const chunk of body) {
      if (chunk instanceof Uint8Array) { for (const byte of chunk) yield byte; } else yield chunk;
    }
    return;
  }
  const reader = body.getReader();
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) return;
      if (value instanceof Uint8Array) { for (const byte of value) yield byte; } else yield value;
    }
  } finally {
    reader.releaseLock();
  }
}

// 🚪️ Every `host-async` ASYNC import funnels through here — posts one `ShardFrame::Envelope` up to
// the kernel over the SAME shape `🧵️shard-client.ts` declares (`to`/`from`/`lane`/`seq`/
// `deadlineMs`/`coalesce`/`cancelOf`/`payload`), with `payload: {kind: "effect-request", payload:
// {effect, requestId, params}}` — the SAME `{kind, payload}` envelope-payload shape
// `ShardEventEnvelope` already uses for turn events, reused rather than inventing a new one. Resolves
// or rejects once `__resolveEffect`/`__rejectEffect` fires for the matching `requestId`.
function effectRequest(effect, params) {
  const requestId = `${boundActorId}:${effect}:${++effectSeq}`;
  return new Promise((resolve, reject) => {
    pendingEffects.set(requestId, { resolve, reject });
    self.postMessage({
      kind: "frame",
      actorId: boundActorId,
      frame: {
        kind: "Envelope",
        envelope: {
          to: "kernel",
          from: { kind: "actor", id: boundActorId },
          lane: "Background",
          seq: effectSeq,
          deadlineMs: null,
          coalesce: null,
          cancelOf: null,
          payload: { kind: "effect-request", payload: { effect, requestId, params } },
        },
      },
    });
  });
}

// 🚪️ `emit`/`emit-patch` are plain (non-async) WIT `func`s — the ONE fire-and-forget door for the
// ~24 one-way `effect` variants (plus `respond`) and for UI patches. No `requestId`/Promise: posts
// and returns immediately, same envelope shape as `effectRequest` above minus the correlation.
function postFireAndForget(kind, payload) {
  self.postMessage({
    kind: "frame",
    actorId: boundActorId,
    frame: {
      kind: "Envelope",
      envelope: { to: "kernel", from: { kind: "actor", id: boundActorId }, lane: "Background", seq: ++effectSeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload } },
    },
  });
}

const call = (effect) => (params) => effectRequest(effect, params);
export const storageRead = call("storage-read");
export const storageWrite = call("storage-write");
export const storageDelete = call("storage-delete");
export const blobLoad = call("blob-load");
export const blobWrite = call("blob-write");
export const blobRead = (hash) => effectRequest("blob-read", { hash }).then(streamToByteGenerator);
export const httpFetch = (params) => effectRequest("http-fetch", params).then((response) => ({ ...response, body: streamToByteGenerator(response.body) }));
export const documentRead = call("document-read");
export const documentWrite = call("document-write");
export const linkResolve = (link) => effectRequest("link-resolve", { link });
export const registryQuery = call("registry-query");
export const ioCompose = call("io-compose");
export const ioRun = call("io-run");
export const cacheDerive = call("cache-derive");
export const cacheRead = call("cache-read");
export const invokeExtension = call("invoke-extension");
export const openWindow = call("open-window");
export const openDialog = call("open-dialog");
export const dispatchAction = call("dispatch-action");
export const spawnPluginInstance = call("spawn-plugin-instance");
export const requestFileOpen = call("request-file-open");
export const requestMediaFrames = call("request-media-frames");
export const requestCapability = call("request-capability");
export const spawnJob = (job, kind, input, placement) => effectRequest("spawn-job", { job, kind, input, placement });

export function emit(value) {
  postFireAndForget("effect-emit", value);
}

export function emitPatch(patch) {
  postFireAndForget("ui-patch-emit", patch);
}
//#endregion 🌉️host-async
