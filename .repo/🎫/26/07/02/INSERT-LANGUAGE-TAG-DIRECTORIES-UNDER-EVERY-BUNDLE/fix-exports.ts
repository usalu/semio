#!/usr/bin/env bun
/** 🔧 Fix package.json exports to point at js/ or rs/pkg after language-tag migration. */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "target", "dist", ".venv", ".git", ".cursor", ".repo", "pkg"]);

function walk(d: string, out: string[] = []): string[] {
  for (const name of readdirSync(d)) {
    if (SKIP.has(name)) continue;
    const p = join(d, name);
    if (statSync(p).isDirectory()) walk(p, out);
    else if (name === "package.json" && p !== join(REPO, "package.json")) out.push(p);
  }
  return out;
}

for (const pkgPath of walk(REPO)) {
  const dir = pkgPath.replace(/\/package\.json$/, "");
  const rel = dir.replace(REPO + "/", "");
  if (rel.endsWith("/js") || rel.endsWith("/rs") || rel.endsWith("/go") || rel.endsWith("/py") || rel.endsWith("/cs")) continue;
  const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as Record<string, unknown>;
  let changed = false;
  const jsIndex = existsSync(join(dir, "js/index.ts")) ? "./js/index.ts" : existsSync(join(dir, "js/index.tsx")) ? "./js/index.tsx" : null;
  if (jsIndex) {
    if (!pkg.exports) {
      pkg.exports = { ".": jsIndex };
      changed = true;
    } else if (typeof pkg.exports === "object") {
      const exp = pkg.exports as Record<string, string>;
      for (const [k, v] of Object.entries(exp)) {
        if (typeof v !== "string" || v.startsWith("./js/") || v.startsWith("./rs/")) continue;
        if (v === "./index.ts" || v === "./index.tsx" || v.endsWith("/index.ts") || v.endsWith("/index.tsx")) {
          exp[k] = jsIndex;
          changed = true;
        }
      }
    }
  }
  if (changed) writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
}

console.log("exports fix complete");
