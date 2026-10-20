/** 🏷️ Names one JavaScript runtime that can host the framework; the registry rows live in the schema and every type in this module is ours. */
export type RuntimeIdV1 = "bun" | "node";

/** 📚️ Names one runtime-dependent capability family; neutral code asks for a family, never for a runtime. */
export type RuntimeFamilyV1 = "glob" | "toml" | "jsonc" | "yaml" | "which" | "resolve" | "sleep" | "spawn" | "stdin" | "file" | "serve" | "sqlite" | "ffi" | "transpile" | "bundle" | "semver" | "identity" | "main" | "gc";

/** 🧭️ States how an adapter provides a family: its own native API, a standard API, our portable code, or not at all. */
export type RuntimeSupportV1 = "native" | "standard" | "portable" | "missing";

/** 🪪️ Identifies the hosting runtime; `key` is stable text for cache keys and receipts, `executable` is what a parent spawns. */
export interface RuntimeIdentityV1 { readonly id: RuntimeIdV1; readonly version: string; readonly revision: string; readonly key: string; readonly executable: string }

/** 🚫️ Refusal raised when an adapter does not provide a requested family. */
export class RuntimeCapabilityMissingV1 extends Error {
  readonly family: RuntimeFamilyV1;
  readonly runtime: RuntimeIdV1;
  constructor(family: RuntimeFamilyV1, runtime: RuntimeIdV1) { super(`Runtime ${runtime} lacks capability ${family}`); this.name = "RuntimeCapabilityMissingV1"; this.family = family; this.runtime = runtime; }
}

/** 🌐️ Options of one glob scan. */
export interface RuntimeGlobScanOptionsV1 { readonly cwd?: string; readonly dot?: boolean; readonly onlyFiles?: boolean; readonly absolute?: boolean; readonly followSymlinks?: boolean }
/** 🌐️ One compiled glob pattern. */
export interface RuntimeGlobV1 { match(path: string): boolean; scan(options?: RuntimeGlobScanOptionsV1): AsyncIterableIterator<string>; scanSync(options?: RuntimeGlobScanOptionsV1): IterableIterator<string> }

/** 🔎️ Options of one executable lookup. */
export interface RuntimeWhichOptionsV1 { readonly PATH?: string; readonly cwd?: string }

/** 🚀️ Describes how one standard stream of a child is wired. */
export type RuntimeStdioV1 = "pipe" | "inherit" | "ignore";
/** 🚀️ Options of one child process; `timeout` is a wall-clock budget in milliseconds. */
export interface RuntimeSpawnOptionsV1 { readonly cwd?: string; readonly env?: Readonly<Record<string, string | undefined>>; readonly stdin?: RuntimeStdioV1 | string | Uint8Array; readonly stdout?: RuntimeStdioV1; readonly stderr?: RuntimeStdioV1; readonly timeout?: number; readonly signal?: AbortSignal; readonly detached?: boolean }
/** 🚀️ One running child; streams are web streams so no runtime stream type leaks. */
export interface RuntimeChildV1 { readonly pid: number; readonly exited: Promise<number>; readonly stdout: ReadableStream<Uint8Array>; readonly stderr: ReadableStream<Uint8Array>; readonly exitCode: number | null; kill(signal?: NodeJS.Signals | number): void }
/** 🚀️ The outcome of one synchronous child. */
export interface RuntimeSpawnSyncResultV1 { readonly exitCode: number | null; readonly signalCode: string | null; readonly success: boolean; readonly stdout: Buffer; readonly stderr: Buffer }

/** 🌍️ Options of one HTTP server; the handler speaks web `Request` and `Response`. */
export interface RuntimeServeOptionsV1 { readonly hostname?: string; readonly port?: number; readonly idleTimeoutSeconds?: number; readonly maximumRequestBodyBytes?: number; readonly fetch: (request: Request) => Response | Promise<Response> }
/** 🌍️ One listening HTTP server. */
export interface RuntimeServerV1 { readonly port: number; readonly hostname: string; stop(force?: boolean): Promise<void> }

/** 📄️ A response body that is either a blob or a byte stream. */
export type RuntimeBodyV1 = Blob | ReadableStream<Uint8Array>;
/** 📄️ One file for a response or a read, without a runtime file type. */
export interface RuntimeFileV1 { readonly path: string; exists(): Promise<boolean>; size(): Promise<number>; text(): Promise<string>; body(): RuntimeBodyV1 }

