#!/usr/bin/env bun
/** 🧭 Geometry package task router: `bun ./script.ts <dev|build|test|wasm> [args…]` — builds the Topologic wasm bridge, runs the R3F play harness, and executes focused Vitest checks. */
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, statSync } from "node:fs";
import { join } from "node:path";

const cwd = import.meta.dir;
const workspaceRoot = join(cwd, "../../../../");
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

function shellQuote(value: string): string {
	return `"${value.replaceAll('"', '\\"')}"`;
}

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

function runShell(commandLine: string, options: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
	const result = spawnSync(commandLine, {
		cwd: options.cwd ?? cwd,
		env: options.env ?? env,
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

function hasTool(commandName: string): boolean {
	const probe = spawnSync(process.platform === "win32" ? "where" : "which", [commandName], {
		cwd,
		env,
		shell: true,
		stdio: "ignore",
	});
	return probe.status === 0;
}

function emsdkRoot(): string {
	return join(workspaceRoot, ".repo", "cache", "emsdk");
}

function emsdkCommand(): string {
	return process.platform === "win32" ? join(emsdkRoot(), "emsdk.bat") : join(emsdkRoot(), "emsdk");
}

function emppCommand(): string {
	return process.platform === "win32"
		? join(emsdkRoot(), "upstream", "emscripten", "em++.bat")
		: join(emsdkRoot(), "upstream", "emscripten", "em++");
}

function emcmakeCommand(): string {
	return process.platform === "win32"
		? join(emsdkRoot(), "upstream", "emscripten", "emcmake.bat")
		: join(emsdkRoot(), "upstream", "emscripten", "emcmake");
}

function ensureEmscripten(): string {
	if (hasTool("em++")) return "em++";
	const root = emsdkRoot();
	if (!existsSync(root)) {
		runShell(`git clone --depth 1 https://github.com/emscripten-core/emsdk.git ${shellQuote(root)}`, { cwd: workspaceRoot });
	}
	const emsdk = emsdkCommand();
	const empp = emppCommand();
	if (!existsSync(empp)) {
		if (process.platform === "win32") {
			runShell(`call ${shellQuote(emsdk)} install latest && call ${shellQuote(emsdk)} activate latest`, { cwd: root });
		} else {
			runShell(`${shellQuote(emsdk)} install latest && ${shellQuote(emsdk)} activate latest`, { cwd: root });
		}
	}
	return empp;
}

function wasmOutputPath(): string {
	return join(cwd, "wasm", "generated", "topologic-kernel.js");
}

function wasmSourcePath(): string {
	return join(cwd, "wasm", "topologic-kernel.cpp");
}

function wasmNeedsBuild(): boolean {
	return true;
}

function buildWasm(force = false): void {
	if (!force && !wasmNeedsBuild()) return;
	ensureEmscripten();
	const outDir = join(cwd, "wasm", "generated");
	mkdirSync(outDir, { recursive: true });
	const buildDir = join(workspaceRoot, ".repo", "cache", "elements-geometry-topologic-wasm");
	mkdirSync(buildDir, { recursive: true });
	const cacheDir = join(workspaceRoot, ".repo", "cache", "emscripten-cache");
	mkdirSync(cacheDir, { recursive: true });
	const configureLine = [
		shellQuote(emcmakeCommand()),
		"cmake",
		"-S",
		shellQuote(join(cwd, "topologic")),
		"-B",
		shellQuote(buildDir),
		"-DCMAKE_BUILD_TYPE=Release",
		"-DTOPOLOGICCORE_BUILD_SHARED=OFF",
		"-DTOPOLOGIC_BUILD_PYTHON_BINDINGS=OFF",
		"-DTOPOLOGIC_BUILD_WASM_BRIDGE=ON",
	].join(" ");
	runShell(configureLine, { cwd, env: { ...env, EM_CACHE: cacheDir } });
	runShell(`cmake --build ${shellQuote(buildDir)} --config Release --target TopologicWasmKernel`, {
		cwd,
		env: { ...env, EM_CACHE: cacheDir },
	});
}

if (command === "wasm") {
	buildWasm(true);
} else if (command === "dev") {
	buildWasm();
	const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
	const port = process.env.GEOMETRY_PLAY_PORT ?? "6016";
	run(["run", "vite", "--config", "play/vite.config.ts", "--host", host, "--port", port, ...extra]);
} else if (command === "build") {
	buildWasm();
	runSync(["bun", "run", "vite", "build", "--config", "play/vite.config.ts", ...extra]);
} else if (command === "test") {
	buildWasm();
	runSync(["bunx", "vitest", "run", "--config", "vitest.config.ts", ...extra]);
} else {
	console.error("usage: bun ./script.ts <dev|build|test|wasm> [args…]");
	process.exit(1);
}