#!/usr/bin/env bun
/**
 * 🧭 Monorepo command router: `bun ./script.ts <verb> [segments…]` (e.g. `script.ts dev`, `script.ts dev mcp`, `script.ts generate neo4j compose`).
 */
import {
  Script,
  ScriptRouter,
  buildBudgetMs,
  coverageDir,
  coverageEnabled,
  daemonBudgetOpts,
  devToolingEnv,
  dispatchPolicyArgv,
  dispatchSubcommand,
  defineLint,
  enforceCoverageThreshold,
  frameworkOsPlaygroundDevEnv,
  getWorkspaceRoot,
  goCoverageArgs,
  goLevelTestArgs,
  goProfileToLcov,
  loadFrameworkOsPlaygroundCatalog,
  mergeLcov,
  orchestratorBudgetOpts,
  parseLcov,
  renderLcov,
  resolveCliBin,
  resolveFrameworkOsPlaygroundPlugin,
  resolveTestLevel,
  runCmd,
  runCmdStatus,
  runProbe,
  runTestBudgeted,
  spawnDaemon,
  summarizeCoverage,
  installMicroCommitGitHooks,
  runCommit,
  runMicroCommit,
  runWorkspaceScriptMain,
  TechnologyLinter,
  TEST_LEVELS,
  tryRun,
  type BreachRecord,
  type LcovFileRecord,
  type TestLevel,
} from "./repo/lib/js/index.ts";
import { existsSync, linkSync, mkdirSync, chmodSync, chownSync, copyFileSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, extname, join, relative, resolve } from "node:path";
import { createServer } from "node:net";
import { stat } from "node:fs/promises";
import { resolveActiveScopes } from "./.storybook/scopes.ts";

const WORKSPACE_ROOT = import.meta.dir;
const BUN = process.execPath;
const NATIVE_BOOTSTRAP_DIR = join(WORKSPACE_ROOT, "repo", "native", "bootstrap");

export { Script };

function resolvePlaygroundDevApp(segments: string[]): { readonly app: string; readonly rest: string[] } | null {
  const resolved = resolveFrameworkOsPlaygroundPlugin(loadFrameworkOsPlaygroundCatalog(), segments);
  if (!resolved) return null;
  return { app: resolved.plugin, rest: [...resolved.rest] };
}

function runFrameworkOsPlaygroundDev(plugin: string, rest: string[] = []): void {
  runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:dev", ...rest], {
    cwd: WORKSPACE_ROOT,
    env: frameworkOsPlaygroundDevEnv(loadFrameworkOsPlaygroundCatalog(), plugin),
    ...daemonBudgetOpts(),
  });
}

//#region 🔖NativeOsScript
/** 🖥️Runs native bootstrap shells under `repo/native/bootstrap` (setup|start). */
export class NativeOsScript extends Script {
  run(segments: string[]): void {
    const cmd = segments[0] ?? "setup";
    const env = { ...process.env, COMPOSE_REPO_ROOT: this.root };
    if (process.platform === "win32") {
      const ps1 = join(NATIVE_BOOTSTRAP_DIR, "script.ps1");
      if (!existsSync(ps1)) {
        console.error(`[native] missing ${ps1}; expected repo/native/bootstrap/script.ps1.`);
        process.exit(1);
      }
      runCmd("powershell.exe", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", ps1, cmd], { cwd: this.root, env });
      return;
    }
    if (process.platform === "darwin" || process.platform === "linux") {
      const sh = join(NATIVE_BOOTSTRAP_DIR, "script.sh");
      if (!existsSync(sh)) {
        console.error(`[native] missing ${sh}; expected repo/native/bootstrap/script.sh.`);
        process.exit(1);
      }
      runCmd("bash", [sh, cmd], { cwd: this.root, env });
      return;
    }
    console.error(`[native] unsupported platform ${process.platform}`);
    process.exit(1);
  }
}
//#endregion 🔖NativeOsScript

//#region 🔖SccacheSetup
const SCCACHE_VERSION = "0.10.0";

/** ⚡Ensures `sccache` is on PATH for `.cargo/config.toml` rustc-wrapper. */
function ensureSccache(): void {
  try {
    if (runProbe("sccache", ["--version"]).status === 0) return;
  } catch {
    /* install below */
  }

  const asset = sccacheReleaseAsset();
  if (!asset) {
    console.warn("[setup] sccache auto-install unsupported on this platform; install manually.");
    return;
  }

  const binDir = process.platform === "win32" ? join(process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"), "bin") : join(homedir(), ".local", "bin");
  const binName = process.platform === "win32" ? "sccache.exe" : "sccache";
  const dest = join(binDir, binName);
  if (existsSync(dest)) return;

  const cacheDir = join(WORKSPACE_ROOT, ".repo", "cache", "sccache");
  mkdirSync(cacheDir, { recursive: true });
  const archive = join(cacheDir, asset);
  const url = `https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/${asset}`;
  console.log(`[setup] downloading sccache ${SCCACHE_VERSION}…`);
  runCmd("curl", ["-fSL", "-o", archive, url]);

  if (asset.endsWith(".tar.gz")) {
    runCmd("tar", ["-xzf", archive, "-C", cacheDir]);
    const extractedDir = readdirSync(cacheDir).find((name) => name.startsWith("sccache-") && !name.endsWith(".tar.gz"));
    if (!extractedDir) throw new Error("sccache archive extraction failed");
    copyFileSync(join(cacheDir, extractedDir, binName), dest);
  } else {
    const extractDir = join(cacheDir, "extract");
    rmSync(extractDir, { recursive: true, force: true });
    mkdirSync(extractDir, { recursive: true });
    if (process.platform === "win32") {
      runCmd("powershell.exe", ["-NoProfile", "-Command", `Expand-Archive -Path '${archive}' -DestinationPath '${extractDir}' -Force`]);
    } else {
      runCmd("unzip", ["-o", archive, "-d", extractDir]);
    }
    const extractedDir = readdirSync(extractDir).find((name) => name.startsWith("sccache-"));
    if (!extractedDir) throw new Error("sccache archive extraction failed");
    copyFileSync(join(extractDir, extractedDir, binName), dest);
  }

  if (process.platform !== "win32") chmodSync(dest, 0o755);
  console.log(`[setup] installed sccache -> ${dest}`);
}

function sccacheReleaseAsset(): string | null {
  if (process.platform === "darwin") {
    return process.arch === "arm64" ? `sccache-v${SCCACHE_VERSION}-aarch64-apple-darwin.tar.gz` : `sccache-v${SCCACHE_VERSION}-x86_64-apple-darwin.tar.gz`;
  }
  if (process.platform === "linux") {
    const report = process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } | undefined;
    const libc = report?.header?.glibcVersionRuntime ? "gnu" : "musl";
    return process.arch === "arm64" ? `sccache-v${SCCACHE_VERSION}-aarch64-unknown-linux-${libc}.tar.gz` : `sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-${libc}.tar.gz`;
  }
  if (process.platform === "win32") {
    return process.arch === "arm64" ? `sccache-v${SCCACHE_VERSION}-aarch64-pc-windows-msvc.zip` : `sccache-v${SCCACHE_VERSION}-x86_64-pc-windows-msvc.zip`;
  }
  return null;
}
//#endregion 🔖SccacheSetup

//#region 🔖SetupScript
export class SetupScript extends Script {
  run(segments: string[]): void {
    if (!segments[0]) {
      this.runFull();
      return;
    }
    dispatchSubcommand(
      segments,
      {
        postinstall: () => this.runPostinstall(),
        git: () => this.runGit(),
        native: (rest) => new NativeOsScript(this.root).run(rest),
      },
      "bun ./script.ts setup [postinstall|git|native]",
    );
  }

  private runPostinstall(): void {
    const pkgPath = join(this.root, "node_modules", "lightningcss", "package.json");
    if (!existsSync(pkgPath)) return;
    const { version } = JSON.parse(readFileSync(pkgPath, "utf8")) as { version: string };
    const report = process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } | undefined;
    const libc = process.platform === "linux" ? (report?.header?.glibcVersionRuntime ? "gnu" : "musl") : "";
    const key = [process.platform, process.arch, libc].filter(Boolean).join("/");
    const pkgByKey: Record<string, string> = {
      "win32/x64": "lightningcss-win32-x64-msvc",
      "win32/arm64": "lightningcss-win32-arm64-msvc",
      "darwin/x64": "lightningcss-darwin-x64",
      "darwin/arm64": "lightningcss-darwin-arm64",
      "linux/x64/gnu": "lightningcss-linux-x64-gnu",
      "linux/x64/musl": "lightningcss-linux-x64-musl",
      "linux/arm64/gnu": "lightningcss-linux-arm64-gnu",
      "linux/arm64/musl": "lightningcss-linux-arm64-musl",
    };
    const platformPkg = pkgByKey[key];
    if (!platformPkg) return;
    if (existsSync(join(this.root, "node_modules", platformPkg))) return;
    runCmd("bun", ["add", "--no-save", `${platformPkg}@${version}`], { cwd: this.root });
  }

  private runGit(): void {
    runCmd("git", ["config", "--local", "core.symlinks", "true"], { cwd: this.root });
    const repoClientPath = resolveCliBin(this.root);
    if (!existsSync(repoClientPath)) {
      runCmd("go", ["build", "-o", repoClientPath, "./repo/client/mcp/go"], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
        budgetMs: buildBudgetMs(),
      });
    }
    runCmd(repoClientPath, ["configure"], { cwd: this.root });
    installMicroCommitGitHooks(this.root);
    const source = "AGENTS.md";
    for (const alias of ["CLAUDE.md", "GEMINI.md"]) {
      const aliasPath = join(this.root, alias);
      if (existsSync(aliasPath)) rmSync(aliasPath, { force: true });
      try {
        symlinkSync(source, aliasPath, "file");
      } catch (error) {
        if (process.platform !== "win32") throw error;
        linkSync(join(this.root, source), aliasPath);
      }
    }
  }

  private runFull(): void {
    if (process.argv.includes("--with-native-os")) {
      console.log(`[setup] ${process.platform} native bootstrap…`);
      tryRun(BUN, [join(this.root, "script.ts"), "setup", "native"], { cwd: this.root });
    }

    console.log("[setup] uv sync…");
    tryRun("uv", ["sync", "--all-packages", "--all-groups"]);
    console.log("[setup] neo4j MCP server prefetch (uvx)…");
    tryRun("uvx", ["--quiet", "mcp-neo4j-cypher", "--help"]);
    console.log("[setup] cargo fetch…");
    tryRun("cargo", ["fetch", "--manifest-path", "Cargo.toml"]);
    console.log("[setup] cargo-nextest…");
    tryRun("cargo", ["install", "cargo-nextest", "--locked"]);
    console.log("[setup] C++ toolchain and vcpkg…");
    tryRun("bun", [join(this.root, "script.ts"), "cpp", "setup"], { cwd: this.root });
    console.log("[setup] go build repo client…");
    const clientOut = join(this.root, "repo", "client", process.platform === "win32" ? "client.exe" : "client");
    tryRun("go", ["build", "-o", clientOut, "./repo/client/mcp/go"], { env: { ...process.env, GOWORK: join(this.root, "go.work") } });
    console.log("[setup] dotnet restore…");
    tryRun("dotnet", ["restore", "Monorepo.sln"]);
    console.log("[setup] rustup wasm target…");
    tryRun("rustup", ["target", "add", "wasm32-unknown-unknown"]);
    console.log("[setup] cargo-llvm-cov (exhaustive-level coverage)…");
    tryRun("cargo", ["install", "cargo-llvm-cov", "--locked"]);
    console.log("[setup] sccache…");
    try {
      ensureSccache();
    } catch (error) {
      console.warn("[setup] sccache install skipped:", error);
    }

    const browsersPath = join(this.root, "node_modules", ".cache", "ms-playwright");
    mkdirSync(browsersPath, { recursive: true });
    console.log("[setup] Playwright browsers…");
    tryRun("bunx", ["playwright", "install", "--with-deps", "chromium"], {
      env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsersPath },
    });

    if (process.platform === "linux") {
      const chromeSandbox = join(this.root, "node_modules", "electron", "dist", "chrome-sandbox");
      if (existsSync(chromeSandbox)) {
        try {
          chownSync(chromeSandbox, 0, 0);
          chmodSync(chromeSandbox, 0o4755);
          console.log("[setup] Electron chrome-sandbox permissions set.");
        } catch (e) {
          console.warn("[setup] chrome-sandbox chmod skipped:", e);
        }
      }
    }

    console.log("[setup] git workspace (symlinks, hook cleanup)…");
    new SetupScript(this.root).run(["git"]);

    console.log("[setup] VS Code extension build & package…");
    tryRun("bun", ["nx", "run", "@semio-tech/repo-vscode:build"], { cwd: this.root });
    tryRun("bun", ["nx", "run", "@semio-tech/repo-vscode:build-vsix"], { cwd: this.root });
    console.log("[setup] done.");
  }
}
//#endregion 🔖SetupScript

//#region 🔖StartScript
export class StartScript extends Script {
  run(_segments: string[]): void {
    process.chdir(this.root);
    const runGenerate = () => {
      if (runCmdStatus(BUN, [join(this.root, "script.ts"), "generate"], { cwd: this.root, ...orchestratorBudgetOpts() }) !== 0) {
        console.log("[start] `bun run generate` did not refresh all `.repo/🛂` bundles (Neo4j may be offline).");
      }
    };

    if (!existsSync(join(this.root, "node_modules", "nx", "package.json"))) {
      console.log("[start] node_modules incomplete — run `bun install` and `bun ./script.ts setup`.");
      return;
    }

    if (process.env.DEVCONTAINER === "true") {
      runGenerate();
      console.log("[start] Devcontainer session ready.");
      return;
    }

    if (process.platform === "win32" || process.platform === "darwin" || process.platform === "linux") {
      new NativeOsScript(this.root).run(["start"]);
    } else {
      console.log(`[start] Unsupported platform ${process.platform}.`);
    }
    runGenerate();
  }
}
//#endregion 🔖StartScript

