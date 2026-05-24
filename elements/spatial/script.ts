#!/usr/bin/env bun
/** @emoji 🧭 Spatial package task router: `bun ./script.ts <js|react|play> <command> [...args]`. */
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";

const spatialRoot = import.meta.dir;
const [surface, command, ...args] = process.argv.slice(2);

function run(commandName: string, commandArgs: string[], cwd: string): void {
	execFileSync(commandName, commandArgs, {
		cwd,
		stdio: "inherit",
		env: process.env,
	});
}

function runVitest(surfaceRoot: string, include: string[]): void {
	run("bunx", ["vitest", "run", "--config", "../vitest.config.ts", ...include, ...args], resolve(spatialRoot, surfaceRoot));
}

switch (`${surface}:${command}`) {
	case "js:test": {
		runVitest("js", ["js/index.ts"]);
		break;
	}
	case "react:test": {
		runVitest("react", ["react/index.tsx"]);
		break;
	}
	case "play:dev": {
		run("bunx", ["vite", "--config", "vite.config.ts", ...args], resolve(spatialRoot, "play"));
		break;
	}
	case "play:build": {
		run("bunx", ["vite", "build", "--config", "vite.config.ts", ...args], resolve(spatialRoot, "play"));
		break;
	}
	case "play:test": {
		runVitest("play", ["play/index.ts"]);
		break;
	}
	default: {
		console.error("usage: bun ./script.ts <js|react|play> <test|dev|build> [...args]");
		process.exit(1);
	}
}