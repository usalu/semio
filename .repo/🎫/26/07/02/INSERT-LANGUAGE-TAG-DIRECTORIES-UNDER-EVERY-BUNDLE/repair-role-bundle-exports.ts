#!/usr/bin/env bun
/** 🔧 Restore ./js/ exports for role bundles with sources under js/. */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";

function findRepoRoot(start: string): string {
	let dir = start;
	while (dir !== dirname(dir)) {
		if (existsSync(join(dir, "package.json"))) {
			const pkg = JSON.parse(readFileSync(join(dir, "package.json"), "utf8")) as { name?: string };
			if (pkg.name === "compose") return dir;
		}
		dir = dirname(dir);
	}
	throw new Error("repo root not found");
}

const REPO = findRepoRoot(import.meta.dir);
const LANGUAGE_TAGS = new Set(["js", "rs", "py", "go", "cs", "ts"]);
const FRAMEWORK_TAGS = new Set(["react", "r3f", "react-renderer"]);
const SKIP = new Set(["node_modules", "target", "dist", ".git", ".repo", ".cursor", "pkg"]);

function rel(p: string): string {
	return relative(REPO, p).replace(/\\/g, "/");
}

function isBundle(dir: string): boolean {
	return existsSync(join(dir, "package.json")) && existsSync(join(dir, "project.json"));
}

function walkBundles(dir: string, out: string[] = []): string[] {
	for (const name of readdirSync(dir)) {
		if (SKIP.has(name)) continue;
		const p = join(dir, name);
		if (!statSync(p).isDirectory()) continue;
		if (isBundle(p)) out.push(p);
		walkBundles(p, out);
	}
	return out;
}

const touched = new Set<string>();

for (const bundle of walkBundles(REPO)) {
	const tag = basename(bundle);
	if (LANGUAGE_TAGS.has(tag) || FRAMEWORK_TAGS.has(tag)) continue;
	const jsDir = join(bundle, "js");
	if (!existsSync(jsDir) || !statSync(jsDir).isDirectory()) continue;

	const pkgPath = join(bundle, "package.json");
	const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as { exports?: Record<string, string> };
	if (!pkg.exports) continue;

	let changed = false;
	const nextExports: Record<string, string> = {};
	for (const [key, value] of Object.entries(pkg.exports)) {
		if (typeof value !== "string" || !value.startsWith("./") || value.startsWith("./js/")) {
			nextExports[key] = value;
			continue;
		}
		const file = value.slice(2);
		const jsPath = join(jsDir, file);
		if (existsSync(jsPath)) {
			nextExports[key] = `./js/${file}`;
			changed = true;
		} else {
			nextExports[key] = value;
		}
	}
	if (changed) {
		pkg.exports = nextExports;
		writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
		touched.add(rel(pkgPath));
	}
}

console.log(`[DEBUG] repaired ${touched.size} package.json files`);
for (const f of [...touched].sort()) console.log(`  ${f}`);
