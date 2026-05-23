#!/usr/bin/env bun
/** 🧭 `@semio/sketchpad-js` router: `bun ./script.ts <policy|test> [args…]`. */
import type { FileLinter } from "../../../../../repo/lib/js/src/index.ts";
import {
	BundleScript,
	ScriptRouter,
	defineLint,
	dependencyBoundaryBreachesForFile,
	getWorkspaceRoot,
	runBundleScriptMain,
	runPolicyOnlyMain,
	runVitest,
} from "../../../../../repo/lib/js/src/index.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@semio/sketchpad-js-index", (l: FileLinter) => {
	const repoRoot = getWorkspaceRoot();
	const file = l.path();
	return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

if (import.meta.main) {
	const cmd = process.argv[2];
	if (cmd === "policy") {
		await runPolicyOnlyMain(import.meta.url);
	} else {
		await runBundleScriptMain(router, import.meta.url);
	}
}
