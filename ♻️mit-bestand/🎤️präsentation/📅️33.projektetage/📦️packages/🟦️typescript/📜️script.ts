#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-praesentation-projektetage` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { BundleScript, ScriptRouter, playPollingEnv, playgroundDevPortString, playgroundPortEnv, runBun, runBundleScriptMain, runViteBunxDev } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDev(this.root, segments, {
      config: "../../🏗️builder/🌐️vite/🟦️.ts",
      portEnv: playgroundPortEnv("projektetage"),
      defaultPort: playgroundDevPortString("projektetage"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.some((arg) => /^(?:--outDir|--config|--root)(?:=|$)/.test(arg))) throw new Error("Build output and configuration are owned by this Nx target");
    runBun(["run", "vite", "build", "--config", "../../🏗️builder/🌐️vite/🟦️.ts", ...segments], this.root, playPollingEnv());
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
