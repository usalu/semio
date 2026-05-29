#!/usr/bin/env bun
/** ­ƒº¡ Scene package task router: `bun ./script.ts <dev|build|test> [argsÔÇª]` ÔÇö `test` runs Vitest then Playwright against the play harness. */
import { spawn, spawnSync } from "node:child_process";
import { existsSync, rmSync } from "node:fs";
import path from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const command = segs[0];
const extra = segs.slice(1);

const env = {
	...process.env,
	...(process.env.WATCHPACK_POLLING !== undefined
		? {}
		: { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};
delete env.NODE_OPTIONS;
delete env.VSCODE_INSPECTOR_OPTIONS;

function run(args: string[], options: { cwd?: string } = {}): void {
	const child = spawn("bun", args, {
		cwd: options.cwd ?? cwd,
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

function runSync(args: string[], options: { cwd?: string } = {}): void {
	const result = spawnSync(args[0], args.slice(1), {
		cwd: options.cwd ?? cwd,
		env,
		shell: true,
		stdio: "inherit",
	});
	if (result.error) {
		console.error(result.error);
		process.exit(1);
	}
	if (result.status !== 0) {
		process.exit(result.status ?? 1);
	}
}

if (command === "dev") {
	const viteCache = path.join(cwd, "node_modules", ".vite");
	if (existsSync(viteCache)) {
		rmSync(viteCache, { recursive: true, force: true });
	}
	const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
	const port = process.env.SCENE_PLAY_PORT ?? "6013";
	run(["run", "vite", "--config", "play/vite.config.ts", "--host", host, "--port", port, ...extra]);
} else if (command === "build") {
	runSync(["bun", "run", "vite", "build", "--config", "play/vite.config.ts", ...extra]);
} else if (command === "test") {
	runSync(["bunx", "vitest", "run", "--config", "vitest.config.ts", ...extra]);
	runSync(["bunx", "playwright", "test", "--config", "play/playwright.config.ts", ...extra], { cwd });
} else {
	console.error("usage: bun ./script.ts <dev|build|test> [argsÔÇª]");
	process.exit(1);
}
