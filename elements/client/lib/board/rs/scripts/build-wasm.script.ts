#!/usr/bin/env bun
/** 🦀➡️🌐 Builds `elements_board` WASM via wasm-pack into `rs/pkg/` for the thin JS bridge. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rsDir = resolve(__dirname, "..");
const pkgDir = join(rsDir, "pkg");
const pkgJsonPath = join(pkgDir, "package.json");

if (process.env.ELEMENTS_BOARD_SKIP_WASM_BUILD === "1") {
	console.log("[elements/board/rs] ELEMENTS_BOARD_SKIP_WASM_BUILD=1 → skipping wasm-pack build");
	process.exit(0);
}

{
	console.log("[elements/board/rs] wasm-pack build --release --target web --out-dir pkg --no-pack");
	const t0 = Date.now();
	const res = spawnSync(
		"bun",
		["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"],
		{ cwd: rsDir, stdio: "inherit" },
	);
	if (res.status !== 0) {
		console.error("[elements/board/rs] wasm-pack build failed");
		process.exit(res.status ?? 1);
	}
	console.log(`[elements/board/rs] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);
}

if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
const pkgJson = {
	name: "@elements/board-wasm",
	type: "module",
	version: "0.1.0",
	files: ["elements_board_bg.wasm", "elements_board.js", "elements_board.d.ts", "elements_board_bg.wasm.d.ts"],
	main: "elements_board.js",
	module: "elements_board.js",
	types: "elements_board.d.ts",
	sideEffects: ["./snippets/*"],
};
writeFileSync(pkgJsonPath, `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

const wasmPath = join(pkgDir, "elements_board_bg.wasm");
if (existsSync(wasmPath)) {
	const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
	console.log(`[elements/board/rs] pkg/elements_board_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
} else {
	console.error(`[elements/board/rs] expected wasm output missing: ${wasmPath}`);
	process.exit(1);
}
