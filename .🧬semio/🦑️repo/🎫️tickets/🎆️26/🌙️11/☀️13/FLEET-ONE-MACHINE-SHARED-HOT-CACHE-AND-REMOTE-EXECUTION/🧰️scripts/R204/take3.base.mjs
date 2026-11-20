import { execFile } from "node:child_process";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { localMachineSpeedFactorV1, scaledMillisecondsV1 } from "../../../../🐢️speed/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export const TAKE_POLICY_V1 = Object.freeze({ ...schema.definitions.Policy.const });
const TEXTS = schema.definitions.Texts.const;

/** ⏱️ The pause before look number `round` (0 is the first) at the cache for records that other machines may have stored meanwhile: it starts at `pollFirstMilliseconds` and grows by `pollGrowthPercent` percent per look up to `pollMaximumMilliseconds`, in integer arithmetic, so every language computes the same numbers.
 * @see ./🧪️tests/🐍️.py */
export function takePollWaitV1(round, policy = TAKE_POLICY_V1) {
  let wait = policy.pollFirstMilliseconds;
  for (let step = 0; step < round && wait < policy.pollMaximumMilliseconds; step++) wait = Math.min(policy.pollMaximumMilliseconds, Math.floor(wait * policy.pollGrowthPercent / 100));
  return Math.min(wait, policy.pollMaximumMilliseconds);
}

/** 🔢 The looks a lane runs at once: the declared `concurrency`, never more than the projects the cut of this process allows at once (a look walks the script closure of its owner, about as much memory as an inference, so 24 uncut looks held 3 GB of live heap). Infinity where the process is not cut.
 * @see ./🧪️tests/🟦️.ts */
export function takeConcurrencyV1(cutConcurrency = Infinity, policy = TAKE_POLICY_V1) {
  return Math.max(1, Math.min(policy.concurrency, cutConcurrency));
}

/** 🚦 A look admitted like an inference: before it starts, the footprint gate of the process (`gate()`, none installed: no wait) settles from its soft share of the grant and holds from its hard share until the footprint falls, so looks never pile up above the memory a plan may take, and a look that finds no other look in flight (`state.flying`, shared by every lane of the pass) is always admitted, so the pass can never starve behind a host that is short of memory (they had no admission at all: a burst of Rust owner looks moved the tree from 0.6 to 4.7 GB in seconds).
 * @see ./🧪️tests/🟦️.ts */
export function takeAdmittedV1(attempt, gate, signal, state = { flying: 0 }) {
  return async name => {
    if (state.flying > 0) await gate()?.admit(signal);
    state.flying++;
    try { return await attempt(name); } finally { state.flying--; }
  };
}

/** 🛤️ The lanes of a take pass: the owners the pools of the fleet say they stored and that are not stored here (`hot`, first: their look finds a record), the owners whose record is stored here (`local`, decided by the CPU of this process) and the others (`remote`, decided by the fleet cache and the network), which run beside each other so that an owner the fleet holds does not wait behind thousands of local looks. Every name is in exactly one lane, in its order; an empty lane is left out.
 * @see ./🧪️tests/🔥️hot-lane */
export function takeLanesV1(names, isPresent, isPublished = () => false) {
  const hot = [], local = [], remote = [];
  for (const name of names) (isPresent(name) ? local : isPublished(name) ? hot : remote).push(name);
  return [hot, local, remote].filter(lane => lane.length > 0);
}

/** 🐘️ The names of a take pass in two groups: the light ones, which run together in the lanes, and the heavy ones (the owners the cost table calls `alone` or `slow`), which are looked at one at a time after every light one has been looked at, each in its order. Every name is in exactly one group.
 * @see ./🧪️tests/🟦️.ts */
export function takeSplitV1(names, isHeavy) {
  const light = [], heavy = [];
  for (const name of names) (isHeavy(name) ? heavy : light).push(name);
  return Object.freeze({ light, heavy });
}

