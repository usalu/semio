import { readdirSync, readFileSync, writeFileSync, mkdirSync, existsSync, statSync } from "fs";
import { join, dirname, relative, basename } from "path";

const ticket = process.argv[2];
if (!ticket) throw new Error("pass ticket dir");
const paths = JSON.parse(readFileSync(join(ticket, "scratch-w6-paths.json"), "utf8"));
const { barrel, coreDir, elDir } = paths;

function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  if (!hits.length) throw new Error(`cannot resolve ${bare} under ${parent}`);
  hits.sort((a, b) => a.length - b.length);
  return join(parent, hits[0]);
}
function compName(dir) {
  return readdirSync(dir).find((n) => n.endsWith("component.tsx")) ?? "🟦️component.tsx";
}
function compFile(dir) {
  return join(dir, compName(dir));
}
const rel = (fromDir, toFile) => {
  let r = relative(fromDir, toFile).replaceAll("\\", "/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};

const classNamesDir = resolveUnder(coreDir, "ClassNames");
const portsDir = resolveUnder(coreDir, "Ports");
const uiLabelDir = resolveUnder(coreDir, "UiLabel");
const labelDir = resolveUnder(coreDir, "Label");
const elementIdDir = resolveUnder(coreDir, "ElementId");
const portsFile = compFile(portsDir);
const classNamesFile = compFile(classNamesDir);
const uiLabelFile = compFile(uiLabelDir);
const elementIdFile = compFile(elementIdDir);

// Fix ElementId Ports import (emoji path)
{
  let t = readFileSync(elementIdFile, "utf8");
  const fixed = t.replace(/from "\.\.\/Ports\//g, `from "../${basename(portsDir)}/`);
  if (fixed !== t) {
    writeFileSync(elementIdFile, fixed);
    console.log("fixed ElementId Ports import");
  }
}

const headerFrom = (dirName) => `// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/${dirName}/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header`;

const W3 = `// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🎱️elements/🟡️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf
// file without the same marker; grep for \`🚧️W3-interim\` must be empty before W6 closes.`;

let lines = readFileSync(barrel, "utf8").split("\n");
const log = [];

function findLine(re, from = 0) {
  for (let i = from; i < lines.length; i++) if (re.test(lines[i])) return i;
  return -1;
}

function extractBlockByBrace(startLineIdx) {
  // startLineIdx is line with export function/const that opens {
  let depth = 0;
  let started = false;
  for (let i = startLineIdx; i < lines.length; i++) {
    for (const ch of lines[i]) {
      if (ch === "{") { depth++; started = true; }
      if (ch === "}") depth--;
    }
    if (started && depth === 0) return { start: startLineIdx, end: i };
  }
  throw new Error("unclosed block at " + (startLineIdx + 1));
}

function removeLines(ranges) {
  // ranges: array of [start,end] inclusive indices, sorted descending
  const sorted = [...ranges].sort((a, b) => b[0] - a[0]);
  for (const [s, e] of sorted) {
    lines.splice(s, e - s + 1);
  }
}

function replaceRange(start, end, replacementLines) {
  lines.splice(start, end - start + 1, ...replacementLines);
}

// ============================================================================
//#region StyleClasses → ClassNames
// ============================================================================
const styleExports = [
  "waitingBorderClass",
  "waitingBorderActiveClass",
  "loadingBorderClass",
  "loadingBorderActiveClass",
  "interactiveOnClass",
  "uiFormControlBrowserDefaultProps",
  "interactiveTabActiveClass",
  "groupHoverExcludingHandleBgFillClass",
  "hoverExcludingHandleTextEmphasizedClass",
  "formControlFocusBorderClass",
  "interactiveControlTransitionClass",
  "interactiveHoverClass",
  "borderNormalBottomClass",
  "borderNormalClass",
  "borderElementClass",
  "interactiveActiveFillClass",
  "interactiveActiveBorderClass",
  "veilClass",
  "glassClass",
  "surfaceClass",
  "menuListItemClassName",
  "interactiveHoverFillClass",
  "shellFloorPaints",
  "shellFloorFillClass",
];
// Also move private consts
const stylePrivates = [
  "hoverExcludingHandleBgFillClass",
  "hoverExcludingHandleActiveBgClass",
  "hoverExcludingHandleActiveBorderClass",
];

// Collect definition text for each symbol from barrel (may be multi-line)
function collectExportConstOrFn(name) {
  const start = findLine(new RegExp(`^(?:export )?(?:const|function) ${name}\\b`));
  if (start < 0) return null;
  const L = lines[start];
  // include preceding doc comments
  let docStart = start;
  while (docStart > 0 && (/^\s*\/\*\*/.test(lines[docStart - 1]) || /^\s*\*/.test(lines[docStart - 1]) || /^\s*\*\//.test(lines[docStart - 1]) || lines[docStart - 1].startsWith("/**") || lines[docStart - 1].trim() === "")) {
    if (lines[docStart - 1].trim() === "" && docStart < start) {
      // stop at blank only if we already have a doc
      const hasDoc = lines.slice(docStart, start).some((l) => l.includes("/**") || l.includes("@emoji"));
      if (hasDoc) break;
    }
    docStart--;
    if (docStart > 0 && lines[docStart].startsWith("/**")) {
      // include full block comment above
      break;
    }
  }
  // better: walk up while line is comment-ish
  docStart = start;
  while (docStart > 0) {
    const prev = lines[docStart - 1];
    if (prev.trim() === "") { docStart--; continue; }
    if (prev.trim().startsWith("/**") || prev.trim().startsWith("*") || prev.trim().startsWith("*/") || prev.includes("@emoji")) {
      docStart--;
      continue;
    }
    break;
  }
  // ensure we start at /**
  while (docStart < start && !lines[docStart].includes("/**") && !lines[docStart].includes("@emoji") && !/^(?:export )?(?:const|function)/.test(lines[docStart])) {
    docStart++;
  }

  let end;
  if (/^export function |^function /.test(L) || /\([^)]*\)\s*\{/.test(L) || /\{$/.test(L.trim())) {
    // function or object const with braces
    if (L.includes("= {") || L.trim().endsWith("{") || /^export function |^function /.test(L)) {
      const block = extractBlockByBrace(start);
      end = block.end;
      // for `} as const satisfies` trailing
      if (lines[end + 1] && /as const|satisfies/.test(lines[end + 1])) end++;
      // sometimes closing is `};` on same or `) as const;`
    } else if (L.includes("(") && !L.includes(";") && !L.trim().endsWith(";")) {
      // cn( multi-line
      end = start;
      while (end < lines.length && !lines[end].includes(";")) end++;
    } else {
      end = start;
    }
  } else if (!L.includes(";") && L.includes("cn(")) {
    end = start;
    while (end < lines.length && !lines[end].includes(";")) end++;
  } else if (!L.trim().endsWith(";") && !L.trim().endsWith("}")) {
    end = start;
    while (end < lines.length && !lines[end].trim().endsWith(";") && !lines[end].trim().endsWith("},") && !(lines[end].includes(";") && end > start)) {
      end++;
      if (end - start > 40) break;
    }
  } else {
    end = start;
  }
  // template string single line with ;
  if (L.includes("`") && L.includes(";")) end = start;
  if (L.includes('= "') && L.includes(";")) end = start;
  if (L.includes("= `") && L.includes(";")) end = start;

  return { docStart, start, end, text: lines.slice(docStart, end + 1).join("\n"), name };
}

// Simpler approach: hardcode the StyleClasses body in topo order (handcrafted from known defs)
const styleClassesBody = `//#region 🎨️StyleClasses
/** @emoji 🌀️ Dashed, slow-spinning + gently pulsing waiting ring in the element's normal border color. */
export const waitingBorderClass = "border-waiting";

/** @emoji 🌀️ Waiting ring recolored to the active stroke; pair with selected/active elements. */
export const waitingBorderActiveClass = cn(waitingBorderClass, "border-waiting-active");

/** @emoji 🌀️ Clockwise spinning + pulsing loading ring in the element's normal border color. */
export const loadingBorderClass = "border-loading";

/** @emoji 🌀️ Loading ring recolored to the active stroke; pair with selected/active elements. */
export const loadingBorderActiveClass = cn(loadingBorderClass, "border-loading-active");

/** @emoji 🎨️ Shared transition for interactive chrome (hover, focus, active backgrounds). */
export const interactiveControlTransitionClass = "transition-[color,border-color,background-color]";

/** @emoji 🎯️ Focus/open on form controls: accent border color only, never extra ring width. */
export const formControlFocusBorderClass = cn("outline-none", interactiveControlTransitionClass, "focus-visible:border-accent data-[state=open]:border-accent aria-invalid:border-destructive focus-visible:ring-0 shadow-none");

/**
 * @emoji 🫳️ Hover-reactive utilities suppressed while a nested DragHandle is hovered — hovering the grip
 * then only highlights the grip, not the whole element. Pair with \`{HANDLE_HOVER_SCOPE_ATTR}\` on the same element
 * (the handle toggles \`data-handle-hovered\` on its nearest \`data-hover-scope\` ancestor via plain DOM writes, no
 * re-render). Deliberately avoids \`:has()\` — it isn't reliably supported across every environment this ships to
 * (older embedded webviews), and \`:has()\`-based ancestor exclusion also matches ANY ancestor with a matching
 * class, not necessarily the nearest one, which is wrong once tree rows nest.
 *
 * These MUST be written as complete literal strings, not built via \`\${}\` interpolation in a helper function —
 * Tailwind's build only discovers classes by scanning source files for literal text, it never executes JS, so a
 * class name assembled from a template placeholder at runtime is invisible to it and silently generates no CSS
 * at all (this broke hover entirely here once already).
 */
const hoverExcludingHandleBgFillClass = "hover:not-data-[handle-hovered=true]:bg-hover-interactive-fill";
const hoverExcludingHandleActiveBgClass = "hover:not-data-[handle-hovered=true]:bg-active-base/90";
const hoverExcludingHandleActiveBorderClass = "hover:not-data-[handle-hovered=true]:border-active-base";

export const groupHoverExcludingHandleBgFillClass = "group-hover/tree-row:not-group-data-[handle-hovered=true]/tree-row:bg-hover-interactive-fill";

export const hoverExcludingHandleTextEmphasizedClass = "hover:not-data-[handle-hovered=true]:text-emphasized";

/** @emoji 🎨️ Normal-border gray fill for interactive hover states. */
export const interactiveHoverFillClass = "hover:bg-hover-interactive-fill";

/** @emoji 🎨️ Interactive hover: normal-border fill + emphasized content. */
export const interactiveHoverClass = cn(interactiveHoverFillClass, "hover:text-emphasized");

/** @emoji 📏️ Active stroke paired with {@link interactiveActiveFillClass}. */
export const interactiveActiveBorderClass = "border-active-base";

/** @emoji 🎨️ Shared active fill for pressed tabs, toggles, and nav selection. */
export const interactiveActiveFillClass = cn("bg-active-base", interactiveActiveBorderClass, "text-emphasized", hoverExcludingHandleActiveBgClass, hoverExcludingHandleActiveBorderClass, hoverExcludingHandleTextEmphasizedClass);

/** @emoji 🎨️ Active/on: primary fill + active border + emphasized content (never the transient hover fill). */
export const interactiveOnClass = cn(
  "data-[state=on]:bg-active-base",
  "data-[state=on]:border-active-base",
  "data-[state=on]:text-emphasized",
  "data-[state=on]:hover:bg-active-base/90",
  "data-[state=on]:hover:border-active-base",
  "data-[state=on]:hover:text-emphasized",
);

/** @emoji 🎨️ Active tab: primary fill + active border + emphasized content. */
export const interactiveTabActiveClass = cn(
  "data-[state=active]:bg-active-base",
  "data-[state=active]:border-active-base",
  "data-[state=active]:text-emphasized",
  "data-[state=active]:hover:bg-active-base/90",
  "data-[state=active]:hover:border-active-base",
  "data-[state=active]:hover:text-emphasized",
);

/** @emoji 🚫️ React props that disable native browser affordances on editable UI controls. */
export const uiFormControlBrowserDefaultProps = {
  autoComplete: "off",
  autoCorrect: "off",
  autoCapitalize: "off",
  spellCheck: false,
  "data-1p-ignore": true,
  "data-lpignore": "true",
} as const satisfies Pick<React.InputHTMLAttributes<HTMLInputElement>, "autoComplete" | "autoCorrect" | "autoCapitalize" | "spellCheck"> & { readonly "data-1p-ignore": boolean; readonly "data-lpignore": string };

/** @emoji 📏️ Subtle normal stroke for controls, windows, dividers, and in-chrome separators. */
export const borderNormalClass = "!border-normal";

/** @emoji 📏️ Normal bottom edge utility for in-chrome dividers (not shell navbar — navbar uses a CSS \`::after\` stroke). */
export const borderNormalBottomClass = \`border-b \${borderNormalClass}\`;

/** @emoji 📏️ Implicit element border color (controls, dropdowns, dividers). */
export const borderElementClass = "border-element";

/** @emoji 🎨️ Opaque per-level fill — background-color only, no blur (see \`[data-level]\` cascade in 🎨️ui.css). */
export const surfaceClass = "ui-surface";

export const glassClass = "ui-glass";

/** @emoji 🎨️ Fullscreen scrim; host element must carry \`data-level="dialog"\` for correct tint. */
export const veilClass = "ui-veil";

/** @emoji 📋️ Hover row styling for menus, selects, comboboxes, and context menus. */
export const menuListItemClassName = cn(
  "text-element",
  interactiveHoverClass,
  "focus:bg-hover-interactive-fill focus:text-emphasized",
  "data-[active=true]:bg-hover-interactive-fill data-[active=true]:text-emphasized",
  "data-[selected=true]:bg-active-base data-[selected=true]:border-active-base data-[selected=true]:text-emphasized",
);

/** @emoji 🎨️ Whether a base-floor chrome row (navbar/footer/canvas/mode-body) must paint its own
 * {@link surfaceClass}, or stay transparent so Layout's one continuous base surface shows through.
 * Nested same-level paints are the "navbar ≠ canvas ≠ footer" bug class — one base floor, one fill. */
export function shellFloorPaints(parent: SurfaceScopeValue | null): boolean {
  return !(parent?.level === "base" && parent.fill !== "none");
}

/** @emoji 🎨️ Fill class for base-floor chrome — {@link surfaceClass} when standalone, transparent on Layout's painted base. */
export function shellFloorFillClass(parent: SurfaceScopeValue | null): string {
  return shellFloorPaints(parent) ? surfaceClass : "bg-transparent";
}

/** @emoji Re-export private handle-hover fill for chrome tab cells still composing in the barrel. */
export { hoverExcludingHandleBgFillClass };
//#endregion 🎨️StyleClasses
`;

// Surface file first (Level + SurfaceScopeValue) so ClassNames can type-import
const surfaceDirName = "🌈️Surface";
const surfaceDir = join(coreDir, surfaceDirName);
mkdirSync(surfaceDir, { recursive: true });
const surfaceFile = join(surfaceDir, "🟦️component.tsx");

// Extract Level..Surface block from barrel (L7125 region start through Surface.displayName)
const levelRegionStart = findLine(/^\/\/ #region 🎈️Level Context$/);
if (levelRegionStart < 0) throw new Error("Level Context region missing");
const levelTypeLine = findLine(/^export type Level =/, levelRegionStart);
const surfaceDisplayEnd = findLine(/^Surface\.displayName = "Surface";/, levelTypeLine);
if (surfaceDisplayEnd < 0) throw new Error("Surface.displayName missing");

// Also grab surface-active cluster
const surfaceActiveStart = findLine(/^const surfaceActiveRoots = new Set/);
const useSurfaceActiveStart = findLine(/^export function useSurfaceActive/);
const useSurfaceActiveBlock = extractBlockByBrace(useSurfaceActiveStart);
const surfaceActiveBindProps = findLine(/^export interface SurfaceActiveBindProps/, surfaceActiveStart);
const isSurfaceActiveFn = findLine(/^export function isSurfaceActiveBackgroundPointer/, surfaceActiveStart);

// Build Surface file content from barrel slices
const levelThroughSurface = lines.slice(levelTypeLine, surfaceDisplayEnd + 1);
// Remove orphan docstring-only lines that reference moved style classes without defs - keep as-is for now

const surfaceActiveSlice = lines.slice(surfaceActiveStart, useSurfaceActiveBlock.end + 1);
// Include SurfaceActiveBindProps interface - it's before useSurfaceActive
const bindPropsStart = surfaceActiveBindProps;
// Rebuild surface-active in order: roots state → helpers → isSurfaceActiveBackgroundPointer → bind props → useSurfaceActive
// The slice from surfaceActiveRoots through useSurfaceActiveBlock.end already includes bind props if bindProps is between.

const surfaceContent = `${headerFrom(surfaceDirName)}

// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "${rel(surfaceDir, portsFile)}";
import { cn } from "${rel(surfaceDir, classNamesFile)}";
import { glassClass, surfaceClass, veilClass } from "${rel(surfaceDir, classNamesFile)}";
// #endregion 🔌️Adapters

// #region 🎈️Surface
${levelThroughSurface.join("\n")}

${surfaceActiveSlice.join("\n")}
// #endregion 🎈️Surface
`;

writeFileSync(surfaceFile, surfaceContent);
log.push(`created ${surfaceFile}`);

// Now expand ClassNames with StyleClasses + type import SurfaceScopeValue
let cnText = readFileSync(classNamesFile, "utf8");
if (!cnText.includes("shellFloorPaints")) {
  // Add React import for uiFormControlBrowserDefaultProps
  if (!cnText.includes('import * as React')) {
    cnText = cnText.replace(
      "// #endregion 🔌️Adapters",
      `import * as React from "react";\nimport type { SurfaceScopeValue } from "${rel(classNamesDir, surfaceFile)}";\n// #endregion 🔌️Adapters`,
    );
  }
  cnText = cnText.replace("//#endregion 🎨️ClassNames", `//#endregion 🎨️ClassNames\n\n${styleClassesBody}\n`);
  writeFileSync(classNamesFile, cnText);
  log.push("expanded ClassNames with StyleClasses");
}

// Style symbols to re-export from barrel and remove defs
const styleSyms = [
  "waitingBorderClass", "waitingBorderActiveClass", "loadingBorderClass", "loadingBorderActiveClass",
  "interactiveOnClass", "uiFormControlBrowserDefaultProps", "interactiveTabActiveClass",
  "groupHoverExcludingHandleBgFillClass", "hoverExcludingHandleTextEmphasizedClass",
  "formControlFocusBorderClass", "interactiveControlTransitionClass",
  "interactiveHoverClass", "borderNormalBottomClass", "borderNormalClass", "borderElementClass",
  "interactiveActiveFillClass", "interactiveActiveBorderClass", "veilClass", "glassClass", "surfaceClass",
  "menuListItemClassName", "interactiveHoverFillClass", "shellFloorPaints", "shellFloorFillClass",
  "hoverExcludingHandleBgFillClass",
];

// Remove old style defs from barrel (and private consts). Find by definition line and remove with preceding docs.
function removeDef(name, { allowConst = true } = {}) {
  const re = new RegExp(`^(?:export )?(?:const|function) ${name}\\b`);
  const idx = findLine(re);
  if (idx < 0) {
    console.warn("missing def", name);
    return;
  }
  let docStart = idx;
  while (docStart > 0) {
    const prev = lines[docStart - 1];
    if (prev.trim() === "") { docStart--; continue; }
    if (prev.includes("/**") || prev.trim().startsWith("*") || prev.includes("@emoji") || prev.trim().startsWith("//")) {
      docStart--;
      continue;
    }
    break;
  }
  while (docStart < idx && lines[docStart].trim() === "") docStart++;
  let end = idx;
  const L = lines[idx];
  if (L.includes("{") && (!L.includes("}") || L.trim().endsWith("{") || /\{\s*$/.test(L) || L.includes("= {"))) {
    end = extractBlockByBrace(idx).end;
    // trailing `as const satisfies...;` on following lines already inside? 
    if (lines[end] && !lines[end].includes(";") && lines[end + 1] && /as const|satisfies/.test(lines[end + 1])) {
      end++;
    }
  } else if (!L.includes(";")) {
    while (end < lines.length && !lines[end].includes(";")) end++;
  }
  // Also remove following orphan doc-only comment lines that duplicated moved docs? skip
  lines.splice(docStart, end - docStart + 1);
}

// Replace Level..Surface block with import-export
{
  // re-find after potential line shifts — do style removal AFTER surface region replace
}

// Surface symbols
const surfaceSyms = [
  "Level", "LEVELS", "LevelProvider", "useLevel", "getLevelZClass",
  "SurfaceFill", "surfaceFillClass", "SurfaceScopeValue", "SurfaceScope", "useSurface",
  "SurfaceProps", "Surface",
  "isSurfaceActiveBackgroundPointer", "SurfaceActiveBindProps", "useSurfaceActive",
];

// Replace level region body for Level..Surface with import, keep rest of Level Context region
{
  const s = findLine(/^export type Level =/);
  const e = findLine(/^Surface\.displayName = "Surface";/, s);
  if (s < 0 || e < 0) throw new Error("Level/Surface block missing for replace");
  const importPath = rel(dirname(barrel), surfaceFile);
  const replacement = [
    `import {`,
    `  type Level,`,
    `  LEVELS,`,
    `  LevelProvider,`,
    `  useLevel,`,
    `  getLevelZClass,`,
    `  type SurfaceFill,`,
    `  surfaceFillClass,`,
    `  type SurfaceScopeValue,`,
    `  SurfaceScope,`,
    `  useSurface,`,
    `  type SurfaceProps,`,
    `  Surface,`,
    `  isSurfaceActiveBackgroundPointer,`,
    `  type SurfaceActiveBindProps,`,
    `  useSurfaceActive,`,
    `} from "${importPath}";`,
    `export {`,
    `  type Level,`,
    `  LEVELS,`,
    `  LevelProvider,`,
    `  useLevel,`,
    `  getLevelZClass,`,
    `  type SurfaceFill,`,
    `  surfaceFillClass,`,
    `  type SurfaceScopeValue,`,
    `  SurfaceScope,`,
    `  useSurface,`,
    `  type SurfaceProps,`,
    `  Surface,`,
    `  isSurfaceActiveBackgroundPointer,`,
    `  type SurfaceActiveBindProps,`,
    `  useSurfaceActive,`,
    `};`,
  ];
  replaceRange(s, e, replacement);
  log.push("wired Surface in barrel");
}

// Remove surface-active cluster from barrel (still present later in file)
{
  const s = findLine(/^const surfaceActiveRoots = new Set/);
  if (s >= 0) {
    const u = findLine(/^export function useSurfaceActive/, s);
    const end = extractBlockByBrace(u).end;
    // also remove isSurfaceActiveBackgroundTarget and related before roots? roots is the start of the cluster
    // Check for comment block above
    let docStart = s;
    while (docStart > 0 && (lines[docStart - 1].includes("@emoji") || lines[docStart - 1].trim().startsWith("/**") || lines[docStart - 1].trim().startsWith("*") || lines[docStart - 1].trim() === "")) {
      docStart--;
    }
    lines.splice(docStart, end - docStart + 1);
    log.push("removed surfaceActive cluster from barrel");
  }
}

// Remove style defs from barrel and add import-export near existing cn re-export
for (const name of [
  "waitingBorderActiveClass", "loadingBorderActiveClass", "waitingBorderClass", "loadingBorderClass",
  "interactiveOnClass", "uiFormControlBrowserDefaultProps", "interactiveTabActiveClass",
  "groupHoverExcludingHandleBgFillClass", "hoverExcludingHandleTextEmphasizedClass",
  "windowPaneChromeToggleClass", // DON'T remove - not moved
].filter((n) => n !== "windowPaneChromeToggleClass")) {
  // skip
}

const styleRemoveOrder = [
  "waitingBorderActiveClass", "loadingBorderActiveClass", "waitingBorderClass", "loadingBorderClass",
  "interactiveOnClass", "uiFormControlBrowserDefaultProps", "interactiveTabActiveClass",
  "groupHoverExcludingHandleBgFillClass", "hoverExcludingHandleTextEmphasizedClass",
  "formControlFocusBorderClass", "interactiveControlTransitionClass",
  "menuListItemClassName", "interactiveHoverClass",
  "borderNormalBottomClass", "borderNormalClass", "borderElementClass",
  "interactiveActiveFillClass", "interactiveActiveBorderClass",
  "veilClass", "glassClass", "surfaceClass",
  "shellFloorFillClass", "shellFloorPaints",
  "interactiveHoverFillClass",
  "hoverExcludingHandleBgFillClass", "hoverExcludingHandleActiveBgClass", "hoverExcludingHandleActiveBorderClass",
];
for (const name of styleRemoveOrder) {
  removeDef(name);
}

// Insert style import-export next to cn re-export
{
  const cnImport = findLine(/import \{ cn \} from .*ClassNames/);
  if (cnImport < 0) throw new Error("cn import missing in barrel");
  const cnExport = findLine(/^export \{ cn \};/, cnImport);
  const styleImportPath = rel(dirname(barrel), classNamesFile);
  const styleBlock = [
    `import {`,
    `  waitingBorderClass,`,
    `  waitingBorderActiveClass,`,
    `  loadingBorderClass,`,
    `  loadingBorderActiveClass,`,
    `  interactiveOnClass,`,
    `  uiFormControlBrowserDefaultProps,`,
    `  interactiveTabActiveClass,`,
    `  groupHoverExcludingHandleBgFillClass,`,
    `  hoverExcludingHandleTextEmphasizedClass,`,
    `  hoverExcludingHandleBgFillClass,`,
    `  formControlFocusBorderClass,`,
    `  interactiveControlTransitionClass,`,
    `  interactiveHoverClass,`,
    `  interactiveHoverFillClass,`,
    `  borderNormalBottomClass,`,
    `  borderNormalClass,`,
    `  borderElementClass,`,
    `  interactiveActiveFillClass,`,
    `  interactiveActiveBorderClass,`,
    `  veilClass,`,
    `  glassClass,`,
    `  surfaceClass,`,
    `  menuListItemClassName,`,
    `  shellFloorPaints,`,
    `  shellFloorFillClass,`,
    `} from "${styleImportPath}";`,
    `export {`,
    `  waitingBorderClass,`,
    `  waitingBorderActiveClass,`,
    `  loadingBorderClass,`,
    `  loadingBorderActiveClass,`,
    `  interactiveOnClass,`,
    `  uiFormControlBrowserDefaultProps,`,
    `  interactiveTabActiveClass,`,
    `  groupHoverExcludingHandleBgFillClass,`,
    `  hoverExcludingHandleTextEmphasizedClass,`,
    `  hoverExcludingHandleBgFillClass,`,
    `  formControlFocusBorderClass,`,
    `  interactiveControlTransitionClass,`,
    `  interactiveHoverClass,`,
    `  interactiveHoverFillClass,`,
    `  borderNormalBottomClass,`,
    `  borderNormalClass,`,
    `  borderElementClass,`,
    `  interactiveActiveFillClass,`,
    `  interactiveActiveBorderClass,`,
    `  veilClass,`,
    `  glassClass,`,
    `  surfaceClass,`,
    `  menuListItemClassName,`,
    `  shellFloorPaints,`,
    `  shellFloorFillClass,`,
    `};`,
  ];
  // Insert after export { cn };
  lines.splice(cnExport + 1, 0, ...styleBlock);
  log.push("wired StyleClasses in barrel");
}

writeFileSync(barrel, lines.join("\n"));
console.log("phase1 done", log);
console.log("barrel lines", lines.length);
