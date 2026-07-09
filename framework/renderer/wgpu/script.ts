#!/usr/bin/env bun
/** @emoji 🧊 `@semio-tech/framework-renderer-wgpu` task router. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, GIS_MAP_WGPU_TILE_PROXY_PORT, SEMIO_GIS_MAP_TILE_BASE_URL_ENV, getWorkspaceRoot, runBundleScriptMain, runVitest, frameworkOsPlaygroundDefaultPort } from "../../../repo/lib/js/index.ts";
import { startGisMapTileProxyServer } from "../../../ui/styling/vite-elements-assets.ts";

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

function gisMapTileProxyBaseUrl(): string {
  return `http://127.0.0.1:${GIS_MAP_WGPU_TILE_PROXY_PORT}`;
}

function ensureGisMapTileProxyServer(plugin: string): void {
  if (plugin !== "gis") return;
  startGisMapTileProxyServer(repoRoot, GIS_MAP_WGPU_TILE_PROXY_PORT);
  console.log(`[DEBUG] gis tile proxy serving at ${gisMapTileProxyBaseUrl()}`);
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
    ensureGisMapTileProxyServer(plugin);
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(plugin, "wgpu"));
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
    ensureGisMapTileProxyServer(filterPlugin);
    const nativeEnv: NodeJS.ProcessEnv = {
      ...process.env,
      SEMIO_PLUGIN_MODULES: pluginOutRoot,
    };
    if (filterPlugin === "gis") {
      nativeEnv[SEMIO_GIS_MAP_TILE_BASE_URL_ENV] = gisMapTileProxyBaseUrl();
    }
    const run = spawnSync("cargo", ["run", "-p", crateName, "--bin", "semio-wgpu-native", "--release", "--features", "native-bin", "--", "--plugin", filterPlugin], {
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

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", TrunkBuildScript)
  .register("build", TrunkBuildScript)
  .register("serve", TrunkServeScript)
  .register("dev", TrunkServeScript)
  .register("native", NativeRunScript)
  .register("native-build", NativeBuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
