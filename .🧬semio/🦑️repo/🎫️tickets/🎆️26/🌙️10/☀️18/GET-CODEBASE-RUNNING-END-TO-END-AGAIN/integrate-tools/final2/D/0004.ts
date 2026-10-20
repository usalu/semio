import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1} from "../🕸️units/📥️capture/🟦️.ts";
import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1,type CargoSourceBoundaryPolicyV1} from "../📥️source/🎟️admission/🟦️.ts";
import { compilerAdmissionBudgetV1, type CompilerAdmissionControl } from "../../../🏗️build/🎟️capacity/🟦️.ts";
import { checkResourceLeaseControl, requireMaterializedPathV1 } from "../../../🔒️leases/🟦️.ts";
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { existsSync, rmSync } from "node:fs";
import { mkdir, readFile, readdir, rename, writeFile, rm } from "node:fs/promises";
import { createServer, type Socket } from "node:net";
import { tmpdir, totalmem } from "node:os";
import { basename, dirname, extname, join, resolve } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";
import { captureSchedulerReceiptV1, readSchedulerReceiptV1, schedulerIdentityV1, schedulerReceiptFileV1, schedulerReceiptHoldsV1, writeSchedulerReceiptV1 } from "./🔖️scheduler/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🪓️termination/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🎟️ The admission policy constants: reserve, prediction headroom, cold-start model and feedback thresholds. */
export const ADMISSION_POLICY_V1 = schema.definitions.Policy.const;

/** 🧠️ Bounds physical memory by an actual positive process constraint; zero reports no known bound. */
export function admissionMemoryBytesV1(physicalBytes=totalmem(),constrainedBytes=process.constrainedMemory()):number{
 if(!Number.isSafeInteger(physicalBytes)||physicalBytes<1||!Number.isSafeInteger(constrainedBytes)||constrainedBytes<0)throw Error("Actual finite memory capacity required");
 return constrainedBytes>0?Math.min(physicalBytes,constrainedBytes):physicalBytes;
}

/** ⚖️ Shares effective memory after the unchanged reserve without reconstructing capacity above its real bound. */
export function admissionMemoryShareV1(effectiveBytes:number,tokens:number,parallelism:number):number{
 if(![effectiveBytes,tokens,parallelism].every(value=>Number.isSafeInteger(value)&&value>0)||tokens>parallelism)throw Error("Actual bounded memory share required");
 return Math.min(effectiveBytes,ADMISSION_POLICY_V1.reserveBytes+Math.floor((effectiveBytes-ADMISSION_POLICY_V1.reserveBytes)*tokens/parallelism));
}


/** 📨️ The wrapper's admission request (`AdmitRequestV1`). */
export type AdmitRequestV1 = Readonly<{ v: 1; op: "admit"; pass: string; pid: number; crate: string; package: string; version: string; target: string; crateTypes: readonly string[]; outDir: string; extraFilename: string; source: string; manifestDir: string; primary: boolean; buildScriptOutDir: string | null; externs: readonly string[] }>;

/** 🏁️ The wrapper's completion report (`DoneReportV1`). */
export type DoneReportV1 = Readonly<{ v: 1; op: "done"; exit: number; peakRssBytes: number; userMs: number; sysMs: number; startMs: number; endMs: number; rmetaMs: number | null }>;

/** 📒️ One recorded observation of a unit key (`ObservationV1`), the scheduling knowledge `units.jsonl` keeps. */
export type UnitObservationV1 = Readonly<{ v: 1; key: string; peakRssBytes: number; outputBytes: number; wallMs: number; cpuMs: number; remainingMs: number; atMs: number }>;

/** 🔗️ One dependency of a unit by the artifact identities it may name and what it consumes: metadata only (pipelined) or the complete library. */
export type UnitEdgeV1 = Readonly<{ artifacts: readonly string[]; edge: "rmeta" | "full" }>;

