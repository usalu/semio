#!/usr/bin/env bun
/** 🧭 `@semio/sketchpad` router: `bun ./script.ts <policy|wasm|dev|build|test|e2e|setup> [args…]`. */
import { join } from "node:path";
import type { FileLinter } from "../../../../../repo/lib/js/src/index.ts";
import {
	BundleScript,
	ScriptRouter,
	defineLint,
	dependencyBoundaryBreachesForFile,
	getWorkspaceRoot,
	runBun,
	runBunx,
	runBundleScriptMain,
	runPolicyOnlyMain,
	runViteBuild,
	runViteBunxDevPlain,
	runVitest,
} from "../../../../../repo/lib/js/src/index.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@semio/sketchpad-js-index", (l: FileLinter) => {
	const repoRoot = getWorkspaceRoot();
	const file = l.path();
	return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

/** @emoji 🦀 Builds the puzzle 2d wasm package and the generated ui asset registry the sketchpad bundles. */
class WasmScript extends BundleScript {
	run(): void {
		runBun([join(this.repoRoot, "puzzle/2d/rs/script.ts"), "wasm"], this.root);
		runBun([join(this.repoRoot, "ui/assets/script.ts"), "generate", "all"], join(this.repoRoot, "ui/assets"));
	}
}

class DevScript extends BundleScript {
	run(segments: string[]): void {
		new WasmScript(this.root, this.repoRoot).run();
		runViteBunxDevPlain(this.root, segments);
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		new WasmScript(this.root, this.repoRoot).run();
		runViteBuild(this.root, segments, "vite.config.ts");
	}
}

/** @emoji 🎭 Embedded Playwright E2E region against the sketchpad dev server. */
class E2eScript extends BundleScript {
	run(segments: string[]): void {
		runBun(["--import", "./pw-loader.mjs", join(this.repoRoot, "node_modules/playwright/cli.js"), "test", ...segments], this.root, {
			...process.env,
			SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS: "1",
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		new WasmScript(this.root, this.repoRoot).run();
		runVitest(this.root, segments);
		new E2eScript(this.root, this.repoRoot).run([]);
	}
}

class SetupScript extends BundleScript {
	run(): void {
		runBunx(["playwright", "install", "--with-deps", "chromium"], this.root);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("wasm", WasmScript)
	.register("dev", DevScript)
	.register("build", BuildScript)
	.register("test", TestScript)
	.register("e2e", E2eScript)
	.register("setup", SetupScript);

if (import.meta.main) {
	const cmd = process.argv[2];
	if (cmd === "policy") {
		await runPolicyOnlyMain(import.meta.url);
	} else {
		await runBundleScriptMain(router, import.meta.url);
	}
}
