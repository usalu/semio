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

const iconsDir = resolveUnder(el, "Icons");
const iconsComp = compFile(iconsDir);
const iconsText = readFileSync(iconsComp, "utf8");

// Collect exports from icons file
const valueExports = [];
const typeExports = [];
for (const l of iconsText.split("\n")) {
  let m = l.match(/^export (?:async )?function (\w+)/);
  if (m) { valueExports.push(m[1]); continue; }
  m = l.match(/^export const (\w+)/);
  if (m) { valueExports.push(m[1]); continue; }
  m = l.match(/^export interface (\w+)/);
  if (m) { typeExports.push(m[1]); continue; }
  m = l.match(/^export type (\w+)/);
  if (m) { typeExports.push(m[1]); continue; }
  m = l.match(/^export \{([^}]+)\}/);
  if (m) {
    for (const p of m[1].split(",")) {
      const name = p.trim();
      if (name) valueExports.push(name);
    }
  }
}
const values = [...new Set(valueExports)];
const types = [...new Set(typeExports)];
console.log("exports", values.length, types.length);

let lines = readFileSync(barrel, "utf8").split("\n");
// If Icon region still has body, replace it
let start = -1, end = -1;
for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes("#region") && lines[i].includes("🔖️Icon") && !lines[i].includes("IconRender")) start = i;
  if (start >= 0 && lines[i].includes("#endregion") && lines[i].includes("🔖️Icon") && !lines[i].includes("Codec")) { end = i; break; }
}
console.log("barrel Icon region", start + 1, end + 1);
if (start < 0 || end < 0) {
  console.error("region not found");
  process.exit(1);
}

// Check if already stubbed
const regionText = lines.slice(start, end + 1).join("\n");
if (regionText.includes("createIconComponent") || regionText.includes("export function Icon")) {
  const importPath = rel(dirname(barrel), iconsComp);
  const importList = [...values, ...types.map((t) => `type ${t}`)].join(", ");
  const replacement = `${lines[start]}
import { ${importList} } from "${importPath}";
export { ${importList} };
${lines[end]}
`;
  const newBarrel = lines.slice(0, start).join("\n") + "\n" + replacement + "\n" + lines.slice(end + 1).join("\n");
  writeFileSync(barrel, newBarrel);
  console.log("patched barrel Icon region");
  lines = newBarrel.split("\n");
} else {
  console.log("Icon region already stub?");
}

let o = 0, c = 0;
for (const l of lines) {
  if (/^\/\/\s*#region\b/.test(l)) o++;
  if (/^\/\/\s*#endregion\b/.test(l)) c++;
}
console.log({ opens: o, closes: c, balanced: o === c });

// Verify icons file has needed imports - check for activeUiTheme usage
if (iconsText.includes("activeUiTheme") && !iconsText.includes('from "@semio-tech/ui-styling"')) {
  console.error("missing styling import");
}
// Check METABOLISM etc
const head = iconsText.split("\n").slice(0, 50).join("\n");
console.log("--- icons head ---\n", head);

// Fix DragHandle
const dh = resolveUnder(el, "DragHandle");
const dhf = compFile(dh);
let dht = readFileSync(dhf, "utf8");
if (dht.includes("GripVerticalIcon") && dht.includes("index.tsx")) {
  dht = dht.replace(
    /import \{ ChromeControlHint, GripVerticalIcon, MoveIcon \} from "([^"]+)";/,
    `import { ChromeControlHint } from "$1";\nimport { GripVerticalIcon, MoveIcon } from "${rel(dh, iconsComp)}";`,
  );
  writeFileSync(dhf, dht);
  console.log("fixed DragHandle");
}

// Rewire
const syms = new Set([
  "Icon","ControlIcon","renderControlIcon","IconSource","IconSizeToken","resolveIconSizePx","IconProps",
  "CheckIcon","ChevronDownIcon","ChevronUpIcon","ChevronLeftIcon","ChevronRightIcon",
  "CloseIcon","Maximize2Icon","Minimize2Icon","ChevronDownIconAlt","CheckIconAlt","CloseIconAlt",
  "DocumentIcon","FolderIcon","FolderOpenIcon","GripVerticalIcon","MoveIcon",
  "decodeIcon","encodeIcon","classifyIconSelectorMode","IconSelectorMode","createIconComponent",
  "ChevronsUpDownIcon","PlusIcon","SearchIcon","SettingsIcon","LoaderIcon",
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
console.log("rewired", rewired);

// ButtonGroup still needs ControlIcon, renderControlIcon from Icons
const bg = resolveUnder(el, "ButtonGroup");
const bgf = compFile(bg);
let bgt = readFileSync(bgf, "utf8");
console.log("ButtonGroup still barrel?", bgt.includes("index.tsx"));
