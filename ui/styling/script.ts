#!/usr/bin/env bun
/** @emoji ⚙️ Reads `ui/styling/tokens.json`; emits palette CSS, TS, C#, Rust, and Python styling artifacts. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/src/index.ts";

const stylingRoot = import.meta.dir;
const tokensPath = join(stylingRoot, "tokens.json");
const generatedDir = join(stylingRoot, "generated");
const jsGeneratedDir = join(stylingRoot, "js");
const netPaletteDir = join(stylingRoot, "net", "Elements.Styling", "Generated");
const rustGeneratedDir = join(stylingRoot, "rs", "src");
const pyGeneratedDir = join(stylingRoot, "py", "styling");
const repoRoot = join(stylingRoot, "..", "..");

/** @emoji 📁 Canonical `ui/asset` directory (fonts, cursors, …). */
export const ELEMENTS_ASSETS_ROOT = join(stylingRoot, "..", "asset");
const elementsAssetsRoot = ELEMENTS_ASSETS_ROOT;
const composeNetPaletteDir = join(repoRoot, "compose", "client", "lib", "net", "Elements.Styling", "Generated");

const GOOGLE_FONTS_UA =
	"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const GOOGLE_FONT_QUERIES: Record<string, string> = {
	"font/anta": "Anta",
	"font/kelly-slab": "Kelly Slab",
	"font/share-tech-mono": "Share Tech Mono",
	"font/noto-emoji": "Noto Emoji",
};

type Rgba8 = [number, number, number, number];

interface PaintRef {
	token?: string;
	hex?: string;
	alpha?: number;
	mix?: [string, string, number];
}

interface Tokens {
	version: number;
	colors: Record<string, string>;
	spacing: Record<string, string>;
	fontStacks: Record<string, string>;
	fontFaces: { family: string; src: string }[];
	canvasFonts?: Record<string, string>;
	strokes?: Record<string, number | number[]>;
	radii?: Record<string, number>;
	opacities?: Record<string, number>;
	metrics?: Record<string, Record<string, number | number[]>>;
	themes?: Record<string, Record<string, Record<string, PaintRef>>>;
}

function colorKeyToCssVar(key: string): string {
	return `--color-${key.replaceAll("_", "-")}`;
}

function toPascalCase(s: string): string {
	return s
		.split(/[^a-zA-Z0-9]+/)
		.filter(Boolean)
		.map((p) => p[0]!.toUpperCase() + p.slice(1))
		.join("");
}

function toSnakeCase(s: string): string {
	return s.replace(/[A-Z]/g, (m) => `_${m.toLowerCase()}`);
}

function toScreamingSnake(s: string): string {
	return toSnakeCase(s).toUpperCase();
}

function loadTokens(): Tokens {
	const raw = readFileSync(tokensPath, "utf8");
	return JSON.parse(raw) as Tokens;
}

/** @emoji 📏 Derives dag component width as twice the IO channel column width. */
function resolveMetrics(metrics: Tokens["metrics"]): NonNullable<Tokens["metrics"]> {
	const out = structuredClone(metrics ?? {}) as NonNullable<Tokens["metrics"]>;
	const dag = out.dag;
	if (dag && typeof dag.ioColumnWidth === "number") {
		dag.componentWidth = dag.ioColumnWidth * 2;
	}
	return out;
}

