#!/usr/bin/env bun
/** 🏷️ Repo-wide migration: insert language-tag subdirectories under every bundle. */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { execSync } from "node:child_process";

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
const SKIP_DIRS = new Set(["node_modules", "target", "dist", ".venv", "storybook-static", ".git", ".cursor", ".repo", "pkg", "generated"]);
const SKIP_WALK = new Set([...SKIP_DIRS]);
const ROOT_DATA_DIRS = new Set(["example", "manifest", "generated"]);
const ROOT_KEEP = new Set(["package.json", "project.json", "script.ts", "AGENTS.md", "README.md", "LICENSE.md", ".env.example", "Dockerfile", "Caddyfile", "uv.lock"]);
const MIXED_BUNDLES = new Set([
	"flow/core",
	"coda/client/bin/assistant",
	"compose/client/bin/engine",
	"repo/server/coordinator",
]);
const CAD_JS_CHILDREN = [
	"cad/js/core",
	"cad/js/runtime",
	"cad/js/query",
	"cad/js/kernel/brepjs",
	"cad/js/renderer/core",
	"cad/js/renderer/react",
	"cad/js/renderer",
	"cad/js/module/spatial-shape",
	"cad/js/module/aec-building",
	"cad/js/module/aec-building-structure",
	"cad/js/module/aec-building-energy",
	"cad/js/machine/stately",
];

type Lang = "js" | "rs" | "py" | "go" | "cs";
type MoveRecord = { bundle: string; lang: Lang; files: string[] };

const moved: MoveRecord[] = [];
const log: string[] = [];

function rel(p: string): string {
	return relative(REPO, p).replace(/\\/g, "/");
}

function repairMisplacedFileDirs(bundlePath: string): void {
	for (const lang of ["js", "rs", "py", "go", "cs"] as const) {
		const langDir = join(bundlePath, lang);
		if (!existsSync(langDir)) continue;
		for (const name of readdirSync(langDir)) {
			const entry = join(langDir, name);
			if (!statSync(entry).isDirectory()) continue;
			const inner = join(entry, name);
			if (existsSync(inner) && statSync(inner).isFile()) {
				const tmp = join(langDir, `.${name}.tmp`);
				gitMv(inner, tmp);
				rmSync(entry, { recursive: true, force: true });
				gitMv(tmp, entry);
			}
		}
	}
}

function gitMv(from: string, to: string): void {
	mkdirSync(dirname(to), { recursive: true });
	if (!existsSync(from)) return;
	if (existsSync(to) && statSync(to).isDirectory()) {
		const inner = join(to, basename(from));
		if (existsSync(inner)) return;
		to = inner;
	}
	try {
		execSync(`git mv "${from}" "${to}"`, { cwd: REPO, stdio: "pipe" });
	} catch {
		mkdirSync(dirname(to), { recursive: true });
		execSync(`mv "${from}" "${to}"`, { cwd: REPO, stdio: "pipe" });
	}
}

function isBundleDir(dir: string): boolean {
	const names = readdirSync(dir);
	return (
		names.includes("package.json") ||
		names.includes("Cargo.toml") ||
		names.includes("pyproject.toml") ||
		names.includes("go.mod") ||
		names.some((n) => n.endsWith(".csproj"))
	);
}

function nestedBundleSubdirs(dir: string): string[] {
	const out: string[] = [];
	for (const name of readdirSync(dir)) {
		if (SKIP_DIRS.has(name)) continue;
		const sub = join(dir, name);
		if (!statSync(sub).isDirectory()) continue;
		if (isBundleDir(sub)) out.push(sub);
	}
	return out;
}

function isCompliant(bundlePath: string): boolean {
	return LANGUAGE_TAGS.has(basename(bundlePath));
}

function extLang(name: string): Lang | null {
	if (name.endsWith(".ts") || name.endsWith(".tsx")) return "js";
	if (name.endsWith(".rs")) return "rs";
	if (name.endsWith(".py")) return "py";
	if (name.endsWith(".go")) return "go";
	if (name.endsWith(".cs") || name.endsWith(".csproj")) return "cs";
	return null;
}

