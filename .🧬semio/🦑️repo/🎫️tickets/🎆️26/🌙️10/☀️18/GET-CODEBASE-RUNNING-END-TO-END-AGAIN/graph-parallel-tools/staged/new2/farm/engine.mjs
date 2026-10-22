import { spawn } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { serialize } from "node:v8";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🧊️ Freezes a declaration recursively. */
const freeze = value => { if (value !== null && typeof value === "object") { for (const child of Object.values(value)) freeze(child); Object.freeze(value); } return value; };

/** 🏊️ The declared limits of the graph inference farm: the schema file is the single source. */
export const GRAPH_FARM_POLICY_V1 = freeze(structuredClone(schema.$defs.Policy.const));
/** 🗣️ The bilingual texts of the farm, read from the schema file. */
export const GRAPH_FARM_MESSAGES_V1 = freeze(structuredClone(schema.$defs.Messages.const));
/** 🗣️ The bilingual reasons a job comes back without an answer; the code of a reason is its text before the first colon. */
export const GRAPH_FARM_REASONS_V1 = freeze(structuredClone(schema.$defs.Reasons2.const));
const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;

/** 🌐️ The languages of the process: the one the environment names, else every supported language in declared order (English first, German second); there is no default language. */
export function selectGraphFarmLanguagesV1(environment) {
  const named = P.languageEnvironment.map(name => environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value => P.languages.includes(value));
  return named === undefined ? [...P.languages] : [named];
}

/** 🗣️ Names a reason in one language: a known code is translated and keeps its detail after the colon, any other text stays as it is. */
function reasonTextV1(language, reason) {
  const colon = reason.indexOf(":"), code = colon < 0 ? reason : reason.slice(0, colon), text = GRAPH_FARM_REASONS_V1[language][code];
  return text === undefined ? reason : colon < 0 ? text : text + reason.slice(colon);
}

/** 🗣️ Renders one farm message as one line per language of the environment. */
export function renderGraphFarmMessagesV1(key, fields, environment) {
  return selectGraphFarmLanguagesV1(environment).map(language => {
    const template = GRAPH_FARM_MESSAGES_V1[language][key];
    if (template === undefined) throw Error("Closed graph farm message required: " + key);
    return template.replace(/\{([a-z]+)\}/gu, (_token, name) => name === "reason" ? reasonTextV1(language, String(fields[name])) : String(fields[name]));
  });
}

/** 📐️ Sizes the farm: the explicit environment value wins, otherwise the smallest of the processors left to the workers, the memory the host and the parent leave them, and the work there is to share; fewer than two workers is no farm. */
export function planGraphFarmV1({ projects, environment = {}, host, parentMegabytes = 0, policy = P }) {
  const named = environment[policy.workerMegabytesEnvironment]?.trim();
  if (named !== undefined && named !== "" && (!/^[0-9]+$/u.test(named) || Number(named) < policy.minimumWorkerMegabytes)) throw Error("Worker memory of at least " + policy.minimumWorkerMegabytes + " MB required: " + policy.workerMegabytesEnvironment);
  const workerMegabytes = named === undefined || named === "" ? policy.workerMegabytes : Number(named), base = { workerMegabytes }, text = environment[policy.workersEnvironment]?.trim().toLowerCase();
  if (text === "0" || text === "off" || text === "false") return { workers: 0, reason: "disabled", ...base };
  if (text !== undefined && text !== "" && text !== "auto") {
    const forced = Number(text);
    if (!Number.isSafeInteger(forced) || forced < 0 || forced > policy.maximumWorkers) throw Error("Graph workers of 0 to " + policy.maximumWorkers + " required: " + policy.workersEnvironment);
    return { workers: forced, reason: forced === 0 ? "disabled" : "forced", ...base };
  }
  if (projects < policy.minimumProjects) return { workers: 0, reason: "small", ...base };
  const budget = Math.min(host.totalMegabytes * policy.hostShareBasisPoints / 10000, host.availableMegabytes ?? Infinity);
  const bounds = [
    ["work", Math.floor(projects / policy.projectsPerWorker)],
    ["memory", Math.floor((budget - Math.max(parentMegabytes, policy.parentReserveMegabytes) - policy.recordTableMegabytes - policy.seedMegabytes) / workerMegabytes)],
    ["cpu", host.cores - policy.cpuReserve],
    ["limit", policy.maximumWorkers],
  ];
  const [reason, workers] = bounds.reduce((best, row) => row[1] < best[1] ? row : best);
  return workers < 2 ? { workers: 0, reason: "single", ...base } : { workers, reason, ...base };
}

