#!/usr/bin/env bun
/**
 * 🧰 Zero-touch workspace bootstrap: optional native OS bootstrap (`setup.*.sh` / `setup.windows.script.ps1`),
 * uv, neo4j MCP prefetch (uvx), cargo, go client, dotnet, rust wasm, Playwright, Electron sandbox, `setup.git`, VSIX.
 */
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, chmodSync, chownSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

const root = import.meta.dir;

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

//#region 🔖NativeOsBootstrap
if (process.argv.includes("--with-native-os")) {
  if (process.platform === "win32") {
    const ps = join(root, "setup.windows.script.ps1");
    if (existsSync(ps)) {
      console.log("[setup] Windows native bootstrap (setup.windows.script.ps1)…");
      tryRun("powershell", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", ps]);
    }
  } else if (process.platform === "darwin") {
    const sh = join(root, "setup.mac.sh");
    if (existsSync(sh)) {
      console.log("[setup] macOS native bootstrap (setup.mac.sh)…");
      tryRun("bash", [sh]);
    }
  } else {
    const sh = join(root, "setup.linux.sh");
    if (existsSync(sh)) {
      console.log("[setup] Linux native bootstrap (setup.linux.sh)…");
      tryRun("bash", [sh]);
    }
  }
}
//#endregion

console.log("[setup] uv sync…");
tryRun("uv", ["sync", "--all-packages", "--all-groups"]);

//#region 🔖Neo4jMcpPrefetch
console.log("[setup] neo4j MCP server prefetch (uvx)…");
tryRun("uvx", ["--quiet", "mcp-neo4j-cypher", "--help"]);
//#endregion

console.log("[setup] cargo fetch…");
tryRun("cargo", ["fetch", "--manifest-path", "Cargo.toml"]);

console.log("[setup] go build repo client…");
const clientOut = join(root, "repo", "client", process.platform === "win32" ? "client.exe" : "client");
tryRun("go", ["build", "-o", clientOut, "./repo/client/mcp"], { env: { ...process.env, GOWORK: join(root, "go.work") } });

console.log("[setup] dotnet restore…");
tryRun("dotnet", ["restore", "Monorepo.sln"]);

console.log("[setup] rustup wasm target…");
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
  console.log("[setup] wrote ~/.cargo/config.toml wasm flags.");
}
//#endregion

//#region 🔖Playwright
const browsersPath = join(root, "node_modules", ".cache", "ms-playwright");
mkdirSync(browsersPath, { recursive: true });
console.log("[setup] Playwright browsers…");
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
      console.log("[setup] Electron chrome-sandbox permissions set.");
    } catch (e) {
      console.warn("[setup] chrome-sandbox chmod skipped:", e);
    }
  }
}
//#endregion

//#region 🔖GitWorkspace
console.log("[setup] git workspace (symlinks, hooks)…");
tryRun("bun", [join(root, "setup.git.script.ts")]);
//#endregion

console.log("[setup] VS Code extension build & package…");
tryRun("bun", ["nx", "run", "repo:build"]);
tryRun("bun", ["nx", "run", "repo:build-vsix"]);

console.log("[setup] done.");
