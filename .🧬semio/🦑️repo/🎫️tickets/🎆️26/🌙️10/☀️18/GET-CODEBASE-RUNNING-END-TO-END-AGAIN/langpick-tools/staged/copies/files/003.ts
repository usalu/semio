import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { resolve } from "node:path";
import { acquireQueuedResourceLease, type LeaseWait, type ResourceLease, type ResourceLeaseControl } from "../../../../🔒️leases/🟦️.ts";
import { readLeaseQueuesV1 } from "../../../../🔒️leases/📋️status/🟦️.ts";
import { systemAncestryV1 } from "../../../../🪪️agent-sessions/🧭️ancestry/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const protocol = schema.definitions.Protocol.const, messages = schema.definitions.Messages.const;

/** 🔐️ One held custody of a Cargo build directory and profile; `inherited` is true when an ancestor process of this one holds it, so nothing was taken. */
export type CargoCustodyV1 = Readonly<{ release(): void; inherited: boolean }>;

type EntryV1 = { count: number; lease?: ResourceLease; ready: Promise<void>; abort: AbortController };
const entries = new Map<string, EntryV1>();
let ancestors: ReadonlySet<number> | undefined;

/** 🏷️ The lease resource that names one build directory and profile, the physical scope of Cargo's unit locks. */
export function cargoCustodyResourceV1(buildDirectory: string, profile: string): string {
  return JSON.stringify([protocol.namespace, buildDirectory, profile]);
}

/** 🌳️ The ancestor processes of this one, nearest first; they run the work this process is part of. */
export function processAncestorsV1(): ReadonlySet<number> {
  return ancestors ??= new Set(systemAncestryV1(process.platform, process.ppid, protocol.ancestorDepth).map(row => row.pid));
}

/** 🎒️ The processes whose exclusive ticket heads the arrival queue of a custody resource, that is the ones holding it. */
export function cargoCustodyHoldersV1(directory: string, resource: string): readonly number[] {
  const hash = createHash("sha256").update(resource).digest("hex");
  return readLeaseQueuesV1(resolve(directory)).filter(row => row.hash === hash && row.position === protocol.holderPosition && row.mode === "exclusive").map(row => row.pid);
}

function say(directory: string, resource: string, buildDirectory: string, profile: string, seconds: number): void {
  const holder = cargoCustodyHoldersV1(directory, resource)[0];
  process.stderr.write(noticeLanguagesV1(["en", "de"] as const).map((language) => messages.waiting[language].replace("{directory}", buildDirectory).replace("{profile}", profile).replace("{pid}", String(holder ?? "?")).replace("{seconds}", String(seconds))).join("\n") + "\n");
}

function join(key: string, entry: EntryV1, signal: AbortSignal): Promise<CargoCustodyV1> {
  entry.count++;
  let released = false;
  const leave = (): void => {
    if (released) return;
    released = true;
    if (--entry.count > 0) return;
    if (entries.get(key) === entry) entries.delete(key);
    if (entry.lease) entry.lease.release(); else entry.abort.abort();
  };
  const cancelled = new Promise<never>((_, reject) => {
    const abort = (): void => reject(signal.reason ?? new Error("Aborted"));
    if (signal.aborted) abort(); else signal.addEventListener("abort", abort, { once: true });
  });
  cancelled.catch(() => undefined);
  return Promise.race([entry.ready, cancelled]).then(
    () => Object.freeze({ release: leave, inherited: false }),
    error => { leave(); throw error; },
  );
}

/** 🦀️ Takes the exclusive custody of one Cargo build directory and profile for this process tree, behind the fair arrival queue. `-Zfine-grain-locking` takes a shared file lock on every fresh unit of a build and an exclusive one on every dirty unit, so two Cargo processes whose freshness views differ (a live edit, an environment-dependent build script) each hold what the other waits for and sleep for ever. Cargo cannot be told to avoid that, so at most one process tree compiles into a build directory and profile at a time; passes of one driver, and processes it spawns, share its custody. */
export async function acquireCargoCustodyV1(options: Readonly<{ directory: string; buildDirectory: string; profile: string; signal: AbortSignal; control?: ResourceLeaseControl; onWait?: (progress: LeaseWait) => void | Promise<void> }>): Promise<CargoCustodyV1> {
  options.signal.throwIfAborted();
  const resource = cargoCustodyResourceV1(options.buildDirectory, options.profile), key = resolve(options.directory) + "\0" + resource, present = entries.get(key);
  if (present) return join(key, present, options.signal);
  const holders = cargoCustodyHoldersV1(options.directory, resource);
  if (holders.length && holders.some(pid => pid !== process.pid && processAncestorsV1().has(pid))) return Object.freeze({ release() {}, inherited: true });
  const entry: EntryV1 = { count: 0, ready: undefined as unknown as Promise<void>, abort: new AbortController() }, started = Date.now();
  let lastSaid = -Infinity;
  entry.ready = acquireQueuedResourceLease({
    directory: options.directory, resource, mode: "exclusive", owner: randomBytes(16).toString("base64url"), signal: options.control ? options.control.signal : entry.abort.signal, control: options.control,
    onWait: async progress => {
      await options.onWait?.(progress);
      const elapsed = Date.now() - started;
      if (elapsed >= protocol.announceAfterMs && elapsed - lastSaid >= protocol.announceEveryMs) { lastSaid = elapsed; say(options.directory, resource, options.buildDirectory, options.profile, Math.round(elapsed / 1000)); }
    },
  }).then(lease => { entry.lease = lease; if (entry.count === 0) lease.release(); });
  entry.ready.catch(() => { if (entries.get(key) === entry) entries.delete(key); });
  entries.set(key, entry);
  return join(key, entry, options.signal);
}
