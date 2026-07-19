#!/usr/bin/env bun
/** 🧭 Elements react UI router: `bun ./script.ts <dev|build|lint|test|policy> [args…]`. */
import type { BundleLinter } from "../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, devToolingEnv, runBundleScriptMain, runBunx, runCmd, runVitest } from "../../../repo/lib/js/index.ts";
import { defineLint } from "../../../repo/lib/js/index.ts";

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

/** This bundle has no local `.storybook` config — its stories live in the root Storybook's `ui` scope, so `dev`/`build` delegate there instead of running a broken standalone instance. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./script.ts", "dev", "storybook", "ui", ...segments], { cwd: this.repoRoot, env: storybookEnv() });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./script.ts", "build", "storybook", ...segments], { cwd: this.repoRoot, env: storybookEnv({ STORYBOOK_SCOPE: "ui" }) });
  }
}

class LintScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "eslint.config.ts", ".", ...segments], this.root, storybookEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root, storybookEnv());
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("lint", LintScript).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url);