function classifyRootEntry(name: string, bundlePath: string): Lang | null {
	const bundleRel = rel(bundlePath);
	if (name === "script.ts" || ROOT_KEEP.has(name)) return null;
	if (name === "Cargo.toml" || name === "build.rs") return "rs";
	if (name === "pyproject.toml" || name === "tox.ini") return "py";
	if (name === "go.mod" || name === "go.sum") return "go";
	if (name.endsWith(".csproj")) return "cs";
	if (name === "vitest.config.ts" || name === "tsconfig.json" || name.startsWith("vite.") || name === "next.config.ts" || name === "next-env.d.ts") return "js";
	if (name.endsWith(".html") && bundleRel.includes("assistant") || name.endsWith(".html") && bundleRel.includes("engine")) return "js";
	return extLang(name);
}

function detectLanguages(bundlePath: string): Lang[] {
	const bundleRel = rel(bundlePath);
	if (MIXED_BUNDLES.has(bundleRel)) return ["js", "rs", "py", "go", "cs"].filter((l) => {
		if (l === "js" && bundleRel === "repo/server/coordinator") return existsSync(join(bundlePath, "app")) || existsSync(join(bundlePath, "next.config.ts"));
		if (l === "go" && bundleRel === "repo/server/coordinator") return existsSync(join(bundlePath, "go.mod"));
		if (l === "py" && (bundleRel.includes("assistant") || bundleRel.includes("engine"))) return existsSync(join(bundlePath, "main.py"));
		if (l === "js" && (bundleRel.includes("assistant") || bundleRel.includes("engine"))) return existsSync(join(bundlePath, "mcp-app.tsx"));
		if (l === "js" && bundleRel === "flow/core") return existsSync(join(bundlePath, "index.ts"));
		if (l === "rs" && bundleRel === "flow/core") return existsSync(join(bundlePath, "lib.rs"));
		return false;
	}) as Lang[];

	const langs = new Set<Lang>();
	for (const name of readdirSync(bundlePath)) {
		const full = join(bundlePath, name);
		if (statSync(full).isDirectory()) {
			if (SKIP_DIRS.has(name) || name === "Properties" || name === "bin" || name === "obj" || name === "stubs" || name === "app") continue;
			if (nestedBundleSubdirs(bundlePath).includes(full)) continue;
			if (name === "app" && rel(bundlePath) === "repo/server/coordinator") continue;
			continue;
		}
		const lang = classifyRootEntry(name, bundlePath);
		if (lang) langs.add(lang);
	}
	if (existsSync(join(bundlePath, "app")) && rel(bundlePath) !== "repo/server/coordinator") langs.add("js");
	if (existsSync(join(bundlePath, "Properties"))) langs.add("cs");
	if (existsSync(join(bundlePath, "stubs"))) langs.add("py");
	return [...langs];
}

function filesForLang(bundlePath: string, lang: Lang): string[] {
	const bundleRel = rel(bundlePath);
	const nested = new Set(nestedBundleSubdirs(bundlePath).map(rel));
	const out: string[] = [];

	if (MIXED_BUNDLES.has(bundleRel)) {
		if (lang === "js") {
			for (const name of ["mcp-app.tsx", "mcp-app.html", "vite.mcp-app.config.ts", "next.config.ts", "next-env.d.ts", "tsconfig.json"]) {
				if (existsSync(join(bundlePath, name))) out.push(name);
			}
			if (existsSync(join(bundlePath, "index.ts"))) out.push("index.ts");
			if (bundleRel === "flow/core" && existsSync(join(bundlePath, "index.ts"))) out.push("index.ts");
			if (bundleRel === "repo/server/coordinator" && existsSync(join(bundlePath, "app"))) out.push("app");
		}
		if (lang === "rs") {
			for (const name of readdirSync(bundlePath)) {
				if (name.endsWith(".rs") || name === "Cargo.toml" || name === "build.rs") out.push(name);
			}
		}
		if (lang === "py") {
			for (const name of ["main.py", "pyproject.toml", "uv.lock", "tox.ini"]) {
				if (existsSync(join(bundlePath, name))) out.push(name);
			}
			if (existsSync(join(bundlePath, "stubs"))) out.push("stubs");
		}
		if (lang === "go") {
			for (const name of readdirSync(bundlePath)) {
				if (name.endsWith(".go") || name === "go.mod" || name === "go.sum") out.push(name);
			}
		}
		return out;
	}

	for (const name of readdirSync(bundlePath)) {
		const full = join(bundlePath, name);
		if (statSync(full).isDirectory()) {
			if (nested.has(rel(full))) continue;
			if (ROOT_DATA_DIRS.has(name)) continue;
			if (lang === "cs" && name === "Properties") out.push(name);
			if (lang === "py" && name === "stubs") out.push(name);
			if (lang === "js" && name === "app") out.push(name);
			continue;
		}
		if (classifyRootEntry(name, bundlePath) === lang) out.push(name);
	}
	return out;
}