/** 🧬️ The environment that bounds the heap of one worker process of the runtime in use; a runtime without such a switch gets none. */
export function graphFarmRuntimeEnvironmentV1(runtime, workerMegabytes, policy = P) {
  const row = policy.runtime[runtime];
  if (row === undefined) return {};
  const environment = { ...row.environment };
  if (runtime === "bun") environment.BUN_JSC_forceRAMSize = String(Math.floor(workerMegabytes * row.ramFactor) * MEGABYTE);
  else if (runtime === "node") environment.NODE_OPTIONS = "--max-old-space-size=" + Math.floor(workerMegabytes * row.ramFactor);
  return environment;
}

/** 📤️ Sends each distinct item once: an item the sender has already sent travels as its identifier, a new one as an identifier and the item. */
export function createInternEncoderV1({ keyOf } = {}) {
  const byObject = new WeakMap(), byKey = new Map();
  let next = 1;
  return {
    encode(items) {
      const ids = new Uint32Array(items.length), fresh = [];
      for (let index = 0; index < items.length; index++) {
        const item = items[index], object = item !== null && typeof item === "object";
        let id = object ? byObject.get(item) : undefined;
        if (id === undefined) {
          const key = object ? keyOf?.(item) : item;
          id = key === undefined ? undefined : byKey.get(key);
          if (id === undefined) { id = next++; fresh.push([id, item]); if (key !== undefined) byKey.set(key, id); }
          if (object) byObject.set(item, id);
        }
        ids[index] = id;
      }
      return { ids, fresh };
    },
    get size() { return next - 1; },
  };
}

/** 📥️ Receives what an encoder sent: new items join the table through `canonical`, so equal items of different senders become one object. */
export function createInternDecoderV1({ canonical = item => item, maximumEntries = P.maximumTableEntries } = {}) {
  const table = [];
  return {
    learn(fresh) {
      for (const [id, item] of fresh) {
        if (!Number.isSafeInteger(id) || id < 1 || id > maximumEntries) throw Error("Bounded intern identifier required");
        table[id] = canonical(item);
      }
    },
    decode(ids) {
      const items = new Array(ids.length);
      for (let index = 0; index < ids.length; index++) {
        const item = table[ids[index]];
        if (item === undefined) throw Error("Known intern identifier required: " + ids[index]);
        items[index] = item;
      }
      return items;
    },
    get size() { return table.length; },
  };
}

/** 🚦️ Names the runtime that runs this process, so its heap switches can be chosen without naming a tool in the callers. */
export function graphFarmRuntimeV1() { return typeof Bun === "undefined" ? "node" : "bun"; }

/**
 * 🏊️ A bounded pool of worker processes. Each worker loads the service modules once (`load`), opens one session per service kind its parent describes (`open(session)`, any time and again for the next build) and runs jobs of an opened kind one at a time (`run(job)`). A worker that dies, fails a job or reports a different implementation never fails the caller: `infer` answers `undefined` and the caller infers that job itself.
 * `prestart()` starts and loads every worker at once, so the processes boot while the caller still prepares the session. A job may be a function of `{ slot, generation }` (the generation counts the processes the slot has had), evaluated when the job is sent, so a caller can add what that very process still lacks.
 * `accept(kind, identity)` decides whether a worker that opened a session may serve it; a refusal disables the farm. A worker that grew past its memory budget is replaced after its job. `limit(count)` shrinks the pool to the workers the host grants.
 */
