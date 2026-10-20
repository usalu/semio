import { readFileSync, existsSync, statSync } from "node:fs";
import { dirname, join, relative, resolve, isAbsolute } from "node:path";
import { ROOT } from "./targets.ts";

const transpiler = new Bun.Transpiler({ loader: "tsx" });
const sourceCache = new Map<string, string>();
const importCache = new Map<string, string[]>();
const SRC = /\.[cm]?[jt]sx?$/;

export const read = (abs: string) => {
  let s = sourceCache.get(abs);
  if (s === undefined) { s = readFileSync(abs, "utf8"); sourceCache.set(abs, s); }
  return s;
};

export function stripComments(src: string): string {
  let out = "", i = 0;
  const n = src.length;
  while (i < n) {
    const c = src[i]!, d = src[i + 1];
    if (c === "/" && d === "/") { while (i < n && src[i] !== "\n") i++; continue; }
    if (c === "/" && d === "*") { i += 2; while (i < n && !(src[i] === "*" && src[i + 1] === "/")) { if (src[i] === "\n") out += "\n"; i++; } i += 2; continue; }
    if (c === '"' || c === "'" || c === "`") {
      const q = c; out += c; i++;
      while (i < n && src[i] !== q) { if (src[i] === "\\") { out += src[i]!; i++; } out += src[i]!; i++; }
      out += q; i++; continue;
    }
    out += c; i++;
  }
  return out;
}

function resolveSpec(from: string, spec: string): string | null {
  if (spec.startsWith("node:") || spec.startsWith("bun:")) return null;
  try {
    const r = Bun.resolveSync(spec, dirname(from));
    if (r.includes("/node_modules/") || !r.startsWith(ROOT + "/") || !SRC.test(r)) return null;
    return r;
  } catch { return null; }
}

export function importsOf(abs: string): string[] {
  const hit = importCache.get(abs);
  if (hit) return hit;
  let specs: string[] = [];
  try { specs = transpiler.scanImports(read(abs)).map((i) => i.path); } catch { specs = [...read(abs).matchAll(/(?:from|import)\s*\(?\s*["']([^"']+)["']/g)].map((m) => m[1]!); }
  const out: string[] = [];
  for (const s of specs) { const r = resolveSpec(abs, s); if (r) out.push(r); }
  for (const m of stripComments(read(abs)).matchAll(/import\(\s*["']([^"']+)["']\s*\)/g)) { const r = resolveSpec(abs, m[1]!); if (r) out.push(r); }
  importCache.set(abs, out);
  return out;
}

const literalCache = new Map<string, string[]>();
export function literalFiles(abs: string, repoRoot = ROOT): string[] {
  const hit = literalCache.get(abs);
  if (hit) return hit;
  const out = new Set<string>();
  for (const m of stripComments(read(abs)).matchAll(/["'`]([^"'`\n]{3,300}\.(?:ts|tsx|mjs|js))["'`]/g)) {
    const lit = m[1]!;
    if (lit.includes("${") || lit.includes("*")) continue;
    for (const base of [dirname(abs), repoRoot]) {
      const p = resolve(base, lit);
      if (p.startsWith(ROOT + "/") && !p.includes("/node_modules/") && existsSync(p) && statSync(p).isFile()) { out.add(p); break; }
    }
  }
  const r = [...out];
  literalCache.set(abs, r);
  return r;
}

export function closure(entries: string[], withLiterals = true): string[] {
  const seen = new Set<string>();
  const stack = [...entries];
  while (stack.length) {
    const f = stack.pop()!;
    if (seen.has(f)) continue;
    seen.add(f);
    for (const i of importsOf(f)) stack.push(i);
    if (withLiterals) for (const l of literalFiles(f)) stack.push(l);
  }
  return [...seen].sort();
}
export const rel = (abs: string) => relative(ROOT, abs);