function prefixExportPath(value: string, lang: Lang): string {
	if (!value.startsWith("./")) return value;
	const rest = value.slice(2);
	if (rest.startsWith(`${lang}/`) || rest.startsWith("pkg/")) return value;
	return `./${lang}/${rest}`;
}

function updatePackageJson(bundlePath: string, langs: Lang[]): void {
	const pkgPath = join(bundlePath, "package.json");
	if (!existsSync(pkgPath)) return;
	const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as Record<string, unknown>;
	const primaryLang = langs.includes("js") ? "js" : langs[0];
	if (pkg.exports && typeof pkg.exports === "object") {
		for (const [key, val] of Object.entries(pkg.exports as Record<string, string>)) {
			if (typeof val !== "string") continue;
			if (val.includes("/pkg/")) {
				(pkg.exports as Record<string, string>)[key] = val.replace("./pkg/", "./rs/pkg/");
			} else if (primaryLang && !val.startsWith(`./${primaryLang}/`)) {
				(pkg.exports as Record<string, string>)[key] = prefixExportPath(val, primaryLang);
			}
		}
	}
	for (const field of ["main", "module", "types", "sourceRoot"] as const) {
		if (typeof pkg[field] === "string") {
			const v = pkg[field] as string;
			if (v.includes("/pkg/")) pkg[field] = v.replace("./pkg/", "./rs/pkg/");
			else if (primaryLang) pkg[field] = prefixExportPath(v, primaryLang);
		}
	}
	if (pkg.repository && typeof pkg.repository === "object" && (pkg.repository as { directory?: string }).directory) {
		// directory updated separately for cad moves
	}
	writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
}

function updateScriptTs(bundlePath: string, langs: Lang[]): void {
	const scriptPath = join(bundlePath, "script.ts");
	if (!existsSync(scriptPath)) return;
	let content = readFileSync(scriptPath, "utf8");
	const bundleRel = rel(bundlePath);
	if (langs.includes("rs")) {
		content = content.replace(/rsDir:\s*this\.root/g, 'rsDir: join(this.root, "rs")');
		content = content.replace(/cwd:\s*this\.root/g, 'cwd: join(this.root, "rs")');
		if (!content.includes('from "node:path"') && !content.includes("join(this.root")) {
			content = content.replace(/^(import .+\n)+/m, (m) => `${m}import { join } from "node:path";\n`);
		}
	}
	if (langs.includes("js") && content.includes("runVitest(this.root")) {
		content = content.replace(/runVitest\(this\.root,\s*segments\)/g, 'runVitest(this.root, segments, "js/vitest.config.ts")');
		content = content.replace(/runVitest\(this\.root,\s*segments,\s*"vitest\.config\.ts"\)/g, 'runVitest(this.root, segments, "js/vitest.config.ts")');
	}
	if (langs.includes("go") && content.includes("this.root") && bundleRel.includes("cli")) {
		content = content.replace(/cwd:\s*this\.root/g, 'cwd: join(this.root, "go")');
	}
	writeFileSync(scriptPath, content);
}

function updateVitestIncludes(bundlePath: string): void {
	const cfg = join(bundlePath, "js/vitest.config.ts");
	if (!existsSync(cfg) || !statSync(cfg).isFile()) return;
	let content = readFileSync(cfg, "utf8");
	content = content.replace(/include:\s*\[[^\]]+\]/, (m) => m.replace(/"([^"]+)"/g, (_, f) => `"${f}"`));
	writeFileSync(cfg, content);
}

