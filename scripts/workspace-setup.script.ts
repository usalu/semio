#!/usr/bin/env bun
/**
 * Zero-touch workspace bootstrap: uv, neo4j MCP prefetch (uvx), cargo, go client, dotnet,
 * rust wasm target, cargo wasm flags, Playwright browsers, Linux Electron sandbox, git hooks, VSIX build.
 */
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, chmodSync, chownSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

const root = join(import.meta.dir, "..");

function run(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}) {
  execFileSync(cmd, args, {
    stdio: "inherit",
    cwd: opts.cwd ?? root,
    env: opts.env ?? process.env,
  });
}

function tryRun(cmd: string, args: string[], opts: { cwd?: string } = {}) {
  try {
    run(cmd, args, opts);
  } catch {
    /* optional steps */
  }
}

console.log("[workspace-setup] uv sync…");
tryRun("uv", ["sync", "--all-packages", "--all-groups"]);

//#region 🔖Neo4jMcpPrefetch
console.log("[workspace-setup] neo4j MCP server prefetch (uvx)…");
tryRun("uvx", ["--quiet", "mcp-neo4j-cypher", "--help"]);
//#endregion

console.log("[workspace-setup] cargo fetch…");
tryRun("cargo", ["fetch", "--manifest-path", "Cargo.toml"]);

console.log("[workspace-setup] go build repo client…");
const clientOut = join(root, "repo", "client", process.platform === "win32" ? "client.exe" : "client");
tryRun("go", ["build", "-o", clientOut, "./repo/client/mcp"], { env: { ...process.env, GOWORK: join(root, "go.work") } });

console.log("[workspace-setup] dotnet restore…");
tryRun("dotnet", ["restore", "Monorepo.sln"]);

console.log("[workspace-setup] rustup wasm target…");
tryRun("rustup", ["target", "add", "wasm32-unknown-unknown"]);

//#region 🔖RustWasmCargoConfig
const cargoHome = join(homedir(), ".cargo");
const cargoConfig = join(cargoHome, "config.toml");
if (!existsSync(cargoConfig)) {
  mkdirSync(cargoHome, { recursive: true });
  writeFileSync(
    cargoConfig,
    `[target.wasm32-unknown-unknown]\nrustflags = ["--cfg", "getrandom_backend=wasm_js"]\n`,
  );
  console.log("[workspace-setup] wrote ~/.cargo/config.toml wasm flags.");
}
//#endregion

//#region 🔖Playwright
const browsersPath = join(root, "node_modules", ".cache", "ms-playwright");
mkdirSync(browsersPath, { recursive: true });
console.log("[workspace-setup] Playwright browsers…");
tryRun("bunx", ["playwright", "install", "--with-deps", "chromium"], {
  env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsersPath },
});
//#endregion

//#region 🔖ElectronSandboxLinux
if (process.platform === "linux") {
  const chromeSandbox = join(root, "node_modules", "electron", "dist", "chrome-sandbox");
  if (existsSync(chromeSandbox)) {
    try {
      chownSync(chromeSandbox, 0, 0);
      chmodSync(chromeSandbox, 0o4755);
      console.log("[workspace-setup] Electron chrome-sandbox permissions set.");
    } catch (e) {
      console.warn("[workspace-setup] chrome-sandbox chmod skipped:", e);
    }
  }
}
//#endregion

//#region 🔖RepoHooks
const configureBin = join(root, "repo", "client", process.platform === "win32" ? "client.exe" : "client");
if (existsSync(configureBin)) {
  console.log("[workspace-setup] repo client configure…");
  tryRun(configureBin, ["configure"]);
}
//#endregion

console.log("[workspace-setup] VS Code extension build & package…");
tryRun("bun", ["nx", "run", "repo:build"]);
tryRun("bun", ["nx", "run", "repo:build-vsix"]);

console.log("[workspace-setup] done.");
