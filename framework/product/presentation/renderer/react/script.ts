#!/usr/bin/env bun
/** 🧭 `@framework/presentation/renderer/react` task router: `bun ./script.ts test|policy`. */
import type { BundleLinter } from "../../../../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForBundleDir, getWorkspaceRoot } from "../../../../../repo/lib/js/src/index.ts";
import { BundleScript, ScriptRouter, devToolingEnv, runBunx, runBundleScriptMain } from "../../../../../repo/lib/js/src/index.ts";
import { defineLint } from "../../../../../repo/lib/js/src/index.ts";

export const policy = defineLint("@framework/presentation/renderer/react-bundle", (l: BundleLinter) => {
	const repoRoot = getWorkspaceRoot();
	return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBunx(["vitest", "run", "--config", "vitest.config.ts", ...segments], this.root, devToolingEnv());
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
