import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const tick = process.argv[2];

function findFrameworkRoot(root) {
  for (const name of readdirSync(root)) {
    const p = join(root, name);
    try {
      if (statSync(p).isDirectory() && name.includes("framework")) return p;
    } catch {}
  }
  throw new Error("framework not found");
}

function findElement(fw, fragment) {
  const stack = [fw];
  while (stack.length) {
    const dir = stack.pop();
    let entries;
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      const p = join(dir, name);
      let st;
      try {
        st = statSync(p);
      } catch {
        continue;
      }
      if (st.isDirectory()) {
        if (["node_modules", "target", "dist", ".git"].includes(name)) continue;
        stack.push(p);
      } else if (name.includes("component.tsx") && p.includes(fragment) && p.includes("elements")) {
        return p;
      }
    }
  }
  throw new Error(`element not found: ${fragment}`);
}

const root = "/Users/ueli/Documents/semio";
const fw = findFrameworkRoot(root);
const hostPath = findElement(fw, "ShellHost");
const helpersPath = findElement(fw, "ShellHelpers");

let hl = readFileSync(helpersPath, "utf8");
let ht = readFileSync(hostPath, "utf8");

function collectImported(src) {
  const imported = new Set();
  const re = /import\s+(?:type\s+)?(?:\{([^}]+)\}|\*\s+as\s+(\w+)|(\w+))\s+from/g;
  let m;
  while ((m = re.exec(src))) {
    if (m[1]) {
      for (const part of m[1].split(",")) {
        let p = part.trim();
        if (!p) continue;
        p = p.replace(/^type\s+/, "").trim();
        p = p.split(/\s+as\s+/).pop().trim();
        if (p) imported.add(p);
      }
    } else if (m[2]) imported.add(m[2]);
    else if (m[3]) imported.add(m[3]);
  }
  return imported;
}

function collectLocal(src) {
  const local = new Set();
  const re =
    /^(?:export\s+)?(?:async\s+)?function\s+(\w+)|^(?:export\s+)?(?:const|let|var|type|interface|class|enum)\s+(\w+)/gm;
  let m;
  while ((m = re.exec(src))) local.add(m[1] || m[2]);
  return local;
}

function helperDecls(src) {
  const exported = new Map(); // name -> kind
  const priv = new Map();
  let m;
  const expFn = /^export\s+(async\s+)?function\s+(\w+)/gm;
  while ((m = expFn.exec(src))) exported.set(m[2], m[1] ? "async function" : "function");
  const expConst = /^export\s+const\s+(\w+)/gm;
  while ((m = expConst.exec(src))) exported.set(m[1], "const");
  const expType = /^export\s+(?:type|interface)\s+(\w+)/gm;
  const types = new Set();
  while ((m = expType.exec(src))) types.add(m[1]);

  const privFn = /^(async\s+)?function\s+(\w+)/gm;
  while ((m = privFn.exec(src))) {
    if (!exported.has(m[2])) priv.set(m[2], m[1] ? "async function" : "function");
  }
  // const Name = OR const Name: Type =
  const privConst = /^const\s+(\w+)\s*(?::[^=]+)?=/gm;
  while ((m = privConst.exec(src))) {
    if (!exported.has(m[1])) priv.set(m[1], "const");
  }
  return { exported, priv, types };
}

// Strip imports and line comments for usage body
const body = ht
  .split("\n")
  .filter((line) => {
    const s = line.trim();
    return !(s.startsWith("import ") || s.startsWith("//") || s.startsWith("*") || s.startsWith("/*"));
  })
  .join("\n");

const imported = collectImported(ht);
const local = collectLocal(ht);
const { exported, priv, types } = helperDecls(hl);

const needExport = [];
const needImport = [];
for (const [name, kind] of [...exported, ...priv]) {
  if (imported.has(name) || local.has(name)) continue;
  if (!new RegExp(`\\b${name}\\b`).test(body)) continue;
  if (priv.has(name)) needExport.push(name);
  needImport.push(name);
}

const needTypeImport = [];
for (const name of types) {
  if (imported.has(name) || local.has(name)) continue;
  if (!new RegExp(`\\b${name}\\b`).test(body)) continue;
  needTypeImport.push(name);
}

console.log("needExport", needExport);
console.log("needImport", needImport.length, needImport);
console.log("needTypeImport", needTypeImport);

