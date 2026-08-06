import { readdirSync, readFileSync, writeFileSync, mkdirSync, statSync } from "fs";
import { join, dirname, relative } from "path";

const paths = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
const el = paths[1];
const barrel = paths[2];
const lines = readFileSync(barrel, "utf8").split("\n");

function child(dir, pred) {
  return readdirSync(dir).find((n) => pred(n));
}
const core = join(el, child(el, (n) => n.includes("core")));
const portsFile = join(core, child(core, (n) => n.includes("Ports")), child(join(core, child(core, (n) => n.includes("Ports"))), (n) => n.endsWith(".tsx")));
const cnFile = join(core, child(core, (n) => n.includes("ClassNames")), child(join(core, child(core, (n) => n.includes("ClassNames"))), (n) => n.endsWith(".tsx")));
const uiLabelFile = join(core, child(core, (n) => n.includes("UiLabel")), child(join(core, child(core, (n) => n.includes("UiLabel"))), (n) => n.endsWith(".tsx")));
const agDir = join(el, "ActionGroup");
const compName = child(agDir, (n) => n.endsWith("component.tsx"));
const agLines = readFileSync(join(agDir, compName), "utf8").split("\n");
const adaptersOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"));
const adaptersClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("Adapters"));
const interimLine = agLines.find((l) => l.includes("W3-interim") && l.includes("remaining symbols"));
const headerSrc = agLines.slice(0, 6);
function makeHeader(name) {
  return [headerSrc[0], headerSrc[1].replace(/elements\/ActionGroup/, `elements/${name}`), ...headerSrc.slice(2)].join("\n");
}
const rel = (fromDir, toFile) => {
  let r = relative(fromDir, toFile).replaceAll("\\", "/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};

// Find region
let regionStart = -1, regionEnd = -1, cutStart = -1;
for (let i = 0; i < lines.length; i++) {
  if (/^\/\/ #region .*ContextMenu/.test(lines[i])) regionStart = i;
  if (regionStart >= 0 && cutStart < 0 && lines[i].startsWith("const contextMenuShortcutClassName")) cutStart = i;
  if (regionStart >= 0 && /^\/\/ #endregion .*ContextMenu/.test(lines[i])) {
    regionEnd = i;
    break;
  }
}
if (regionStart < 0 || cutStart < 0 || regionEnd < 0) {
  console.error("bounds", { regionStart, cutStart, regionEnd });
  process.exit(1);
}

const bodyLines = lines.slice(cutStart, regionEnd); // exclusive endregion
const body = bodyLines.join("\n");

// Collect exports
const exportNames = [];
const typeExportNames = [];
for (const l of bodyLines) {
  let m = l.match(/^export (?:async )?function (\w+)/);
  if (m) { exportNames.push(m[1]); continue; }
  m = l.match(/^export const (\w+)/);
  if (m) { exportNames.push(m[1]); continue; }
  m = l.match(/^export interface (\w+)/);
  if (m) { typeExportNames.push(m[1]); continue; }
  m = l.match(/^export type (\w+)/);
  if (m) { typeExportNames.push(m[1]); continue; }
}

const cmDir = join(el, "ContextMenu");
mkdirSync(cmDir, { recursive: true });
const cmComp = join(cmDir, compName);
const barrelRel = rel(cmDir, barrel);
const portsRel = rel(cmDir, portsFile);
const cnRel = rel(cmDir, cnFile);
const uiLabelRel = rel(cmDir, uiLabelFile);

// Region markers from barrel
const regionOpen = lines[regionStart];
const regionClose = lines[regionEnd];

const file = `${makeHeader("ContextMenu")}

${adaptersOpen}
import * as React from "react";
import { createPortal } from "react-dom";
import { reactHostPort } from "${portsRel}";
import { cn } from "${cnRel}";
import { type UiLabel } from "${uiLabelRel}";
${interimLine}
import {
  floatingMenuItemClass,
  ContextMenuChrome,
  type IconSource,
  Icon,
  useLabel,
  createDOMEventBinding,
  getElementById,
} from "${barrelRel}";
${adaptersClose}

${regionOpen.replace("// #region", "// #region")}
${body}
${regionClose}
`;

// Fix: createDOMEventBinding and getElementById are IN the body we're extracting - remove from interim import
// Also queryElement is in body. renderPortalInto uses createPortal locally.

let fixedFile = file.replace(
  `import {
  floatingMenuItemClass,
  ContextMenuChrome,
  type IconSource,
  Icon,
  useLabel,
  createDOMEventBinding,
  getElementById,
} from "${barrelRel}";`,
  `import {
  floatingMenuItemClass,
  ContextMenuChrome,
  type IconSource,
  Icon,
  useLabel,
} from "${barrelRel}";`,
);

// Ensure body functions that were previously non-exported stay; exports stay.
// Remove duplicate region open inside body if we already have regionOpen - body doesn't include region open
writeFileSync(cmComp, fixedFile);
console.log("WROTE", cmComp, "exports", exportNames.length, "types", typeExportNames.length);

// Build barrel replacement: keep preamble (regionStart+1 .. cutStart-1), then import-then-export, then endregion
const preamble = lines.slice(regionStart + 1, cutStart).join("\n");
const barrelDir = dirname(barrel);
const cmImportPath = rel(barrelDir, cmComp);

const valueExports = [...new Set(exportNames)];
const typeExports = [...new Set(typeExportNames)];
const importList = [
  ...valueExports,
  ...typeExports.map((t) => `type ${t}`),
].join(", ");
const exportList = [
  ...valueExports,
  ...typeExports.map((t) => `type ${t}`),
].join(", ");

const replacement = `${regionOpen}
${preamble}

import { ${importList} } from "${cmImportPath}";
export { ${exportList} };
${regionClose}
`;

const before = lines.slice(0, regionStart).join("\n");
const after = lines.slice(regionEnd + 1).join("\n");
const newBarrel = before + "\n" + replacement + "\n" + after;
writeFileSync(barrel, newBarrel);

// balance
const nl = newBarrel.split("\n");
let o = 0, c = 0;
for (const l of nl) {
  if (/^\/\/\s*#region\b/.test(l)) o++;
  if (/^\/\/\s*#endregion\b/.test(l)) c++;
}
console.log({ opens: o, closes: c, balanced: o === c, lines: nl.length });
console.log("valueExports", valueExports);
console.log("typeExports", typeExports);
// verify no leftover function ContextMenu in barrel body
if (/\nexport const ContextMenu:/.test(newBarrel) && newBarrel.indexOf("export const ContextMenu:") !== newBarrel.lastIndexOf("export const ContextMenu:")) {
  console.error("duplicate ContextMenu export?");
}
if (newBarrel.includes("export const ContextMenu: React.FC")) {
  console.error("OLD ContextMenu DEF STILL IN BARREL");
  process.exit(1);
}
console.log("ContextMenu def removed from barrel OK");
