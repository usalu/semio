#!/usr/bin/env bun
/** 🔗️ Proves that every pets alias and path the quiz integration added names an existing file: the site Vite config, the quiz React vitest config and its tsconfig. Usage: `bun alias_targets_check.ts` from the repository root. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const root = process.cwd();
const react = "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react";
const sources = [`🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts`, `${react}/🧪️tests/🎚️config/🟦️.ts`];
let missing = 0;
for (const source of sources) {
  for (const line of readFileSync(resolve(root, source), "utf8").split("\n").filter((entry) => entry.includes("pets"))) {
    const target = /resolve\(repoRoot, "([^"]+)"\)/u.exec(line)?.[1];
    if (target === undefined) continue;
    const found = existsSync(resolve(root, target));
    if (!found) missing += 1;
    process.stdout.write(`${found ? "ok     " : "MISSING"}  ${source}  ->  ${target}\n`);
  }
}
const tsconfig = resolve(root, react, "📦️packages/🟦️typescript/tsconfig.json");
const paths = (JSON.parse(readFileSync(tsconfig, "utf8")) as { compilerOptions: { paths: Record<string, string[]> } }).compilerOptions.paths;
for (const [name, targets] of Object.entries(paths).filter(([key]) => key.includes("pets"))) {
  const found = existsSync(resolve(dirname(tsconfig), targets[0]!));
  if (!found) missing += 1;
  process.stdout.write(`${found ? "ok     " : "MISSING"}  tsconfig ${name}  ->  ${targets[0]}\n`);
}
process.exit(missing === 0 ? 0 : 1);