function migrateBundle(bundlePath: string): void {
	const bundleRel = rel(bundlePath);
	if (isCompliant(bundlePath)) {
		log.push(`skip compliant: ${bundleRel}`);
		return;
	}
	if (!isBundleDir(bundlePath)) return;

	const langs = detectLanguages(bundlePath);
	if (langs.length === 0) {
		log.push(`skip no langs: ${bundleRel}`);
		return;
	}

	for (const lang of langs) {
		const langDir = join(bundlePath, lang);
		mkdirSync(langDir, { recursive: true });
		const files = filesForLang(bundlePath, lang).filter((file) => existsSync(join(bundlePath, file)));
		if (files.length === 0) continue;
		const record: MoveRecord = { bundle: bundleRel, lang, files: [] };
		for (const file of files) {
			const from = join(bundlePath, file);
			const to = join(langDir, file);
			if (!existsSync(from)) continue;
			gitMv(from, to);
			record.files.push(file);
		}
		if (record.files.length) moved.push(record);
	}

	updatePackageJson(bundlePath, langs);
	updateScriptTs(bundlePath, langs);
	repairMisplacedFileDirs(bundlePath);
	updateVitestIncludes(bundlePath);
	if (langs.includes("rs") && existsSync(join(bundlePath, "pkg"))) {
		rmSync(join(bundlePath, "pkg"), { recursive: true, force: true });
	}
	log.push(`migrated: ${bundleRel} → ${langs.join(",")}`);
}

function dismantleCadJs(): void {
	for (const oldPath of CAD_JS_CHILDREN.sort((a, b) => b.length - a.length)) {
		const newPath = oldPath.replace("/js/", "/").replace("cad/js/", "cad/");
		const from = join(REPO, oldPath);
		const to = join(REPO, newPath);
		if (!existsSync(from)) continue;
		mkdirSync(dirname(to), { recursive: true });
		gitMv(from, to);
		log.push(`cad dismantle: ${oldPath} → ${newPath}`);
	}
	const cadJs = join(REPO, "cad/js");
	if (existsSync(cadJs)) {
		const remaining = readdirSync(cadJs);
		if (remaining.length <= 1) {
			for (const name of remaining) rmSync(join(cadJs, name), { recursive: true, force: true });
			rmSync(cadJs, { recursive: true, force: true });
		}
	}
}

function collectBundles(dir: string, out: string[] = []): string[] {
	if (dir === REPO) {
		for (const name of readdirSync(dir)) {
			if (SKIP_WALK.has(name)) continue;
			const sub = join(dir, name);
			if (statSync(sub).isDirectory()) collectBundles(sub, out);
		}
		return out;
	}
	if (basename(dir).startsWith(".") && basename(dir) !== ".") return out;
	if (!existsSync(dir) || !statSync(dir).isDirectory()) return out;
	if (isBundleDir(dir)) out.push(dir);
	for (const name of readdirSync(dir)) {
		if (SKIP_WALK.has(name)) continue;
		const sub = join(dir, name);
		if (statSync(sub).isDirectory()) collectBundles(sub, out);
	}
	return out;
}

function updateRootPackageJsonWorkspaces(): void {
	const pkgPath = join(REPO, "package.json");
	const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as { workspaces: string[] };
	pkg.workspaces = pkg.workspaces.map((w) => w.replace(/cad\/js\//g, "cad/"));
	writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
}

function updateCargoWorkspace(): void {
	const cargoPath = join(REPO, "Cargo.toml");
	let content = readFileSync(cargoPath, "utf8");
	const compliantRs = new Set([
		"vcs/rs", "s/rs", "draw/rs", "forms/rs", "shooting/rs", "cad/rs", "framework/product/presentation/rs",
		"ui/styling/rs", "infinite/cavas/rs", "reasoning/mindmap/rs", "gis/2d/rs", "puzzle/2d/rs", "raster/rs",
		"writer/rs", "puzzle/3d/rs", "puzzle/5d/rs", "procedural/2d/rs", "procedural/3d/rs", "compose/client/lib/rs",
		"kernel/3d/brep/rs", "kernel/2d/rs", "layout/rs",
	]);
	content = content.replace(/^(\s*"[^"]+"),?\s*$/gm, (line) => {
		const m = line.match(/^\s*"([^"]+)"/);
		if (!m) return line;
		const member = m[1];
		if (member.endsWith("/rs") || compliantRs.has(member)) return line;
		if (member.includes("/js/")) return line.replace("/js/", "/");
		if (existsSync(join(REPO, member, "Cargo.toml"))) return line;
		if (existsSync(join(REPO, member, "rs", "Cargo.toml"))) {
			return line.replace(`"${member}"`, `"${member}/rs"`);
		}
		return line;
	});
	writeFileSync(cargoPath, content);
}

