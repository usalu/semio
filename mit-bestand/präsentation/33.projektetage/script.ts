#!/usr/bin/env bun
/** 🧭 `@mit-bestand/praesentation/projektetage` task router: `bun ./script.ts <dev|build> [args…]`. */
import {
	BundleScript,
	ScriptRouter,
	playPollingEnv,
	runBun,
	runBundleScriptMain,
	runViteBunxDev,
} from "../../../repo/lib/js/src/index.ts";

class DevScript extends BundleScript {
	run(segments: string[]): void {
		runViteBunxDev(this.root, segments, {
			portEnv: "PRAESENTATION_PROJEKTETAGE_PORT",
			defaultPort: "6050",
		});
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("dev", DevScript)
	.register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
