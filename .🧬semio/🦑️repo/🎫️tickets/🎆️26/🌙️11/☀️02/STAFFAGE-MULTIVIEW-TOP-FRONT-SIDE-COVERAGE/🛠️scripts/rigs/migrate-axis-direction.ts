import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const L = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const write = process.argv[2] === "write";
const files: string[] = [];
const walk = (dir: string): void => {
  for (const entry of readdirSync(dir)) {
    if (entry === "node_modules") continue;
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) walk(path);
    else if (entry === "🔣️.json") files.push(path);
  }
};
for (const root of ["🗂️catalog", "🧫️fixtures"]) walk(join(L, root));
let touched = 0;
let solids = 0;
for (const file of files) {
  const text = readFileSync(file, "utf8");
  let count = 0;
  const next = text.replace(/("axisBone": "[^"]*",)(\r?\n)(\s*)(?!"axis":)/g, (_m, head: string, nl: string, indent: string) => {
    count++;
    return `${head}${nl}${indent}"axis": "u",${nl}${indent}`;
  });
  if (count > 0) {
    touched++;
    solids += count;
    if (write) writeFileSync(file, next);
  }
}
console.log(JSON.stringify({ write, files: touched, revolvedSolids: solids }));
