#!/usr/bin/env bun
/**
 * 🧭 Monorepo command router: `bun ./script.ts <verb> [segments…]` (e.g. `script.ts dev`, `script.ts dev mcp`, `script.ts generate neo4j semio`).
 */
import { execFileSync, spawn, spawnSync } from "node:child_process";
import { existsSync, linkSync, mkdirSync, chmodSync, chownSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { extname, join, resolve } from "node:path";
import { createServer } from "node:net";
import { stat } from "node:fs/promises";
import { Neo4jCypherExport, getAllNeo4jGraphExportSpecs, joinNeo4jGraphDatabaseName, partitionNeo4jGraphCliArgv } from "./generate.neo4j.gen.ts";

const WORKSPACE_ROOT = import.meta.dir;
const BUN = process.execPath;

//#region 🔖ScriptFramework
/** 🧭Abstract workspace command; `run` receives argv segments after the verb (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script {
  constructor(protected readonly root: string) {}
  abstract run(segments: string[]): void | Promise<void>;
}

function runCmd(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
  execFileSync(cmd, args, {
    stdio: "inherit",
    cwd: opts.cwd ?? WORKSPACE_ROOT,
    env: opts.env ?? process.env,
  });
}

function devToolingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  const env = { ...process.env, ...extra };
  delete env.NODE_OPTIONS;
  delete env.VSCODE_INSPECTOR_OPTIONS;
  env.NX_NATIVE_COMMAND_RUNNER ??= "false";
  env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT ??= "false";
  env.NX_TUI ??= "false";
  return env;
}

function tryRun(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
  try {
    runCmd(cmd, args, opts);
  } catch {
    /* optional */
  }
}
//#endregion 🔖ScriptFramework

