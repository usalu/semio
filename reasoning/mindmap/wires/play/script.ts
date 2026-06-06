#!/usr/bin/env bun
/** 🔗 `@reasoning/mindmap/wires/play` router: `bun ./script.ts <dev|build|test>`. */
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBun,
  runBundleScriptMain,
  runCargo,
  runViteBunxDev,
  runVitest,
} from "../../../../repo/lib/js/src/index.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([join(this.repoRoot, "puzzle/2d/rs/script.ts"), "wasm"], this.root, playPollingEnv());
    runViteBunxDev(this.root, segments, { portEnv: "WIRES_PLAY_PORT", defaultPort: "6015" });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([join(this.repoRoot, "puzzle/2d/rs/script.ts"), "wasm"], this.root, playPollingEnv());
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(
      ["test", "-p", "reasoning_mindmap_wires", "-p", "reasoning_mindmap", "-p", "reasoning_mindmap_rs", "-p", "puzzle_2d"],
      this.repoRoot,
      playPollingEnv(),
    );
    runBun([join(this.repoRoot, "puzzle/2d/rs/script.ts"), "wasm"], this.root, playPollingEnv());
    runBun([join(this.repoRoot, "reasoning/mindmap/wires/react/script.ts"), "test"], this.root, playPollingEnv());
    runVitest(join(this.repoRoot, "reasoning/mindmap/wires/react"), segments);
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
