#!/usr/bin/env bun
/** 🧭 `@puzzle/5d/play` task router: `bun ./script.ts <dev|build|test> [args…]`. */
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBun,
  runBundleScriptMain,
  runPlaywright,
  runViteBunxDev,
  runVitest,
} from "../../../repo/lib/js/src/bundle-script.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDev(this.root, segments, { portEnv: "TOPOLOGY_PLAY_PORT", defaultPort: "6014" });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
    runPlaywright(this.root, "playwright.config.ts", segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
