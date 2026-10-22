import { createRequire } from "node:module";
import type { RuntimeAdapterV1, RuntimeFamilyV1, RuntimeIdV1, RuntimeIdentityV1, RuntimeSupportV1 } from "../📋️contract/🟦️.ts";

/** 📒️ The closed registry of hostable runtimes; its shape is the `Registry` definition of the schema. */
export const RUNTIME_REGISTRY_V1 = {"version": 1, "families": ["glob", "toml", "jsonc", "yaml", "which", "resolve", "sleep", "spawn", "stdin", "file", "serve", "sqlite", "ffi", "transpile", "bundle", "semver", "identity", "gc", "plugin", "test"], "runtimes": [{"id": "bun", "detect": "bun", "executable": "bun", "minimumVersion": "1.3.14", "adapter": "🍞️bun", "support": {"glob": "native", "toml": "native", "jsonc": "native", "yaml": "native", "which": "native", "resolve": "native", "sleep": "native", "spawn": "standard", "stdin": "native", "file": "native", "serve": "native", "sqlite": "native", "ffi": "native", "transpile": "native", "bundle": "native", "semver": "native", "identity": "native", "gc": "native", "plugin": "native", "test": "native"}, "builtins": {"prefixes": ["bun:"], "names": ["bun"]}, "typeEnvironment": "bun", "sourceUrl": "blob", "product": "Bun", "packageManager": {"executable": "bun", "add": ["add", "--no-save"], "exec": ["x"]}}, {"id": "node", "detect": "node", "executable": "node", "minimumVersion": "24.2.0", "adapter": "🟢️node", "support": {"glob": "portable", "toml": "portable", "jsonc": "portable", "yaml": "portable", "which": "portable", "resolve": "portable", "sleep": "standard", "spawn": "standard", "stdin": "standard", "file": "standard", "serve": "standard", "sqlite": "standard", "ffi": "provider", "transpile": "portable", "bundle": "provider", "semver": "portable", "identity": "native", "gc": "standard", "plugin": "standard", "test": "portable"}, "builtins": {"prefixes": ["node:"], "names": []}, "typeEnvironment": "node", "sourceUrl": "data", "product": "Node.js", "packageManager": {"executable": "npm", "add": ["install", "--no-save"], "exec": ["exec", "--"]}}]} as const;

function leaf(): typeof import("../🗣️messages/🟦️.ts") { return createRequire(import.meta.url)("../🗣️messages/🟦️.ts"); }

/** 🗣️ Renders one catalog message; the catalog module is loaded on the first failure only, so a process that never fails never parses it. */
export function runtimeMessageV1(id: string, parameters: Readonly<Record<string, string | number>>): string {
  try { return leaf().runtimeMessageV1(id, parameters); } catch { return `${id} ${JSON.stringify(parameters)}`; }
}

const FAMILY_OF: Readonly<Record<Exclude<keyof RuntimeAdapterV1, "id">, RuntimeFamilyV1>> = Object.freeze({ identity: "identity", glob: "glob", toml: "toml", jsonc: "jsonc", yaml: "yaml", which: "which", resolve: "resolve", sleep: "sleep", spawn: "spawn", spawnSync: "spawn", stdinText: "stdin", stdinStream: "stdin", file: "file", serve: "serve", sqlite: "sqlite", ffiOpen: "ffi", ffiRead: "ffi", ffiPointer: "ffi", ffiToArrayBuffer: "ffi", ffiCallback: "ffi", transpiler: "transpile", bundle: "bundle", loaderPlugin: "plugin", testApi: "test", testCommand: "test", testPreload: "test", semverSatisfies: "semver", semverOrder: "semver", gc: "gc" });
const loaded = new Map<RuntimeIdV1, RuntimeAdapterV1>();

/** 🔭️ Detects the hosting runtime from the registry's detection keys, first row first. */
export function detectRuntimeIdV1(): RuntimeIdV1 {
  for (const row of RUNTIME_REGISTRY_V1.runtimes) if (row.detect in process.versions) return row.id as RuntimeIdV1;
  throw new Error(runtimeMessageV1("hostUnknown", {}));
}

function adapterOf(id: RuntimeIdV1): RuntimeAdapterV1 {
  const known = loaded.get(id);
  if (known !== undefined) return known;
  const load = createRequire(import.meta.url);
  const adapter = id === "bun" ? (load("../🍞️bun/🟦️.ts") as typeof import("../🍞️bun/🟦️.ts")).bunRuntimeAdapterV1 : id === "node" ? (load("../🟢️node/🟦️.ts") as typeof import("../🟢️node/🟦️.ts")).nodeRuntimeAdapterV1 : undefined;
  if (adapter === undefined) throw new Error(runtimeMessageV1("unknownRuntime", { runtime: String(id) }));
  loaded.set(id, adapter);
  return adapter;
}

/** 🔌️ Returns the adapter of one named runtime, loading it and its portable stack on first request so a process parses only the adapters it uses. */
export async function loadRuntimeAdapterV1(id: RuntimeIdV1): Promise<RuntimeAdapterV1> { return adapterOf(id); }

let hostId: RuntimeIdV1 | undefined;
/** 🧭️ Names the hosting runtime without loading its adapter. */
export function runtimeHostIdV1(): RuntimeIdV1 { return hostId ??= detectRuntimeIdV1(); }

/** 🎯️ Returns the adapter of the hosting runtime, loaded on the first call that needs a member. */
export function runtimeAdapterV1(): RuntimeAdapterV1 { return adapterOf(runtimeHostIdV1()); }

/** 🪪️ Identifies the hosting runtime from the registry and the process alone; only the revision needs the adapter, so it is read on demand. */
export function runtimeHostIdentityV1(): RuntimeIdentityV1 {
  const id = runtimeHostIdV1(), row = RUNTIME_REGISTRY_V1.runtimes.find(candidate => candidate.id === id)!, version = (process.versions as Readonly<Record<string, string>>)[row.detect]!;
  return { id, product: row.product, version, get revision(): string { return adapterOf(id).identity().revision; }, key: `${id}:${version}`, executable: process.execPath };
}

/** 🔌️ Returns an adapter that was already loaded, the host's own or one a test loaded. */
export function selectRuntimeAdapterV1(id: RuntimeIdV1): RuntimeAdapterV1 {
  const adapter = loaded.get(id);
  if (adapter === undefined) throw new Error(runtimeMessageV1("adapterNotLoaded", { runtime: id }));
  return adapter;
}

/** 📊️ States how a runtime provides a family, from the registry and not from the adapter object. */
export function runtimeSupportV1(family: RuntimeFamilyV1, id: RuntimeIdV1 = runtimeHostIdV1()): RuntimeSupportV1 {
  const row = RUNTIME_REGISTRY_V1.runtimes.find(candidate => candidate.id === id);
  return (row?.support as Readonly<Record<string, RuntimeSupportV1>> | undefined)?.[family] ?? "missing";
}

/** 🧲️ Fetches one adapter member or refuses with the capability error naming its family. */
export function requireRuntimeMemberV1<Name extends Exclude<keyof RuntimeAdapterV1, "id">>(name: Name): NonNullable<RuntimeAdapterV1[Name]> {
  const adapter = runtimeAdapterV1(), member = adapter[name];
  if (member === undefined) throw new (leaf().RuntimeCapabilityMissingV1)(FAMILY_OF[name], adapter.id);
  return member as NonNullable<RuntimeAdapterV1[Name]>;
}