// Export privates
const changedExports = [];
for (const name of needExport) {
  const kind = priv.get(name);
  if (!kind) continue;
  if (kind === "const") {
    const re = new RegExp(`^const ${name}\\b`, "m");
    if (!re.test(hl)) {
      console.log("WARN const not found", name);
      continue;
    }
    hl = hl.replace(re, `export const ${name}`);
    changedExports.push(name);
  } else if (kind === "async function") {
    const re = new RegExp(`^async function ${name}\\b`, "m");
    hl = hl.replace(re, `export async function ${name}`);
    changedExports.push(name);
  } else {
    const re = new RegExp(`^function ${name}\\b`, "m");
    hl = hl.replace(re, `export function ${name}`);
    changedExports.push(name);
  }
}
writeFileSync(helpersPath, hl);
console.log("exported", changedExports);

// Rebuild full ShellHelpers import: merge ALL current ShellHelpers imports into one block
ht = readFileSync(hostPath, "utf8");
hl = readFileSync(helpersPath, "utf8");

// Collect all names already imported from ShellHelpers + new needs
const helperImportNames = new Set();
const helperTypeNames = new Set();
const helperImportRe =
  /import\s+(type\s+)?\{([^}]+)\}\s+from\s+"\.\.\/ShellHelpers\/[^"]+";/g;
let im;
const blocks = [];
while ((im = helperImportRe.exec(ht))) {
  blocks.push(im[0]);
  const isTypeOnly = !!im[1];
  for (const part of im[2].split(",")) {
    let p = part.trim();
    if (!p) continue;
    const isType = isTypeOnly || /^type\s+/.test(p);
    p = p.replace(/^type\s+/, "").trim();
    p = p.split(/\s+as\s+/).pop().trim();
    if (!p) continue;
    if (isType) helperTypeNames.add(p);
    else helperImportNames.add(p);
  }
}

for (const n of needImport) helperImportNames.add(n);
for (const n of needTypeImport) helperTypeNames.add(n);

// Always keep these if present in helpers
for (const n of [
  "DEFAULT_PANEL_WIDTH_PX",
  "dispatchOpenedFiles",
  "EMPTY_APP_LABELS_OVERLAY",
  "loadPluginModuleResilient",
  "runRequestMediaFrames",
  "shellLabel",
  "useUIHistory",
  "synthesizeLocalizedLabel",
]) {
  if (new RegExp(`^export (?:async )?function ${n}\\b|^export const ${n}\\b`, "m").test(hl)) {
    helperImportNames.add(n);
  }
}

const allValues = [...helperImportNames].sort();
const allTypes = [...helperTypeNames].sort();

// Verify exports exist
const missing = [];
for (const n of allValues) {
  if (!new RegExp(`^export (?:async )?function ${n}\\b|^export const ${n}\\b`, "m").test(hl)) {
    missing.push(["value", n]);
  }
}
for (const n of allTypes) {
  if (!new RegExp(`^export (?:type|interface) ${n}\\b`, "m").test(hl)) {
    missing.push(["type", n]);
  }
}
console.log("missing after export", missing);

const valueLines = allValues.map((n) => `  ${n}`).join(",\n");
const typeLines = allTypes.map((n) => `  type ${n}`).join(",\n");
const importBlock =
  allTypes.length > 0
    ? `import {\n${valueLines},\n${typeLines},\n} from "../ShellHelpers/🟦️component.tsx";`
    : `import {\n${valueLines},\n} from "../ShellHelpers/🟦️component.tsx";`;

// Remove all ShellHelpers import blocks, insert one at first location
let firstPos = -1;
ht = ht.replace(helperImportRe, (match, ...args) => {
  // lastIndex nonsense - use function with offset
  return match;
});

// Manual: find all matches with positions
const matches = [...ht.matchAll(/import\s+(type\s+)?\{([^}]+)\}\s+from\s+"\.\.\/ShellHelpers\/[^"]+";/g)];
if (matches.length === 0) {
  console.error("no ShellHelpers imports found");
  process.exit(1);
}
firstPos = matches[0].index;
// Remove from end to start
for (let i = matches.length - 1; i >= 0; i--) {
  const m = matches[i];
  ht = ht.slice(0, m.index) + ht.slice(m.index + m[0].length);
}
// Clean double blank lines around removal
ht = ht.replace(/\n{3,}/g, "\n\n");
// Recompute firstPos after removals — insert at original firstPos clamped
const insertAt = Math.min(firstPos, ht.length);
ht = ht.slice(0, insertAt) + importBlock + "\n" + ht.slice(insertAt);
ht = ht.replace(/\n{3,}/g, "\n\n");

writeFileSync(hostPath, ht);
console.log("wrote unified import", allValues.length, "values", allTypes.length, "types");
writeFileSync(
  join(tick, "🧪shellhost-helpers-fix-2.json"),
  JSON.stringify({ changedExports, allValues, allTypes, missing, needExport, needImport }, null, 2),
);