//#region 🔖DevScript
export class DevScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "storybook") {
      await this.runStorybook(segments.slice(1));
      return;
    }
    if (segments[0] === "storybook-static") {
      await this.runStorybookStatic();
      return;
    }
    if (segments[0] === "s") {
      runFrameworkOsPlaygroundDev("s", segments.slice(1));
      return;
    }
    const playgroundApp = resolvePlaygroundDevApp(segments);
    if (playgroundApp) {
      runFrameworkOsPlaygroundDev(playgroundApp.app, playgroundApp.rest);
      return;
    }
    if (segments[0] === "mcp") {
      this.runMcp(segments.slice(1));
      return;
    }
    runCmd("bun", ["nx", "run", "@semio-tech/compose-desktop:dev"], { cwd: this.root, ...daemonBudgetOpts() });
  }

  private parseStorybookSegments(segments: string[]): { scope: string; args: string[] } {
    const scopeSegments: string[] = [];
    const args: string[] = [];
    let parsingScope = true;
    for (const segment of segments) {
      if (parsingScope && segment !== "--" && !segment.startsWith("-")) {
        scopeSegments.push(segment);
        continue;
      }
      parsingScope = false;
      if (segment !== "--") args.push(segment);
    }
    // A single scope arg may itself be a comma-list composing top-level scopes ("ui,compose/algorithm").
    // Multiple space-separated args keep the old slash-join behavior for hierarchical nx targets ("compose" "algorithm" -> "compose/algorithm").
    const scope = scopeSegments.length === 1 && scopeSegments[0]!.includes(",") ? scopeSegments[0]! : scopeSegments.join("/");
    const scopeIds = scope
      ? scope
          .split(",")
          .map((s) => s.trim())
          .filter(Boolean)
      : [];
    for (const id of scopeIds) {
      if (!/^[a-z0-9][a-z0-9/-]*$/i.test(id)) {
        console.error(`[dev.storybook] invalid scope ${JSON.stringify(id)}`);
        process.exit(1);
      }
    }
    if (scopeIds.length > 0) {
      try {
        resolveActiveScopes(scopeIds.join(","));
      } catch (error) {
        console.error(error instanceof Error ? `[dev.storybook] ${error.message}` : error);
        process.exit(1);
      }
    }
    return { scope: scopeIds.join(","), args };
  }

  private async runStorybook(extra: string[]): Promise<void> {
    const storybook = this.parseStorybookSegments(extra);
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    const port = process.env.STORYBOOK_PORT ?? "6010";
    const useExactPort = process.env.STORYBOOK_EXACT_PORT === "1" || process.env.STORYBOOK_EXACT_PORT === "true";
    const storybookArgs = ["storybook", "dev", "-c", ".storybook", "-p", port, ...(useExactPort ? ["--exact-port"] : []), "--host", host, "--no-open", "--debug", ...storybook.args];
    runCmd("bunx", storybookArgs, {
      cwd: this.root,
      env: {
        ...process.env,
        STORYBOOK_SCOPE: storybook.scope,
        WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
        CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
      },
      ...daemonBudgetOpts(),
    });
  }

  private async runStorybookStatic(): Promise<void> {
    const host = process.env.STORYBOOK_STATIC_HOST ?? "0.0.0.0";
    const port = Number(process.env.STORYBOOK_PORT ?? "6010");
    const documentRoot = resolve(this.root, "storybook-static");
    const server = Bun.serve({
      hostname: host,
      port,
      async fetch(request) {
        const requestUrl = new URL(request.url);
        const requestPath = decodeURIComponent(requestUrl.pathname);
        const candidatePath = resolve(documentRoot, `.${requestPath}`);
        if (!candidatePath.startsWith(documentRoot)) return new Response("Forbidden", { status: 403 });
        const filePath = await (async () => {
          try {
            const fileInfo = await stat(candidatePath);
            if (fileInfo.isDirectory()) return resolve(candidatePath, "index.html");
            return candidatePath;
          } catch {
            if (extname(candidatePath) === "") return resolve(candidatePath, "index.html");
            return candidatePath;
          }
        })();
        try {
          const file = Bun.file(filePath);
          if (!(await file.exists())) return new Response("Not Found", { status: 404 });
          return new Response(file);
        } catch {
          return new Response("Not Found", { status: 404 });
        }
      },
    });

    console.log(`storybook-static listening on http://${host}:${port}`);
    await new Promise(() => {});
  }

  private runMcp(segments: string[]): void {
    const a = segments[0];
    if (a === "engine") {
      runCmd("bun", [join(this.root, "compose", "client", "bin", "engine", "script.ts"), "dev", "mcp"], { cwd: this.root, ...daemonBudgetOpts() });
      return;
    }
    if (a === "neo4j") {
      this.runMcpNeo4j(segments.slice(1));
      return;
    }
    if (a === "stdio") {
      this.runMcpStdioRepo(segments.slice(1));
      return;
    }
    const mode = a === "repo" ? "repo" : "default";
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    if (mode === "repo") {
      runCmd("npx", ["--yes", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"], {
        cwd: this.root,
        env: { ...process.env, HOST: host },
        ...daemonBudgetOpts(),
      });
      return;
    }
    runCmd("npx", ["--yes", "@modelcontextprotocol/inspector"], { cwd: this.root, ...daemonBudgetOpts() });
  }

  private runMcpNeo4j(neoSegments: string[]): void {
    const { nameParts, passthrough } = partitionNeo4jGraphCliArgv(neoSegments);
    const hasName = nameParts.length > 0;
    const graphDatabase = hasName ? joinNeo4jGraphDatabaseName(nameParts) : process.env.NEO4J_DATABASE || "compose";
    const args = [...passthrough];
    if (hasName && !args.includes("--namespace")) args.push("--namespace", graphDatabase);
    runCmd("uvx", ["mcp-neo4j-cypher", ...args], {
      cwd: this.root,
      env: {
        ...process.env,
        NEO4J_URI: process.env.NEO4J_URI || "bolt://localhost:7687",
        NEO4J_USERNAME: process.env.NEO4J_USERNAME || "neo4j",
        NEO4J_PASSWORD: process.env.NEO4J_PASSWORD || "password",
        NEO4J_DATABASE: graphDatabase,
        NEO4J_TELEMETRY: process.env.NEO4J_TELEMETRY || "false",
      },
      ...daemonBudgetOpts(),
    });
  }

  private runMcpStdioRepo(slugs: string[]): void {
    const slug = (slugs[0] ?? "client").trim().toLowerCase();
    const extra = slugs.slice(1);
    const bin = resolveCliBin(this.root);
    if (!existsSync(bin)) {
      runCmd("go", ["build", "-o", bin, "./repo/client/mcp/go"], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
        budgetMs: buildBudgetMs(),
      });
    }
    runCmd(bin, ["mcp", slug, ...extra], {
      cwd: this.root,
      env: { ...process.env, GOWORK: join(this.root, "go.work") },
      ...daemonBudgetOpts(),
    });
  }
}
//#endregion 🔖DevScript

//#region 🔖NxScript
export class NxScript extends Script {
  run(segments: string[]): void {
    runCmd("node", [join(this.root, "node_modules", "nx", "bin", "nx.js"), ...segments], {
      cwd: this.root,
      env: devToolingEnv(),
      ...orchestratorBudgetOpts(),
    });
  }
}
//#endregion 🔖NxScript

//#region 🔖GenerateScript
export class GenerateScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "neo4j") {
      new Neo4jCypherExport(this.root).runFromArgv(segments.slice(1));
      return;
    }
    let successes = 0;
    let failures = 0;
    const exporter = new Neo4jCypherExport(this.root);
    for (const spec of getAllNeo4jGraphExportSpecs(process.env)) {
      const joined = joinNeo4jGraphDatabaseName(spec);
      const prev = process.env.NEO4J_DATABASE;
      process.env.NEO4J_DATABASE = joined;
      try {
        if (exporter.tryExportFromArgv([...spec])) successes += 1;
        else {
          failures += 1;
          console.error(`[generate] neo4j (${joined}) failed.`);
        }
      } finally {
        if (prev === undefined) delete process.env.NEO4J_DATABASE;
        else process.env.NEO4J_DATABASE = prev;
      }
    }
    if (successes === 0) {
      console.error("[generate] no Neo4j database could be exported.");
      process.exit(1);
    }
    if (failures > 0) {
      console.error(`[generate] partial success (${successes} ok, ${failures} failed).`);
    }
    console.log(`[generate] Neo4j Cypher export finished (${successes} ok, ${failures} skipped/failed) under .repo/🛂.`);
  }
}
//#endregion 🔖GenerateScript

//#region 🔖LintScript
export class LintScript extends Script {
  run(segments: string[]): void {
    // nx orchestrators: exempt — total wall time spans every project and legitimately exceeds any single
    // command's budget; each leaf project's own build/test/lint commands are individually budgeted.
    if (segments[0] === "repo") {
      runCmd("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    runCmd("bun", ["nx", "run-many", "-t", "lint", "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bunx", ["dependency-cruiser@16", "compose", "framework", "flow", "layout", "puzzle", "ui", "draw", "note", "sequence", "s", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
  }
}
//#endregion 🔖LintScript

//#region 🔖VerifyScript
/** 🧪Aggregates lint + generated-catalog freshness + region/host-contract script lints (`gate`, the cheap pre-`ticket_close` step every refactor session runs), plus the full test suite for the top-level `verify` verb. */
export class VerifyScript extends Script {
  async run(segments: string[]): Promise<void> {
    await this.runGate();
    if (segments[0] === "gate") return;
    runCmd("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
  }

  private async runGate(): Promise<void> {
    // Deliberately calls dependency-cruiser directly rather than `LintScript`/`nx run-many -t lint --all`:
    // several unrelated projects (repo/client/vscode, compose-js, …) have pre-existing broken eslint configs,
    // and framework-renderer-wgpu:lint has known pending color-literal violations (see spawn_task follow-ups) —
    // this gate must stay a meaningful, currently-green signal for refactor sessions, not inherit that noise.
    console.log("[verify] dependency-cruiser boundaries…");
    runCmd("bunx", ["dependency-cruiser@16", "compose", "framework", "flow", "layout", "puzzle", "ui", "draw", "note", "sequence", "s", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
    console.log("[verify] generated catalog freshness…");
    // nx orchestrators: exempt — leaves individually budgeted.
    runCmd("bun", ["nx", "run", "@semio-tech/plugin-registry:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] region/host-contract script lints…");
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-react:lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:plugin", "lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/ui-styling-tokens:check-no-px"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] leveled test target coverage…");
    this.checkLeveledTestTargets();
    console.log("[verify] gate passed.");
  }

  /** 📊Every `project.json` with a `test` target must also declare `test-quick`/`test-long`/`test-exhaustive` —
   * otherwise `nx run-many -t test-exhaustive` silently skips that project and the exhaustive-level coverage
   * gate under-counts it. Guards against the gap this ticket closed (26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE) reopening one project.json at a time. */
  private checkLeveledTestTargets(): void {
    const offenders: string[] = [];
    const walk = (dir: string): void => {
      for (const entry of readdirSync(dir, { withFileTypes: true })) {
        if (entry.name === "node_modules" || entry.name === "target" || entry.name.startsWith(".")) continue;
        const full = join(dir, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (entry.name !== "project.json") continue;
        let targets: Record<string, unknown>;
        try {
          targets = (JSON.parse(readFileSync(full, "utf8")) as { targets?: Record<string, unknown> }).targets ?? {};
        } catch {
          continue;
        }
        if (!("test" in targets)) continue;
        const missing = (["test-quick", "test-long", "test-exhaustive"] as const).filter((t) => !(t in targets));
        if (missing.length) offenders.push(`${relative(this.root, full)} missing ${missing.join(", ")}`);
      }
    };
    walk(this.root);
    if (offenders.length) {
      console.error(`[verify] ${offenders.length} project.json file(s) have a "test" target without leveled siblings:`);
      for (const o of offenders) console.error(`  ${o}`);
      process.exit(1);
    }
  }
}
//#endregion 🔖VerifyScript

//#region 🔖FormatScript
export class FormatScript extends Script {
  run(_segments: string[]): void {
    runCmd("bunx", ["prettier", "-w", "."], { cwd: this.root, shell: true });
  }
}
//#endregion 🔖FormatScript

//#region 🔖TestScript
/** 🎚️Nx target name for a level — `fundamental` keeps the bare `test` target; others get a `test-<level>` suffix. */
function testTargetForLevel(level: TestLevel): string {
  return level === "fundamental" ? "test" : `test-${level}`;
}

export class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    if (rest[0] === "storybook") {
      await this.runStorybookPlaywright();
      return;
    }
    if (rest[0] === "repo-client") {
      await this.runRepoGoTest("./repo/client/cli/go", level, rest.slice(1));
      return;
    }
    if (rest[0] === "repo-mcp") {
      const clientOut = resolveCliBin(this.root);
      runCmd("go", ["build", "-o", clientOut, "./repo/client/mcp/go"], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
        budgetMs: buildBudgetMs(),
      });
      await this.runRepoGoTest("./repo/client/cli/go", level, ["-run", "Mcp|MCP|mcp", ...rest.slice(1)]);
      return;
    }
    const collectingCoverage = level === "exhaustive" && coverageEnabled();
    if (collectingCoverage) {
      // Stale reports from a previous run must never leak into this one's percentage — test-exhaustive is
      // already nx `cache: false`, so this is the only place that needs to clear it.
      for (const kind of ["js", "rust", "go", "py", "dotnet"] as const) rmSync(coverageDir(this.root, kind), { recursive: true, force: true });
    }

    // nx orchestrators: exempt — leaves individually budgeted.
    runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio-tech/compose-js", "@semio-tech/compose-react"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "compose/graphql:build"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run-many", "-t", testTargetForLevel(level), "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) {
      await this.runStorybookPlaywright();
    }

    if (collectingCoverage) this.enforceCoverageGate();
  }

  /** 📊Walks every `*.lcov`/`lcov.info`/`coverage.info`/`*.cover` file under `.repo/coverage/`, merges them into one repo-wide LCOV, writes `summary.json`, and hard-fails below the 95% threshold — the exhaustive-level gate. */
  private enforceCoverageGate(): void {
    const walk = (dir: string, matches: (name: string) => boolean, found: string[] = []): string[] => {
      if (!existsSync(dir)) return found;
      for (const entry of readdirSync(dir, { withFileTypes: true })) {
        const full = join(dir, entry.name);
        if (entry.isDirectory()) walk(full, matches, found);
        else if (matches(entry.name)) found.push(full);
      }
      return found;
    };
    const recordSets: LcovFileRecord[][] = [
      ...walk(coverageDir(this.root, "rust"), (n) => n.endsWith(".lcov")).map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "js"), (n) => n === "lcov.info").map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "py"), (n) => n.endsWith(".lcov")).map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "dotnet"), (n) => n === "coverage.info").map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "go"), (n) => n.endsWith(".cover")).map((f) => goProfileToLcov(readFileSync(f, "utf8"))),
    ];
    const merged = mergeLcov(recordSets);
    const summary = summarizeCoverage(merged);
    writeFileSync(join(this.root, ".repo", "coverage", "lcov.info"), renderLcov(merged));
    writeFileSync(join(this.root, ".repo", "coverage", "summary.json"), JSON.stringify(summary, null, 2));
    enforceCoverageThreshold(summary, 95);
  }

  private async waitForUrl(url: string, timeoutMs: number): Promise<void> {
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {
      try {
        const response = await fetch(url);
        if (response.ok) return;
      } catch {
        /* retry */
      }
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
    throw new Error(`Timed out waiting for ${url}`);
  }

  private isTcpPortFree(port: number, host: string): Promise<boolean> {
    return new Promise((resolve) => {
      const server = createServer();
      server.unref();
      server.once("error", () => resolve(false));
      server.listen(port, host, () => {
        server.close(() => resolve(true));
      });
    });
  }

  private async pickStorybookStaticPort(preferred: number, span: number): Promise<number> {
    for (let port = preferred; port < preferred + span; port += 1) {
      if (await this.isTcpPortFree(port, "0.0.0.0")) return port;
    }
    throw new Error(`No free TCP port in ${preferred}..${preferred + span - 1}`);
  }

  /** ⏱️`goLevelTestArgs` keeps `-short` (skipping the `testing.Short()`-gated real-monorepo-scan tests in `repo/client/cli/go/main_test.go`) through `quick` and adds a cumulative `-skip` above the requested level. */
  private async runRepoGoTest(module: string, level: TestLevel, extraArgs: string[]): Promise<void> {
    await runTestBudgeted("go", ["test", module, ...goLevelTestArgs(level), ...goCoverageArgs(this.root, module), ...extraArgs], {
      cwd: this.root,
      env: { ...process.env, GOWORK: join(this.root, "go.work") },
    });
  }

  private async runStorybookPlaywright(): Promise<void> {
    const preferred = Number(process.env.STORYBOOK_PORT ?? 6010);
    const storybookPort = String(await this.pickStorybookStaticPort(preferred, 50));
    const baseUrl = `http://127.0.0.1:${storybookPort}/`;
    runCmd("bun", [join(this.root, "script.ts"), "build", "storybook"], { cwd: this.root, ...orchestratorBudgetOpts() });
    const server = spawnDaemon("bun", [join(this.root, "script.ts"), "dev", "storybook-static"], {
      cwd: this.root,
      env: { ...process.env, STORYBOOK_PORT: storybookPort },
    });
    try {
      await this.waitForUrl(new URL("index.html", baseUrl).href, 120000);
      runCmd("bunx", ["playwright", "test", ".storybook/puzzle-2d.spec.ts", "--config", ".storybook/playwright.config.ts"], {
        cwd: this.root,
        env: {
          ...process.env,
          PLAYWRIGHT_BASE_URL: baseUrl,
          PLAYWRIGHT_BROWSERS_PATH: process.env.PLAYWRIGHT_BROWSERS_PATH ?? `${this.root}/node_modules/.cache/ms-playwright`,
          STORYBOOK_PORT: storybookPort,
        },
      });
    } finally {
      server.kill();
    }
  }
}
//#endregion 🔖TestScript