//#region 🔖SetupScript
export class SetupScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "postinstall") {
      this.runPostinstall();
      return;
    }
    if (segments[0] === "git") {
      this.runGit();
      return;
    }
    this.runFull();
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
      runCmd("go", ["run", "./repo/client/mcp", "configure"], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
      });
    }
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
      if (process.platform === "win32") {
        const ps = join(this.root, "script.ps1");
        if (existsSync(ps)) {
          console.log("[setup] Windows native bootstrap…");
          tryRun("powershell", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", ps, "setup"]);
        }
      } else if (process.platform === "darwin" || process.platform === "linux") {
        const sh = join(this.root, "script.sh");
        if (existsSync(sh)) {
          console.log(`[setup] ${process.platform} native bootstrap…`);
          tryRun("bash", [sh, "setup"]);
        }
      }
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
    tryRun("go", ["build", "-o", clientOut, "./repo/client/mcp"], { env: { ...process.env, GOWORK: join(this.root, "go.work") } });
    console.log("[setup] dotnet restore…");
    tryRun("dotnet", ["restore", "Monorepo.sln"]);
    console.log("[setup] rustup wasm target…");
    tryRun("rustup", ["target", "add", "wasm32-unknown-unknown"]);

    const cargoHome = join(homedir(), ".cargo");
    const cargoConfig = join(cargoHome, "config.toml");
    if (!existsSync(cargoConfig)) {
      mkdirSync(cargoHome, { recursive: true });
      writeFileSync(cargoConfig, `[target.wasm32-unknown-unknown]\nrustflags = ["--cfg", "getrandom_backend=wasm_js"]\n`);
      console.log("[setup] wrote ~/.cargo/config.toml wasm flags.");
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

    console.log("[setup] git workspace (symlinks, hooks)…");
    new SetupScript(this.root).run(["git"]);

    console.log("[setup] VS Code extension build & package…");
    tryRun("bun", ["nx", "run", "repo:build"], { cwd: this.root });
    tryRun("bun", ["nx", "run", "repo:build-vsix"], { cwd: this.root });
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

    if (process.platform === "win32") {
      runCmd("powershell.exe", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", join(this.root, "script.ps1"), "start"], {
        cwd: this.root,
      });
    } else if (process.platform === "darwin" || process.platform === "linux") {
      runCmd("bash", [join(this.root, "script.sh"), "start"], { cwd: this.root });
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
    if (segments[0] === "board") {
      runCmd("bun", ["nx", "run", "@elements/board:dev", ...segments.slice(1)], { cwd: this.root, env: devToolingEnv() });
      return;
    }
    if (segments[0] === "scene") {
      runCmd("bun", ["nx", "run", "@elements/scene:dev", ...segments.slice(1)], { cwd: this.root, env: devToolingEnv() });
      return;
    }
    if (segments[0] === "spatial") {
      runCmd("bun", ["nx", "run", "@spatial/js-renderer-r3f:dev", ...segments.slice(1)], { cwd: this.root, env: devToolingEnv() });
      return;
    }
    if (segments[0] === "mcp") {
      this.runMcp(segments.slice(1));
      return;
    }
    runCmd("bun", ["nx", "run", "@semio/desktop:dev"], { cwd: this.root });
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
      runCmd("bun", [join(this.root, "semio", "client", "bin", "engine", "script.ts"), "dev", "mcp"], { cwd: this.root });
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
    const graphDatabase = hasName ? joinNeo4jGraphDatabaseName(nameParts) : process.env.NEO4J_DATABASE || "semio";
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
      client: "./repo/client/mcp",
      codex: "./repo/client/mcp/codex",
      copilot: "./repo/client/mcp/copilot",
      cursor: "./repo/client/mcp/cursor",
      kiro: "./repo/client/mcp/kiro",
      claude: "./repo/client/mcp/claude",
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
    runCmd("bunx", ["dependency-cruiser@16", "semio/client/lib/js", "semio/client/lib/react", "semio/client/lib/sketchpad", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
  }
}
//#endregion 🔖LintScript

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
    if (segments[0] === "storybook") {
      await this.runStorybookPlaywright();
      return;
    }
    runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/js", "@semio/react"], { cwd: this.root });
    runCmd("bun", ["nx", "run", "semio/graphql:build"], { cwd: this.root });
    runCmd("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], { cwd: this.root });
    runCmd("bun", ["nx", "run", "workspace:test-storybook"], { cwd: this.root });
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
      runCmd("bunx", ["playwright", "test", ".storybook/board.spec.ts", "--config", ".storybook/playwright.config.ts"], {
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
      "3dm": "@semio/3dm-ui:build",
      assets: "@semio/assets:build",
      desktop: "@semio/desktop:build",
      engine: "@semio/engine:build",
      storybook: "workspace:build-storybook",
      "coda-desktop": "@coda/desktop:build",
      "repo-cli": "@repo/client:build",
      "repo-server": "@repo/coordinator:build",
      "repo-vscode": "repo:build-vsix",
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
      runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/play", "@semio/docs"], { cwd: this.root });
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
  const result = spawnSync(
    vswhere,
    [
      "-latest",
      "-version",
      "[18.0,19.0)",
      "-products",
      "*",
      "-requires",
      "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
      "-property",
      "installationPath",
    ],
    { encoding: "utf8" },
  );
  if (result.status !== 0) return undefined;
  const installPath = result.stdout.trim();
  return installPath || undefined;
}
//#endregion 🔖CppScriptHelpers

//#region 🔖CppScript
export class CppScript extends Script {
  run(segments: string[]): void {
    const command = segments[0] ?? "all";
    const preset = this.resolvePreset(segments.slice(1));
    if (command === "setup") {
      this.runSetup();
      return;
    }
    if (command === "configure") {
      this.runConfigure(preset);
      return;
    }
    if (command === "build") {
      this.runBuild(preset);
      return;
    }
    if (command === "test") {
      this.runTest(preset);
      return;
    }
    if (command === "all") {
      this.runSetup();
      this.runConfigure(preset);
      this.runBuild(preset);
      this.runTest(preset);
      return;
    }
    console.error("[cpp] usage: bun ./script.ts cpp [setup|configure|build|test|all] [preset]");
    process.exit(1);
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
    console.error("[cpp] On native Windows run: .\\script.ps1 setup");
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
      desktop: "@semio/desktop:publish",
      play: "@semio/play:publish",
      sketchpad: "@semio/sketchpad:publish",
      docs: "@semio/docs:publish",
      "coda-desktop": "@coda/desktop:publish",
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

//#region 🔖PurgeScript
export class PurgeScript extends Script {
  run(segments: string[]): void {
    if (segments[0] !== "neo4j") {
      console.error("[purge] usage: bun ./script.ts purge neo4j");
      process.exit(1);
    }
    const database = process.env.NEO4J_DATABASE || "semio";
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

//#region 🔖Dispatch
const registry = new Map<string, Script>([
  ["nx", new NxScript(WORKSPACE_ROOT)],
  ["setup", new SetupScript(WORKSPACE_ROOT)],
  ["start", new StartScript(WORKSPACE_ROOT)],
  ["dev", new DevScript(WORKSPACE_ROOT)],
  ["generate", new GenerateScript(WORKSPACE_ROOT)],
  ["lint", new LintScript(WORKSPACE_ROOT)],
  ["format", new FormatScript(WORKSPACE_ROOT)],
  ["test", new TestScript(WORKSPACE_ROOT)],
  ["build", new BuildScript(WORKSPACE_ROOT)],
  ["cpp", new CppScript(WORKSPACE_ROOT)],
  ["publish", new PublishScript(WORKSPACE_ROOT)],
  ["purge", new PurgeScript(WORKSPACE_ROOT)],
]);

async function main(): Promise<void> {
  const segments = process.argv.slice(2);
  if (segments.length === 0) {
    console.error("usage: bun ./script.ts <nx|setup|start|dev|generate|lint|format|test|build|cpp|publish|purge> [segments…]");
    process.exit(1);
  }
  const verb = segments[0];
  const script = registry.get(verb);
  if (!script) {
    console.error(`unknown verb ${JSON.stringify(verb)}`);
    process.exit(1);
  }
  await Promise.resolve(script.run(segments.slice(1)));
}

await main();
//#endregion 🔖Dispatch