/** 🧾️ One admitted rustc unit as the broker measured it. */
export type UnitRecordV1 = Readonly<{ key: string; pass: string; profile: string; target: string; crate: string; package: string; version: string; crateTypes: readonly string[]; primary: boolean; artifact: string; dependencies: readonly UnitEdgeV1[]; buildScriptOutDir: string | null; predictedBytes: number; requestMs: number; admittedMs: number; startMs: number; endMs: number; rmetaMs: number | null; peakRssBytes: number; userMs: number; sysMs: number; exit: number }>;

/** 🧳️ A unit waiting for admission, as the ordering and decision laws see it. */
export type AdmissionWaiterV1 = Readonly<{ predictedBytes: number; expectedWallMs: number | null; remainingMs: number; arrival: number }>;

/** 🏃️ A running unit, as the decision law sees it. */
export type AdmissionRunningV1 = Readonly<{ predictedBytes: number; expectedEndMs: number | null }>;

/** 🧮️ The machine state one admission decision is taken in. */
export type AdmissionStateV1 = Readonly<{ budgetBytes: number; frozen: boolean; nowMs: number; running: readonly AdmissionRunningV1[] }>;

/** 🛂️ Validates one value against a named admission schema definition. */
function admitDefinition<T>(name: keyof typeof schema.definitions, value: unknown): T {
  const errors = validateJsonSchemaSubset(schema.definitions[name], value, schema);
  if (errors.length) throw Error(`Invalid ${name}: ${errors.join("; ")}`);
  return value as T;
}

/** 🔑️ The scheduling identity of a unit: package, crate, target, profile and crate types. */
export function unitKeyV1(request: Pick<AdmitRequestV1, "package" | "crate" | "target" | "crateTypes">, profile: string): string {
  return `${request.package}@${request.crate}|${request.target}|${profile}|${[...request.crateTypes].sort().join(",")}`;
}

/** 🏷️ The artifact identity a unit produces: its output directory and file stem (`<crate><extra-filename>`). */
export function unitArtifactV1(request: Pick<AdmitRequestV1, "outDir" | "crate" | "extraFilename">): string {
  return join(resolve(request.outDir), `${request.crate}${request.extraFilename}`);
}

/** 🔗️ The artifact identities an `--extern` path may name (`lib` prefixes are ambiguous for crates named `lib*`) and its edge kind. */
export function externArtifactsV1(path: string): { readonly candidates: readonly string[]; readonly edge: "rmeta" | "full" } {
  const directory = dirname(resolve(path)), stem = basename(path, extname(path));
  return { candidates: [...new Set([join(directory, stem.replace(/^lib/u, "")), join(directory, stem)])], edge: extname(path) === ".rmeta" ? "rmeta" : "full" };
}

/** 📐️ The `q` quantile with linear interpolation between closest ranks (R-7, the NumPy and d3 default). */
export function quantileR7V1(values: readonly number[], q: number): number {
  if (!values.length) throw Error("Quantile of no values");
  const sorted = [...values].sort((left, right) => left - right), position = (sorted.length - 1) * q, lower = Math.floor(position);
  return lower + 1 >= sorted.length ? sorted[lower]! : sorted[lower]! + (position - lower) * (sorted[lower + 1]! - sorted[lower]!);
}

/** 🔮️ Predicted peak resident bytes of a unit: `max(headroom × p95(last window peaks), floor)`; without history the cold-start model, and the component allowance for an unknown final component unit. */
export function predictPeakBytesV1(peaks: readonly number[], cold: Readonly<{ kloc: number; componentFinal: boolean }>): number {
  const policy = ADMISSION_POLICY_V1;
  if (peaks.length) return Math.max(Math.ceil(policy.headroom * quantileR7V1(peaks.slice(-policy.window), policy.quantile)), policy.floorBytes);
  return cold.componentFinal ? policy.componentFinalBytes : Math.max(Math.ceil(policy.coldBaseBytes + policy.coldBytesPerKloc * cold.kloc), policy.floorBytes);
}

/** ⏳️ Expected wall time of a key: the median of its recorded walls in the window, rounded half up; `null` without history. */
export function expectedWallMsV1(history: readonly UnitObservationV1[]): number | null {
  return history.length ? Math.round(quantileR7V1(history.slice(-ADMISSION_POLICY_V1.window).map((row) => row.wallMs), 0.5)) : null;
}

