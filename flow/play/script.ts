#!/usr/bin/env bun
/** 🧭 `@flow/play` task router. */
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
} from "../../repo/lib/js/src/index.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../ui/styling/playground-dev-ports.ts";

const wasmScript = join(import.meta.dir, "../core/script.ts");
const validateRuntimeScript = join(
  import.meta.dir,
  "../../.repo/🎫/26/06/07/FLOW-LANGUAGE-VERTICAL-SLICE/validate-flow-runtime.mjs",
);

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runViteBunxDev(this.root, segments, {
      portEnv: playgroundPortEnv("flow"),
      defaultPort: playgroundDevPortString("flow"),
      fixedPort: true,
    });
  }
}

class ValidateScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runBun([validateRuntimeScript, ...segments], this.root, {
      ...playPollingEnv(),
      FLOW_PLAY_PORT: process.env.FLOW_PLAY_PORT ?? playgroundDevPortString("flow"),
    });
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
    runCargo(["test", "-p", "flow_core"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("validate", ValidateScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
