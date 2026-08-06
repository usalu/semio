#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmdStatus, runViteBunxDev, withViteConfigLoader } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
import { buildEngineWasm, buildPlugins, ensurePluginRegistry } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📜️script.ts";
import { DEMONSTRATOR_PANES } from "./🟦️brand.ts";

const demonstratorRoot = import.meta.dir;

/** @emoji 🎪️ Builds the demonstrator's plugin crate + every pane's declared engines into the shared
 * `🧧framework/os/dev` `🔌️plugin-modules/` dir this page's own `⚙️vite.config.ts` static-serves from.
 *
 * 🎪️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: all six panes now share ONE plugin crate
 * (`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust` — see its docstring),
 * so the plugin build only needs to run once, keyed off any one pane's variant (`buildPlugins`/
 * `ensurePluginRegistry` resolve a variant to its plugin crate; all six now resolve to the same one).
 * This also removes the six-way `writePlaygroundSession` race that used to exist here — each pane's
 * `buildPlugins` call overwrote the same generated session file with its own variant, in array order.
 * Engines still need a per-pane pass: only some panes declare one (e.g. only `verfolgen` needs
 * tiled-map), and each variant's own registry row carries its own `engines` list independently even
 * though they now share a `pluginId`. */
async function buildDemonstratorPlugins(): Promise<void> {
  const primaryVariant = DEMONSTRATOR_PANES[0]?.variant;
  if (primaryVariant) {
    if (process.env.SKIP_PLUGIN_BUILD === "1") {
      await ensurePluginRegistry(primaryVariant);
    } else {
      await buildPlugins(primaryVariant);
    }
  }
  for (const pane of DEMONSTRATOR_PANES) {
    await buildEngineWasm(pane.variant, "react");
  }
}

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
