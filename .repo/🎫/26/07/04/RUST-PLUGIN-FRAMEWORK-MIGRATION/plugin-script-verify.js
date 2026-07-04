#!/usr/bin/env bun
// @bun

// framework/product/os/dev/script.ts
import { spawnSync } from "child_process";
import { mkdirSync, watch } from "fs";
import { join } from "path";
import {
  BundleScript,
  ScriptRouter,
  getWorkspaceRoot,
  runBundleScriptMain,
  runVitest,
  runViteBunxDev
} from "../../../../repo/lib/js/index.ts";
import { PLUGIN_BUILD_TARGETS } from "./js/plugin-registry.ts";
var repoRoot = getWorkspaceRoot();
var wasmTarget = "wasm32-unknown-unknown";
var pluginOutRoot = join(repoRoot, "framework/product/os/dev/public/plugin-modules");
function ensureWasmTarget() {
  const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
  if (!probe.stdout?.includes(wasmTarget)) {
    spawnSync("rustup", ["target", "add", wasmTarget], { stdio: "inherit" });
  }
}
async function readPackageName(cratePath) {
  const content = await Bun.file(join(repoRoot, cratePath, "Cargo.toml")).text();
  const match = content.match(/^name = "([^"]+)"/m);
  if (!match)
    throw new Error(`missing package name in ${cratePath}/Cargo.toml`);
  return match[1];
}
async function buildPlugin(target) {
  const packageName = await readPackageName(target.cratePath);
  const build = spawnSync("cargo", ["build", "-p", packageName, "--target", wasmTarget, "--release"], { cwd: repoRoot, stdio: "inherit" });
  if (build.status !== 0)
    throw new Error(`plugin build failed: ${target.pluginId}`);
  const artifact = join(repoRoot, "target", wasmTarget, "release", `${packageName.replace(/-/g, "_")}.wasm`);
  const outDir = join(pluginOutRoot, target.pluginId);
  mkdirSync(outDir, { recursive: true });
  const wasmBindgen = spawnSync("wasm-bindgen", ["--version"], { encoding: "utf8" });
  if (wasmBindgen.status !== 0) {
    spawnSync("cargo", ["install", "wasm-bindgen-cli", "--locked"], { stdio: "inherit" });
  }
  const bindgen = spawnSync("wasm-bindgen", ["--target", "web", "--out-dir", outDir, "--out-name", target.wasmOut.replace(/\.wasm$/, ""), artifact], { cwd: repoRoot, stdio: "inherit" });
  if (bindgen.status !== 0)
    throw new Error(`wasm-bindgen failed: ${target.pluginId}`);
  console.log(`[DEBUG] built plugin ${target.pluginId} -> ${outDir}`);
}

class PluginBuildScript extends BundleScript {
  async run(_segments) {
    ensureWasmTarget();
    mkdirSync(pluginOutRoot, { recursive: true });
    for (const target of PLUGIN_BUILD_TARGETS) {
      await buildPlugin(target);
    }
  }
}

class PluginWatchScript extends BundleScript {
  async run(_segments) {
    await new PluginBuildScript(this.root).run([]);
    for (const target of PLUGIN_BUILD_TARGETS) {
      const watchRoot = join(repoRoot, target.cratePath);
      watch(watchRoot, { recursive: true }, () => {
        buildPlugin(target).catch((error) => {
          console.error("[DEBUG] plugin watch rebuild failed", error);
        });
      });
    }
    console.log("[DEBUG] watching plugin crates for hot-swap rebuilds");
  }
}

class DevScript extends BundleScript {
  async run(segments) {
    await new PluginBuildScript(this.root).run([]);
    const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    runViteBunxDev(this.root, segments, {
      portEnv: "S_OS_PORT",
      defaultPort: "6066",
      fixedPort: true,
      env: {
        SEMIO_PLUGIN: plugin
      }
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments) {
    await new PluginBuildScript(this.root).run([]);
    spawnSync("bun", ["run", "vite", "build", "--config", "vite.config.ts", ...segments], {
      cwd: this.root,
      stdio: "inherit"
    });
  }
}

class TestScript extends BundleScript {
  run(segments) {
    runVitest(this.root, segments, "vitest.config.ts");
  }
}
var router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript).register("plugin", class extends BundleScript {
  async run(segments) {
    const sub = segments[0];
    if (sub === "watch")
      return new PluginWatchScript(this.root).run(segments.slice(1));
    return new PluginBuildScript(this.root).run(segments.slice(1));
  }
});
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
