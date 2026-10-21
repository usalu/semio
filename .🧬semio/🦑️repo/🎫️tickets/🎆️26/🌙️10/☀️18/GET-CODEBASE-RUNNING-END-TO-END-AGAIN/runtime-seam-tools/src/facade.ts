import type { RuntimeBundlePluginV1, RuntimeTestCommandOptionsV1, RuntimeByteStreamV1, RuntimeBundleOptionsV1, RuntimeBundleResultV1, RuntimeChildV1, RuntimeFamilyV1, RuntimeFileV1, RuntimeGlobScanOptionsV1, RuntimeGlobV1, RuntimeIdentityV1, RuntimeLoaderV1, RuntimeNativeCallbackV1, RuntimeNativeLibraryV1, RuntimeNativePointerV1, RuntimeNativeReadV1, RuntimeNativeSymbolV1, RuntimeNativeSystemV1, RuntimeNativeTypeV1, RuntimeServeOptionsV1, RuntimeServerV1, RuntimeSpawnOptionsV1, RuntimeSpawnSyncResultV1, RuntimeSqliteDatabaseV1, RuntimeSqliteOptionsV1, RuntimeSupportV1, RuntimeTranspilerOptionsV1, RuntimeTranspilerV1, RuntimeWhichOptionsV1, RuntimeImportRecordV1 } from "./📋️contract/🟦️.ts";
export { RuntimeCapabilityMissingV1 } from "./🚫️missing/🟦️.ts";
import { RUNTIME_REGISTRY_V1, requireRuntimeMemberV1, runtimeAdapterV1, runtimeSupportV1 } from "./🧭️selection/🟦️.ts";

export type * from "./📋️contract/🟦️.ts";
export { RUNTIME_REGISTRY_V1, detectRuntimeIdV1, loadRuntimeAdapterV1, runtimeSupportV1, selectRuntimeAdapterV1 } from "./🧭️selection/🟦️.ts";

/** 🪪️ Identifies the hosting runtime. */
export function runtimeIdentityV1(): RuntimeIdentityV1 { return runtimeAdapterV1().identity(); }
/** 🏃️ Gives the executable that starts a script in the hosting runtime. */
export function runtimeExecutableV1(): string { return runtimeIdentityV1().executable; }
/** 🔎️ Tells whether a command word names a registered runtime executable or the one running now. */
export function isRuntimeExecutableV1(command: string): boolean {
  return command === process.execPath || RUNTIME_REGISTRY_V1.runtimes.some(row => command === row.executable || command === `${row.executable}.exe`);
}
/** 🔑️ Gives stable identity text for cache keys, such as `bun:1.3.14`. */
export function runtimeKeyV1(): string { return runtimeIdentityV1().key; }
/** 🏷️ Names the TypeScript `--types` environment that matches the hosting runtime. */
export function runtimeTypeEnvironmentV1(): string { return RUNTIME_REGISTRY_V1.runtimes.find(row => row.id === runtimeAdapterV1().id)!.typeEnvironment; }
/** ❓️ Tells whether the hosting runtime provides a family at all. */
export function runtimeSupportsV1(family: RuntimeFamilyV1): boolean { return runtimeSupportV1(family) !== "missing"; }
/** 🏷️ Tells how the hosting runtime provides a family. */
export function runtimeSupportLevelV1(family: RuntimeFamilyV1): RuntimeSupportV1 { return runtimeSupportV1(family); }

