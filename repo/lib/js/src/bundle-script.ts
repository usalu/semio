import { execFileSync, spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { getWorkspaceRoot } from "./cli.ts";
import { dispatchPolicyArgv } from "./policy-cli.ts";

//#region 🔖Script
/** 🧭Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script {
  constructor(
    protected readonly root: string,
    protected readonly repoRoot: string,
  ) {}
  abstract run(segments: string[]): void | Promise<void>;
}

/** 📦Bundle-scoped command with `root` at the package directory. */
export abstract class BundleScript extends Script {
  constructor(bundleRoot: string, repoRoot?: string) {
    super(bundleRoot, repoRoot ?? findRepoRoot(bundleRoot));
  }
}
//#endregion 🔖Script

//#region 🔖Router
export type ScriptCommand = new (root: string, repoRoot: string) => Script;

/** 🧭Declarative subcommand registry for a single `script.ts`. */
export class ScriptRouter {
  private readonly commands = new Map<string, ScriptCommand>();

  constructor(
    readonly bundleRoot: string,
    readonly repoRoot: string = findRepoRoot(bundleRoot),
  ) {}

  /** 📌Registers a subcommand implemented by a `Script` subclass. */
  register(name: string, Command: ScriptCommand): this {
    this.commands.set(name, Command);
    return this;
  }

  /** 📋Human-readable usage line for this router. */
  usage(): string {
    const names = [...this.commands.keys()];
    if (names.length === 0) return "bun ./script.ts policy";
    return `bun ./script.ts <${names.join("|")}> [args…]`;
  }

  /** 📊Whether any subcommands are registered (policy-only bundles may have none). */
  hasCommands(): boolean {
    return this.commands.size > 0;
  }

  /** ▶️Dispatches `segments[0]` to a registered command class. */
  async run(segments: string[]): Promise<void> {
    const name = segments[0];
    if (!name) {
      console.error(`usage: ${this.usage()}`);
      process.exit(1);
    }
    const Command = this.commands.get(name);
    if (!Command) {
      console.error(`unknown command ${JSON.stringify(name)}`);
      console.error(`usage: ${this.usage()}`);
      process.exit(1);
    }
    await Promise.resolve(new Command(this.bundleRoot, this.repoRoot).run(segments.slice(1)));
  }
}

export type RunBundleScriptMainOptions = {
  defaultCommand?: string;
};

/** 🚪Policy-only bundle entry when no other subcommands are registered. */
export async function runPolicyOnlyMain(scriptUrl: string): Promise<void> {
  const segments = process.argv.slice(2);
  if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  console.error("usage: bun ./script.ts policy");
  process.exit(1);
}

/**
 * 🚪Bundle `script.ts` entry: handles optional `policy`, then routes remaining argv through `router`.
 * Export `policy` / `policyFile` from the same file when policy lint applies.
 */
export async function runBundleScriptMain(
  router: ScriptRouter,
  scriptUrl: string,
  opts: RunBundleScriptMainOptions = {},
): Promise<void> {
  let segments = process.argv.slice(2);
  if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  if (opts.defaultCommand && segments.length === 0) {
    segments = [opts.defaultCommand];
  }
  if (!router.hasCommands()) {
    console.error(`usage: ${router.usage()}`);
    process.exit(1);
  }
  await router.run(segments);
}

/** 🚪Workspace root `script.ts` entry (no policy dispatch). */
export async function runWorkspaceScriptMain(router: ScriptRouter): Promise<void> {
  await router.run(process.argv.slice(2));
}

/**
 * 🧭Nested subcommand dispatch inside a `Script.run` implementation.
 * `handlers` keys are the first argv segment; `defaultKey` runs when argv is empty.
 */
export function dispatchSubcommand(
  segments: string[],
  handlers: Record<string, (rest: string[]) => void | Promise<void>>,
  usage: string,
  defaultKey?: string,
): void | Promise<void> {
  const key = segments[0] ?? defaultKey;
  const handler = key ? handlers[key] : undefined;
  if (!handler) {
    console.error(`usage: ${usage}`);
    process.exit(1);
  }
  return handler(segments.slice(1));
}

/** 📁Walks parents until the monorepo root (`nx.json` + workspace `package.json`). */
export function findRepoRoot(start: string): string {
  let dir = start;
  for (let i = 0; i < 32; i++) {
    if (existsSync(join(dir, "nx.json")) && existsSync(join(dir, "package.json"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return getWorkspaceRoot();
}
//#endregion 🔖Router

//#region 🔖Process
/** 🏃Runs a subprocess with inherited stdio; throws on non-zero exit. */
export function runCmd(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
  execFileSync(cmd, args, {
    stdio: "inherit",
    cwd: opts.cwd,
    env: opts.env ?? process.env,
  });
}

/** 🏃Like `runCmd` but ignores failures. */
export function tryRun(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
  try {
    runCmd(cmd, args, opts);
  } catch {
    /* optional */
  }
}

/** 🧰Dev tooling env without IDE-injected node options. */
export function devToolingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  const env = { ...process.env, ...extra };
  delete env.NODE_OPTIONS;
  delete env.VSCODE_INSPECTOR_OPTIONS;
  env.NX_NATIVE_COMMAND_RUNNER ??= "false";
  env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT ??= "false";
  env.NX_TUI ??= "false";
  return env;
}

/** 🥖Runs `bun` with inherited stdio in `cwd`. */
export function runBun(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  runCmd(process.execPath, args, { cwd, env });
}

/** 🥖Runs `bunx` synchronously in `cwd`. */
export function runBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const result = spawnSync(process.execPath, ["x", ...args], { cwd, env, shell: false, stdio: "inherit" });
  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }
  if (result.status !== 0) process.exit(result.status ?? 1);
}

/** 🥖Spawns `bunx` asynchronously; exits with child code. */
export function spawnBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const child = spawn(process.execPath, ["x", ...args], { cwd, env, shell: false, stdio: "inherit" });
  child.on("exit", (code) => process.exit(code ?? 0));
  child.on("error", (error) => {
    console.error(error);
    process.exit(1);
  });
}

/** 🥖Spawns `bun` asynchronously; exits with child code. */
export function spawnBun(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const child = spawn(process.execPath, args, { cwd, env, shell: true, stdio: "inherit" });
  child.on("exit", (code) => process.exit(code ?? 0));
  child.on("error", (error) => {
    console.error(error);
    process.exit(1);
  });
}

/** ▶️Vite dev server with polling-friendly env defaults. */
export function runViteDev(
  bundleRoot: string,
  segments: string[],
  opts: { config: string; portEnv?: string; defaultPort?: string },
): void {
  const env = playPollingEnv();
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173";
  spawnBun(
    ["run", "vite", "--config", opts.config, "--host", host, "--port", port, ...segments],
    bundleRoot,
    env,
  );
}

/** ▶️Vite production build. */
export function runViteBuild(bundleRoot: string, segments: string[], config: string): void {
  runBun(["run", "vite", "build", "--config", config, ...segments], bundleRoot, devToolingEnv());
}

/** ▶️Vitest run in bundle directory. */
export function runVitest(bundleRoot: string, segments: string[], config = "vitest.config.ts"): void {
  runBunx(["vitest", "run", "--config", config, "--passWithNoTests", ...segments], bundleRoot, devToolingEnv());
}

/** 🧰Play/vite dev env with optional file-watcher polling defaults. */
export function playPollingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  return devToolingEnv({
    ...(process.env.WATCHPACK_POLLING !== undefined
      ? {}
      : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
    ...extra,
  });
}

/** ▶️Playwright test run in bundle directory. */
export function runPlaywright(bundleRoot: string, config: string, segments: string[] = []): void {
  runBunx(["playwright", "test", "--config", config, ...segments], bundleRoot, playPollingEnv());
}

/** ▶️Vite dev via `bunx` with root-level `vite.config.ts`. */
export function runViteBunxDev(
  bundleRoot: string,
  segments: string[],
  opts: { portEnv?: string; defaultPort?: string; clearViteCache?: boolean } = {},
): void {
  if (opts.clearViteCache) {
    const viteCache = join(bundleRoot, "node_modules", ".vite");
    if (existsSync(viteCache)) rmSync(viteCache, { recursive: true, force: true });
  }
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173";
  spawnBunx(["vite", "--config", "vite.config.ts", "--host", host, "--port", port, ...segments], bundleRoot, playPollingEnv());
}

/** ▶️Vite dev via `bunx` without a fixed config path (extra args only). */
export function runViteBunxDevPlain(bundleRoot: string, segments: string[]): void {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  spawnBunx(["vite", "--host", host, ...segments], bundleRoot, playPollingEnv());
}

/** 🦀Runs `cargo` with inherited stdio. */
export function runCargo(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  runCmd("cargo", args, { cwd, env });
}

export type WasmPackWebPkg = {
  name: string;
  version?: string;
  files: string[];
  main: string;
  module: string;
  types: string;
  sideEffects?: string[];
};

/** 📦`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
export function runWasmPackWebBuild(opts: {
  rsDir: string;
  skipEnvVar: string;
  logPrefix: string;
  pkg: WasmPackWebPkg;
  wasmBaseName: string;
}): void {
  const { rsDir, skipEnvVar, logPrefix, pkg, wasmBaseName } = opts;
  if (process.env[skipEnvVar] === "1") {
    console.log(`[${logPrefix}] ${skipEnvVar}=1 → skipping wasm-pack build`);
    return;
  }
  console.log(`[${logPrefix}] wasm-pack build --release --target web --out-dir pkg --no-pack`);
  const t0 = Date.now();
  const res = spawnSync(
    "bun",
    ["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"],
    { cwd: rsDir, stdio: "inherit" },
  );
  if (res.status !== 0) {
    console.error(`[${logPrefix}] wasm-pack build failed`);
    process.exit(res.status ?? 1);
  }
  console.log(`[${logPrefix}] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

  const pkgDir = join(rsDir, "pkg");
  if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
  const pkgJson = {
    type: "module",
    version: pkg.version ?? "0.1.0",
    sideEffects: pkg.sideEffects ?? ["./snippets/*"],
    ...pkg,
  };
  writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
  if (existsSync(wasmPath)) {
    const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
    console.log(`[${logPrefix}] pkg/${wasmBaseName}_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
  } else {
    console.error(`[${logPrefix}] expected wasm output missing: ${wasmPath}`);
    process.exit(1);
  }
}

/** 🔗Resolves `import.meta.url` of the bundle `script.ts`. */
export function scriptPathFromUrl(scriptUrl: string): string {
  return fileURLToPath(scriptUrl);
}
//#endregion 🔖Process