/** 🥇️ Waiter priority: longest recorded remaining path first, then the largest predicted peak, then arrival order. */
export function admissionOrderV1<T extends AdmissionWaiterV1>(waiters: readonly T[]): T[] {
  return [...waiters].sort((left, right) => right.remainingMs - left.remainingMs || right.predictedBytes - left.predictedBytes || left.arrival - right.arrival);
}

/** ⚖️ Which ordered waiters start now. A waiter starts while the predicted total fits the budget; the first waiter that does not fit holds a
 * reservation (EASY backfilling): later waiters only start if they fit and either end before the reservation's shadow time or fit in the memory
 * left beside it. With nothing running the first waiter always starts, so progress never depends on a prediction. A frozen broker starts nothing
 * while anything runs. Returns indices into `ordered`. */
export function admitWaitersV1(state: AdmissionStateV1, ordered: readonly AdmissionWaiterV1[]): number[] {
  const admitted: number[] = [];
  const releases = state.running.map((row) => ({ bytes: row.predictedBytes, endMs: row.expectedEndMs ?? Infinity }));
  let available = state.budgetBytes - releases.reduce((sum, row) => sum + row.bytes, 0), running = releases.length;
  let shadow: { atMs: number; extraBytes: number } | null = null;
  for (let index = 0; index < ordered.length; index++) {
    const waiter = ordered[index]!, endMs = waiter.expectedWallMs === null ? Infinity : state.nowMs + waiter.expectedWallMs;
    if (running === 0) { admitted.push(index); running++; available -= waiter.predictedBytes; releases.push({ bytes: waiter.predictedBytes, endMs }); continue; }
    if (state.frozen) break;
    if (shadow === null) {
      if (waiter.predictedBytes <= available) { admitted.push(index); running++; available -= waiter.predictedBytes; releases.push({ bytes: waiter.predictedBytes, endMs }); continue; }
      let free = available, atMs = state.nowMs;
      for (const release of [...releases].sort((left, right) => left.endMs - right.endMs)) { if (free >= waiter.predictedBytes) break; free += release.bytes; atMs = release.endMs; }
      shadow = { atMs, extraBytes: free - waiter.predictedBytes };
      continue;
    }
    if (waiter.predictedBytes > available || !(endMs < shadow.atMs || waiter.predictedBytes <= shadow.extraBytes)) continue;
    admitted.push(index);
    running++;
    available -= waiter.predictedBytes;
    if (endMs >= shadow.atMs) shadow.extraBytes -= waiter.predictedBytes;
  }
  return admitted;
}

/** 🧊️ Admission freeze with hysteresis: freeze when measured rustc memory exceeds the budget or swap-out exceeds its rate; resume below the resume fraction. */
export function admissionFrozenV1(wasFrozen: boolean, sample: Readonly<{ actualBytes: number; budgetBytes: number; swapOutBytesPerSecond: number | null }>): boolean {
  const policy = ADMISSION_POLICY_V1, swapping = (sample.swapOutBytesPerSecond ?? 0) > policy.swapOutBytesPerSecond;
  if (sample.actualBytes > sample.budgetBytes || swapping) return true;
  return wasFrozen && sample.actualBytes >= policy.resumeFraction * sample.budgetBytes;
}

/** 🗄️ The default telemetry store: `<cache>/build-telemetry/units.jsonl`, never cleared by fresh builds. */
export function unitHistoryPathV1(cacheRoot: string): string {
  return join(cacheRoot, ADMISSION_POLICY_V1.storeDirectory, ADMISSION_POLICY_V1.storeFile);
}

/** 🗜️ Keeps the newest `window` observations of every key, ordered by time. */
export function compactUnitHistoryV1(rows: readonly UnitObservationV1[]): Map<string, UnitObservationV1[]> {
  const history = new Map<string, UnitObservationV1[]>();
  for (const row of [...rows].sort((left, right) => left.atMs - right.atMs)) history.set(row.key, [...(history.get(row.key) ?? []), row].slice(-ADMISSION_POLICY_V1.window));
  return history;
}