/** 💪️ Whether a machine is large enough never to wait for another: its memory and its cores both reach the declared floor (`strongMemoryMegabytes`, `strongCores`), so the fleet never makes the biggest machine wait for the smallest ones.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function takeStrongMachineV1({ memoryMegabytes, cores }, policy = TAKE_POLICY_V1) {
  return memoryMegabytes >= policy.strongMemoryMegabytes && cores >= policy.strongCores;
}

/** 🚦️ A watch over the progress of a read: `check(sign)` is told a value that changes whenever anything moves (owners done, looks, rows), and answers how long nothing moved once that reached `windowMilliseconds`, else undefined.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function createProgressWatchV1({ now = () => performance.now(), windowMilliseconds = scaledMillisecondsV1(TAKE_POLICY_V1.noProgressMilliseconds, localMachineSpeedFactorV1()) } = {}) {
  let last, since = now();
  return Object.freeze({
    check(sign) {
      const at = now();
      if (sign !== last) { last = sign; since = at; return undefined; }
      return at - since >= windowMilliseconds ? Object.freeze({ sinceMilliseconds: at - since }) : undefined;
    },
  });
}

/** 🔑️ The facts that count as progress of a read: owners finished, taken, started or parked, rows written, owners done in the ledger, records adopted from other machines, and the processor time of the children while owners run. How often the cache was asked is not among them: a look that found nothing moves no owner. */
export const TAKE_PROGRESS_KEYS_V1 = Object.freeze(["finished", "taken", "cursor", "projectIndex", "deferred", "rows", "done", "producers", "cpu"]);

/** 🧮️ The sign that the progress watch compares: the declared progress facts of `row` in order, every other field ignored (the number of lookups grows by the poll and by the wait for the fleet even while nothing arrives, so it must never reach the sign).
 * @see ./🧪️tests/🪧️tail-report/🟦️.ts */
export function takeProgressSignV1(row) { return TAKE_PROGRESS_KEYS_V1.map(key => row[key] ?? 0).join("|"); }

/** 🪧️ A reporter that speaks at most once per window while the sign stays the same: `check(sign)` answers how long nothing moved the first time that reached `windowMilliseconds` and again after every further window, and starts over at every change of the sign.
 * @see ./🧪️tests/🪧️tail-report/🟦️.ts */
export function createTailReporterV1({ now = () => performance.now(), windowMilliseconds = scaledMillisecondsV1(TAKE_POLICY_V1.tailReportMilliseconds, localMachineSpeedFactorV1()) } = {}) {
  let last, since = now(), told = since;
  return Object.freeze({
    check(sign) {
      const at = now();
      if (sign !== last) { last = sign; since = at; told = at; return undefined; }
      if (at - told < windowMilliseconds || at - since < windowMilliseconds) return undefined;
      told = at;
      return Object.freeze({ sinceMilliseconds: at - since });
    },
  });
}

/** 📋️ The reason of every missing owner, one each and in the order of `names`: `heavy` (the cost table says one owner of this machine can never hold it: only a larger machine of the fleet infers it), `running` (a loop of this read has it, with the seconds), `parked` (set aside until memory is free) or `fleet` (waits for a record another machine stores).
 * @see ./🧪️tests/🪧️tail-report/🟦️.ts */
export function takeMissingRowsV1(names, { never, running, deferred, megabytes, limitMegabytes }) {
  return names.map(name => {
    if (never.has(name)) return Object.freeze({ name, reason: "heavy", megabytes: megabytes?.(name), limitMegabytes });
    if (running.has(name)) return Object.freeze({ name, reason: "running", seconds: running.get(name) });
    if (deferred.has(name)) return Object.freeze({ name, reason: "parked" });
    return Object.freeze({ name, reason: "fleet" });
  });
}

/** ⚖️ What a stall is: `heavy` when something is missing and every missing owner is too large for this machine (the read can only wait for a larger one, so it fails with the typed heavy refusal), else `stalled`.
 * @see ./🧪️tests/🪧️tail-report/🟦️.ts */
export function takeStallKindV1(rows) { return rows.length > 0 && rows.every(row => row.reason === "heavy") ? "heavy" : "stalled"; }

