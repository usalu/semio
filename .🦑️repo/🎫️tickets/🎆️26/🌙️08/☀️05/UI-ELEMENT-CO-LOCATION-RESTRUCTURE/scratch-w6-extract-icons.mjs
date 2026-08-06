import { readdirSync, readFileSync, writeFileSync, existsSync, statSync } from "fs";
import { join, dirname, relative } from "path";

const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  hits.sort((a, b) => a.length - b.length);
  if (!hits.length) throw new Error(`no ${bare} in ${parent}`);
  return join(parent, hits[0]);
}
function compFile(dir) {
  return join(dir, readdirSync(dir).find((n) => n.endsWith("component.tsx")));
}
const rel = (fromDir, toFile) => {
  let r = relative(fromDir, toFile).replaceAll("\\", "/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};

const core = resolveUnder(el, "core");
const cn = compFile(resolveUnder(core, "ClassNames"));
const iconsDir = resolveUnder(el, "Icons");
const iconsComp = compFile(iconsDir);
const existing = readFileSync(iconsComp, "utf8");
const cursorBlock = existing.match(/\/\*\*[\s\S]*?export \{ Cursor \};/)?.[0] 
  || existing.split("// #region ").find((s) => s.includes("Cursor")) 
  || "";

// better extract Cursor section from existing
const existingLines = existing.split("\n");
const cursorStart = existingLines.findIndex((l) => l.includes("CursorProps") || l.includes("Cursor icon"));
// Keep from interface CursorProps through export { Cursor }
let c0 = existingLines.findIndex((l) => l.includes("interface CursorProps"));
let c1 = existingLines.findIndex((l) => l.includes("export { Cursor }"));
if (c0 < 0) c0 = existingLines.findIndex((l) => l.includes("const Cursor"));
const cursorSection = existingLines.slice(c0, c1 + 1).join("\n");

const ag = resolveUnder(el, "ActionGroup");
const agLines = readFileSync(compFile(ag), "utf8").split("\n");
const adaptersOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"));
const adaptersClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("Adapters"));
const headerSrc = agLines.slice(0, 6);
const dirBase = iconsDir.split("/").pop();
const makeHeader = () => [headerSrc[0], headerSrc[1].replace(/elements\/[^/]+/, `elements/${dirBase}`), ...headerSrc.slice(2)].join("\n");

let lines = readFileSync(barrel, "utf8").split("\n");
let start = -1, end = -1;
for (let i = 0; i < lines.length; i++) {
  if (/^\/\/ #region 🔖️Icon$/.test(lines[i]) || (/^\/\/ #region /.test(lines[i]) && lines[i].endsWith("Icon") && !lines[i].includes("IconRender") && !lines[i].includes("IconCodec") && !lines[i].includes("Icons") && !lines[i].includes("IconSelector"))) {
    // prefer exact Icon region
    if (lines[i].includes("🔖️Icon") && !lines[i].includes("IconRender")) start = i;
  }
  if (start >= 0 && /^\/\/ #endregion 🔖️Icon$/.test(lines[i])) { end = i; break; }
}
// fallback
if (start < 0) {
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes("#region") && lines[i].includes("Icon") && !lines[i].includes("IconRender") && !lines[i].includes("Icons") && !lines[i].includes("Codec") && !lines[i].includes("Selector")) {
      start = i; break;
    }
  }
}
if (end < 0) {
  for (let i = start + 1; i < lines.length; i++) {
    if (lines[i].includes("#endregion") && lines[i].includes("Icon") && !lines[i].includes("Codec") && !lines[i].includes("Icons") && !lines[i].includes("Selector") && !lines[i].includes("Render")) {
      end = i; break;
    }
  }
}
console.log("Icon region", start + 1, end + 1);
if (start < 0 || end < 0) process.exit(1);

const body = lines.slice(start + 1, end).join("\n"); // without region markers
const regionOpen = lines[start];
const regionClose = lines[end];

// Collect exports
const valueExports = [];
const typeExports = [];
for (const l of lines.slice(start, end + 1)) {
  let m = l.match(/^export (?:async )?function (\w+)/);
  if (m) { valueExports.push(m[1]); continue; }
  m = l.match(/^export const (\w+)/);
  if (m) { valueExports.push(m[1]); continue; }
  m = l.match(/^export interface (\w+)/);
  if (m) { typeExports.push(m[1]); continue; }
  m = l.match(/^export type (\w+)/);
  if (m) { typeExports.push(m[1]); continue; }
}
valueExports.push("Cursor");

const file = `${makeHeader()}

${adaptersOpen}
import * as React from "react";
import { domSizePx } from "@semio-tech/ui-styling";
import { activeUiTheme, type UiTheme } from "@semio-tech/ui-styling";
import {
  ICONS,
  isIconName,
  resolveCatalogIconSvgFromTheme,
  shortcodeCatalogKey,
  shortcodeEmoji,
  type IconName,
} from "@semio-tech/assets";
import {
  isMetabolismIconName,
  METABOLISM_ICONS,
  resolveMetabolismIconSvgFromTheme,
  type MetabolismIconName,
} from "@semio-tech/assets";
import { cn } from "${rel(iconsDir, cn)}";
${adaptersClose}

${regionOpen}
${body}

${cursorSection}

export { Cursor };
${regionClose}
`;

