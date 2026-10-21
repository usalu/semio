import type { RuntimeAdapterV1, RuntimeByteStreamV1, RuntimeBundleOptionsV1, RuntimeBundleResultV1, RuntimeChildV1, RuntimeFileV1, RuntimeNativeCallbackV1, RuntimeNativeReadV1, RuntimeNativeSymbolV1, RuntimeServeOptionsV1, RuntimeServerV1, RuntimeSpawnOptionsV1, RuntimeSpawnSyncResultV1, RuntimeSqliteConstructorV1, RuntimeTranspilerOptionsV1, RuntimeTranspilerV1 } from "../📋️contract/🟦️.ts";
import { bunTestApiV1, bunTestCommandV1 } from "./🧪️test/🟦️.ts";
import { isMainModuleV1 } from "../🪶️portable/🏁️main/🟦️.ts";

const bun = (): any => (globalThis as { Bun?: unknown }).Bun;
const ffi = (): any => process.getBuiltinModule("bun:ffi");
const target = (value: "runtime" | "node" | "browser" | undefined): string | undefined => (value === "runtime" ? "bun" : value);

function stdioOf(value: RuntimeSpawnOptionsV1["stdin"], fallback: string): unknown {
  if (value === undefined) return fallback;
  if (typeof value === "string" && (value === "pipe" || value === "inherit" || value === "ignore")) return value;
  return typeof value === "string" ? Buffer.from(value) : value;
}

function empty(): RuntimeByteStreamV1 { return new ReadableStream<Uint8Array>({ start: controller => controller.close() }) as unknown as RuntimeByteStreamV1; }

function spawn(command: readonly string[], options: RuntimeSpawnOptionsV1 = {}): RuntimeChildV1 {
  const child = bun().spawn([...command], { cwd: options.cwd, env: options.env, stdin: stdioOf(options.stdin, "ignore"), stdout: options.stdout ?? "pipe", stderr: options.stderr ?? "inherit", timeout: options.timeout, signal: options.signal, detached: options.detached });
  return {
    pid: child.pid,
    exited: child.exited,
    stdout: child.stdout instanceof ReadableStream ? child.stdout : empty(),
    stderr: child.stderr instanceof ReadableStream ? child.stderr : empty(),
    get exitCode() { return child.exitCode; },
    kill: signal => { child.kill(signal); },
  };
}

function spawnSync(command: readonly string[], options: RuntimeSpawnOptionsV1 = {}): RuntimeSpawnSyncResultV1 {
  const result = bun().spawnSync([...command], { cwd: options.cwd, env: options.env, stdin: stdioOf(options.stdin, "ignore"), stdout: options.stdout ?? "pipe", stderr: options.stderr ?? "pipe", timeout: options.timeout });
  return { exitCode: result.exitCode, signalCode: result.signalCode ?? null, success: result.success, stdout: result.stdout ?? Buffer.alloc(0), stderr: result.stderr ?? Buffer.alloc(0) };
}

function file(path: string): RuntimeFileV1 {
  const handle = bun().file(path);
  return { path, exists: () => handle.exists(), size: async () => handle.size, text: () => handle.text(), body: () => handle };
}

async function serve(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1> {
  const server = bun().serve({ hostname: options.hostname, port: options.port, idleTimeout: options.idleTimeoutSeconds, maxRequestBodySize: options.maximumRequestBodyBytes, fetch: options.fetch });
  return { port: server.port, hostname: server.hostname, stop: async force => { await server.stop(force); } };
}

function transpiler(options: RuntimeTranspilerOptionsV1 = {}): RuntimeTranspilerV1 {
  const engine = new (bun().Transpiler)({ loader: options.loader, target: target(options.target), define: options.define, deadCodeElimination: options.deadCodeElimination });
  return { transformSync: (source, loader) => engine.transformSync(source, loader), scan: source => engine.scan(source), scanImports: source => engine.scanImports(source) };
}

async function bundle(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1> {
  const result = await bun().build({ entrypoints: [...options.entrypoints], root: options.root, target: target(options.target), format: options.format, define: options.define, external: options.external === undefined ? undefined : [...options.external], minify: options.minify, splitting: options.splitting, treeShaking: options.treeShaking, naming: options.naming, outdir: options.outdir, packages: options.packages, sourcemap: options.sourcemap, metafile: options.metafile, plugins: options.plugins === undefined ? undefined : [...options.plugins] });
  return { success: result.success, metafile: result.metafile, logs: result.logs.map((row: { message: string; level: string }) => ({ message: row.message, level: row.level })), outputs: result.outputs.map((row: any) => ({ path: row.path, kind: row.kind, text: () => row.text(), bytes: async () => new Uint8Array(await row.arrayBuffer()) })) };
}

/** 🍞️ The one Bun adapter: the only place that names Bun globals or `bun:` modules; every family is a thin projection onto owned types. */
export const bunRuntimeAdapterV1: RuntimeAdapterV1 = {
  id: "bun",
  identity: () => ({ id: "bun", version: String(bun().version), revision: String(bun().revision ?? ""), key: `bun:${process.versions.bun}`, executable: process.execPath }),
  glob: pattern => { const glob = new (bun().Glob)(pattern); return { match: path => glob.match(path), scan: options => glob.scan(options), scanSync: options => glob.scanSync(options) }; },
  toml: text => bun().TOML.parse(text),
  jsonc: text => bun().JSONC.parse(text),
  yaml: text => bun().YAML.parse(text),
  which: (command, options) => bun().which(command, options) ?? null,
  resolve: (specifier, from) => bun().resolveSync(specifier, from),
  sleep: milliseconds => bun().sleep(milliseconds),
  spawn,
  spawnSync,
  stdinText: () => bun().stdin.text(),
  stdinStream: () => bun().stdin.stream(),
  file,
  serve,
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
  transpiler,
  bundle,
  loaderPlugin: plugin => { bun().plugin(plugin); },
  testApi: bunTestApiV1,
  testCommand: bunTestCommandV1,
  semverSatisfies: (version, range) => bun().semver.satisfies(version, range),
  semverOrder: (left, right) => bun().semver.order(left, right),
  main: isMainModuleV1,
  gc: synchronous => { bun().gc(synchronous === true); },
};