/** 🗣️ The plain text of a read whose owner count did not advance, in one language: the minutes, the owners done, and every group of missing owners with its reason, its numbers and at most `limit` names.
 * @see ./🧪️tests/🪧️tail-report/🟦️.ts */
export function takeTailTextV1(language, { minutes, done, total, rows, pool }, limit = 4) {
  const texts = TEXTS[language === "de" ? "de" : "en"], parts = [];
  for (const [reason, key] of [["heavy", "tailHeavy"], ["running", "tailRunning"], ["parked", "tailParked"], ["fleet", "tailFleet"]]) {
    const group = rows.filter(row => row.reason === reason);
    if (group.length === 0) continue;
    const names = group.slice(0, limit).map(row => row.name + (reason === "running" ? " " + Math.round(row.seconds ?? 0) + " s" : "")).join(", ") + (group.length > limit ? ", " + fill(texts.tailMore, { count: group.length - limit }) : "");
    parts.push(fill(texts[key], { count: group.length, names, megabytes: Math.max(0, ...group.map(row => row.megabytes ?? 0)), limit: group[0].limitMegabytes ?? 0 }));
  }
  const own = pool === undefined ? "" : " " + fill(texts.tailPool, { state: pool.state, children: pool.children, running: pool.running, rate: pool.ownersPerMinute, reason: pool.text?.[language === "de" ? "de" : "en"] ?? "" }).trim();
  return fill(texts.tail, { minutes, done, total, count: rows.length, groups: parts.join("; ") }) + own;
}

/** ⏱️ The CPU time in milliseconds of a `ps` time column (`[dd-][hh:]mm:ss[.cc]`), 0 where it cannot be read. */
export function takeCpuMillisecondsV1(text) {
  const [days, clock] = String(text).includes("-") ? String(text).split("-") : ["0", String(text)];
  const seconds = clock.split(":").reduce((sum, part) => sum * 60 + Number(part), 0);
  return Number.isFinite(seconds) && Number.isFinite(Number(days)) ? Math.round((Number(days) * 86400 + seconds) * 1000) : 0;
}

/** 🌳️ The CPU milliseconds of every process below `root` in the table `rows` (`{pid, parent, milliseconds}`), the root itself left out. */
export function takeTreeCpuMillisecondsV1(rows, root) {
  const children = new Map();
  for (const row of rows) children.set(row.parent, [...children.get(row.parent) ?? [], row]);
  let sum = 0;
  const seen = new Set([root]), queue = [root];
  while (queue.length > 0) for (const row of children.get(queue.shift()) ?? []) if (!seen.has(row.pid)) { seen.add(row.pid); sum += row.milliseconds; queue.push(row.pid); }
  return sum;
}

/** 📸️ The CPU milliseconds that the child processes of this process (the owner children of the farm and of the pool) have used so far, read from the process table of the operating system (`ps` on macOS and Linux, CIM on Windows); undefined where it cannot be read.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function takeChildrenCpuV1(root = process.pid, run = execFile) {
  const windows = process.platform === "win32";
  const [command, args] = windows ? ["powershell", ["-NoProfile", "-Command", "Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId) $($_.ParentProcessId) $([math]::Round(($_.UserModeTime + $_.KernelModeTime) / 10000))\" }"]] : ["ps", ["-A", "-o", "pid=,ppid=,time="]];
  return new Promise(resolve => {
    try {
      run(command, args, { timeout: 5000, windowsHide: true }, (error, out) => {
        if (error) return resolve(undefined);
        const rows = String(out).split(/\r?\n/u).map(line => line.trim().split(/\s+/u)).filter(parts => parts.length >= 3).map(([pid, parent, time]) => ({ pid: Number(pid), parent: Number(parent), milliseconds: windows ? Number(time) : takeCpuMillisecondsV1(time) }));
        resolve(takeTreeCpuMillisecondsV1(rows, root));
      });
    } catch { resolve(undefined); }
  });
}

/** 🔥️ A sign of CPU progress for the watch: `tick()` reads the CPU of the children and answers a count that grows by one whenever they used at least `cpuProgressPercent` percent of one core since the last tick, so owners that run and compute are progress and owners that hang or wait for nothing are not (an unreadable sample counts as no progress).
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function createCpuProgressV1({ sample, now = () => performance.now(), policy = TAKE_POLICY_V1 }) {
  let previous, at = now(), count = 0;
  return Object.freeze({
    async tick() {
      const used = await sample(), time = now();
      if (used !== undefined && previous !== undefined && time > at && (used - previous) * 100 >= (time - at) * policy.cpuProgressPercent) count++;
      if (used !== undefined) previous = used;
      at = time;
      return count;
    },
  });
}

/** 🗣️ The plain text of a read that stopped for lack of progress, in one language: how long nothing moved, how many owners are done, which run and what the read waits on. */
export function takeNoProgressTextV1(language, fields) { return fill(TEXTS[language === "de" ? "de" : "en"].noProgress, fields); }

