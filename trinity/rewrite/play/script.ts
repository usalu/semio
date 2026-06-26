#!/usr/bin/env bun
/** 🧭 `@semio-tech/trinity-rewrite-play` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runCargo, runViteBunxDev, runVitest } from "../../../repo/lib/js/src/index.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../../ui/styling/playground-dev-ports.ts";

const wasmScript = join(import.meta.dir, "../engine/script.ts");

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], join(import.meta.dir, "../engine"), playPollingEnv());
    runViteBunxDev(this.root, segments, {
      portEnv: playgroundPortEnv("trinity-rewrite"),
      defaultPort: playgroundDevPortString("trinity-rewrite"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], join(import.meta.dir, "../engine"), playPollingEnv());
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "trinity_rewrite"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], join(import.meta.dir, "../engine"), playPollingEnv());
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
