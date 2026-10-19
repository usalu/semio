import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, basename } from "node:path";

type Spec = { parent: string; leaf: string; names: string[]; doc: string; leafFile?: string; base?: string; closure?: boolean; typeImports?: boolean; allowExportStar?: boolean; exportStar?: "append" };
const root = "/Users/ueli/Documents/semio";
const specs: Spec[] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const dry = process.argv.includes("--dry");
const skip = new Set(["node_modules", "target", "pkg", "dist", ".git", "🗑️generated", ".cache"]);
const exts = [".ts", ".tsx", ".mts", ".cts"];
const scanRoots = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching"];
const all: string[] = [];
const pkgFiles: string[] = [];
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name === "package.json") pkgFiles.push(p);
    else if (/\.(ts|tsx|mts|cts)$/u.test(e.name) && !e.name.endsWith(".d.ts")) all.push(p);
  }
};
for (const r of scanRoots) walk(join(root, r));
const fileSet = new Set(all);
const resolveSpec = (from: string, spec: string): string | undefined => {
  if (!spec.startsWith(".")) return undefined;
  const base = resolve(dirname(from), spec);
  for (const c of [base, ...exts.map((e) => base + e), ...exts.map((e) => join(base, "index" + e))]) if (fileSet.has(c)) return c;
  return undefined;
};
const pkgs = new Map<string, { dir: string; exports: unknown }>();
for (const f of pkgFiles) {
  try { const j = JSON.parse(readFileSync(f, "utf8")); if (typeof j.name === "string" && j.exports) pkgs.set(j.name, { dir: dirname(f), exports: j.exports }); } catch {}
}
const pickTarget = (v: unknown): string | undefined => {
  if (typeof v === "string") return v;
  if (v && typeof v === "object") for (const k of ["bun", "import", "default", "types", "node"]) { const r = pickTarget((v as Record<string, unknown>)[k]); if (r) return r; }
  return undefined;
};
const resolveBare = (spec: string): string | undefined => {
  if (spec.startsWith(".") || spec.startsWith("node:") || spec.startsWith("bun:")) return undefined;
  const parts = spec.split("/");
  const name = spec.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0]!;
  const pkg = pkgs.get(name);
  if (!pkg) return undefined;
  const sub = "." + spec.slice(name.length);
  const ex = pkg.exports as Record<string, unknown>;
  const isMap = ex && typeof ex === "object" && Object.keys(ex).some((k) => k.startsWith("."));
  const table: Record<string, unknown> = isMap ? ex : { ".": ex };
  let t = pickTarget(table[sub]);
  if (!t) for (const [k, v] of Object.entries(table)) {
    const i = k.indexOf("*");
    if (i < 0) continue;
    if (sub.startsWith(k.slice(0, i)) && sub.endsWith(k.slice(i + 1))) { const mid = sub.slice(i, sub.length - (k.length - i - 1)); const tv = pickTarget(v); if (tv) { t = tv.replace("*", mid); break; } }
  }
  if (!t) return undefined;
  const abs = resolve(pkg.dir, t);
  return fileSet.has(abs) ? abs : undefined;
};
const kindOf = (f: string) => (f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const parse = (f: string, text: string) => ts.createSourceFile(f, text, ts.ScriptTarget.Latest, true, kindOf(f));
const relSpec = (fromDir: string, toFile: string): string => {
  let r = relative(fromDir, toFile);
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const declNames = (s: ts.Statement): string[] => {
  if (ts.isExportDeclaration(s) && s.moduleSpecifier && s.exportClause && ts.isNamedExports(s.exportClause)) return s.exportClause.elements.map((e) => e.name.text);
  if (ts.isVariableStatement(s)) return s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : []));
  if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isEnumDeclaration(s)) && s.name) return [s.name.text];
  return [];
};
const isTypeDecl = (s: ts.Statement) => ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || (ts.isExportDeclaration(s) && s.isTypeOnly);
const idents = (node: ts.Node, out = new Set<string>()): Set<string> => {
  const visit = (n: ts.Node) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skipIt =
        (ts.isPropertyAccessExpression(p) && p.name === n) ||
        (ts.isPropertyAssignment(p) && p.name === n) ||
        (ts.isPropertySignature(p) && p.name === n) ||
        (ts.isMethodDeclaration(p) && p.name === n) ||
        (ts.isMethodSignature(p) && p.name === n) ||
        (ts.isPropertyDeclaration(p) && p.name === n) ||
        (ts.isQualifiedName(p) && p.right === n) ||
        (ts.isMetaProperty(p)) ||
        (ts.isImportSpecifier(p) || ts.isExportSpecifier(p));
      if (!skipIt) out.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return out;
};
const startWithComments = (text: string, s: ts.Statement): number => {
  const ranges = ts.getLeadingCommentRanges(text, s.getFullStart()) ?? [];
  let start = s.getStart();
  for (let i = ranges.length - 1; i >= 0; i--) {
    const r = ranges[i]!;
    const body = text.slice(r.pos, r.end);
    if (/^\/\/\s*#(end)?region/u.test(body)) break;
    const gap = text.slice(r.end, start);
    if ((gap.match(/\n/gu) ?? []).length > 1) break;
    start = r.pos;
  }
  return start;
};

type Imp = { decl: ts.ImportDeclaration; names: { local: string; text: string; typeOnly: boolean; kind: "named" | "default" | "ns"; imported?: string }[]; spec: string; typeOnlyDecl: boolean; attrs: string };
const importsOf = (sf: ts.SourceFile): Imp[] => {
  const out: Imp[] = [];
  for (const s of sf.statements) {
    if (!ts.isImportDeclaration(s) || !ts.isStringLiteral(s.moduleSpecifier)) continue;
    const c = s.importClause;
    const names: Imp["names"] = [];
    if (c) {
      if (c.name) names.push({ local: c.name.text, text: c.name.text, typeOnly: c.isTypeOnly, kind: "default" });
      if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) names.push({ local: c.namedBindings.name.text, text: `* as ${c.namedBindings.name.text}`, typeOnly: c.isTypeOnly, kind: "ns" });
      if (c.namedBindings && ts.isNamedImports(c.namedBindings))
        for (const e of c.namedBindings.elements) names.push({ local: e.name.text, text: e.getText(), typeOnly: c.isTypeOnly || e.isTypeOnly, kind: "named", imported: (e.propertyName ?? e.name).text });
    }
    out.push({ decl: s, names, spec: s.moduleSpecifier.text, typeOnlyDecl: !!c?.isTypeOnly, attrs: s.attributes ? s.attributes.getText() : "" });
  }
  return out;
};
const fmtImport = (spec: string, items: { text: string; typeOnly: boolean; kind: string }[], attrs = ""): string => {
  const tail = attrs ? ` ${attrs}` : "";
  const types = items.filter((i) => i.typeOnly), vals = items.filter((i) => !i.typeOnly);
  const strip = (t: string) => t.replace(/^type\s+/u, "");
  const lines: string[] = [];
  const def = vals.filter((i) => i.kind === "default"), ns = vals.filter((i) => i.kind === "ns"), nm = vals.filter((i) => i.kind === "named");
  if (def.length || nm.length || ns.length) {
    const parts: string[] = [];
    if (def.length) parts.push(def[0]!.text);
    if (ns.length) parts.push(ns[0]!.text);
    if (nm.length) parts.push(`{ ${nm.map((i) => i.text).join(", ")} }`);
    lines.push(`import ${parts.join(", ")} from "${spec}"${tail};`);
  }
  if (types.length) {
    const tdef = types.filter((i) => i.kind === "default"), tns = types.filter((i) => i.kind === "ns"), tnm = types.filter((i) => i.kind === "named");
    const parts: string[] = [];
    if (tdef.length) parts.push(strip(tdef[0]!.text));
    if (tns.length) parts.push(tns[0]!.text);
    if (tnm.length) parts.push(`{ ${tnm.map((i) => strip(i.text)).join(", ")} }`);
    lines.push(`import type ${parts.join(", ")} from "${spec}"${tail};`);
  }
  return lines.join("\n");
};

