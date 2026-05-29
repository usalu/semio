#!/usr/bin/env bun
/** 🧭 `@puzzle/2d-react` task router: `bun ./script.ts test|policy [args…]`. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { FileLinter } from "../../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/src/cli.ts";
import {
  BundleScript,
  ScriptRouter,
  devToolingEnv,
  runBun,
  runBundleScriptMain,
  runVitest,
} from "../../../repo/lib/js/src/bundle-script.ts";
import { defineLint } from "../../../repo/lib/js/src/script.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@puzzle/2d-react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const wasmJs = join(this.root, "../rs/pkg/elements_board.js");
    const wasmEnv = {
      ...devToolingEnv(),
      ELEMENTS_BOARD_SKIP_WASM_BUILD: existsSync(wasmJs) ? "1" : "0",
    };
    runBun([wasmScript, "wasm"], this.root, wasmEnv);
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
