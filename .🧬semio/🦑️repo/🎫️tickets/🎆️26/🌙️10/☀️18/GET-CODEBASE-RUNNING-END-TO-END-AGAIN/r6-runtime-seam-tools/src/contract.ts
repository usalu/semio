/** 🏷️ Names one JavaScript runtime that can host the framework; the registry rows live in the schema and every type in this module is ours. */
export type RuntimeIdV1 = "bun" | "node";

/** 📚️ Names one runtime-dependent capability family; neutral code asks for a family, never for a runtime. */
export type RuntimeFamilyV1 = "glob" | "toml" | "jsonc" | "yaml" | "which" | "resolve" | "sleep" | "spawn" | "stdin" | "file" | "serve" | "sqlite" | "ffi" | "transpile" | "bundle" | "semver" | "identity" | "gc" | "plugin" | "test";

/** 🧭️ States how an adapter provides a family: its own native API, a standard API, our portable code, or not at all. */
export type RuntimeSupportV1 = "native" | "standard" | "portable" | "provider" | "missing";

/** 🪪️ Identifies the hosting runtime; `key` is stable text for cache keys and receipts, `executable` is what a parent spawns. */
export interface RuntimeIdentityV1 { readonly id: RuntimeIdV1; readonly product: string; readonly version: string; readonly revision: string; readonly key: string; readonly executable: string }

/** 🌐️ Options of one glob scan. */
export interface RuntimeGlobScanOptionsV1 { readonly cwd?: string; readonly dot?: boolean; readonly onlyFiles?: boolean; readonly absolute?: boolean; readonly followSymlinks?: boolean }
/** 🌐️ One compiled glob pattern. */
export interface RuntimeGlobV1 { match(path: string): boolean; scan(options?: RuntimeGlobScanOptionsV1): AsyncIterableIterator<string>; scanSync(options?: RuntimeGlobScanOptionsV1): IterableIterator<string> }

/** 🔎️ Options of one executable lookup. */
export interface RuntimeWhichOptionsV1 { readonly PATH?: string; readonly cwd?: string }

/** 🚀️ Describes how one standard stream of a child is wired. */
export type RuntimeStdioV1 = "pipe" | "inherit" | "ignore";
/** 🌊️ A web byte stream that is also async-iterable. */
export type RuntimeByteStreamV1 = ReadableStream<Uint8Array> & AsyncIterable<Uint8Array>;
/** 🚀️ Options of one child process; `timeout` is a wall-clock budget in milliseconds. */
export interface RuntimeSpawnOptionsV1 { readonly cwd?: string; readonly env?: Readonly<Record<string, string | undefined>>; readonly stdin?: RuntimeStdioV1 | string | Uint8Array; readonly stdout?: RuntimeStdioV1; readonly stderr?: RuntimeStdioV1; readonly timeout?: number; readonly signal?: AbortSignal; readonly detached?: boolean }
/** 🚀️ One running child; streams are web streams so no runtime stream type leaks. */
export interface RuntimeChildV1 { readonly pid: number; readonly exited: Promise<number>; readonly stdout: RuntimeByteStreamV1; readonly stderr: RuntimeByteStreamV1; readonly exitCode: number | null; kill(signal?: NodeJS.Signals | number): void }
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
/** 🗃️ The effect of one write statement. */
export interface RuntimeSqliteChangesV1 { readonly changes: number; readonly lastInsertRowid: number | bigint }
/** 🗃️ One prepared statement; rows are plain objects or arrays and a missing row is `null`. */
export interface RuntimeSqliteStatementV1<Row = any, Params extends any[] = any[]> { all(...parameters: Params): Row[]; get(...parameters: Params): Row | null; run(...parameters: Params): RuntimeSqliteChangesV1; values(...parameters: Params): any[][]; safeIntegers(enabled?: boolean): this; finalize(): void }
/** 🗃️ One SQLite connection with the closed method set every framework caller uses. */
export interface RuntimeSqliteDatabaseV1 { query<Row = any, Params extends any[] = any[]>(sql: string): RuntimeSqliteStatementV1<Row, Params>; prepare<Row = any, Params extends any[] = any[]>(sql: string): RuntimeSqliteStatementV1<Row, Params>; run(sql: string, ...parameters: any[]): RuntimeSqliteChangesV1; exec(sql: string): unknown; transaction<T extends (...arguments_: any[]) => any>(body: T): T; close(throwOnError?: boolean): void }
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
  toArrayBuffer(pointer: RuntimeNativePointerV1, byteOffset?: number, byteLength?: number): ArrayBuffer;
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
export interface RuntimeBundleBuilderV1 { onResolve(constraints: { filter: RegExp; namespace?: string }, callback: (arguments_: { path: string; importer: string; namespace: string }) => { path: string; namespace?: string; external?: boolean; loader?: string } | undefined | void | Promise<{ path: string; namespace?: string; external?: boolean; loader?: string } | undefined | void>): void; onLoad(constraints: { filter: RegExp; namespace?: string }, callback: (arguments_: { path: string; namespace: string; loader: string }) => { contents?: string | Uint8Array; loader?: string; exports?: Record<string, unknown> } | undefined | void | Promise<{ contents?: string | Uint8Array; loader?: string; exports?: Record<string, unknown> } | undefined | void>): void }
/** 📦️ One bundler plugin. */
export interface RuntimeBundlePluginV1 { readonly name: string; setup(builder: RuntimeBundleBuilderV1): void | Promise<void> }
/** 📦️ Options of one bundle. */
export interface RuntimeBundleOptionsV1 { readonly entrypoints: readonly string[]; readonly root?: string; readonly target?: "runtime" | "node" | "browser"; readonly format?: "esm" | "cjs" | "iife"; readonly define?: Readonly<Record<string, string>>; readonly external?: readonly string[]; readonly minify?: boolean | { whitespace?: boolean; syntax?: boolean; identifiers?: boolean }; readonly splitting?: boolean; readonly treeShaking?: boolean; readonly naming?: string; readonly outdir?: string; readonly packages?: "external" | "bundle"; readonly sourcemap?: "none" | "inline" | "external"; readonly metafile?: boolean; readonly plugins?: readonly RuntimeBundlePluginV1[] }
/** 📦️ One produced bundle file. */
export interface RuntimeBundleOutputV1 { readonly path: string; readonly kind: string; text(): Promise<string>; bytes(): Promise<Uint8Array> }
/** 📦️ The outcome of one bundle; failed builds carry messages in `logs`. */
export interface RuntimeBundleResultV1 { readonly success: boolean; readonly metafile?: { readonly inputs: Readonly<Record<string, unknown>> }; readonly outputs: readonly RuntimeBundleOutputV1[]; readonly logs: readonly { readonly message: string; readonly level: string }[] }

