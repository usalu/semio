import { existsSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { analyze, ROOT, ts, type FileAnalysis } from "./ast.ts";

export type ExportOrigin = { file: string; name: string; typeOnly: boolean };

export function resolveRel(from: string, spec: string): string | null {
  if (!spec.startsWith(".")) return null;
  const base = normalize(join(dirname(from), spec));
  for (const c of [base, base.replace(/\.js$/u, ".ts"), base + ".ts"]) if (existsSync(join(ROOT, c))) return c;
  return base;
}

const cache = new Map<string, Map<string, ExportOrigin>>();

export function exportMap(file: string, a?: FileAnalysis): Map<string, ExportOrigin> {
  if (!a && cache.has(file)) return cache.get(file)!;
  const an = a ?? analyze(file);
  const out = new Map<string, ExportOrigin>();
  const importBy = new Map<string, { file: string | null; name: string; typeOnly: boolean }>();
  for (const imp of an.imports) for (const b of imp.bindings) importBy.set(b.local, { file: resolveRel(file, imp.spec), name: b.imported, typeOnly: b.typeOnly });
  for (const d of an.decls) if (d.exported) for (const n of d.names) out.set(n, { file, name: n, typeOnly: d.typeOnly });
  for (const e of an.exportsFrom) {
    if (e.spec === null) {
      const sf = an.sf;
      const st = sf.statements[e.index] as import("typescript").ExportDeclaration;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) {
        for (const el of st.exportClause.elements) {
          const local = (el.propertyName ?? el.name).text;
          const imp = importBy.get(local);
          const typeOnly = e.typeOnly || el.isTypeOnly;
          if (imp && imp.file) out.set(el.name.text, { file: imp.file, name: imp.name, typeOnly: typeOnly || imp.typeOnly });
          else out.set(el.name.text, { file, name: local, typeOnly });
        }
      }
      continue;
    }
    const target = resolveRel(file, e.spec);
    if (!target) continue;
    const st = an.sf.statements[e.index] as import("typescript").ExportDeclaration;
    if (e.star) {
      const sub = exportMap(target);
      for (const [n, o] of sub) if (!out.has(n)) out.set(n, o);
    } else if (st.exportClause && ts.isNamedExports(st.exportClause)) {
      const sub = exportMap(target);
      for (const el of st.exportClause.elements) {
        const orig = (el.propertyName ?? el.name).text;
        const o = sub.get(orig) ?? { file: target, name: orig, typeOnly: false };
        out.set(el.name.text, { file: o.file, name: o.name, typeOnly: o.typeOnly || e.typeOnly || el.isTypeOnly });
      }
    } else if (st.exportClause && ts.isNamespaceExport(st.exportClause)) {
      out.set(st.exportClause.name.text, { file: target, name: "*", typeOnly: false });
    }
  }
  // default export
  if (!a) cache.set(file, out);
  return out;
}

import { readdirSync, statSync } from "node:fs";
const definerCache = new Map<string, Map<string, string[]>>();

export function definerIndex(dir: string): Map<string, string[]> {
  const hit = definerCache.get(dir);
  if (hit) return hit;
  const index = new Map<string, string[]>();
  const walk = (d: string) => {
    let names: string[];
    try {
      names = readdirSync(join(ROOT, d));
    } catch {
      return;
    }
    for (const n of names) {
      if (n === "node_modules" || n.startsWith(".")) continue;
      const rel = join(d, n);
      let st;
      try {
        st = statSync(join(ROOT, rel));
      } catch {
        continue;
      }
      if (st.isDirectory()) walk(rel);
      else if (/\.(ts|tsx|mts)$/u.test(n) && !n.endsWith(".d.ts")) {
        let m: Map<string, ExportOrigin>;
        try {
          m = exportMap(rel);
        } catch {
          continue;
        }
        for (const [name, o] of m) if (o.file === rel) (index.get(name) ?? index.set(name, []).get(name)!).push(rel);
      }
    }
  };
  walk(dir);
  definerCache.set(dir, index);
  return index;
}
