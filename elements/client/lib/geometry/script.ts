#!/usr/bin/env bun
/** 🧭 Geometry package task router: `bun ./script.ts <dev|build|test|wasm> [args…]` — builds the Topologic wasm bridge, runs the R3F play harness, and executes focused Vitest checks. */
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { homedir } from "node:os";
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
	if (result.error && result.status == null) {
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
	if (result.error && result.status == null) {
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

function vcpkgRoot(): string {
	return process.env.VCPKG_ROOT ?? join(workspaceRoot, ".repo", "cache", "vcpkg");
}

function emscriptenToolchainFile(): string {
	return join(emsdkRoot(), "upstream", "emscripten", "cmake", "Modules", "Platform", "Emscripten.cmake");
}

function ensureVcpkg(): void {
	const root = vcpkgRoot();
	const vcpkgExe = join(root, process.platform === "win32" ? "vcpkg.exe" : "vcpkg");
	if (!existsSync(root)) {
		mkdirSync(join(workspaceRoot, ".repo", "cache"), { recursive: true });
		runShell(`git clone --depth 1 https://github.com/microsoft/vcpkg.git ${shellQuote(root)}`, { cwd: workspaceRoot });
	}
	if (!existsSync(vcpkgExe)) {
		if (process.platform === "win32") {
			runShell(`cmd.exe /c ${shellQuote(join(root, "bootstrap-vcpkg.bat"))} -disableMetrics`, { cwd: root });
		} else {
			runShell(`bash ${shellQuote(join(root, "bootstrap-vcpkg.sh"))} -disableMetrics`, { cwd: root });
		}
	}
}

function wasmBuildEnv(): NodeJS.ProcessEnv {
	const emsdk = emsdkRoot();
	const emscripten = join(emsdk, "upstream", "emscripten");
	const upstreamBin = join(emsdk, "upstream", "bin");
	const separator = process.platform === "win32" ? ";" : ":";
	const pathPrefix = [join(homedir(), ".local", "bin"), emscripten, upstreamBin]
		.filter((entry) => existsSync(entry))
		.join(separator);
	return {
		...env,
		EMSDK: emsdk,
		EM_CACHE: join(workspaceRoot, ".repo", "cache", "emscripten-cache"),
		VCPKG_ROOT: vcpkgRoot(),
		VCPKG_DISABLE_METRICS: "1",
		VCPKG_MAX_CONCURRENCY: process.env.VCPKG_MAX_CONCURRENCY ?? "2",
		PATH: pathPrefix ? `${pathPrefix}${separator}${env.PATH ?? ""}` : env.PATH,
	};
}

function purgeStaleWasmCmakeCache(buildDir: string): void {
	const cacheFile = join(buildDir, "CMakeCache.txt");
	if (!existsSync(cacheFile)) return;
	const content = readFileSync(cacheFile, "utf8");
	if (
		content.includes("OpenCASCADE_DIR-NOTFOUND") ||
		!content.includes("wasm32-emscripten") ||
		content.includes("CMAKE_GENERATOR:INTERNAL=MinGW Makefiles")
	) {
		rmSync(buildDir, { recursive: true, force: true });
	}
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
	ensureVcpkg();
	const outDir = join(cwd, "wasm", "generated");
	mkdirSync(outDir, { recursive: true });
	const buildDir = join(workspaceRoot, ".repo", "cache", "elements-geometry-topologic-wasm");
	purgeStaleWasmCmakeCache(buildDir);
	mkdirSync(buildDir, { recursive: true });
	const topologicDir = join(cwd, "topologic");
	const wasmEnv = wasmBuildEnv();
	const vcpkgCmake = join(vcpkgRoot(), "scripts", "buildsystems", "vcpkg.cmake");
	const configureLine = [
		"cmake",
		"-G",
		"Ninja",
		"-S",
		shellQuote(topologicDir),
		"-B",
		shellQuote(buildDir),
		"-DCMAKE_BUILD_TYPE=Release",
		`-DCMAKE_TOOLCHAIN_FILE=${shellQuote(vcpkgCmake)}`,
		`-DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=${shellQuote(emscriptenToolchainFile())}`,
		"-DVCPKG_TARGET_TRIPLET=wasm32-emscripten",
		`-DVCPKG_MANIFEST_DIR=${shellQuote(topologicDir)}`,
		`-DVCPKG_OVERLAY_TRIPLETS=${shellQuote(join(topologicDir, "triplets"))}`,
		"-DVCPKG_FEATURE_FLAGS=manifests",
		"-DTOPOLOGICCORE_BUILD_SHARED=OFF",
		"-DTOPOLOGIC_BUILD_PYTHON_BINDINGS=OFF",
		"-DTOPOLOGIC_BUILD_WASM_BRIDGE=ON",
	].join(" ");
	runShell(configureLine, { cwd, env: wasmEnv });
	runShell(`cmake --build ${shellQuote(buildDir)} --target TopologicWasmKernel`, { cwd, env: wasmEnv });
	if (!existsSync(wasmOutputPath())) {
		console.error(`[wasm] expected output missing: ${wasmOutputPath()}`);
		process.exit(1);
	}
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