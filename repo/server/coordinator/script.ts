#!/usr/bin/env bun
/** 🧭 Coordinator package router: `bun ./script.ts build|policy`. */
import { execFileSync } from "node:child_process";
import type { BundleLinter } from "../../lib/js/index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../lib/js/index.ts";
import { getWorkspaceRoot } from "../../lib/js/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../lib/js/index.ts";
import { defineLint } from "../../lib/js/index.ts";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

class BuildScript extends BundleScript {
  run(): void {
    const ext = process.platform === "win32" ? ".exe" : "";
    execFileSync("go", ["build", "-o", `server${ext}`, "."], { cwd: this.root, stdio: "inherit" });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
