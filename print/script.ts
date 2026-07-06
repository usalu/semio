#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./script.ts generate|fonts|build|watch|test`. */
import { spawnSync } from "node:child_process";
import { arch, platform } from "node:os";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { BundleScript, ScriptRouter, getWorkspaceRoot, runBundleScriptMain } from "../repo/lib/js/index.ts";

const printRoot = import.meta.dir;
const repoRoot = getWorkspaceRoot();
const tokensPath = join(repoRoot, "ui/styling/tokens.json");
const texDir = join(printRoot, "tex");
const fontRoot = join(printRoot, "asset/font");
const distDir = join(printRoot, "dist");
const tokensOut = join(texDir, "semio-tokens.sty");

const PRINT_FONTS: readonly { readonly family: string; readonly dir: string; readonly file: string; readonly url: string }[] = [
	{
		family: "Anta",
		dir: "anta",
		file: "Anta-Regular.ttf",
		url: "https://raw.githubusercontent.com/google/fonts/main/ofl/anta/Anta-Regular.ttf",
	},
	{
		family: "Kelly Slab",
		dir: "kelly-slab",
		file: "KellySlab-Regular.ttf",
		url: "https://raw.githubusercontent.com/google/fonts/main/ofl/kellyslab/KellySlab-Regular.ttf",
	},
	{
		family: "Share Tech Mono",
		dir: "share-tech-mono",
		file: "ShareTechMono-Regular.ttf",
		url: "https://raw.githubusercontent.com/google/fonts/main/ofl/sharetechmono/ShareTechMono-Regular.ttf",
	},
	{
		family: "Noto Emoji",
		dir: "noto-emoji",
		file: "NotoEmoji-Regular.ttf",
		url: "https://raw.githubusercontent.com/google/fonts/main/ofl/notoemoji/NotoEmoji%5Bwght%5D.ttf",
	},
];

const TEMPLATES: readonly { readonly id: string; readonly tex: string; readonly pdf: string }[] = [
	{ id: "report", tex: "template/report/report.tex", pdf: "report.pdf" },
	{ id: "paper", tex: "template/paper/paper.tex", pdf: "paper.pdf" },
	{ id: "flyer", tex: "template/flyer/flyer.tex", pdf: "flyer.pdf" },
	{ id: "forschungsbericht", tex: "template/zukunftbau/forschungsbericht.tex", pdf: "forschungsbericht.pdf" },
	{ id: "zwischenbericht", tex: "template/zukunftbau/zwischenbericht.tex", pdf: "zwischenbericht.pdf" },
	{ id: "kompaktbericht", tex: "template/zukunftbau/kompaktbericht.tex", pdf: "kompaktbericht.pdf" },
];

type Tokens = {
	readonly colors: Record<string, string>;
	readonly spacing: Record<string, string>;
	readonly strokes?: Record<string, number | number[]>;
};

function colorKeyToLatex(key: string): string {
	return `semio-${key.replaceAll("_", "-")}`;
}

function remToEm(rem: string): string {
	const match = rem.match(/^([\d.]+)rem$/);
	if (!match) return rem;
	return `${match[1]}em`;
}

