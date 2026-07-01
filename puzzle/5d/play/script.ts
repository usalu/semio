#!/usr/bin/env bun
/** 🧭 `@semio-tech/puzzle-5d-play` task router: `bun ./script.ts <dev|build|test|regenerate-fixture> [fixture <id>] [args…]`. */
import { BundleScript, ScriptRouter, consumePlaygroundFixtureArgv, playPollingEnv, playgroundDevPortString, playgroundPortEnv, runBun, runBundleScriptMain, runPlaywright, runViteBunxDev, runVitest } from "../../../repo/lib/js/index.ts";
import { resolvePuzzle5dPlayFixtureSlug } from "./index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolvePuzzle5dPlayFixtureSlug);
    Object.assign(process.env, fixtureEnv);
    runViteBunxDev(this.root, viteSegments, {
      portEnv: playgroundPortEnv("puzzle-5d"),
      defaultPort: playgroundDevPortString("puzzle-5d"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolvePuzzle5dPlayFixtureSlug);
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteSegments], this.root, playPollingEnv(fixtureEnv));
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
    runPlaywright(this.root, "playwright.config.ts", ["--pass-with-no-tests", ...segments]);
  }
}

class RegenerateNakagin5dFixtureScript extends BundleScript {
  run(): void {
    process.env.REGENERATE_NAKAGIN_5D = "1";
    try {
      runVitest(this.root, ["-t", "regenerates nakagin 5d fixture"]);
    } finally {
      delete process.env.REGENERATE_NAKAGIN_5D;
    }
  }
}

class RegenerateConcreteForest5dFixtureScript extends BundleScript {
  run(): void {
    process.env.REGENERATE_CONCRETE_FOREST_5D = "1";
    try {
      runVitest(this.root, ["-t", "regenerates concrete forest 5d fixture"]);
    } finally {
      delete process.env.REGENERATE_CONCRETE_FOREST_5D;
    }
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("regenerate-fixture", RegenerateNakagin5dFixtureScript)
  .register("regenerate-concrete-forest-fixture", RegenerateConcreteForest5dFixtureScript);

await runBundleScriptMain(router, import.meta.url);
