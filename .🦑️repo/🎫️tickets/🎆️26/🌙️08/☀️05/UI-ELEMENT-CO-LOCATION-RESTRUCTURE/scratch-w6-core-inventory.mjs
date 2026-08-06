import { readdirSync, readFileSync, writeFileSync, existsSync, statSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const FRAMEWORK = join(ROOT, "\u{1F9ED}\uFE0Fframework");
const MODULES = join(FRAMEWORK, "\u{1F50D}\uFE0Fmodules");
const UI = join(MODULES, readdirSync(MODULES).find((n) => n.includes("ui")));
const EL = join(UI, readdirSync(UI).find((n) => n.includes("elements")));
const CORE = join(EL, readdirSync(EL).find((n) => n.toLowerCase().includes("core")));
const PACKAGES = join(UI, "\u{1F4E6}\uFE0Fpackages");
const TS = join(PACKAGES, readdirSync(PACKAGES).find((n) => n.includes("typescript")));
const TARGETS = join(TS, readdirSync(TS).find((n) => n.includes("targets")));
const REACT = join(TARGETS, readdirSync(TARGETS).find((n) => n.includes("react")));
const BARREL = join(REACT, "\u{1F4E6}\uFE0Findex.tsx");

function findTickets() {
  const out = [];
  const stack = [join(ROOT, ".\u{1F980}\uFE0Frepo", "\u{1F39F}\uFE0Ftickets")];
  while (stack.length) {
    const d = stack.pop();
    for (const e of readdirSync(d, { withFileTypes: true })) {
      if (!e.isDirectory()) continue;
      const p = join(d, e.name);
      if (e.name === "UI-ELEMENT-CO-LOCATION-RESTRUCTURE") out.push(p);
      else if (e.name.length <= 24 || /[\u{1F300}-\u{1FAFF}]/u.test(e.name) || /^\d+$/.test(e.name)) stack.push(p);
    }
  }
  return out;
}
const tickets = findTickets();
const TICKET =
  tickets.find((t) => t.includes("\u{1F319}\uFE0F08") && existsSync(join(t, "\u{1F4D1}\uFE0FTEMPLATE-UI.md"))) ||
  tickets.find((t) => existsSync(join(t, "\u{1F4D1}\uFE0FTEMPLATE-UI.md"))) ||
  tickets[0];

const lines = readFileSync(BARREL, "utf8").split("\n");
const out = [];
const log = (...a) => out.push(a.map(String).join(" "));

const TARGET_SYMS = [
  "Label",
  "useLabel",
  "useIdLabel",
  "useControlAccessibleLabel",
  "useControlInlineText",
  "resolveTranslationLabel",
  "useUiTranslation",
  "SurfaceScope",
  "useSurface",
  "useSurfaceActive",
  "LevelProvider",
  "useLevel",
  "Level",
  "getLevelZClass",
  "isSurfaceActiveBackgroundPointer",
  "ElementProps",
  "useTransaction",
  "useFlow",
  "FlowProvider",
  "FlowBlock",
  "FlowInline",
  "useShellScopeOptional",
  "NULL_SHELL_ROOT_REF",
  "useShellKeydown",
  "focusActiveSearchInput",
  "routeWindowSearchEscape",
];

function isDefLine(line, sym) {
  return new RegExp(
    `^\\s*(export\\s+)?(async\\s+)?(function|const|let|var|type|interface|enum|class)\\s+${sym}\\b`,
  ).test(line);
}

const regionMeta = [];
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^\s*\/\/\s*#(region|endregion)\s*(.*)$/);
  if (m) regionMeta.push({ kind: m[1], name: m[2].trim(), line: i + 1 });
}
const stack = [];
const regionRanges = [];
for (const r of regionMeta) {
  if (r.kind === "region") stack.push({ name: r.name, start: r.line });
  else {
    const open = stack.pop();
    if (open) regionRanges.push({ name: open.name, start: open.start, end: r.line, depth: stack.length });
  }
}

log("# W6 core Label/Surface inventory (NO barrel edits)");
log("status: PAUSED — parent serializing ui-react barrel extractions; inventory-only");
log("generated:", new Date().toISOString());
log("barrel:", BARREL);
log("barrel_lines:", lines.length);
log("core_dir:", CORE);
log("core_existing:", readdirSync(CORE).join(", "));
log("ticket:", TICKET);
log("");

for (const d of readdirSync(CORE)) {
  const p = join(CORE, d);
  if (!statSync(p).isDirectory()) continue;
  log("## existing core/" + d);
  for (const f of readdirSync(p)) {
    if (!/\.(tsx?|rs)$/.test(f)) continue;
    const t = readFileSync(join(p, f), "utf8");
    const ex = [...t.matchAll(/^export (?:function|const|type|interface|class|let|enum) (\w+)/gm)].map((m) => m[1]);
    log("  " + f + " lines=" + t.split("\n").length);
    log("  exports:", ex.join(", ") || "(none)");
    if (t.includes("W3-interim")) log("  HAS W3-interim");
  }
}

