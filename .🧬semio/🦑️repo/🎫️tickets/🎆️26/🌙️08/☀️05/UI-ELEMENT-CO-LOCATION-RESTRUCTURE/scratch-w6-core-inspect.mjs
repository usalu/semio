import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const ELEMENTS = join(ROOT, "\u{1F9ED}\uFE0Fframework/\u{1F50D}\uFE0Fmodules/\u{1F5B1}\uFE0Fui/\u{1F9E9}\uFE0Felements");
const CORE = join(ELEMENTS, "\u{1F9F2}\uFE0Fcore");
const REACT = join(
  ROOT,
  "\u{1F9ED}\uFE0Fframework/\u{1F50D}\uFE0Fmodules/\u{1F5B1}\uFE0Fui/\u{1F4E6}\uFE0Fpackages/\u{1F7E2}\uFE0Ftypescript/\u{1F3AF}\uFE0Ftargets/\u{269B}\uFE0Freact",
);
const BARREL = join(REACT, "\u{1F4E6}\uFE0Findex.tsx");
const TICKET = join(
  ROOT,
  ".\u{1F980}\uFE0Frepo/\u{1F39F}\uFE0Ftickets/\u{1F386}\uFE0F26/\u{1F319}\uFE0F08/\u{2600}\uFE0F05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE",
);

const out = [];
const log = (...a) => {
  const s = a.map(String).join(" ");
  out.push(s);
  console.log(s);
};

log("CORE exists", CORE, readdirSync(CORE).join(", "));
for (const d of readdirSync(CORE)) {
  log("==", d);
  const p = join(CORE, d);
  for (const f of readdirSync(p)) {
    const fp = join(p, f);
    if (!/\.(tsx?|rs)$/.test(f)) continue;
    const t = readFileSync(fp, "utf8");
    const ex = [...t.matchAll(/^export (?:function|const|type|interface|class|let|enum) (\w+)/gm)].map(
      (m) => m[1],
    );
    log(" ", f, "lines", t.split("\n").length, "exports:", ex.join(", "));
    if (t.includes("W3-interim")) log("  HAS W3-interim");
  }
}

const lines = readFileSync(BARREL, "utf8").split("\n");
log("barrel lines", lines.length);

const syms = [
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

for (const s of syms) {
  const hits = [];
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(s)) hits.push(`${i + 1}: ${lines[i].slice(0, 140)}`);
  }
  log(`\n### ${s} (${hits.length})`);
  for (const h of hits.slice(0, 12)) log(h);
  if (hits.length > 12) log(`  ... +${hits.length - 12} more`);
}

log("\n### regions of interest");
for (let i = 0; i < lines.length; i++) {
  const l = lines[i];
  if (/^\s*\/\/\s*#region /.test(l) && /(Label|Surface|Level|ElementProps|Flow|Shell|Transaction)/.test(l)) {
    log(String(i + 1).padStart(6), l.slice(0, 140));
  }
}

writeFileSync(join(TICKET, "\u{1F9E9}\uFE0Fw6-core-inspect-out.txt"), out.join("\n"));
log("wrote");
