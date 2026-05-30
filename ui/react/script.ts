#!/usr/bin/env bun
/** 🧭 Elements react UI router: `bun ./script.ts <dev|build|lint|test|policy> [args…]`. */
import type { FileLinter } from "../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../repo/lib/js/src/index.ts";
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

export const policyFile = "index.tsx";

export const policy = defineLint("@ui/react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const storybookEnv = () =>
  devToolingEnv({
    WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
    CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
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
    spawnBunx(["storybook", "build", "-c", ".storybook", ...segments], this.repoRoot, storybookEnv());
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

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
