#!/usr/bin/env bun
/** 🧭 `@semio-tech/framework-playground-core` task router: `bun ./script.ts test`, `bun ./script.ts audit`. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../repo/lib/js/index.ts";

const auditTicketDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../.repo/🎫/26/07/01/PLAYGROUND-WINDOW-MODE-COMPLETENESS-PASS");

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

class AuditScript extends BundleScript {
	run(): void {
		runVitest(auditTicketDir, [], "vitest-audit.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("audit", AuditScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
