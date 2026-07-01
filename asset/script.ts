#!/usr/bin/env bun
/** 🧭 `@semio-tech/semio-asset` router: `bun ./script.ts build`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../repo/lib/js/index.ts";

class BuildScript extends BundleScript {
	run(): void {
		console.log("@semio-tech/semio-asset is a source-only workspace and does not require a build step.");
	}
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
