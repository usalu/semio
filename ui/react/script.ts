#!/usr/bin/env bun
/** 🧭 Elements react UI router: `bun ./script.ts <dev|build|lint|test|policy> [args…]`. */
import type { BundleLinter } from "../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../repo/lib/js/src/index.ts";
import { getWorkspaceRoot } from "../../repo/lib/js/src/index.ts";
import {
  BundleScript,
  ScriptRouter,
  devToolingEnv,
  runBundleScriptMain,
  runBunx,
  spawnBunx,
} from "../../repo/lib/js/src/index.ts";
import { defineLint } from "../../repo/lib/js/src/index.ts";

export const policy = defineLint("@semio-tech/ui-react-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

const storybookEnv = (extra: Record<string, string | undefined> = {}) =>
  devToolingEnv({
    WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
    CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    ...extra,
  });

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    const port = process.env.STORYBOOK_PORT ?? "6006";
    spawnBunx(
      ["storybook", "dev", "-c", ".storybook", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...segments],
      this.repoRoot,
      storybookEnv(),
    );
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    spawnBunx(
      ["storybook", "build", "-c", ".storybook", ...segments],
      this.repoRoot,
      storybookEnv({ STORYBOOK_PRODUCTION_SLICES: "ui,puzzle" }),
    );
  }
}

class LintScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "eslint.config.ts", ".", ...segments], this.root, storybookEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["vitest", "run", "--config", "vitest.config.ts", "--passWithNoTests", ...segments], this.root, storybookEnv());
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root, storybookEnv());
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("test", TestScript)
  .register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url);
