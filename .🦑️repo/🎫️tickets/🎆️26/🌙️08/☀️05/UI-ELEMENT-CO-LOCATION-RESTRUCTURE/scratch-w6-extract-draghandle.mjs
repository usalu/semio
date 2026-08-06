import { readdirSync, readFileSync, writeFileSync, mkdirSync, existsSync } from "fs";
import { join, dirname, relative } from "path";

const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
function resolveUnder(parent, bare) {
  if (existsSync(join(parent, bare))) return join(parent, bare);
  const hit = readdirSync(parent).find((n) => n === bare || (n.endsWith(bare) && !n.slice(0, -bare.length).includes(bare)));
  // prefer exact endswith where prefix has no letters from bare
  const hits = readdirSync(parent).filter((n) => n.endsWith(bare));
  if (hits.length === 1) return join(parent, hits[0]);
  if (hits.length > 1) {
    // prefer shortest
    hits.sort((a, b) => a.length - b.length);
    return join(parent, hits[0]);
  }
  throw new Error(`cannot resolve ${bare} under ${parent}`);
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
const ports = compFile(resolveUnder(core, "Ports"));
const cn = compFile(resolveUnder(core, "ClassNames"));
const ag = resolveUnder(el, "ActionGroup");
const agLines = readFileSync(compFile(ag), "utf8").split("\n");
const adaptersOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"));
const adaptersClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("Adapters"));
const interimLine = agLines.find((l) => l.includes("W3-interim") && l.includes("remaining symbols"));
const headerSrc = agLines.slice(0, 6);
const makeHeader = (name) => [headerSrc[0], headerSrc[1].replace(/elements\/[^/]+/, `elements/${name}`), ...headerSrc.slice(2)].join("\n");
const compName = readdirSync(ag).find((n) => n.endsWith("component.tsx"));

let lines = readFileSync(barrel, "utf8").split("\n");
let start = -1, end = -1;
for (let i = 0; i < lines.length; i++) {
  if (/^\/\/\s*#region .*DragAffordance/.test(lines[i])) start = i;
  if (start >= 0 && /^\/\/\s*#endregion .*DragAffordance/.test(lines[i])) { end = i; break; }
}
if (start < 0) { console.error("DragAffordance missing - maybe already extracted?"); 
  // search for export const DragHandle
  for (let i=0;i<lines.length;i++) if (lines[i].includes("export const DragHandle")) console.log("found at", i+1, lines[i].slice(0,80));
  process.exit(1);
}

const regionOpen = lines[start].replace(/^\/\/#region/, "// #region");
const regionClose = lines[end].replace(/^\/\/#endregion/, "// #endregion");
// body without dropZoneReady trailing comments after DragHandle
const bodyLines = [];
for (let i = start + 1; i < end; i++) {
  if (lines[i].includes("dropZoneReady")) continue;
  bodyLines.push(lines[i]);
}

const dirName = "🧱DragHandle";
const dhDir = join(el, dirName);
mkdirSync(dhDir, { recursive: true });
const dhComp = join(dhDir, compName);

const file = `${makeHeader(dirName)}

${adaptersOpen}
import * as React from "react";
import { cn } from "${rel(dhDir, cn)}";
${interimLine}
import { ChromeControlHint, GripVerticalIcon, MoveIcon } from "${rel(dhDir, barrel)}";
${adaptersClose}

${regionOpen}
/** @emoji data-hover-scope attr for DragHandle hover exclusion. */
export const HANDLE_HOVER_SCOPE_ATTR = "data-hover-scope";

${bodyLines.join("\n")}
${regionClose}
`;
writeFileSync(dhComp, file);
console.log("WROTE", dhComp);

const importPath = rel(dirname(barrel), dhComp);
const replacement = `${regionOpen}
import { DragHandle, HANDLE_HOVER_SCOPE_ATTR } from "${importPath}";
export { DragHandle, HANDLE_HOVER_SCOPE_ATTR };
${regionClose}
`;
const before = lines.slice(0, start).join("\n");
const after = lines.slice(end + 1).join("\n");
let newBarrel = before + "\n" + replacement + "\n" + after;
// remove local HANDLE_HOVER_SCOPE_ATTR const if still present
newBarrel = newBarrel.replace(/^const HANDLE_HOVER_SCOPE_ATTR = "data-hover-scope";\n/m, "");
writeFileSync(barrel, newBarrel);
lines = newBarrel.split("\n");
let o=0,c=0; for (const l of lines){ if(/^\/\/\s*#region\b/.test(l))o++; if(/^\/\/\s*#endregion\b/.test(l))c++; }
console.log({opens:o,closes:c,balanced:o===c});
if (/export const DragHandle:/.test(newBarrel)) { console.error("OLD DEF"); process.exit(1); }
console.log("OK DragHandle");

// rewire DragHandle in leaves
const home = dhComp;
const syms = new Set(["DragHandle", "HANDLE_HOVER_SCOPE_ATTR"]);
function walk(d, out=[]) {
  for (const n of readdirSync(d)) {
    const p = join(d,n); const st = require("fs").statSync(p);
    if (st.isDirectory()) walk(p,out); else if (n.endsWith(".tsx")) out.push(p);
  }
  return out;
}
import { statSync } from "fs";
let rewired=0;
for (const file of walk(el)) {
  if (file === home) continue;
  let t = readFileSync(file,"utf8");
  if (!t.includes("index.tsx") || !/\bDragHandle\b/.test(t)) continue;
  const re = /import\s*\{([^}]+)\}\s*from\s*"([^"]+index\.tsx)";/g;
  let m; const reps=[];
  while ((m = re.exec(t))) {
    const parts = m[1].split(",").map(s=>s.trim()).filter(Boolean);
    const stay=[], move=[];
    for (const part of parts) {
      const mm = part.match(/^(type\s+)?(\w+)/);
      if (mm && syms.has(mm[2])) move.push(part); else stay.push(part);
    }
    if (!move.length) continue;
    let r = relative(dirname(file), home).replaceAll("\\","/"); if (!r.startsWith(".")) r="./"+r;
    const direct = `import { ${move.join(", ")} } from "${r}";`;
    reps.push([m[0], stay.length ? `import { ${stay.join(", ")} } from "${m[2]}";\n${direct}` : direct, !stay.length]);
  }
  if (!reps.length) continue;
  for (const [old, neu, dropped] of reps) {
    if (dropped) {
      t = t.replace(`// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.\n${old}`, neu);
      if (t.includes(old)) t = t.replace(old, neu);
    } else t = t.replace(old, neu);
  }
  writeFileSync(file, t); rewired++; console.log("rewired", relative(process.cwd(), file));
}
console.log("rewired", rewired);
