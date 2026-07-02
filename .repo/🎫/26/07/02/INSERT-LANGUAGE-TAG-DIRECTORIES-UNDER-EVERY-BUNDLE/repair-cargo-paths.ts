#!/usr/bin/env bun
/** 🔧 Validate and repair every Cargo.toml path dependency after language-tag migration. */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "target", "dist", ".git", ".cursor", ".repo"]);

function walk(d: string, out: string[] = []): string[] {
	for (const n of readdirSync(d)) {
		if (SKIP.has(n)) continue;
		const p = join(d, n);
		if (statSync(p).isDirectory()) walk(p, out);
		else if (n === "Cargo.toml") out.push(p);
	}
	return out;
}

function findCrate(startAbs: string): string | null {
	let dir = startAbs;
	for (let i = 0; i < 6; i++) {
		if (existsSync(join(dir, "Cargo.toml"))) return dir;
		const parent = dirname(dir);
		if (parent === dir) break;
		dir = parent;
	}
	return null;
}

function repairPath(fromFile: string, rel: string): string {
	const fromDir = dirname(fromFile);
	const abs = normalize(join(fromDir, rel));
	if (existsSync(join(abs, "Cargo.toml"))) return rel.replace(/\\/g, "/");

	const crateNameGuess = rel.split("/").pop()?.replace(/-/g, "_");
	const candidates: string[] = [];
	const parts = rel.split("/").filter(Boolean);
	for (let i = 0; i < parts.length; i++) {
		const sub = parts.slice(i).join("/");
		candidates.push(sub, `${sub}/rs`, `../${sub}`, `../${sub}/rs`);
	}
	candidates.push(`${rel}/rs`, rel.replace(/\/rs\/?$/, ""));

	const relFromRepo = relative(REPO, fromDir).replace(/\\/g, "/");
	const segments = relFromRepo.split("/");
	for (let up = 0; up <= segments.length; up++) {
		const prefix = "../".repeat(up);
		for (const tail of ["neural/engine/rs", "neural/dag/rs", "mathematical/core/rs", "mathematical/graph/manifest/rs", "mathematical/graph/dsl/rs", "ui/styling/rs", "infinite/cavas/rs", "vcs/rs", "reasoning/mindmap/rs"]) {
			candidates.push(`${prefix}${tail}`);
		}
	}

	for (const c of [...new Set(candidates)]) {
		const a = normalize(join(fromDir, c));
		if (existsSync(join(a, "Cargo.toml"))) {
			return c.replace(/\\/g, "/");
		}
	}

	const found = findCrate(abs);
	if (found) {
		const fixed = relative(fromDir, found).replace(/\\/g, "/");
		return fixed.startsWith(".") ? fixed : `./${fixed}`;
	}

	if (crateNameGuess) {
		for (const file of walk(REPO)) {
			if (file === fromFile) continue;
			const txt = readFileSync(file, "utf8");
			const m = txt.match(/^name\s*=\s*"([^"]+)"/m);
			if (m?.[1] === crateNameGuess) {
				const fixed = relative(fromDir, dirname(file)).replace(/\\/g, "/");
				return fixed.startsWith(".") ? fixed : `./${fixed}`;
			}
		}
	}

	return rel;
}

let files = 0;
for (const file of walk(REPO)) {
	let content = readFileSync(file, "utf8");
	let changed = false;
	content = content.replace(/(\w[\w-]*)\s*=\s*\{\s*path\s*=\s*"([^"]+)"([^}]*)\}/g, (full, name, p, rest) => {
		const abs = normalize(join(dirname(file), p));
		if (existsSync(join(abs, "Cargo.toml"))) return full;
		const fixed = repairPath(file, p);
		if (fixed !== p) {
			changed = true;
			console.log(`${file.replace(REPO + "/", "")}: ${name} ${p} -> ${fixed}`);
			return `${name} = { path = "${fixed}"${rest}}`;
		}
		return full;
	});
	if (changed) {
		writeFileSync(file, content);
		files++;
	}
}
console.log(`repaired ${files} manifests`);
