#!/usr/bin/env bun
/** @emoji 🖼 Vendors Lucide SVGs into `icons/` and codegen JS, C#, and Python bindings. */
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// #endregion 🧲Header

// #region 🔌Adapters
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/src/index.ts";
// #endregion 🔌Adapters

//#region 🔖Constants
const LUCIDE_VERSION = "0.536.0";

/** @emoji 📋 Canonical icon ids (kebab-case) vendored from Lucide ISC assets. */
const VENDORED_ICON_IDS = [
  "alert-circle",
  "arrow-down",
  "arrow-left",
  "arrow-right",
  "arrow-right-left",
  "arrow-up",
  "award",
  "bar-chart-3",
  "book-open",
  "box",
  "camera",
  "check",
  "chevron-down",
  "chevron-left",
  "chevron-right",
  "chevron-up",
  "chevrons-up-down",
  "circle",
  "circle-dot",
  "clipboard-list",
  "clock",
  "cloud",
  "code",
  "combine",
  "component",
  "copy",
  "crosshair",
  "external-link",
  "eye",
  "file-archive",
  "file-code",
  "file-image",
  "file-json",
  "file-spreadsheet",
  "file-text",
  "file-type",
  "file-video",
  "filter",
  "focus",
  "folder",
  "folder-open",
  "globe",
  "graduation-cap",
  "grid-3x3",
  "grip-vertical",
  "hammer",
  "hand",
  "hard-drive",
  "hash",
  "home",
  "image",
  "image-plus",
  "image-up",
  "info",
  "landmark",
  "lasso",
  "layers",
  "layout",
  "layout-grid",
  "lightbulb",
  "link",
  "link-2-off",
  "list-tree",
  "loader-2",
  "maximize-2",
  "message-circle",
  "message-square",
  "minimize-2",
  "minus",
  "monitor",
  "moon",
  "more-horizontal",
  "mouse-pointer",
  "mouse-pointer-2",
  "move-3d",
  "network",
  "panel-left",
  "panel-right",
  "panel-top",
  "pause",
  "play",
  "file-video",
  "plug",
  "plus",
  "puzzle",
  "rotate-ccw",
  "save",
  "scaling",
  "search",
  "settings",
  "settings-2",
  "shapes",
  "sigma",
  "skip-back",
  "skip-forward",
  "smartphone",
  "smile",
  "square",
  "sun",
  "tablet",
  "table-2",
  "tags",
  "text-search",
  "triangle-alert",
  "user",
  "users",
  "wrench",
  "x",
  "bell",
  "calendar-days",
  "check-circle-2",
  "cylinder",
  "download",
  "eye-off",
  "hexagon",
  "library",
  "list",
  "lock",
  "lock-open",
  "move",
  "rotate-cw",
  "trash-2",
  "zoom-in",
  "zoom-out",
] as const;

type VendoredIconId = (typeof VENDORED_ICON_IDS)[number];
//#endregion 🔖Constants

//#region 🔧SvgNormalize
/** @emoji ✂️ Strips Lucide chrome and keeps stroke icons on `currentColor`. */
export function normalizeLucideSvg(raw: string): string {
  let svg = raw.replace(/<!--[\s\S]*?-->/g, "").trim();
  svg = svg.replace(/\sclass="[^"]*"/g, "");
  svg = svg.replace(/<svg\b([^>]*)>/, (_match, attrs: string) => {
    const cleaned = attrs.replace(/\swidth="[^"]*"/g, "").replace(/\sheight="[^"]*"/g, "");
    return `<svg${cleaned}>`;
  });
  svg = svg.replace(/\sstroke-width="[^"]*"/g, ' stroke-width="2"');
  if (!/stroke="currentColor"/.test(svg)) {
    svg = svg.replace(/<svg\b/, '<svg stroke="currentColor"');
  }
  if (!/fill="none"/.test(svg) && /<svg[^>]*fill=/.test(svg) === false) {
    svg = svg.replace(/<svg\b/, '<svg fill="none"');
  }
  return svg.trim();
}

