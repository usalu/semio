#!/usr/bin/env bun
/** 🧭 `@semio-tech/puzzle-2d-play` task router: `bun ./script.ts <dev|build|test> [args…]`. */
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  consumePlaygroundFixtureArgv,
  playPollingEnv,
  playgroundDevPortString,
  playgroundPortEnv,
  runBun,
  runBundleScriptMain,
  runCargo,
  runPlaywright,
  runViteBunxDev,
  runVitest,
} from "../../../repo/lib/js/src/index.ts";
import { resolvePuzzle2dPlayFixtureSlug } from "./index.ts";

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolvePuzzle2dPlayFixtureSlug);
    runBun([wasmScript, "wasm"], this.root, playPollingEnv(fixtureEnv));
    Object.assign(process.env, fixtureEnv);
    runViteBunxDev(this.root, viteSegments, {
      portEnv: playgroundPortEnv("puzzle-2d"),
      defaultPort: playgroundDevPortString("puzzle-2d"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolvePuzzle2dPlayFixtureSlug);
    runBun([wasmScript, "wasm"], this.root, playPollingEnv(fixtureEnv));
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteSegments], this.root, playPollingEnv(fixtureEnv));
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "puzzle_2d"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runVitest(this.root, segments);
    runPlaywright(this.root, "playwright.config.ts", ["--pass-with-no-tests", ...segments]);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