/** 🗃️ Bindable SQLite value. */
export type RuntimeSqliteValueV1 = string | number | bigint | boolean | null | Uint8Array;
/** 🗃️ Options of one SQLite connection. */
export interface RuntimeSqliteOptionsV1 { readonly readonly?: boolean; readonly create?: boolean; readonly readwrite?: boolean }
/** 🗃️ One prepared statement; rows are plain objects or arrays. */
export interface RuntimeSqliteStatementV1 { all(...parameters: unknown[]): any[]; get(...parameters: unknown[]): any; run(...parameters: unknown[]): unknown; values(...parameters: unknown[]): any[][]; finalize(): void }
/** 🗃️ One SQLite connection with the closed method set every framework caller uses. */
export interface RuntimeSqliteDatabaseV1 { query(sql: string): RuntimeSqliteStatementV1; prepare(sql: string): RuntimeSqliteStatementV1; run(sql: string, ...parameters: unknown[]): unknown; exec(sql: string): unknown; transaction<T extends (...arguments_: any[]) => any>(body: T): T; close(throwOnError?: boolean): void }
/** 🗃️ Constructor shape of the connection class a caller may instantiate. */
export interface RuntimeSqliteConstructorV1 { new (path?: string, options?: RuntimeSqliteOptionsV1): RuntimeSqliteDatabaseV1 }

/** 🔌️ Native value types a foreign symbol may declare. */
export type RuntimeNativeTypeV1 = "void" | "bool" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "ptr" | "cstring";
/** 🔌️ Opaque machine address returned by a foreign symbol. */
export type RuntimeNativePointerV1 = number;
/** 🔌️ Declares one foreign symbol. */
export interface RuntimeNativeSymbolV1 { readonly args: readonly RuntimeNativeTypeV1[]; readonly returns: RuntimeNativeTypeV1 }
/** 🔌️ One loaded library; every symbol is a plain function. */
export interface RuntimeNativeLibraryV1<Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>> = Readonly<Record<string, RuntimeNativeSymbolV1>>> { readonly symbols: { readonly [Name in keyof Symbols]: (...arguments_: any[]) => any }; close(): void }
/** 🔌️ Typed reads at a machine address. */
export interface RuntimeNativeReadV1 { i8(pointer: RuntimeNativePointerV1, offset?: number): number; u8(pointer: RuntimeNativePointerV1, offset?: number): number; i16(pointer: RuntimeNativePointerV1, offset?: number): number; u16(pointer: RuntimeNativePointerV1, offset?: number): number; i32(pointer: RuntimeNativePointerV1, offset?: number): number; u32(pointer: RuntimeNativePointerV1, offset?: number): number; i64(pointer: RuntimeNativePointerV1, offset?: number): bigint; u64(pointer: RuntimeNativePointerV1, offset?: number): bigint; ptr(pointer: RuntimeNativePointerV1, offset?: number): RuntimeNativePointerV1 }
/** 🔌️ One native callback handed to foreign code. */
export interface RuntimeNativeCallbackV1 { readonly pointer: RuntimeNativePointerV1; close(): void }

/** 🔌️ The native-binding surface as one object: type names, library loading, typed reads, buffer addresses and callbacks. */
export interface RuntimeNativeSystemV1 {
  readonly FFIType: Readonly<Record<RuntimeNativeTypeV1, RuntimeNativeTypeV1>>;
  dlopen<Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>>>(path: string, symbols: Symbols): RuntimeNativeLibraryV1<Symbols>;
  readonly read: RuntimeNativeReadV1;
  ptr(view: ArrayBufferView): RuntimeNativePointerV1;
  readonly JSCallback: new (body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1) => RuntimeNativeCallbackV1;
}

/** 🔬️ Loader of a source text. */
export type RuntimeLoaderV1 = "js" | "jsx" | "ts" | "tsx";
/** 🔬️ Options of one source transpiler. */
export interface RuntimeTranspilerOptionsV1 { readonly loader?: RuntimeLoaderV1; readonly target?: "runtime" | "node" | "browser"; readonly define?: Readonly<Record<string, string>>; readonly deadCodeElimination?: boolean }
/** 🔬️ One import found in a source text. */
export interface RuntimeImportRecordV1 { readonly path: string; readonly kind: "import-statement" | "require-call" | "require-resolve" | "dynamic-import" | "import-rule" | "url-token" | "internal" | "entry-point-build" | "entry-point-run" }
/** 🔬️ One source transpiler. */
export interface RuntimeTranspilerV1 { transformSync(source: string, loader?: RuntimeLoaderV1): string; scan(source: string): { readonly exports: string[]; readonly imports: RuntimeImportRecordV1[] }; scanImports(source: string): RuntimeImportRecordV1[] }