//#region 🔖BuildScript
export class BuildScript extends Script {
  run(segments: string[]): void {
    const slice = segments[0];
    const single: Record<string, string> = {
      "3dm": "@semio-tech/compose-3dm-ui:build",
      assets: "@semio-tech/semio-asset:build",
      desktop: "@semio-tech/compose-desktop:build",
      engine: "@semio-tech/compose-engine:build",
      storybook: "workspace:build-storybook",
      "coda-desktop": "@semio-tech/coda-desktop:build",
      "repo-cli": "@semio-tech/repo-client:build",
      "repo-server": "@semio-tech/repo-coordinator:build",
      "repo-vscode": "@semio-tech/repo-vscode:build-vsix",
    };

    // nx orchestrators: exempt — leaves individually budgeted.
    if (!slice) {
      runCmd("bun", ["nx", "run-many", "-t", "build", "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
      runCmd("bun", ["nx", "run", "workspace:build-storybook"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    if (slice === "storybook") {
      runCmd("bunx", ["storybook", "build", "-c", ".storybook", "--output-dir", "storybook-static"], { cwd: this.root });
      return;
    }
    if (slice === "sites") {
      runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio-tech/compose-sketchpad-play", "@semio-tech/compose-sketchpad-docs"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    const target = single[slice];
    if (!target) {
      console.error(`[build] unknown slice ${JSON.stringify(slice)}`);
      process.exit(1);
    }
    runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}
//#endregion 🔖BuildScript

//#region 🔖CppScriptHelpers
const WINDOWS_CMAKE_GENERATOR = "Visual Studio 18 2026";

function vswhereExecutable(): string {
  const programFilesX86 = process.env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)";
  return join(programFilesX86, "Microsoft Visual Studio", "Installer", "vswhere.exe");
}

function queryVisualStudio2026InstallPath(): string | undefined {
  if (process.platform !== "win32") return undefined;
  const vswhere = vswhereExecutable();
  if (!existsSync(vswhere)) return undefined;
  const result = runProbe(vswhere, ["-latest", "-version", "[18.0,19.0)", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"]);
  if (result.status !== 0) return undefined;
  const installPath = result.stdout.trim();
  return installPath || undefined;
}
//#endregion 🔖CppScriptHelpers

//#region 🔖CppScript
export class CppScript extends Script {
  run(segments: string[]): void {
    const preset = this.resolvePreset(segments.slice(1));
    dispatchSubcommand(
      segments,
      {
        setup: () => this.runSetup(),
        configure: () => this.runConfigure(preset),
        build: () => this.runBuild(preset),
        test: () => this.runTest(preset),
        all: () => {
          this.runSetup();
          this.runConfigure(preset);
          this.runBuild(preset);
          this.runTest(preset);
        },
      },
      "bun ./script.ts cpp [setup|configure|build|test|all] [preset]",
      "all",
    );
  }

  private resolvePreset(segments: string[]): string {
    const explicit = segments.find((segment) => !segment.startsWith("-"));
    if (explicit) return explicit;
    if (process.env.CMAKE_PRESET) return process.env.CMAKE_PRESET;
    if (process.platform === "win32") return "windows";
    if (process.platform === "darwin") return "macos";
    return "linux";
  }

  private runSetup(): void {
    this.ensureTool("cmake", "cmake");
    if (process.platform !== "win32") this.ensureTool("ninja", "ninja");
    this.ensureVcpkg();
    if (process.platform === "win32") this.ensureWindowsMsvc();
  }

  private runConfigure(preset: string): void {
    this.runSetup();
    this.purgeStaleCmakeCache(preset);
    runCmd(this.resolveTool("cmake"), ["--preset", preset], { cwd: this.root, env: this.cppEnv(), budgetMs: buildBudgetMs() });
  }

  private runBuild(preset: string): void {
    runCmd(this.resolveTool("cmake"), ["--build", "--preset", preset], { cwd: this.root, env: this.cppEnv(), budgetMs: buildBudgetMs() });
  }

  private runTest(preset: string): void {
    runCmd(this.resolveTool("ctest"), ["--preset", preset], { cwd: this.root, env: this.cppEnv(), budgetMs: buildBudgetMs() });
  }

  private ensureTool(command: string, uvTool: string): void {
    if (this.hasTool(command)) return;
    if (this.hasTool("uv")) {
      runCmd(this.resolveTool("uv"), ["tool", "install", "--upgrade", uvTool], { cwd: this.root });
      if (this.hasTool(command)) return;
    }
    console.error(`[cpp] ${command} is required. Run the native bootstrap or rebuild the devcontainer.`);
    process.exit(1);
  }

  private hasTool(command: string): boolean {
    try {
      return runProbe(this.resolveTool(command), ["--version"], { env: this.cppEnv() }).status === 0;
    } catch {
      return false;
    }
  }

  private resolveTool(command: string): string {
    const extension = process.platform === "win32" ? ".exe" : "";
    const candidates = [command, join(homedir(), ".local", "bin", `${command}${extension}`), join(homedir(), ".local", "bin", command)];
    return candidates.find((candidate) => existsSync(candidate)) ?? command;
  }

  private ensureVcpkg(): void {
    const vcpkgRoot = this.vcpkgRoot();
    const vcpkgExe = join(vcpkgRoot, process.platform === "win32" ? "vcpkg.exe" : "vcpkg");
    if (!existsSync(vcpkgRoot)) {
      mkdirSync(join(this.root, ".repo", "cache"), { recursive: true });
      runCmd("git", ["clone", "--depth", "1", "https://github.com/microsoft/vcpkg.git", vcpkgRoot], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
    if (!existsSync(vcpkgExe)) {
      if (process.platform === "win32") {
        runCmd("cmd.exe", ["/c", join(vcpkgRoot, "bootstrap-vcpkg.bat"), "-disableMetrics"], { cwd: vcpkgRoot, budgetMs: buildBudgetMs() });
      } else {
        runCmd("bash", [join(vcpkgRoot, "bootstrap-vcpkg.sh"), "-disableMetrics"], { cwd: vcpkgRoot, budgetMs: buildBudgetMs() });
      }
    }
  }

  private cppEnv(): NodeJS.ProcessEnv {
    return {
      ...devToolingEnv(),
      CMAKE_BUILD_PARALLEL_LEVEL: process.env.CMAKE_BUILD_PARALLEL_LEVEL ?? "4",
      VCPKG_ROOT: this.vcpkgRoot(),
      VCPKG_DISABLE_METRICS: "1",
      VCPKG_MAX_CONCURRENCY: process.env.VCPKG_MAX_CONCURRENCY ?? "4",
    };
  }

  private vcpkgRoot(): string {
    return process.env.VCPKG_ROOT || join(this.root, ".repo", "cache", "vcpkg");
  }

  private ensureWindowsMsvc(): void {
    if (process.platform !== "win32") return;
    if (queryVisualStudio2026InstallPath()) return;
    console.error("[cpp] Visual Studio 2026 with the Desktop development with C++ workload is required.");
    console.error("[cpp] On native Windows run: bun ./script.ts setup native");
    process.exit(1);
  }

  private purgeStaleCmakeCache(preset: string): void {
    const cacheDir = join(this.root, ".repo", "cache", "cmake", preset);
    const cacheFile = join(cacheDir, "CMakeCache.txt");
    if (!existsSync(cacheFile)) return;
    const content = readFileSync(cacheFile, "utf8");
    const generatorMatch = content.match(/^CMAKE_GENERATOR:INTERNAL=(.+)$/m);
    const cachedGenerator = generatorMatch?.[1]?.trim();
    if (process.platform === "win32" && cachedGenerator && cachedGenerator !== WINDOWS_CMAKE_GENERATOR) {
      console.log(`[cpp] Removing stale CMake cache for preset "${preset}" (generator ${cachedGenerator}).`);
      rmSync(cacheDir, { recursive: true, force: true });
    }
  }
}
//#endregion 🔖CppScript

//#region 🔖PublishScript
export class PublishScript extends Script {
  run(segments: string[]): void {
    const slice = segments[0];
    const map: Record<string, string> = {
      desktop: "@semio-tech/compose-desktop:publish",
      play: "@semio-tech/compose-sketchpad-play:publish",
      sketchpad: "@semio-tech/compose-sketchpad:publish",
      docs: "@semio-tech/compose-sketchpad-docs:publish",
      "coda-desktop": "@semio-tech/coda-desktop:publish",
    };
    if (!slice) {
      console.error(`[publish] usage: bun ./script.ts publish <${Object.keys(map).join(" | ")}>`);
      process.exit(1);
    }
    const target = map[slice];
    if (!target) {
      console.error(`[publish] unknown slice ${JSON.stringify(slice)}`);
      process.exit(1);
    }
    runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() }); // nx orchestrator — leaves individually budgeted
  }
}
//#endregion 🔖PublishScript

//#region 🔖QueryScript
export class QueryScript extends Script {
  run(segments: string[]): void {
    const sub = segments[0] ?? "test";
    const queryDir = join(this.root, "compose/client/lib/query");
    if (sub === "build") {
      runCmd(bun, [join(queryDir, "script.ts"), "build"], { cwd: this.root });
      return;
    }
    if (sub === "wasm") {
      runCmd(bun, [join(queryDir, "script.ts"), "wasm"], { cwd: this.root });
      return;
    }
    if (sub === "test") {
      runCmd(bun, [join(queryDir, "script.ts"), "test", ...segments.slice(1)], { cwd: this.root });
      return;
    }
    console.error(`[query] unknown subcommand ${JSON.stringify(sub)}`);
    process.exit(1);
  }
}
//#endregion 🔖QueryScript

//#region 🔖PurgeScript
export class PurgeScript extends Script {
  run(segments: string[]): void {
    if (segments[0] !== "neo4j") {
      console.error("[purge] usage: bun ./script.ts purge neo4j");
      process.exit(1);
    }
    const database = process.env.NEO4J_DATABASE || "compose";
    const uri = process.env.NEO4J_URI || "bolt://localhost:7687";
    const user = process.env.NEO4J_USERNAME || "neo4j";
    const password = process.env.NEO4J_PASSWORD || "password";
    if (runCmdStatus("cypher-shell", ["-a", uri, "-u", user, "-p", password, "-d", database, "--format", "plain", "RETURN 1 AS ok;"], { cwd: this.root }) !== 0) {
      console.warn("[purge.neo4j] cypher-shell unavailable — skip.");
      process.exit(0);
    }
    console.log("[purge.neo4j] connectivity ok; no_operation.");
  }
}
//#endregion 🔖PurgeScript

//#region 🔖MicroCommitScript
/** 🎆Stages WIP changes and writes deterministic micro-commit templates (GitKraken + CLI). */
export class MicroCommitScript extends Script {
  run(segments: string[]): void {
    runMicroCommit(this.root, segments);
  }
}
//#endregion 🔖MicroCommitScript

//#region 🔖CommitScript
/** 🔀Bundle micro-commits into a signed squash commit with per-bundle summaries. */
export class CommitScript extends Script {
  run(segments: string[]): void {
    runCommit(this.root, segments);
  }
}
//#endregion 🔖CommitScript

//#region 🔖OsScript
/** 🕸️Headless OS studio commands — computes a media graph without a UI (`os run <bundle>.studio`). */
export class OsScript extends Script {
  run(segments: string[]): void {
    const sub = segments[0];
    if (sub === "run") {
      const rest = segments.slice(1);
      const bundle = rest.find((segment) => !segment.startsWith("--"));
      if (!bundle) {
        console.error("[os.run] usage: bun ./script.ts os run <bundle>.studio [--node <id>] [--watch] [--dry]");
        process.exit(1);
      }
      const watch = rest.includes("--watch");
      runCmd("cargo", ["run", "--release", "-p", "semio-framework-os-run", "--", ...rest], { cwd: this.root, ...(watch ? daemonBudgetOpts() : { budgetMs: buildBudgetMs() }) });
      return;
    }
    console.error(`[os] unknown subcommand ${JSON.stringify(sub)}`);
    process.exit(1);
  }
}
//#endregion 🔖OsScript

//#region 🔖Dispatch
const router = new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT)
  .register("os", OsScript)
  .register("nx", NxScript)
  .register("setup", SetupScript)
  .register("start", StartScript)
  .register("dev", DevScript)
  .register("generate", GenerateScript)
  .register("lint", LintScript)
  .register("verify", VerifyScript)
  .register("format", FormatScript)
  .register("test", TestScript)
  .register("build", BuildScript)
  .register("cpp", CppScript)
  .register("publish", PublishScript)
  .register("purge", PurgeScript)
  .register("query", QueryScript)
  .register("micro-commit", MicroCommitScript)
  .register("commit", CommitScript);

//#endregion 🔖Dispatch

//#region 🔖generate-neo4j-gen
/**
 * 🛂 Neo4j → `.repo/🛂/<graph>.cypher` export (pure module; invoked from root `script.ts`). Product graphs are fixed specs; extra Bolt graphs use `NEO4J_EXTRA_GRAPH_DATABASES` (comma-separated). Argv segments join with `-` via `joinNeo4jGraphDatabaseName`.
 */
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const NEO4J_VERSION = "5.26.26";

/** 🏗️Product graphs only (compose stack); not arbitrary developer databases. */
export const NEO4J_PRODUCT_GRAPH_DATABASE_SPECS = [["compose"], ["elements"], ["coda"], ["reuse"]] as const;

/** 🗑️Env key: comma-separated extra Bolt graph names for `bun run generate` and native `.repo/🛂/*.cypher` stubs. */
export const NEO4J_EXTRA_GRAPH_DATABASES_ENV = "NEO4J_EXTRA_GRAPH_DATABASES";

/** 🔗Bolt user graph name from argv segments after `neo4j` / `generate neo4j` (hyphen join). */
export function joinNeo4jGraphDatabaseName(parts: readonly string[]): string {
  return parts.join("-");
}

/** 🔀Parses `NEO4J_EXTRA_GRAPH_DATABASES` into trimmed non-empty graph names. */
export function parseExtraNeo4jGraphDatabaseNamesFromEnv(env: NodeJS.ProcessEnv = process.env): string[] {
  const raw = env[NEO4J_EXTRA_GRAPH_DATABASES_ENV]?.trim();
  if (!raw) return [];
  return raw
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

/** 📋Product graph argv rows plus `[name]` per extra env entry. */
export function getAllNeo4jGraphExportSpecs(env: NodeJS.ProcessEnv = process.env): string[][] {
  const core: string[][] = NEO4J_PRODUCT_GRAPH_DATABASE_SPECS.map((row) => [...row]);
  const extras = parseExtraNeo4jGraphDatabaseNamesFromEnv(env).map((n) => [n]);
  return [...core, ...extras];
}

/** 🧾Bolt graph names allowed for `generate neo4j …` (product + extras). */
export function neo4jExportDatabaseNameSet(env: NodeJS.ProcessEnv = process.env): Set<string> {
  return new Set(getAllNeo4jGraphExportSpecs(env).map((spec) => joinNeo4jGraphDatabaseName(spec)));
}

/** @deprecated Prefer `getAllNeo4jGraphExportSpecs`; product-only joined names. */
export const NEO4J_GRAPH_DATABASE_NAMES = NEO4J_PRODUCT_GRAPH_DATABASE_SPECS.map((s) => joinNeo4jGraphDatabaseName(s));

export type Neo4jGraphDatabaseName = (typeof NEO4J_GRAPH_DATABASE_NAMES)[number];

export function partitionNeo4jGraphCliArgv(segments: string[]): { nameParts: string[]; passthrough: string[] } {
  const nameParts: string[] = [];
  let i = 0;
  while (i < segments.length && !segments[i]!.startsWith("-")) {
    nameParts.push(segments[i]!);
    i += 1;
  }
  return { nameParts, passthrough: segments.slice(i) };
}

export class Neo4jCypherExport {
  constructor(private readonly repoRoot: string) {}

  resolveCypherShell(): string | null {
    const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
    const cachedShell = join(this.repoRoot, ".repo", "cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
    const candidates = [process.env.NEO4J_CYPHER_SHELL, cachedShell, runtimeName].filter((value): value is string => Boolean(value));

    for (const candidate of candidates) {
      if (candidate.includes("/") || candidate.includes("\\")) {
        if (existsSync(candidate)) return candidate;
        continue;
      }
      try {
        if (runProbe(candidate, ["--version"]).status === 0) return candidate;
      } catch {
        /* try next */
      }
    }
    return null;
  }

  buildCypherEnv(): NodeJS.ProcessEnv {
    const env = { ...process.env };
    if (process.platform === "win32") {
      const javaHome = "C:\\Program Files\\Microsoft\\jdk-21.0.11.10-hotspot";
      const javaExecutable = join(javaHome, "bin", "java.exe");
      if (existsSync(javaExecutable)) {
        env.JAVA_HOME = javaHome;
        env.Path = `${join(javaHome, "bin")};${env.Path || ""}`;
      }
    }
    return env;
  }

  runCypher(database: string, cypher: string): { ok: boolean; stdout: string; stderr: string } {
    const shell = this.resolveCypherShell();
    if (!shell) {
      return { ok: false, stdout: "", stderr: "cypher-shell not found (install Neo4j tools or set NEO4J_CYPHER_SHELL)." };
    }

    const queryDir = join(this.repoRoot, ".repo", "cache");
    mkdirSync(queryDir, { recursive: true });
    const queryPath = join(queryDir, `neo4j-generate-query-${process.pid}-${Date.now()}.cypher`);
    writeFileSync(queryPath, `${cypher.trim()}\n`, "utf8");

    try {
      const result = runProbe(shell, ["-a", process.env.NEO4J_URI || "bolt://localhost:7687", "-u", process.env.NEO4J_USERNAME || "neo4j", "-p", process.env.NEO4J_PASSWORD || "password", "-d", database, "--format", "plain", "-f", queryPath], {
        cwd: this.repoRoot,
        env: this.buildCypherEnv(),
      });

      return {
        ok: result.status === 0,
        stdout: result.stdout,
        stderr: result.stderr,
      };
    } finally {
      try {
        unlinkSync(queryPath);
      } catch {
        /* temp query cleanup */
      }
    }
  }

  apocExportCypherAllToAbsoluteFile(database: string, absoluteFile: string): { ok: boolean; message: string } {
    const neoPath = absoluteFile.replace(/\\/g, "/");
    const apocTarget = /^[A-Za-z]:\//.test(neoPath) ? `file:${neoPath}` : neoPath.startsWith("/") ? `file://${neoPath}` : neoPath;
    const pathLiteral = JSON.stringify(apocTarget);
    const cypher = [
      `CALL apoc.export.cypher.all(${pathLiteral}, {`,
      `  format: "cypher-shell",`,
      `  writeNodeProperties: true,`,
      `  ifNotExists: true,`,
      `  useOptimizations: { type: "UNWIND_BATCH", unwindBatchSize: 100 }`,
      `})`,
      `YIELD file, batches, source, format, nodes, relationships, properties, time, rows, batchSize`,
      `RETURN file, batches, source, format, nodes, relationships, properties, time, rows, batchSize;`,
    ].join("\n");

    const { ok, stdout, stderr } = this.runCypher(database, cypher);
    if (!ok) {
      return {
        ok: false,
        message: `${stderr || stdout || "unknown error"}\n` + "Ensure APOC is installed, apoc.export.file.enabled=true, and Neo4j may write this absolute path (set apoc.import.file.use_neo4j_config=false on Desktop — see setup scripts).",
      };
    }
    return { ok: true, message: stdout.trim() };
  }

  writeGeneratedCypherBundle(technology: string, database: string, body: string, finalPath: string): void {
    const stamp = new Date().toISOString();
    const header = [
      "// SPDX-License-Identifier: AGPL-3.0-only",
      "// Generated exclusively from the live Neo4j database — do not edit this file by hand.",
      "// Refresh: `bun run generate` (root `script.ts`).",
      `// graph: ${technology} | database: ${database} | generated: ${stamp}`,
      "//",
      "",
    ].join("\n");

    writeFileSync(finalPath, `${header}${body.trim()}\n`, "utf8");
  }

  tryExportFromArgv(argv: string[]): boolean {
    const { nameParts, passthrough } = partitionNeo4jGraphCliArgv(argv);
    if (passthrough.length > 0) {
      console.error(`[generate:neo4j] unexpected extra arguments (use only graph name segments before any -flags): ${JSON.stringify(passthrough)}`);
      return false;
    }
    const joined = nameParts.length > 0 ? joinNeo4jGraphDatabaseName(nameParts) : (process.env.NEO4J_DATABASE ?? "compose");
    const allowed = neo4jExportDatabaseNameSet(process.env);
    if (!allowed.has(joined)) {
      const hint = parseExtraNeo4jGraphDatabaseNamesFromEnv(process.env).length === 0 ? ` Set ${NEO4J_EXTRA_GRAPH_DATABASES_ENV} to a comma-separated list of extra Bolt graph names (e.g. metabolism,mydb).` : "";
      console.error(`[generate:neo4j] graph database must be one of: ${[...allowed].sort().join(", ")} (got ${JSON.stringify(joined)}; argv segments ${JSON.stringify(nameParts)}).${hint}`);
      return false;
    }

    const technology = joined;
    const database = process.env.NEO4J_DATABASE ?? joined;
    const outDir = join(this.repoRoot, ".repo", "🛂");
    mkdirSync(outDir, { recursive: true });

    const finalAbs = join(outDir, `${technology}.cypher`);
    const cacheDir = join(this.repoRoot, ".repo", "cache");
    mkdirSync(cacheDir, { recursive: true });
    const tmpAbs = join(cacheDir, `.generate-${technology}-${process.pid}.tmp.cypher`);

    const probe = this.runCypher(database, "RETURN 1 AS ok;");
    if (!probe.ok) {
      console.error(`[generate:neo4j] cannot reach database ${JSON.stringify(database)}:\n${probe.stderr || probe.stdout}`);
      return false;
    }

    if (existsSync(tmpAbs)) unlinkSync(tmpAbs);

    const result = this.apocExportCypherAllToAbsoluteFile(database, tmpAbs);
    if (!result.ok) {
      console.error(`[generate:neo4j] apoc.export.cypher.all failed:\n${result.message}`);
      return false;
    }

    if (!existsSync(tmpAbs)) {
      console.error(`[generate:neo4j] expected export file missing at ${tmpAbs} after APOC call.`);
      return false;
    }

    const body = readFileSync(tmpAbs, "utf8");
    unlinkSync(tmpAbs);
    this.writeGeneratedCypherBundle(technology, database, body, finalAbs);

    console.log(`[generate:neo4j] wrote ${finalAbs} (database ${database}).`);
    if (result.message) console.log(result.message);
    return true;
  }

  runFromArgv(argv: string[]): void {
    if (!this.tryExportFromArgv(argv)) process.exit(1);
  }
}
//#endregion 🔖generate-neo4j-gen

//#region 🔖Policy
/**
 * ⚖️ Wave 4 app-plugin consistency policy — the machine-checkable subset of the Wave 4 V1 (duplication),
 * V2 (structure), V3 (coupling) audit findings under `.repo/🎫/26/07/18/WAVE-4-*-AUDIT`, wired via
 * `repo/lib/js/nx-plugin.mjs` into the synthetic `breach-script_ts` nx lint target (`bun ./script.ts policy`).
 * Judgment-call findings (a real SDK/primitive gap, e.g. the terminology native/reuse Labels axis, or
 * puzzle's icon-based `tree_item_with_action`) are encoded as explicit low-priority allowlisted/tracked
 * breaches, never as a hard `policy` failure — see `POLICY_SDK_GAP_ALLOWLIST` below.
 */

//#region 🔧PolicyFsScan
const POLICY_SKIP_DIRS = new Set(["node_modules", ".git", ".repo", "target", "dist", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);

/** 🔎Repo-relative `…/plugin/rs` dirs holding a `lib.rs` + `Cargo.toml` (app plugin crates, plus `framework/plugin/rs` itself). */
function policyDiscoverPluginCrateDirs(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      if (!ent.isDirectory() || POLICY_SKIP_DIRS.has(ent.name)) continue;
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.name === "rs" && relDir.endsWith("/plugin")) {
        const childAbs = join(repoRoot, childRel);
        if (existsSync(join(childAbs, "lib.rs")) && existsSync(join(childAbs, "Cargo.toml"))) found.push(childRel);
        continue;
      }
      walk(childRel);
    }
  };
  walk("");
  return found.sort();
}
//#endregion 🔧PolicyFsScan

//#region 🔧PolicyRegionParsing
type PolicyRegionEvent = { kind: "open" | "close"; line: number; spaceAfterSlashes: boolean; label: string };
type PolicyRegionSpan = { label: string; closeLabel: string; startLine: number; endLine: number };
type PolicyModSpan = { name: string; startLine: number; endLine: number };

const POLICY_REGION_OPEN_RE = /^(\s*)\/\/(\s*)#region(?:\s+(.*))?\s*$/;
const POLICY_REGION_CLOSE_RE = /^(\s*)\/\/(\s*)#endregion(?:\s+(.*))?\s*$/;

function policyParseRegionEvents(lines: readonly string[]): PolicyRegionEvent[] {
  const events: PolicyRegionEvent[] = [];
  lines.forEach((line, i) => {
    const om = line.match(POLICY_REGION_OPEN_RE);
    if (om) {
      events.push({ kind: "open", line: i + 1, spaceAfterSlashes: (om[2] ?? "").length > 0, label: (om[3] ?? "").trim() });
      return;
    }
    const cm = line.match(POLICY_REGION_CLOSE_RE);
    if (cm) events.push({ kind: "close", line: i + 1, spaceAfterSlashes: (cm[2] ?? "").length > 0, label: (cm[3] ?? "").trim() });
  });
  return events;
}

function policyPairRegionSpans(events: readonly PolicyRegionEvent[]): PolicyRegionSpan[] {
  const stack: PolicyRegionEvent[] = [];
  const spans: PolicyRegionSpan[] = [];
  for (const ev of events) {
    if (ev.kind === "open") {
      stack.push(ev);
      continue;
    }
    const open = stack.pop();
    if (!open) continue;
    spans.push({ label: open.label, closeLabel: ev.label, startLine: open.line, endLine: ev.line });
  }
  return spans;
}

/**
 * 🧹Masks `"..."` string-literal contents and `'x'` char-literal contents (same length, so indices stay
 * aligned) — line-bounded (`\n` excluded from both classes) and the char-literal form requires exactly one
 * char/escape, so a Rust lifetime apostrophe (`&'static`, `'a`) never greedily pairs with an unrelated
 * quote elsewhere in the file (which would otherwise corrupt brace-counting across huge, unrelated spans).
 */
function policyMaskLiterals(line: string): string {
  return line.replace(/"(?:[^"\\\n]|\\.)*"/g, (m) => `"${" ".repeat(Math.max(0, m.length - 2))}"`).replace(/'(?:\\.|[^'\\\n])'/g, (m) => `'${" ".repeat(Math.max(0, m.length - 2))}'`);
}

const POLICY_MOD_OPEN_RE = /^\s*pub mod (\w+)\b.*\{\s*$/;

function policyParseModSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { name: string; startLine: number; depth: number }[] = [];
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const modMatch = raw.match(POLICY_MOD_OPEN_RE);
    if (modMatch) stack.push({ name: modMatch[1]!, startLine: i + 1, depth });
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      spans.push({ name: top.name, startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

function policyModAtLine(modSpans: readonly PolicyModSpan[], lineNo: number): string {
  const containing = modSpans.filter((s) => s.startLine <= lineNo && lineNo <= s.endLine);
  return containing.sort((a, b) => a.endLine - a.startLine - (b.endLine - b.startLine))[0]?.name ?? "";
}

const POLICY_MOD_ANY_OPEN_RE = /^\s*(?:pub\s+)?mod\s+(\w+)\b.*\{\s*$/;

/** 🧪Brace-spans of `#[cfg(test)] mod … { … }` blocks — synthetic test fixtures (e.g. `App::builder` in a unit test) aren't real app registrations. */
function policyTestModSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { name: string; startLine: number; depth: number; isTest: boolean }[] = [];
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const modMatch = raw.match(POLICY_MOD_ANY_OPEN_RE);
    if (modMatch) {
      const isTest = lines.slice(Math.max(0, i - 2), i).some((l) => /#\[cfg\(test\)\]/.test(l)) || modMatch[1] === "tests";
      stack.push({ name: modMatch[1]!, startLine: i + 1, depth, isTest });
    }
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      if (top.isTest) spans.push({ name: top.name, startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

/** 🏷️Strips a leading non-letter (emoji/sigil) prefix off a region label, e.g. "🔖Tests" -> "Tests". */
function policyLabelName(label: string): string {
  return label.replace(/^[^\p{L}]+/u, "").trim();
}
//#endregion 🔧PolicyRegionParsing

//#region 🔧PolicyFnParsing
/** 🧱Extracts a `{ … }` function body starting the brace-scan at/after `fromIdx`, string-literal-safe. */
function policyExtractFnBody(content: string, fromIdx: number): string {
  const masked = policyMaskLiterals(content);
  const openIdx = masked.indexOf("{", fromIdx);
  if (openIdx === -1) return "";
  let depth = 0;
  for (let i = openIdx; i < masked.length; i++) {
    if (masked[i] === "{") depth++;
    else if (masked[i] === "}") {
      depth--;
      if (depth === 0) return content.slice(openIdx, i + 1);
    }
  }
  return content.slice(openIdx);
}

function policyLineOfIndex(content: string, idx: number): number {
  return content.slice(0, idx).split("\n").length;
}

/** 🐫PascalCase(app id) + "App", e.g. "gis2d-play" -> "Gis2dPlayApp". */
function policyPascalAppStructName(id: string): string {
  const parts = id.split(/[-_]+/).filter(Boolean);
  return `${parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("")}App`;
}
//#endregion 🔧PolicyFnParsing

//#region 🔧PolicyAllowlists
/**
 * 🎫 Wave 4 V1 duplication audit (`.repo/🎫/26/07/18/WAVE-4-V1-DUPLICATION-HUNTER-AUDIT`): both crates
 * resolve a second "terminology" axis (native/reuse) the SDK's locale-only `app_labels!`/`LocaleLabels`
 * primitive can't express. Flagged for a Wave-4 decision (extend the primitive to two axes, or formally
 * accept the gap) — tracked here as a low-priority, non-failing breach until that decision lands.
 */
const POLICY_LABELS_TWO_AXIS_ALLOWLIST = new Set<string>(["cad/plugin/rs#CadLabels", "puzzle/plugin/rs#Puzzle2dLabels", "puzzle/plugin/rs#Puzzle3dLabels", "puzzle/plugin/rs#Puzzle5dLabels"]);

/**
 * 🎫 Wave 4 V1 duplication audit: puzzle's d3/d5 `tree_item_with_action` redefinitions add an `icon_id`
 * param the SDK's description-based primitive can't express (icon rendering) — documented real gap,
 * tracked here as a low-priority, non-failing breach rather than a should-fix duplicate.
 */
const POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST = new Set<string>(["puzzle/plugin/rs#d3", "puzzle/plugin/rs#d5"]);

/**
 * 🎫 Wave 4 V3 coupling audit (`.repo/🎫/26/07/18/WAVE-4-APP-TO-APP-COUPLING-AND-FRAMEWORK-IDENTITY-LEAK-AUDIT`):
 * these crates are neutral shared domain/library crates that also happen to ship their own minimal
 * playground app (documented via each crate's `AGENTS.md`) — depending on them is not app-to-app coupling.
 */
const POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST = new Set<string>(["flow_core", "flow_module_draw", "flow_module_brep", "trinity_jack", "trinity_ram", "mathematical_graph_drawing", "mathematical_geometry", "infinite_board_port_directed", "infinite_board_port_directed_dag"]);

/**
 * 🛡️Path prefixes (repo-relative) always allowed as plugin/rs dependency targets: generic shared
 * infra. `"protocol/"` was added at CW8 — every app now imports `protocol::` directly (CW7's import
 * sweep) the same way it already reaches `vcs`/`ui`/`framework`. Deliberately NOT `"db/"`: db stays
 * reachable only through the hub servers (and, behind a feature, `db_engine`), enforced by
 * `policyDbServerOnlyBreaches` instead of this always-allowed list.
 */
const POLICY_ALWAYS_ALLOWED_DEP_PREFIXES = ["framework/", "ui/", "vcs/", "store/", "playbook/", "protocol/", "repo/"];

/**
 * 🎫 dsl/ derive-engine migration lock step: technologies whose example/*.json fixture has not yet
 * been converted to its own DSL-text extension (e.g. `.puzzle2d`, `.flow`). Tracked here as a documented,
 * still-open follow-up rather than a silent exception — remove an entry once that technology's fixture
 * is migrated (see an already-migrated sibling crate's `*_fixture` round-trip test for the pattern).
 */
const POLICY_JSON_FIXTURE_ALLOWLIST = new Set<string>([]);

/** 🛡️Path prefixes exempt from the no-JSON-fixture rule structurally — not a technology document fixture (e.g. `coda/`'s `example/` holds a whole simulated ACC project's run/iteration artifacts, unrelated to the dsl/ migration). */
const POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST = ["coda/"];

/**
 * 🎫 pack/ binary-document-layer rollout lock step (`.repo/🎫/26/07/27/
 * PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS`): every `*.rs` file below that already calls
 * `assert_dsl_round_trip(`/`assert_document_text_round_trip(` but does not yet ALSO call
 * `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(` on the same fixtures — seeded at
 * wave 1 with every file that fails the check today (wave 0 only built the `pack` crate family itself;
 * wave 1 only wired `vcs`/`dsl_derive`/`framework`, proving the mechanism on `vcs/rs/lib.rs`'s own
 * `DemoProjection` fixture, which is why `vcs/rs/lib.rs` is NOT in this list). Wave 2's per-app-family
 * agents add the pack-equivalence assertions beside each technology's existing DSL round-trip tests and
 * remove that file from this list; wave 3 verifies it has shrunk to empty.
 */
const POLICY_PACK_COMPLETENESS_ALLOWLIST = new Set<string>([]);

/**
 * 🎫 CW7 command-envelope law lock step (`.repo/🎫/26/07/27/
 * INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`): every `*.rs` file that already calls
 * `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(` but does not yet ALSO call
 * `vcs::test_support::assert_command_envelope_round_trip` (added in CW7) on the same fixtures — seeded
 * at CW8 with every file that fails the check today, exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST`
 * was seeded for the dsl/pack lock step (`vcs/rs/lib.rs` itself proves the mechanism on its own
 * `DemoProjection`/`LossyOperation` fixtures, which is why it is NOT in this list). Remove an entry
 * once that file adds the command-envelope-round-trip call.
 */
const POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST = new Set<string>([
  "animate/present/rs/lib.rs",
  "architect/program/rs/lib.rs",
  "cad/rs/lib.rs",
  "compose/client/lib/rs/lib.rs",
  "draw/rs/lib.rs",
  "dsl/rs/lib.rs",
  "fem/2d/rs/lib.rs",
  "fem/3d/rs/lib.rs",
  "flow/core/rs/lib.rs",
  "forms/rs/lib.rs",
  "framework/product/os/core/rs/lib.rs",
  "gis/plugin/rs/lib.rs",
  "imperative/core/rs/lib.rs",
  "infinite/board/port/directed/dag/rs/lib.rs",
  "layout/rs/lib.rs",
  "lowpoly/core/rs/lib.rs",
  "mathematical/plugin/rs/lib.rs",
  "norm/core/rs/lib.rs",
  "norm/din/4108/rs/lib.rs",
  "norm/din/en/16798/rs/lib.rs",
  "norm/din/v/18599/rs/lib.rs",
  "norm/en/1990/rs/lib.rs",
  "norm/en/1991/rs/lib.rs",
  "norm/en/1992/rs/lib.rs",
  "norm/en/1993/rs/lib.rs",
  "norm/en/1994/rs/lib.rs",
  "norm/en/1995/rs/lib.rs",
  "norm/en/1996/rs/lib.rs",
  "norm/en/1997/rs/lib.rs",
  "norm/en/1998/rs/lib.rs",
  "norm/en/1999/rs/lib.rs",
  "norm/iso/16757/rs/lib.rs",
  "norm/vdi/3805/rs/lib.rs",
  "note/plugin/rs/lib.rs",
  "playbook/module/procedural/rs/lib.rs",
  "playbook/rs/lib.rs",
  "procedural/2d/rs/lib.rs",
  "procedural/3d/rs/lib.rs",
  "process/3d/rs/lib.rs",
  "puzzle/2d/rs/lib.rs",
  "puzzle/3d/rs/lib.rs",
  "puzzle/5d/rs/lib.rs",
  "puzzle/plugin/rs/lib.rs",
  "raster/plugin/rs/lib.rs",
  "reasoning/mindmap/rs/lib.rs",
  "remodel/rs/lib.rs",
  "s/plugin/rs/lib.rs",
  "s/rs/lib.rs",
  "sequence/core/rs/lib.rs",
  "shooting/rs/lib.rs",
  "sourcing/curate/rs/lib.rs",
  "trinity/ram/rs/lib.rs",
  "trinity/rewrite/engine/rs/lib.rs",
  "vcs/plugin/rs/lib.rs",
  "writer/rs/lib.rs",
]);

/**
 * 🎫 dsl/ derive-engine migration lock step: known generic bridges whose `DocumentDsl`/`OpText` coverage
 * cannot be seen as a `#[derive(dsl::Dsl...)]` attribute directly on the type (a blanket/generic impl
 * elsewhere covers them) — accepted as DSL-complete by `policyDslCompletenessBreaches` without a
 * hand-rolled-impl grep hit under that exact type name.
 * - `Value` (`serde_json::Value`): blanket `impl DocumentDsl for serde_json::Value` in `vcs/rs/lib.rs`
 *   (the schema-less escape hatch for a technology whose `DocumentApp::Projection` predates its own
 *   typed DSL derive — see `puzzle_2d`'s `🔖ValueBridge` region).
 * - `SetDocumentOperation` (`norm_core::SetDocumentOperation<D>`): one hand-rolled generic
 *   `impl<D: DocumentDsl> OpText for SetDocumentOperation<D>` in `norm/core/rs/lib.rs`, shared by every
 *   norm family's `Operation` type instead of a per-family derive.
 */
const POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST = new Set<string>(["Value", "SetDocumentOperation"]);
//#endregion 🔧PolicyAllowlists

//#region 🔧PolicyRuleRegionFormat
/** 📏V2 rule: `//#region 🔖Name` / `//#endregion 🔖Name` (no space after `//`), tests region must be `🧪Tests`. */
function policyRegionFormatBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const events = policyParseRegionEvents(lines);
  for (const ev of events) {
    const marker = ev.kind === "open" ? "region" : "endregion";
    if (ev.spaceAfterSlashes) {
      breaches.push({
        id: `region-format-space-${scope}-${ev.line}`,
        summary: `Region marker has a space after "//" (must be "//#${marker}", not "// #${marker}")`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: ev.line,
        priority: "high",
        reason: "Wave 4 V2 structure audit: region markers must be //#region 🔖Name / //#endregion 🔖Name, no space after //.",
        solution: `Remove the space between "//" and "#${marker}" on line ${ev.line}.`,
      });
    }
    if (!ev.label) {
      breaches.push({
        id: `region-format-missing-label-${scope}-${ev.line}`,
        summary: `//#${marker} marker is missing its label`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: ev.line,
        priority: "medium",
        reason: "Every region open/close marker must carry its emoji+name label so pairs stay self-documenting.",
        solution: `Add the matching label after //#${marker} on line ${ev.line}.`,
      });
    }
  }
  for (const span of policyPairRegionSpans(events)) {
    const bare = policyLabelName(span.label);
    if (/^tests?$/i.test(bare) && span.label !== "🧪Tests") {
      breaches.push({
        id: `region-tests-label-${scope}-${span.startLine}`,
        summary: `Tests region labeled "${span.label}" must be exactly "🧪Tests"`,
        kind: "app-plugin/region-tests-label",
        scope,
        line: span.startLine,
        priority: "medium",
        reason: "Wave 4 V2 structure audit: the tests region sigil is reserved as 🧪Tests.",
        solution: `Rename the region label at line ${span.startLine} (and its matching //#endregion) to "🧪Tests".`,
      });
    }
    if (span.closeLabel && span.closeLabel !== span.label) {
      breaches.push({
        id: `region-label-mismatch-${scope}-${span.startLine}`,
        summary: `//#endregion label "${span.closeLabel}" does not match its //#region label "${span.label}"`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: span.endLine,
        priority: "medium",
        reason: "Region open/close labels must match so the pairing stays unambiguous.",
        solution: `Fix the //#endregion label at line ${span.endLine} to read "${span.label}".`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleRegionFormat

//#region 🔧PolicyRuleManifestRegion
/** 📏V2 rule: every `App::builder(...)` call must be enclosed in a region labeled exactly `🔖Manifest`. */
function policyManifestRegionBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const spans = policyPairRegionSpans(policyParseRegionEvents(lines));
  const testSpans = policyTestModSpans(lines);
  const breaches: BreachRecord[] = [];
  lines.forEach((line, i) => {
    if (!/App::builder\(/.test(line)) return;
    const lineNo = i + 1;
    if (testSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine)) return; // synthetic test fixture, not a real app registration
    const enclosed = spans.some((s) => s.label === "🔖Manifest" && s.startLine <= lineNo && lineNo <= s.endLine);
    if (!enclosed) {
      breaches.push({
        id: `manifest-region-${scope}-${lineNo}`,
        summary: `App::builder(...) call is not enclosed in a "🔖Manifest" region`,
        kind: "app-plugin/manifest-region",
        scope,
        line: lineNo,
        priority: "medium",
        reason: "Wave 4 V2 structure audit: each app's App::builder(...) registration must live inside its own //#region 🔖Manifest.",
        solution: `Wrap the App::builder(...) call at line ${lineNo} in a dedicated //#region 🔖Manifest / //#endregion 🔖Manifest.`,
      });
    }
  });
  return breaches;
}
//#endregion 🔧PolicyRuleManifestRegion

//#region 🔧PolicyRuleStructNaming
/** 📏V2 rule: the `DocumentApp` struct backing `<PREFIX>_APP_ID` must be named `PascalCase(id) + "App"`. */
function policyStructNamingBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const idsByConst = new Map<string, string>();
  const constRe = /const\s+([A-Z][A-Z0-9_]*_APP_ID)\s*:\s*&str\s*=\s*"([^"]+)"/g;
  let cm: RegExpExecArray | null;
  while ((cm = constRe.exec(content))) idsByConst.set(cm[1]!, cm[2]!);

  const implRe = /impl\s+DocumentApp\s+for\s+(\w+)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = implRe.exec(content))) {
    const structName = m[1]!;
    const window = content.slice(m.index + m[0].length, m.index + m[0].length + 2000);
    const constMatch = window.match(/\b([A-Z][A-Z0-9_]*_APP_ID)\b/);
    const id = constMatch ? idsByConst.get(constMatch[1]!) : undefined;
    if (!id) continue;
    const expected = policyPascalAppStructName(id);
    if (structName !== expected) {
      breaches.push({
        id: `struct-naming-${scope}-${structName}`,
        summary: `App struct "${structName}" for id "${id}" should be named "${expected}"`,
        kind: "app-plugin/struct-naming",
        scope,
        line: policyLineOfIndex(content, m.index),
        priority: "medium",
        reason: 'Wave 4 V2 structure audit: app struct name must be PascalCase(app id) + "App".',
        solution: `Rename struct ${structName} to ${expected} (and all its references) in ${scope}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleStructNaming

//#region 🔧PolicyRuleModLayout
/** 📏V2 rule: crates declaring 2+ apps must isolate each `impl DocumentApp for` inside its own `pub mod`. */
function policyModLayoutBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const implLines: number[] = [];
  const implRe = /impl\s+DocumentApp\s+for\s+\w+\s*\{/;
  lines.forEach((line, i) => {
    if (implRe.test(line)) implLines.push(i + 1);
  });
  if (implLines.length < 2) return [];
  const modSpans = policyParseModSpans(lines);
  const breaches: BreachRecord[] = [];
  for (const lineNo of implLines) {
    if (!modSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine)) {
      breaches.push({
        id: `mod-layout-${scope}-${lineNo}`,
        summary: `Multi-app crate declares an app (DocumentApp impl at line ${lineNo}) outside any "pub mod app_<name>" wrapper`,
        kind: "app-plugin/mod-layout",
        scope,
        line: lineNo,
        priority: "high",
        reason: "Wave 4 V2 structure audit: crates with 2+ apps must isolate each app inside its own pub mod (app_2d/app_3d, d2/d3/d5, ...).",
        solution: `Wrap the app declared at line ${lineNo} in its own pub mod block, matching the pattern used by gis/procedural/trinity/puzzle/s.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleModLayout

//#region 🔧PolicyRuleSdkMechanisms
/** 🔎Resolves a `use ... importedName as alias` rename so delegation checks accept the aliased call form too. */
function policyResolveImportAlias(content: string, importedName: string): string | undefined {
  return content.match(new RegExp(`\\b${importedName}\\s+as\\s+(\\w+)\\b`))?.[1];
}

/** 📏V1 rule: local `fn selection_ids` must delegate to `semio_framework_plugin::selection_ids` before adding a fallback key. */
function policySelectionIdsBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const alias = policyResolveImportAlias(content, "selection_ids");
  const re = /fn\s+selection_ids\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) {
    const body = policyExtractFnBody(content, m.index);
    if (body.includes("semio_framework_plugin::selection_ids(") || (alias && new RegExp(`\\b${alias}\\(`).test(body))) continue;
    const lineNo = policyLineOfIndex(content, m.index);
    breaches.push({
      id: `sdk-selection-ids-${scope}-${lineNo}`,
      summary: `Local "fn selection_ids" does not delegate to semio_framework_plugin::selection_ids`,
      kind: "app-plugin/sdk-selection-ids",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: apps needing an extra fallback key must still call the SDK's selection_ids core first (see procedural::app_3d, reasoning/mindmap for the reference shape).",
      solution: `Rewrite selection_ids at line ${lineNo} to call semio_framework_plugin::selection_ids(args) first and only add the extra fallback key on top.`,
    });
  }
  return breaches;
}

/** 📏V1 rule: local `fn new_app`/`fn new_app_with_registry`/`fn meta` must stay thin typed delegates to the SDK testkit (allowing a `::<Turbofish>` generic before the call's `(`). */
function policyTestkitDelegateBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const specs: readonly { re: RegExp; mustRe: RegExp; label: string }[] = [
    { re: /fn\s+new_app_with_registry\s*[<(]/g, mustRe: /testkit::new_app_with_registry(?:::<[^>]*>)?\s*\(/, label: "testkit::new_app_with_registry" },
    { re: /fn\s+new_app\s*[<(]/g, mustRe: /testkit::new_app(?:::<[^>]*>)?\s*\(/, label: "testkit::new_app" },
    { re: /fn\s+meta\s*\(/g, mustRe: /testkit::meta\s*\(/, label: "testkit::meta" },
  ];
  for (const { re, mustRe, label } of specs) {
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) {
      const body = policyExtractFnBody(content, m.index);
      if (mustRe.test(body)) continue;
      const lineNo = policyLineOfIndex(content, m.index);
      breaches.push({
        id: `sdk-testkit-delegate-${scope}-${lineNo}`,
        summary: `Local "${m[0]!.trim()}" does not delegate to semio_framework_plugin::${label}`,
        kind: "app-plugin/sdk-testkit-delegate",
        scope,
        line: lineNo,
        priority: "medium",
        reason: "Wave 4 V1 duplication audit: per-type new_app/new_app_with_registry/meta helpers must stay thin typed delegates to the SDK testkit.",
        solution: `Make the helper at line ${lineNo} call ${label}(...) instead of reimplementing it.`,
      });
    }
  }
  return breaches;
}

/** 📏V1 rule: local `tree_item_with_action` redefinitions need an allowlisted SDK gap; other `tree_item_*` wrappers must delegate to it. */
function policyTreeItemBreaches(scope: string, content: string, lines: readonly string[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const modSpans = policyParseModSpans(lines);

  const redefRe = /fn\s+tree_item_with_action\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = redefRe.exec(content))) {
    const lineNo = policyLineOfIndex(content, m.index);
    const mod = policyModAtLine(modSpans, lineNo);
    if (POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST.has(`${scope}#${mod}`)) continue;
    breaches.push({
      id: `sdk-tree-item-redefinition-${scope}-${lineNo}`,
      summary: `Local "fn tree_item_with_action" shadows the SDK primitive of the same name`,
      kind: "app-plugin/sdk-tree-item",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: redefining tree_item_with_action locally is only accepted for a documented SDK gap (e.g. icon rendering).",
      solution: `Delete this and call semio_framework_plugin::tree_item_with_action directly, or if it exists for a genuine SDK gap, add it to POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST citing the ticket.`,
    });
  }

  const wrapRe = /fn\s+(tree_item_\w+)\s*\(/g;
  while ((m = wrapRe.exec(content))) {
    const fnName = m[1]!;
    if (fnName === "tree_item_with_action") continue;
    const signatureEnd = content.indexOf(")", m.index);
    const signature = signatureEnd === -1 ? "" : content.slice(m.index, content.indexOf("{", signatureEnd));
    if (!signature.includes("ActionDescriptor")) continue; // structurally different helper (e.g. no action param) — not a delegation candidate
    const body = policyExtractFnBody(content, m.index);
    if (body.includes("tree_item_with_action(")) continue;
    const lineNo = policyLineOfIndex(content, m.index);
    breaches.push({
      id: `sdk-tree-item-wrapper-${scope}-${lineNo}`,
      summary: `Local "fn ${fnName}" does not delegate to tree_item_with_action`,
      kind: "app-plugin/sdk-tree-item",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: tree_item_* wrapper helpers must build on tree_item_with_action via struct-update syntax.",
      solution: `Rewrite ${fnName} at line ${lineNo} to construct its result via tree_item_with_action(...) plus a struct-update override.`,
    });
  }
  return breaches;
}

/** 📏V1 rule: `struct XLabels` must be defined inside `semio_framework_plugin::app_labels! { ... }`, unless allowlisted as a documented SDK gap. */
function policyLabelsStructBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const re = /struct\s+(\w*Labels)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) {
    const structName = m[1]!;
    // 🌱 Line-based lookback (not a fixed char window): the `app_labels!` invocation can sit several
    // lines above the struct decl behind a multi-line /// doc comment (e.g. remodel, procedural).
    const lineNo = policyLineOfIndex(content, m.index);
    const precedingLines = content.split("\n").slice(Math.max(0, lineNo - 8), lineNo - 1);
    if (precedingLines.some((l) => l.includes("app_labels!"))) continue;
    const allowed = POLICY_LABELS_TWO_AXIS_ALLOWLIST.has(`${scope}#${structName}`);
    breaches.push({
      id: `sdk-labels-struct-${scope}-${structName}`,
      summary: allowed ? `"struct ${structName}" is a tracked SDK-primitive gap (terminology axis) — Wave 4 decision pending` : `"struct ${structName}" hand-rolls its label set instead of semio_framework_plugin::app_labels!/LocaleLabels`,
      kind: "app-plugin/sdk-labels-struct",
      scope,
      line: lineNo,
      priority: allowed ? "low" : "medium",
      reason: allowed
        ? "Wave 4 V1 duplication audit flagged this for a Wave-4 design decision (extend LocaleLabels/app_labels! to a two-axis resolver, or formally accept the gap) — tracked, not a lint failure."
        : "Wave 4 V1 duplication audit: hand-rolled Labels structs (NATIVE/REUSE-style consts + resolver fn) should route through semio_framework_plugin::app_labels!/LocaleLabels unless there's a documented SDK-primitive gap.",
      solution: allowed ? `See .repo/🎫/26/07/18/WAVE-4-V1-DUPLICATION-HUNTER-AUDIT for the pending decision; if formally accepted, keep this allowlisted with that citation.` : `Route ${structName} through semio_framework_plugin::app_labels! { ... }, or if it needs a second axis, add it to POLICY_LABELS_TWO_AXIS_ALLOWLIST citing a ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRuleSdkMechanisms

//#region 🔧PolicyRuleCargoArtifacts
/** 📏V2 rule: no stray `Cargo.lock`/`target/` checked into a `…/plugin/rs` crate dir. */
function policyCargoArtifactBreaches(repoRoot: string, crateDirs: readonly string[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const dir of crateDirs) {
    for (const stray of ["Cargo.lock", "target"]) {
      if (existsSync(join(repoRoot, dir, stray))) {
        breaches.push({
          id: `stray-cargo-artifact-${dir}-${stray}`,
          summary: `Stray "${stray}" checked into ${dir}/`,
          kind: "app-plugin/stray-cargo-artifact",
          scope: dir,
          priority: "high",
          reason: "Wave 4 V2 structure audit: */plugin/rs crates must not carry their own Cargo.lock or target/ (workspace-managed).",
          solution: `Remove ${dir}/${stray} (and add it to .gitignore if missing).`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleCargoArtifacts

//#region 🔧PolicyRuleAppCoupling
const POLICY_CARGO_DEP_RE = /^([\w.-]+)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$/gm;

/** 📏V3 rule: a `…/plugin/rs` crate's path-dependencies must not reach into another app's plugin crate (blocking) or undocumented domain crate (tracked). */
function policyAppCouplingBreaches(repoRoot: string, crateDirs: readonly string[]): BreachRecord[] {
  const appTops = crateDirs.map((d) => d.replace(/\/plugin\/rs$/, ""));
  const breaches: BreachRecord[] = [];
  for (const dir of crateDirs) {
    const selfTop = dir.replace(/\/plugin\/rs$/, "");
    const cargoTomlAbs = join(repoRoot, dir, "Cargo.toml");
    if (!existsSync(cargoTomlAbs)) continue;
    const text = readFileSync(cargoTomlAbs, "utf8");
    let m: RegExpExecArray | null;
    POLICY_CARGO_DEP_RE.lastIndex = 0;
    while ((m = POLICY_CARGO_DEP_RE.exec(text))) {
      const depName = m[1]!;
      const depPath = m[2]!;
      const resolvedAbs = resolve(join(repoRoot, dir), depPath);
      const resolvedRel = relative(repoRoot, resolvedAbs).split("\\").join("/");
      if (POLICY_ALWAYS_ALLOWED_DEP_PREFIXES.some((p) => resolvedRel.startsWith(p))) continue;
      const otherTop = appTops.find((top) => top !== selfTop && (resolvedRel === top || resolvedRel.startsWith(`${top}/`)));
      if (!otherTop) continue;
      const otherPluginDir = `${otherTop}/plugin/rs`;
      if (resolvedRel === otherPluginDir) {
        breaches.push({
          id: `app-coupling-plugin-${dir}-${depName}`,
          summary: `${dir} depends directly on another app's plugin crate (${depName} -> ${resolvedRel})`,
          kind: "app-plugin/app-coupling",
          scope: dir,
          priority: "high",
          reason: "Wave 4 V3 coupling audit: no plugin crate may depend on another app's plugin crate.",
          solution: `Remove the "${depName}" dependency from ${dir}/Cargo.toml, or move the shared logic into a neutral domain crate outside any app's plugin/rs.`,
        });
        continue;
      }
      if (POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST.has(depName)) continue;
      breaches.push({
        id: `app-coupling-domain-${dir}-${depName}`,
        summary: `${dir} depends on a crate under another app's tree (${depName} -> ${resolvedRel}) not yet vetted as shared infra`,
        kind: "app-plugin/app-coupling",
        scope: dir,
        priority: "low",
        reason: "Wave 4 V3 coupling audit: dependencies into another app's folder are only acceptable for documented neutral shared domain crates (see that app's AGENTS.md); everything else needs a look.",
        solution: `If "${depName}" is genuinely a shared domain/library crate (documented via AGENTS.md), add it to POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST citing the ticket; otherwise remove the dependency.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleAppCoupling

//#region 🔧PolicyRuleNoJsonFixtures
/** 🔎Repo-wide `.../example/*.json` file paths (repo-relative), skipping the same dirs `policyDiscoverPluginCrateDirs` skips. */
function policyDiscoverExampleJsonFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (relDir.split("/").pop() === "example" && ent.name.endsWith(".json")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 📏dsl/ migration lock-step rule: no example/*.json fixture may exist — every technology's example fixtures moved to its own DSL-text extension (`.puzzle2d`, `.flow`, `.draw`, …); a documented allowlist covers not-yet-migrated technologies. */
function policyJsonFixtureBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverExampleJsonFiles(repoRoot)) {
    if (POLICY_JSON_FIXTURE_ALLOWLIST.has(relPath)) continue;
    if (POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST.some((p) => relPath.startsWith(p))) continue;
    breaches.push({
      id: `no-json-fixture-${relPath}`,
      summary: `"${relPath}" is a JSON example fixture — technologies now use DSL-text fixtures`,
      kind: "dsl-migration/no-json-fixture",
      scope: relPath,
      priority: "high",
      reason: "The dsl/ derive-engine migration moved every technology's example fixtures from JSON to its own DSL-text extension (e.g. .puzzle2d, .flow, .draw); new */example/*.json files should never be added.",
      solution: `Convert ${relPath} to its technology's DSL-text extension (see an already-migrated sibling crate's *_fixture round-trip test for the pattern), or if this technology is not yet converted, add it to POLICY_JSON_FIXTURE_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRuleNoJsonFixtures

//#region 🔧PolicyRuleOpsGrammar
/** 🔎Repo-wide `*.ops` file paths (repo-relative). */
function policyDiscoverOpsFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".ops")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🏷️`.ops` op-log structural line keywords (see `vcs::print_document_text`/`parse_document_text`). */
const POLICY_OPS_STRUCTURAL_KEYWORDS = ["@doc", "@edit", "@change", "@checkpoint", "@alternative", "@active"];

/**
 * 📏dsl/ migration lock-step rule: basic `.ops`-file grammar sanity — one structural `@doc`/`@edit`/
 * `@change`/`@checkpoint`/`@alternative`/`@active` line, or one 2-space-indented op-text line under a
 * preceding `@edit`, per line; never blank. Forward-looking: no `*.ops` fixture exists in the repo yet
 * (`FolderTextStorage`'s `.ops` companion file is additive/unwired — see `vcs/rs/lib.rs`), but the check
 * is ready the day one lands.
 */
function policyOpsGrammarBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverOpsFiles(repoRoot)) {
    const text = readFileSync(join(repoRoot, relPath), "utf8");
    const rawLines = text.split(/\r?\n/);
    const lines = rawLines.length > 0 && rawLines[rawLines.length - 1] === "" ? rawLines.slice(0, -1) : rawLines;
    lines.forEach((line, i) => {
      const lineNo = i + 1;
      if (line.trim().length === 0) {
        breaches.push({
          id: `ops-grammar-blank-${relPath}-${lineNo}`,
          summary: `"${relPath}" line ${lineNo} is blank`,
          kind: "dsl-migration/ops-grammar",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: "The .ops op-log grammar is one structural @-line or one indented op-text line per line — no blank lines.",
          solution: `Remove the blank line at ${relPath}:${lineNo}.`,
        });
        return;
      }
      const isStructural = POLICY_OPS_STRUCTURAL_KEYWORDS.some((kw) => line.startsWith(kw));
      const isIndentedOp = line.startsWith("  ") && line.trim().length > 0;
      if (!isStructural && !isIndentedOp) {
        breaches.push({
          id: `ops-grammar-line-${relPath}-${lineNo}`,
          summary: `"${relPath}" line ${lineNo} is neither a structural @-line nor a 2-space-indented op-text line`,
          kind: "dsl-migration/ops-grammar",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: "Every .ops line must be a recognized @doc/@edit/@change/@checkpoint/@alternative/@active structural line or a 2-space-indented op-text line under an @edit block.",
          solution: `Fix ${relPath}:${lineNo} to match the .ops grammar (see vcs::print_document_text/parse_document_text).`,
        });
      }
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRuleOpsGrammar

//#region 🔧PolicyRuleDslCompleteness
/** 🔎Repo-wide `*.rs` file paths (repo-relative), skipping the same dirs `policyDiscoverPluginCrateDirs` skips. */
function policyAllRustFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".rs")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

type PolicyDocumentAppUsage = { scope: string; line: number; projectionType: string; operationType: string };

const POLICY_DOCUMENT_APP_IMPL_RE = /impl\s+DocumentApp\s+for\s+\w+\s*\{/g;

/** 🔎Extracts `type Projection = X;` / `type Operation = Y;` (generic args stripped) from every `impl DocumentApp for … { … }` block in one file's content. */
function policyDocumentAppUsages(scope: string, content: string): PolicyDocumentAppUsage[] {
  const usages: PolicyDocumentAppUsage[] = [];
  let m: RegExpExecArray | null;
  POLICY_DOCUMENT_APP_IMPL_RE.lastIndex = 0;
  while ((m = POLICY_DOCUMENT_APP_IMPL_RE.exec(content))) {
    const body = policyExtractFnBody(content, m.index);
    const projectionMatch = body.match(/type\s+Projection\s*=\s*([\w:<>]+)\s*;/);
    const operationMatch = body.match(/type\s+Operation\s*=\s*([\w:<>]+)\s*;/);
    if (!projectionMatch || !operationMatch) continue;
    usages.push({
      scope,
      line: policyLineOfIndex(content, m.index),
      projectionType: projectionMatch[1]!.split("::").pop()!.replace(/<.*$/, ""),
      operationType: operationMatch[1]!.split("::").pop()!.replace(/<.*$/, ""),
    });
  }
  return usages;
}

const POLICY_STRUCT_OR_ENUM_DECL_RE = /^\s*(?:pub\s+)?(?:struct|enum)\s+(\w+)\b/gm;
const POLICY_DSL_DERIVE_RE = /derive\([^)]*\bDsl(?:Document|Ops|Record|Enum|Scalar)\b[^)]*\)/;
// 🎞️ CW9: `OpText` moved from `vcs` to `protocol` (see protocol/command/rs) — CW7's import-path sweep
// rewrote every hand-rolled `impl vcs::OpText for X` to `impl protocol::OpText for X`, so this must
// recognize both prefixes (`DocumentDsl` stays vcs-owned, but allowing `protocol::` there too is
// harmless since no such impl exists).
const POLICY_HAND_ROLLED_IMPL_RE = /impl(?:<[^>]*>)?\s+(?:vcs::|protocol::)?(DocumentDsl|OpText)\s+for\s+(?:vcs::|protocol::)?(\w+)/g;

/** 🧬One O(total content) pass building every type name that's DSL-complete — via `#[derive(dsl::Dsl…)]` a few lines above its own `struct`/`enum` declaration, or a hand-rolled `impl (vcs::|protocol::)?DocumentDsl`/`impl (vcs::|protocol::)?OpText` (an explicit generic impl also counts, e.g. `impl<D> OpText for SetDocumentOperation<D>`) — split by trait since a type may satisfy one but not the other. */
function policyDslCompleteTypeNames(files: readonly { content: string }[]): { documentDsl: Set<string>; opText: Set<string> } {
  const documentDsl = new Set<string>();
  const opText = new Set<string>();
  for (const { content } of files) {
    const lines = content.split("\n");
    let m: RegExpExecArray | null;
    POLICY_STRUCT_OR_ENUM_DECL_RE.lastIndex = 0;
    while ((m = POLICY_STRUCT_OR_ENUM_DECL_RE.exec(content))) {
      const lineNo = policyLineOfIndex(content, m.index);
      const precedingLines = lines.slice(Math.max(0, lineNo - 6), lineNo - 1).join("\n");
      if (POLICY_DSL_DERIVE_RE.test(precedingLines)) {
        documentDsl.add(m[1]!);
        opText.add(m[1]!);
      }
    }
    POLICY_HAND_ROLLED_IMPL_RE.lastIndex = 0;
    while ((m = POLICY_HAND_ROLLED_IMPL_RE.exec(content))) {
      (m[1] === "DocumentDsl" ? documentDsl : opText).add(m[2]!);
    }
  }
  return { documentDsl, opText };
}

const POLICY_USE_ALIAS_RE = /\b(\w+)\s+as\s+(\w+)\b/g;

/**
 * 🔎One O(total content) pass building every `RealName as AliasName` rename seen in any `use` item
 * (e.g. `use raster_core::{RasterProjection as RasterDocument};`) — a technology's block-kind wrapper
 * commonly re-exports another crate's already-DSL-complete type under a locally-meaningful alias
 * (`forms`'s `PlaybookSpec as FormSpec`, `raster/plugin`'s `RasterProjection as RasterDocument`), so a
 * plain per-file struct/impl scan alone would report a false gap on the alias name.
 */
function policyTypeAliasMap(files: readonly { content: string }[]): Map<string, string> {
  const aliasOf = new Map<string, string>();
  for (const { content } of files) {
    let m: RegExpExecArray | null;
    POLICY_USE_ALIAS_RE.lastIndex = 0;
    while ((m = POLICY_USE_ALIAS_RE.exec(content))) aliasOf.set(m[2]!, m[1]!);
  }
  return aliasOf;
}

/** 🔎Resolves `typeName` through `aliasOf` (`RealName as AliasName` renames) up to a handful of hops, so an alias chain still bottoms out at its real declaration. */
function policyResolveAlias(aliasOf: ReadonlyMap<string, string>, typeName: string): string {
  let resolved = typeName;
  for (let hop = 0; hop < 5; hop++) {
    const next = aliasOf.get(resolved);
    if (!next || next === resolved) break;
    resolved = next;
  }
  return resolved;
}

/**
 * 📏dsl/ migration lock-step rule: every `impl DocumentApp for X` app's `Projection`/`Operation` type
 * must be DSL-complete — `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type itself (or
 * on the real type behind a `RealName as AliasName` import rename), a hand-rolled `impl DocumentDsl`/
 * `impl OpText`, or a documented generic bridge (see `POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`).
 * Advisory/textual — the real compile-time gate is `DocumentApp`'s `Projection: vcs::DocumentDsl` /
 * `Operation: vcs::OpText` bounds in `framework/plugin/rs/lib.rs`; this catches the same gap without
 * needing a full `cargo build`. A single pass builds the DSL-complete type-name sets once
 * (`policyDslCompleteTypeNames`) so checking every app's usage stays O(1) instead of re-scanning the
 * whole corpus per usage.
 */
function policyDslCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const paths = policyAllRustFiles(repoRoot);
  const files = paths.map((relPath) => ({ relPath, content: readFileSync(join(repoRoot, relPath), "utf8") }));
  const complete = policyDslCompleteTypeNames(files);
  const aliasOf = policyTypeAliasMap(files);

  for (const { relPath, content } of files) {
    for (const usage of policyDocumentAppUsages(relPath, content)) {
      const checks: readonly { label: "Projection" | "Operation"; typeName: string; trait: "DocumentDsl" | "OpText"; completeNames: Set<string> }[] = [
        { label: "Projection", typeName: usage.projectionType, trait: "DocumentDsl", completeNames: complete.documentDsl },
        { label: "Operation", typeName: usage.operationType, trait: "OpText", completeNames: complete.opText },
      ];
      for (const { label, typeName, trait, completeNames } of checks) {
        if (POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST.has(typeName)) continue;
        if (completeNames.has(typeName)) continue;
        if (completeNames.has(policyResolveAlias(aliasOf, typeName))) continue;
        breaches.push({
          id: `dsl-completeness-${relPath}-${usage.line}-${label}`,
          summary: `"${usage.scope}"'s DocumentApp::${label} = ${typeName} has neither a #[derive(dsl::Dsl...)] nor a hand-rolled impl ${trait}`,
          kind: "dsl-migration/completeness",
          scope: relPath,
          line: usage.line,
          priority: "high",
          reason: "Every DocumentApp app's Projection must implement vcs::DocumentDsl and Operation must implement vcs::OpText (compiler-enforced since the Lock step) — via #[derive(dsl::DslDocument)]/#[derive(dsl::DslOps)], a hand-rolled impl, or a documented generic bridge.",
          solution: `Add #[derive(dsl::DslDocument)] (Projection) / #[derive(dsl::DslOps)] (Operation) to ${typeName}, write a hand-rolled impl ${trait} for ${typeName}, or if it's a genuine generic bridge add it to POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST citing why.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleDslCompleteness

//#region 🔧PolicyRulePackCompleteness
/**
 * 📏pack/ rollout lock-step rule: every `*.rs` file that calls `assert_dsl_round_trip(`/
 * `assert_document_text_round_trip(` must ALSO call `assert_dsl_pack_equivalence(`/
 * `assert_document_pack_round_trip(` somewhere in the same file — dsl and pack are two projections of
 * the same value model (see `vcs::DocumentPack`'s LAW doc comment), so a technology proving its DSL
 * round trip without also proving its pack round trip is an incomplete migration. A documented
 * allowlist (`POLICY_PACK_COMPLETENESS_ALLOWLIST`) tracks not-yet-converted files exactly like
 * `POLICY_JSON_FIXTURE_ALLOWLIST` does for the dsl/ migration — remove an entry once that file adds the
 * pack-equivalence call (see `vcs/rs/lib.rs`'s own `demo_dsl_pack_equivalence`/
 * `document_text_round_trips_after_apply_and_checkpoint` tests for the pattern).
 */
function policyPackCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_PACK_COMPLETENESS_ALLOWLIST.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const hasDslCheck = content.includes("assert_dsl_round_trip(") || content.includes("assert_document_text_round_trip(");
    if (!hasDslCheck) continue;
    const hasPackCheck = content.includes("assert_dsl_pack_equivalence(") || content.includes("assert_document_pack_round_trip(");
    if (hasPackCheck) continue;
    breaches.push({
      id: `pack-completeness-${relPath}`,
      summary: `"${relPath}" calls assert_dsl_round_trip/assert_document_text_round_trip but never assert_dsl_pack_equivalence/assert_document_pack_round_trip`,
      kind: "pack-migration/completeness",
      scope: relPath,
      priority: "high",
      reason: "Every DSL round-trip test must have a pack-round-trip sibling on the same fixture(s), at the same test level — dsl and pack are two projections of the same (RecordSpec, RecordValue) value model, never two independently-maintained sources of truth.",
      solution: `Add an assert_dsl_pack_equivalence(...)/assert_document_pack_round_trip(...) call beside ${relPath}'s existing DSL round-trip test(s), or if this technology hasn't adopted pack yet, add "${relPath}" to POLICY_PACK_COMPLETENESS_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRulePackCompleteness

//#region 🔧PolicyRuleCommandEnvelopeCompleteness
/**
 * 📏CW7 command-envelope law rule — mirrors `policyPackCompletenessBreaches`'s shrinking-allowlist
 * pattern one step further: every file that already proves the dsl/pack round-trip laws
 * (`assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(`) must ALSO call
 * `vcs::test_support::assert_command_envelope_round_trip` beside them — a technology's `Edit<Operation>`
 * round-tripping through `protocol::OperationEnvelope`s is a third projection of the same value model,
 * not an optional extra. `POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST` tracks not-yet-converted
 * files exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST` does for the pack lock step.
 */
function policyCommandEnvelopeCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const hasPackCheck = content.includes("assert_dsl_pack_equivalence(") || content.includes("assert_document_pack_round_trip(");
    if (!hasPackCheck) continue;
    if (content.includes("assert_command_envelope_round_trip")) continue;
    breaches.push({
      id: `command-envelope-completeness-${relPath}`,
      summary: `"${relPath}" proves the pack round-trip law but never calls assert_command_envelope_round_trip`,
      kind: "protocol-migration/command-envelope-completeness",
      scope: relPath,
      priority: "high",
      reason: "CW7 added vcs::test_support::assert_command_envelope_round_trip as the command-envelope law every technology's Edit/Operation pair must also prove, beside its existing dsl/pack round-trip laws.",
      solution: `Add a vcs::test_support::assert_command_envelope_round_trip::<Projection, Operation>(...) call beside ${relPath}'s existing pack round-trip test(s), or if this technology hasn't wired the command-envelope law yet, add "${relPath}" to POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRuleCommandEnvelopeCompleteness

//#region 🔧PolicyRuleProtocolMigration
/**
 * 🔒Names that must never again be reached through a re-created `vcs::` shim: the temporary CW3
 * `pub use protocol::{...}` re-export deleted in CW8 (`Operation`/`OperationDiff`/`OperationMeta`/
 * `OpText`/`Edit`/`merge_concurrent_diffs`/`ReconcileReport`/`ReconcileSeverity`), the CW3-extracted
 * `semio_framework_core` types (`HybridLogicalTimestamp`/`OperationEnvelope`/`OpDag`/`UndoPolicy`/
 * `MergeStrategyKind`), and the hub wire-frame enums (`HubClientFrame`/`HubServerFrame`).
 */
const POLICY_PROTOCOL_MIGRATION_NAMES = [
  "Operation",
  "OperationDiff",
  "OperationMeta",
  "OpText",
  "Edit",
  "merge_concurrent_diffs",
  "ReconcileReport",
  "ReconcileSeverity",
  "HybridLogicalTimestamp",
  "OperationEnvelope",
  "OpDag",
  "UndoPolicy",
  "MergeStrategyKind",
  "HubClientFrame",
  "HubServerFrame",
] as const;
const POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE = new RegExp(`\\bvcs::(${POLICY_PROTOCOL_MIGRATION_NAMES.join("|")})\\b`, "g");
const POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE = /use\s+(?:::)?vcs::\{([^}]*)\}/gs;

/**
 * 📏CW8 regression-prevention rule: `vcs/rs/lib.rs`'s temporary CW3 `pub use protocol::{Operation,
 * OperationDiff, OpText, OperationMeta, Edit, merge_concurrent_diffs, ReconcileReport,
 * ReconcileSeverity}` shim is deleted — every dependent crate now imports `protocol::` directly (see
 * `policyCommandEnvelopeCompletenessBreaches` above and CW7's import sweep). This rule fires the
 * moment `vcs::` is asked to re-supply any of those names again, or any of the other CW3-extracted
 * `semio_framework_core` types or the hub wire-frame enums — none of which `vcs` has ever re-exported,
 * so today this is pure regression prevention, not an open migration.
 */
function policyProtocolMigrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (relPath === "vcs/rs/lib.rs") continue; // the crate that used to own the shim; never reaches itself via "vcs::"
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    const isCommentLine = (line: number): boolean => /^\s*(\/\/|\*)/.test(content.split(/\r?\n/)[line - 1] ?? "");

    let m: RegExpExecArray | null;
    POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE.lastIndex = 0;
    while ((m = POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE.exec(content))) {
      // Skip "crate::vcs::X" — a crate's own nested module happening to be named "vcs" (e.g.
      // compose's GraphQL "vcs" entity module) is not the external "vcs" crate's root.
      if (content.slice(Math.max(0, m.index - 7), m.index) === "crate::") continue;
      const line = lineOf(m.index);
      if (isCommentLine(line) || seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `protocol-migration-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references vcs::${m[1]} — the CW8 shim is gone, import protocol::${m[1]} directly`,
        kind: "protocol-migration/vcs-shim-regression",
        scope: relPath,
        line,
        priority: "high",
        reason: "CW8 deleted vcs/rs/lib.rs's temporary pub-use shim for protocol-owned Operation/OpText/OperationDiff/OperationMeta/Edit/ReconcileReport/ReconcileSeverity and never re-exported the CW3-extracted framework_core types or hub wire frames — vcs:: must never resolve any of them again.",
        solution: `Import the name from "protocol" directly (e.g. "use protocol::${m[1]};") instead of "vcs::${m[1]}".`,
      });
    }

    POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE.lastIndex = 0;
    while ((m = POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE.exec(content))) {
      const tokens = new Set((m[1] ?? "").split(/[^\w]+/).filter(Boolean));
      const hit = POLICY_PROTOCOL_MIGRATION_NAMES.find((name) => tokens.has(name));
      if (!hit) continue;
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `protocol-migration-use-${relPath}-${line}`,
        summary: `"${relPath}:${line}" imports ${hit} from vcs:: — the CW8 shim is gone, import protocol::${hit} directly`,
        kind: "protocol-migration/vcs-shim-regression",
        scope: relPath,
        line,
        priority: "high",
        reason: "CW8 deleted vcs/rs/lib.rs's temporary pub-use shim; a multi-name \"use vcs::{...}\" block must not re-import a name that only protocol:: still owns.",
        solution: `Split the "use vcs::{...}" block at ${relPath}:${line}: keep vcs's own names there and add a separate "use protocol::{${hit}, ...};" for ${hit}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleProtocolMigration

//#region 🔧PolicyRuleDbServerOnly
/** 🔎Repo-wide `Cargo.toml` file paths (repo-relative), same walker + `POLICY_SKIP_DIRS` as the other policy file discoverers. */
function policyDiscoverCargoTomlFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "Cargo.toml") found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🛡️Directory prefixes always allowed to carry a `db`/`db_*` Cargo dependency: the db family itself and the os hub server. Compose hub crates (`compose/**​/hub/**`) are matched structurally in `policyDbAllowedDir` since they aren't one fixed prefix. */
const POLICY_DB_SERVER_ONLY_ALLOWED_PREFIXES = ["db/", "framework/product/os/hub/"];

/** 🛡️True if `dir` (a repo-relative directory, trailing "/") may depend on `db`/`db_*`: under `db/` itself, under the os hub server, or any compose crate whose path runs through a `hub/` segment. */
function policyDbAllowedDir(dir: string): boolean {
  if (POLICY_DB_SERVER_ONLY_ALLOWED_PREFIXES.some((p) => dir.startsWith(p))) return true;
  const segments = dir.split("/").filter(Boolean);
  return segments[0] === "compose" && segments.includes("hub");
}

const POLICY_DB_DEP_RE = /^(db(?:_[a-z0-9]+)*)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$/gm;

/**
 * 📏db/ server-only rule: no `db`/`db_*` family Cargo dependency may live outside `db/` itself, the
 * `os-hub` server (`framework/product/os/hub/`), or a compose hub crate (`compose/**​/hub/**`) — db is
 * server-side storage for the hubs; clients keep local-first backbones (`vcs` + `store`) and
 * only ever reach db indirectly over the wire. `POLICY_ALWAYS_ALLOWED_DEP_PREFIXES` deliberately does
 * NOT include `"db/"` (unlike `"protocol/"`) so this stays the one gate that enforces it.
 */
function policyDbServerOnlyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverCargoTomlFiles(repoRoot)) {
    const lastSlash = relPath.lastIndexOf("/");
    const dir = lastSlash === -1 ? "" : relPath.slice(0, lastSlash + 1);
    if (policyDbAllowedDir(dir)) continue;
    const text = readFileSync(join(repoRoot, relPath), "utf8");
    let m: RegExpExecArray | null;
    POLICY_DB_DEP_RE.lastIndex = 0;
    while ((m = POLICY_DB_DEP_RE.exec(text))) {
      const depName = m[1]!;
      breaches.push({
        id: `db-server-only-${relPath}-${depName}`,
        summary: `"${relPath}" depends on "${depName}" outside db/'s server-only boundary`,
        kind: "protocol-migration/db-server-only",
        scope: relPath,
        priority: "high",
        reason: "db is server-side storage for the hubs — only db/ itself, framework/product/os/hub/, and compose's hub crates may depend on a db/db_* crate; clients keep local-first backbones (vcs + store) and only ever reach db indirectly over the wire.",
        solution: `Remove the "${depName}" dependency from ${relPath}, or if this crate genuinely is a hub server, move/confirm it under db/, framework/product/os/hub/, or a compose/**/hub/** directory.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleDbServerOnly

//#region 🔧PolicyRuleNoPackFiles
/** 🔎Repo-wide `*.pack` file paths (repo-relative), same walker + `POLICY_SKIP_DIRS` as `policyDiscoverExampleJsonFiles`/`policyDiscoverOpsFiles`. */
function policyDiscoverPackFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".pack")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/**
 * 📏pack/ rollout rule: no `*.pack` binary file may ever be committed. Pack is authoritative-but-
 * regeneratable local storage + export/import (`FolderTextStorage`/`FolderSqliteStorage`'s pack
 * columns/files, both dev-disposable), never a checked-in artifact — examples stay committed DSL text
 * so diffs and golden hashes stay human-legible (golden fixtures are text blake3 hashes, not binary
 * blobs). `target/` and other build-artifact dirs are already excluded via `POLICY_SKIP_DIRS`.
 */
function policyNoPackFilesBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverPackFiles(repoRoot)) {
    breaches.push({
      id: `no-pack-file-${relPath}`,
      summary: `"${relPath}" is a committed *.pack binary file — pack is dev-disposable and must never be committed`,
      kind: "pack-migration/no-pack-file",
      scope: relPath,
      priority: "high",
      reason: "pack's disk role is regeneratable local storage + export/import, not a committed artifact; committing binary .pack blobs would also break the text-diffability the DSL-mirror/golden-hash design relies on.",
      solution: `Delete ${relPath} from the repo (git rm) and rely on its DSL-text mirror / regenerate it via DocumentPack::encode_pack.`,
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRuleNoPackFiles

//#region 🔧PolicyRuleNoRawSpawn
const POLICY_RAW_SPAWN_EXEMPT = new Set(["repo/lib/js/index.ts"]);
const POLICY_RAW_SPAWN_RE = /\b(spawnSync|execSync|execFileSync|Bun\.spawn|spawn)\s*\(/g;

/** 🔎Strips TS/JS comments and string literals so policy regexes only see executable code. */
function policyStripTsCommentsAndStrings(content: string): string {
  const out: string[] = [];
  let i = 0;
  while (i < content.length) {
    const ch = content[i]!;
    const next = content[i + 1];
    if (ch === "/" && next === "/") {
      while (i < content.length && content[i] !== "\n") {
        out.push(" ");
        i++;
      }
      continue;
    }
    if (ch === "/" && next === "*") {
      i += 2;
      while (i < content.length && !(content[i] === "*" && content[i + 1] === "/")) {
        out.push(" ");
        i++;
      }
      if (i < content.length) i += 2;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      const quote = ch;
      out.push(" ");
      i++;
      while (i < content.length) {
        if (content[i] === "\\") {
          out.push(" ");
          out.push(" ");
          i += 2;
          continue;
        }
        if (content[i] === quote) {
          out.push(" ");
          i++;
          break;
        }
        out.push(" ");
        i++;
      }
      continue;
    }
    out.push(ch);
    i++;
  }
  return out.join("");
}

/** 🔎Repo-wide `script.ts` file paths (repo-relative), skipping node_modules/target/.repo and other policy skip dirs. */
function policyDiscoverScriptTsFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "script.ts") found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/**
 * 📏Budgeted-spawn rule: every `script.ts` must route subprocesses through the budgeted runners in
 * `repo/lib/js/index.ts` — raw `spawn`/`spawnSync`/`execSync`/`execFileSync`/`Bun.spawn` bypass
 * wall-clock budgets and are forbidden outside the budget implementation itself.
 */
function policyRawSpawnBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverScriptTsFiles(repoRoot)) {
    if (POLICY_RAW_SPAWN_EXEMPT.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const stripped = policyStripTsCommentsAndStrings(content);
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    let m: RegExpExecArray | null;
    POLICY_RAW_SPAWN_RE.lastIndex = 0;
    while ((m = POLICY_RAW_SPAWN_RE.exec(stripped))) {
      const token = m[1] ?? "spawn";
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-raw-spawn-${relPath}-${line}`,
        summary: `"${relPath}:${line}" uses raw ${token}( — route subprocesses through runCmd/runCmdStatus/runProbe/runTestBudgeted from repo/lib/js/index.ts`,
        kind: "budget/no-raw-spawn",
        scope: relPath,
        line,
        priority: "high",
        reason: "Raw child_process/Bun.spawn calls bypass the repo's wall-clock budget layer; script.ts files must use the budgeted runners so orchestrators, tool calls and tests cannot hang forever.",
        solution: `Replace ${token}( at ${relPath}:${line} with runCmd, runCmdStatus, runProbe, or runTestBudgeted from repo/lib/js/index.ts (or orchestratorBudgetOpts/daemonBudgetOpts for long-running classes).`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleNoRawSpawn

//#region 🔧PolicyRuleNoBudgetNull
/**
 * 📏Budget-null rule: `budgetMs: null` is not a supported escape hatch — use orchestratorBudgetOpts()
 * or daemonBudgetOpts() (or an explicit positive budget) so every subprocess has a named budget class.
 */
function policyBudgetNullBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverScriptTsFiles(repoRoot)) {
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    const re = /budgetMs:\s*null/g;
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) {
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-budget-null-${relPath}-${line}`,
        summary: `"${relPath}:${line}" passes budgetMs: null — use orchestratorBudgetOpts() or daemonBudgetOpts() instead`,
        kind: "budget/no-budget-null",
        scope: relPath,
        line,
        priority: "high",
        reason: "budgetMs: null disables wall-clock budgeting for that subprocess; named orchestrator/daemon budget classes keep long-running work bounded without anonymous unbounded escapes.",
        solution: `At ${relPath}:${line}, replace budgetMs: null with orchestratorBudgetOpts() for nx/script fan-outs or daemonBudgetOpts() for dev servers/MCP stdio, or pass an explicit positive budgetMs.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧PolicyRuleNoBudgetNull

//#region 🔧PolicyRuleMcpConfig
const POLICY_MCP_CONFIG_PATHS = [".cursor/mcp.json", ".mcp.json", ".vscode/mcp.json", ".windsurf/mcp.json", ".kiro/settings/mcp.json", ".codex/config.toml"] as const;

type PolicyMcpServerEntry = { type?: string; command?: string; args?: string[] };

/** 🔎True when a repo MCP server still launches via bun script.ts mcp stdio instead of repo/client/client. */
function policyMcpRepoServerUsesBunScript(entry: PolicyMcpServerEntry): boolean {
  if ((entry.type ?? "stdio") !== "stdio") return false;
  const cmd = (entry.command ?? "").trim();
  const args = entry.args ?? [];
  if (cmd !== "bun") return false;
  const hasScript = args.some((a) => a.includes("script.ts"));
  const hasMcp = args.includes("mcp") || args.some((a) => a.includes("mcp"));
  return hasScript && hasMcp;
}

function policyMcpRepoServerFromJson(doc: unknown): PolicyMcpServerEntry | undefined {
  if (!doc || typeof doc !== "object") return undefined;
  const root = doc as Record<string, unknown>;
  const servers = (root.mcpServers ?? root.servers) as Record<string, PolicyMcpServerEntry> | undefined;
  return servers?.repo;
}

function policyMcpRepoServerFromToml(content: string): PolicyMcpServerEntry | undefined {
  const section = /\[mcp_servers\.repo\]([\s\S]*?)(?:\n\[|$)/.exec(content);
  if (!section) return undefined;
  const block = section[1] ?? "";
  const command = /^\s*command\s*=\s*"([^"]*)"/m.exec(block)?.[1];
  const argsMatch = /^\s*args\s*=\s*\[([^\]]*)\]/m.exec(block);
  const args = argsMatch?.[1]
    ?.split(",")
    .map((s) => s.trim().replace(/^"|"$/g, ""))
    .filter(Boolean);
  const type = /^\s*type\s*=\s*"([^"]*)"/m.exec(block)?.[1];
  return { type, command, args };
}

/**
 * 📏MCP-config rule: the repo MCP server must exec `repo/client/client mcp <kind>` directly — not
 * `bun script.ts … mcp …`, which recompiles through go run and bypasses the budgeted binary path.
 */
function policyMcpConfigBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of POLICY_MCP_CONFIG_PATHS) {
    const abs = join(repoRoot, relPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const entry = relPath.endsWith(".toml") ? policyMcpRepoServerFromToml(content) : policyMcpRepoServerFromJson(JSON.parse(content) as unknown);
    if (!entry || !policyMcpRepoServerUsesBunScript(entry)) continue;
    breaches.push({
      id: `mcp-config-${relPath}`,
      summary: `"${relPath}" repo MCP server still uses bun script.ts mcp — point it at repo/client/client`,
      kind: "budget/mcp-config-repo-binary",
      scope: relPath,
      priority: "high",
      reason: "The repo MCP server must exec the prebuilt repo/client/client binary (args: mcp <kind>) so IDE tool calls skip bun script.ts and go run recompilation.",
      solution: `In ${relPath}, set the repo server to command "repo/client/client" with args ["mcp", "<kind>"] (e.g. cursor, codex, client) and type stdio.`,
    });
  }
  return breaches;
}
//#endregion 🔧PolicyRuleMcpConfig

//#region 🔖PolicyExport
/** ⚖️Runs every Wave 4 rule over every discovered `…/plugin/rs` crate; `framework/plugin/rs` is exempted from the SDK-mechanism rules (it *is* the SDK). */
export const policy = defineLint("@semio-tech/workspace-app-plugin-consistency", (_l: TechnologyLinter): BreachRecord[] => {
  const repoRoot = getWorkspaceRoot();
  const crateDirs = policyDiscoverPluginCrateDirs(repoRoot);
  const breaches: BreachRecord[] = [];

  for (const dir of crateDirs) {
    const abs = join(repoRoot, dir, "lib.rs");
    const content = readFileSync(abs, "utf8");
    const lines = content.split(/\r?\n/);

    breaches.push(...policyRegionFormatBreaches(dir, lines));
    breaches.push(...policyManifestRegionBreaches(dir, lines));
    breaches.push(...policyStructNamingBreaches(dir, content));
    breaches.push(...policyModLayoutBreaches(dir, lines));

    if (dir === "framework/plugin/rs") continue; // the SDK itself, not a consumer of its own primitives
    breaches.push(...policySelectionIdsBreaches(dir, content));
    breaches.push(...policyTestkitDelegateBreaches(dir, content));
    breaches.push(...policyTreeItemBreaches(dir, content, lines));
    breaches.push(...policyLabelsStructBreaches(dir, content));
  }

  breaches.push(...policyCargoArtifactBreaches(repoRoot, crateDirs));
  breaches.push(...policyAppCouplingBreaches(repoRoot, crateDirs));
  breaches.push(...policyJsonFixtureBreaches(repoRoot));
  breaches.push(...policyOpsGrammarBreaches(repoRoot));
  breaches.push(...policyDslCompletenessBreaches(repoRoot));
  breaches.push(...policyPackCompletenessBreaches(repoRoot));
  breaches.push(...policyCommandEnvelopeCompletenessBreaches(repoRoot));
  breaches.push(...policyProtocolMigrationBreaches(repoRoot));
  breaches.push(...policyDbServerOnlyBreaches(repoRoot));
  breaches.push(...policyNoPackFilesBreaches(repoRoot));
  breaches.push(...policyRawSpawnBreaches(repoRoot));
  breaches.push(...policyBudgetNullBreaches(repoRoot));
  breaches.push(...policyMcpConfigBreaches(repoRoot));
  return breaches;
});
//#endregion 🔖PolicyExport
//#endregion 🔖Policy

if (import.meta.main) {
  if (!(await dispatchPolicyArgv(process.argv.slice(2), import.meta.url))) {
    await runWorkspaceScriptMain(router);
  }
}
