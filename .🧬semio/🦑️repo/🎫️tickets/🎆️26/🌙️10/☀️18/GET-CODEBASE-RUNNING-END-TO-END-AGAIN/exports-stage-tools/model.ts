/**
 * 🧭 Package model: sibling lanes per source owner, authored export entries and the deep origin of every exported name.
 */
import ts from "typescript";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { ROOT, loadPackages, posix, type Pkg } from "./lib.ts";

export const model = loadPackages();
export const abs = (p: string): string => resolve(ROOT, p);
export const rel = (p: string): string => posix(relative(ROOT, p));

const siblingsByOwner = new Map<string, Pkg[]>();
for (const p of model.all) (siblingsByOwner.get(p.sourceOwner) ?? siblingsByOwner.set(p.sourceOwner, []).get(p.sourceOwner)!).push(p);
export const siblingsOf = (owner: string): Pkg[] => siblingsByOwner.get(owner) ?? [];
export const lane = (owner: string): Pkg | undefined => {
  const list = siblingsOf(owner).filter(p => p.name);
  return list.find(p => p.dir.endsWith("/📦️packages/🟦️typescript")) ?? list.find(p => !p.dir.endsWith("/📦️packages/🦀️rust") && !p.name.endsWith("-rs")) ?? list[0];
};

export const entryFile = (value: unknown): string | null => {
  if (typeof value === "string") return value;
  if (Array.isArray(value)) { for (const v of value) { const r = entryFile(v); if (r) return r; } return null; }
  if (value && typeof value === "object") {
    const o = value as Record<string, unknown>;
    for (const k of ["semio-source", "import", "default", "types", "node"]) if (k in o) { const r = entryFile(o[k]); if (r) return r; }
    for (const v of Object.values(o)) { const r = entryFile(v); if (r) return r; }
  }
  return null;
};
export type Entry = { pkg: Pkg; sub: string; file: string };
export const entriesOf = (pkg: Pkg): Entry[] => {
  const out: Entry[] = [];
  for (const [sub, value] of Object.entries(pkg.exports)) {
    if (!sub.startsWith(".") || sub.includes("*")) continue;
    const t = entryFile(value);
    if (t && t.startsWith("./")) out.push({ pkg, sub, file: rel(resolve(abs(pkg.dir), t)) });
  }
  return out;
};