/** 🧪️ Matchers of one expectation; the closed subset the framework's own tests and lanes use. */
export interface RuntimeMatchersV1 {
  readonly not: RuntimeMatchersV1;
  readonly resolves: RuntimeMatchersV1;
  readonly rejects: RuntimeMatchersV1;
  toBe(expected: unknown): void; toEqual(expected: unknown): void; toStrictEqual(expected: unknown): void; toMatchObject(expected: unknown): void;
  toBeTruthy(): void; toBeFalsy(): void; toBeNull(): void; toBeUndefined(): void; toBeDefined(): void; toBeNaN(): void;
  toBeGreaterThan(expected: number | bigint): void; toBeGreaterThanOrEqual(expected: number | bigint): void; toBeLessThan(expected: number | bigint): void; toBeLessThanOrEqual(expected: number | bigint): void; toBeCloseTo(expected: number, digits?: number): void;
  toContain(expected: unknown): void; toContainEqual(expected: unknown): void; toHaveLength(expected: number): void; toHaveProperty(path: string, value?: unknown): void; toMatch(expected: RegExp | string): void; toBeInstanceOf(expected: Function): void;
  toThrow(expected?: RegExp | string | Function): void;
}
/** 🧪️ The completion callback of a test body that reports through it instead of returning. */
export type RuntimeTestDoneV1 = (error?: unknown) => void;
/** 🧪️ One test body: it returns, returns a promise, or reports through a completion callback. */
export type RuntimeTestBodyV1 = (done: RuntimeTestDoneV1) => unknown;
/** 🧪️ Options of one test; a plain number is a timeout in milliseconds. */
export type RuntimeTestOptionsV1 = number | { readonly timeout?: number };
/** 🧪️ Declares tests; `skip`, `only`, `todo`, `skipIf` are the closed variants. */
export interface RuntimeTestFunctionV1 { (name: string, body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1): void; skip(name: string, body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1): void; only(name: string, body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1): void; todo(name: string, body?: RuntimeTestBodyV1): void; skipIf(condition: unknown): RuntimeTestFunctionV1 }
/** 🧪️ The thin test API every runtime's test runner is projected onto. */
export interface RuntimeTestApiV1 {
  readonly test: RuntimeTestFunctionV1;
  readonly describe: (name: string, body: () => void) => void;
  readonly beforeAll: (body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1) => void; readonly afterAll: (body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1) => void; readonly beforeEach: (body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1) => void; readonly afterEach: (body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1) => void;
  readonly expect: (actual: unknown, message?: string) => RuntimeMatchersV1;
}
/** 🧪️ Options of one test-runner invocation. */
export interface RuntimeTestCommandOptionsV1 { readonly filter?: string; readonly timeoutMilliseconds?: number; readonly concurrency?: number }

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
  stdinStream?(): RuntimeByteStreamV1;
  file?(path: string): RuntimeFileV1;
  serve?(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1>;
  sqlite?(): RuntimeSqliteConstructorV1;
  ffiOpen?<Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>>>(path: string, symbols: Symbols): RuntimeNativeLibraryV1<Symbols>;
  ffiRead?(): RuntimeNativeReadV1;
  ffiPointer?(view: ArrayBufferView): RuntimeNativePointerV1;
  ffiToArrayBuffer?(pointer: RuntimeNativePointerV1, byteOffset?: number, byteLength?: number): ArrayBuffer;
  ffiCallback?(body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1): RuntimeNativeCallbackV1;
  transpiler?(options?: RuntimeTranspilerOptionsV1): RuntimeTranspilerV1;
  bundle?(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1>;
  loaderPlugin?(plugin: RuntimeBundlePluginV1): void;
  testApi?(): RuntimeTestApiV1;
  testCommand?(files: readonly string[], options?: RuntimeTestCommandOptionsV1): readonly string[];
  testPreload?(args: readonly string[], preload: string): readonly string[] | undefined;
  semverSatisfies?(version: string, range: string): boolean;
  semverOrder?(left: string, right: string): -1 | 0 | 1;
  gc?(synchronous?: boolean): void;
}
