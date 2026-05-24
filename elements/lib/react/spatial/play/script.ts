#!/usr/bin/env bun
/** @emoji 🧭 `@elements/spatial-play` task router — `bun ./script.ts <dev|build|test> [args…]`. */
import { execFileSync, spawn } from "node:child_process";

const cwd = import.meta.dir;
const command = process.argv[2];
const args = process.argv.slice(3);
const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const env = {
	...process.env,
	CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "1",
};

if (command === "dev") {
	const child = spawn("bunx", ["vite", "--config", "vite.config.ts", "--host", host, ...args], {
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
	execFileSync("bunx", ["vite", "build", "--config", "vite.config.ts", ...args], {
		cwd,
		stdio: "inherit",
		env,
	});
} else if (command === "test") {
	execFileSync("bunx", ["vitest", "run", "--config", "vitest.config.ts", ...args], {
		cwd,
		stdio: "inherit",
		env,
	});
} else {
	console.error("usage: bun ./script.ts <dev|build|test> [args…]");
	process.exit(1);
}
