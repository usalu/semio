#!/usr/bin/env bun
/** ⚛️ Hoist react/js sources back to react bundle roots — react is a framework tag, not a role. */
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

const reactJsDirs = walk(REPO).filter((d) => basename(d) === "js" && basename(dirname(d)) === "react");
console.log(`[DEBUG] found ${reactJsDirs.length} react/js dirs`);

for (const jsDir of reactJsDirs.sort()) {
  const reactDir = dirname(jsDir);
  for (const name of readdirSync(jsDir).sort()) {
    const from = join(jsDir, name);
    const to = join(reactDir, name);
    if (existsSync(to)) {
      console.warn(`skip conflict: ${rel(to)} already exists`);
      continue;
    }
    gitMv(from, to);
    console.log(`moved ${rel(from)} -> ${rel(to)}`);
  }
  if (readdirSync(jsDir).length === 0) {
    rmSync(jsDir, { recursive: true, force: true });
    console.log(`removed empty ${rel(jsDir)}`);
  }
}

const touched = new Set<string>();

function patchFile(path: string, replacer: (text: string) => string): void {
  if (!existsSync(path) || !statSync(path).isFile()) return;
  const before = readFileSync(path, "utf8");
  const after = replacer(before);
  if (after === before) return;
  writeFileSync(path, after);
  touched.add(rel(path));
}

function walkFiles(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    if (SKIP.has(name)) continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) out.push(...walkFiles(p));
    else if (/\.(ts|tsx|json|md)$/.test(name)) out.push(p);
  }
  return out;
}

for (const file of walkFiles(REPO)) {
  patchFile(file, (text) => text.replaceAll("/react/js/", "/react/").replaceAll('"./js/', '"./').replaceAll("'./js/", "'./").replaceAll('"js/vitest.config.ts"', '"vitest.config.ts"'));
}

console.log(`[DEBUG] patched ${touched.size} files`);
for (const f of [...touched].sort()) console.log(`  ${f}`);