/** 📖️ Reads the store; a torn trailing line from an interrupted append is skipped, every other row must satisfy `ObservationV1`. */
export async function readUnitHistoryV1(path: string): Promise<Map<string, UnitObservationV1[]>> {
  if (!existsSync(path)) return new Map();
  const lines = (await readFile(path, "utf8")).split("\n").filter(Boolean), rows: UnitObservationV1[] = [];
  for (const [index, line] of lines.entries()) {
    let value: unknown;
    try { value = JSON.parse(line); } catch (error) { if (index === lines.length - 1) continue; throw error; }
    rows.push(admitDefinition<UnitObservationV1>("ObservationV1", value));
  }
  return compactUnitHistoryV1(rows);
}

/** ✍️ Merges new observations into the store under its exclusive lease and rewrites it compacted, atomically. */
export async function recordUnitHistoryV1(path: string, rows: readonly UnitObservationV1[], options: Readonly<{ leaseDirectory: string; control: CompilerAdmissionControl }>): Promise<void> {
  checkResourceLeaseControl(options.control);
  path = requireMaterializedPathV1(path);
  const temporary = requireMaterializedPathV1(`${path}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`);
  for (const row of rows) admitDefinition<UnitObservationV1>("ObservationV1", row);
  const lease = await acquireQueuedResourceLease({ directory: options.leaseDirectory, resource: JSON.stringify([ADMISSION_POLICY_V1.storeNamespace, resolve(path)]), mode: "exclusive", owner: randomBytes(16).toString("base64url"), signal: options.control.signal, control: options.control });
  try {
    const merged = compactUnitHistoryV1([...[...(await readUnitHistoryV1(path)).values()].flat(), ...rows]);
    checkResourceLeaseControl(options.control);
    await mkdir(dirname(path), { recursive: true });
    checkResourceLeaseControl(options.control);
    await writeFile(temporary, [...merged.values()].flat().sort((left, right) => left.atMs - right.atMs).map((row) => `${JSON.stringify(row)}\n`).join(""));
    checkResourceLeaseControl(options.control);
    await rename(temporary, path);
    checkResourceLeaseControl(options.control);
  } finally { try { await rm(temporary, { force: true }); } finally { lease.release(); } }
}

const klocCache = new Map<string, Promise<number>>();

/** 📏️ Thousands of Rust source lines under a crate root's directory, the cold-start size estimate (cached per directory). */
export function sourceKlocV1(source: string): Promise<number> {
  const root = dirname(resolve(source));
  let cached = klocCache.get(root);
  if (!cached) {
    const walk = async (directory: string): Promise<number> => {
      let lines = 0;
      for (const entry of await readdir(directory, { withFileTypes: true }).catch(() => [])) {
        if (entry.isDirectory() && !ADMISSION_POLICY_V1.sourceSkipDirectories.includes(entry.name)) lines += await walk(join(directory, entry.name));
        else if (entry.isFile() && entry.name.endsWith(ADMISSION_POLICY_V1.sourceExtension)) { const text = await readFile(join(directory, entry.name)); for (const byte of text) if (byte === 10) lines++; }
      }
      return lines;
    };
    cached = walk(root).then((lines) => lines / 1000);
    klocCache.set(root, cached);
  }
  return cached;
}

/** 🔌️ A private broker endpoint: a short Unix socket path (sockets are limited to ~104 bytes) or a Windows named pipe. */
export function brokerEndpointV1(platform: NodeJS.Platform = process.platform): string {
  const name = `semio-broker-${process.pid}-${randomBytes(6).toString("hex")}`;
  return platform === "win32" ? `\\\\.\\pipe\\${name}` : join(tmpdir(), `${name}.sock`);
}

/** 🛣️ One registered Cargo pass, so units can be keyed by profile. */
export type BrokerPassV1 = Readonly<{ id: string; profile: string; target: string }>;

/** 📡️ Live broker counters for the timeline and the stall watchdog. */
export type BrokerStatsV1 = Readonly<{ running: number; waiting: number; frozen: boolean; budgetBytes: number; predictedRunningBytes: number; actualRunningBytes: number; lastActivityMs: number }>;