/** 🔒️ The outcomes that decide an owner: once an owner is present, fetched or inferred a later look that finds it stale or missing (the look of the take pass runs beside the inference of the same owner) never takes it back, so a ledger never shows an owner as stale that this read already has. */
const DECIDED_V1 = Object.freeze(["present", "fetched", "inferred"]);

/** 📒️ What a reader knows about its owners while it resolves them: the outcome of every look, which owners it published as wanted for the fleet and how many run here; the counts it reports and the line in both languages. An owner counts once, by its last outcome. */
export function createTakeLedgerV1(total) {
  const states = new Map(), wanted = new Set();
  let running = 0;
  const count = outcome => { let sum = 0; for (const state of states.values()) if (state === outcome) sum++; return sum; };
  const counts = () => {
    const present = count("present"), fetched = count("fetched"), here = count("inferred"), stale = count("stale");
    return Object.freeze({ total, present, fetched, here, stale, running, wanted: wanted.size, done: present + fetched + here, missing: total - present - fetched - here });
  };
  return Object.freeze({
    mark(name, outcome) { if (!DECIDED_V1.includes(outcome) && DECIDED_V1.includes(states.get(name))) return; states.set(name, outcome); if (outcome === "present" || outcome === "fetched" || outcome === "inferred") wanted.delete(name); },
    want(names) { for (const name of names) if (!["present", "fetched", "inferred"].includes(states.get(name))) wanted.add(name); },
    run(delta) { running = Math.max(0, running + delta); },
    state(name) { return states.get(name); },
    counts,
    line(language) { return fill(TEXTS[language === "de" ? "de" : "en"].progress, counts()); },
    staleRowLine(language, name, row) { return fill(TEXTS[language === "de" ? "de" : "en"].staleRow, { name, kind: row.kind, path: row.path, fact: row.fact }); },
    staleLine(language, name) { return fill(TEXTS[language === "de" ? "de" : "en"].staleOwner, { name, count: counts().stale }); },
    waveLine(language) { const row = counts(); return fill(TEXTS[language === "de" ? "de" : "en"].wave, row); },
  });
}

const fill = (text, values) => text.replace(/\{(\w+)\}/gu, (_all, name) => String(values[name] ?? 0));

/** 🔍️ Looks at every name with `concurrency` workers, once, and again (up to `faultRetries` times, after `faultWaitMilliseconds`) those whose look the cache did not answer (`fault`); answers the outcome of every name. `attempt(name)` answers "present", "fetched", "stale", "miss" or "fault"; a fault that remains is a "miss" (the caller infers or waits for it, the fleet decides nothing). Never throws for an attempt that throws: that name is a miss. `signal` stops the pass early (names not looked at are absent from the answer). */
export async function takePassV1({ names, attempt, concurrency = TAKE_POLICY_V1.concurrency, faultRetries = TAKE_POLICY_V1.faultRetries, faultWaitMilliseconds = TAKE_POLICY_V1.faultWaitMilliseconds, sleep = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds)), signal, mark = () => {} }) {
  const outcomes = new Map();
  let pending = [...names];
  for (let round = 0; round <= faultRetries && pending.length > 0 && signal?.aborted !== true; round++) {
    if (round > 0) await sleep(faultWaitMilliseconds * 2 ** (round - 1));
    const faulted = [];
    let next = 0;
    await Promise.all(Array.from({ length: Math.min(concurrency, pending.length) }, async () => {
      while (next < pending.length && signal?.aborted !== true) {
        const name = pending[next++];
        let outcome;
        try { outcome = await attempt(name); } catch { outcome = "miss"; }
        if (outcome === "fault" && round < faultRetries) { faulted.push(name); continue; }
        outcomes.set(name, outcome === "fault" ? "miss" : outcome);
        mark(name, outcomes.get(name));
      }
    }));
    pending = faulted;
  }
  return outcomes;
}

