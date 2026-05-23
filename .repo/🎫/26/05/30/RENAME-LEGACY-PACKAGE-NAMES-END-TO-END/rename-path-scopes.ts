#!/usr/bin/env bun
/** @emoji 🏷️ One-shot: align package scopes with folder paths (folder = scope). */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio";

const replacements: [string, string][] = [
	["@framework/playground-renderer-react", "@framework/playground/renderer/react"],
	["@framework/playground-react", "@framework/playground/renderer/react"],
	["@framework/platform-react", "@framework/platform/renderer/react"],
	["@puzzle/2d-wasm", "@puzzle/2d/rs"],
	["@puzzle/board-wasm", "@puzzle/2d/rs"],
	["@puzzle/2d-play", "@puzzle/2d/play"],
	["@puzzle/2d-react", "@puzzle/2d/react"],
	["@puzzle/3d-play", "@puzzle/3d/play"],
	["@puzzle/3d-react", "@puzzle/3d/react"],
	["@puzzle/5d-play", "@puzzle/5d/play"],
	["@puzzle/5d-react", "@puzzle/5d/react"],
	["@puzzle/board", "@puzzle/2d/react"],
	["@puzzle/scene", "@puzzle/3d/react"],
	["@puzzle/topology", "@puzzle/5d/react"],
	["@cad/js-renderer-r3f", "@cad/js/renderer"],
	["@cad/js-kernel-brepjs", "@cad/js/kernel/brepjs"],
	["@cad/js-machine-stately", "@cad/js/machine/stately"],
	["@cad/js-core", "@cad/js/core"],
	["@cad/js-query", "@cad/js/query"],
	["@cad/js-workspace", "@cad/js"],
	["@spatial/js-renderer-r3f", "@cad/js/renderer"],
	["@spatial/js-kernel-brepjs", "@cad/js/kernel/brepjs"],
	["@spatial/js-machine-stately", "@cad/js/machine/stately"],
	["@spatial/js-core", "@cad/js/core"],
	["@spatial/js-query", "@cad/js/query"],
	["@spatial/js-workspace", "@cad/js"],
	["@elements/framework-react", "@framework/platform/renderer/react"],
	["@elements/framework", "@framework/platform"],
	["@elements/playground-react", "@framework/playground/renderer/react"],
	["@elements/playground", "@framework/playground"],
	["@elements/board-wasm", "@puzzle/2d/rs"],
	["@elements/board", "@puzzle/2d/react"],
	["@elements/scene", "@puzzle/3d/react"],
	["@elements/topology", "@puzzle/5d/react"],
	["@elements/styling-core", "@ui/styling-tokens"],
	["@elements/styling", "@ui/styling"],
	["@elements/ui", "@ui/react"],
	["@semio/architect-wasm", "@semio/query/pkg"],
	["@semio/architect", "@semio/query"],
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
	"semio",
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
