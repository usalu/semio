#!/usr/bin/env bun
/** @emoji 🧭 `@framework/playground-react` task router — `bun ./script.ts test`. */
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const command = process.argv[2];
const here = dirname(fileURLToPath(import.meta.url));

if (command === "test") {
	const result = spawnSync("bun", ["x", "vitest", "run", "--config", join(here, "vitest.config.ts"), "--passWithNoTests"], {
		stdio: "inherit",
		cwd: here,
	});
	process.exit(result.status ?? 1);
}
console.error("usage: bun ./script.ts test");
process.exit(1);