/** 📣️ The names of the owners that the pools of the fleet say they stored: every pool child appends the name to `pool-published-<wave>.txt` below its origin in the shared store, so a reader finds them with one directory read per origin and looks at those first instead of rotating through thousands of owners. Files older than `publishedMaximumAgeMilliseconds` and everything unreadable count for nothing.
 * @see ./🧪️tests/📣️published/🟦️.ts */
export function fleetPublishedNamesV1(store, now = Date.now(), policy = TAKE_POLICY_V1) {
  const names = new Set();
  if (typeof store !== "string" || store === "") return names;
  let origins = [];
  try { origins = readdirSync(join(store, "v1", "origins")); } catch { return names; }
  for (const origin of origins) {
    const directory = join(store, "v1", "origins", origin);
    let files = [];
    try { files = readdirSync(directory).filter(file => /^pool-published-[0-9a-f]+\.txt$/u.test(file)); } catch { continue; }
    for (const file of files) { try { if (now - statSync(join(directory, file)).mtimeMs > policy.publishedMaximumAgeMilliseconds) continue; for (const line of readFileSync(join(directory, file), "utf8").split("\n")) if (line !== "") names.add(line); } catch {} }
  }
  return names;
}

/** 🔁️ Looks again at the cache for the owners that are still missing, with a growing pause: `queue()` names the owners not started yet (a batch of at most `pollBatch` of them is looked at, rotating, so every owner gets its turn), `attempt(name)` answers like in {@link takePassV1}; a record that appeared is reported to `arrived(name, outcome)` and resets the pause. With `hotOnly` only the owners the fleet lists are looked at, every `pollFirstMilliseconds`, and nothing is rotated through. Stops when `stop()` is true or `signal` aborts; answers how many looks it made and how many records arrived. */
export async function takePollV1({ queue, attempt, arrived, stop, signal, hot, hotOnly = false, now = Date.now, concurrency, sleep = (milliseconds, abort) => new Promise(resolve => { const timer = setTimeout(resolve, milliseconds); abort?.addEventListener("abort", () => { clearTimeout(timer); resolve(); }, { once: true }); }), policy = TAKE_POLICY_V1 }) {
  let round = 0, offset = 0, looks = 0, found = 0;
  const missedAt = new Map();
  while (stop() !== true && signal?.aborted !== true) {
    const listed = () => { const published = hot?.(); return published === undefined || published.size === 0 ? [] : queue().filter(name => published.has(name) && now() - (missedAt.get(name) ?? -Infinity) >= policy.hotRetryMilliseconds); };
    await sleep(hotOnly || listed().length > 0 ? policy.pollFirstMilliseconds : takePollWaitV1(round, policy), signal);
    if (stop() === true || signal?.aborted === true) break;
    const names = queue(), first = listed(), batch = first.slice(0, policy.pollBatch);
    if (names.length === 0 || hotOnly && batch.length === 0) { round++; continue; }
    if (batch.length === 0) {
      offset %= names.length;
      batch.push(...names.slice(offset, offset + policy.pollBatch));
      if (batch.length < policy.pollBatch) batch.push(...names.slice(0, Math.min(names.length - batch.length, policy.pollBatch - batch.length)));
      offset += policy.pollBatch;
    }
    let hit = 0, next = 0;
    await Promise.all(Array.from({ length: Math.min(concurrency ?? takeConcurrencyV1(Infinity, policy), batch.length) }, async () => {
      while (next < batch.length && stop() !== true) {
        const name = batch[next++];
        looks++;
        let outcome;
        try { outcome = await attempt(name); } catch { continue; }
        if (outcome === "present" || outcome === "fetched") { hit++; found++; await arrived(name, outcome); }
        else missedAt.set(name, now());
      }
    }));
    round = hit > 0 ? 0 : round + 1;
  }
  return Object.freeze({ looks, found });
}

