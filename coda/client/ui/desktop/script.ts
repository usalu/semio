#!/usr/bin/env bun
/** 🧭 `@coda/desktop` router: `bun ./script.ts <dev|build|publish|policy> [args…]`. */
import type { FileLinter } from "../../../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/src/index.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/src/index.ts";
import { BundleScript, ScriptRouter, runBunx, runBundleScriptMain } from "../../../../repo/lib/js/src/index.ts";
import { defineLint } from "../../../../repo/lib/js/src/index.ts";

export const policyFile = "renderer.tsx";

export const policy = defineLint("@coda/desktop-renderer", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const forgeEnv = (): NodeJS.ProcessEnv => ({
  ...process.env,
  ELECTRON_DISABLE_SANDBOX: process.env.ELECTRON_DISABLE_SANDBOX ?? "1",
});

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["electron-forge", "start", ...segments], this.root, forgeEnv());
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["electron-forge", "make", ...segments], this.root, {
      ...forgeEnv(),
      NODE_OPTIONS: process.env.NODE_OPTIONS ?? "--max-old-space-size=8192",
    });
  }
}

class PublishScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["electron-forge", "publish", ...segments], this.root, forgeEnv());
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root, forgeEnv());
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("publish", PublishScript)
  .register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url);
