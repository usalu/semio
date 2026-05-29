#!/usr/bin/env bun
/** 🧭 `@puzzle/2d-react` task router: `bun ./script.ts test [args…]`. */
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

const cwd = import.meta.dir;
const wasmScript = join(cwd, "../rs/scripts/build-wasm.script.ts");
const segs = process.argv.slice(2);
const command = segs[0] ?? "test";
const extra = segs.slice(1);

const env = { ...process.env };
delete env.NODE_OPTIONS;
delete env.VSCODE_INSPECTOR_OPTIONS;

if (command === "test") {
	const wasmJs = join(cwd, "../rs/pkg/elements_board.js");
	const wasmEnv = { ...env, ELEMENTS_BOARD_SKIP_WASM_BUILD: existsSync(wasmJs) ? "1" : "0" };
	spawnSync("bun", [wasmScript], { cwd, env: wasmEnv, shell: true, stdio: "inherit" });
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
