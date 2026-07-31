#!/usr/bin/env bun
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose";
const replacements: [string, string][] = [
  ["@puzzle/board-wasm", "@puzzle/2d-wasm"],
  ["@puzzle/board", "@puzzle/2d-react"],
  ["@puzzle/scene", "@puzzle/3d-react"],
  ["@puzzle/topology", "@puzzle/5d-react"],
  ["@semio-tech/framework-playground-core-react", "@semio-tech/framework-playground-core-renderer-react"],
];

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    if (name === "node_modules" || name === "dist" || name === "test-results" || name === ".repo") continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) out.push(...walk(p));
    else if (/\.(ts|tsx|json|mjs)$/.test(name)) out.push(p);
  }
  return out;
}

for (const base of ["puzzle", "framework", "cad", ".storybook"]) {
  for (const file of walk(join(root, base))) {
    let c = readFileSync(file, "utf8");
    const orig = c;
    for (const [from, to] of replacements) c = c.split(from).join(to);
    if (c !== orig) writeFileSync(file, c);
  }
}

for (const file of ["package.json", "script.ts", ".vscode/launch.json"]) {
  const p = join(root, file);
  let c = readFileSync(p, "utf8");
  const orig = c;
  for (const [from, to] of replacements) c = c.split(from).join(to);
  c = c.split("@puzzle/2d-react:dev").join("@puzzle/2d-play:dev").split("@puzzle/3d-react:dev").join("@puzzle/3d-play:dev");
  if (c !== orig) writeFileSync(p, c);
}

console.log("[rename-scopes] done");
