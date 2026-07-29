#!/usr/bin/env bun
/** 🧭 `@semio-tech/coda-assistant` router: `bun ./script.ts <dev|build|policy>`. */
import type { FileLinter } from "../../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, daemonBudgetOpts, runBundleScriptMain, runBunx, runCmd } from "../../../../repo/lib/js/index.ts";
import { defineLint } from "../../../../repo/lib/js/index.ts";

export const policyFile = "mcp-app.tsx";

export const policy = defineLint("coda-assistant-mcp-app", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("uv", ["run", "main.py", ...segments], { cwd: this.root, ...daemonBudgetOpts() });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["vite", "build", "--config", "vite.mcp-app.config.ts", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
