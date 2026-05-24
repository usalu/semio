#!/usr/bin/env bun
/** @emoji 🧭 `@elements/spatial-react` task router — `bun ./script.ts test [vitest args…]`. */
import { execFileSync } from "node:child_process";

const command = process.argv[2];
const args = process.argv.slice(3);

if (command === "test") {
	execFileSync("bunx", ["vitest", "run", "--config", "vitest.config.ts", ...args], {
		cwd: import.meta.dir,
		stdio: "inherit",
		env: process.env,
	});
} else {
	console.error("usage: bun ./script.ts test [vitest args…]");
	process.exit(1);
}
