#!/usr/bin/env bun
/** @emoji 🏷️ One-shot: align package scopes with folder paths (folder = scope). */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose";

const replacements: [string, string][] = [
	["@semio-tech/framework-playground-core-renderer-react", "@semio-tech/framework-playground-renderer-react"],
	["@semio-tech/framework-playground-core-react", "@semio-tech/framework-playground-renderer-react"],
	["@semio-tech/framework-platform-core-react", "@semio-tech/framework-platform-renderer-react"],
	["@puzzle/2d-wasm", "@semio-tech/puzzle-2d-rs"],
	["@puzzle/board-wasm", "@semio-tech/puzzle-2d-rs"],
	["@puzzle/2d-play", "@semio-tech/puzzle-2d-play"],
	["@puzzle/2d-react", "@semio-tech/puzzle-2d-react"],
	["@puzzle/3d-play", "@semio-tech/puzzle-3d-play"],
	["@puzzle/3d-react", "@semio-tech/puzzle-3d-react"],
	["@puzzle/5d-play", "@semio-tech/puzzle-5d-play"],
	["@puzzle/5d-react", "@semio-tech/puzzle-5d-react"],
	["@puzzle/board", "@semio-tech/puzzle-2d-react"],
	["@puzzle/scene", "@semio-tech/puzzle-3d-react"],
	["@puzzle/topology", "@semio-tech/puzzle-5d-react"],
	["@semio-tech/cad-js-renderer-r3f", "@semio-tech/cad-js-renderer"],
	["@semio-tech/cad-js-kernel-brepjs", "@semio-tech/cad-js-kernel-brepjs"],
	["@semio-tech/cad-js-machine-stately", "@semio-tech/cad-js-machine-stately"],
	["@semio-tech/cad-js-core", "@semio-tech/cad-js-core"],
	["@semio-tech/cad-js-query", "@semio-tech/cad-js-query"],
	["@semio-tech/cad-js-workspace", "@semio-tech/cad-js"],
	["@spatial/js-renderer-r3f", "@semio-tech/cad-js-renderer"],
	["@spatial/js-kernel-brepjs", "@semio-tech/cad-js-kernel-brepjs"],
	["@spatial/js-machine-stately", "@semio-tech/cad-js-machine-stately"],
	["@spatial/js-core", "@semio-tech/cad-js-core"],
	["@spatial/js-query", "@semio-tech/cad-js-query"],
	["@spatial/js-workspace", "@semio-tech/cad-js"],
	["@elements/framework-react", "@semio-tech/framework-platform-renderer-react"],
	["@elements/framework", "@semio-tech/framework-platform-core"],
	["@elements/playground-react", "@semio-tech/framework-playground-renderer-react"],
	["@elements/playground", "@semio-tech/framework-playground-core"],
	["@elements/board-wasm", "@semio-tech/puzzle-2d-rs"],
	["@elements/board", "@semio-tech/puzzle-2d-react"],
	["@elements/scene", "@semio-tech/puzzle-3d-react"],
	["@elements/topology", "@semio-tech/puzzle-5d-react"],
	["@elements/styling-core", "@semio-tech/ui-styling-tokens"],
	["@elements/styling", "@semio-tech/ui-styling"],
	["@elements/ui", "@semio-tech/ui-react"],
	["@compose/architect-wasm", "@semio-tech/compose-query/pkg"],
	["@compose/architect", "@semio-tech/compose-query"],
];

const skipDir = new Set(["node_modules", "dist", "test-results", ".git", "target"]);
const exts = /\.(ts|tsx|json|mjs|cjs|go|md|sln)$/;

function walk(dir: string): string[] {
	const out: string[] = [];
	for (const name of readdirSync(dir)) {
		if (skipDir.has(name)) continue;
		const p = join(dir, name);
		try {
			const st = statSync(p);
			if (st.isDirectory()) {
				if (name === ".repo") {
					for (const sub of readdirSync(p)) {
						if (sub.startsWith("26")) continue;
						out.push(...walk(join(p, sub)));
					}
					continue;
				}
				out.push(...walk(p));
			} else if (exts.test(name) && !name.endsWith("bun.lock")) {
				out.push(p);
			}
		} catch {
			/* ignore */
		}
	}
	return out;
}

function applyReplacements(content: string): string {
	let c = content;
	for (const [from, to] of replacements) c = c.split(from).join(to);
	return c;
}

const roots = [
	"ui",
	"framework",
	"puzzle",
	"cad",
	"compose",
	"coda",
	"repo",
	".storybook",
	".vscode",
	"package.json",
	"script.ts",
	"project.json",
	"nx.json",
	"eslint.config.mjs",
	"Monorepo.sln",
	"tsconfig.json",
];

let changed = 0;
for (const rel of roots) {
	const base = join(root, rel);
	try {
		const st = statSync(base);
		const files = st.isDirectory() ? walk(base) : [base];
		for (const file of files) {
			const orig = readFileSync(file, "utf8");
			const next = applyReplacements(orig);
			if (next !== orig) {
				writeFileSync(file, next);
				changed++;
			}
		}
	} catch {
		/* missing */
	}
}

console.log(`[rename-path-scopes] updated ${changed} files`);
