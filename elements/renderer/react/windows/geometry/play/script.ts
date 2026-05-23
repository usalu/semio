#!/usr/bin/env bun
/** 🧭 Geometry play router: `bun ./script.ts <dev|build> [vite args…]` — builds the Topologic wasm bridge first, then runs the dedicated playground. */
import { spawn, spawnSync } from "node:child_process";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const command = segs[0];
const extra = segs.slice(1);
const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";

const env = {
	...process.env,
	...(process.env.WATCHPACK_POLLING !== undefined ? {} : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};
delete env.NODE_OPTIONS;
delete env.VSCODE_INSPECTOR_OPTIONS;

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
	if (result.status !== 0) {
		process.exit(result.status ?? 1);
	}
}

function ensureWasm(): void {
	runSync(["bun", "../script.ts", "wasm"]);
}

if (command === "dev") {
	ensureWasm();
	const child = spawn("bunx", ["vite", "--config", "vite.config.ts", "--host", host, ...extra], {
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
	
} else if (command === "build") {
	ensureWasm();
	runSync(["bunx", "vite", "build", "--config", "vite.config.ts", ...extra]);
	
} else {
	console.error("usage: bun ./script.ts <dev|build> [vite args…]");
	process.exit(1);
}
