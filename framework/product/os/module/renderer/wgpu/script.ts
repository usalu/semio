#!/usr/bin/env bun
/** @emoji 🧊 `@semio-tech/framework-renderer-wgpu` task router. */
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  SEMIO_ASSET_SERVER_PORT,
  SEMIO_ASSET_BASE_URL_ENV,
  buildBudgetMs,
  daemonBudgetOpts,
  getWorkspaceRoot,
  orchestratorBudgetOpts,
  resolveTestLevel,
  runBundleScriptMain,
  runCargoTestBudgeted,
  runCmd,
  runCmdStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  frameworkOsPlaygroundDefaultPort,
  loadFrameworkOsPlaygroundCatalog,
} from "../../../../../../repo/lib/js/index.ts";
import { startAssetServer } from "../../../../../module/ui/styling/vite-elements-assets.ts";
import type { PlaygroundAssetSpec } from "../../plugin/registry/generated/playgrounds.ts";
import { writePlaygroundSession } from "../../plugin/registry/script.ts";

const repoRoot = getWorkspaceRoot();
const wasmTarget = "wasm32-unknown-unknown";
const crateName = "semio-framework-renderer-wgpu";
const outDir = join(repoRoot, "framework/product/os/module/dev/js/renderer-modules/wgpu");
const pluginOutRoot = join(repoRoot, "framework/product/os/module/dev/js/plugin-modules");

//#region 🌐 DevServer
function trunkEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  delete env.NO_COLOR;
  delete env.FORCE_COLOR;
  return env;
}

/** 🌐 Runs a long-lived child without blocking Bun's asset-server event loop. */
async function runInteractiveCommand(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv): Promise<number> {
  const daemon = spawnDaemon(command, args, { cwd, env });
  const terminate = () => daemon.kill();
  process.once("SIGINT", terminate);
  process.once("SIGTERM", terminate);
  try {
    return await new Promise<number>((resolve, reject) => {
      daemon.child.once("error", reject);
      daemon.child.once("exit", (code) => resolve(code ?? 1));
    });
  } finally {
    process.off("SIGINT", terminate);
    process.off("SIGTERM", terminate);
    daemon.kill();
  }
}

function ensureWasmTarget(): void {
  const probe = runProbe("rustup", ["target", "list", "--installed"]);
  if (!probe.stdout.includes(wasmTarget)) {
    runCmd("rustup", ["target", "add", wasmTarget]);
  }
}

function ensureTrunk(): void {
  const probe = runProbe("trunk", ["--version"]);
  if (probe.status !== 0) {
    runCmd("cargo", ["install", "trunk", "--locked"], { budgetMs: buildBudgetMs() });
  }
}

function syncStableRendererArtifacts(): void {
  const js = readdirSync(outDir).find((name) => name.startsWith("semio-framework-renderer-wgpu-") && name.endsWith(".js"));
  const wasm = readdirSync(outDir).find((name) => name.startsWith("semio-framework-renderer-wgpu-") && name.endsWith("_bg.wasm"));
  if (!js) throw new Error("missing trunk wgpu renderer js artifact");
  copyFileSync(join(outDir, js), join(outDir, "semio_framework_renderer_wgpu.js"));
  if (wasm) copyFileSync(join(outDir, wasm), join(outDir, "semio-framework-renderer-wgpu_bg.wasm"));
}

function assetServerBaseUrl(): string {
  return `http://127.0.0.1:${SEMIO_ASSET_SERVER_PORT}`;
}

/** @emoji 🗂️ The active playground variant's declared asset specs (tile-proxy, mesh-collection, static-dir). */
function variantAssetSpecs(variant: string): readonly PlaygroundAssetSpec[] {
  const row = loadFrameworkOsPlaygroundCatalog().find((entry) => entry.variant === variant);
  return row?.assets ?? [];
}

/** @emoji 🌐 Generic dev-time asset server bootstrap driven by the active playground's declared
 * asset specs — mesh-collection/static-dir are served here so Trunk proxies and native
 * `SEMIO_ASSET_BASE_URL` can resolve `/mesh/*` (and fixture routes) without Vite. */
function ensureAssetServer(variant: string): void {
  const specs = variantAssetSpecs(variant);
  if (specs.length === 0) return;
  startAssetServer(repoRoot, SEMIO_ASSET_SERVER_PORT, specs);
  console.log(`[DEBUG] asset server serving at ${assetServerBaseUrl()} (${specs.map((s) => `${s.kind}:${s.route}`).join(", ")})`);
}

/** 🎯Resolves the `--app <appId>` args for `semio-wgpu-native` from the catalog row matching `filterPlugin`, or `[]` when the row has no `app`. */
function resolveNativeAppArgs(catalog: ReturnType<typeof loadFrameworkOsPlaygroundCatalog>, filterPlugin: string): string[] {
  const row = catalog.find((r) => r.variant === filterPlugin);
  return row?.app ? ["--app", row.app] : [];
}