/** 🎟️ The driver's admission broker. */
export interface AdmissionBrokerV1 {
  readonly endpoint: string;
  registerPass(pass: BrokerPassV1): void;
  observe(sample: Readonly<{ nonRustcBytes: number; swapOutBytesPerSecond: number | null }>): void;
  stats(): BrokerStatsV1;
  units(): readonly UnitRecordV1[];
  close(): Promise<void>;
}

type Entry = { request: AdmitRequestV1; key: string; profile: string; predictedBytes: number; expectedWallMs: number | null; remainingMs: number; arrival: number; requestMs: number; admittedMs: number; rssBytes: number; socket: Socket };

/** 🎟️ Starts the admission broker: wrappers connect, ask for admission, stream resident memory and report completion.
 * The budget is `physical − reserve − rolling maximum of the driver's other children`; measured overshoot or swap-out freezes admission. */
export async function startAdmissionBrokerV1(options: Readonly<{ physicalBytes: number; history: ReadonlyMap<string, readonly UnitObservationV1[]>; endpoint?: string; now?: () => number }>): Promise<AdmissionBrokerV1> {
  const now = options.now ?? Date.now, endpoint = options.endpoint ?? brokerEndpointV1(), passes = new Map<string, BrokerPassV1>();
  const waiting: Entry[] = [], running = new Set<Entry>(), completed: UnitRecordV1[] = [], nonRustc: { atMs: number; bytes: number }[] = [];
  let frozen = false, arrivals = 0, lastActivityMs = now();
  const budget = (): number => options.physicalBytes - ADMISSION_POLICY_V1.reserveBytes - Math.max(0, ...nonRustc.map((row) => row.bytes));
  const verdict = (socket: Socket, admitted: boolean): void => { if (!socket.destroyed) socket.write(`{"v":1,"op":"verdict","admitted":${admitted}}\n`); };
  const schedule = (): void => {
    const ordered = admissionOrderV1(waiting), state = { budgetBytes: budget(), frozen, nowMs: now(), running: [...running].map((entry) => ({ predictedBytes: entry.predictedBytes, expectedEndMs: entry.expectedWallMs === null ? null : entry.admittedMs + entry.expectedWallMs })) };
    for (const index of admitWaitersV1(state, ordered)) {
      const entry = ordered[index]!;
      waiting.splice(waiting.indexOf(entry), 1);
      entry.admittedMs = now();
      running.add(entry);
      lastActivityMs = entry.admittedMs;
      verdict(entry.socket, true);
    }
  };
  const forget = (entry: Entry | undefined): void => {
    if (!entry) return;
    const index = waiting.indexOf(entry);
    if (index >= 0) waiting.splice(index, 1);
    running.delete(entry);
    schedule();
  };
  const server = createServer((socket) => {
    let entry: Entry | undefined, finished = false, chain = Promise.resolve();
    socket.on("error", () => undefined);
    socket.once("close", () => { chain = chain.then(() => { if (!finished) forget(entry); }); });
    createInterface({ input: socket, crlfDelay: Infinity }).on("line", (line) => {
      chain = chain.then(async () => {
        const message = JSON.parse(line) as { op?: string };
        if (message.op === "admit" && !entry) {
          const request = admitDefinition<AdmitRequestV1>("AdmitRequestV1", message), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          const key = unitKeyV1(request, pass.profile), history = options.history.get(key) ?? [];
          const componentFinal = request.crateTypes.includes("cdylib") && request.target !== "host";
          const predictedBytes = predictPeakBytesV1(history.map((row) => row.peakRssBytes), { kloc: history.length || componentFinal ? 0 : await sourceKlocV1(request.source), componentFinal });
          entry = { request, key, profile: pass.profile, predictedBytes, expectedWallMs: expectedWallMsV1(history), remainingMs: history.at(-1)?.remainingMs ?? 0, arrival: arrivals++, requestMs: now(), admittedMs: 0, rssBytes: 0, socket };
          waiting.push(entry);
          lastActivityMs = entry.requestMs;
          schedule();
          if (waiting.includes(entry)) verdict(socket, false);
        } else if (message.op === "rss" && entry) {
          entry.rssBytes = admitDefinition<{ bytes: number }>("RssReportV1", message).bytes;
        } else if (message.op === "done" && entry && running.has(entry)) {
          const done = admitDefinition<DoneReportV1>("DoneReportV1", message), request = entry.request;
          finished = true;
          running.delete(entry);
          lastActivityMs = now();
          completed.push(Object.freeze({ key: entry.key, pass: request.pass, profile: entry.profile, target: request.target, crate: request.crate, package: request.package, version: request.version, crateTypes: request.crateTypes, primary: request.primary, artifact: unitArtifactV1(request), dependencies: request.externs.map((path) => { const extern = externArtifactsV1(path); return { artifacts: extern.candidates, edge: extern.edge }; }), buildScriptOutDir: request.buildScriptOutDir, predictedBytes: entry.predictedBytes, requestMs: entry.requestMs, admittedMs: entry.admittedMs, startMs: done.startMs, endMs: done.endMs, rmetaMs: done.rmetaMs, peakRssBytes: done.peakRssBytes, userMs: done.userMs, sysMs: done.sysMs, exit: done.exit }));
          schedule();
        } else throw Error(`Unexpected broker message ${line.slice(0, 200)}`);
      }).catch((error) => { console.error(`[admission] ${error instanceof Error ? error.message : String(error)}`); socket.destroy(); });
    });
  });
  if (process.platform !== "win32") rmSync(endpoint, { force: true });
  await new Promise<void>((accept, reject) => { server.once("error", reject); server.listen(endpoint, () => { server.off("error", reject); accept(); }); });
  return {
    endpoint,
    registerPass: (pass) => { passes.set(pass.id, pass); },
    observe: (sample) => {
      const at = now();
      nonRustc.push({ atMs: at, bytes: sample.nonRustcBytes });
      while (nonRustc.length && nonRustc[0]!.atMs < at - ADMISSION_POLICY_V1.nonRustcWindowMs) nonRustc.shift();
      frozen = admissionFrozenV1(frozen, { actualBytes: [...running].reduce((sum, entry) => sum + entry.rssBytes, 0), budgetBytes: budget(), swapOutBytesPerSecond: sample.swapOutBytesPerSecond });
      schedule();
    },
    stats: () => ({ running: running.size, waiting: waiting.length, frozen, budgetBytes: budget(), predictedRunningBytes: [...running].reduce((sum, entry) => sum + entry.predictedBytes, 0), actualRunningBytes: [...running].reduce((sum, entry) => sum + entry.rssBytes, 0), lastActivityMs }),
    units: () => completed,
    close: () => new Promise<void>((accept) => { server.close(() => { if (process.platform !== "win32") rmSync(endpoint, { force: true }); accept(); }); for (const entry of [...waiting, ...running]) entry.socket.destroy(); }),
  };
}

