import { spawn, type ChildProcess, type SpawnOptions } from "node:child_process";
import { reapWithParentV1 } from "../🧹️reaper/🟦️.ts";
import { ownedKillRecorderV1, terminateOwnedChildTreeGracefullyV1, type OwnedChildHandle } from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🔌️ What the registry needs of a child kind: how to stop it and whether it still runs. */
export interface SharedChildHostV1<Child extends OwnedChildHandle> {
  readonly stop: (child: Child, reason: string) => Promise<void>;
  readonly running?: (child: Child) => boolean;
}

/** 🎫️ One requester's hold on a shared child. */
export interface SharedChildLeaseV1<Child extends OwnedChildHandle> {
  readonly key: string;
  readonly requester: string;
  readonly child: Child;
  readonly started: boolean;
  readonly release: () => Promise<void>;
}

/** 🤝️ The registry of shared children by key and their requesters. */
export interface SharedChildRegistryV1<Child extends OwnedChildHandle> {
  readonly acquire: (key: string, requester: string, start: () => Child) => SharedChildLeaseV1<Child>;
  readonly requesters: (key: string) => readonly string[];
  readonly releaseAll: (requester: string) => Promise<void>;
  readonly size: () => number;
}

interface Entry<Child extends OwnedChildHandle> { readonly key: string; readonly child: Child; readonly requesters: Set<string> }

const runs = (child: OwnedChildHandle): boolean => child.exitCode === null && child.signalCode === null;

/** 🤝️ A registry whose last release stops the child through `host.stop`. Pure bookkeeping: it never signals anything itself.
 * @see ./🧬️schema/🔣️.json */
export function createSharedChildRegistryV1<Child extends OwnedChildHandle>(host: SharedChildHostV1<Child>): SharedChildRegistryV1<Child> {
  const entries = new Map<string, Entry<Child>>(), running = host.running ?? runs;
  const release = (entry: Entry<Child>, requester: string): Promise<void> => {
    if (!entry.requesters.delete(requester) || entry.requesters.size > 0) return Promise.resolve();
    if (entries.get(entry.key) === entry) entries.delete(entry.key);
    const text = schema.definitions.Texts.const;
    return running(entry.child) ? host.stop(entry.child, ["en", "de"].map((language) => text[language as "en" | "de"]["last-left"].replace("{requester}", requester).replace("{key}", entry.key)).join(" / ")) : Promise.resolve();
  };
  return {
    acquire(key, requester, start) {
      let entry = entries.get(key), started = false;
      if (entry === undefined || !running(entry.child)) {
        entry = { key, child: start(), requesters: new Set() };
        entries.set(key, entry);
        started = true;
      }
      entry.requesters.add(requester);
      const held = entry;
      return { key, requester, child: held.child, started, release: () => release(held, requester) };
    },
    requesters: (key) => [...(entries.get(key)?.requesters ?? [])],
    releaseAll: (requester) => Promise.all([...entries.values()].map((entry) => release(entry, requester))).then(() => undefined),
    size: () => entries.size,
  };
}

/** 🚀️ Starts one shared child in its own process group (POSIX) so that no requester's stop reaches it, registered with the reaper so that it does not outlive this process. */
export function spawnSharedChildV1(command: string, args: readonly string[], options: SpawnOptions = {}): ChildProcess {
  const child = spawn(command, [...args], { ...options, detached: process.platform !== "win32", windowsHide: true });
  const unregister = reapWithParentV1(child.pid);
  child.once("exit", unregister);
  return child;
}

/** 🛑️ The host of node children: the last release stops the whole tree with SIGTERM, the declared grace and SIGKILL, every signal recorded under `label`. */
export function sharedChildNodeHostV1(label: string, write?: (line: string) => void): SharedChildHostV1<ChildProcess> {
  return { stop: (child, reason) => terminateOwnedChildTreeGracefullyV1(child, { record: ownedKillRecorderV1(label, () => reason, write) }) };
}
