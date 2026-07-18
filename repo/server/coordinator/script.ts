#!/usr/bin/env bun
/** 🧭 Coordinator package router: `bun ./script.ts build|test|policy`. */
import { execFileSync } from "node:child_process";
import type { BundleLinter } from "../../lib/js/index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../lib/js/index.ts";
import { getWorkspaceRoot } from "../../lib/js/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../lib/js/index.ts";
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

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