/** 🪜️ Recorded observations of a finished build: every successful unit's peak, measured output footprint (`outputBytes` by unit artifact), wall, CPU and
 * remaining critical path. */
export function unitObservationsV1(units: readonly UnitRecordV1[], remainingMs: ReadonlyMap<string, number>, outputBytes: ReadonlyMap<string, number>, atMs: number): UnitObservationV1[] {
  return units.filter((unit) => unit.exit === 0).map((unit) => { const bytes = outputBytes.get(unit.artifact); if (bytes === undefined) throw Error(`Unit output footprint was never measured: ${unit.key}`); return { v: 1 as const, key: unit.key, peakRssBytes: unit.peakRssBytes, outputBytes: bytes, wallMs: Math.max(0, unit.endMs - unit.startMs), cpuMs: unit.userMs + unit.sysMs, remainingMs: Math.max(0, Math.round(remainingMs.get(unit.artifact) ?? 0)), atMs }; });
}

/** 📏️ The bytes every output of one unit occupies: the files of its output directory named by its artifact stem (`<stem>.*` or `lib<stem>.*`). */
export function unitOutputBytesV1(artifact: string, entries: readonly string[], size: (name: string) => number): number {
  const stem = basename(artifact);
  return entries.filter((name) => name.startsWith(stem + ".") || name.startsWith("lib" + stem + ".")).reduce((sum, name) => sum + size(name), 0);
}

