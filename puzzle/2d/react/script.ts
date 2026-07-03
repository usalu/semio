#!/usr/bin/env bun
/** 🧭 `@semio-tech/puzzle-2d-react` task router: `bun ./script.ts test|policy [args…]`. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { FileLinter } from "../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/index.ts";
import {
  BundleScript,
  ScriptRouter,
  devToolingEnv,
  runBun,
  runBundleScriptMain,
  runVitest,
} from "../../../repo/lib/js/index.ts";
import { defineLint } from "../../../repo/lib/js/index.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@semio-tech/puzzle-2d-react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const wasmJs = join(this.root, "../rs/pkg/puzzle_2d.js");
    const wasmEnv = {
      ...devToolingEnv(),
      PUZZLE_2D_RS_SKIP_WASM_BUILD: existsSync(wasmJs) ? "1" : "0",
    };
    runBun([wasmScript, "wasm"], this.root, wasmEnv);
    runVitest(this.root, segments, "vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
