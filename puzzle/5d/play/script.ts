#!/usr/bin/env bun
/** 🧭 `@puzzle/5d/play` task router: `bun ./script.ts <dev|build|test|regenerate-fixture> [args…]`. */
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runPlaywright, runViteBunxDev, runVitest } from "../../../repo/lib/js/src/index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDev(this.root, segments, { portEnv: "PUZZLE_5D_PLAY_PORT", defaultPort: "6014" });
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

class Regenerate5dFixtureScript extends BundleScript {
  run(): void {
    process.env.REGENERATE_NAKAGIN_5D = "1";
    try {
      runVitest(this.root, ["-t", "regenerates nakagin 5d fixture"]);
    } finally {
      delete process.env.REGENERATE_NAKAGIN_5D;
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript).register("regenerate-fixture", Regenerate5dFixtureScript);

await runBundleScriptMain(router, import.meta.url);
