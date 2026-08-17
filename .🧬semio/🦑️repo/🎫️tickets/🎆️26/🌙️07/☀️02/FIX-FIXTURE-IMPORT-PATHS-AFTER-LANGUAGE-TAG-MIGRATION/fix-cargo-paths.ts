#!/usr/bin/env bun
/** 🔧️ Fix Cargo.toml path dependencies after language-tag migration. */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "target", "dist", ".git", ".cursor", ".repo"]);

function walk(d: string, out: string[] = []): string[] {
  for (const n of readdirSync(d)) {
    if (SKIP.has(n)) continue;
    const p = join(d, n);
    if (statSync(p).isDirectory()) walk(p, out);
    else if (n === "Cargo.toml") out.push(p);
  }
  return out;
}

function resolveCrateDir(fromFile: string, relPath: string): string | null {
  const base = normalize(join(dirname(fromFile), relPath));
  const candidates = [base, join(base, "rs"), join(base, "..", "rs")];
  for (const c of candidates) {
    if (existsSync(join(c, "Cargo.toml"))) return c;
  }
  return null;
}

let fixed = 0;
for (const file of walk(REPO)) {
  let content = readFileSync(file, "utf8");
  let changed = false;
  content = content.replace(/path\s*=\s*"([^"]+)"/g, (full, p: string) => {
    if (p.endsWith("/rs") || p.endsWith("/rs/")) {
      const abs = normalize(join(dirname(file), p));
      if (existsSync(join(abs, "Cargo.toml"))) return full;
    }
    const resolved = resolveCrateDir(file, p);
    if (!resolved) return full;
    const newRel = normalize(join(dirname(file), p)).endsWith("/rs") ? p : join(p.replace(/\/$/, ""), "rs").replace(/\\/g, "/");
    const absNew = normalize(join(dirname(file), newRel));
    if (absNew === normalize(join(dirname(file), p))) return full;
    if (existsSync(join(absNew, "Cargo.toml"))) {
      changed = true;
      return `path = "${newRel.replace(/\\/g, "/")}"`;
    }
    return full;
  });
  if (changed) {
    writeFileSync(file, content);
    fixed++;
    console.log(file.replace(REPO + "/", ""));
  }
}
console.log(`fixed ${fixed} Cargo.toml files`);