/** 📦️ One bundler plugin hook set. */
export interface RuntimeBundleBuilderV1 { onResolve(constraints: { filter: RegExp; namespace?: string }, callback: (arguments_: { path: string; importer?: string; namespace?: string }) => { path: string; namespace?: string; external?: boolean } | undefined | void | Promise<{ path: string; namespace?: string; external?: boolean } | undefined | void>): void; onLoad(constraints: { filter: RegExp; namespace?: string }, callback: (arguments_: { path: string; namespace?: string }) => { contents?: string | Uint8Array; loader?: string; exports?: Record<string, unknown> } | undefined | void | Promise<{ contents?: string | Uint8Array; loader?: string; exports?: Record<string, unknown> } | undefined | void>): void }
/** 📦️ One bundler plugin. */
export interface RuntimeBundlePluginV1 { readonly name: string; setup(builder: RuntimeBundleBuilderV1): void | Promise<void> }
/** 📦️ Options of one bundle. */
export interface RuntimeBundleOptionsV1 { readonly entrypoints: readonly string[]; readonly root?: string; readonly target?: "runtime" | "node" | "browser"; readonly format?: "esm" | "cjs" | "iife"; readonly define?: Readonly<Record<string, string>>; readonly external?: readonly string[]; readonly minify?: boolean | { whitespace?: boolean; syntax?: boolean; identifiers?: boolean }; readonly splitting?: boolean; readonly treeShaking?: boolean; readonly naming?: string; readonly outdir?: string; readonly packages?: "external" | "bundle"; readonly sourcemap?: "none" | "inline" | "external"; readonly metafile?: boolean; readonly plugins?: readonly RuntimeBundlePluginV1[] }
/** 📦️ One produced bundle file. */
export interface RuntimeBundleOutputV1 { readonly path: string; readonly kind: string; text(): Promise<string>; bytes(): Promise<Uint8Array> }
/** 📦️ The outcome of one bundle; failed builds carry messages in `logs`. */
export interface RuntimeBundleResultV1 { readonly success: boolean; readonly metafile?: { readonly inputs: Readonly<Record<string, unknown>> }; readonly outputs: readonly RuntimeBundleOutputV1[]; readonly logs: readonly { readonly message: string; readonly level: string }[] }

/** 🔢️ One adapter: every family, with `undefined` for a family the runtime lacks. */
export interface RuntimeAdapterV1 {
  readonly id: RuntimeIdV1;
  identity(): RuntimeIdentityV1;
  glob?(pattern: string): RuntimeGlobV1;
  toml?(text: string): any;
  jsonc?(text: string): any;
  yaml?(text: string): any;
  which?(command: string, options?: RuntimeWhichOptionsV1): string | null;
  resolve?(specifier: string, from: string): string;
  sleep?(milliseconds: number): Promise<void>;
  spawn?(command: readonly string[], options?: RuntimeSpawnOptionsV1): RuntimeChildV1;
  spawnSync?(command: readonly string[], options?: RuntimeSpawnOptionsV1): RuntimeSpawnSyncResultV1;
  stdinText?(): Promise<string>;
  stdinStream?(): ReadableStream<Uint8Array>;
  file?(path: string): RuntimeFileV1;
  serve?(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1>;
  sqlite?(): RuntimeSqliteConstructorV1;
  ffiOpen?<Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>>>(path: string, symbols: Symbols): RuntimeNativeLibraryV1<Symbols>;
  ffiRead?(): RuntimeNativeReadV1;
  ffiPointer?(view: ArrayBufferView): RuntimeNativePointerV1;
  ffiCallback?(body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1): RuntimeNativeCallbackV1;
  transpiler?(options?: RuntimeTranspilerOptionsV1): RuntimeTranspilerV1;
  bundle?(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1>;
  semverSatisfies?(version: string, range: string): boolean;
  semverOrder?(left: string, right: string): -1 | 0 | 1;
  main?(metaUrl: string): boolean;
  gc?(synchronous?: boolean): void;
}
