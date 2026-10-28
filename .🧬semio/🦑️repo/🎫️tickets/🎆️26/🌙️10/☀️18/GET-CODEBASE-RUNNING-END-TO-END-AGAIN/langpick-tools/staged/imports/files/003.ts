import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, rmdirSync, writeFileSync } from "node:fs";
import { randomUUID } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { resolveLanguageV1, type OsLanguageListV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { withResourceLeases } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

export type ServiceSession = { readonly schema: "semio.nx.service/v1"; readonly id: string; readonly owner: string; readonly pid: number };
export const SERVICE_READY_ENDPOINT = "/__semio/nx-service";
const idPattern = new RegExp(schema.$defs.Session.properties.id.pattern), urlPattern = new RegExp(schema.$defs.Ready.properties.url.pattern);
const keys = (value: unknown, expected: string): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join() === expected;

/** 🔐️ Serializes each service record transaction across cooperating Nx processes. */
function mutateServiceSession<T>(root: string, signal: AbortSignal, operation: () => T): Promise<T> {
  const contract = schema.$defs.MutationLease.const;
  return withResourceLeases({ directory: join(root, contract.directory), resources: [{ resource: contract.resource, mode: "exclusive" }], signal }, async () => operation());
}

/** 🛎️ Validates the owned service generation independently of any process-ID reuse. */
export function parseServiceSession(value: unknown): ServiceSession {
  if (!keys(value, "id,owner,pid,schema") || value.schema !== "semio.nx.service/v1" || typeof value.id !== "string" || !idPattern.test(value.id) || typeof value.owner !== "string" || !value.owner || value.owner.length > 256 || !Number.isSafeInteger(value.pid) || Number(value.pid) < 1) throw new Error("Invalid service session or pid");
  return value as ServiceSession;
}

/** 🗂️ Rejects links before accessing a task-owned service directory. */
function serviceDirectory(root: string, pid: number): string {
  if (!Number.isSafeInteger(pid) || pid < 1) throw new Error("Invalid Nx invocation pid");
  const directory = join(root, String(pid));
  for (let path = resolve(directory); dirname(path) !== path; path = dirname(path)) if (existsSync(path) && lstatSync(path).isSymbolicLink()) throw new Error(`Symlink service directory: ${path}`);
  return directory;
}

/** 📖️ Reads a bounded regular service record while rejecting unowned filesystem entries. */
function readRecord(path: string): unknown {
  const metadata = lstatSync(path);
  if (!metadata.isFile() || metadata.size > 4096) throw new Error(`Invalid service record: ${path}`);
  return JSON.parse(readFileSync(path, "utf8"));
}

/** 📬️ Publishes a small service record without exposing a partially written generation. */
function publishRecord(directory: string, name: string, value: unknown): void {
  const temporary = join(directory, `.stage-${randomUUID()}`);
  try { writeFileSync(temporary, JSON.stringify(value) + "\n", { flag: "wx" }); renameSync(temporary, join(directory, name)); }
  finally { rmSync(temporary, { force: true }); }
}

/** 📋️ Reads the generation explicitly prepared by this Nx invocation. */
export function readServiceSession(root: string, owner: string, pid: number): ServiceSession {
  const session = parseServiceSession(readRecord(join(serviceDirectory(root, pid), "session.json")));
  if (session.owner !== owner || session.pid !== pid) throw new Error("Service session owner mismatch");
  return session;
}

/** 🆕️ Starts a fresh generation after an uncached Nx preparation prerequisite. */
export async function openServiceSession(root: string, owner: string, pid: number, signal = AbortSignal.timeout(60000)): Promise<ServiceSession> {
  const directory = serviceDirectory(root, pid), session = parseServiceSession({ schema: "semio.nx.service/v1", id: randomUUID(), owner, pid });
  return mutateServiceSession(root, signal, () => {
    if (existsSync(directory)) {
      if (existsSync(join(directory, "session.json"))) readServiceSession(root, owner, pid);
      else if (readdirSync(directory).length) throw new Error(`Unowned service directory: ${directory}`);
    }
    mkdirSync(directory, { recursive: true });
    publishRecord(directory, "session.json", session);
    return session;
  });
}

/** 🛣️ Restricts readiness to an actual loopback TCP listener without credentials or redirects. */
function serviceUrl(value: unknown): string {
  if (typeof value !== "string" || !urlPattern.test(value)) throw new Error("Invalid service URL");
  let url: URL;
  try { url = new URL(value); } catch { throw new Error("Invalid service URL"); }
  if (!url.port || Number(url.port) > 65535) throw new Error("Invalid service URL port");
  return value;
}

/** 🧾️ Reads a complete readiness record with an independently validated generation. */
function parseReady(value: unknown): { session: ServiceSession; url: string } {
  if (!keys(value, "schema,session,url") || value.schema !== "semio.nx.service-ready/v1") throw new Error("Invalid service readiness record");
  return { session: parseServiceSession(value.session), url: serviceUrl(value.url) };
}

/** 📡️ Announces the listener only if its prepared generation remains current. */
export async function publishServiceReady(root: string, session: ServiceSession, url: string, signal = AbortSignal.timeout(60000)): Promise<void> {
  parseServiceSession(session);
  serviceUrl(url);
  return mutateServiceSession(root, signal, () => {
    const current = readServiceSession(root, session.owner, session.pid), directory = serviceDirectory(root, session.pid);
    if (current.id !== session.id) throw new Error("Service generation changed before readiness");
    if (existsSync(join(directory, "ready.json"))) {
      const previous = parseReady(readRecord(join(directory, "ready.json"))).session;
      if (previous.owner !== session.owner || previous.pid !== session.pid) throw new Error("Service readiness owner mismatch");
    }
    publishRecord(directory, "ready.json", { schema: "semio.nx.service-ready/v1", session, url });
  });
}

/** ⏱️ A product's declared E2E service readiness: the timeout, the progress cadence and the boot evidence that timeout derives from. */
export type ServiceReadinessBudget = {
  readonly schemaVersion: 1;
  readonly readyTimeoutMs: number;
  readonly progressIntervalMs: number;
  readonly evidence: {
    readonly kind: "estimate" | "measurement";
    readonly coldBootMs: { readonly minimum: number; readonly maximum: number };
    readonly warmBootMs: { readonly minimum: number; readonly maximum: number };
    readonly marginFactor: number;
    readonly source: string;
  };
};
/** 📈️ One progress observation while a consumer waits for a service generation. */
export type ServiceReadyProgress = { readonly elapsedMs: number; readonly timeoutMs: number; readonly phase: "awaiting-ready-record" | "verifying-listener" };
/** 📣️ Progress cadence and sink of one readiness wait. */
export type ServiceReadyWait = { readonly intervalMs: number; readonly report: (progress: ServiceReadyProgress) => void };
const budgetSchema = schema.$defs.ReadinessBudget, bootRangeSchema = schema.$defs.BootRange;
const inRange = (value: unknown, bounds: { readonly minimum: number; readonly maximum: number }): value is number => Number.isSafeInteger(value) && Number(value) >= bounds.minimum && Number(value) <= bounds.maximum;
const bootRange = (value: unknown): value is { minimum: number; maximum: number } => keys(value, "maximum,minimum") && inRange(value.minimum, bootRangeSchema.properties.minimum) && inRange(value.maximum, bootRangeSchema.properties.maximum) && Number(value.minimum) <= Number(value.maximum);

/** 🧮️ Admits a declared readiness budget only when its timeout covers the cold-boot maximum times the declared margin. */
export function parseServiceReadinessBudgetV1(value: unknown): ServiceReadinessBudget {
  const reject = (reason: string): never => { throw new Error(`Invalid service readiness budget: ${reason}`); };
  if (!keys(value, "evidence,progressIntervalMs,readyTimeoutMs,schemaVersion") || value.schemaVersion !== 1) return reject("closed versioned record required");
  const evidence = value.evidence;
  if (!keys(evidence, "coldBootMs,kind,marginFactor,source,warmBootMs") || !budgetSchema.properties.evidence.properties.kind.enum.includes(evidence.kind as never)) return reject("closed evidence record required");
  if (!bootRange(evidence.coldBootMs) || !bootRange(evidence.warmBootMs) || evidence.warmBootMs.maximum > evidence.coldBootMs.maximum) return reject("ordered boot ranges required");
  const margin = budgetSchema.properties.evidence.properties.marginFactor;
  if (typeof evidence.marginFactor !== "number" || !(evidence.marginFactor >= margin.minimum && evidence.marginFactor <= margin.maximum)) return reject("margin factor out of range");
  if (typeof evidence.source !== "string" || !evidence.source.length || evidence.source.length > budgetSchema.properties.evidence.properties.source.maxLength) return reject("evidence source required");
  if (!inRange(value.readyTimeoutMs, budgetSchema.properties.readyTimeoutMs) || !inRange(value.progressIntervalMs, budgetSchema.properties.progressIntervalMs)) return reject("timeout or cadence out of range");
  if (Number(value.readyTimeoutMs) < Math.ceil(evidence.coldBootMs.maximum * evidence.marginFactor)) return reject("timeout below cold-boot maximum times margin");
  if (Number(value.progressIntervalMs) > Number(value.readyTimeoutMs)) return reject("cadence beyond timeout");
  return value as ServiceReadinessBudget;
}

const progressText = {
  en: { "awaiting-ready-record": "waiting for the dev server to publish its ready record", "verifying-listener": "verifying the listener identity", of: "of" },
  de: { "awaiting-ready-record": "warte darauf, dass der Dev-Server seinen Bereitschaftseintrag veröffentlicht", "verifying-listener": "prüfe die Identität des Listeners", of: "von" },
} as const;

/** 🗣️ Chooses the progress language by the shared rule of the CLI help; no language is assumed. */
export function serviceReadyProgressLanguage(environment: Readonly<Record<string, string | undefined>>, osLanguages?: OsLanguageListV1): "en" | "de" {
  return resolveLanguageV1(["en", "de"], [], environment, osLanguages).language as "en" | "de";
}

/** 🔤️ Renders one readiness progress observation as a single log line. */
export function serviceReadyProgressLine(owner: string, progress: ServiceReadyProgress, language: "en" | "de"): string {
  const text = progressText[language], seconds = (milliseconds: number): number => Math.floor(milliseconds / 1000);
  return `[e2e] ${owner}: ${text[progress.phase]} (${seconds(progress.elapsedMs)} s ${text.of} ${seconds(progress.timeoutMs)} s)`;
}

/** ⏳️ Requires the current generation's HTTP identity before allowing its consumer to proceed within the caller's declared timeout, reporting progress on the declared cadence. */
export async function waitForServiceReady(root: string, session: ServiceSession, signal: AbortSignal, timeoutMs: number, wait?: ServiceReadyWait): Promise<string> {
  signal.throwIfAborted();
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1) throw new Error("Service readiness requires a declared positive timeout");
  const path = join(serviceDirectory(root, session.pid), "ready.json"), start = performance.now(), deadline = start + timeoutMs;
  let nextReport = start;
  while (performance.now() < deadline) {
    signal.throwIfAborted();
    if (wait && performance.now() >= nextReport) {
      wait.report({ elapsedMs: Math.round(performance.now() - start), timeoutMs, phase: existsSync(path) ? "verifying-listener" : "awaiting-ready-record" });
      nextReport = performance.now() + wait.intervalMs;
    }
    if (readServiceSession(root, session.owner, session.pid).id !== session.id) throw new Error("Service generation changed while waiting");
    if (existsSync(path)) {
      const ready = parseReady(readRecord(path));
      if (ready.session.id === session.id && ready.session.owner === session.owner && ready.session.pid === session.pid) {
        let matches = false;
        try {
          const response = await fetch(new URL(SERVICE_READY_ENDPOINT, ready.url), { signal: AbortSignal.any([signal, AbortSignal.timeout(Math.max(1, Math.min(1000, Math.ceil(deadline - performance.now()))))]), redirect: "error" });
          if (response.ok) {
            const actual = parseServiceSession(await response.json());
            matches = actual.id === session.id && actual.owner === session.owner && actual.pid === session.pid;
          }
        } catch { signal.throwIfAborted(); }
        if (matches) {
          if (readServiceSession(root, session.owner, session.pid).id !== session.id) throw new Error("Service generation changed during readiness response");
          return ready.url;
        }
      }
    }
    await delay(Math.max(1, Math.min(40, deadline - performance.now())), undefined, { signal });
  }
  throw new Error(`Service did not become ready with the expected identity within ${timeoutMs} ms: ${session.owner}`);
}

/** 🧹️ Removes only this completed generation's records and preserves unrelated or newer state. */
export async function closeServiceSession(root: string, session: ServiceSession, signal = AbortSignal.timeout(5000)): Promise<void> {
  parseServiceSession(session);
  return mutateServiceSession(root, signal, () => {
    const directory = serviceDirectory(root, session.pid), currentPath = join(directory, "session.json");
    if (!existsSync(currentPath) || readServiceSession(root, session.owner, session.pid).id !== session.id) return;
    const readyPath = join(directory, "ready.json");
    if (existsSync(readyPath) && parseReady(readRecord(readyPath)).session.id === session.id) rmSync(readyPath);
    rmSync(currentPath);
    try { rmdirSync(directory); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOTEMPTY" && (error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
  });
}
