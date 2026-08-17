import { readdirSync, readFileSync, writeFileSync, statSync, existsSync } from "fs";
import { join, dirname, relative } from "path";

const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
const core = join(el, readdirSync(el).find((n) => n.includes("core")));
console.log("core", core, readdirSync(core));

function resolveUnder(parent, bareName) {
  if (existsSync(join(parent, bareName))) return bareName;
  const hit = readdirSync(parent).find((n) => n === bareName || n.endsWith(bareName));
  return hit || bareName;
}

const coreMap = {};
for (const bare of ["Ports", "ClassNames", "UiLabel", "ElementId", "Label"]) {
  coreMap[bare] = resolveUnder(core, bare);
}
console.log("coreMap", coreMap);

function rewrite(text) {
  let t = text;
  // elements/core/Bare or ../core/Bare or .../core/Bare
  for (const [bare, real] of Object.entries(coreMap)) {
    if (bare === real) continue;
    const re = new RegExp(`(core\\/)${bare}(\\/)`, "g");
    t = t.replace(re, `$1${real}$2`);
  }
  return t;
}

let fixed = 0;
function walk(d, out = []) {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (/\.(tsx|ts)$/.test(n)) out.push(p);
  }
  return out;
}

for (const file of [barrel, ...walk(el)]) {
  const t = readFileSync(file, "utf8");
  const n = rewrite(t);
  if (n !== t) {
    writeFileSync(file, n);
    fixed++;
    console.log("fixed", relative(process.cwd(), file));
  }
}
console.log("fixed", fixed);

// verify barrel core imports
const t = readFileSync(barrel, "utf8");
for (const m of t.matchAll(/from "([^"]*core\/[^"]*)"/g)) {
  const abs = join(dirname(barrel), m[1]);
  if (!existsSync(abs)) console.error("MISSING", m[1]);
  else console.log("OK", m[1]);
}

// verify ButtonGroup imports
const bg = join(el, resolveUnder(el, "ButtonGroup"), readdirSync(join(el, resolveUnder(el, "ButtonGroup"))).find((n) => n.endsWith("component.tsx")));
console.log("ButtonGroup file", bg);
console.log(readFileSync(bg, "utf8").split("\n").filter((l) => l.startsWith("import")).join("\n"));
