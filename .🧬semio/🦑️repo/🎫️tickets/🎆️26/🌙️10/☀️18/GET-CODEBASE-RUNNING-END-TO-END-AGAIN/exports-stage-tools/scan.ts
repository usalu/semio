/**
 * 🔭 One pass over every source file: relative edges whose target exists, bare imports of workspace packages. Cached in the generated folder.
 *
 *   bun scan.ts [--refresh]
 */
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { builtinModules } from "node:module";
import { GENERATED, ROOT, listAll, posix } from "./lib.ts";
const BUILTIN = new Set(builtinModules.map(m => m.replace(/^node:/u, "").split("/")[0]!));

export type RawEdge = { file: string; spec: string; target: string; kind: "static" | "dynamic" | "require" };
export type BareUse = { file: string; spec: string; pkg: string; sub: string };
export type ThirdUse = { file: string; pkg: string };
const SRC = /\.(ts|tsx|mts|cts|js|mjs|cjs)$/u;
const SPEC = /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\brequire\s*\(\s*|\bcreateRequire\([^)\n]*\)\s*\(\s*)(["'])([^"'\n]+)\1/gu;
const DO_NOT_FOLLOW = /(^|\/)\.cache(\/|$)|(^|\/)\.git(\/|$)|(^|\/)\.nx(\/|$)|(^|\/)dist(\/|$)|(^|\/)target(\/|$)|(^|\/)📤️dist(\/|$)|(^|\/)🗑️generated(\/|$)|(^|\/)🔌️plugin-modules(\/|$)|(^|\/)node_modules(\/|$)|(^|\/)📦️packages\/🦀️rust\/pkg(\/|$)|(^|\/)coverage(\/|$)/u;

export function scan(refresh = false): { edges: RawEdge[]; bare: BareUse[]; third: ThirdUse[]; files: number } {
  const cache = resolve(GENERATED, "scan.json");
  if (!refresh && existsSync(cache) && Date.now() - statSync(cache).mtimeMs < 45 * 60_000) return JSON.parse(readFileSync(cache, "utf8"));
  const all = listAll();
  const known = new Set(all);
  const sources = all.filter(f => SRC.test(f) && !DO_NOT_FOLLOW.test(f));
  const edges: RawEdge[] = [], bare: BareUse[] = [], third: ThirdUse[] = [];
  const tryResolve = (fromRel: string, spec: string): string | undefined => {
    const clean = spec.split("?")[0]!;
    const base = posix(relative(ROOT, resolve(ROOT, dirname(fromRel), clean)));
    const stem = base.replace(/\.(js|mjs|cjs)$/u, "");
    for (const c of [base, base + ".ts", base + ".tsx", base + "/🟦️.ts", base + "/🟦️.tsx", stem + ".ts", stem + ".tsx", stem + ".mts"]) if (known.has(c)) return c;
    return undefined;
  };
  for (const file of sources) {
    let text: string;
    try { text = readFileSync(resolve(ROOT, file), "utf8"); } catch { continue; }
    for (const m of text.matchAll(SPEC)) {
      const spec = m[2]!;
      const before = text.slice(Math.max(0, m.index! - 12), m.index!);
      const kind = /require\s*\(|createRequire/u.test(m[0]) ? "require" : /import\s*\(/u.test(m[0]) ? "dynamic" : "static";
      void before;
      if (spec.startsWith(".")) {
        const target = tryResolve(file, spec);
        if (target) edges.push({ file, spec, target, kind });
      } else if (spec.startsWith("@semio-tech/")) {
        const parts = spec.split("?")[0]!.split("/");
        const pkg = parts.slice(0, 2).join("/");
        bare.push({ file, spec, pkg, sub: parts.length > 2 ? "./" + parts.slice(2).join("/") : "." });
      } else if (/^(@[a-z0-9][\w.-]*\/)?[a-z0-9][\w.-]*(\/|$)/u.test(spec) && !spec.includes(":")) {
        const parts = spec.split("?")[0]!.split("/");
        const pkg = spec.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0]!;
        if (!BUILTIN.has(pkg) && pkg !== "bun" && pkg.length > 1) third.push({ file, pkg });
      }
    }
  }
  const out = { edges, bare, third, files: sources.length };
  writeFileSync(cache, JSON.stringify(out));
  return out;
}

if (import.meta.main) {
  const t0 = Date.now();
  const r = scan(process.argv.includes("--refresh"));
  console.log(`files ${r.files} edges ${r.edges.length} bare ${r.bare.length} third ${r.third.length} ${Date.now() - t0}ms`);
}