export function createFarmV1({ entry, services, workers, environment = process.env, signal, policy = P, accept = () => true, report = () => {}, spawnProcess = spawn, runtime = graphFarmRuntimeV1(), command = process.execPath, commandArguments = [], workerMegabytes = policy.workerMegabytes }) {
  if (!Number.isSafeInteger(workers) || workers < 1 || workers > policy.maximumWorkers) throw Error("Graph farm of 1 to " + policy.maximumWorkers + " workers required");
  if (services === null || typeof services !== "object") throw Error("Graph farm services required");
  services = { ...services };
  const slots = Array.from({ length: workers }, (_, index) => ({ index, generation: 0, child: undefined, state: "empty", job: undefined, timer: undefined, opened: new Map(), opening: undefined })), queue = [], sessions = new Map();
  const stats = { workers, jobs: 0, replies: 0, failures: 0, lost: 0, recycled: 0, started: 0, fallbacks: 0, peakRssMegabytes: 0, opens: 0 }, created = performance.now();
  let nextId = 0, closed = false, disabled, restarts = 0, sessionDirectory, active = workers;
  const childEnvironment = { ...environment, ...graphFarmRuntimeEnvironmentV1(runtime, workerMegabytes, policy), SEMIO_GRAPH_WORKER: "1" };
  delete childEnvironment[policy.workersEnvironment];

  const settle = (job, value) => { if (job.settled) return; job.settled = true; job.detach?.(); clearTimeout(job.timer); job.resolve(value); };
  const retire = (slot, graceful) => {
    const child = slot.child;
    clearTimeout(slot.timer);
    slot.child = undefined; slot.state = "empty"; slot.opened = new Map(); slot.opening = undefined;
    if (!child) return;
    child.removeAllListeners("message");
    if (graceful && child.connected) { try { child.send({ type: "shutdown" }); } catch {} setTimeout(() => { try { child.kill("SIGKILL"); } catch {} }, policy.shutdownGraceMilliseconds).unref?.(); }
    else { try { child.kill("SIGKILL"); } catch {} }
  };
  const lose = (slot, why) => {
    const job = slot.job;
    slot.job = undefined;
    retire(slot, false);
    stats.lost++;
    if (job) { stats.fallbacks++; settle(job, { fallback: why }); }
    if (++restarts > policy.maximumRestarts) disable("restarts: " + why);
    pump();
  };
  const disable = why => {
    if (disabled !== undefined) return;
    disabled = why;
    report("disabled", { reason: why });
    for (const slot of slots) { const job = slot.job; slot.job = undefined; retire(slot, true); if (job) { stats.fallbacks++; settle(job, { fallback: "disabled: " + why }); } }
    for (const job of queue.splice(0)) { stats.fallbacks++; settle(job, { fallback: "disabled: " + why }); }
  };
  const launch = slot => {
    slot.state = "loading"; slot.generation++; stats.started++;
    const child = spawnProcess(command, [...commandArguments, entry], { stdio: ["ignore", "inherit", "inherit", "ipc"], serialization: "advanced", env: childEnvironment });
    slot.child = child;
    slot.timer = setTimeout(() => lose(slot, "init timeout"), policy.initTimeoutMilliseconds);
    slot.timer.unref?.();
    child.on("message", message => {
      if (slot.child !== child) return;
      if (message?.type === "loaded") {
        clearTimeout(slot.timer);
        stats.loadedAfterMilliseconds ??= Math.round(performance.now() - created);
        slot.state = "loaded";
        pump();
      } else if (message?.type === "opened") {
        clearTimeout(slot.timer);
        const session = sessions.get(message.kind);
        slot.opening = undefined;
        if (session === undefined || message.token !== session.token) { slot.state = "loaded"; pump(); return; }
        if (!(accepts.get(message.kind) ?? accept)(message.kind, message.identity)) { disable("implementation differs"); return; }
        slot.opened.set(message.kind, message.token); slot.state = "loaded"; stats.opens++;
        pump();
      } else if (message?.type === "fatal") lose(slot, message.message);
      else if (message?.type === "result" || message?.type === "failure") {
        const job = slot.job;
        if (!job || job.id !== message.id) return;
        slot.job = undefined; slot.state = "loaded";
        if (message.type === "result") { stats.replies++; stats.peakRssMegabytes = Math.max(stats.peakRssMegabytes, message.rssMegabytes); settle(job, { reply: message }); }
        else { stats.failures++; stats.fallbacks++; settle(job, { failure: message }); }
        if (message.type === "result" && message.rssMegabytes > workerMegabytes) { stats.recycled++; report("recycled", { worker: slot.index, megabytes: Math.round(message.rssMegabytes) }); retire(slot, true); }
        pump();
      }
    });
    child.once("exit", () => { if (slot.child === child) lose(slot, "exited"); });
    child.once("error", error => { if (slot.child === child) lose(slot, String(error?.message ?? error)); });
    try { child.send({ type: "load", services }); } catch (error) { lose(slot, String(error?.message ?? error)); }
  };
  const accepts = new Map();
  const sendOpen = (slot, kind) => {
    const session = sessions.get(kind);
    slot.state = "opening"; slot.opening = kind;
    slot.timer = setTimeout(() => lose(slot, "open timeout"), policy.initTimeoutMilliseconds);
    slot.timer.unref?.();
    try { slot.child.send({ type: "open", kind, service: services[kind], token: session.token, sessionFile: session.file }); } catch (error) { lose(slot, String(error?.message ?? error)); }
  };
  const ready = (slot, kind) => slot.opened.get(kind) === sessions.get(kind)?.token;
  const pump = () => {
    if (closed || disabled !== undefined) return;
    for (const slot of slots) {
      if (slot.index >= active || slot.state !== "loaded") continue;
      const wanted = queue.find(job => !job.settled && sessions.has(job.kind));
      if (!wanted) break;
      const job = queue.find(item => !item.settled && sessions.has(item.kind) && ready(slot, item.kind));
      if (!job) { sendOpen(slot, wanted.kind); continue; }
      queue.splice(queue.indexOf(job), 1);
      stats.firstJobAfterMilliseconds ??= Math.round(performance.now() - created);
      slot.state = "busy"; slot.job = job;
      job.timer = setTimeout(() => lose(slot, "job timeout"), policy.jobTimeoutMilliseconds);
      job.timer.unref?.();
      try { slot.child.send({ type: "run", id: job.id, kind: job.kind, job: typeof job.payload === "function" ? job.payload({ slot: slot.index, generation: slot.generation }) : job.payload }); } catch (error) { lose(slot, String(error?.message ?? error)); }
    }
    for (let at = queue.length - 1; at >= 0; at--) if (queue[at].settled) queue.splice(at, 1);
    let waiting = queue.filter(job => sessions.has(job.kind)).length - slots.filter(slot => slot.index < active && (slot.state === "loading" || slot.state === "opening")).length;
    for (const slot of slots) if (waiting > 0 && slot.index < active && slot.state === "empty") { launch(slot); waiting--; }
  };
  const close = async () => {
    if (closed) return;
    closed = true;
    for (const job of queue.splice(0)) settle(job, { fallback: "closed" });
    const exits = [];
    for (const slot of slots) {
      const child = slot.child, job = slot.job;
      slot.job = undefined;
      if (job) settle(job, { fallback: "closed" });
      if (child) exits.push(new Promise(done => { child.once("exit", done); setTimeout(done, policy.shutdownGraceMilliseconds + 500).unref?.(); }));
      retire(slot, true);
    }
    await Promise.all(exits);
    if (sessionDirectory !== undefined) rmSync(sessionDirectory, { recursive: true, force: true });
  };
  if (signal) { const abort = () => { void close(); }; if (signal.aborted) abort(); else signal.addEventListener("abort", abort, { once: true }); }
  return Object.freeze({
    width: workers,
    get closed() { return closed || disabled !== undefined; },
    stats: () => ({ ...stats, restarts, ...(disabled === undefined ? {} : { disabled }) }),
    prestart() {
      if (closed || disabled !== undefined) return;
      for (const slot of slots) if (slot.index < active && slot.state === "empty") launch(slot);
    },
    limit(count) {
      if (!Number.isSafeInteger(count) || count < 1) throw Error("Positive worker limit required");
      active = Math.min(count, workers);
      for (const slot of slots) if (slot.index >= active && slot.state !== "busy" && slot.child) retire(slot, true);
    },
    open(kind, session, { accept: acceptKind, service } = {}) {
      if (closed || disabled !== undefined) return false;
      if (service !== undefined) services[kind] ??= service;
      if (services[kind] === undefined) throw Error("Declared graph farm service required: " + kind);
      sessionDirectory ??= mkdtempSync(join(tmpdir(), "semio-farm-session-"));
      const token = (sessions.get(kind)?.token ?? 0) + 1, file = join(sessionDirectory, kind + "-" + token + ".bin");
      writeFileSync(file, serialize(session));
      sessions.set(kind, { token, file });
      if (acceptKind) accepts.set(kind, acceptKind);
      pump();
      return true;
    },
    infer(kind, payload, options = {}) {
      if (closed || disabled !== undefined) { stats.fallbacks++; return Promise.resolve({ fallback: disabled === undefined ? "closed" : "disabled: " + disabled }); }
      options.signal?.throwIfAborted();
      stats.jobs++;
      return new Promise((resolve, reject) => {
        const job = { id: nextId++, kind, payload, resolve, reject, settled: false, timer: undefined, detach: undefined };
        if (options.signal) { const abort = () => { const at = queue.indexOf(job); if (at >= 0) queue.splice(at, 1); job.settled = true; reject(options.signal.reason); }; options.signal.addEventListener("abort", abort, { once: true }); job.detach = () => options.signal.removeEventListener("abort", abort); }
        queue.push(job);
        pump();
      });
    },
    close,
  });
}

