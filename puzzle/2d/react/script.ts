#!/usr/bin/env bun
/** 🧭 `@puzzle/2d-react` task router: `bun ./script.ts test [args…]`. */
import { spawnSync } from "node:child_process";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const command = segs[0] ?? "test";
const extra = segs.slice(1);

const env = { ...process.env };
delete env.NODE_OPTIONS;
delete env.VSCODE_INSPECTOR_OPTIONS;

if (command === "test") {
	const result = spawnSync("bunx", ["vitest", "run", "--passWithNoTests", "--config", "vitest.config.ts", ...extra], {
		cwd,
		env,
		shell: true,
		stdio: "inherit",
	});
	process.exit(result.status ?? 1);
}

console.error("usage: bun ./script.ts test [args…]");
process.exit(1);
