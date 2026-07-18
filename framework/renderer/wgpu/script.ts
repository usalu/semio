#!/usr/bin/env bun
/** @emoji 🧊 `@semio-tech/framework-renderer-wgpu` task router. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  SEMIO_ASSET_SERVER_PORT,
  SEMIO_ASSET_BASE_URL_ENV,
  getWorkspaceRoot,
  runBundleScriptMain,
  runVitest,
  frameworkOsPlaygroundDefaultPort,
  loadFrameworkOsPlaygroundCatalog,
} from "../../../repo/lib/js/index.ts";
import { startAssetServer } from "../../../ui/styling/vite-elements-assets.ts";
import type { PlaygroundAssetSpec } from "../../plugin/registry/generated/playgrounds.ts";

const repoRoot = getWorkspaceRoot();
const wasmTarget = "wasm32-unknown-unknown";
const crateName = "semio-framework-renderer-wgpu";
const outDir = join(repoRoot, "framework/product/os/dev/renderer-modules/wgpu");
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");

function trunkEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  delete env.NO_COLOR;
  delete env.FORCE_COLOR;
  return env;
}

function ensureWasmTarget(): void {
  const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
  if (!probe.stdout?.includes(wasmTarget)) {
    spawnSync("rustup", ["target", "add", wasmTarget], { stdio: "inherit" });
  }
}

function ensureTrunk(): void {
  const probe = spawnSync("trunk", ["--version"], { encoding: "utf8" });
  if (probe.status !== 0) {
    spawnSync("cargo", ["install", "trunk", "--locked"], { stdio: "inherit" });
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

/** @emoji 🗂️ The active playground variant's `tile-proxy` asset specs (the only kind the standalone
 * asset server serves — `static-dir`/`mesh-collection` assets are Vite-only). */
function variantTileProxyAssets(variant: string): readonly Extract<PlaygroundAssetSpec, { kind: "tile-proxy" }>[] {
  const row = loadFrameworkOsPlaygroundCatalog().find((entry) => entry.variant === variant);
  return (row?.assets ?? []).filter((asset): asset is Extract<PlaygroundAssetSpec, { kind: "tile-proxy" }> => asset.kind === "tile-proxy");
}

/** @emoji 🌐 Generic dev-time asset server bootstrap driven by the active playground's declared
 * `tile-proxy` asset specs — replaces the previous GIS-only `ensureGisMapTileProxyServer` (which never
 * actually triggered: it compared the playground *variant* env value, e.g. `"gis2d"`, against the
 * *pluginId* `"gis"`, a mismatch fixed here by resolving specs straight from the registry instead). */
function ensureAssetServer(variant: string): void {
  const specs = variantTileProxyAssets(variant);
  if (specs.length === 0) return;
  startAssetServer(repoRoot, SEMIO_ASSET_SERVER_PORT, specs);
  console.log(`[DEBUG] asset proxy serving at ${assetServerBaseUrl()}`);
}

/** 🎯Resolves the `--app <appId>` args for `semio-wgpu-native` from the catalog row matching `filterPlugin`, or `[]` when the row has no `app`. */
function resolveNativeAppArgs(catalog: ReturnType<typeof loadFrameworkOsPlaygroundCatalog>, filterPlugin: string): string[] {
  const row = catalog.find((r) => r.variant === filterPlugin);
  return row?.app ? ["--app", row.app] : [];
}

function buildBootScript(bundleRoot: string): void {
  const bootTs = join(bundleRoot, "js/boot.ts");
  const bootJs = join(bundleRoot, "js/boot.js");
  const pluginFilter = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
  const build = spawnSync("bun", ["build", bootTs, "--outfile", bootJs, "--target", "browser", "--format", "esm", "--define", `DEFAULT_PLUGIN_FILTER=${JSON.stringify(pluginFilter)}`], { cwd: bundleRoot, stdio: "inherit" });
  if (build.status !== 0) throw new Error("boot.js build failed");
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
    const build = spawnSync("trunk", args, { cwd: this.root, stdio: "inherit", env: trunkEnv() });
    if (build.status !== 0) throw new Error("trunk build failed for wgpu renderer");
    syncStableRendererArtifacts();
    console.log(`[DEBUG] trunk built wgpu renderer -> ${outDir}`);
  }
}

class TrunkServeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    ensureTrunk();
    ensureWasmTarget();
    buildBootScript(this.root);
    const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    ensureAssetServer(plugin);
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(catalog, plugin, "wgpu"));
    const port = process.env.S_OS_PORT ?? defaultPort;
    const extra = segments.filter((segment, index, all) => segment !== "--port" && all[index - 1] !== "--port");
    const args = ["serve", "--config", "Trunk.toml", "--port", port, ...extra];
    const serve = spawnSync("trunk", args, { cwd: this.root, stdio: "inherit", env: trunkEnv() });
    if (serve.status !== 0) throw new Error("trunk serve failed for wgpu renderer");
  }
}

class NativeBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    const renderer = spawnSync("cargo", ["build", "-p", crateName, "--bin", "semio-wgpu-native", "--release", "--features", "native-bin"], { cwd: repoRoot, stdio: "inherit" });
    if (renderer.status !== 0) throw new Error("native wgpu renderer build failed");
    const osDevScript = join(repoRoot, "framework/product/os/dev/script.ts");
    const plugin = spawnSync("bun", [osDevScript, "plugin", filterPlugin], {
      cwd: join(repoRoot, "framework/product/os/dev"),
      stdio: "inherit",
      env: { ...process.env, SEMIO_RENDERER: "wgpu", SEMIO_PLUGIN: filterPlugin },
    });
    if (plugin.status !== 0) throw new Error(`wasm plugin build failed: ${filterPlugin}`);
    console.log(`[DEBUG] built native wgpu renderer and wasm plugins for ${filterPlugin}`);
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
    if (variantTileProxyAssets(filterPlugin).length > 0) {
      nativeEnv[SEMIO_ASSET_BASE_URL_ENV] = assetServerBaseUrl();
    }
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const appArgs = resolveNativeAppArgs(catalog, filterPlugin);
    const run = spawnSync("cargo", ["run", "-p", crateName, "--bin", "semio-wgpu-native", "--release", "--features", "native-bin", "--", "--plugin", filterPlugin, ...appArgs], {
      cwd: repoRoot,
      stdio: "inherit",
      env: nativeEnv,
    });
    if (run.status !== 0) throw new Error("native wgpu renderer run failed");
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "vitest.config.ts");
  }
}

//#region 🔖LintScript
/** 🎨Raw color-construction calls (`Rgba::new`/`from_srgb8`) must live only inside `ui/wgpu`'s theme module — the renderer takes every color via `ui_wgpu::Theme`. */
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
    console.error(`framework-renderer-wgpu: found ${violations.length} raw color-construction call(s) outside ui/wgpu theme:`);
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