/** 🛠️ Compiles the scheduler crate (host, release) and returns its executables by name, examples included on request. */
export async function buildSchedulerV1(options: Readonly<{ cwd: string; targetDirectory: string; buildDirectory: string; environment: Readonly<Record<string, string | undefined>>; examples?: boolean; toolchain?: string; control: CompilerAdmissionControl; storage:NativeStorageReservationV1;sourceBoundary:CargoSourceBoundaryPolicyV1 }>): Promise<ReadonlyMap<string, string>> {
  requireNativeStorageReservationV1(options.storage,options.control);await options.storage.recheck();for(const path of [options.targetDirectory,options.buildDirectory,...["CARGO_HOME","TMPDIR","TMP","TEMP"].map(name=>{const path=options.environment[name];if(!path)throw Error("Bounded scheduler environment required: "+name);return path;})])requireNativeStoragePathV1(options.storage.backing.root,path);
  const budget = compilerAdmissionBudgetV1(options.control), signal = options.control.signal;
  const manifest = fileURLToPath(new URL("./📦️packages/🦀️rust/Cargo.toml", import.meta.url));
  const receiptFile=join(options.targetDirectory,schedulerReceiptFileV1()),identity=options.toolchain&&!options.examples?schedulerIdentityV1(options.toolchain,options.environment):undefined,held=identity?schedulerReceiptHoldsV1(readSchedulerReceiptV1(receiptFile),identity,manifest):undefined;if(held)return held;
  const graph=await captureCargoCompilationGraphV1({workspaceRoot:options.cwd,cwd:options.cwd,program:"cargo",prefix:[],args:["build","--locked","--release","--manifest-path",manifest,"--bins",...(options.examples?["--examples"]:[])],environment:options.environment,maximumOutputBytes:16777216},options.control,actualCargoCompilerPlanningPortV1());await admitCargoSourceBoundariesV1(graph,options.cwd,options.sourceBoundary,options.control,actualCargoSourceAdmissionPortV1(options.environment,options.cwd,options.storage));await options.storage.recheck();
  const child = spawn("cargo", ["build", "--locked", "--release", "--manifest-path", manifest, "--bins", ...(options.examples ? ["--examples"] : []), "--message-format=json-render-diagnostics"], { cwd: options.cwd, env: { ...options.environment, CARGO_TARGET_DIR: options.targetDirectory, CARGO_BUILD_BUILD_DIR: options.buildDirectory, RUSTC_WRAPPER: "", CARGO_BUILD_RUSTC_WRAPPER: "" }, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
  const cancel = (): void => terminateOwnedChildTree(child);
  const deadline = budget === 0 ? undefined : setTimeout(cancel, budget);
  signal.addEventListener("abort", cancel, { once: true });
  const executables = new Map<string, string>(), diagnostics: string[] = [];
  try {
    const [status] = await Promise.all([
      new Promise<number>((accept) => { child.once("error", () => accept(1)); child.once("close", (code) => accept(code ?? 1)); }),
      (async () => { for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) { try { const message = JSON.parse(line); if (message.reason === "compiler-artifact" && typeof message.executable === "string") executables.set(message.target.name, message.executable); } catch { diagnostics.push(line); } } })(),
      (async () => { for await (const line of createInterface({ input: child.stderr!, crlfDelay: Infinity })) if (diagnostics.length < 200) diagnostics.push(line); })(),
    ]);
    checkResourceLeaseControl(options.control);
    if (status !== 0 || !executables.has("semio-build-scheduler")) throw Error(`Build scheduler compilation failed (${status}):\n${diagnostics.join("\n")}`);
    if (identity) { const receipt = captureSchedulerReceiptV1(identity, manifest, executables); if (receipt) writeSchedulerReceiptV1(receiptFile, receipt); }
    return executables;
  } finally { if (deadline) clearTimeout(deadline); signal.removeEventListener("abort", cancel); }
}
