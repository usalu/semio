import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { RuntimeCapabilityMissingV1, type RuntimeAdapterV1, type RuntimeFamilyV1, type RuntimeIdV1, type RuntimeSupportV1 } from "../📋️contract/🟦️.ts";
import { bunRuntimeAdapterV1 } from "../🍞️bun/🟦️.ts";
import { nodeRuntimeAdapterV1 } from "../🟢️node/🟦️.ts";

/** 📒️ The closed registry of hostable runtimes, read from the schema. */
export const RUNTIME_REGISTRY_V1 = schema.definitions.Registry.const;

const ADAPTERS: Readonly<Record<RuntimeIdV1, RuntimeAdapterV1>> = Object.freeze({ bun: bunRuntimeAdapterV1, node: nodeRuntimeAdapterV1 });
const FAMILY_OF: Readonly<Record<Exclude<keyof RuntimeAdapterV1, "id">, RuntimeFamilyV1>> = Object.freeze({ identity: "identity", glob: "glob", toml: "toml", jsonc: "jsonc", yaml: "yaml", which: "which", resolve: "resolve", sleep: "sleep", spawn: "spawn", spawnSync: "spawn", stdinText: "stdin", stdinStream: "stdin", file: "file", serve: "serve", sqlite: "sqlite", ffiOpen: "ffi", ffiRead: "ffi", ffiPointer: "ffi", ffiCallback: "ffi", transpiler: "transpile", bundle: "bundle", semverSatisfies: "semver", semverOrder: "semver", main: "main", gc: "gc" });
let active: RuntimeAdapterV1 | undefined;

/** 🔭️ Detects the hosting runtime from the registry's detection keys, first row first. */
export function detectRuntimeIdV1(): RuntimeIdV1 {
  for (const row of RUNTIME_REGISTRY_V1.runtimes) if (row.detect in process.versions) return row.id as RuntimeIdV1;
  throw new Error("Hosting runtime is not in the runtime registry");
}

/** 🔌️ Returns the adapter of one named runtime; used by conformance tests and by the active selection. */
export function selectRuntimeAdapterV1(id: RuntimeIdV1): RuntimeAdapterV1 {
  const adapter = ADAPTERS[id];
  if (adapter === undefined) throw new Error(`Unknown runtime ${String(id)}`);
  return adapter;
}

/** 🎯️ Returns the adapter of the hosting runtime, chosen once per process. */
export function runtimeAdapterV1(): RuntimeAdapterV1 { return (active ??= selectRuntimeAdapterV1(detectRuntimeIdV1())); }

/** 📊️ States how a runtime provides a family, from the registry and not from the adapter object. */
export function runtimeSupportV1(family: RuntimeFamilyV1, id: RuntimeIdV1 = runtimeAdapterV1().id): RuntimeSupportV1 {
  const row = RUNTIME_REGISTRY_V1.runtimes.find(candidate => candidate.id === id);
  return (row?.support as Readonly<Record<string, RuntimeSupportV1>> | undefined)?.[family] ?? "missing";
}

/** 🧲️ Fetches one adapter member or refuses with the capability error naming its family. */
export function requireRuntimeMemberV1<Name extends Exclude<keyof RuntimeAdapterV1, "id">>(name: Name): NonNullable<RuntimeAdapterV1[Name]> {
  const adapter = runtimeAdapterV1(), member = adapter[name];
  if (member === undefined) throw new RuntimeCapabilityMissingV1(FAMILY_OF[name], adapter.id);
  return member as NonNullable<RuntimeAdapterV1[Name]>;
}