function buildBootScript(bundleRoot: string): void {
  const bootTs = join(bundleRoot, "js/boot.ts");
  const bootJs = join(bundleRoot, "js/boot.js");
  const variant = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
  const sessionPath = join(repoRoot, "framework/product/os/module/dev/js/generated/session.ts");
  writePlaygroundSession(variant, sessionPath, repoRoot);
  if (runCmdStatus("bun", ["build", bootTs, "--outfile", bootJs, "--target", "browser", "--format", "esm"], { cwd: bundleRoot }) !== 0) throw new Error("boot.js build failed");
}

class TrunkBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    ensureTrunk();
    ensureWasmTarget();
    buildBootScript(this.root);
    mkdirSync(outDir, { recursive: true });
    const release = segments.includes("--release") || segments.includes("--dist");
    const args = ["build", "--config", "Trunk.toml"];
    if (release) args.push("--release");
    if (runCmdStatus("trunk", args, { cwd: this.root, env: trunkEnv(), budgetMs: buildBudgetMs() }) !== 0) throw new Error("trunk build failed for wgpu renderer");
    syncStableRendererArtifacts();
    console.log(`[DEBUG] trunk built wgpu renderer -> ${outDir}`);
  }
}

class TrunkServeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    ensureTrunk();
    ensureWasmTarget();
    buildBootScript(this.root);
    const program = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    ensureAssetServer(plugin);
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(catalog, program, "wgpu"));
    const port = process.env.S_OS_PORT ?? defaultPort;
    const extra = segments.filter((segment, index, all) => segment !== "--port" && all[index - 1] !== "--port");
    const args = ["serve", "--config", "Trunk.toml", "--port", port, ...extra];
    if ((await runInteractiveCommand("trunk", args, this.root, trunkEnv())) !== 0) throw new Error("trunk serve failed for wgpu renderer");
  }
}
//#endregion 🌐 DevServer

class NativeBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    if (runCmdStatus("cargo", ["build", "-p", crateName, "--bin", "semio-wgpu-native", "--release", "--features", "native-bin"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
      throw new Error("native wgpu renderer build failed");
    }
    const osDevScript = join(repoRoot, "framework/product/os/module/dev/js/script.ts");
    // Recurses into os/dev's own `program` build loop, whose per-plugin `cargo build` calls are individually budgeted.
    const program = runCmdStatus("bun", [osDevScript, "plugin", filterPlugin], {
      cwd: join(repoRoot, "framework/product/os/module/dev/js"),
      env: { ...process.env, SEMIO_RENDERER: "wgpu", SEMIO_PLUGIN: filterPlugin },
      ...orchestratorBudgetOpts(),
    });
    if (program !== 0) throw new Error(`wasm program build failed: ${filterPlugin}`);
    console.log(`[DEBUG] built native wgpu renderer and wasm programs for ${filterPlugin}`);
  }
}

class NativeRunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    await new NativeBuildScript(this.root).run([filterPlugin]);
    ensureAssetServer(filterPlugin);
    const nativeEnv: NodeJS.ProcessEnv = {
      ...process.env,
      SEMIO_PLUGIN_MODULES: pluginOutRoot,
    };
    if (variantAssetSpecs(filterPlugin).length > 0) {
      nativeEnv[SEMIO_ASSET_BASE_URL_ENV] = assetServerBaseUrl();
    }
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const appArgs = resolveNativeAppArgs(catalog, filterPlugin);
    // Interactive native app window — runs until the user closes it.
    if (runCmdStatus("cargo", ["run", "-p", crateName, "--bin", "semio-wgpu-native", "--release", "--features", "native-bin", "--", "--plugin", filterPlugin, ...appArgs], { cwd: repoRoot, env: nativeEnv, ...daemonBudgetOpts() }) !== 0) {
      throw new Error("native wgpu renderer run failed");
    }
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([crateName], this.repoRoot, rest);
    await runVitest(this.root, rest, "vitest.config.ts");
  }
}

//#region 🔖LintScript
/** 🎨Raw color-construction calls (`Rgba::new`/`from_srgb8`) must live only inside `framework/ui/wgpu`'s theme module — the renderer takes every color via `ui_wgpu::Theme`. */
function collectWgpuColorLiteralViolations(bundleRoot: string): string[] {
  const libPath = join(bundleRoot, "rs", "lib.rs");
  if (!existsSync(libPath)) return [];
  const text = readFileSync(libPath, "utf8");
  const violations: string[] = [];
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]!;
    if (/\bRgba::new\(|\bfrom_srgb8\(/.test(line)) {
      violations.push(`rs/lib.rs:${i + 1}: ${line.trim()}`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = collectWgpuColorLiteralViolations(this.root);
    if (violations.length === 0) {
      console.log("framework-renderer-wgpu: color-literal lint passed");
      return;
    }
    console.error(`framework-renderer-wgpu: found ${violations.length} raw color-construction call(s) outside framework/ui/wgpu theme:`);
    for (const v of violations.slice(0, 40)) console.error(`  ${v}`);
    if (violations.length > 40) console.error(`  … and ${violations.length - 40} more`);
    process.exit(1);
  }
}
//#endregion 🔖LintScript

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", TrunkBuildScript)
  .register("build", TrunkBuildScript)
  .register("serve", TrunkServeScript)
  .register("dev", TrunkServeScript)
  .register("native", NativeRunScript)
  .register("native-build", NativeBuildScript)
  .register("test", TestScript)
  .register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
