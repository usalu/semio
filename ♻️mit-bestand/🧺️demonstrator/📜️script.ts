#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmdStatus, runViteBunxDev } from "../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { buildEngineWasm, buildPlugins, ensurePluginRegistry } from "../../🧰️framework/🛍️product/💻️os/🔨️module/🧑️‍💻️dev/⚡️implementation/🟦️typescript/📜️script.ts";
import { DEMONSTRATOR_PANES } from "./🟦️brand.ts";

const demonstratorRoot = import.meta.dir;

/** @emoji 🎪️ Builds every demonstrator pane's plugin crate + declared engines into the shared
 * `🧧framework/os/dev` `🔌️plugin-modules/` dir this page's own `⚙️vite.config.ts` static-serves from —
 * one live page hosting six panes needs all six variants' plugins available at once, unlike a single
 * `os/dev` session which only ever needs its own active variant. */
async function buildDemonstratorPlugins(): Promise<void> {
  for (const pane of DEMONSTRATOR_PANES) {
    if (process.env.SKIP_PLUGIN_BUILD === "1") {
      await ensurePluginRegistry(pane.variant);
    } else {
      await buildPlugins(pane.variant);
    }
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
    await buildDemonstratorPlugins();
    if (runCmdStatus("bun", ["run", "vite", "build", "--config", "⚙️vite.config.ts", ...segments], { cwd: this.root, env: process.env }) !== 0) {
      throw new Error("demonstrator landing build failed");
    }
    console.log(`[build] demonstrator built at ${join(demonstratorRoot, "dist")}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
