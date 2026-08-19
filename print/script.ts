#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./script.ts generate|fonts|build|watch|test|test-e2e`. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { arch, platform } from "node:os";
import { existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { inflateRawSync, inflateSync, deflateSync } from "node:zlib";
import { BundleScript, ScriptRouter, getWorkspaceRoot, resolveTestLevel, runBundleScriptMain, TEST_LEVELS } from "../repo/lib/js/index.ts";

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

const TEMPLATES: readonly { readonly id: string; readonly tex: string }[] = [
  { id: "report", tex: "template/report/report.tex" },
  { id: "paper", tex: "template/paper/paper.tex" },
  { id: "flyer", tex: "template/flyer/flyer.tex" },
  { id: "forschungsbericht", tex: "template/zukunftbau/forschungsbericht.tex" },
  { id: "zwischenbericht", tex: "template/zukunftbau/zwischenbericht.tex" },
  { id: "kompaktbericht", tex: "template/zukunftbau/kompaktbericht.tex" },
];

const VIZ_GALLERY_DIR = join(printRoot, "template/viz-gallery");
const VIZ_TAXONOMY_PATH = join(printRoot, "asset/viz-taxonomy.md");

function vizTemplates(): readonly { readonly id: string; readonly tex: string }[] {
  if (!existsSync(VIZ_GALLERY_DIR)) return [];
  return readdirSync(VIZ_GALLERY_DIR)
    .filter((name) => name.endsWith(".tex") && !name.includes("-dark"))
    .sort()
    .map((name) => ({ id: basename(name, ".tex"), tex: `template/viz-gallery/${name}` }));
}

function parseVizTaxonomyLeaves(md: string): string[] {
  const leaves: string[] = [];
  let section = "";
  for (const line of md.split(/\n/)) {
    const header = line.match(/^##\s+(\d+)/);
    if (header) {
      section = header[1]!;
      continue;
    }
    const leaf = line.match(/^- .+ `([^`]+)` (?:mark|chart|layout|axis|scale)$/);
    if (leaf && section) leaves.push(`${section}/${leaf[1]}`);
  }
  return leaves;
}

function parseVizCovers(dir: string): Set<string> {
  const covers = new Set<string>();
  if (!existsSync(dir)) return covers;
  for (const name of readdirSync(dir).filter((file) => file.endsWith(".tex"))) {
    const source = readFileSync(join(dir, name), "utf8");
    for (const match of source.matchAll(/% viz-covers:\s+(\S+)/g)) covers.add(match[1]!);
  }
  return covers;
}

function assertVizCoverage(): { readonly leaves: number; readonly missing: readonly string[] } {
  const leaves = parseVizTaxonomyLeaves(readFileSync(VIZ_TAXONOMY_PATH, "utf8"));
  const covers = parseVizCovers(VIZ_GALLERY_DIR);
  return { leaves: leaves.length, missing: leaves.filter((leaf) => !covers.has(leaf)) };
}

const VIZ_API_COMMANDS = [
  "\\SemioVizChart",
  "\\SemioVizMark",
  "\\SemioVizPath",
  "\\SemioVizLayout",
  "\\SemioVizAxis",
  "\\SemioVizGrid",
  "\\SemioVizLegend",
  "\\SemioVizTable",
  "\\SemioVizRow",
  "\\SemioVizScale",
  "\\SemioVizChartKind",
  "\\SemioVizDemo",
  "\\SemioVizText",
  "VizFigure",
  "VizSection",
  "VizColumn",
] as const;

function vizGallerySources(): string {
  if (!existsSync(VIZ_GALLERY_DIR)) return "";
  return readdirSync(VIZ_GALLERY_DIR)
    .filter((name) => name.endsWith(".tex") && !name.includes("-dark"))
    .map((name) => readFileSync(join(VIZ_GALLERY_DIR, name), "utf8"))
    .join("\n");
}

function assertVizApi(): { readonly missing: readonly string[] } {
  const source = readFileSync(join(VIZ_GALLERY_DIR, "viz-api.tex"), "utf8");
  return { missing: VIZ_API_COMMANDS.filter((command) => !source.includes(command)) };
}

function inflatePdfStreamBody(body: Buffer): Buffer {
  try {
    return inflateSync(body);
  } catch {
    return inflateRawSync(body);
  }
}

function pdfStableHash(pdfPath: string): string {
  const raw = readFileSync(pdfPath).toString("binary");
  const inflated = raw.replace(/stream\r?\n([\s\S]*?)\r?\nendstream/g, (_all, body: string) => {
    try {
      return `stream\n${inflatePdfStreamBody(Buffer.from(body, "binary")).toString("binary")}\nendstream`;
    } catch {
      return `stream\n${body}\nendstream`;
    }
  });
  const text = inflated
    .replace(/\/CreationDate\s*\([^)]*\)/g, "")
    .replace(/\/ModDate\s*\([^)]*\)/g, "")
    .replace(/\/ID\s*\[[^\]]*\]/g, "")
    .replace(/\(D:[0-9+\-'Z]+\)/g, "")
    .replace(/\/Producer\s*\([^)]*\)/g, "")
    .replace(/\/Creator\s*\([^)]*\)/g, "")
    .replace(/<x:xmpmeta[\s\S]*?<\/x:xmpmeta>/g, "");
  return createHash("sha256").update(text, "binary").digest("hex");
}

function deriveDarkTexSource(lightSource: string): string {
  if (/\btheme=dark\b/.test(lightSource)) throw new Error("source already has theme=dark; use the light source as canonical");
  const withDarkTheme = /\btheme=light\b/.test(lightSource) ? lightSource.replace(/\btheme=light\b/, "theme=dark") : lightSource.replace(/(\\documentclass\[[^\]]*)\]/, "$1,theme=dark]");
  if (withDarkTheme === lightSource) throw new Error("cannot derive dark variant: \\documentclass has no options to append theme=dark");
  const body = withDarkTheme.replace(/(asset\/logo\/[^,\s}\]]+?)(?<!-dark)(\.(?:png|jpe?g|webp|pdf))(?=[,\s}\]])/gi, "$1-dark$2");
  const banner = "% Generated by print/script.ts deriveDarkTexSource — do not edit or commit.\n";
  const magic = body.match(/^(?:% !TEX[^\n]*\n)*/)?.[0] ?? "";
  return `${magic}${banner}${body.slice(magic.length)}`;
}

function templatePdfNames(texRel: string): { readonly light: string; readonly dark: string } {
  const base = basename(texRel, ".tex");
  return { light: `${base}.pdf`, dark: `${base}-dark.pdf` };
}

type PaintRef = {
  readonly token?: string;
  readonly hex?: string;
  readonly alpha?: number;
  readonly mix?: readonly [string, string, number];
};

type Tokens = {
  readonly colors: Record<string, string>;
  readonly spacing: Record<string, string>;
  readonly strokes?: Record<string, number | number[]>;
  readonly opacities?: {
    readonly glassPanelAlpha?: number;
  };
  readonly appearances?: Record<string, Record<string, Record<string, PaintRef>>>;
  readonly metrics?: {
    readonly chrome?: {
      readonly controlHeightUiSpacing?: number;
      readonly paddingStandardUiSpacing?: number;
      readonly navbarHeightUiSpacing?: number;
      readonly footerHeightUiSpacing?: number;
      readonly glassPanelBlurPx?: number;
      readonly glassSaturate?: number;
    };
    readonly typography?: {
      readonly textXsPx?: number;
      readonly text2xsPx?: number;
      readonly textSmPx?: number;
    };
  };
};

type PanelManifestEntry = {
  readonly id: string;
  readonly page: number;
  readonly xPt: number;
  readonly yPt: number;
  readonly wPt: number;
  readonly hPt: number;
};

const PANEL_GLASS_DIR = ".semio-panel-glass";
const PANEL_RENDER_DPI = 200;
const PDF_PT_PER_INCH = 72;

const CHROME_PAINT_KEYS = ["base", "window", "canvas", "panel", "borderNormal", "borderEmphasized", "activeBase", "activeForeground", "foreground", "accent"] as const;

function parseHex6(hex: string): [number, number, number] {
  const s = hex.trim().replace(/^#/, "");
  if (s.length === 3) {
    return [Number.parseInt(s[0]! + s[0], 16), Number.parseInt(s[1]! + s[1], 16), Number.parseInt(s[2]! + s[2], 16)];
  }
  const v = Number.parseInt(s, 16);
  return [(v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff];
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

function resolvePaint(colors: Record<string, string>, ref: PaintRef): string {
  if (ref.mix) {
    const [a, b, ratio] = ref.mix;
    const bHex = b === "transparent" ? "#000000" : colors[b];
    if (!bHex) throw new Error(`tokens.colors[${b}] missing`);
    return blendHex(colors[a]!, bHex, ratio);
  }
  if (ref.hex) return ref.hex;
  if (ref.token) {
    const v = colors[ref.token];
    if (!v) throw new Error(`tokens.colors[${ref.token}] missing`);
    return v;
  }
  throw new Error("paint ref needs token, hex, or mix");
}

function chromePaintKeyToLatex(key: string): string {
  return key.replace(/[A-Z]/g, (m) => `-${m.toLowerCase()}`);
}

function remFactor(rem: string): number {
  const match = rem.match(/^([\d.]+)rem$/);
  if (!match) return Number.parseFloat(rem) || 0;
  return Number.parseFloat(match[1]!);
}

function colorKeyToLatex(key: string): string {
  return `semio-${key.replaceAll("_", "-")}`;
}

function panelManifestPath(outDir: string, jobname: string): string {
  return join(outDir, `${jobname}.panels`);
}

function panelGlassDir(workDir: string, jobname: string): string {
  return join(workDir, PANEL_GLASS_DIR, jobname);
}

function detectTheme(texAbs: string): "light" | "dark" {
  const source = readFileSync(texAbs, "utf8");
  return /\btheme\s*=\s*dark\b/.test(source) ? "dark" : "light";
}

function parsePt(value: string): number {
  return Number.parseFloat(value.replace(/pt$/i, ""));
}

function parsePanelManifest(manifestPath: string): PanelManifestEntry[] {
  const text = readFileSync(manifestPath, "utf8").trim();
  if (!text) return [];
  return text.split("\n").map((line) => {
    const [id, page, xPt, yPt, wPt, hPt] = line.split(";").map((part) => part.trim());
    if (!id || !page || !xPt || !yPt || !wPt || !hPt) throw new Error(`invalid panel manifest line: ${line}`);
    return {
      id,
      page: Number.parseInt(page, 10),
      xPt: parsePt(xPt),
      yPt: parsePt(yPt),
      wPt: parsePt(wPt),
      hPt: parsePt(hPt),
    };
  });
}

function loadTokens(): Tokens {
  return JSON.parse(readFileSync(tokensPath, "utf8")) as Tokens;
}

function panelGlassTintHex(theme: "light" | "dark"): string {
  const tokens = loadTokens();
  const paint = tokens.appearances?.[theme]?.chrome?.panel;
  if (!paint) throw new Error(`tokens.appearances.${theme}.chrome.panel missing`);
  return resolvePaint(tokens.colors, paint);
}

function loadPdfjsNapiCanvas(): { createCanvas: (width: number, height: number) => import("@napi-rs/canvas").Canvas } {
  const pdfjsEntry = fileURLToPath(new URL("../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
  return createRequire(pdfjsEntry)("@napi-rs/canvas");
}

/** @emoji 🪟 Rasterizes pass-1 PDF pages and writes frosted glass PNGs for each panel manifest entry. */
export async function renderPanelGlass(options: { readonly manifestPath: string; readonly pdfPath: string; readonly glassDir: string; readonly theme: "light" | "dark" }): Promise<void> {
  const entries = parsePanelManifest(options.manifestPath);
  if (entries.length === 0) return;
  const tokens = loadTokens();
  const glassPanelAlpha = tokens.opacities?.glassPanelAlpha ?? 0.58;
  const glassPanelBlurPx = tokens.metrics?.chrome?.glassPanelBlurPx ?? 40;
  const glassSaturate = tokens.metrics?.chrome?.glassSaturate ?? 1.45;
  const panelTint = panelGlassTintHex(options.theme);
  const [tintR, tintG, tintB] = parseHex6(panelTint);
  const tintAlpha = Math.round(glassPanelAlpha * 255);
  const renderScale = PANEL_RENDER_DPI / PDF_PT_PER_INCH;
  const blurSigma = Math.max(1.5, (glassPanelBlurPx * renderScale) / 9);

  const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
  const { createCanvas } = loadPdfjsNapiCanvas();
  const sharp = (await import("sharp")).default;

  const pdfBytes = readFileSync(options.pdfPath);
  const doc = await pdfjs.getDocument({ data: new Uint8Array(pdfBytes), useSystemFonts: true }).promise;
  const pageCache = new Map<number, { readonly png: Buffer; readonly pageWidthPt: number; readonly pageHeightPt: number }>();

  mkdirSync(options.glassDir, { recursive: true });

  for (const entry of entries) {
    let pageRender = pageCache.get(entry.page);
    if (!pageRender) {
      const page = await doc.getPage(entry.page);
      const viewport = page.getViewport({ scale: renderScale });
      const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
      const context = canvas.getContext("2d");
      if (!context) throw new Error("panel glass canvas 2d unavailable");
      await page.render({ canvas, canvasContext: context, viewport }).promise;
      const pageWidthPt = (page.view[2] ?? 0) - (page.view[0] ?? 0);
      const pageHeightPt = (page.view[3] ?? 0) - (page.view[1] ?? 0);
      pageRender = { png: canvas.toBuffer("image/png"), pageWidthPt, pageHeightPt };
      pageCache.set(entry.page, pageRender);
    }

    const cropLeft = Math.max(0, Math.round(entry.xPt * renderScale));
    const cropTop = Math.max(0, Math.round((pageRender.pageHeightPt - entry.yPt - entry.hPt) * renderScale));
    const cropWidth = Math.max(1, Math.round(entry.wPt * renderScale));
    const cropHeight = Math.max(1, Math.round(entry.hPt * renderScale));

    const tintRaw = Buffer.alloc(cropWidth * cropHeight * 4);
    for (let i = 0; i < cropWidth * cropHeight; i++) {
      const offset = i * 4;
      tintRaw[offset] = tintR;
      tintRaw[offset + 1] = tintG;
      tintRaw[offset + 2] = tintB;
      tintRaw[offset + 3] = tintAlpha;
    }
    const processed = await sharp(pageRender.png)
      .extract({
        left: Math.min(cropLeft, Math.max(0, Math.round(pageRender.pageWidthPt * renderScale) - 1)),
        top: Math.min(cropTop, Math.max(0, Math.round(pageRender.pageHeightPt * renderScale) - 1)),
        width: cropWidth,
        height: cropHeight,
      })
      .removeAlpha()
      .blur(blurSigma)
      .modulate({ saturation: glassSaturate })
      .composite([
        {
          input: tintRaw,
          raw: { width: cropWidth, height: cropHeight, channels: 4 },
          blend: "over",
        },
      ])
      .png({ compressionLevel: 9 })
      .toBuffer();

    writeFileSync(join(options.glassDir, `${entry.id}.png`), processed);
    console.log(`[DEBUG] print panel glass ${entry.id} page ${entry.page}`);
  }
  writeFileSync(join(options.glassDir, ".ready"), "");
}

function resetPanelArtifacts(workDir: string, outDir: string, jobname: string): void {
  const manifestPath = panelManifestPath(outDir, jobname);
  const glassDir = panelGlassDir(workDir, jobname);
  if (existsSync(manifestPath)) rmSync(manifestPath);
  if (existsSync(glassDir)) rmSync(glassDir, { recursive: true });
}

async function compilePrintDocumentWithPanels(tectonic: string, texAbs: string, outDir: string, workDir = dirname(texAbs)): Promise<void> {
  const jobname = basename(texAbs, ".tex");
  resetPanelArtifacts(workDir, outDir, jobname);
  compilePrintDocument(tectonic, texAbs, outDir, workDir);
  const manifestPath = panelManifestPath(outDir, jobname);
  if (existsSync(manifestPath)) {
    const entries = parsePanelManifest(manifestPath);
    if (entries.length > 0) {
      const pdfPath = join(outDir, `${jobname}.pdf`);
      await renderPanelGlass({
        manifestPath,
        pdfPath,
        glassDir: panelGlassDir(workDir, jobname),
        theme: detectTheme(texAbs),
      });
    }
  }
  compilePrintDocument(tectonic, texAbs, outDir, workDir);
}

function remToEm(rem: string): string {
  const match = rem.match(/^([\d.]+)rem$/);
  if (!match) return rem;
  return `${match[1]}em`;
}

/** @emoji 🎨 Writes `tex/semio-tokens.sty` from {@link ui/styling/tokens.json}. */
export function emitSemioTokensSty(): void {
  const tokens = JSON.parse(readFileSync(tokensPath, "utf8")) as Tokens;
  const lines: string[] = ["% Generated from ui/styling/tokens.json — run `bun ./script.ts generate`.", "\\NeedsTeXFormat{LaTeX2e}", "\\ProvidesPackage{semio-tokens}[2026/07/06 v0.1.0 semio design tokens]", "\\RequirePackage{xcolor}", ""];
  for (const [key, hex] of Object.entries(tokens.colors)) {
    const name = colorKeyToLatex(key);
    const html = hex.replace(/^#/, "");
    lines.push(`\\definecolor{${name}}{HTML}{${html}}`);
  }
  lines.push("");
  const unitFactor = remFactor(tokens.spacing.compact ?? "0.2rem");
  const unitEm = `${+unitFactor.toFixed(3)}em`;
  lines.push(`\\newcommand{\\semio@spacing@unit}{${unitEm}}`);
  lines.push(`\\newcommand{\\semio@spacing@single}{${unitEm}}`);
  lines.push(`\\newcommand{\\semio@spacing@double}{${+(unitFactor * 2).toFixed(3)}em}`);
  const hairline = typeof tokens.strokes?.chromeBorderHairline === "number" ? tokens.strokes.chromeBorderHairline * 0.75 : typeof tokens.strokes?.gridLarge === "number" ? tokens.strokes.gridLarge * 0.75 : 0.75;
  const strokeDefault = typeof tokens.strokes?.chromeBorderDefault === "number" ? tokens.strokes.chromeBorderDefault * 0.75 : typeof tokens.strokes?.edgeBase === "number" ? tokens.strokes.edgeBase * 0.75 : 1.5;
  const strokeFocus = typeof tokens.strokes?.chromeBorderFocus === "number" ? tokens.strokes.chromeBorderFocus * 0.75 : typeof tokens.strokes?.dagNodeSelected === "number" ? tokens.strokes.dagNodeSelected * 0.75 : 1.75;
  lines.push(`\\newcommand{\\semio@stroke@hairline}{${hairline}pt}`);
  lines.push(`\\newcommand{\\semio@stroke@default}{${strokeDefault}pt}`);
  lines.push(`\\newcommand{\\semio@stroke@focus}{${strokeFocus}pt}`);
  lines.push("");
  const chromeMetrics = tokens.metrics?.chrome;
  if (chromeMetrics) {
    const titleBarHeight = unitFactor * (chromeMetrics.controlHeightUiSpacing ?? 7);
    const chromePadding = unitFactor * (chromeMetrics.paddingStandardUiSpacing ?? 1);
    const navbarHeight = unitFactor * (chromeMetrics.navbarHeightUiSpacing ?? 9);
    const footerHeight = unitFactor * (chromeMetrics.footerHeightUiSpacing ?? 9);
    lines.push(`\\newcommand{\\semio@chrome@titlebar@height}{${+titleBarHeight.toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@padding}{${+chromePadding.toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@navbar@height}{${+navbarHeight.toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@footer@height}{${+footerHeight.toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@icon@scale}{1}`);
    lines.push(`\\newcommand{\\semio@chrome@icon@scale@footer}{1}`);
  }
  const typography = tokens.metrics?.typography;
  if (typography) {
    // 🎨 Absolute pt (not em) so chip/body text height is identical regardless of the
    // ambient font size at the \fontsize call site — only chip width adapts to content.
    const chipFontPt = (typography.text2xsPx ?? 9.6) * 0.75;
    const bodyFontPt = (typography.textSmPx ?? 12.8) * 0.75;
    lines.push(`\\newcommand{\\semio@chrome@font@chip}{${+chipFontPt.toFixed(3)}pt}`);
    lines.push(`\\newcommand{\\semio@chrome@font@body}{${+bodyFontPt.toFixed(3)}pt}`);
  }
  lines.push("");
  for (const themeName of ["light", "dark"] as const) {
    const chrome = tokens.appearances?.[themeName]?.chrome;
    if (!chrome) continue;
    for (const key of CHROME_PAINT_KEYS) {
      const paint = chrome[key];
      if (!paint) continue;
      const hex = resolvePaint(tokens.colors, paint).replace(/^#/, "");
      const latexKey = chromePaintKeyToLatex(key);
      lines.push(`\\definecolor{semio-chrome-${themeName}-${latexKey}}{HTML}{${hex}}`);
    }
  }
  lines.push("");
  mkdirSync(texDir, { recursive: true });
  const content = lines.join("\n");
  // 🔒 Only rewrite when the tokens actually changed — keeps watchers from
  // retriggering and skips the write entirely on the common no-op call.
  if (existsSync(tokensOut) && readFileSync(tokensOut, "utf8") === content) return;
  // 🔒 Concurrent document builds all call this. A direct writeFileSync let a
  // tectonic process reading semio-tokens.sty observe a torn write from another
  // build still in progress — surfaced as "semio.cls: Missing \begin{document}"
  // deep in an unrelated document's log. Write to a per-process temp file first,
  // then rename over the target: renameSync replaces the destination in one
  // filesystem operation, so a concurrent reader always sees a complete file.
  const tmpOut = `${tokensOut}.${process.pid}.tmp`;
  writeFileSync(tmpOut, content, "utf8");
  renameSync(tmpOut, tokensOut);
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

const DARK_TEX_DIR = ".semio-dark";

function tectonicEnv(workDir?: string): NodeJS.ProcessEnv {
  const sep = process.platform === "win32" ? ";" : ":";
  const work = workDir ? `${workDir}//${sep}${workDir}${sep}` : "";
  return {
    ...process.env,
    TEXINPUTS: `${work}${texDir}//${sep}${texDir}${sep}`,
  };
}

function clearStaleTocAuxFiles(workDir: string, outDir: string, jobname: string): void {
  for (const ext of [".sctoc", ".semio-toc", ".register-toc", ".window-breaks"]) {
    for (const dir of [workDir, outDir]) {
      const path = join(dir, `${jobname}${ext}`);
      if (existsSync(path)) rmSync(path);
    }
  }
}

function compilePrintDocument(tectonic: string, texAbs: string, outDir: string, workDir = dirname(texAbs)): void {
  emitSemioTokensSty();
  const texFile = relative(workDir, texAbs).replaceAll("\\", "/");
  const jobname = basename(texAbs, ".tex");
  mkdirSync(outDir, { recursive: true });
  clearStaleTocAuxFiles(workDir, outDir, jobname);
  clearStaleTocAuxFiles(dirname(texAbs), outDir, jobname);
  const args = ["--keep-logs", "--keep-intermediates", "--synctex", "--reruns", "2", "-Z", `search-path=${texDir}`, "-Z", `search-path=${workDir}`, "--outdir", outDir, texFile];
  const env = tectonicEnv(workDir);
  const build = spawnSync(tectonic, args, { cwd: workDir, stdio: "inherit", env });
  if (build.status !== 0) throw new Error(`tectonic build failed for ${texAbs}`);
  const pdf = join(outDir, `${basename(texAbs, ".tex")}.pdf`);
  if (!existsSync(pdf)) throw new Error(`missing PDF output: ${pdf}`);
  console.log(`[DEBUG] print built ${relative(repoRoot, pdf)}`);
}

function writeDerivedDarkTex(lightTexAbs: string): string {
  const lightDir = dirname(lightTexAbs);
  const darkDir = join(lightDir, DARK_TEX_DIR);
  mkdirSync(darkDir, { recursive: true });
  const darkTexAbs = join(darkDir, `${basename(lightTexAbs, ".tex")}-dark.tex`);
  writeFileSync(darkTexAbs, deriveDarkTexSource(readFileSync(lightTexAbs, "utf8")), "utf8");
  return darkTexAbs;
}

async function compileLightAndDark(tectonic: string, texAbs: string, outDir: string): Promise<void> {
  const lightDir = dirname(texAbs);
  await compilePrintDocumentWithPanels(tectonic, texAbs, outDir, lightDir);
  const darkTexAbs = writeDerivedDarkTex(texAbs);
  await compilePrintDocumentWithPanels(tectonic, darkTexAbs, outDir, lightDir);
}

/** @emoji 🖨️ Compiles one `.tex` file with the semio print search path and Tectonic. */
export async function buildPrintDocument(texAbs: string, outDir = join(dirname(texAbs), "dist")): Promise<void> {
  emitSemioTokensSty();
  const tectonic = await ensureTectonic();
  await compileLightAndDark(tectonic, texAbs, outDir);
}

async function compileTemplate(tectonic: string, template: (typeof TEMPLATES)[number]): Promise<void> {
  const texAbs = join(printRoot, template.tex);
  await compileLightAndDark(tectonic, texAbs, distDir);
}

function resolveTemplates(filter: string[]): (typeof TEMPLATES)[number][] {
  if (filter.length === 0) return [...TEMPLATES];
  const wanted = new Set(filter.map((id) => id.replace(/-dark$/, "")));
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
  const rebuild = async () => {
    for (const template of templates) {
      try {
        await compileTemplate(tectonic, template);
      } catch (error) {
        console.error(`[DEBUG] print watch rebuild failed for ${template.id}:`, error);
      }
    }
  };
  await rebuild();
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
        if (abs.includes(`${DARK_TEX_DIR}`) || /-dark\.tex$/i.test(abs)) return;
        if (!/\.(tex|sty|cls|ttf|json)$/i.test(abs)) return;
        try {
          const mtime = statSync(abs).mtimeMs;
          if (mtimes.get(abs) === mtime) return;
          touch(abs);
          void rebuild();
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
    if (segments[0] === "viz") {
      const filter = segments.slice(1);
      const templates = vizTemplates().filter((template) => filter.length === 0 || filter.includes(template.id) || filter.includes(template.id.replace(/^viz-/, "")));
      if (templates.length === 0) throw new Error(`unknown viz gallery id(s): ${filter.join(", ")}`);
      for (const template of templates) await compileTemplate(tectonic, template);
      return;
    }
    for (const template of resolveTemplates(segments)) await compileTemplate(tectonic, template);
  }
}

class WatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "viz") {
      await watchVizTemplates(segments.slice(1));
      return;
    }
    await watchTemplates(segments);
  }
}

async function watchVizTemplates(filter: string[]): Promise<void> {
  const templates = vizTemplates().filter((template) => filter.length === 0 || filter.includes(template.id) || filter.includes(template.id.replace(/^viz-/, "")));
  if (templates.length === 0) throw new Error(`unknown viz gallery id(s): ${filter.join(", ")}`);
  const tectonic = await ensureTectonic();
  emitSemioTokensSty();
  const rebuild = async () => {
    for (const template of templates) {
      try {
        await compileTemplate(tectonic, template);
      } catch (error) {
        console.error(`[DEBUG] print viz watch rebuild failed for ${template.id}:`, error);
      }
    }
  };
  await rebuild();
  console.log(`[DEBUG] print watching viz ${templates.map((template) => template.id).join(", ")}`);
}

//#region ⏱️Test
/** ⏱️Fundamental unit tests (pure functions, no Tectonic/network/fs I/O) always run; the full 12-PDF Tectonic build only runs at the `long` level and above. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "viz") {
      const mode = segments[1] ?? "coverage";
      if (mode !== "quick" && mode !== "coverage" && mode !== "full") throw new Error(`unknown viz test mode: ${mode}`);
      this.runUnitTests();
      if (mode !== "quick") this.runVizCoverage();
      if (mode === "full") await this.runVizBuild();
      return;
    }
    const { level } = resolveTestLevel(segments);
    this.runUnitTests();
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) await this.runTectonicBuild();
  }

  private runUnitTests(): void {
    //#region parseHex6 / blendHex
    assert.deepEqual(parseHex6("#ff0080"), [255, 0, 128]);
    assert.deepEqual(parseHex6("abc"), [170, 187, 204]);
    assert.equal(blendHex("#ffffff", "#000000", 0.5), "#808080");
    assert.equal(blendHex("#ff0000", "#0000ff", 1), "#ff0000");
    assert.equal(blendHex("#ff0000", "#0000ff", 0), "#0000ff");
    //#endregion

    //#region remFactor / remToEm
    assert.equal(remFactor("1.5rem"), 1.5);
    assert.equal(remFactor("0.2rem"), 0.2);
    assert.equal(remToEm("2rem"), "2em");
    assert.equal(remToEm("garbage"), "garbage");
    //#endregion

    //#region colorKeyToLatex / chromePaintKeyToLatex
    assert.equal(colorKeyToLatex("accent_strong"), "semio-accent-strong");
    assert.equal(chromePaintKeyToLatex("activeForeground"), "active-foreground");
    assert.equal(chromePaintKeyToLatex("base"), "base");
    //#endregion

    //#region parsePt / parsePanelManifest
    assert.equal(parsePt("12.5pt"), 12.5);
    const manifestPath = join(distDir, ".semio-print-test.panels");
    mkdirSync(distDir, { recursive: true });
    writeFileSync(manifestPath, "panel-1;2;10pt;20pt;100pt;200pt\n");
    assert.deepEqual(parsePanelManifest(manifestPath), [{ id: "panel-1", page: 2, xPt: 10, yPt: 20, wPt: 100, hPt: 200 }]);
    writeFileSync(manifestPath, "");
    assert.deepEqual(parsePanelManifest(manifestPath), []);
    rmSync(manifestPath);
    //#endregion

    //#region resolvePaint
    const colors = { accent: "#112233", base: "#000000" };
    assert.equal(resolvePaint(colors, { hex: "#abcdef" }), "#abcdef");
    assert.equal(resolvePaint(colors, { token: "accent" }), "#112233");
    assert.equal(resolvePaint(colors, { mix: ["accent", "base", 0] }), "#000000");
    assert.throws(() => resolvePaint(colors, {}));
    assert.throws(() => resolvePaint(colors, { token: "missing" }));
    //#endregion

    //#region deriveDarkTexSource
    const lightSource = "\\documentclass[a4paper]{article}\n\\includegraphics{asset/logo/mark.png}\n";
    const darkSource = deriveDarkTexSource(lightSource);
    assert.match(darkSource, /theme=dark/);
    assert.match(darkSource, /asset\/logo\/mark-dark\.png/);
    assert.match(darkSource, /^% Generated by print\/script\.ts deriveDarkTexSource/);
    const withMagic = "% !TEX program = tectonic\n% !TEX root = report.tex\n\\documentclass[theme=light]{semio}\n";
    const darkWithMagic = deriveDarkTexSource(withMagic);
    assert.match(darkWithMagic, /^% !TEX program = tectonic\n% !TEX root = report\.tex\n% Generated by print\/script\.ts deriveDarkTexSource/);
    assert.match(darkWithMagic, /theme=dark/);
    assert.throws(() => deriveDarkTexSource("\\documentclass[theme=dark]{article}"));
    assert.throws(() => deriveDarkTexSource("\\documentclass{article}"));
    //#endregion

    //#region templatePdfNames / resolveTemplates
    assert.deepEqual(templatePdfNames("template/report/report.tex"), { light: "report.pdf", dark: "report-dark.pdf" });
    assert.equal(resolveTemplates([]).length, TEMPLATES.length);
    assert.equal(resolveTemplates(["report", "report-dark"]).length, 1);
    assert.throws(() => resolveTemplates(["not-a-template"]));
    //#endregion

    //#region viz coverage
    const taxonomy = readFileSync(VIZ_TAXONOMY_PATH, "utf8");
    const vizLeaves = parseVizTaxonomyLeaves(taxonomy);
    assert.ok(vizLeaves.includes("0/dot"));
    assert.ok(vizLeaves.includes("1/vertical-bar-chart"));
    assert.ok(vizLeaves.includes("76/charts"));
    assert.ok(!vizLeaves.includes("7/trees"));
    assert.ok(vizLeaves.length > 1500);
    assert.ok(vizLeaves.length < 2000);
    assert.equal(new Set(vizLeaves.map((leaf) => leaf.split("/")[0])).size, 80);
    assert.equal(new Set(vizLeaves).size, vizLeaves.length);
    const vizCovers = parseVizCovers(VIZ_GALLERY_DIR);
    assert.ok(vizCovers.has("0/dot"));
    assert.ok(vizCovers.has("1/vertical-bar-chart"));
    assert.ok(vizGallerySources().includes("\\SemioVizDemo"));
    //#endregion

    //#region pdfStableHash
    const pdfA = join(distDir, ".semio-print-test-a.pdf");
    const pdfB = join(distDir, ".semio-print-test-b.pdf");
    const streamA = deflateSync(Buffer.from("/ID[<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa><bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb>] viz-content"));
    const streamB = deflateSync(Buffer.from("/ID[<cccccccccccccccccccccccccccccccc><dddddddddddddddddddddddddddddddd>] viz-content"));
    const wrap = (id: string, stream: Buffer) =>
      Buffer.concat([
        Buffer.from(`%PDF-\n8 0 obj\n<</Type/ObjStm/Filter/FlateDecode>>\nstream\n`, "binary"),
        stream,
        Buffer.from(`\nendstream\nendobj\n61 0 obj\n<</Type/XRef/ID[${id}]>>\nendobj\n`, "binary"),
      ]);
    writeFileSync(pdfA, wrap("<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa><bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb>", streamA));
    writeFileSync(pdfB, wrap("<cccccccccccccccccccccccccccccccc><dddddddddddddddddddddddddddddddd>", streamB));
    assert.equal(pdfStableHash(pdfA), pdfStableHash(pdfB));
    rmSync(pdfA);
    rmSync(pdfB);
    //#endregion

    //#region Window layout
    const windowSource = readFileSync(join(texDir, "semio-window.sty"), "utf8");
    const tableSource = readFileSync(join(texDir, "semio-table.sty"), "utf8");
    const componentsSource = readFileSync(join(texDir, "semio-components.sty"), "utf8");
    const classSource = readFileSync(join(texDir, "semio.cls"), "utf8");
    const headingTrackSource = windowSource.slice(windowSource.indexOf("\\newcount\\semio@chrome@heading@level"), windowSource.indexOf("\\newsavebox{\\semio@window@cap@slot}"));
    assert.match(classSource, /\\RequirePackage\[style=\\semio@citestyle,backend=bibtex,sorting=nyt,backref=true\]\{biblatex\}/);
    assert.match(componentsSource, /\\NewDocumentCommand\{\\makecoverpages\}\{\}\{%[\s\S]*?\\newgeometry\{[^}]+\}\s*\\thispagestyle\{empty\}/);
    assert.match(headingTrackSource, /\\semio@chrome@heading@level=99\\relax/);
    assert.match(headingTrackSource, /\\ifnum\\semio@chrome@heading@candidate@level<\\semio@chrome@heading@level[\s\S]*?\\global\\semio@chrome@heading@level=\\semio@chrome@heading@candidate@level/);
    assert.match(headingTrackSource, /\\ifnum\\semio@chrome@heading@candidate@level=\\semio@chrome@heading@level[\s\S]*?\\semio@chrome@heading@set\{#2\}%[\s\S]*?\\else[\s\S]*?\\global\\let\\semio@nav@short@pending\\relax[\s\S]*?\\expandafter\\markright\\expandafter\{\\semio@chrome@heading\}/);
    assert.match(tableSource, /\\newcommand\{\\semio@table@long@head@copy\}\{%[\s\S]*?\\ifsemio@table@long@pageparts[\s\S]*?\\global\\advance\\semio@table@long@part\\@ne[\s\S]*?\\semio@table@long@part@overlay[\s\S]*?\\copy\\LT@head/);
    assert.match(tableSource, /\\patchcmd\{\\LT@start\}[\s\S]*?\{\\copy\\LT@head\}[\s\S]*?\{\\semio@table@long@head@copy\}/);
    assert.match(tableSource, /\\end\{longtable\}%\s*\\semio@table@long@parts@record/);
    assert.match(tableSource, /\\newcommand\{\\semio@table@long@parts@record\}\{%\s*\\ifsemio@table@long@pageparts[\s\S]*?\\semio@window@break@record/);
    assert.match(tableSource, /\\newcommand\{\\SemioTableLong\}[\s\S]*?\\semio@table@long@pagepartstrue[\s\S]*?\\semio@table@long@render[\s\S]*?\\semio@table@long@pagepartsfalse/);
    assert.match(windowSource, /\\semio_window_vskip_stroke_hairline: \{[\s\S]*?\\vskip\\dimexpr-\\semio@stroke@hairline-5\.75pt\\relax/);
    assert.match(windowSource, /overlay~unbroken=\{\\semio@window@break@record\{1\}/);
    assert.match(windowSource, /overlay~first=\{\\semio@window@frame@bottom@stroke\}/);
    assert.match(windowSource, /overlay~middle=\{[\s\S]*?\\semio@window@frame@bottom@stroke/);
    assert.match(windowSource, /bottomrule~at~break=\\semio@stroke@hairline/);
    assert.match(windowSource, /toprule~at~break=0pt/);
    assert.match(windowSource, /\\semio@window@header@invoke@tcb/);
    assert.match(windowSource, /\\semio@heading@cap@muted@open/);
    assert.match(windowSource, /toprule=0pt/);
    assert.match(tableSource, /\\semio@table@long@title@chrome@row[\s\S]*?\\semio@window@header@invoke/);
    assert.match(tableSource, /\\NewDocumentCommand \\SemioTableHeaderRow \{ m \} \{[\s\S]*?\\semio@table@row@sep/);
    assert.match(tableSource, /\\semio_table_long_header_build:nn #1#2 \{[\s\S]*?\\semio@table@long@header@left@cell[\s\S]*?\\clist_item:nn \{#1\} \{1\}/);
    assert.doesNotMatch(tableSource, /\\newcommand\{\\semio@table@long@header@(repeat|continuation)@three\}\[3\]\{%\s*\\hhline/);
    //#endregion

    console.log("print: unit tests passed");
  }

  /** 🖨️Full Tectonic build of every template in light+dark — needs the Tectonic toolchain and font downloads. */
  private async runTectonicBuild(): Promise<void> {
    emitSemioTokensSty();
    await fetchPrintFonts();
    const tectonic = await ensureTectonic();
    for (const template of TEMPLATES) await compileTemplate(tectonic, template);
    for (const template of TEMPLATES) {
      for (const pdf of Object.values(templatePdfNames(template.tex))) {
        const pdfPath = join(distDir, pdf);
        if (!existsSync(pdfPath)) throw new Error(`test missing PDF: ${pdfPath}`);
      }
    }
    console.log(`print: all ${TEMPLATES.length * 2} template PDFs built`);
  }

  private runVizCoverage(): void {
    const coverage = assertVizCoverage();
    if (coverage.missing.length > 0) throw new Error(`viz coverage missing ${coverage.missing.length}/${coverage.leaves}: ${coverage.missing.join(", ")}`);
    const api = assertVizApi();
    if (api.missing.length > 0) throw new Error(`viz public API unused in galleries: ${api.missing.join(", ")}`);
    console.log(`print: viz coverage ${coverage.leaves}/${coverage.leaves} leaves, API ${VIZ_API_COMMANDS.length}/${VIZ_API_COMMANDS.length}`);
  }

  private async runVizBuild(): Promise<void> {
    emitSemioTokensSty();
    await fetchPrintFonts();
    const tectonic = await ensureTectonic();
    const templates = vizTemplates();
    if (templates.length === 0) throw new Error("no viz gallery templates");
    for (const template of templates) await compileTemplate(tectonic, template);
    for (const template of templates) {
      for (const pdf of [`${template.id}.pdf`, `${template.id}-dark.pdf`]) {
        const pdfPath = join(distDir, pdf);
        if (!existsSync(pdfPath)) throw new Error(`test missing PDF: ${pdfPath}`);
      }
    }
    const probe = templates.find((template) => template.id === "viz-api") ?? templates.find((template) => template.id === "viz-19") ?? templates[0]!;
    const pdfPath = join(distDir, `${probe.id}.pdf`);
    await compileTemplate(tectonic, probe);
    const magic = readFileSync(pdfPath).subarray(0, 5).toString("binary");
    if (magic !== "%PDF-" || statSync(pdfPath).size < 1024) throw new Error(`viz gallery ${probe.id} did not produce a PDF`);
    const hash = pdfStableHash(pdfPath);
    await compileTemplate(tectonic, probe);
    const hashAgain = pdfStableHash(pdfPath);
    if (hash !== hashAgain) throw new Error(`viz gallery ${probe.id} hash drifted (${hash.slice(0, 12)} vs ${hashAgain.slice(0, 12)})`);
    console.log(`print: all ${templates.length * 2} viz gallery PDFs built (deterministic hash=${hash.slice(0, 12)})`);
  }
}
//#endregion ⏱️Test

const router = new ScriptRouter(printRoot).register("generate", GenerateScript).register("fonts", FontsScript).register("build", BuildScript).register("watch", WatchScript).register("test", TestScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
}