const reverse = new Map<string, Set<string>>();
{
  const re = /(?:from\s*|import\s*\(\s*|import\s+)["']([^"']+)["']/gu;
  for (const f of all) {
    let src: string; try { src = readFileSync(f, "utf8"); } catch { continue; }
    for (const m of src.matchAll(re)) {
      const sp = m[1]!;
      const t = sp.startsWith(".") ? resolveSpec(f, sp) : resolveBare(sp);
      if (t && t !== f) { let set = reverse.get(t); if (!set) { set = new Set(); reverse.set(t, set); } set.add(f); }
    }
  }
}
const edits = new Map<string, string>();
const read = (f: string) => edits.get(f) ?? readFileSync(f, "utf8");
const problems: string[] = [];
const rows: string[] = [];

for (const spec of specs) {
  const parent = spec.parent;
  const text = read(parent);
  const sf = parse(parent, text);
  const wanted = new Set(spec.names);
  let moved: ts.Statement[] = [];
  let keep: ts.Statement[] = [];
  const typeNames = new Set<string>();
  const seen = new Set<string>();
  const partition = () => {
    moved = []; keep = []; typeNames.clear(); seen.clear();
    for (const s of sf.statements) {
      const ns = declNames(s);
      if (ns.some((n) => wanted.has(n))) {
        if (!spec.closure && !ns.every((n) => wanted.has(n))) problems.push(`${basename(parent)}: statement declares mixed names ${ns.join(",")}`);
        for (const n of ns) wanted.add(n);
        moved.push(s);
        for (const n of ns) { seen.add(n); if (isTypeDecl(s)) typeNames.add(n); }
      } else keep.push(s);
    }
  };
  partition();
  if (spec.closure) {
    for (;;) {
      const mi = new Set<string>();
      for (const s of moved) idents(s, mi);
      const topKeep = new Map<string, ts.Statement>();
      for (const s of keep) for (const n of declNames(s)) topKeep.set(n, s);
      const add = [...mi].filter((n) => topKeep.has(n) && !wanted.has(n));
      if (!add.length) break;
      for (const n of add) wanted.add(n);
      partition();
    }
    rows.push(`   closure moved ${moved.length} statements, ${moved.reduce((a, s) => a + s.getText().split("\n").length, 0)} lines: ${[...seen].join(", ").slice(0, 300)}`);
  }
  const dropped: ts.Statement[] = [];
  for (const s of [...keep]) {
    if (ts.isExportDeclaration(s) && !s.moduleSpecifier && s.exportClause && ts.isNamedExports(s.exportClause)) {
      const els = s.exportClause.elements.map((e) => (e.propertyName ?? e.name).text);
      if (els.every((n) => wanted.has(n))) { dropped.push(s); keep.splice(keep.indexOf(s), 1); }
      else if (els.some((n) => wanted.has(n))) problems.push(`${basename(parent)}: local export list mixes moved and kept names`);
    }
  }
  for (const n of wanted) if (!seen.has(n)) problems.push(`${parent}: name ${n} not found as top-level declaration`);
  const movedNames = [...seen];
  const movedIdents = new Set<string>();
  for (const s of moved) idents(s, movedIdents);
  const keepIdents = new Set<string>();
  for (const s of keep) if (!ts.isImportDeclaration(s)) idents(s, keepIdents);
  const topNames = new Map<string, ts.Statement>();
  for (const s of keep) for (const n of declNames(s)) topNames.set(n, s);
  const importLocals = new Set(importsOf(sf).flatMap((i) => i.names.map((n) => n.local)));
  const unresolved = [...movedIdents].filter((n) => topNames.has(n) && !importLocals.has(n));
  if (unresolved.length) problems.push(`${basename(dirname(parent))}: moved code needs non-moved parent decls: ${unresolved.join(", ")}`);
  const imps = importsOf(sf);
  const leafDir = resolve(dirname(parent), spec.base ?? ".", spec.leaf);
  const leafFile = join(leafDir, spec.leafFile ?? "🟦️.ts");
  if (existsSync(leafFile)) problems.push(`leaf exists: ${leafFile}`);
  const leafImports = new Map<string, { text: string; typeOnly: boolean; kind: string }[]>();
  const leafAttrs = new Map<string, string>();
  for (const imp of imps) {
    const used = imp.names.filter((n) => movedIdents.has(n.local));
    if (!used.length) continue;
    let spec2 = imp.spec;
    if (spec2.startsWith(".")) {
      const target = resolveSpec(parent, spec2);
      const abs = target ?? resolve(dirname(parent), spec2);
      spec2 = relSpec(leafDir, abs);
    }
    const arr = leafImports.get(spec2) ?? [];
    for (const u of used) arr.push({ text: u.text, typeOnly: spec.typeImports ? true : u.typeOnly, kind: u.kind });
    leafImports.set(spec2, arr);
    if (imp.attrs) leafAttrs.set(spec2, imp.attrs);
  }
  const chunks: string[] = [];
  for (const s of moved) {
    const st = startWithComments(text, s);
    let body = text.slice(st, s.getEnd());
    const hasExport = ts.isExportDeclaration(s) || (ts.canHaveModifiers(s) && ts.getModifiers(s)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword));
    if (!hasExport) {
      const off = s.getStart() - st;
      body = body.slice(0, off) + "export " + body.slice(off);
    }
    chunks.push(body);
  }
  const header = `/** ${spec.doc} */`;
  const impText = [...leafImports.entries()].map(([sp, items]) => fmtImport(sp, items, leafAttrs.get(sp))).join("\n");
  const leafText = [header, impText, chunks.join("\n\n")].filter(Boolean).join("\n\n") + "\n";
  const removals = [...moved, ...dropped].map((s) => {
    const st = startWithComments(text, s);
    let en = s.getEnd();
    while (text[en] === "\n" && text.slice(en, en + 2) !== "\n\n" ) en++;
    if (text[en] === "\n") en++;
    while (text[en] === "\n") en++;
    return [st, en] as const;
  });
  let newText = text;
  for (const [st, en] of [...removals].sort((a, b) => b[0] - a[0])) newText = newText.slice(0, st) + newText.slice(en);
  const parentNeeds = movedNames.filter((n) => keepIdents.has(n));
  const lastImport = [...sf.statements].filter(ts.isImportDeclaration).pop();
  const leafSpecFromParent = relSpec(dirname(parent), leafFile);
  const importLine = parentNeeds.length ? fmtImport(leafSpecFromParent, parentNeeds.map((n) => ({ text: typeNames.has(n) ? `type ${n}` : n, typeOnly: typeNames.has(n), kind: "named" }))) : "";
  const reparse = parse(parent, newText);
  const lastImport2 = [...reparse.statements].filter(ts.isImportDeclaration).pop();
  if (importLine) {
    if (lastImport2) newText = newText.slice(0, lastImport2.getEnd()) + "\n" + importLine + newText.slice(lastImport2.getEnd());
    else {
      const hm = /^\/\/ ?#endregion [^\n]*Header[^\n]*\n/mu.exec(newText);
      const at0 = hm && hm.index < 2000 ? hm.index + hm[0].length : 0;
      newText = newText.slice(0, at0) + (at0 ? "\n" : "") + importLine + "\n" + (at0 ? "" : "\n") + newText.slice(at0);
      rows.push(`   NOTE ${basename(parent)}: no import existed; inserted after the header region`);
    }
  }
  const sf3 = parse(parent, newText);
  const keepIdents2 = new Set<string>();
  for (const s of sf3.statements) if (!ts.isImportDeclaration(s)) idents(s, keepIdents2);
  const removeSpecs: [number, number, string][] = [];
  for (const imp of importsOf(sf3)) {
    const dead = imp.names.filter((n) => movedIdents.has(n.local) && !keepIdents2.has(n.local));
    if (!dead.length) continue;
    const live = imp.names.filter((n) => !dead.includes(n));
    const replacement = live.length ? fmtImport(imp.spec, live, imp.attrs) : "";
    let en = imp.decl.getEnd();
    if (!replacement && newText[en] === "\n") en++;
    removeSpecs.push([imp.decl.getStart(), en, replacement]);
  }
  for (const [st, en, rep] of removeSpecs.sort((a, b) => b[0] - a[0])) newText = newText.slice(0, st) + rep + newText.slice(en);
  edits.set(parent, newText);
  edits.set(leafFile, leafText);

  const importerCounts: string[] = [];
  const bareHits = new Set<string>();
  for (const f of [...(reverse.get(parent) ?? [])]) {
    if (f === parent) continue;
    let src = read(f);
    const fsf = parse(f, src);
    const rewrites: [number, number, string][] = [];
    for (const imp of importsOf(fsf)) {
      if (!imp.spec.startsWith(".") && resolveBare(imp.spec) === parent) {
        for (const n of imp.names) if (n.kind === "named" && wanted.has(n.imported!)) bareHits.add(`${n.imported}${n.typeOnly ? ":type" : ""}`);
        if (imp.names.some((n) => n.kind === "ns")) problems.push(`${f}: namespace import of parent through package entry`);
        continue;
      }
      if (!imp.spec.startsWith(".") || resolveSpec(f, imp.spec) !== parent) continue;
      const hit = imp.names.filter((n) => n.kind === "named" && wanted.has(n.imported!));
      if (imp.names.some((n) => n.kind === "ns")) {
        const nsn = imp.names.find((n) => n.kind === "ns")!.local;
        if (new RegExp(`\\b${nsn}\\.(${movedNames.join("|")})\\b`, "u").test(src)) problems.push(`${f}: namespace use of moved name`);
      }
      if (!hit.length) continue;
      const rest = imp.names.filter((n) => !hit.includes(n));
      const parts: string[] = [];
      if (rest.length) parts.push(fmtImport(imp.spec, rest));
      parts.push(fmtImport(relSpec(dirname(f), leafFile), hit.map((h) => ({ text: h.text, typeOnly: h.typeOnly, kind: "named" }))));
      rewrites.push([imp.decl.getStart(), imp.decl.getEnd(), parts.join("\n")]);
    }
    for (const s of fsf.statements) {
      if (ts.isExportDeclaration(s) && s.moduleSpecifier && ts.isStringLiteral(s.moduleSpecifier) && s.moduleSpecifier.text.startsWith(".") && resolveSpec(f, s.moduleSpecifier.text) === parent) {
        if (s.exportClause && ts.isNamedExports(s.exportClause) && s.exportClause.elements.some((e) => wanted.has((e.propertyName ?? e.name).text))) {
          const mv = s.exportClause.elements.filter((e) => wanted.has((e.propertyName ?? e.name).text));
          const rest = s.exportClause.elements.filter((e) => !mv.includes(e));
          const kw = s.isTypeOnly ? "export type" : "export";
          const parts: string[] = [];
          if (rest.length) parts.push(`${kw} { ${rest.map((e) => e.getText()).join(", ")} } from "${(s.moduleSpecifier as ts.StringLiteral).text}";`);
          parts.push(`${kw} { ${mv.map((e) => e.getText()).join(", ")} } from "${relSpec(dirname(f), leafFile)}";`);
          rewrites.push([s.getStart(), s.getEnd(), parts.join("\n")]);
        }
        if (!s.exportClause && spec.exportStar === "append") rewrites.push([s.getStart(), s.getEnd(), `${s.getText()}\nexport * from "${relSpec(dirname(f), leafFile)}";`]);
        else if (!s.exportClause && !spec.allowExportStar) problems.push(`${f}: export * from parent`);
      }
    }
    const dynRe = /import\(\s*["']([^"']+)["']\s*\)/gu;
    for (const m of src.matchAll(dynRe)) if ((m[1]!.startsWith(".") ? resolveSpec(f, m[1]!) : resolveBare(m[1]!)) === parent) {
      const around = src.slice(Math.max(0, m.index! - 200), m.index! + m[0].length + 100);
      if (movedNames.some((n) => new RegExp(`\\b${n}\\b`, "u").test(around))) problems.push(`${f}: dynamic import of parent near moved name (check manually)`);
    }
    if (rewrites.length) {
      for (const [st, en, rep] of rewrites.sort((a, b) => b[0] - a[0])) src = src.slice(0, st) + rep + src.slice(en);
      edits.set(f, src);
      importerCounts.push(f);
    }
  }
  if (bareHits.size) {
    const vals = [...bareHits].filter((h) => !h.endsWith(":type")).map((h) => h);
    const tys = [...bareHits].filter((h) => h.endsWith(":type")).map((h) => h.slice(0, -5)).filter((h) => !vals.includes(h));
    const lines: string[] = [];
    if (vals.length) lines.push(`export { ${vals.join(", ")} } from "${leafSpecFromParent}";`);
    if (tys.length) lines.push(`export type { ${tys.join(", ")} } from "${leafSpecFromParent}";`);
    const cur = edits.get(parent)!;
    const sfp = parse(parent, cur);
    const li = [...sfp.statements].filter(ts.isImportDeclaration).pop();
    const at = li ? li.getEnd() : 0;
    edits.set(parent, cur.slice(0, at) + "\n" + lines.join("\n") + cur.slice(at));
    rows.push(`   (package-entry consumers keep these on the parent: ${[...bareHits].join(", ")} -> record package exports follow-up)`);
  }
  rows.push(`${relative(root, parent).replace(/️/gu, "")}\n   -> ${spec.leaf} [${movedNames.join(", ")}] importers rewritten: ${importerCounts.length}`);
}
console.log(rows.join("\n"));
if (problems.length) { console.log("PROBLEMS:\n" + problems.join("\n")); }
if (!dry && problems.length === 0) {
  for (const [f, t] of edits) { mkdirSync(dirname(f), { recursive: true }); writeFileSync(f, t); }
  console.log(`wrote ${edits.size} files`);
} else if (!dry) console.log("not written (problems)");
else console.log(`dry run, would write ${edits.size} files`);