/** 🎣️ The take pass of a read that adopts before it looks: beside the lanes of `takePassV1` (own looks, slow under load) an adopt loop (`takePollV1` with `hotOnly`, `adoptConcurrency` looks at once, each owner again after `adoptRetryMilliseconds`) looks at every undecided owner that the fleet lists as stored as soon as it is listed, so a record the fleet delivers is taken within one poll of its publication and never waits for the lanes; the loop ends with the lanes or when `stop()` says every owner is decided. Answers what the adopt loop did.
 * @see ./🧪️tests/🎣️fetch-first/🟦️.ts */
export async function takeFetchFirstV1({ lanes, heavy = [], concurrency = TAKE_POLICY_V1.concurrency, attempt, signal, mark = () => {}, hot, queue, arrived, stop = () => false, now, sleep, policy = TAKE_POLICY_V1 }) {
  const adopting = new AbortController(), abort = signal === undefined ? adopting.signal : AbortSignal.any([signal, adopting.signal]);
  const adopt = takePollV1({ hot, hotOnly: true, queue, attempt, arrived, stop, signal: abort, now, sleep, concurrency: Math.min(policy.adoptConcurrency, concurrency), policy: { ...policy, hotRetryMilliseconds: policy.adoptRetryMilliseconds } }).catch(() => Object.freeze({ looks: 0, found: 0 }));
  try {
    await Promise.all(lanes.map(lane => takePassV1({ names: lane, concurrency, attempt, signal, mark })));
    await takePassV1({ names: heavy, concurrency: 1, attempt, signal, mark });
  } finally { adopting.abort(); }
  return adopt;
}

/**
 * 🚀️ Runs the take pass and the inference of the owners beside each other, so that a reader never waits for the fleet or for the warm pass before it infers: `release()` opens the inference when the take pass ends or `releaseMilliseconds` after the start, whichever comes first, and `stopWarm()` stops the warm pass at that moment (its children are the ones the inference needs); `infer()` is called at the release and `end()` tells the pass that nothing is left to take. The pass and the warm pass are awaited at the end; a failing inference wins over a failing pass, a failing pass is rethrown.
 * @see ./🧪️tests/🚀️release/🟦️.ts
 */
export async function runTakeBesideInferV1({ take, warm, stopWarm, release, infer, end, releaseMilliseconds = TAKE_POLICY_V1.releaseMilliseconds, sleep = (milliseconds, abort) => new Promise(resolve => { const timer = setTimeout(resolve, milliseconds); abort?.addEventListener("abort", () => { clearTimeout(timer); resolve(); }, { once: true }); }) }) {
  const waiting = new AbortController();
  let open = false, opening;
  const gate = new Promise(resolve => { opening = resolve; });
  const opened = () => { if (open) return; open = true; waiting.abort(); try { stopWarm?.(); release(); } finally { opening(); } };
  const timer = sleep(releaseMilliseconds, waiting.signal).then(opened);
  const taking = (async () => { try { await take(); } finally { opened(); } })(), warming = warm === undefined ? Promise.resolve() : (async () => warm())();
  const settled = Promise.allSettled([taking, warming, timer]);
  let failure, failed = false;
  try { await gate; await infer(); } catch (error) { failure = error; failed = true; }
  try { end?.(); } finally { opened(); }
  const [passed] = await settled;
  if (failed) throw failure;
  if (passed.status === "rejected") throw passed.reason;
}
