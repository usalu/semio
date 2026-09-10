/** 🩺️ Weighs the guest side of the extension-result delivery path on the REAL staged
 * `wasm32-wasip2` module: every `cabi_realloc` the jco lowering asks for, and `memory.buffer
 * .byteLength` before and after every `poll` turn. Boot #12 of
 * `📓️runtime-verification-2026-09-09.md` died with `unreachable … abort ← abort_internal ←
 * cabi_realloc ← poll` on the FIRST `Event::Completed` delivery, which is either an absurd lowered
 * size or an exhausted guest heap — this probe tells the two apart by printing both.
 *
 * `WebAssembly.instantiate` is patched BEFORE the component module is imported (jco captures it once
 * at module scope as `instantiateCore`), so the returned `{ exports }` can be a plain object whose
 * `cabi_realloc` is a logging shim around the real export.
 *
 * Run: `node --experimental-wasm-jspi <this> <module-dir> [payloadBytes] [turns]`. */
const [, , moduleDirArg, payloadText, turnsText] = process.argv;
if (!moduleDirArg) throw new Error("usage: probe <staged-module-dir> [payloadBytes] [turns]");
const payloadBytes = Number(payloadText ?? 1048576);
const turns = Number(turnsText ?? 24);
const started = Date.now();
const log = (...parts) => console.log(`[probe ${String(Date.now() - started).padStart(6)}ms]`, ...parts);
process.on("unhandledRejection", (reason) => { log("UNHANDLED REJECTION", reason?.stack ?? reason); process.exit(3); });

const reallocCalls = [];
let reallocBytes = 0;
let guestMemory = null;
let recording = false;
const nativeInstantiate = WebAssembly.instantiate.bind(WebAssembly);
WebAssembly.instantiate = async function instantiate(module, imports) {
  const result = await nativeInstantiate(module, imports);
  const instance = result instanceof WebAssembly.Instance ? result : result.instance;
  const exports = instance.exports;
  if (typeof exports.cabi_realloc !== "function") return result;
  guestMemory = exports.memory ?? guestMemory;
  const real = exports.cabi_realloc;
  const shim = (oldPtr, oldLen, align, newLen) => {
    if (recording) { reallocCalls.push(newLen); reallocBytes += newLen; }
    return real(oldPtr, oldLen, align, newLen);
  };
  const copied = Object.create(null);
  for (const key of Object.keys(exports)) copied[key] = exports[key];
  copied.cabi_realloc = shim;
  return { instance, module: result.module ?? module, exports: copied };
};

const megabytes = (value) => `${(value / 1048576).toFixed(2)}MB`;
const memoryBytes = () => guestMemory?.buffer.byteLength ?? 0;
const takeRealloc = () => {
  const calls = reallocCalls.length;
  const bytes = reallocBytes;
  const largest = calls === 0 ? 0 : Math.max(...reallocCalls);
  reallocCalls.length = 0;
  reallocBytes = 0;
  return { calls, bytes, largest };
};

const directory = moduleDirArg.endsWith("/") ? moduleDirArg : `${moduleDirArg}/`;
const name = directory.split("/").filter(Boolean).pop();
const bridge = await import(new URL(`file://${directory}\u{1F309}️bridge.js`).href);
const api = await bridge.createActorApi(`${name}#1`, 1n);
recording = true;
log(`${name}: instantiated memory=${megabytes(memoryBytes())}`);

const budget = { fuel: 80000000, wallMs: 5000, memoryBytes: 268435456, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };
const drive = async (label, events) => {
  const before = memoryBytes();
  takeRealloc();
  const at = Date.now();
  let result;
  try {
    result = await api.poll(events, null, null, budget);
  } catch (error) {
    const stats = takeRealloc();
    log(`${label}: TRAPPED after ${Date.now() - at}ms realloc calls=${stats.calls} bytes=${stats.bytes} largest=${stats.largest} memory ${megabytes(before)} → ${megabytes(memoryBytes())}`);
    log(`${label}: ${error?.stack ?? error}`);
    return null;
  }
  const stats = takeRealloc();
  log(`${label}: ${Date.now() - at}ms realloc calls=${stats.calls} bytes=${stats.bytes} largest=${stats.largest} memory ${megabytes(before)} → ${megabytes(memoryBytes())} (+${memoryBytes() - before}) effects=${result.effects?.length ?? "?"} patches=${result.uiPatches?.length ?? "?"}`);
  return result;
};

const openEvent = {
  kind: "instance-open",
  payload: {
    instance: 1,
    activationGeneration: 1n,
    requestSequence: 1,
    appId: process.env.SEMIO_REALLOC_APP ?? "s.procedural.generation3d@1/*#editor",
    actor: `${name}#1`,
    config: new Uint8Array(),
    assets: [],
    capabilities: [],
    quotas: new Uint8Array(),
  },
};

await drive("open", [openEvent]);
for (let turn = 1; turn <= turns; turn += 1) await drive(`idle #${turn}`, []);
const settled = memoryBytes();

const { guestAnswerPages, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES } = await import(new URL("file:///Users/ueli/Documents/semio/\u{1F9F0}\u{FE0F}framework/\u{1F528}\u{FE0F}modules/\u{23F1}\u{FE0F}trace/\u{1F9EE}\u{FE0F}memory/\u{1F7E6}\u{FE0F}.ts").href);
for (const size of [0, 1024, 65536, 262144, 1048576, 4194304, 16777216, 67108864, 134217728, 268435456, payloadBytes]) {
  const answer = new Uint8Array(size);
  const { prologue, terminal } = guestAnswerPages(answer);
  const events = [
    ...prologue.map((page) => ({ kind: "http-chunk", payload: { req: BigInt(1000 + size), params: { bytes: page, done: false } } })),
    { kind: "completed", payload: { req: BigInt(1000 + size), outcome: { tag: "ok", val: terminal } } },
  ];
  const delivered = await drive(`completed ${size}B in ${events.length} event(s), page bound ${GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES}`, events);
  if (delivered === null) break;
}
log(`settled=${megabytes(settled)} final=${megabytes(memoryBytes())} ceiling=${megabytes(536870912)} used=${((memoryBytes() / 536870912) * 100).toFixed(1)}%`);
