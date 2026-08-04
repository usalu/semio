#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-praesentation-projektetage` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { BundleScript, ScriptRouter, playPollingEnv, playgroundDevPortString, playgroundPortEnv, runBun, runBundleScriptMain, runViteBunxDev } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDev(this.root, ["--config", "js/⚙️vite.config.ts", ...segments], {
      portEnv: playgroundPortEnv("projektetage"),
      defaultPort: playgroundDevPortString("projektetage"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun(["run", "vite", "build", "--config", "js/⚙️vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
