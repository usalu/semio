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
const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;

/** 🌐️ The languages of the process: the one the environment names, else every supported language in declared order (English first, German second); there is no default language. */
export function selectGraphFarmLanguagesV1(environment) {
  const named = P.languageEnvironment.map(name => environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value => P.languages.includes(value));
  return named === undefined ? [...P.languages] : [named];
}

/** 🗣️ Renders one farm message as one line per language of the environment. */
export function renderGraphFarmMessagesV1(key, fields, environment) {
  return selectGraphFarmLanguagesV1(environment).map(language => {
    const template = GRAPH_FARM_MESSAGES_V1[language][key];
    if (template === undefined) throw Error("Closed graph farm message required: " + key);
    return template.replace(/\{([a-z]+)\}/gu, (_token, name) => String(fields[name]));
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
    ["memory", Math.floor((budget - Math.max(parentMegabytes, policy.parentReserveMegabytes)) / workerMegabytes)],
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
 * 🏊️ A bounded pool of worker processes, each running one service module (`open(session)` once, then `run(job)` per job, one at a time). A worker that dies, fails a job or reports a different implementation never fails the caller: `infer` answers `undefined` and the caller infers that job itself.
 * A job may be a function of `{ slot, generation }` (the generation counts the processes the slot has had), evaluated when the job is sent, so a caller can add what that very process still lacks.
 * `accept(identity)` decides whether a worker that announced its identity may serve; a refusal disables the farm. A worker that grew past its memory budget is replaced after its job.
 */
export function createFarmV1({ entry, service, session, workers, environment = process.env, signal, policy = P, accept = () => true, report = () => {}, spawnProcess = spawn, runtime = graphFarmRuntimeV1(), command = process.execPath, commandArguments = [], workerMegabytes = policy.workerMegabytes }) {
  if (!Number.isSafeInteger(workers) || workers < 1 || workers > policy.maximumWorkers) throw Error("Graph farm of 1 to " + policy.maximumWorkers + " workers required");
  const slots = Array.from({ length: workers }, (_, index) => ({ index, generation: 0, child: undefined, state: "empty", job: undefined, ready: undefined, timer: undefined })), queue = [];
  const stats = { workers, jobs: 0, replies: 0, failures: 0, lost: 0, recycled: 0, started: 0, fallbacks: 0, peakRssMegabytes: 0 };
  let nextId = 0, closed = false, disabled, restarts = 0, sessionDirectory, sessionFile;
  const childEnvironment = { ...environment, ...graphFarmRuntimeEnvironmentV1(runtime, workerMegabytes, policy), SEMIO_GRAPH_WORKER: "1" };
  delete childEnvironment[policy.workersEnvironment];

  const settle = (job, value) => { if (job.settled) return; job.settled = true; job.detach?.(); clearTimeout(job.timer); job.resolve(value); };
  const retire = (slot, graceful) => {
    const child = slot.child;
    clearTimeout(slot.timer);
    slot.child = undefined; slot.state = "empty";
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
    if (job) { stats.fallbacks++; settle(job, undefined); }
    if (++restarts > policy.maximumRestarts) disable("restarts: " + why);
    pump();
  };
  const disable = why => {
    if (disabled !== undefined) return;
    disabled = why;
    report("disabled", { reason: why });
    for (const slot of slots) { const job = slot.job; slot.job = undefined; retire(slot, true); if (job) { stats.fallbacks++; settle(job, undefined); } }
    for (const job of queue.splice(0)) { stats.fallbacks++; settle(job, undefined); }
  };
  const launch = slot => {
    slot.state = "starting"; slot.generation++; stats.started++;
    if (sessionFile === undefined) { sessionDirectory = mkdtempSync(join(tmpdir(), "semio-farm-session-")); sessionFile = join(sessionDirectory, "session.bin"); writeFileSync(sessionFile, serialize(session)); }
    const child = spawnProcess(command, [...commandArguments, entry], { stdio: ["ignore", "inherit", "inherit", "ipc"], serialization: "advanced", env: childEnvironment });
    slot.child = child;
    slot.timer = setTimeout(() => lose(slot, "init timeout"), policy.initTimeoutMilliseconds);
    slot.timer.unref?.();
    child.on("message", message => {
      if (slot.child !== child) return;
      if (message?.type === "ready") {
        clearTimeout(slot.timer);
        if (!accept(message.identity)) { disable("implementation differs"); return; }
        slot.state = "idle"; pump();
      } else if (message?.type === "fatal") lose(slot, message.message);
      else if (message?.type === "result" || message?.type === "failure") {
        const job = slot.job;
        if (!job || job.id !== message.id) return;
        slot.job = undefined; slot.state = "idle";
        if (message.type === "result") { stats.replies++; stats.peakRssMegabytes = Math.max(stats.peakRssMegabytes, message.rssMegabytes); settle(job, { reply: message }); }
        else { stats.failures++; stats.fallbacks++; settle(job, { failure: message }); }
        if (message.type === "result" && message.rssMegabytes > workerMegabytes) { stats.recycled++; report("recycled", { worker: slot.index, megabytes: Math.round(message.rssMegabytes) }); retire(slot, true); }
        pump();
      }
    });
    child.once("exit", () => { if (slot.child === child) lose(slot, "exited"); });
    child.once("error", error => { if (slot.child === child) lose(slot, String(error?.message ?? error)); });
    try { child.send({ type: "init", service, sessionFile }); } catch (error) { lose(slot, String(error?.message ?? error)); }
  };
  const pump = () => {
    if (closed || disabled !== undefined) return;
    for (const slot of slots) {
      if (!queue.length) return;
      if (slot.state !== "idle") continue;
      const job = queue.shift();
      if (job.settled) continue;
      slot.state = "busy"; slot.job = job;
      job.timer = setTimeout(() => lose(slot, "job timeout"), policy.jobTimeoutMilliseconds);
      job.timer.unref?.();
      try { slot.child.send({ type: "run", id: job.id, job: typeof job.payload === "function" ? job.payload({ slot: slot.index, generation: slot.generation }) : job.payload }); } catch (error) { lose(slot, String(error?.message ?? error)); }
    }
    let waiting = queue.length - slots.filter(slot => slot.state === "starting").length;
    for (const slot of slots) if (waiting > 0 && slot.state === "empty") { launch(slot); waiting--; }
  };
  const close = async () => {
    if (closed) return;
    closed = true;
    for (const job of queue.splice(0)) settle(job, undefined);
    const exits = [];
    for (const slot of slots) {
      const child = slot.child, job = slot.job;
      slot.job = undefined;
      if (job) settle(job, undefined);
      if (child) exits.push(new Promise(done => { child.once("exit", done); setTimeout(done, policy.shutdownGraceMilliseconds + 500).unref?.(); }));
      retire(slot, true);
    }
    await Promise.all(exits);
    if (sessionDirectory !== undefined) rmSync(sessionDirectory, { recursive: true, force: true });
  };
  if (signal) { const abort = () => { void close(); }; if (signal.aborted) abort(); else signal.addEventListener("abort", abort, { once: true }); }
  return Object.freeze({
    width: workers,
    stats: () => ({ ...stats, restarts, ...(disabled === undefined ? {} : { disabled }) }),
    infer(payload, options = {}) {
      if (closed || disabled !== undefined) { stats.fallbacks++; return Promise.resolve(undefined); }
      options.signal?.throwIfAborted();
      stats.jobs++;
      return new Promise((resolve, reject) => {
        const job = { id: nextId++, payload, resolve, reject, settled: false, timer: undefined, detach: undefined };
        if (options.signal) { const abort = () => { const at = queue.indexOf(job); if (at >= 0) queue.splice(at, 1); job.settled = true; reject(options.signal.reason); }; options.signal.addEventListener("abort", abort, { once: true }); job.detach = () => options.signal.removeEventListener("abort", abort); }
        queue.push(job);
        pump();
      });
    },
    close,
  });
}
