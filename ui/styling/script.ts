#!/usr/bin/env bun
/** @emoji ⚙️ Reads `ui/styling/tokens.json`; emits palette CSS, TS, C#, Rust, and Python styling artifacts. */
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/index.ts";
import { parseUiTheme, resolveThemeMetrics, resolveThemePaint, type ThemePaintRef, type UiTheme } from "./js/theme.ts";

const stylingRoot = import.meta.dir;
const tokensPath = join(stylingRoot, "tokens.json");
const generatedDir = join(stylingRoot, "generated");
const jsGeneratedDir = join(stylingRoot, "js");
const netPaletteDir = join(stylingRoot, "net", "Elements.Styling", "Generated");
const rustGeneratedDir = join(stylingRoot, "rs");
const pyGeneratedDir = join(stylingRoot, "py", "styling");
const repoRoot = join(stylingRoot, "..", "..");

/** @emoji 📁 Canonical `ui/asset` directory (fonts, cursors, …). */
export const ELEMENTS_ASSETS_ROOT = join(stylingRoot, "..", "asset");
const elementsAssetsRoot = ELEMENTS_ASSETS_ROOT;
const composeNetPaletteDir = join(repoRoot, "compose", "client", "lib", "net", "Elements.Styling", "Generated");

const GOOGLE_FONTS_UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const GOOGLE_FONT_QUERIES: Record<string, string> = {
  "font/anta": "Anta",
  "font/kelly-slab": "Kelly Slab",
  "font/share-tech-mono": "Share Tech Mono",
  "font/noto-emoji": "Noto Emoji",
};

type Rgba8 = [number, number, number, number];

type PaintRef = ThemePaintRef;

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
  appearances?: Record<string, Record<string, Record<string, PaintRef>>>;
}

const APPEARANCE_NAMES = ["light", "dark"] as const;

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
  return resolveThemeMetrics(metrics ?? {}) as NonNullable<Tokens["metrics"]>;
}

