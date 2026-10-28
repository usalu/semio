import { RUNTIME_REGISTRY_V1, requireRuntimeMemberV1, runtimeHostIdV1, runtimeHostIdentityV1, runtimeSupportV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeFamilyV1, RuntimeIdV1, RuntimeIdentityV1, RuntimeSupportV1 } from "../📋️contract/🟦️.ts";

/** 🪪️ Identifies the hosting runtime. */
export function runtimeIdentityV1(): RuntimeIdentityV1 { return runtimeHostIdentityV1(); }
interface HostRowV1 { readonly id: string; readonly executable: string; readonly adapter: string }
const hostRow = (): HostRowV1 => (RUNTIME_REGISTRY_V1.runtimes as readonly HostRowV1[]).find(row => row.id === runtimeHostIdV1())!;
/** 🏃️ Gives the executable that starts a script in the hosting runtime. */
export function runtimeExecutableV1(): string { return runtimeIdentityV1().executable; }
/** 🔎️ Tells whether a command word names a registered runtime executable or the one running now. */
export function isRuntimeExecutableV1(command: string): boolean {
  return command === process.execPath || RUNTIME_REGISTRY_V1.runtimes.some(row => command === row.executable || command === `${row.executable}.exe`);
}
/** 🧭️ Resolves the program word of declared data: the hosting runtime's registry name becomes the executable running now, every other word stays as declared. */
export function runtimeProgramV1(program: string): string { return program === hostRow().executable ? runtimeIdentityV1().executable : program; }
/** 💬️ Tells whether a command names the hosting runtime: its registry name, with or without a directory and `.exe`, or the executable running now. */
export function isHostRuntimeCommandV1(command: string): boolean {
  if (command === process.execPath || (command.split(/[\\/]/u).pop() ?? "").replace(/\.exe$/iu, "") === hostRow().executable) return true;
  const path = process.getBuiltinModule("node:path");
  return command.includes(path.sep) && path.resolve(command) === path.resolve(process.execPath);
}
/** 🏷️ Gives the bare command word the shell resolves to the hosting runtime. */
export function runtimeCommandNameV1(): string { return hostRow().executable; }
/** 📋️ Lists the identifiers of every registered runtime in registry order; they double as package export condition names. */
export function runtimeIdsV1(): RuntimeIdV1[] { return (RUNTIME_REGISTRY_V1.runtimes as readonly HostRowV1[]).map(row => row.id as RuntimeIdV1); }
/** 🗂️ Lists the bare command words of every registered runtime in registry order. */
export function runtimeCommandNamesV1(): string[] { return (RUNTIME_REGISTRY_V1.runtimes as readonly HostRowV1[]).map(row => row.executable); }
/** 🍞️ Names the directory that holds the hosting runtime's adapter-specific code next to a module, such as a preload. */
export function runtimeAdapterNameV1(): string { return hostRow().adapter; }
/** 🔤️ Lists the names of the global namespace objects of every registered runtime in registry order: the detection key with a capital first letter, which source analyzers need to recognise runtime members in foreign code. */
export function runtimeNamespaceNamesV1(): string[] { return (RUNTIME_REGISTRY_V1.runtimes as readonly { detect: string }[]).map(row => row.detect.charAt(0).toUpperCase() + row.detect.slice(1)); }
/** 🔑️ Gives stable identity text for cache keys, such as `bun:1.3.14`. */
export function runtimeKeyV1(): string { return runtimeIdentityV1().key; }
/** 🧬️ Names the TypeScript `--types` environment that matches the hosting runtime. */
export function runtimeTypeEnvironmentV1(): string { return (RUNTIME_REGISTRY_V1.runtimes as readonly { id: string; typeEnvironment: string }[]).find(row => row.id === runtimeHostIdV1())!.typeEnvironment; }
/** ❓️ Tells whether the hosting runtime provides a family at all. */
export function runtimeSupportsV1(family: RuntimeFamilyV1): boolean { return runtimeSupportV1(family) !== "missing"; }
/** 📢️ Tells how the hosting runtime provides a family. */
export function runtimeSupportLevelV1(family: RuntimeFamilyV1): RuntimeSupportV1 { return runtimeSupportV1(family); }
/** 🧱️ Tells whether a specifier names a module that some registered runtime provides itself; it is a fact about source text, so every registry row counts, not only the host. */
export function isRuntimeBuiltinSpecifierV1(specifier: string): boolean {
  return RUNTIME_REGISTRY_V1.runtimes.some(row => (row.builtins.names as readonly string[]).includes(specifier) || row.builtins.prefixes.some(prefix => specifier.startsWith(prefix)));
}
/** 🚀️ Gives the executable that starts a script in the named runtime: the running one for the host, else the registry's executable name. */
export function runtimeExecutableForV1(id: RuntimeIdV1): string {
  return runtimeHostIdV1() === id ? process.execPath : ((RUNTIME_REGISTRY_V1.runtimes as readonly { id: string; executable: string }[]).find(row => row.id === id)?.executable ?? id);
}
/** 📛️ Lists the names of modules that registered runtimes provide under a bare name. */
export function runtimeBuiltinNamesV1(): string[] { return RUNTIME_REGISTRY_V1.runtimes.flatMap(row => [...row.builtins.names] as string[]); }
/** 🔭️ Names the `process.versions` key that identifies the hosting runtime. */
export function runtimeDetectKeyV1(): string { return (RUNTIME_REGISTRY_V1.runtimes as readonly { id: string; detect: string }[]).find(row => row.id === runtimeHostIdV1())!.detect; }
/** 📦️ Builds the command that runs a package-manager operation (`add` or `exec`) of the hosting runtime's ecosystem. */
export function runtimePackageCommandV1(operation: "add" | "exec", items: readonly string[]): readonly [string, string[]] {
  const row = (RUNTIME_REGISTRY_V1.runtimes as readonly { id: string; packageManager: { executable: string; add: readonly string[]; exec: readonly string[] } }[]).find(candidate => candidate.id === runtimeHostIdV1())!.packageManager;
  return [row.executable === runtimeHostIdV1() ? runtimeExecutableV1() : row.executable, [...(operation === "add" ? row.add : row.exec), ...items]];
}
/** 🧹️ Hints a garbage collection. */
export function runtimeCollectGarbageV1(synchronous?: boolean): void { requireRuntimeMemberV1("gc")(synchronous); }