const sources = ["", ".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json", "/🟦️.ts", "/🟦️.tsx"];
export const resolveRel = (fromFile: string, spec: string): string | null => {
  if (!spec.startsWith(".")) return null;
  const base = resolve(dirname(abs(fromFile)), spec.split("?")[0]!);
  const alts = [base.replace(/\.js$/u, ".ts"), base.replace(/\.js$/u, ".tsx")];
  for (const c of [...sources.map(s => base + s), ...alts]) { try { if (statSync(c).isFile()) return rel(c); } catch { /* next */ } }
  return null;
};
export const parseFile = (file: string): ts.SourceFile => ts.createSourceFile(file, readFileSync(abs(file), "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : file.endsWith(".js") || file.endsWith(".mjs") || file.endsWith(".cjs") ? ts.ScriptKind.JS : ts.ScriptKind.TS);
const has = (n: ts.Node, k: ts.SyntaxKind): boolean => !!ts.getModifiers(n as ts.HasModifiers)?.some(m => m.kind === k);

export type Origin = { file: string; name: string; typeKind: boolean; via: boolean };
const originCache = new Map<string, Map<string, Origin> | null>();
export function originsOf(file: string, depth = 0): Map<string, Origin> | null {
  if (originCache.has(file)) return originCache.get(file)!;
  originCache.set(file, null);
  if (depth > 8 || !/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(file) || !existsSync(abs(file))) return null;
  let sf: ts.SourceFile;
  try { sf = parseFile(file); } catch { return null; }
  const out = new Map<string, Origin>();
  const self = (name: string, typeKind = false): void => { out.set(name, { file, name, typeKind, via: false }); };
  for (const st of sf.statements) {
    if (ts.isExportDeclaration(st)) {
      const target = st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? st.moduleSpecifier.text : null;
      const resolved = target ? resolveRel(file, target) : null;
      const sub = resolved ? originsOf(resolved, depth + 1) : null;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) {
        for (const e of st.exportClause.elements) {
          if (!target) { self(e.name.text); continue; }
          const orig = (e.propertyName ?? e.name).text;
          const base = sub?.get(orig) ?? { file: resolved ?? "bare:" + target, name: orig, typeKind: false, via: false };
          out.set(e.name.text, { ...base, via: base.via || st.isTypeOnly || e.isTypeOnly });
        }
      } else if (st.exportClause && ts.isNamespaceExport(st.exportClause)) out.set(st.exportClause.name.text, { file: resolved ?? "bare:" + target, name: "*", typeKind: false, via: st.isTypeOnly });
      else if (target) {
        if (sub) { for (const [k, v] of sub) if (k !== "default" && !out.has(k)) out.set(k, { ...v, via: v.via || st.isTypeOnly }); }
        else out.set("*bare:" + target, { file: "bare:" + target, name: "*", typeKind: false, via: false });
      }
    } else if (ts.isExportAssignment(st)) self("default");
    else if (has(st, ts.SyntaxKind.ExportKeyword)) {
      if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) self(d.name.text); else for (const b of d.name.elements) if (ts.isBindingElement(b) && ts.isIdentifier(b.name)) self(b.name.text); }
      else if (ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st)) { if (has(st, ts.SyntaxKind.DefaultKeyword)) self("default"); else if (st.name) self(st.name.text); }
      else if (ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) self(st.name.text, true);
      else if (ts.isEnumDeclaration(st) || ts.isModuleDeclaration(st)) self((st.name as ts.Identifier).text);
    }
  }
  originCache.set(file, out);
  return out;
}

const jsonCache = new Map<string, boolean>();
export function jsonGlueExposes(entry: string, json: string): boolean {
  const key = entry + "\0" + json;
  if (jsonCache.has(key)) return jsonCache.get(key)!;
  let ok = false;
  try {
    if (/\.(ts|tsx|mts)$/u.test(entry)) {
      const sf = parseFile(entry); let reaches = false, exports = false;
      for (const st of sf.statements) {
        if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && resolveRel(entry, st.moduleSpecifier.text) === json) reaches = true;
        if (ts.isExportAssignment(st)) exports = true;
        if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause) && st.exportClause.elements.some(e => e.name.text === "default")) exports = true;
      }
      ok = reaches && exports;
    }
  } catch { ok = false; }
  jsonCache.set(key, ok);
  return ok;
}

export type Use = { names: string[] | "*"; spec: string };
export const EXPOSE_OK = (toOrigins: Map<string, Origin>, eo: Map<string, Origin>, names: string[] | "*"): boolean => {
  const needed = names === "*" ? [...toOrigins.keys()].filter(k => !k.startsWith("*bare:")) : names;
  if (!needed.length) return false;
  return needed.every(n => { const a = toOrigins.get(n), b = eo.get(n); return a && b && a.file === b.file && a.name === b.name && !(b.via && !b.typeKind && !(a.via && !a.typeKind)); });
};

/** Existing entries of the sibling lane(s) of `owner` that already expose `to` for every name of the use. */
export function coveringEntries(owner: string, to: string, names: string[] | "*"): Entry[] {
  const out: Entry[] = [];
  const toOrigins = to.endsWith(".json") ? null : originsOf(to);
  for (const p of siblingsOf(owner)) for (const en of entriesOf(p)) {
    if (en.file === to) { out.push(en); continue; }
    if (to.endsWith(".json")) { if ((names === "*" || (names.length === 1 && names[0] === "default")) && jsonGlueExposes(en.file, to)) out.push(en); continue; }
    if (!toOrigins) continue;
    const eo = originsOf(en.file);
    if (eo && EXPOSE_OK(toOrigins, eo, names)) out.push(en);
  }
  return out;
}
