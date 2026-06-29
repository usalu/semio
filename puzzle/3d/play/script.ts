#!/usr/bin/env bun
/** 🧭 `@semio-tech/puzzle-3d-play` task router: `bun ./script.ts <dev|build|test> [fixture <id>] [args…]`. */
import {
  BundleScript,
  ScriptRouter,
  consumePlaygroundFixtureArgv,
  playPollingEnv,
  playgroundDevPortString,
  playgroundPortEnv,
  runBun,
  runBundleScriptMain,
  runPlaywright,
  runViteBunxDev,
  runVitest,
} from "../../../repo/lib/js/src/index.ts";
import { resolvePuzzle3dPlayFixtureSlug } from "./index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolvePuzzle3dPlayFixtureSlug);
    Object.assign(process.env, fixtureEnv);
    runViteBunxDev(this.root, viteSegments, {
      portEnv: playgroundPortEnv("puzzle-3d"),
      defaultPort: playgroundDevPortString("puzzle-3d"),
      fixedPort: true,
      clearViteCache: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolvePuzzle3dPlayFixtureSlug);
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteSegments], this.root, playPollingEnv(fixtureEnv));
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
    runPlaywright(this.root, "playwright.config.ts", ["--pass-with-no-tests", ...segments]);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