/** @emoji 🏷 PascalCase identifier for C# from kebab icon id. */
export function iconIdToPascal(id: string): string {
  return id
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

/** @emoji 🐍 Upper snake for Python constants. */
export function iconIdToPythonConst(id: string): string {
  return id.replace(/-/g, "_").toUpperCase();
}
//#endregion 🔧SvgNormalize

//#region 📦Vendor
function lucideStaticRoot(repoRoot: string): string {
  return resolve(repoRoot, "node_modules/lucide-static/icons");
}

function vendorIcons(repoRoot: string, assetsDir: string): readonly VendoredIconId[] {
  const sourceRoot = lucideStaticRoot(repoRoot);
  const iconsDir = join(assetsDir, "icons");
  mkdirSync(iconsDir, { recursive: true });
  const vendored: VendoredIconId[] = [];
  for (const id of VENDORED_ICON_IDS) {
    const sourcePath = join(sourceRoot, `${id}.svg`);
    if (!existsSync(sourcePath)) {
      throw new Error(`Missing Lucide source icon: ${sourcePath}`);
    }
    const normalized = normalizeLucideSvg(readFileSync(sourcePath, "utf8"));
    writeFileSync(join(iconsDir, `${id}.svg`), `${normalized}\n`, "utf8");
    vendored.push(id);
  }
  return vendored;
}
//#endregion 📦Vendor

//#region 🧬Codegen
function readVendoredSvgs(iconsDir: string): Record<string, string> {
  const out: Record<string, string> = {};
  for (const file of readdirSync(iconsDir).filter((name) => name.endsWith(".svg"))) {
    const id = basename(file, ".svg");
    out[id] = normalizeLucideSvg(readFileSync(join(iconsDir, file), "utf8"));
  }
  return out;
}

function escapeTsString(value: string): string {
  return value.replace(/\\/g, "\\\\").replace(/`/g, "\\`").replace(/\$/g, "\\$");
}

function escapeCSharpString(value: string): string {
  return value.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
}

function escapePythonString(value: string): string {
  return value.replace(/\\/g, "\\\\").replace(/"""/g, '\\"\\"\\"');
}

function generateJs(icons: Record<string, string>, generatedDir: string): void {
  const entries = Object.keys(icons)
    .sort()
    .map((id) => `  "${id}": \`${escapeTsString(icons[id]!)}\`,`)
    .join("\n");
  const body = `// Generated by ui/assets/script.ts — do not edit.
export const ICONS = {
${entries}
} as const;

export type IconName = keyof typeof ICONS;

export const ICON_NAMES = Object.keys(ICONS) as IconName[];
`;
  writeFileSync(join(generatedDir, "icons.ts"), body, "utf8");
}

function generateCs(icons: Record<string, string>, generatedDir: string): void {
  const constants = Object.keys(icons)
    .sort()
    .map((id) => `    public const string ${iconIdToPascal(id)} = "${id}";`)
    .join("\n");
  const dictEntries = Object.keys(icons)
    .sort()
    .map((id) => `            ["${id}"] = @"${escapeCSharpString(icons[id]!)}",`)
    .join("\n");
  const body = `// Generated by ui/assets/script.ts — do not edit.
namespace Ui.Assets;

public static class Icons
{
${constants}

    public static IReadOnlyDictionary<string, string> SvgByName { get; } = new Dictionary<string, string>
    {
${dictEntries}
    };
}
`;
  writeFileSync(join(generatedDir, "Icons.cs"), body, "utf8");
}

function generatePy(icons: Record<string, string>, generatedDir: string): void {
  const literals = Object.keys(icons)
    .sort()
    .map((id) => `    "${id}": """${escapePythonString(icons[id]!)}""",`)
    .join("\n");
  const names = Object.keys(icons)
    .sort()
    .map((id) => `    "${id}",`)
    .join("\n");
  const body = `# Generated by ui/assets/script.ts — do not edit.
from __future__ import annotations

from typing import Final, Literal

IconName = Literal[
${names}
]

ICONS: Final[dict[str, str]] = {
${literals}
}
`;
  writeFileSync(join(generatedDir, "icons.py"), body, "utf8");
}

function writeVendoredReadme(assetsDir: string, ids: readonly string[]): void {
  const sorted = [...ids].sort();
  const list = sorted.map((id) => `- \`${id}\` (from Lucide \`${id}\`)`).join("\n");
  const body = `# UI assets

Shared fonts, cursors, lists, and vendored UI icons served at \`/assets/*\`.

## Icons (vendored from Lucide)

Initial chrome icons were copied from [Lucide](https://lucide.dev) v${LUCIDE_VERSION} (\`lucide-static\`, **ISC License**).
Source: https://github.com/lucide-icons/lucide

Do not edit files under \`icons/*.svg\` by hand — run **build ui assets** (\`bun ./script.ts generate all\`) after changing \`VENDORED_ICON_IDS\` in \`script.ts\`.

### Vendored icon ids

${list}
`;
  writeFileSync(join(assetsDir, "README.md"), body, "utf8");
}
//#endregion 🧬Codegen

//#region 🚀Commands
function runGenerate(target: string): void {
  const assetsDir = import.meta.dir;
  const repoRoot = getWorkspaceRoot();
  const generatedDir = join(assetsDir, "icons", "generated");
  mkdirSync(generatedDir, { recursive: true });
  const vendored = vendorIcons(repoRoot, assetsDir);
  writeVendoredReadme(assetsDir, vendored);
  const icons = readVendoredSvgs(join(assetsDir, "icons"));
  if (target === "js" || target === "all") generateJs(icons, generatedDir);
  if (target === "net" || target === "all") generateCs(icons, generatedDir);
  if (target === "py" || target === "all") generatePy(icons, generatedDir);
  console.log(`[ui/assets] generated ${Object.keys(icons).length} icons → ${target}`);
}

class GenerateScript extends BundleScript {
  run(segments: string[]): void {
    const target = (segments[0] ?? "all").toLowerCase();
    if (!["js", "net", "py", "all"].includes(target)) {
      throw new Error(`Unknown generate target: ${target}`);
    }
    runGenerate(target);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("normalizeLucideSvg", () => {
    it("strips root svg width/height and keeps currentColor stroke", () => {
      const raw = `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" stroke="#000"><path d="M0 0"/></svg>`;
      const out = normalizeLucideSvg(raw);
      expect(out).not.toMatch(/<svg[^>]*width="/);
      expect(out).toContain('stroke="currentColor"');
    });

    it("preserves width/height on child shapes such as layout-grid rects", () => {
      const raw = `<svg width="24" height="24"><rect width="7" height="7" x="3" y="3" rx="1" /></svg>`;
      const out = normalizeLucideSvg(raw);
      expect(out).not.toMatch(/<svg[^>]*width="/);
      expect(out).toContain('width="7"');
      expect(out).toContain('height="7"');
    });
  });
  describe("iconIdToPascal", () => {
    it("maps kebab-case to PascalCase", () => {
      expect(iconIdToPascal("layout-grid")).toBe("LayoutGrid");
    });
  });
}
//#endregion 🚀Commands