function parseHex6(hex: string): [number, number, number] {
	const s = hex.trim().replace(/^#/, "");
	if (s.length === 3) {
		return [Number.parseInt(s[0]! + s[0], 16), Number.parseInt(s[1]! + s[1], 16), Number.parseInt(s[2]! + s[2], 16)];
	}
	const v = Number.parseInt(s, 16);
	return [(v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff];
}

function tokenHex(colors: Record<string, string>, key: string): string {
	const v = colors[key];
	if (!v) {
		throw new Error(`tokens.colors[${key}] missing`);
	}
	return v;
}

function blendHex(a: string, b: string, ratioA: number): string {
	const [ar, ag, ab] = parseHex6(a);
	const [br, bg, bb] = parseHex6(b);
	const t = Math.min(1, Math.max(0, ratioA));
	const r = Math.round(ar * t + br * (1 - t));
	const g = Math.round(ag * t + bg * (1 - t));
	const bl = Math.round(ab * t + bb * (1 - t));
	return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${bl.toString(16).padStart(2, "0")}`;
}

function resolvePaint(colors: Record<string, string>, ref: PaintRef): Rgba8 {
	let hex: string;
	let alpha = ref.alpha ?? 1;
	if (ref.mix) {
		const [a, b, ratio] = ref.mix;
		const bHex = b === "transparent" ? "#000000" : tokenHex(colors, b);
		hex = blendHex(tokenHex(colors, a), bHex, ratio);
		if (b === "transparent" && ref.alpha === undefined) {
			alpha = 1 - ratio;
		}
	} else if (ref.hex) {
		hex = ref.hex;
	} else if (ref.token) {
		hex = tokenHex(colors, ref.token);
	} else {
		throw new Error("paint ref needs token, hex, or mix");
	}
	const [r, g, b] = parseHex6(hex);
	return [r, g, b, Math.round(alpha * 255)];
}

function srgbByteToLinear(c: number): number {
	const x = c / 255;
	return x <= 0.04045 ? x / 12.92 : ((x + 0.055) / 1.055) ** 2.4;
}

function rgba8ToLinear(rgba: Rgba8): [number, number, number, number] {
	return [srgbByteToLinear(rgba[0]), srgbByteToLinear(rgba[1]), srgbByteToLinear(rgba[2]), rgba[3] / 255];
}

function rustF32(x: number): string {
	return `${x.toFixed(8).replace(/\.?0+$/, "")}_f32`;
}

function rustF64Lit(v: number): string {
	return Number.isInteger(v) ? `${v}.0` : String(v);
}

function resolveThemes(tokens: Tokens): Record<string, Record<string, Record<string, Rgba8>>> {
	const out: Record<string, Record<string, Record<string, Rgba8>>> = {};
	for (const [themeName, groups] of Object.entries(tokens.themes ?? {})) {
		out[themeName] = {};
		for (const [groupName, paints] of Object.entries(groups)) {
			out[themeName]![groupName] = {};
			for (const [paintName, ref] of Object.entries(paints)) {
				out[themeName]![groupName]![paintName] = resolvePaint(tokens.colors, ref);
			}
		}
	}
	return out;
}

function emitPaletteFonts(tokens: Tokens): string {
	const assetBase = "/asset";
	const lines: string[] = ["/* Generated from ui/styling/tokens.json — run `bun ./script.ts generate`. */"];
	for (const face of tokens.fontFaces) {
		const fam = face.family.includes(" ") ? JSON.stringify(face.family) : `"${face.family}"`;
		lines.push("@font-face {");
		lines.push(`  font-family: ${fam};`);
		lines.push(`  src: url("${assetBase}/${face.src}") format("woff2");`);
		lines.push("  font-weight: normal;");
		lines.push("  font-style: normal;");
		lines.push("  font-display: swap;");
		lines.push("}");
		lines.push("");
	}
	return lines.join("\n");
}

function emitPaletteTheme(tokens: Tokens): string {
	const lines: string[] = ["/* Generated from ui/styling/tokens.json — run `bun ./script.ts generate`. */", "@theme {", "  /* Primary brand colors */"];
	for (const [k, v] of Object.entries(tokens.colors)) {
		lines.push(`  ${colorKeyToCssVar(k)}: ${v};`);
	}
	lines.push("  /* Font interfaces */");
	lines.push(`  --font-sans: ${tokens.fontStacks.sans};`);
	lines.push(`  --font-serif: ${tokens.fontStacks.serif};`);
	lines.push(`  --font-mono: ${tokens.fontStacks.mono};`);
	lines.push("  /* Layout spacing */");
	for (const [k, v] of Object.entries(tokens.spacing)) {
		lines.push(`  --spacing-${k.replaceAll("_", "-")}: ${v};`);
	}
	lines.push("}");
	lines.push("");
	return lines.join("\n");
}

function emitJsonConst(name: string, value: unknown, indent = ""): string {
	return `${indent}export const ${name} = ${JSON.stringify(value, null, 2).replaceAll("\n", `\n${indent}`)} as const;\n`;
}

function emitTypeScriptTokens(tokens: Tokens, resolvedThemes: ReturnType<typeof resolveThemes>): string {
	const lines: string[] = ["/* Generated from ui/styling/tokens.json — run `bun ./script.ts generate`. */", ""];
	lines.push("export const STYLING_TOKENS = {");
	for (const [k, v] of Object.entries(tokens.colors)) {
		lines.push(`  "${k}": "${v}",`);
	}
	lines.push("} as const;");
	lines.push("");
	lines.push("export type StylingTokenKey = keyof typeof STYLING_TOKENS;");
	lines.push("");
	lines.push(emitJsonConst("STYLING_STROKES", tokens.strokes ?? {}));
	lines.push(emitJsonConst("STYLING_RADII", tokens.radii ?? {}));
	lines.push(emitJsonConst("STYLING_OPACITIES", tokens.opacities ?? {}));
	lines.push(emitJsonConst("STYLING_METRICS", resolveMetrics(tokens.metrics)));
	lines.push(emitJsonConst("STYLING_CANVAS_FONTS", tokens.canvasFonts ?? {}));
	const wasmThemes: Record<string, Record<string, number[]>> = {};
	for (const [themeName, groups] of Object.entries(resolvedThemes)) {
		if (!groups.board) {
			continue;
		}
		wasmThemes[themeName] = {};
		for (const [paintName, rgba] of Object.entries(groups.board)) {
			const camel = paintName;
			wasmThemes[themeName]![camel] = [...rgba];
		}
	}
	lines.push(emitJsonConst("STYLING_BOARD_THEMES", wasmThemes));
	lines.push("export type StylingThemeName = keyof typeof STYLING_BOARD_THEMES;");
	lines.push("");
	const mapThemes: Record<string, Record<string, number[]>> = {};
	for (const [themeName, groups] of Object.entries(resolvedThemes)) {
		if (!groups.map) {
			continue;
		}
		mapThemes[themeName] = {};
		for (const [paintName, rgba] of Object.entries(groups.map)) {
			mapThemes[themeName]![paintName] = [...rgba];
		}
	}
	lines.push(emitJsonConst("STYLING_MAP_THEMES", mapThemes));
	lines.push("");
	const canvasThemes: Record<string, Record<string, number[]>> = {};
	for (const [themeName, groups] of Object.entries(resolvedThemes)) {
		if (!groups.canvas) {
			continue;
		}
		canvasThemes[themeName] = {};
		for (const [paintName, rgba] of Object.entries(groups.canvas)) {
			canvasThemes[themeName]![paintName] = [...rgba];
		}
	}
	lines.push(emitJsonConst("STYLING_CANVAS_THEMES", canvasThemes));
	lines.push("");
	return lines.join("\n");
}

function emitCSharp(tokens: Tokens): string {
	const lines: string[] = [
		"// <auto-generated />",
		"// Generated from ui/styling/tokens.json — run `bun ./script.ts generate`.",
		"namespace Elements.Styling;",
		"",
		"public static class Palette",
		"{",
	];
	for (const [k, v] of Object.entries(tokens.colors)) {
		lines.push(`  public const string ${toPascalCase(k)} = "${v}";`);
	}
	lines.push("}");
	lines.push("");
	lines.push("public static class Strokes");
	lines.push("{");
	for (const [k, v] of Object.entries(tokens.strokes ?? {})) {
		if (Array.isArray(v)) {
			lines.push(`  public static readonly double[] ${toPascalCase(k)} = [${v.join(", ")}];`);
		} else {
			lines.push(`  public const double ${toPascalCase(k)} = ${v};`);
		}
	}
	lines.push("}");
	lines.push("");
	lines.push("public static class Radii");
	lines.push("{");
	for (const [k, v] of Object.entries(tokens.radii ?? {})) {
		lines.push(`  public const double ${toPascalCase(k)} = ${v};`);
	}
	lines.push("}");
	lines.push("");
	return lines.join("\n");
}

function emitRust(tokens: Tokens, resolvedThemes: ReturnType<typeof resolveThemes>): string {
	const lines: string[] = ["// @emoji 🎨 Auto-generated from ui/styling/tokens.json — do not edit by hand.", ""];
	for (const [group, values] of Object.entries({ strokes: tokens.strokes, radii: tokens.radii, opacities: tokens.opacities })) {
		if (!values) {
			continue;
		}
		lines.push(`pub mod ${group} {`);
		for (const [k, v] of Object.entries(values)) {
			const name = toScreamingSnake(k);
			if (Array.isArray(v)) {
				lines.push(`    pub const ${name}: &[f64] = &[${v.map((x) => rustF64Lit(x)).join(", ")}];`);
			} else if (group === "opacities" && Number.isInteger(v) && v > 1) {
				lines.push(`    pub const ${name}: u8 = ${v};`);
			} else {
				lines.push(`    pub const ${name}: f64 = ${rustF64Lit(v)};`);
			}
		}
		lines.push("}");
		lines.push("");
	}
	lines.push("pub mod metrics {");
	for (const [section, values] of Object.entries(resolveMetrics(tokens.metrics))) {
		lines.push(`    pub mod ${section} {`);
		for (const [k, v] of Object.entries(values)) {
			const name = toScreamingSnake(k);
			if (Array.isArray(v)) {
				lines.push(`        pub const ${name}: &[f64] = &[${v.map((x) => rustF64Lit(x)).join(", ")}];`);
			} else if (section === "board" && k === "maxWorldClipTiles") {
				lines.push(`        pub const ${name}: u32 = ${v};`);
			} else if (section === "map" && (k === "labelMaxMin" || k === "labelMaxMax")) {
				lines.push(`        pub const ${name}: u32 = ${v};`);
			} else {
				lines.push(`        pub const ${name}: f64 = ${rustF64Lit(v)};`);
			}
		}
		lines.push("    }");
	}
	lines.push("}");
	lines.push("");
	lines.push("pub mod canvas_fonts {");
	for (const [k, v] of Object.entries(tokens.canvasFonts ?? {})) {
		lines.push(`    pub const ${toScreamingSnake(k)}: &str = ${JSON.stringify(v)};`);
	}
	lines.push("}");
	lines.push("");
	for (const group of ["board", "map", "canvas"] as const) {
		lines.push(`pub struct ${toPascalCase(group)}Theme {`);
		const sample = resolvedThemes.light?.[group];
		if (sample) {
			for (const key of Object.keys(sample)) {
				lines.push(`    pub ${toSnakeCase(key)}: [f32; 4],`);
			}
		}
		lines.push("}");
		lines.push("");
		for (const themeName of ["light", "dark"] as const) {
			const paints = resolvedThemes[themeName]?.[group];
			if (!paints) {
				continue;
			}
			const constName = `${group.toUpperCase()}_${themeName.toUpperCase()}`;
			lines.push(`pub const ${constName}: ${toPascalCase(group)}Theme = ${toPascalCase(group)}Theme {`);
			for (const [key, rgba] of Object.entries(paints)) {
				const lin = rgba8ToLinear(rgba);
				lines.push(`    ${toSnakeCase(key)}: [${lin.map((x) => rustF32(x)).join(", ")}],`);
			}
			lines.push("};");
			lines.push("");
		}
	}
	return lines.join("\n");
}

function emitPython(tokens: Tokens, resolvedThemes: ReturnType<typeof resolveThemes>): string {
	const lines: string[] = [
		'"""@emoji 🎨 Auto-generated from ui/styling/tokens.json — do not edit by hand."""',
		"from __future__ import annotations",
		"from dataclasses import dataclass",
		"from typing import Final",
		"",
		"STYLING_TOKENS: Final[dict[str, str]] = {",
	];
	for (const [k, v] of Object.entries(tokens.colors)) {
		lines.push(`    ${JSON.stringify(k)}: ${JSON.stringify(v)},`);
	}
	lines.push("}");
	lines.push("");
	lines.push(`STYLING_STROKES: Final[dict[str, float | list[float]]] = ${JSON.stringify(tokens.strokes ?? {}, null, 4)}`);
	lines.push(`STYLING_RADII: Final[dict[str, float]] = ${JSON.stringify(tokens.radii ?? {}, null, 4)}`);
	lines.push(`STYLING_OPACITIES: Final[dict[str, float]] = ${JSON.stringify(tokens.opacities ?? {}, null, 4)}`);
	lines.push(`STYLING_METRICS: Final[dict[str, dict[str, float | list[float]]]] = ${JSON.stringify(resolveMetrics(tokens.metrics), null, 4)}`);
	lines.push("");
	for (const group of ["board", "map", "canvas"] as const) {
		lines.push("@dataclass(frozen=True, slots=True)");
		lines.push(`class ${toPascalCase(group)}Theme:`);
		const sample = resolvedThemes.light?.[group];
		if (sample) {
			for (const key of Object.keys(sample)) {
				lines.push(`    ${toSnakeCase(key)}: tuple[int, int, int, int]`);
			}
		}
		lines.push("");
		for (const themeName of ["light", "dark"] as const) {
			const paints = resolvedThemes[themeName]?.[group];
			if (!paints) {
				continue;
			}
			const constName = `${group.toUpperCase()}_${themeName.toUpperCase()}`;
			const fields = Object.entries(paints)
				.map(([k, rgba]) => `${toSnakeCase(k)}=(${rgba.join(", ")})`)
				.join(", ");
			lines.push(`${constName}: Final[${toPascalCase(group)}Theme] = ${toPascalCase(group)}Theme(${fields})`);
		}
		lines.push("");
	}
	return lines.join("\n");
}

function googleFontsCssUrl(family: string): string {
	const query = family.trim().replaceAll(" ", "+");
	return `https://fonts.googleapis.com/css2?family=${query}:wght@400&display=swap`;
}

function parseGoogleFontWoff2Map(css: string): Map<string, string> {
	const map = new Map<string, string>();
	let subset: string | undefined;
	for (const line of css.split("\n")) {
		const comment = line.match(/^\s*\/\*\s*([^*]+?)\s*\*\/\s*$/);
		if (comment) {
			subset = comment[1]!.trim().toLowerCase();
			continue;
		}
		const urlMatch = line.match(/url\((https:[^)]+\.woff2)\)/);
		if (!urlMatch) {
			continue;
		}
		const url = urlMatch[1]!;
		if (subset) {
			map.set(subset, url);
			subset = undefined;
			continue;
		}
		const indexMatch = url.match(/\.(\d+)\.woff2/);
		if (indexMatch) {
			map.set(indexMatch[1]!, url);
		}
	}
	return map;
}

function resolveFontFaceUrl(src: string, woff2ByKey: Map<string, string>): string | undefined {
	const base = src.split("/").pop()?.replace(/\.woff2$/, "") ?? "";
	if (src.startsWith("font/noto-emoji/")) {
		if (base === "emoji-400") {
			return woff2ByKey.get("2") ?? woff2ByKey.get("0");
		}
		const index = base.replace(/-400$/, "");
		return woff2ByKey.get(index) ?? woff2ByKey.get("9");
	}
	return woff2ByKey.get(base);
}

/** @emoji ⬇️ Downloads token font woff2 files into `ui/asset/font`. */
export async function fetchElementsFonts(): Promise<void> {
	const tokens = loadTokens();
	const cssByFamilyDir = new Map<string, Map<string, string>>();
	for (const [dir, family] of Object.entries(GOOGLE_FONT_QUERIES)) {
		const res = await fetch(googleFontsCssUrl(family), { headers: { "User-Agent": GOOGLE_FONTS_UA } });
		if (!res.ok) {
			throw new Error(`Google Fonts CSS failed for ${family}: ${res.status}`);
		}
		cssByFamilyDir.set(dir, parseGoogleFontWoff2Map(await res.text()));
	}
	let wrote = 0;
	for (const face of tokens.fontFaces) {
		const dirKey = Object.keys(GOOGLE_FONT_QUERIES).find((key) => face.src.startsWith(`${key}/`));
		if (!dirKey) {
			throw new Error(`No Google Fonts mapping for ${face.src}`);
		}
		const woff2ByKey = cssByFamilyDir.get(dirKey);
		if (!woff2ByKey?.size) {
			throw new Error(`No woff2 entries parsed for ${dirKey}`);
		}
		const remoteUrl = resolveFontFaceUrl(face.src, woff2ByKey);
		if (!remoteUrl) {
			throw new Error(`Could not resolve woff2 URL for ${face.src}`);
		}
		const dest = join(elementsAssetsRoot, face.src);
		mkdirSync(dirname(dest), { recursive: true });
		if (existsSync(dest)) {
			continue;
		}
		const fileRes = await fetch(remoteUrl);
		if (!fileRes.ok) {
			throw new Error(`Font download failed for ${face.src}: ${fileRes.status}`);
		}
		const bytes = new Uint8Array(await fileRes.arrayBuffer());
		if (bytes.length < 4 || bytes[0] !== 0x77 || bytes[1] !== 0x4f || bytes[2] !== 0x46 || bytes[3] !== 0x32) {
			throw new Error(`Downloaded bytes for ${face.src} are not woff2 (got ${bytes.length} bytes)`);
		}
		writeFileSync(dest, bytes);
		wrote += 1;
	}
	console.log(`ui/styling: fonts ready under ui/asset (${wrote} downloaded, ${tokens.fontFaces.length} total)`);
}

/** @emoji 🎨 Writes all styling artifacts from {@link tokens.json}. */
export function generateStylingArtifacts(): void {
	const tokens = loadTokens();
	const resolvedThemes = resolveThemes(tokens);
	mkdirSync(generatedDir, { recursive: true });
	mkdirSync(jsGeneratedDir, { recursive: true });
	mkdirSync(netPaletteDir, { recursive: true });
	mkdirSync(composeNetPaletteDir, { recursive: true });
	mkdirSync(rustGeneratedDir, { recursive: true });
	mkdirSync(pyGeneratedDir, { recursive: true });
	const fonts = emitPaletteFonts(tokens);
	const theme = emitPaletteTheme(tokens);
	const paletteCss = `${fonts}\n${theme}`;
	writeFileSync(join(generatedDir, "palette-fonts.css"), fonts, "utf8");
	writeFileSync(join(generatedDir, "palette-theme.css"), theme, "utf8");
	writeFileSync(join(jsGeneratedDir, "palette.css"), paletteCss, "utf8");
	writeFileSync(join(jsGeneratedDir, "tokens.generated.ts"), emitTypeScriptTokens(tokens, resolvedThemes), "utf8");
	const cs = emitCSharp(tokens);
	writeFileSync(join(netPaletteDir, "Palette.g.cs"), cs, "utf8");
	writeFileSync(join(composeNetPaletteDir, "Palette.g.cs"), cs, "utf8");
	writeFileSync(join(rustGeneratedDir, "generated.rs"), emitRust(tokens, resolvedThemes), "utf8");
	writeFileSync(join(pyGeneratedDir, "generated.py"), emitPython(tokens, resolvedThemes), "utf8");
}

class GenerateScript extends BundleScript {
	run(): void {
		generateStylingArtifacts();
		console.log("ui/styling: wrote generated CSS/TS/C#/Rust/Python styling artifacts");
	}
}

class FontsScript extends BundleScript {
	async run(): Promise<void> {
		await fetchElementsFonts();
	}
}

if (import.meta.main) {
	const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("fonts", FontsScript);
	await runBundleScriptMain(router, import.meta.url);
}
