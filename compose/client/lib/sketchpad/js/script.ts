#!/usr/bin/env bun
/** 🧭 `@semio-tech/compose-sketchpad-js` router: `bun ./script.ts dev|test|policy [args…]`. */
import { existsSync, rmSync } from "node:fs";
import { join } from "node:path";
import type { FileLinter } from "../../../../../repo/lib/js/index.ts";
import {
	BundleScript,
	ScriptRouter,
	defineLint,
	dependencyBoundaryBreachesForFile,
	getWorkspaceRoot,
	runBundleScriptMain,
	runPolicyOnlyMain,
	runVitest,
	runViteBunxDevPlain,
} from "../../../../../repo/lib/js/index.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@semio-tech/compose-sketchpad-js-index", (l: FileLinter) => {
	const repoRoot = getWorkspaceRoot();
	const file = l.path();
	return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class DevScript extends BundleScript {
	run(segments: string[]): void {
		const viteCache = join(this.root, "node_modules", ".vite");
		if (existsSync(viteCache)) rmSync(viteCache, { recursive: true, force: true });
		const args = segments.includes("--force") ? segments : ["--force", ...segments];
		runViteBunxDevPlain(this.root, args);
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("test", TestScript);

if (import.meta.main) {
	const cmd = process.argv[2];
	if (cmd === "policy") {
		await runPolicyOnlyMain(import.meta.url);
	} else {
		await runBundleScriptMain(router, import.meta.url);
	}
}
