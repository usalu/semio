#!/usr/bin/env bun
/** 🧭 `@semio-tech/repo-lib` router: `bun ./script.ts <lint|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx, runCmd } from "./index.ts";

class LintScript extends BundleScript {
	run(): void {
		runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
	}
}

class TestScript extends BundleScript {
	run(): void {
		runCmd(process.execPath, ["test", "./index.test.ts"], { cwd: this.root });
	}
}

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
