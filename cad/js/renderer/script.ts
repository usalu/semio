#!/usr/bin/env bun
/** @emoji 🧭 `@cad/js-renderer-r3f` task router: `dev` | `build` | `test` | `policy`. */
import { spawn, spawnSync } from "node:child_process";
import type { FileLinter } from "../../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/src/cli.ts";
import { dispatchPolicyArgv } from "../../../repo/lib/js/src/policy-cli.ts";
import { defineLint } from "../../../repo/lib/js/src/script.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@cad/js-renderer-r3f-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
}
const command = segs[0];
const extra = segs.slice(1);

const env = {
	...process.env,
	...(process.env.WATCHPACK_POLLING !== undefined ? {} : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};
delete env.NODE_OPTIONS;
delete env.VSCODE_INSPECTOR_OPTIONS;

function run(args: string[]): void {
	const child = spawn("bun", args, {
		cwd,
		env,
		shell: true,
		stdio: "inherit",
	});
	child.on("exit", (code) => process.exit(code ?? 0));
	child.on("error", (error) => {
		console.error(error);
		process.exit(1);
	});
}

function runSync(args: string[]): void {
	const result = spawnSync(args[0], args.slice(1), {
		cwd,
		env,
		shell: true,
		stdio: "inherit",
	});
	if (result.error) {
		console.error(result.error);
		process.exit(1);
	}
	if (result.status !== 0) process.exit(result.status ?? 1);
}

if (command === "dev") {
	const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
	const port = process.env.SPATIAL_R3F_PLAY_PORT ?? "6020";
	run(["run", "vite", "--config", "play/vite.config.ts", "--host", host, "--port", port, ...extra]);
} else if (command === "build") {
	runSync(["bun", "run", "vite", "build", "--config", "play/vite.config.ts", ...extra]);
} else if (command === "test") {
	runSync(["bunx", "vitest", "run", "--config", "vitest.config.ts", ...extra]);
} else {
	console.error("usage: bun ./script.ts <dev|build|test> [args…]");
	process.exit(1);
}
