#!/usr/bin/env bun
/** 🔍️ Find bundles that still have language source files at bundle root (should be in lang/ subdir). */
import { existsSync, readdirSync, statSync } from "node:fs";
import { basename, join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const LANGUAGE_TAGS = new Set(["js", "rs", "py", "go", "cs", "ts"]);
const SKIP = new Set(["node_modules", "target", "dist", ".venv", ".git", ".cursor", ".repo", "pkg", "generated", "example", "manifest"]);

function isBundle(d: string): boolean {
  const n = readdirSync(d);
  return n.includes("package.json") || n.includes("Cargo.toml") || n.includes("go.mod") || n.some((x) => x.endsWith(".csproj"));
}

function walk(d: string, out: string[] = []): string[] {
  for (const name of readdirSync(d)) {
    if (SKIP.has(name)) continue;
    const p = join(d, name);
    if (!statSync(p).isDirectory()) continue;
    if (isBundle(p) && !LANGUAGE_TAGS.has(basename(p))) {
      const rootSources = readdirSync(p).filter((f) => {
        if (f === "script.ts" || f === "package.json" || f === "project.json") return false;
        return /\.(ts|tsx|rs|py|go|cs)$/.test(f) || f === "Cargo.toml" || f === "go.mod" || f.endsWith(".csproj");
      });
      if (rootSources.length) out.push(`${p.replace(REPO + "/", "")}: ${rootSources.join(", ")}`);
    }
    walk(p, out);
  }
  return out;
}

for (const line of walk(REPO)) console.log(line);
