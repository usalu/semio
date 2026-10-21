import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { RuntimeCapabilityMissingV1 } from "../🚫️missing/🟦️.ts";
import type { RuntimeAdapterV1, RuntimeFamilyV1, RuntimeIdV1, RuntimeSupportV1 } from "../📋️contract/🟦️.ts";

/** 📒️ The closed registry of hostable runtimes, read from the schema. */
export const RUNTIME_REGISTRY_V1 = schema.definitions.Registry.const;
export { RuntimeCapabilityMissingV1 };

const FAMILY_OF: Readonly<Record<Exclude<keyof RuntimeAdapterV1, "id">, RuntimeFamilyV1>> = Object.freeze({ identity: "identity", glob: "glob", toml: "toml", jsonc: "jsonc", yaml: "yaml", which: "which", resolve: "resolve", sleep: "sleep", spawn: "spawn", spawnSync: "spawn", stdinText: "stdin", stdinStream: "stdin", file: "file", serve: "serve", sqlite: "sqlite", ffiOpen: "ffi", ffiRead: "ffi", ffiPointer: "ffi", ffiToArrayBuffer: "ffi", ffiCallback: "ffi", transpiler: "transpile", bundle: "bundle", loaderPlugin: "plugin", testApi: "test", testCommand: "test", semverSatisfies: "semver", semverOrder: "semver", main: "main", gc: "gc" });
const loaded = new Map<RuntimeIdV1, RuntimeAdapterV1>();

/** 🔭️ Detects the hosting runtime from the registry's detection keys, first row first. */
export function detectRuntimeIdV1(): RuntimeIdV1 {
  for (const row of RUNTIME_REGISTRY_V1.runtimes) if (row.detect in process.versions) return row.id as RuntimeIdV1;
  throw new Error("Hosting runtime is not in the runtime registry");
}

/** 🔌️ Loads the adapter of one named runtime on first request; a bundler sees each import as a separate chunk and a host never parses the adapters it does not use. */
export async function loadRuntimeAdapterV1(id: RuntimeIdV1): Promise<RuntimeAdapterV1> {
  const known = loaded.get(id);
  if (known !== undefined) return known;
  const adapter = id === "bun" ? (await import("../🍞️bun/🟦️.ts")).bunRuntimeAdapterV1 : id === "node" ? (await import("../🟢️node/🟦️.ts")).nodeRuntimeAdapterV1 : undefined;
  if (adapter === undefined) throw new Error(`Unknown runtime ${String(id)}`);
  loaded.set(id, adapter);
  return adapter;
}

const active: RuntimeAdapterV1 = await loadRuntimeAdapterV1(detectRuntimeIdV1());

/** 🎯️ Returns the adapter of the hosting runtime. */
export function runtimeAdapterV1(): RuntimeAdapterV1 { return active; }

/** 🔌️ Returns an adapter that was already loaded, the host's own or one a test loaded. */
export function selectRuntimeAdapterV1(id: RuntimeIdV1): RuntimeAdapterV1 {
  const adapter = loaded.get(id);
  if (adapter === undefined) throw new Error(`Runtime adapter ${id} is not loaded; await loadRuntimeAdapterV1 first`);
  return adapter;
}

/** 📊️ States how a runtime provides a family, from the registry and not from the adapter object. */
export function runtimeSupportV1(family: RuntimeFamilyV1, id: RuntimeIdV1 = active.id): RuntimeSupportV1 {
  const row = RUNTIME_REGISTRY_V1.runtimes.find(candidate => candidate.id === id);
  return (row?.support as Readonly<Record<string, RuntimeSupportV1>> | undefined)?.[family] ?? "missing";
}

/** 🧲️ Fetches one adapter member or refuses with the capability error naming its family. */
export function requireRuntimeMemberV1<Name extends Exclude<keyof RuntimeAdapterV1, "id">>(name: Name): NonNullable<RuntimeAdapterV1[Name]> {
  const member = active[name];
  if (member === undefined) throw new RuntimeCapabilityMissingV1(FAMILY_OF[name], active.id);
  return member as NonNullable<RuntimeAdapterV1[Name]>;
}
