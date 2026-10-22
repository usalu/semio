import { createRequire } from "node:module";
import type { RuntimeAdapterV1, RuntimeNativeCallbackV1, RuntimeNativeReadV1, RuntimeNativeSymbolV1, RuntimeSqliteConstructorV1 } from "../📋️contract/🟦️.ts";

const bun = (): any => (globalThis as { Bun?: unknown }).Bun;
const ffi = (): any => process.getBuiltinModule("bun:ffi");
const tooling = (): typeof import("./🧩️tooling/🟦️.ts") => createRequire(import.meta.url)("./🧩️tooling/🟦️.ts");

/** 🍞️ The one Bun adapter: the only place that names Bun globals or `bun:` modules; every family is a thin projection onto owned types. */
export const bunRuntimeAdapterV1: RuntimeAdapterV1 = {
  id: "bun",
  identity: () => ({ id: "bun", product: "Bun", version: String(bun().version), revision: String(bun().revision ?? ""), key: `bun:${process.versions.bun}`, executable: process.execPath }),
  glob: pattern => { const glob = new (bun().Glob)(pattern); return { match: path => glob.match(path), scan: options => glob.scan(options), scanSync: options => glob.scanSync(options) }; },
  toml: text => bun().TOML.parse(text),
  jsonc: text => bun().JSONC.parse(text),
  yaml: text => bun().YAML.parse(text),
  which: (command, options) => bun().which(command, options) ?? null,
  resolve: (specifier, from) => bun().resolveSync(specifier, from),
  sleep: milliseconds => bun().sleep(milliseconds),
  spawn: (...arguments_: Parameters<typeof import("./🧩️tooling/🟦️.ts").spawn>) => (tooling().spawn as (...rest: unknown[]) => any)(...arguments_),
  spawnSync: (...arguments_: Parameters<typeof import("./🧩️tooling/🟦️.ts").spawnSync>) => (tooling().spawnSync as (...rest: unknown[]) => any)(...arguments_),
  stdinText: () => bun().stdin.text(),
  stdinStream: () => bun().stdin.stream(),
  file: (...arguments_: Parameters<typeof import("./🧩️tooling/🟦️.ts").file>) => (tooling().file as (...rest: unknown[]) => any)(...arguments_),
  serve: (...arguments_: Parameters<typeof import("./🧩️tooling/🟦️.ts").serve>) => (tooling().serve as (...rest: unknown[]) => any)(...arguments_),
  sqlite: () => (process.getBuiltinModule("bun:sqlite") as unknown as { Database: RuntimeSqliteConstructorV1 }).Database,
  ffiOpen: (path: string, symbols) => {
    const declared: Record<string, { args: readonly string[]; returns: string }> = {};
    for (const [name, symbol] of Object.entries(symbols) as [string, RuntimeNativeSymbolV1][]) declared[name] = { args: symbol.args, returns: symbol.returns };
    const library = ffi().dlopen(path, declared);
    return { symbols: library.symbols, close: () => library.close() };
  },
  ffiRead: (): RuntimeNativeReadV1 => ffi().read,
  ffiPointer: view => ffi().ptr(view),
  ffiToArrayBuffer: (pointer, byteOffset, byteLength) => ffi().toArrayBuffer(pointer, byteOffset, byteLength),
  ffiCallback: (body, signature): RuntimeNativeCallbackV1 => { const callback = new (ffi().JSCallback)(body, { args: [...signature.args], returns: signature.returns, threadsafe: false }); return { pointer: callback.ptr, close: () => callback.close() }; },
  transpiler: (...arguments_: Parameters<typeof import("./🧩️tooling/🟦️.ts").transpiler>) => (tooling().transpiler as (...rest: unknown[]) => any)(...arguments_),
  bundle: (...arguments_: Parameters<typeof import("./🧩️tooling/🟦️.ts").bundle>) => (tooling().bundle as (...rest: unknown[]) => any)(...arguments_),
  loaderPlugin: plugin => tooling().loaderPlugin(plugin),
  testApi: () => tooling().bunTestApiV1(),
  testCommand: (files, options) => tooling().bunTestCommandV1(files, options),
  testPreload: (args, preload) => tooling().bunTestPreloadV1(args, preload),
  semverSatisfies: (version, range) => bun().semver.satisfies(version, range),
  semverOrder: (left, right) => bun().semver.order(left, right),
  gc: synchronous => { bun().gc(synchronous === true); },
};
