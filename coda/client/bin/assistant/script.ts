#!/usr/bin/env bun
/** 🧭 `@semio-tech/coda-assistant` router: `bun ./script.ts <dev|build|policy>`. */
import { execFileSync } from "node:child_process";
import type { FileLinter } from "../../../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/src/index.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/src/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx } from "../../../../repo/lib/js/src/index.ts";
import { defineLint } from "../../../../repo/lib/js/src/index.ts";

export const policyFile = "mcp-app.tsx";

export const policy = defineLint("coda-assistant-mcp-app", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class DevScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("uv", ["run", "main.py", ...segments], { cwd: this.root, stdio: "inherit" });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["vite", "build", "--config", "vite.mcp-app.config.ts", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
