import { readdirSync, readFileSync, writeFileSync, statSync, existsSync } from "fs";
import { join, dirname, relative } from "path";

const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");

// Map bare name -> actual dir name
const dirBySuffix = {};
for (const n of readdirSync(el)) {
  // strip leading emoji/symbols roughly: take trailing PascalCase-ish
  const m = n.match(/([A-Za-z][A-Za-z0-9]*)$/);
  if (m) dirBySuffix[m[1]] = n;
  dirBySuffix[n] = n;
}

function resolveElementDir(name) {
  if (existsSync(join(el, name))) return name;
  if (dirBySuffix[name]) return dirBySuffix[name];
  const hit = readdirSync(el).find((n) => n.endsWith(name));
  return hit || name;
}

function rewritePath(p) {
  // match .../elements/NAME/...
  return p.replace(/elements\/([^/"']+)\//g, (full, name) => {
    const resolved = resolveElementDir(name);
    if (resolved === name) return full;
    return full.replace(`elements/${name}/`, `elements/${resolved}/`);
  });
}

let fixedFiles = 0;
function walk(d, out = []) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (/\.(tsx|ts|jsx|js)$/.test(n)) out.push(p);
  }
  return out;
}

// Fix barrel
let barrelText = readFileSync(barrel, "utf8");
const barrelNew = rewritePath(barrelText);
if (barrelNew !== barrelText) {
  writeFileSync(barrel, barrelNew);
  fixedFiles++;
  console.log("fixed barrel");
}

// Fix all element leaves
for (const file of walk(el)) {
  let t = readFileSync(file, "utf8");
  const n = rewritePath(t);
  // also fix relative sibling imports like ../Button/ ../ButtonGroup/ ../ActionGroup/
  let n2 = n.replace(/from\s+"(\.\.\/)([A-Za-z][A-Za-z0-9]*)\//g, (full, dots, name) => {
    const resolved = resolveElementDir(name);
    if (resolved === name) return full;
    return full.replace(`${dots}${name}/`, `${dots}${resolved}/`);
  });
  // and "./Button" style won't apply
  if (n2 !== t) {
    writeFileSync(file, n2);
    fixedFiles++;
    console.log("fixed", relative(process.cwd(), file));
  }
}

console.log("dir map Button->", resolveElementDir("Button"), "ButtonGroup->", resolveElementDir("ButtonGroup"), "ContextMenu->", resolveElementDir("ContextMenu"), "Icons->", resolveElementDir("Icons"), "ActionGroup->", resolveElementDir("ActionGroup"), "Select->", resolveElementDir("Select"));
console.log("fixedFiles", fixedFiles);

// verify imports resolve
const t = readFileSync(barrel, "utf8");
for (const m of t.matchAll(/from "([^"]*elements\/[^"]*)"/g)) {
  const rel = m[1];
  // resolve from barrel dir
  const abs = join(dirname(barrel), rel);
  if (!existsSync(abs)) console.error("MISSING", rel, "->", abs);
}
console.log("barrel element import existence check done");