function resolvePaint(colors: Record<string, string>, ref: PaintRef): Rgba8 {
  return resolveThemePaint(colors, ref);
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

function resolveAppearances(tokens: Tokens): Record<string, Record<string, Record<string, Rgba8>>> {
  const out: Record<string, Record<string, Record<string, Rgba8>>> = {};
  for (const [appearanceName, groups] of Object.entries(tokens.appearances ?? {})) {
    out[appearanceName] = {};
    for (const [groupName, paints] of Object.entries(groups)) {
      out[appearanceName]![groupName] = {};
      for (const [paintName, ref] of Object.entries(paints)) {
        out[appearanceName]![groupName]![paintName] = resolvePaint(tokens.colors, ref);
      }
    }
  }
  return out;
}

function paletteGroupNames(resolvedAppearances: ReturnType<typeof resolveAppearances>): string[] {
  return Object.keys(resolvedAppearances.light ?? {}).sort();
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

/** @emoji 🎨 Builds the default "semio" `UiTheme` verbatim from tokens.json (the paint refs stay unresolved). */
function buildSemioUiTheme(tokens: Tokens): UiTheme {
  return {
    id: "semio",
    label: "semio",
    colors: tokens.colors,
    spacing: tokens.spacing,
    fontStacks: tokens.fontStacks,
    canvasFonts: tokens.canvasFonts ?? {},
    strokes: tokens.strokes ?? {},
    radii: tokens.radii ?? {},
    opacities: tokens.opacities ?? {},
    metrics: resolveMetrics(tokens.metrics),
    appearances: (tokens.appearances ?? {}) as UiTheme["appearances"],
  };
}

function emitTypeScriptTokens(tokens: Tokens, resolvedAppearances: ReturnType<typeof resolveAppearances>): string {
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
  for (const group of paletteGroupNames(resolvedAppearances)) {
    const groupPalettes: Record<string, Record<string, number[]>> = {};
    for (const [appearanceName, groups] of Object.entries(resolvedAppearances)) {
      const paints = groups[group];
      if (!paints) {
        continue;
      }
      groupPalettes[appearanceName] = {};
      for (const [paintName, rgba] of Object.entries(paints)) {
        groupPalettes[appearanceName]![paintName] = [...rgba];
      }
    }
    lines.push(emitJsonConst(`STYLING_${group.toUpperCase()}_PALETTES`, groupPalettes));
    if (group === "board") {
      lines.push("export type StylingAppearanceName = keyof typeof STYLING_BOARD_PALETTES;");
    }
    lines.push("");
  }
  lines.push(emitJsonConst("STYLING_SEMIO_THEME", buildSemioUiTheme(tokens)));
  return lines.join("\n");
}

function emitCSharp(tokens: Tokens): string {
  const lines: string[] = ["// <auto-generated />", "// Generated from ui/styling/tokens.json — run `bun ./script.ts generate`.", "namespace Elements.Styling;", "", "public static class Palette", "{"];
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

function emitRust(tokens: Tokens, resolvedAppearances: ReturnType<typeof resolveAppearances>): string {
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
  for (const group of paletteGroupNames(resolvedAppearances)) {
    lines.push(`pub struct ${toPascalCase(group)}Palette {`);
    const sample = resolvedAppearances.light?.[group];
    if (sample) {
      for (const key of Object.keys(sample)) {
        lines.push(`    pub ${toSnakeCase(key)}: [f32; 4],`);
      }
    }
    lines.push("}");
    lines.push("");
    for (const appearanceName of APPEARANCE_NAMES) {
      const paints = resolvedAppearances[appearanceName]?.[group];
      if (!paints) {
        continue;
      }
      const constName = `${group.toUpperCase()}_${appearanceName.toUpperCase()}`;
      lines.push(`pub const ${constName}: ${toPascalCase(group)}Palette = ${toPascalCase(group)}Palette {`);
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

function emitPython(tokens: Tokens, resolvedAppearances: ReturnType<typeof resolveAppearances>): string {
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
  for (const group of paletteGroupNames(resolvedAppearances)) {
    lines.push("@dataclass(frozen=True, slots=True)");
    lines.push(`class ${toPascalCase(group)}Palette:`);
    const sample = resolvedAppearances.light?.[group];
    if (sample) {
      for (const key of Object.keys(sample)) {
        lines.push(`    ${toSnakeCase(key)}: tuple[int, int, int, int]`);
      }
    }
    lines.push("");
    for (const appearanceName of APPEARANCE_NAMES) {
      const paints = resolvedAppearances[appearanceName]?.[group];
      if (!paints) {
        continue;
      }
      const constName = `${group.toUpperCase()}_${appearanceName.toUpperCase()}`;
      const fields = Object.entries(paints)
        .map(([k, rgba]) => `${toSnakeCase(k)}=(${rgba.join(", ")})`)
        .join(", ");
      lines.push(`${constName}: Final[${toPascalCase(group)}Palette] = ${toPascalCase(group)}Palette(${fields})`);
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
  const base =
    src
      .split("/")
      .pop()
      ?.replace(/\.woff2$/, "") ?? "";
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
  const resolvedAppearances = resolveAppearances(tokens);
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
  writeFileSync(join(jsGeneratedDir, "tokens.generated.ts"), emitTypeScriptTokens(tokens, resolvedAppearances), "utf8");
  const cs = emitCSharp(tokens);
  writeFileSync(join(netPaletteDir, "Palette.g.cs"), cs, "utf8");
  writeFileSync(join(composeNetPaletteDir, "Palette.g.cs"), cs, "utf8");
  writeFileSync(join(rustGeneratedDir, "generated.rs"), emitRust(tokens, resolvedAppearances), "utf8");
  writeFileSync(join(pyGeneratedDir, "generated.py"), emitPython(tokens, resolvedAppearances), "utf8");
  validatePremadeThemes();
}

const premadeThemeDir = join(stylingRoot, "theme");

/** @emoji 🔎 Parses and resolves every premade `*.theme.json` so a broken preset fails `generate` instead of shipping. */
function validatePremadeThemes(): void {
  if (!existsSync(premadeThemeDir)) {
    return;
  }
  for (const entry of readdirSync(premadeThemeDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".theme.json")) {
      continue;
    }
    const path = join(premadeThemeDir, entry.name);
    const raw = readFileSync(path, "utf8");
    try {
      parseUiTheme(JSON.parse(raw));
    } catch (err) {
      throw new Error(`ui/styling/theme/${entry.name} is invalid: ${err instanceof Error ? err.message : String(err)}`);
    }
  }
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

const PX_SCAN_ROOTS = ["ui/react", "ui/styling/js", "framework/product", "coda/client/ui", "flow/react", "cad/renderer", "puzzle", "infinite/world", "gis/2d"] as const;

const PX_SCAN_SKIP = ["/.repo/", "/node_modules/", "/.storybook/", "/fixture/", "tokens.generated.", "session.json", ".plan.md"];

const PX_PATTERNS: { name: string; re: RegExp }[] = [{ name: "tailwind-arbitrary-px", re: /\[(?!9999px)[-0-9]*\.?[0-9]+px\]/ }];

function isPxScanExemptLine(line: string): boolean {
  if (line.includes("--stroke-hairline: 1px")) {
    return true;
  }
  if (line.includes("expect(") || line.includes("toContain(")) {
    return true;
  }
  if (line.includes("@media") && line.includes("px")) {
    return true;
  }
  if (/\b(h|w|min-h|min-w|max-h|max-w)-px\b/.test(line)) {
    return true;
  }
  if (line.includes("rounded-[9999px]")) {
    return true;
  }
  if (line.includes("cursor:") && line.includes("url(")) {
    return true;
  }
  if (/font=["'`]\d/.test(line) && line.includes("px")) {
    return true;
  }
  if (line.includes("transform:") && line.includes("px")) {
    return true;
  }
  if (line.includes("translate3d") || line.includes("translate(")) {
    return true;
  }
  if (line.includes("patchAutoAnimate") || line.includes("innerHTML")) {
    return true;
  }
  return false;
}

function shouldPxScanFile(path: string): boolean {
  if (!/\.(tsx?|css)$/.test(path)) {
    return false;
  }
  return !PX_SCAN_SKIP.some((skip) => path.includes(skip));
}

function collectPxViolations(repoRoot: string): { file: string; line: number; kind: string; text: string }[] {
  const violations: { file: string; line: number; kind: string; text: string }[] = [];

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === ".repo") {
          continue;
        }
        walk(full);
        continue;
      }
      const rel = full.slice(repoRoot.length + 1);
      if (!shouldPxScanFile(rel)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (isPxScanExemptLine(line)) {
          continue;
        }
        for (const { name, re } of PX_PATTERNS) {
          if (re.test(line)) {
            violations.push({ file: rel, line: i + 1, kind: name, text: line.trim() });
            break;
          }
        }
      }
    }
  };

  for (const root of PX_SCAN_ROOTS) {
    const abs = join(repoRoot, root);
    if (existsSync(abs)) {
      walk(abs);
    }
  }
  return violations;
}

/** @emoji 🚫 Fails when hardcoded `px` sizing literals remain in scanned source roots. */
class CheckNoPxScript extends BundleScript {
  run(): void {
    const violations = collectPxViolations(repoRoot);
    if (violations.length === 0) {
      console.log("ui/styling: no hardcoded px sizing violations");
      return;
    }
    console.error(`ui/styling: found ${violations.length} hardcoded px sizing violation(s):`);
    for (const v of violations.slice(0, 80)) {
      console.error(`  ${v.file}:${v.line} [${v.kind}] ${v.text}`);
    }
    if (violations.length > 80) {
      console.error(`  … and ${violations.length - 80} more`);
    }
    process.exit(1);
  }
}

const COLOR_SCAN_ROOTS = PX_SCAN_ROOTS.filter((root) => !root.startsWith("ui/styling"));

const COLOR_SCAN_SKIP = [...PX_SCAN_SKIP, "/dist/", "/.vite/", "/.stage/", "/renderer-modules/", "/plugin-modules/", "generated/", "generated.rs", "generated.py", "Palette.g.cs", "palette.css"];

const COLOR_PATTERNS: { name: string; re: RegExp }[] = [
  { name: "raw-hex-color", re: /#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?(?:[0-9a-fA-F]{2})?\b/ },
  { name: "raw-rgb-hsl-color", re: /\b(?:rgba?|hsla?)\(\s*[\d.]/ },
  { name: "tailwind-neutral-color-class", re: /\b(?:bg|text|border|ring|fill|stroke|from|via|to|divide|outline|decoration|caret|accent|shadow)-(?:zinc|gray|slate|neutral|stone)-\d{2,3}\b/ },
];

function isColorScanExemptLine(line: string): boolean {
  if (line.includes("expect(") || line.includes("toContain(") || line.includes("toBe(")) {
    return true;
  }
  if (/^\s*(\/\/|\*|\/\*)/.test(line)) {
    return true;
  }
  return false;
}

function shouldColorScanFile(path: string): boolean {
  if (!/\.(tsx?|css|rs)$/.test(path)) {
    return false;
  }
  return !COLOR_SCAN_SKIP.some((skip) => path.includes(skip));
}

function collectColorViolations(repoRootPath: string): { file: string; line: number; kind: string; text: string }[] {
  const violations: { file: string; line: number; kind: string; text: string }[] = [];

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === ".repo") {
          continue;
        }
        walk(full);
        continue;
      }
      const rel = full.slice(repoRootPath.length + 1);
      if (!shouldColorScanFile(rel)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (isColorScanExemptLine(line)) {
          continue;
        }
        for (const { name, re } of COLOR_PATTERNS) {
          if (re.test(line)) {
            violations.push({ file: rel, line: i + 1, kind: name, text: line.trim() });
            break;
          }
        }
      }
    }
  };

  for (const root of COLOR_SCAN_ROOTS) {
    const abs = join(repoRootPath, root);
    if (existsSync(abs)) {
      walk(abs);
    }
  }
  return violations;
}

/** @emoji 🚫 Fails when hardcoded hex/rgb/hsl colors or zinc/gray/slate/neutral/stone Tailwind classes remain outside `ui/styling` (design tokens are the single source of color truth). */
class CheckNoRawColorsScript extends BundleScript {
  run(): void {
    const violations = collectColorViolations(repoRoot);
    if (violations.length === 0) {
      console.log("ui/styling: no hardcoded color violations");
      return;
    }
    console.error(`ui/styling: found ${violations.length} hardcoded color violation(s):`);
    for (const v of violations.slice(0, 80)) {
      console.error(`  ${v.file}:${v.line} [${v.kind}] ${v.text}`);
    }
    if (violations.length > 80) {
      console.error(`  … and ${violations.length - 80} more`);
    }
    process.exit(1);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("fonts", FontsScript).register("check-no-px", CheckNoPxScript).register("check-no-raw-colors", CheckNoRawColorsScript);
  await runBundleScriptMain(router, import.meta.url);
}
