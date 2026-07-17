#!/usr/bin/env bun
/** 🧭 Algorithms bundle router: `bun ./script.ts dev [storybook args…]` / `bun ./script.ts test [args…]`. */
import { BundleScript, ScriptRouter, devToolingEnv, runBundleScriptMain, runVitest, spawnBunx } from "../../../repo/lib/js/index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    const port = process.env.STORYBOOK_PORT ?? "6006";
    const env = devToolingEnv({
      WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
      CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    });
    spawnBunx(["storybook", "dev", "-c", ".storybook", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...segments], this.repoRoot, env);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
