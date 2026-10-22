import type { RuntimeBundleOptionsV1, RuntimeBundleResultV1, RuntimeBundlePluginV1, RuntimeByteStreamV1, RuntimeChildV1, RuntimeFileV1, RuntimeServeOptionsV1, RuntimeServerV1, RuntimeSpawnOptionsV1, RuntimeSpawnSyncResultV1, RuntimeTestApiV1, RuntimeTestCommandOptionsV1, RuntimeTranspilerOptionsV1, RuntimeTranspilerV1 } from "../../📋️contract/🟦️.ts";

const bun = (): any => (globalThis as { Bun?: unknown }).Bun;
const target = (value: "runtime" | "node" | "browser" | undefined): string | undefined => (value === "runtime" ? "bun" : value);

function stdioOf(value: RuntimeSpawnOptionsV1["stdin"], fallback: string): unknown {
  if (value === undefined) return fallback;
  if (typeof value === "string" && (value === "pipe" || value === "inherit" || value === "ignore")) return value;
  return typeof value === "string" ? Buffer.from(value) : value;
}

function empty(): RuntimeByteStreamV1 { return new ReadableStream<Uint8Array>({ start: controller => controller.close() }) as unknown as RuntimeByteStreamV1; }

export function spawn(command: readonly string[], options: RuntimeSpawnOptionsV1 = {}): RuntimeChildV1 {
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

export function spawnSync(command: readonly string[], options: RuntimeSpawnOptionsV1 = {}): RuntimeSpawnSyncResultV1 {
  const result = bun().spawnSync([...command], { cwd: options.cwd, env: options.env, stdin: stdioOf(options.stdin, "ignore"), stdout: options.stdout ?? "pipe", stderr: options.stderr ?? "pipe", timeout: options.timeout });
  return { exitCode: result.exitCode, signalCode: result.signalCode ?? null, success: result.success, stdout: result.stdout ?? Buffer.alloc(0), stderr: result.stderr ?? Buffer.alloc(0) };
}

export function file(path: string): RuntimeFileV1 {
  const handle = bun().file(path);
  return { path, exists: () => handle.exists(), size: async () => handle.size, text: () => handle.text(), body: () => handle };
}

export async function serve(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1> {
  const server = bun().serve({ hostname: options.hostname, port: options.port, idleTimeout: options.idleTimeoutSeconds, maxRequestBodySize: options.maximumRequestBodyBytes, ...(options.tls === undefined ? {} : { tls: { cert: bun().file(options.tls.certificate), key: bun().file(options.tls.key) } }), fetch: options.fetch });
  return { port: server.port, hostname: server.hostname, stop: async force => { await server.stop(force); } };
}

export function transpiler(options: RuntimeTranspilerOptionsV1 = {}): RuntimeTranspilerV1 {
  const engine = new (bun().Transpiler)({ loader: options.loader, target: target(options.target), define: options.define, deadCodeElimination: options.deadCodeElimination });
  return { transformSync: (source, loader) => engine.transformSync(source, loader), scan: source => engine.scan(source), scanImports: source => engine.scanImports(source) };
}

export async function bundle(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1> {
  const result = await bun().build({ entrypoints: [...options.entrypoints], root: options.root, target: target(options.target), format: options.format, define: options.define, external: options.external === undefined ? undefined : [...options.external], minify: options.minify, splitting: options.splitting, treeShaking: options.treeShaking, naming: options.naming, outdir: options.outdir, packages: options.packages, sourcemap: options.sourcemap, metafile: options.metafile, plugins: options.plugins === undefined ? undefined : [...options.plugins] });
  return { success: result.success, metafile: result.metafile, logs: result.logs.map((row: { message: string; level: string }) => ({ message: row.message, level: row.level })), outputs: result.outputs.map((row: any) => ({ path: row.path, kind: row.kind, text: () => row.text(), bytes: async () => new Uint8Array(await row.arrayBuffer()) })) };
}

export function bunTestApiV1(): RuntimeTestApiV1 {
  const loaded = process.getBuiltinModule("bun:test") as unknown as RuntimeTestApiV1;
  const { test, describe, beforeAll, afterAll, beforeEach, afterEach, expect } = loaded;
  return { test, describe, beforeAll, afterAll, beforeEach, afterEach, expect };
}

export function bunTestCommandV1(files: readonly string[], options: RuntimeTestCommandOptionsV1 = {}): string[] {
  return ["test", ...(options.timeoutMilliseconds === undefined ? [] : ["--timeout", String(options.timeoutMilliseconds)]), ...(options.filter === undefined ? [] : ["-t", options.filter]), ...files];
}

/** 🧪️ Inserts a preload module after the `test` verb; other commands and commands that already preload have no answer. */
export function bunTestPreloadV1(args: readonly string[], preload: string): readonly string[] | undefined {
  return args[0] === "test" && !args.includes("--preload") ? ["test", "--preload", preload, ...args.slice(1)] : undefined;
}

/** 🔌️ Registers a plugin as a runtime module hook of this process. */
export function loaderPlugin(plugin: RuntimeBundlePluginV1): void { bun().plugin(plugin); }
