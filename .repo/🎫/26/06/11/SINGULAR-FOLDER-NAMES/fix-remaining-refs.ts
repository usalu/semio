#!/usr/bin/env bun
/** @emoji 📁 Second-pass reference fixes after singular folder rename. */
import { existsSync, readFileSync, readdirSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", ".git", ".repo", ".venv", "temp", ".nx", "dist", "target"]);
const EXT = new Set([".ts", ".tsx", ".js", ".jsx", ".json", ".md", ".mdx", ".go", ".py", ".cs", ".rs", ".toml", ".yaml", ".yml"]);

const REPLACEMENTS: [string, string][] = [
	["@semio/algorithms", "@semio/algorithm"],
	["@semio/icons", "@semio/icon"],
	["semio/dev/algorithms", "semio/dev/algorithm"],
	["semio/asset/icons", "semio/asset/icon"],
	["../../../../assets", "../../../../asset"],
	["../../../assets", "../../../asset"],
	["../../assets", "../../asset"],
	["../assets", "../asset"],
	["../../../../fixtures", "../../../../fixture"],
	["../../../fixtures", "../../../fixture"],
	["../../fixtures", "../../fixture"],
	["../fixtures", "../fixture"],
	["sketchpad/docs", "sketchpad/doc"],
	["/assets\"", "/asset\""],
	["/fixtures\"", "/fixture\""],
	["/assets'", "/asset'"],
	["/fixtures'", "/fixture'"],
	["path: \"/assets\"", "path: \"/asset\""],
	["mit-bestand/recherche/standards", "mit-bestand/recherche/standard"],
];

function walk(dir: string, out: string[]): void {
	if ([...SKIP].some((s) => dir.includes(`/${s}/`) || dir.endsWith(`/${s}`))) return;
	for (const entry of readdirSync(dir)) {
		if (SKIP.has(entry)) continue;
		const full = join(dir, entry);
		let st;
		try {
			st = statSync(full);
		} catch {
			continue;
		}
		if (st.isDirectory()) {
			walk(full, out);
			continue;
		}
		const ext = entry.includes(".") ? entry.slice(entry.lastIndexOf(".")) : "";
		if (EXT.has(ext)) out.push(full);
	}
}

const files: string[] = [];
walk(root, files);
let changed = 0;
for (const file of files) {
	const original = readFileSync(file, "utf8");
	let next = original;
	for (const [from, to] of REPLACEMENTS) next = next.split(from).join(to);
	if (next !== original) {
		writeFileSync(file, next);
		changed++;
	}
}

if (existsSync(join(root, "mit-bestand/recherche/standards"))) {
	renameSync(join(root, "mit-bestand/recherche/standards"), join(root, "mit-bestand/recherche/standard"));
	console.log("[DEBUG] renamed mit-bestand/recherche/standards -> standard");
}

console.log(`[DEBUG] second pass updated ${changed} files`);