/** @emoji 🎨 Writes `tex/semio-tokens.sty` from {@link ui/styling/tokens.json}. */
export function emitSemioTokensSty(): void {
	const tokens = JSON.parse(readFileSync(tokensPath, "utf8")) as Tokens;
	const lines: string[] = [
		"% Generated from ui/styling/tokens.json — run `bun ./script.ts generate`.",
		"\\NeedsTeXFormat{LaTeX2e}",
		"\\ProvidesPackage{semio-tokens}[2026/07/06 v0.1.0 semio design tokens]",
		"\\RequirePackage{xcolor}",
		"",
	];
	for (const [key, hex] of Object.entries(tokens.colors)) {
		const name = colorKeyToLatex(key);
		const html = hex.replace(/^#/, "");
		lines.push(`\\definecolor{${name}}{HTML}{${html}}`);
	}
	lines.push("");
	for (const [key, value] of Object.entries(tokens.spacing)) {
		lines.push(`\\newcommand{\\semio@spacing@${key.replaceAll("-", "@")}}{${remToEm(value)}}`);
	}
	const hairline = typeof tokens.strokes?.gridLarge === "number" ? tokens.strokes.gridLarge * 0.75 : 0.75;
	const strokeDefault = typeof tokens.strokes?.edgeBase === "number" ? tokens.strokes.edgeBase * 0.75 : 1.5;
	const strokeFocus = typeof tokens.strokes?.dagNodeSelected === "number" ? tokens.strokes.dagNodeSelected * 0.75 : 1.75;
	lines.push(`\\newcommand{\\semio@stroke@hairline}{${hairline}pt}`);
	lines.push(`\\newcommand{\\semio@stroke@default}{${strokeDefault}pt}`);
	lines.push(`\\newcommand{\\semio@stroke@focus}{${strokeFocus}pt}`);
	lines.push("");
	mkdirSync(texDir, { recursive: true });
	writeFileSync(tokensOut, lines.join("\n"), "utf8");
}

/** @emoji ⬇️ Downloads token TTF files into `asset/font`. */
export async function fetchPrintFonts(): Promise<void> {
	let wrote = 0;
	for (const font of PRINT_FONTS) {
		const destDir = join(fontRoot, font.dir);
		const dest = join(destDir, font.file);
		mkdirSync(destDir, { recursive: true });
		if (existsSync(dest)) continue;
		const fileRes = await fetch(font.url);
		if (!fileRes.ok) throw new Error(`Font download failed for ${font.family}: ${fileRes.status}`);
		const bytes = new Uint8Array(await fileRes.arrayBuffer());
		if (bytes.length < 4 || bytes[0] !== 0x00 || bytes[1] !== 0x01 || bytes[2] !== 0x00 || bytes[3] !== 0x00) {
			throw new Error(`Downloaded bytes for ${font.family} are not TTF (got ${bytes.length} bytes)`);
		}
		writeFileSync(dest, bytes);
		wrote += 1;
	}
	console.log(`print: fonts ready under print/asset/font (${wrote} downloaded, ${PRINT_FONTS.length} total)`);
}

const TECTONIC_VERSION = "0.16.9";

function tectonicTarget(): string {
	if (platform() === "darwin" && arch() === "arm64") return "aarch64-apple-darwin";
	if (platform() === "darwin") return "x86_64-apple-darwin";
	if (platform() === "win32") return "x86_64-pc-windows-msvc";
	if (platform() === "linux" && arch() === "arm64") return "aarch64-unknown-linux-musl";
	if (platform() === "linux" && arch() === "arm") return "arm-unknown-linux-musleabihf";
	if (platform() === "linux" && arch() === "ia32") return "i686-unknown-linux-gnu";
	return "x86_64-unknown-linux-gnu";
}

function tectonicCacheDir(): string {
	return join(repoRoot, ".repo/cache/tectonic", TECTONIC_VERSION);
}

function tectonicBinPath(): string {
	const ext = platform() === "win32" ? ".exe" : "";
	return join(tectonicCacheDir(), `tectonic${ext}`);
}

async function downloadTectonicBinary(): Promise<string> {
	const target = tectonicTarget();
	const isZip = platform() === "win32";
	const archiveName = `tectonic-${TECTONIC_VERSION}-${target}.${isZip ? "zip" : "tar.gz"}`;
	const url = `https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40${TECTONIC_VERSION}/${archiveName}`;
	const cacheDir = tectonicCacheDir();
	const archivePath = join(cacheDir, archiveName);
	mkdirSync(cacheDir, { recursive: true });
	const binPath = tectonicBinPath();
	if (existsSync(binPath)) return binPath;
	if (!existsSync(archivePath)) {
		const res = await fetch(url);
		if (!res.ok) throw new Error(`tectonic download failed (${res.status}) for ${url}`);
		writeFileSync(archivePath, new Uint8Array(await res.arrayBuffer()));
	}
	if (isZip) {
		const unzip = spawnSync("unzip", ["-o", archivePath, "-d", cacheDir], { stdio: "inherit" });
		if (unzip.status !== 0) {
			const extract = spawnSync("tar", ["-xf", archivePath, "-C", cacheDir], { stdio: "inherit" });
			if (extract.status !== 0) throw new Error("tectonic archive extract failed");
		}
	} else {
		const extract = spawnSync("tar", ["-xzf", archivePath, "-C", cacheDir], { stdio: "inherit" });
		if (extract.status !== 0) throw new Error("tectonic archive extract failed");
	}
	if (!existsSync(binPath)) throw new Error(`tectonic binary missing after extract: ${binPath}`);
	if (platform() !== "win32") spawnSync("chmod", ["+x", binPath], { stdio: "inherit" });
	return binPath;
}

let tectonicCommandCache: string | undefined;

async function ensureTectonic(): Promise<string> {
	if (tectonicCommandCache) return tectonicCommandCache;
	const probe = spawnSync("tectonic", ["--version"], { encoding: "utf8" });
	if (probe.status === 0) {
		tectonicCommandCache = "tectonic";
		return tectonicCommandCache;
	}
	tectonicCommandCache = await downloadTectonicBinary();
	return tectonicCommandCache;
}

function tectonicEnv(): NodeJS.ProcessEnv {
	const sep = process.platform === "win32" ? ";" : ":";
	return {
		...process.env,
		TEXINPUTS: `${texDir}${sep}`,
	};
}

function compilePrintDocument(tectonic: string, texAbs: string, outDir: string): void {
	const workDir = dirname(texAbs);
	const texFile = basename(texAbs);
	mkdirSync(outDir, { recursive: true });
	const build = spawnSync(
		tectonic,
		["--keep-logs", "--synctex", "-Z", `search-path=${texDir}`, "--outdir", outDir, texFile],
		{ cwd: workDir, stdio: "inherit", env: tectonicEnv() },
	);
	if (build.status !== 0) throw new Error(`tectonic build failed for ${texAbs}`);
	const pdf = join(outDir, `${basename(texAbs, ".tex")}.pdf`);
	if (!existsSync(pdf)) throw new Error(`missing PDF output: ${pdf}`);
	console.log(`[DEBUG] print built ${relative(repoRoot, pdf)}`);
}

/** @emoji 🖨️ Compiles one `.tex` file with the semio print search path and Tectonic. */
export async function buildPrintDocument(texAbs: string, outDir = join(dirname(texAbs), "dist")): Promise<void> {
	emitSemioTokensSty();
	const tectonic = await ensureTectonic();
	compilePrintDocument(tectonic, texAbs, outDir);
}

function compileTemplate(tectonic: string, template: (typeof TEMPLATES)[number]): void {
	const texAbs = join(printRoot, template.tex);
	compilePrintDocument(tectonic, texAbs, distDir);
}

function resolveTemplates(filter: string[]): (typeof TEMPLATES)[number][] {
	if (filter.length === 0) return [...TEMPLATES];
	const wanted = new Set(filter);
	const resolved = TEMPLATES.filter((template) => wanted.has(template.id));
	if (resolved.length === 0) throw new Error(`unknown template id(s): ${filter.join(", ")}`);
	return resolved;
}

function collectWatchRoots(): string[] {
	return [texDir, join(printRoot, "template"), fontRoot].filter((path) => existsSync(path));
}

async function watchTemplates(filter: string[]): Promise<void> {
	const templates = resolveTemplates(filter);
	const tectonic = await ensureTectonic();
	emitSemioTokensSty();
	const rebuild = () => {
		for (const template of templates) {
			try {
				compileTemplate(tectonic, template);
			} catch (error) {
				console.error(`[DEBUG] print watch rebuild failed for ${template.id}:`, error);
			}
		}
	};
	rebuild();
	const mtimes = new Map<string, number>();
	const touch = (path: string) => {
		try {
			mtimes.set(path, statSync(path).mtimeMs);
		} catch {
			/* ignore */
		}
	};
	const scan = (dir: string): void => {
		for (const root of collectWatchRoots()) {
			watch(root, { recursive: true }, (_event, file) => {
				if (!file) return;
				const abs = join(root, file);
				if (!/\.(tex|sty|cls|ttf|json)$/i.test(abs)) return;
				try {
					const mtime = statSync(abs).mtimeMs;
					if (mtimes.get(abs) === mtime) return;
					touch(abs);
					rebuild();
				} catch {
					/* ignore */
				}
			});
		}
	};
	scan(printRoot);
	console.log(`[DEBUG] print watching ${templates.map((t) => t.id).join(", ")}`);
}

class GenerateScript extends BundleScript {
	run(): void {
		emitSemioTokensSty();
		console.log("print: wrote tex/semio-tokens.sty");
	}
}

class FontsScript extends BundleScript {
	async run(): Promise<void> {
		await fetchPrintFonts();
	}
}

class BuildScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		const tectonic = await ensureTectonic();
		emitSemioTokensSty();
		for (const template of resolveTemplates(segments)) compileTemplate(tectonic, template);
	}
}

class WatchScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		await watchTemplates(segments);
	}
}

class TestScript extends BundleScript {
	async run(): Promise<void> {
		emitSemioTokensSty();
		await fetchPrintFonts();
		const tectonic = await ensureTectonic();
		for (const template of TEMPLATES) compileTemplate(tectonic, template);
		for (const template of TEMPLATES) {
			const pdf = join(distDir, template.pdf);
			if (!existsSync(pdf)) throw new Error(`test missing PDF: ${pdf}`);
		}
		console.log(`print: all ${TEMPLATES.length} template PDFs built`);
	}
}

const router = new ScriptRouter(printRoot)
	.register("generate", GenerateScript)
	.register("fonts", FontsScript)
	.register("build", BuildScript)
	.register("watch", WatchScript)
	.register("test", TestScript);

if (import.meta.main) {
	await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
}
