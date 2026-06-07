#!/usr/bin/env bun
/** 🧭 `@dag/play` task router. */
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
} from "../../../../../../repo/lib/js/src/index.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../../../../../ui/styling/playground-dev-ports.ts";

const wasmScript = join(import.meta.dir, "../script.ts");
const validateRuntimeScript = join(
  import.meta.dir,
  "../../../../../../.repo/🎫/26/06/07/EXTRACT-GENERIC-GRAPH-CANVAS-FROM-PUZZLE-2D-AND-ADD-DAG/validate-dag-runtime.mjs",
);

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], join(import.meta.dir, ".."), playPollingEnv());
    runViteBunxDev(this.root, segments, {
      portEnv: playgroundPortEnv("dag"),
      defaultPort: playgroundDevPortString("dag"),
      fixedPort: true,
    });
  }
}

class ValidateScript extends BundleScript {
  run(segments: string[]): void {
    runBun([validateRuntimeScript, ...segments], this.root, {
      ...playPollingEnv(),
      DAG_PLAY_PORT: process.env.DAG_PLAY_PORT ?? playgroundDevPortString("dag"),
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], join(import.meta.dir, ".."), playPollingEnv());
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "mathematical_graph_port_directed_dag"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], join(import.meta.dir, ".."), playPollingEnv());
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("validate", ValidateScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
