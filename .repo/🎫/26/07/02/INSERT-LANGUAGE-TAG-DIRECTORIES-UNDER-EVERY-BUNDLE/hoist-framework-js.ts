#!/usr/bin/env bun
/** 🧩 Hoist framework-tag bundle sources (r3f, react-renderer) out of nested js/. */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { execSync } from "node:child_process";

function findRepoRoot(start: string): string {
  let dir = start;
  while (dir !== dirname(dir)) {
    if (existsSync(join(dir, "package.json"))) {
      const pkg = JSON.parse(readFileSync(join(dir, "package.json"), "utf8")) as { name?: string };
      if (pkg.name === "compose") return dir;
    }
    dir = dirname(dir);
  }
  throw new Error("repo root not found");
}

const REPO = findRepoRoot(import.meta.dir);
const FRAMEWORK_TAGS = new Set(["r3f", "react-renderer"]);
const SKIP = new Set(["node_modules", "target", "dist", ".git", ".repo", ".cursor", "pkg"]);

function rel(p: string): string {
  return relative(REPO, p).replace(/\\/g, "/");
}

function gitMv(from: string, to: string): void {
  mkdirSync(dirname(to), { recursive: true });
  execSync(`git mv "${from}" "${to}"`, { cwd: REPO, stdio: "pipe" });
}

function walk(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    if (SKIP.has(name)) continue;
    const p = join(dir, name);
    if (!statSync(p).isDirectory()) continue;
    out.push(p);
    walk(p, out);
  }
  return out;
}

for (const jsDir of walk(REPO).filter((d) => basename(d) === "js" && FRAMEWORK_TAGS.has(basename(dirname(d))))) {
  const bundleDir = dirname(jsDir);
  for (const name of readdirSync(jsDir).sort()) {
    const from = join(jsDir, name);
    const to = join(bundleDir, name);
    if (existsSync(to)) continue;
    gitMv(from, to);
    console.log(`moved ${rel(from)} -> ${rel(to)}`);
  }
  if (readdirSync(jsDir).length === 0) rmSync(jsDir, { recursive: true, force: true });
}
