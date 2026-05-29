#!/usr/bin/env bun
/** 🧭 `@puzzle/2d-play` task router: `bun ./script.ts <dev|build|test> [args…]`. */
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBun,
  runBundleScriptMain,
  runCargo,
  runPlaywright,
  runViteBunxDev,
  runVitest,
} from "../../../repo/lib/js/src/bundle-script.ts";

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runViteBunxDev(this.root, segments, { portEnv: "BOARD_PLAY_PORT", defaultPort: "6012" });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "puzzle_board"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runVitest(this.root, segments);
    runPlaywright(this.root, "playwright.config.ts", segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
