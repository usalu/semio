#!/usr/bin/env bun
/**
 * 🧭 Monorepo command router: `bun ./script.ts <verb> [segments…]` (e.g. `script.ts dev`, `script.ts dev mcp`, `script.ts generate neo4j compose`).
 */
import { spawn, spawnSync } from "node:child_process";
import {
  Script,
  ScriptRouter,
  devToolingEnv,
  dispatchSubcommand,
  frameworkOsPlaygroundDevEnv,
  loadFrameworkOsPlaygroundCatalog,
  resolveFrameworkOsPlaygroundPlugin,
  runCmd,
  runTestBudgeted,
  installMicroCommitGitHooks,
  runCommit,
  runMicroCommit,
  runWorkspaceScriptMain,
  tryRun,
} from "./repo/lib/js/index.ts";
import { existsSync, linkSync, mkdirSync, chmodSync, chownSync, copyFileSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { extname, join, resolve } from "node:path";
import { createServer } from "node:net";
import { stat } from "node:fs/promises";

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
  const probe = spawnSync("sccache", ["--version"], { encoding: "utf8" });
  if (probe.status === 0) return;

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
    const repoClientCandidates = [join(this.root, "repo", "client", "client.exe"), join(this.root, "repo", "client", "client")];
    const repoClientPath = repoClientCandidates.find((p) => existsSync(p));
    if (repoClientPath) {
      runCmd(repoClientPath, ["configure"], { cwd: this.root });
    } else {
      runCmd("go", ["run", "./repo/client/mcp/go", "configure"], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
      });
    }
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
    console.log("[setup] C++ toolchain and vcpkg…");
    tryRun("bun", [join(this.root, "script.ts"), "cpp", "setup"], { cwd: this.root });
    console.log("[setup] go build repo client…");
    const clientOut = join(this.root, "repo", "client", process.platform === "win32" ? "client.exe" : "client");
    tryRun("go", ["build", "-o", clientOut, "./repo/client/mcp/go"], { env: { ...process.env, GOWORK: join(this.root, "go.work") } });
    console.log("[setup] dotnet restore…");
    tryRun("dotnet", ["restore", "Monorepo.sln"]);
    console.log("[setup] rustup wasm target…");
    tryRun("rustup", ["target", "add", "wasm32-unknown-unknown"]);
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
      const r = spawnSync(BUN, [join(this.root, "script.ts"), "generate"], { stdio: "inherit", cwd: this.root });
      if (r.status !== 0) {
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
    runCmd("bun", ["nx", "run", "@semio-tech/compose-desktop:dev"], { cwd: this.root });
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
    const scope = scopeSegments.join("/");
    if (scope && !/^[a-z0-9][a-z0-9/-]*$/i.test(scope)) {
      console.error(`[dev.storybook] invalid scope ${JSON.stringify(scope)}`);
      process.exit(1);
    }
    if (scope && !existsSync(join(this.root, ".storybook", "stories", ...scopeSegments))) {
      console.error(`[dev.storybook] unknown scope ${JSON.stringify(scope)}`);
      process.exit(1);
    }
    return { scope, args };
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
      runCmd("bun", [join(this.root, "compose", "client", "bin", "engine", "script.ts"), "dev", "mcp"], { cwd: this.root });
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
    const child =
      mode === "repo"
        ? spawn("npx", ["--yes", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"], { stdio: "inherit", shell: true, cwd: this.root, env: { ...process.env, HOST: host } })
        : spawn("npx", ["--yes", "@modelcontextprotocol/inspector"], {
            stdio: "inherit",
            shell: true,
            cwd: this.root,
          });
    child.on("exit", (c) => process.exit(c ?? 0));
  }

  private runMcpNeo4j(neoSegments: string[]): void {
    const { nameParts, passthrough } = partitionNeo4jGraphCliArgv(neoSegments);
    const hasName = nameParts.length > 0;
    const graphDatabase = hasName ? joinNeo4jGraphDatabaseName(nameParts) : process.env.NEO4J_DATABASE || "compose";
    const args = [...passthrough];
    if (hasName && !args.includes("--namespace")) args.push("--namespace", graphDatabase);
    const r = spawnSync("uvx", ["mcp-neo4j-cypher", ...args], {
      stdio: "inherit",
      env: {
        ...process.env,
        NEO4J_URI: process.env.NEO4J_URI || "bolt://localhost:7687",
        NEO4J_USERNAME: process.env.NEO4J_USERNAME || "neo4j",
        NEO4J_PASSWORD: process.env.NEO4J_PASSWORD || "password",
        NEO4J_DATABASE: graphDatabase,
        NEO4J_TELEMETRY: process.env.NEO4J_TELEMETRY || "false",
      },
    });
    process.exit(r.status ?? 1);
  }

  private runMcpStdioRepo(slugs: string[]): void {
    const slug = (slugs[0] ?? "client").trim().toLowerCase();
    const extra = slugs.slice(1);
    const packages: Record<string, string> = {
      client: "./repo/client/mcp/go",
      codex: "./repo/client/mcp/codex/go",
      copilot: "./repo/client/mcp/copilot/go",
      cursor: "./repo/client/mcp/cursor/go",
      kiro: "./repo/client/mcp/kiro/go",
      claude: "./repo/client/mcp/claude/go",
    };
    const pkg = packages[slug];
    if (!pkg) {
      console.error(`[dev.mcp.stdio] unknown profile ${JSON.stringify(slug)}`);
      process.exit(1);
    }
    const r = spawnSync("go", ["run", pkg, ...extra], {
      cwd: this.root,
      stdio: "inherit",
      env: { ...process.env, GOWORK: join(this.root, "go.work") },
    });
    process.exit(r.status ?? 1);
  }
}
//#endregion 🔖DevScript

//#region 🔖NxScript
export class NxScript extends Script {
  run(segments: string[]): void {
    runCmd("node", [join(this.root, "node_modules", "nx", "bin", "nx.js"), ...segments], {
      cwd: this.root,
      env: devToolingEnv(),
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
    if (segments[0] === "repo") {
      runCmd("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], { cwd: this.root });
      return;
    }
    runCmd("bun", ["nx", "run-many", "-t", "lint", "--all", "--exclude", "workspace"], { cwd: this.root });
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
    runCmd("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], { cwd: this.root });
  }

  private async runGate(): Promise<void> {
    // Deliberately calls dependency-cruiser directly rather than `LintScript`/`nx run-many -t lint --all`:
    // several unrelated projects (repo/client/vscode, compose-js, …) have pre-existing broken eslint configs,
    // and framework-renderer-wgpu:lint has known pending color-literal violations (see spawn_task follow-ups) —
    // this gate must stay a meaningful, currently-green signal for refactor sessions, not inherit that noise.
    console.log("[verify] dependency-cruiser boundaries…");
    runCmd("bunx", ["dependency-cruiser@16", "compose", "framework", "flow", "layout", "puzzle", "ui", "draw", "note", "sequence", "s", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
    console.log("[verify] generated catalog freshness…");
    runCmd("bun", ["nx", "run", "@semio-tech/plugin-registry:check"], { cwd: this.root });
    console.log("[verify] region/host-contract script lints…");
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-react:lint"], { cwd: this.root });
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:plugin", "lint"], { cwd: this.root });
    runCmd("bun", ["nx", "run", "@semio-tech/ui-styling-tokens:check-no-px"], { cwd: this.root });
    console.log("[verify] gate passed.");
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
export class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "e2e") {
      await this.runE2e();
      return;
    }
    if (segments[0] === "storybook") {
      await this.runStorybookPlaywright();
      return;
    }
    if (segments[0] === "repo-client") {
      await this.runRepoGoTest("./repo/client/cli/go", segments.slice(1));
      return;
    }
    if (segments[0] === "repo-mcp") {
      runCmd("go", ["build", "-o", join(this.root, process.platform === "win32" ? "repo/client/client.exe" : "repo/client/client"), "./repo/client/mcp/go"], { cwd: this.root, env: { ...process.env, GOWORK: join(this.root, "go.work") } });
      for (const pkg of ["./repo/client/mcp/go", "./repo/client/mcp/cursor/go", "./repo/client/mcp/copilot/go", "./repo/client/mcp/claude/go", "./repo/client/mcp/codex/go", "./repo/client/mcp/kiro/go"]) {
        runCmd("go", ["build", pkg], { cwd: this.root, env: { ...process.env, GOWORK: join(this.root, "go.work") } });
      }
      await this.runRepoGoTest("./repo/client/cli/go", ["-run", "Mcp|MCP|mcp", ...segments.slice(1)]);
      return;
    }
    runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio-tech/compose-js", "@semio-tech/compose-react"], { cwd: this.root });
    runCmd("bun", ["nx", "run", "compose/graphql:build"], { cwd: this.root });
    runCmd("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], { cwd: this.root });
  }

  /** 🎭Runs every opt-in `test-e2e` target (Postgres containers, VSCode extension host, sketchpad Playwright, …) plus the Storybook board e2e — excluded from the default ≤30s `test` budget. */
  private async runE2e(): Promise<void> {
    runCmd("bun", ["nx", "run-many", "-t", "test-e2e", "--all", "--exclude", "workspace"], { cwd: this.root });
    await this.runStorybookPlaywright();
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

  /** ⏱️`-short` skips the `testing.Short()`-gated real-monorepo-scan tests in `repo/client/cli/go/main_test.go` so the default budgeted `test` target stays ≤30s. */
  private async runRepoGoTest(module: string, extraArgs: string[]): Promise<void> {
    await runTestBudgeted("go", ["test", module, "-short", ...extraArgs], {
      cwd: this.root,
      env: { ...process.env, GOWORK: join(this.root, "go.work") },
    });
  }

  private async runStorybookPlaywright(): Promise<void> {
    const preferred = Number(process.env.STORYBOOK_PORT ?? 6010);
    const storybookPort = String(await this.pickStorybookStaticPort(preferred, 50));
    const baseUrl = `http://127.0.0.1:${storybookPort}/`;
    runCmd("bun", [join(this.root, "script.ts"), "build", "storybook"], { cwd: this.root });
    const server = spawn("bun", [join(this.root, "script.ts"), "dev", "storybook-static"], {
      cwd: this.root,
      stdio: "inherit",
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

    if (!slice) {
      runCmd("bun", ["nx", "run-many", "-t", "build", "--all", "--exclude", "workspace"], { cwd: this.root });
      runCmd("bun", ["nx", "run", "workspace:build-storybook"], { cwd: this.root });
      return;
    }
    if (slice === "storybook") {
      runCmd("bunx", ["storybook", "build", "-c", ".storybook", "--output-dir", "storybook-static"], { cwd: this.root });
      return;
    }
    if (slice === "sites") {
      runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio-tech/compose-sketchpad-play", "@semio-tech/compose-sketchpad-docs"], { cwd: this.root });
      return;
    }
    const target = single[slice];
    if (!target) {
      console.error(`[build] unknown slice ${JSON.stringify(slice)}`);
      process.exit(1);
    }
    runCmd("bun", ["nx", "run", target], { cwd: this.root });
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
  const result = spawnSync(vswhere, ["-latest", "-version", "[18.0,19.0)", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"], { encoding: "utf8" });
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
    runCmd(this.resolveTool("cmake"), ["--preset", preset], { cwd: this.root, env: this.cppEnv() });
  }

  private runBuild(preset: string): void {
    runCmd(this.resolveTool("cmake"), ["--build", "--preset", preset], { cwd: this.root, env: this.cppEnv() });
  }

  private runTest(preset: string): void {
    runCmd(this.resolveTool("ctest"), ["--preset", preset], { cwd: this.root, env: this.cppEnv() });
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
    const result = spawnSync(this.resolveTool(command), ["--version"], {
      stdio: "ignore",
      shell: process.platform === "win32" && !this.resolveTool(command).includes("\\"),
      env: this.cppEnv(),
    });
    return result.status === 0;
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
      runCmd("git", ["clone", "--depth", "1", "https://github.com/microsoft/vcpkg.git", vcpkgRoot], { cwd: this.root });
    }
    if (!existsSync(vcpkgExe)) {
      if (process.platform === "win32") {
        runCmd("cmd.exe", ["/c", join(vcpkgRoot, "bootstrap-vcpkg.bat"), "-disableMetrics"], { cwd: vcpkgRoot });
      } else {
        runCmd("bash", [join(vcpkgRoot, "bootstrap-vcpkg.sh"), "-disableMetrics"], { cwd: vcpkgRoot });
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
    runCmd("bun", ["nx", "run", target], { cwd: this.root });
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
    const r = spawnSync("cypher-shell", ["-a", uri, "-u", user, "-p", password, "-d", database, "--format", "plain", "RETURN 1 AS ok;"], {
      stdio: "inherit",
    });
    if (r.status !== 0) {
      console.warn("[purge.neo4j] cypher-shell unavailable — skip.");
      process.exit(0);
    }
    console.log("[purge.neo4j] connectivity ok; noop.");
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

//#region 🔖Dispatch
const router = new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT)
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
import { spawnSync } from "node:child_process";
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
      const probe = spawnSync(candidate, ["--version"], { stdio: "ignore" });
      if (probe.status === 0) return candidate;
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
      const result = spawnSync(shell, ["-a", process.env.NEO4J_URI || "bolt://localhost:7687", "-u", process.env.NEO4J_USERNAME || "neo4j", "-p", process.env.NEO4J_PASSWORD || "password", "-d", database, "--format", "plain", "-f", queryPath], {
        cwd: this.repoRoot,
        encoding: "utf8",
        env: this.buildCypherEnv(),
      });

      return {
        ok: result.status === 0,
        stdout: typeof result.stdout === "string" ? result.stdout : (result.stdout?.toString() ?? ""),
        stderr: typeof result.stderr === "string" ? result.stderr : (result.stderr?.toString() ?? ""),
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

await runWorkspaceScriptMain(router);
