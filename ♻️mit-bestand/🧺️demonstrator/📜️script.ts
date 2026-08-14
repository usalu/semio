#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmdStatus, runViteBunxDev, withViteConfigLoader } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { buildEngineWasm, buildPlugins, ensurePluginRegistry } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { DEMONSTRATOR_PANES, demonstratorPaneRuntimeVariant } from "./🟦️brand.ts";

const demonstratorRoot = import.meta.dir;

//#region 🎪️DemonstratorPluginBuild
/** @emoji 🎯️ Builds only the primary crate behind a runtime variant; its contributed extensions are
 * already included by the demonstrator crate's own consumer closure. */
async function buildRuntimePlugin(variant: string): Promise<void> {
  const pluginId = PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === variant)?.pluginId;
  if (!pluginId) throw new Error(`unknown demonstrator runtime variant: ${variant}`);
  const previousPluginOnly = process.env.SEMIO_PLUGIN_ONLY;
  process.env.SEMIO_PLUGIN_ONLY = pluginId;
  try {
    await buildPlugins(variant);
  } finally {
    if (previousPluginOnly === undefined) delete process.env.SEMIO_PLUGIN_ONLY;
    else process.env.SEMIO_PLUGIN_ONLY = previousPluginOnly;
  }
}

/** @emoji 🎪️ Builds every pane's runtime plugin crate + declared engines into the shared
 * `🧧framework/os/dev` `🔌️plugin-modules/` dir this page's own `⚙️vite.config.ts` static-serves from.
 *
 * Five panes share the demonstrator crate. Generator deliberately boots the procedural crate, so it
 * must be built separately rather than consuming whichever procedural artifact happens to be staged.
 * The primary variant is restored after both builds so registry session generation stays deterministic.
 * Engines still need a per-pane pass: only some panes declare one (e.g. only `verfolgen` needs
 * tiled-map), and each variant's own registry row carries its own `engines` list independently even
 * though they now share a `pluginId`. */
async function buildDemonstratorPlugins(): Promise<void> {
  const primaryVariant = DEMONSTRATOR_PANES[0]?.variant;
  if (primaryVariant) {
    const stagedCore = join(import.meta.dir, "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/demonstrator/semio_s_plugin_demonstrator_component.core.wasm");
    const hasStaged = existsSync(stagedCore);
    if (process.env.SKIP_PLUGIN_BUILD === "1" || (hasStaged && process.env.FORCE_PLUGIN_BUILD !== "1")) {
      if (hasStaged && process.env.SKIP_PLUGIN_BUILD !== "1") {
        console.log("[DEBUG] reusing staged demonstrator plugin-modules (set FORCE_PLUGIN_BUILD=1 to rebuild)");
      }
      await ensurePluginRegistry(primaryVariant);
      if (process.env.SKIP_ENGINE_BUILD !== "0" && process.env.FORCE_ENGINE_BUILD !== "1") {
        process.env.SKIP_ENGINE_BUILD = "1";
      }
    } else {
      await buildPlugins(primaryVariant);
    }
  }
  const secondaryRuntimeVariants = [...new Set(DEMONSTRATOR_PANES.map((pane) => demonstratorPaneRuntimeVariant(pane.variant)))].filter((variant) => variant !== primaryVariant);
  if (process.env.SKIP_PLUGIN_BUILD !== "1") {
    for (const variant of secondaryRuntimeVariants) await buildRuntimePlugin(variant);
  }
  if (primaryVariant) await ensurePluginRegistry(primaryVariant);
  for (const pane of DEMONSTRATOR_PANES) {
    await buildEngineWasm(pane.variant, "react");
  }
}
//#endregion 🎪️DemonstratorPluginBuild

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildDemonstratorPlugins();
    runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], {
      portEnv: "MIT_BESTAND_DEMONSTRATOR_PORT",
      defaultPort: "6029",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    process.env.SEMIO_BUILD_MODE = "ship";
    await buildDemonstratorPlugins();
    if (runCmdStatus("bun", withViteConfigLoader(["run", "vite", "build", "--config", "⚙️vite.config.ts", ...segments]), { cwd: this.root, env: process.env }) !== 0) {
      throw new Error("demonstrator landing build failed");
    }
    console.log(`[build] demonstrator built at ${join(demonstratorRoot, "dist")}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