// Fix double export { Cursor }
let fixed = file.replace(/export \{ Cursor \};\s*\nexport \{ Cursor \};/, "export { Cursor };");
// If cursorSection already has export { Cursor }, remove the extra
if ((fixed.match(/export \{ Cursor \}/g) || []).length > 1) {
  fixed = fixed.replace(/\nexport \{ Cursor \};\n\/\/ #endregion/, "\n// #endregion");
}

writeFileSync(iconsComp, fixed);
console.log("WROTE icons", iconsComp, "values", valueExports.length, "types", typeExports.length);

const importPath = rel(dirname(barrel), iconsComp);
const importList = [...new Set(valueExports), ...new Set(typeExports).map((t) => `type ${t}`)].join(", ");
const exportList = importList;

// Replace Icon region in barrel
let before = lines.slice(0, start).join("\n");
let after = lines.slice(end + 1).join("\n");
const iconReplacement = `${regionOpen}
import { ${importList} } from "${importPath}";
export { ${exportList} };
${regionClose}
`;

// Also update Icons stub region that only exports Cursor
after = after.replace(
  /\/\/ #region 🛒️Icons\nimport \{ Cursor \} from "[^"]+";\nexport \{ Cursor \};\n\/\/ #endregion 🛒️Icons/,
  `// #region 🛒️Icons\nimport { Cursor } from "${importPath}";\nexport { Cursor };\n// #endregion 🛒️Icons`,
);
// if path already emoji, still ok

let newBarrel = before + "\n" + iconReplacement + "\n" + after;
writeFileSync(barrel, newBarrel);

// balance
lines = newBarrel.split("\n");
let o = 0, c = 0;
for (const l of lines) {
  if (/^\/\/\s*#region\b/.test(l)) o++;
  if (/^\/\/\s*#endregion\b/.test(l)) c++;
}
console.log({ opens: o, closes: c, balanced: o === c, lines: lines.length });
if (newBarrel.includes("export function createIconComponent")) {
  console.error("OLD createIconComponent still in barrel");
  process.exit(1);
}
console.log("Icon cluster extracted");

// Update DragHandle to import icons from Icons leaf
const dh = resolveUnder(el, "DragHandle");
const dhf = compFile(dh);
let dht = readFileSync(dhf, "utf8");
const iconsRel = rel(dh, iconsComp);
dht = dht.replace(
  /import \{ ChromeControlHint, GripVerticalIcon, MoveIcon \} from "[^"]+";/,
  `import { ChromeControlHint } from "${rel(dh, barrel)}";\nimport { GripVerticalIcon, MoveIcon } from "${iconsRel}";`,
);
writeFileSync(dhf, dht);
console.log("updated DragHandle icon imports");

// Rewire high-fanout icon symbols from W3-interim
const syms = new Set([
  "Icon","ControlIcon","renderControlIcon","IconSource","IconSizeToken","resolveIconSizePx",
  "CheckIcon","ChevronDownIcon","ChevronUpIcon","ChevronLeftIcon","ChevronRightIcon",
  "CloseIcon","Maximize2Icon","Minimize2Icon","ChevronDownIconAlt","CheckIconAlt","CloseIconAlt",
  "DocumentIcon","FolderIcon","FolderOpenIcon","GripVerticalIcon","MoveIcon",
  "decodeIcon","encodeIcon","classifyIconSelectorMode","IconSelectorMode","createIconComponent",
]);
function walk(d, out = []) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (n.endsWith(".tsx")) out.push(p);
  }
  return out;
}
let rewired = 0;
for (const file of walk(el)) {
  if (file === iconsComp) continue;
  let t = readFileSync(file, "utf8");
  if (!t.includes("index.tsx")) continue;
  const re = /import\s*\{([^}]+)\}\s*from\s*"([^"]+index\.tsx)";/g;
  let m;
  const reps = [];
  while ((m = re.exec(t))) {
    const parts = m[1].split(",").map((s) => s.trim()).filter(Boolean);
    const stay = [], move = [];
    for (const part of parts) {
      const mm = part.match(/^(type\s+)?(\w+)/);
      if (mm && syms.has(mm[2])) move.push(part);
      else stay.push(part);
    }
    if (!move.length) continue;
    let r = relative(dirname(file), iconsComp).replaceAll("\\", "/");
    if (!r.startsWith(".")) r = "./" + r;
    const direct = `import { ${[...new Set(move)].join(", ")} } from "${r}";`;
    reps.push([m[0], stay.length ? `import { ${stay.join(", ")} } from "${m[2]}";\n${direct}` : direct, !stay.length]);
  }
  if (!reps.length) continue;
  for (const [old, neu, dropped] of reps) {
    if (dropped) {
      t = t.replace(`// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.\n${old}`, neu);
      if (t.includes(old)) t = t.replace(old, neu);
    } else t = t.replace(old, neu);
  }
  writeFileSync(file, t);
  rewired++;
  console.log("rewired", relative(process.cwd(), file));
}
console.log("rewired files", rewired);
