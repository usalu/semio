import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const dir = join(root, "repo/lib/js/src");

const ORDER = [
  "breach.ts",
  "cli.ts",
  "linter.ts",
  "script.ts",
  "dependency-boundary.ts",
  "policy-runner.ts",
  "policy-cli.ts",
  "runner.ts",
  "bundle-script.ts",
] as const;

function stripFile(src: string, file: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[i]?.includes("#region") && lines[i]?.includes("Header")) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  const out: string[] = [];
  for (; i < lines.length; i++) {
    const line = lines[i]!;
    if (/^import\s+.*from\s+["']\.\/[^"']+["'];?\s*$/.test(line.trim())) continue;
    if (/^export\s+\{[^}]+\}\s+from\s+["']\.\/[^"']+["'];?\s*$/.test(line.trim())) continue;
    out.push(line);
  }
  const name = file.replace(/\.ts$/, "");
  return `//#region 🔖${name}\n${out.join("\n").trim()}\n//#endregion 🔖${name}\n`;
}

const header = `//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @repo/lib/js: bundle scripts, policy runner, linters, dependency-boundary lint.
//#endregion 🧲Header

`;

let body = "";
for (const file of ORDER) {
  body += stripFile(readFileSync(join(dir, file), "utf8"), file) + "\n";
}

writeFileSync(join(dir, "index.ts"), header + body);

for (const file of ORDER) {
  if (file === "breach.ts") continue;
  unlinkSync(join(dir, file));
}
unlinkSync(join(dir, "breach.ts"));

console.log("merged repo/lib/js/src into index.ts");
