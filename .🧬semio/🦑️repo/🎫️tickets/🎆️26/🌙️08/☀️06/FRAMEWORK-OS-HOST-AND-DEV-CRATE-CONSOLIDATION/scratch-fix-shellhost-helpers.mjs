import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const tick = process.argv[2];
const plan = JSON.parse(readFileSync(join(tick, "🧪shellhost-import-plan.json"), "utf8"));

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
        if (name === "node_modules" || name === "target" || name === "dist") continue;
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
const changedExports = [];
for (const name of plan.need_export) {
  const re = new RegExp(`^(const|async function|function) ${name}\\b`, "m");
  const m = hl.match(re);
  if (!m) {
    console.log("WARN no decl for", name);
    continue;
  }
  if (hl.includes(`export ${m[1]} ${name}`) || hl.match(new RegExp(`^export (?:async )?function ${name}\\b|^export const ${name}\\b`, "m"))) {
    console.log("already exported", name);
    continue;
  }
  const kind = m[1];
  const repl =
    kind === "const"
      ? `export const ${name}`
      : kind === "async function"
        ? `export async function ${name}`
        : `export function ${name}`;
  hl = hl.replace(re, repl);
  changedExports.push(name);
}
writeFileSync(helpersPath, hl);
console.log("exported", changedExports.length, changedExports);

hl = readFileSync(helpersPath, "utf8");
const existingKeep = [
  "DEFAULT_PANEL_WIDTH_PX",
  "dispatchOpenedFiles",
  "EMPTY_APP_LABELS_OVERLAY",
  "loadPluginModuleResilient",
  "runRequestMediaFrames",
  "shellLabel",
  "useUIHistory",
];
const allValues = [...new Set([...plan.need_import, ...existingKeep])].sort();
const needTypes = [...new Set(plan.need_type_import)].sort();

const missingExport = [];
for (const name of allValues) {
  const ok = new RegExp(
    `^export (?:async )?function ${name}\\b|^export const ${name}\\b`,
    "m",
  ).test(hl);
  if (!ok) missingExport.push(["value", name]);
}
for (const name of needTypes) {
  const ok = new RegExp(`^export (?:type|interface) ${name}\\b`, "m").test(hl);
  if (!ok) missingExport.push(["type", name]);
}
console.log("still missing export:", missingExport);

const valueLines = allValues.map((n) => `  ${n}`).join(",\n");
const typeLines = needTypes.map((n) => `  type ${n}`).join(",\n");
const importBlock =
  needTypes.length > 0
    ? `import {\n${valueLines},\n${typeLines},\n} from "../ShellHelpers/🟦️component.tsx";`
    : `import {\n${valueLines},\n} from "../ShellHelpers/🟦️component.tsx";`;

let ht = readFileSync(hostPath, "utf8");
const importRe =
  /import \{\n(?:  [^\n]+\n)+\} from "\.\.\/ShellHelpers\/[^"]+";/;
if (!importRe.test(ht)) {
  console.error("ShellHelpers import block not found");
  process.exit(1);
}
ht = ht.replace(importRe, importBlock);
writeFileSync(hostPath, ht);
console.log("updated ShellHost import with", allValues.length, "values +", needTypes.length, "types");
console.log("host", hostPath);
console.log("helpers", helpersPath);
writeFileSync(
  join(tick, "🧪shellhost-helpers-fix.json"),
  JSON.stringify({ changedExports, allValues, needTypes, missingExport }, null, 2),
);
