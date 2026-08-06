import { readdirSync, readFileSync, writeFileSync, mkdirSync, existsSync, statSync } from "fs";
import { join, dirname, relative } from "path";

const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  hits.sort((a, b) => a.length - b.length);
  if (!hits.length) throw new Error("no " + bare + " in " + parent);
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
const ports = compFile(resolveUnder(core, "Ports"));
const icons = compFile(resolveUnder(el, "Icons"));
const drag = compFile(resolveUnder(el, "DragHandle"));
const ribbon = compFile(resolveUnder(el, "Ribbon"));
const ag = resolveUnder(el, "ActionGroup");
const agLines = readFileSync(compFile(ag), "utf8").split("\n");
const adaptersOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"));
const adaptersClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("Adapters"));
const interimLine = agLines.find((l) => l.includes("W3-interim") && l.includes("remaining symbols"));
const headerSrc = agLines.slice(0, 6);

let lines = readFileSync(barrel, "utf8").split("\n");
let start = -1, end = -1;
for (let i = 0; i < lines.length; i++) {
  if (lines[i].startsWith("export type PanelTabBarVariant")) start = i;
  if (start >= 0 && /^\/\/ #region .*PanelDock/.test(lines[i])) { end = i; break; }
}
console.log("cut", start + 1, end + 1, "span", end - start);
if (start < 0 || end < 0) process.exit(1);

const body = lines.slice(start, end).join("\n");
const valueExports = [];
const typeExports = [];
for (const l of lines.slice(start, end)) {
  let m = l.match(/^export (?:async )?function (\w+)/);
  if (m) { valueExports.push(m[1]); continue; }
  m = l.match(/^export const (\w+)/);
  if (m) { valueExports.push(m[1]); continue; }
  m = l.match(/^export interface (\w+)/);
  if (m) { typeExports.push(m[1]); continue; }
  m = l.match(/^export type (\w+)/);
  if (m) { typeExports.push(m[1]); continue; }
}
console.log("valueExports", valueExports);
console.log("typeExports", typeExports);

const dirName = "PanelTabBar";
const dir = join(el, "\u{1F4D1}PanelTabBar");
mkdirSync(dir, { recursive: true });
const compName = readdirSync(ag).find((n) => n.endsWith("component.tsx"));
const comp = join(dir, compName);
const makeHeader = () => [headerSrc[0], headerSrc[1].replace(/elements\/[^/]+/, "elements/\u{1F4D1}PanelTabBar"), ...headerSrc.slice(2)].join("\n");

const file = makeHeader() + "\n\n" + adaptersOpen + "\n" +
  "import * as React from \"react\";\n" +
  "import { reactHostPort } from \"" + rel(dir, ports) + "\";\n" +
  "import { cn } from \"" + rel(dir, cn) + "\";\n" +
  "import { Icon, type ControlIcon, renderControlIcon } from \"" + rel(dir, icons) + "\";\n" +
  "import { DragHandle } from \"" + rel(dir, drag) + "\";\n" +
  "import { Ribbon, type RibbonRow } from \"" + rel(dir, ribbon) + "\";\n" +
  interimLine + "\n" +
  "import {\n" +
  "  type Anchor,\n" +
  "  type FlowBlock,\n" +
  "  ChromeControlHint,\n" +
  "  useLabel,\n" +
  "  useFlow,\n" +
  "  useNativeDragArm,\n" +
  "  usePanelDockContext,\n" +
  "  useUiDriverDragSurface,\n" +
  "  useSurfaceActive,\n" +
  "  panelTabIconSlotClass,\n" +
  "  panelTabLabelClass,\n" +
  "  interactiveActiveFillClass,\n" +
  "  chromeControlTabItemClass,\n" +
  "  panelTabButtonClass,\n" +
  "  panelTabStripClass,\n" +
  "  panelTabStripDividerClass,\n" +
  "  PANEL_TREE_UNIT_MIME,\n" +
  "  beginPanelTreeUnitDrag,\n" +
  "  endPanelTreeUnitDrag,\n" +
  "  readActivePanelTreeUnitDrag,\n" +
  "  usePanelTreeUnitDragActive,\n" +
  "  type PanelTreeUnit,\n" +
  "  type DockSkeleton,\n" +
  "  type DockTabSkeleton,\n" +
  "  ANCHORS,\n" +
  "  type UiStatus,\n" +
  "} from \"" + rel(dir, barrel) + "\";\n" +
  adaptersClose + "\n\n" +
  "// #region \u{1F4D1}PanelTabBar\n" +
  body + "\n" +
  "// #endregion \u{1F4D1}PanelTabBar\n";

writeFileSync(comp, file);
console.log("WROTE", comp);

const importPath = rel(dirname(barrel), comp);
const importList = [...new Set(valueExports), ...new Set(typeExports).map((t) => "type " + t)].join(", ");
const replacement = "// #region \u{1F4D1}PanelTabBar\n" +
  "import { " + importList + " } from \"" + importPath + "\";\n" +
  "export { " + importList + " };\n" +
  "// #endregion \u{1F4D1}PanelTabBar\n\n";

const newBarrel = lines.slice(0, start).join("\n") + "\n" + replacement + lines.slice(end).join("\n");
writeFileSync(barrel, newBarrel);
let o = 0, c = 0;
for (const l of newBarrel.split("\n")) {
  if (/^\/\/\s*#region\b/.test(l)) o++;
  if (/^\/\/\s*#endregion\b/.test(l)) c++;
}
console.log({ opens: o, closes: c, balanced: o === c });
if (newBarrel.includes("export const PanelTabBar:")) {
  console.error("OLD PanelTabBar still present");
  process.exit(1);
}
console.log("PanelTabBar extracted");

const syms = new Set([...valueExports, ...typeExports]);
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
for (const filePath of walk(el)) {
  if (filePath === comp) continue;
  let t = readFileSync(filePath, "utf8");
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
    let r = relative(dirname(filePath), comp).replaceAll("\\", "/");
    if (!r.startsWith(".")) r = "./" + r;
    const direct = "import { " + [...new Set(move)].join(", ") + " } from \"" + r + "\";";
    reps.push([m[0], stay.length ? "import { " + stay.join(", ") + " } from \"" + m[2] + "\";\n" + direct : direct, !stay.length]);
  }
  if (!reps.length) continue;
  for (const [old, neu, dropped] of reps) {
    if (dropped) {
      const marked = "// \u{1F6A7}\u{FE0F}W3-interim: remaining symbols still live in the ui-react barrel \u2014 clear before W6.\n" + old;
      if (t.includes(marked)) t = t.replace(marked, neu);
      else t = t.replace(old, neu);
    } else t = t.replace(old, neu);
  }
  writeFileSync(filePath, t);
  rewired++;
  console.log("rewired", relative(process.cwd(), filePath));
}
console.log("rewired", rewired);
