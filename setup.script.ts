#!/usr/bin/env bun
/**
 * 🧰 Zero-touch workspace bootstrap: uv, neo4j MCP prefetch (uvx), cargo, go client, dotnet,
 * rust wasm target, cargo wasm flags, Playwright browsers, Linux Electron sandbox, git hooks, VSIX build.
 * Pass `--postinstall` for the lightweight lightningcss native-binary fix only (npm `postinstall` hook).
 */
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, chmodSync, chownSync, readFileSync, writeFileSync } from "node:fs";
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

//#region 🔖PostinstallLightningcss
if (process.argv.includes("--postinstall")) {
  const pkgPath = join(root, "node_modules", "lightningcss", "package.json");
  if (!existsSync(pkgPath)) process.exit(0);
  const { version } = JSON.parse(readFileSync(pkgPath, "utf8")) as { version: string };
  const report = process.report?.getReport?.() as
    | { header?: { glibcVersionRuntime?: string } }
    | undefined;
  const libc =
    process.platform === "linux"
      ? report?.header?.glibcVersionRuntime
        ? "gnu"
        : "musl"
      : "";
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
  if (!platformPkg) process.exit(0);
  if (existsSync(join(root, "node_modules", platformPkg))) process.exit(0);
  const spec = `${platformPkg}@${version}`;
  execFileSync("bun", ["add", "--no-save", spec], { cwd: root, stdio: "inherit" });
  process.exit(0);
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
tryRun("bun", [join(root, "git.script.ts"), "setup"]);
//#endregion

console.log("[setup] VS Code extension build & package…");
tryRun("bun", ["nx", "run", "repo:build"]);
tryRun("bun", ["nx", "run", "repo:build-vsix"]);

console.log("[setup] done.");