log("");
log("## Symbol definition inventory");
const summary = [];
for (const sym of TARGET_SYMS) {
  const defs = [];
  const importExport = [];
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (!new RegExp(`\\b${sym}\\b`).test(line)) continue;
    if (isDefLine(line, sym)) {
      const enclosing = regionRanges
        .filter((r) => r.start <= i + 1 && i + 1 <= r.end)
        .sort((a, b) => a.end - a.start - (b.end - b.start));
      defs.push({
        line: i + 1,
        text: line.trim().slice(0, 180),
        regions: enclosing.slice(0, 4).map((r) => `${r.name}@L${r.start}-L${r.end}`),
      });
    } else if (/^\s*(import|export)\b/.test(line)) {
      importExport.push({ line: i + 1, text: line.trim().slice(0, 160) });
    }
  }
  const alreadyExtracted =
    defs.length === 0 && importExport.some((r) => r.text.includes("elements/") || r.text.includes("core/"));

  log("");
  log(`### ${sym}`);
  log(`defs: ${defs.length}${alreadyExtracted ? " (likely already re-exported from leaf)" : ""}`);
  for (const d of defs) {
    log(`  DEF L${d.line}: ${d.text}`);
    if (d.regions.length) log(`       innermost_regions: ${d.regions.join(" | ")}`);
  }
  const namedRegions = regionRanges.filter((r) => {
    if (r.name === sym) return true;
    if (sym === "Label" && /\bLabel\b/.test(r.name)) return true;
    if (sym === "Level" && /\bLevel\b/.test(r.name)) return true;
    if (sym === "SurfaceScope" && /Surface/.test(r.name)) return true;
    if (sym === "ElementProps" && /ElementProps|Element/.test(r.name)) return true;
    if ((sym === "useFlow" || sym === "FlowProvider") && /\bFlow\b/.test(r.name)) return true;
    if ((sym === "useShellScopeOptional" || sym === "NULL_SHELL_ROOT_REF") && /Shell/.test(r.name)) return true;
    if (sym === "useTransaction" && /Transaction/.test(r.name)) return true;
    return false;
  });
  if (namedRegions.length) {
    log("  related_regions:");
    for (const r of namedRegions.slice(0, 15)) {
      log(`    ${r.name} L${r.start}-L${r.end} depth=${r.depth} size=${r.end - r.start}`);
    }
  }
  log(`  import_export_hits: ${importExport.length}`);
  for (const r of importExport.slice(0, 8)) log(`    L${r.line}: ${r.text}`);

  summary.push({
    sym,
    defLines: defs.map((d) => d.line),
    innermost: defs[0]?.regions[0] || null,
    importExportCount: importExport.length,
    alreadyExtracted,
  });
}

log("");
log("## Regions matching Label|Surface|Level|ElementProps|Flow|Shell|Transaction|UiLabel|Element");
const interesting = regionRanges
  .filter((r) => /(Label|Surface|Level|ElementProps|Flow|Shell|Transaction|UiLabel|Element)/i.test(r.name))
  .sort((a, b) => a.start - b.start);
for (const r of interesting) {
  log(
    `L${String(r.start).padStart(5)}-L${String(r.end).padStart(5)}  depth=${r.depth}  size=${String(r.end - r.start).padStart(5)}  ${r.name}`,
  );
}

let open = 0;
let min = 0;
for (const line of lines) {
  if (/^\s*\/\/\s*#region\b/.test(line)) open++;
  if (/^\s*\/\/\s*#endregion\b/.test(line)) open--;
  if (open < min) min = open;
}
log("");
log("## Region balance");
log("final_open_minus_close:", open, "min_depth:", min, open === 0 && min === 0 ? "OK" : "IMBALANCE");

log("");
log("## Compact summary table");
log("symbol\tdef_lines\tinnermost_region\timp_exp\talready_extracted");
for (const s of summary) {
  log(`${s.sym}\t${s.defLines.join(",") || "-"}\t${s.innermost || "-"}\t${s.importExportCount}\t${s.alreadyExtracted}`);
}

log("");
log("## Suggested extraction clusters (NOT executed — waiting for parent)");
log("1. Label: extend core/Label and/or UiLabel — useLabel, useIdLabel, useControlAccessibleLabel, useControlInlineText, resolveTranslationLabel, useUiTranslation, Label");
log("2. Surface: new core/Surface — SurfaceScope, useSurface, useSurfaceActive, isSurfaceActiveBackgroundPointer");
log("3. Level: colocate with Surface or new core/Level — LevelProvider, useLevel, Level, getLevelZClass");
log("4. ElementProps: new core/ElementProps — ElementProps + tight base types");
log("5. Flow: new core/Flow — useTransaction, useFlow, FlowProvider, FlowBlock, FlowInline if small cohesive");
log("6. ShellScope: new core/ShellScope — useShellScopeOptional, NULL_SHELL_ROOT_REF, useShellKeydown, focusActiveSearchInput, routeWindowSearchEscape");
log("AVOID overlapping StyleClasses / Icons / Button / ContextMenu work");
log("NEXT: resume extraction when parent signals barrel lock available");

const reportPath = join(TICKET, "\u{1F9E9}\uFE0Fw6-core-label-surface.txt");
writeFileSync(reportPath, out.join("\n") + "\n");
console.log("WROTE", reportPath);
for (const s of summary) {
  console.log(s.sym, "defs@", s.defLines.join(",") || "-", "|", s.innermost || "-", "| extracted?", s.alreadyExtracted);
}