/** 📄️ Parses TOML text. */
export function runtimeParseTomlV1(text: string): any { return requireRuntimeMemberV1("toml")(text); }
/** 📝️ Parses JSON with comments. */
export function runtimeParseJsoncV1(text: string): any { return requireRuntimeMemberV1("jsonc")(text); }
/** 📃️ Parses YAML text. */
export function runtimeParseYamlV1(text: string): any { return requireRuntimeMemberV1("yaml")(text); }
/** 🔎️ Finds an executable on `PATH`. */
export function runtimeWhichV1(command: string, options?: RuntimeWhichOptionsV1): string | null { return requireRuntimeMemberV1("which")(command, options); }
/** 🧭️ Resolves a module specifier from a directory or file. */
export function runtimeResolveV1(specifier: string, from: string): string { return requireRuntimeMemberV1("resolve")(specifier, from); }
/** 💤️ Waits for a number of milliseconds. */
export function runtimeSleepV1(milliseconds: number): Promise<void> { return requireRuntimeMemberV1("sleep")(milliseconds); }
/** 🚀️ Starts a child process. */
export function runtimeSpawnV1(command: readonly string[], options?: RuntimeSpawnOptionsV1): RuntimeChildV1 { return requireRuntimeMemberV1("spawn")(command, options); }
/** 🏃️ Runs a child process to completion. */
export function runtimeSpawnSyncV1(command: readonly string[], options?: RuntimeSpawnOptionsV1): RuntimeSpawnSyncResultV1 { return requireRuntimeMemberV1("spawnSync")(command, options); }
/** 📥️ Reads standard input as text. */
export function runtimeStdinTextV1(): Promise<string> { return requireRuntimeMemberV1("stdinText")(); }
/** 📥️ Exposes standard input as a byte stream. */
export function runtimeStdinStreamV1(): RuntimeByteStreamV1 { return requireRuntimeMemberV1("stdinStream")(); }
/** 📄️ Describes a file for reading or serving. */
export function runtimeFileV1(path: string): RuntimeFileV1 { return requireRuntimeMemberV1("file")(path); }
/** 🌍️ Serves HTTP. */
export function runtimeServeV1(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1> { return requireRuntimeMemberV1("serve")(options); }
/** 🔌️ Opens a native library. */
export function runtimeNativeOpenV1<Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>>>(path: string, symbols: Symbols): RuntimeNativeLibraryV1<Symbols> { return requireRuntimeMemberV1("ffiOpen")(path, symbols); }
/** 🔌️ Gives typed reads at machine addresses. */
export function runtimeNativeReadV1(): RuntimeNativeReadV1 { return requireRuntimeMemberV1("ffiRead")(); }
/** 🔌️ Gives the address of a buffer view. */
export function runtimeNativePointerV1(view: ArrayBufferView): RuntimeNativePointerV1 { return requireRuntimeMemberV1("ffiPointer")(view); }
/** 🔌️ Creates a callback that foreign code can call. */
export function runtimeNativeCallbackV1(body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1): RuntimeNativeCallbackV1 { return requireRuntimeMemberV1("ffiCallback")(body, signature); }
const NATIVE_TYPES: Readonly<Record<RuntimeNativeTypeV1, RuntimeNativeTypeV1>> = Object.freeze({ void: "void", bool: "bool", i8: "i8", u8: "u8", i16: "i16", u16: "u16", i32: "i32", u32: "u32", i64: "i64", u64: "u64", f32: "f32", f64: "f64", ptr: "ptr", cstring: "cstring" });
/** 🔌️ Gives the whole native-binding surface as one object, for modules that bind many symbols. */
export function runtimeNativeSystemV1(): RuntimeNativeSystemV1 {
  const system: RuntimeNativeSystemV1 = {
    FFIType: NATIVE_TYPES,
    dlopen: runtimeNativeOpenV1,
    read: runtimeNativeReadV1(),
    ptr: runtimeNativePointerV1,
    toArrayBuffer: (pointer, byteOffset, byteLength) => requireRuntimeMemberV1("ffiToArrayBuffer")(pointer, byteOffset, byteLength),
    JSCallback: class { readonly pointer: RuntimeNativePointerV1; private readonly handle: RuntimeNativeCallbackV1; constructor(body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1) { this.handle = runtimeNativeCallbackV1(body, signature); this.pointer = this.handle.pointer; } close(): void { this.handle.close(); } },
  };
  return Object.freeze(system);
}
/** 📦️ Bundles entry points. */
export function runtimeBundleV1(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1> { return requireRuntimeMemberV1("bundle")(options); }
/** 🔢️ Tells whether a version satisfies a range. */
export function runtimeSemverSatisfiesV1(version: string, range: string): boolean { return requireRuntimeMemberV1("semverSatisfies")(version, range); }
/** 🔢️ Orders two versions. */
export function runtimeSemverOrderV1(left: string, right: string): -1 | 0 | 1 { return requireRuntimeMemberV1("semverOrder")(left, right); }
/** 🏁️ Tells whether the module at `metaUrl` is the process entry point. */
export function isMainModuleV1(metaUrl: string): boolean { return requireRuntimeMemberV1("main")(metaUrl); }
/** 🧹️ Hints a garbage collection. */
export function runtimeCollectGarbageV1(synchronous?: boolean): void { requireRuntimeMemberV1("gc")(synchronous); }
/** 🧪️ Builds the arguments that start the hosting runtime's test runner on files; flags in `items` pass through. */
export function runtimeTestArgumentsV1(items: readonly string[], options?: RuntimeTestCommandOptionsV1): string[] { return [...requireRuntimeMemberV1("testCommand")(items, options)]; }
/** 🔌️ Registers a plugin as a module resolution and loading hook of this process. */
export function runtimeLoaderPluginV1(plugin: RuntimeBundlePluginV1): void { requireRuntimeMemberV1("loaderPlugin")(plugin); }
/** 🧱️ Tells whether a specifier names a module that some registered runtime provides itself; it is a fact about source text, so every registry row counts, not only the host. */
export function isRuntimeBuiltinSpecifierV1(specifier: string): boolean {
  return RUNTIME_REGISTRY_V1.runtimes.some(row => row.builtins.names.includes(specifier) || row.builtins.prefixes.some(prefix => specifier.startsWith(prefix)));
}

/** 🌐️ A compiled glob whose implementation is chosen by the hosting runtime. */
export class GlobV1 implements RuntimeGlobV1 {
  private readonly inner: RuntimeGlobV1;
  constructor(pattern: string) { this.inner = requireRuntimeMemberV1("glob")(pattern); }
  match(path: string): boolean { return this.inner.match(path); }
  scan(options?: RuntimeGlobScanOptionsV1): AsyncIterableIterator<string> { return this.inner.scan(options); }
  scanSync(options?: RuntimeGlobScanOptionsV1): IterableIterator<string> { return this.inner.scanSync(options); }
}

/** 🔬️ A source transpiler whose implementation is chosen by the hosting runtime. */
export class TranspilerV1 implements RuntimeTranspilerV1 {
  private readonly inner: RuntimeTranspilerV1;
  constructor(options?: RuntimeTranspilerOptionsV1) { this.inner = requireRuntimeMemberV1("transpiler")(options); }
  transformSync(source: string, loader?: RuntimeLoaderV1): string { return this.inner.transformSync(source, loader); }
  scan(source: string): { readonly exports: string[]; readonly imports: RuntimeImportRecordV1[] } { return this.inner.scan(source); }
  scanImports(source: string): RuntimeImportRecordV1[] { return this.inner.scanImports(source); }
}

/** 🗃️ A SQLite connection whose implementation is chosen by the hosting runtime. */
export interface SqliteDatabaseV1 extends RuntimeSqliteDatabaseV1 {}
export class SqliteDatabaseV1 {
  constructor(path?: string, options?: RuntimeSqliteOptionsV1) { return new (requireRuntimeMemberV1("sqlite")())(path, options) as SqliteDatabaseV1; }
}