const pools = new Map();

/** 🏊️ Leases the resident pool of one key (workspace, implementation, worker memory): the first lease creates it, later leases of any graph phase or any later build of the same process share its already booted workers, and the last release keeps the processes for `residentIdleMilliseconds` before they are closed. A pool that was closed or disabled is replaced. */
export function acquireGraphFarmV1({ key, policy = P, ...options }) {
  let held = pools.get(key);
  if (held === undefined || held.farm.closed) { held = { farm: createFarmV1({ policy, ...options }), leases: 0, timer: undefined }; pools.set(key, held); }
  clearTimeout(held.timer);
  held.leases++;
  let released = false;
  return Object.freeze({
    farm: held.farm,
    release() {
      if (released) return;
      released = true;
      if (--held.leases > 0) return;
      held.timer = setTimeout(() => { if (pools.get(key) === held && held.leases === 0) { pools.delete(key); void held.farm.close(); } }, policy.residentIdleMilliseconds);
      held.timer.unref?.();
    },
  });
}

/** 🧹️ Closes every resident pool now: the hook a resident repo service calls when it drops its caches or stops. */
export async function closeGraphFarmsV1() {
  const held = [...pools.values()];
  pools.clear();
  for (const item of held) clearTimeout(item.timer);
  await Promise.all(held.map(item => item.farm.close()));
}
