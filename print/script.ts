#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./script.ts generate|fonts|build|watch|test`. */
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { arch, platform } from "node:os";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
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

function deriveDarkTexSource(lightSource: string): string {
  if (/\btheme=dark\b/.test(lightSource)) throw new Error("source already has theme=dark; use the light source as canonical");
  const withDarkTheme = /\btheme=light\b/.test(lightSource)
    ? lightSource.replace(/\btheme=light\b/, "theme=dark")
    : lightSource.replace(/(\\documentclass\[[^\]]*)\]/, "$1,theme=dark]");
  if (withDarkTheme === lightSource) throw new Error("cannot derive dark variant: \\documentclass has no options to append theme=dark");
  return withDarkTheme.replace(
    /(asset\/logo\/[^,\s}\]]+?)(?<!-dark)(\.(?:png|jpe?g|webp|pdf))(?=[,\s}\]])/gi,
    "$1-dark$2",
  );
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

async function compilePrintDocumentWithPanels(tectonic: string, texAbs: string, outDir: string): Promise<void> {
  const workDir = dirname(texAbs);
  const jobname = basename(texAbs, ".tex");
  resetPanelArtifacts(workDir, outDir, jobname);
  compilePrintDocument(tectonic, texAbs, outDir);
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
  compilePrintDocument(tectonic, texAbs, outDir);
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
    TEXINPUTS: `${texDir}//${sep}${texDir}${sep}`,
  };
}

function clearStaleTocAuxFiles(workDir: string, outDir: string, jobname: string): void {
  for (const ext of [".sctoc", ".semio-toc", ".register-toc"]) {
    for (const dir of [workDir, outDir]) {
      const path = join(dir, `${jobname}${ext}`);
      if (existsSync(path)) rmSync(path);
    }
  }
}

function compilePrintDocument(tectonic: string, texAbs: string, outDir: string): void {
  emitSemioTokensSty();
  const workDir = dirname(texAbs);
  const texFile = basename(texAbs);
  const jobname = basename(texAbs, ".tex");
  mkdirSync(outDir, { recursive: true });
  clearStaleTocAuxFiles(workDir, outDir, jobname);
  const args = ["--keep-logs", "--keep-intermediates", "--synctex", "--reruns", "2", "-Z", `search-path=${texDir}`, "--outdir", outDir, texFile];
  const env = tectonicEnv();
  const build = spawnSync(tectonic, args, { cwd: workDir, stdio: "inherit", env });
  if (build.status !== 0) throw new Error(`tectonic build failed for ${texAbs}`);
  const pdf = join(outDir, `${basename(texAbs, ".tex")}.pdf`);
  if (!existsSync(pdf)) throw new Error(`missing PDF output: ${pdf}`);
  console.log(`[DEBUG] print built ${relative(repoRoot, pdf)}`);
}

async function compileLightAndDark(tectonic: string, texAbs: string, outDir: string): Promise<void> {
  await compilePrintDocumentWithPanels(tectonic, texAbs, outDir);
  const lightSource = readFileSync(texAbs, "utf8");
  const darkSource = deriveDarkTexSource(lightSource);
  const darkTexAbs = join(dirname(texAbs), `${basename(texAbs, ".tex")}-dark.tex`);
  writeFileSync(darkTexAbs, darkSource, "utf8");
  await compilePrintDocumentWithPanels(tectonic, darkTexAbs, outDir);
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
        if (/-dark\.tex$/i.test(abs)) return;
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
    for (const template of resolveTemplates(segments)) await compileTemplate(tectonic, template);
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
    for (const template of TEMPLATES) await compileTemplate(tectonic, template);
    for (const template of TEMPLATES) {
      for (const pdf of Object.values(templatePdfNames(template.tex))) {
        const pdfPath = join(distDir, pdf);
        if (!existsSync(pdfPath)) throw new Error(`test missing PDF: ${pdfPath}`);
      }
    }
    console.log(`print: all ${TEMPLATES.length * 2} template PDFs built`);
  }
}

const router = new ScriptRouter(printRoot).register("generate", GenerateScript).register("fonts", FontsScript).register("build", BuildScript).register("watch", WatchScript).register("test", TestScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
}