function updateAllCargoPathDeps(): void {
	const cargoFiles: string[] = [];
	function walk(d: string) {
		for (const name of readdirSync(d)) {
			if (SKIP_DIRS.has(name) || name.startsWith(".")) continue;
			const p = join(d, name);
			if (statSync(p).isDirectory()) walk(p);
			else if (name === "Cargo.toml") cargoFiles.push(p);
		}
	}
	walk(REPO);
	for (const file of cargoFiles) {
		let content = readFileSync(file, "utf8");
		let changed = false;
		content = content.replace(/path\s*=\s*"([^"]+)"/g, (full, p: string) => {
			const abs = join(dirname(file), p);
			if (existsSync(join(abs, "rs", "Cargo.toml")) && !existsSync(join(abs, "Cargo.toml"))) {
				changed = true;
				const normalized = p.endsWith("/") ? `${p}rs` : `${p}/rs`;
				return `path = "${normalized}"`;
			}
			return full;
		});
		if (changed) writeFileSync(file, content);
	}
}

function updateGoWork(): void {
	const goWorkPath = join(REPO, "go.work");
	if (!existsSync(goWorkPath)) return;
	let content = readFileSync(goWorkPath, "utf8");
	for (const mod of ["repo/client/cli", "repo/server/coordinator", "repo/client/mcp", "repo/client/mcp/claude", "repo/client/mcp/codex", "repo/client/mcp/copilot", "repo/client/mcp/cursor", "repo/client/mcp/kiro"]) {
		if (content.includes(`./${mod}`) && !content.includes(`./${mod}/go`)) {
			if (existsSync(join(REPO, mod, "go.mod"))) {
				content = content.replace(`./${mod}`, `./${mod}/go`);
			}
		}
	}
	writeFileSync(goWorkPath, content);
}

function updateMonorepoSln(): void {
	const slnPath = join(REPO, "Monorepo.sln");
	if (!existsSync(slnPath)) return;
	let content = readFileSync(slnPath, "utf8");
	content = content.replace(/\\([^\\]+)\.csproj/g, (m, name) => {
		if (m.includes("\\cs\\")) return m;
		return m.replace(`${name}.csproj`, `cs\\${name}.csproj`);
	});
	writeFileSync(slnPath, content);
}

function updateRepositoryDirectories(): void {
	for (const bundle of collectBundles(REPO)) {
		const pkgPath = join(bundle, "package.json");
		if (!existsSync(pkgPath)) continue;
		const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as { repository?: { directory?: string } };
		if (pkg.repository?.directory) {
			pkg.repository.directory = rel(bundle);
			writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
		}
	}
}

function updateCrossImports(): void {
	const exts = [".ts", ".tsx", ".rs", ".py", ".go"];
	function walk(d: string) {
		for (const name of readdirSync(d)) {
			if (SKIP_DIRS.has(name) || name.startsWith(".")) continue;
			const p = join(d, name);
			if (statSync(p).isDirectory()) walk(p);
			else if (exts.some((e) => name.endsWith(e))) {
				let content = readFileSync(p, "utf8");
				const orig = content;
				content = content.replace(/cad\/js\//g, "cad/");
				content = content.replace(/from\s+["'](\.\.\/)+repo\/lib\/js\/index\.ts["']/g, (m) => m);
				if (content !== orig) writeFileSync(p, content);
			}
		}
	}
	walk(REPO);
}

const dryRun = process.argv.includes("--dry-run");

if (dryRun) {
	console.log("Dry run — inventory only");
	const bundles = collectBundles(REPO);
	for (const b of bundles.sort()) {
		if (isCompliant(b)) continue;
		const langs = detectLanguages(b);
		if (!langs.length) continue;
		console.log(`${rel(b)}: ${langs.join(", ")} → ${langs.map((l) => filesForLang(b, l).join(", ")).join(" | ")}`);
	}
	process.exit(0);
}

console.log("[DEBUG] Phase 1: dismantle cad/js");
dismantleCadJs();

console.log("[DEBUG] Phase 2: migrate bundles");
const bundles = [...new Set(collectBundles(REPO))].sort((a, b) => b.split("/").length - a.split("/").length);
for (const bundle of bundles) migrateBundle(bundle);

console.log("[DEBUG] Phase 3: update root configs");
updateRootPackageJsonWorkspaces();
updateCargoWorkspace();
updateAllCargoPathDeps();
updateGoWork();
updateMonorepoSln();
updateRepositoryDirectories();
updateCrossImports();

writeFileSync(join(import.meta.dir, "migrate-log.json"), JSON.stringify({ moved, log }, null, 2));
console.log(`Done. ${moved.length} language moves across ${new Set(moved.map((m) => m.bundle)).size} bundles.`);
