import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const [specPath, mode] = process.argv.slice(2);
const write = mode === "--write";
const spec = JSON.parse(fs.readFileSync(specPath, "utf8"));
const from = spec.from.normalize("NFC");
const nameTo = new Map(Object.entries(spec.names).map(([k, v]) => [k, v.normalize("NFC")]));
const defaultTo = spec.default ? spec.default.normalize("NFC") : null;
const opts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"] };
const git = args => spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean).map(s => s.normalize("NFC"));
const needle = spec.bare ?? from.split("/").slice(-2).join("/");
const globs = [":(glob)🧰️framework/**/*.ts", ":(glob)🧰️framework/**/*.tsx", ":(glob)✏️s/**/*.ts", ":(glob)✏️s/**/*.tsx", ":(glob)🌎️hub/**/*.ts", ":(glob)🌎️hub/**/*.tsx", ":(glob)♻️mit-bestand/**/*.ts", ":(glob)♻️mit-bestand/**/*.tsx", ":(glob)🏢️semio-tech/**/*.ts", ":(glob)🏢️semio-tech/**/*.tsx", ":(glob).storybook/**/*.ts", ":(glob).storybook/**/*.tsx"];
const cands = new Set(git(["grep", "-l", "-F", "-e", needle, "--", ...globs]));
for (const f of git(["ls-files", "--others", "--exclude-standard", "--", ":(glob)**/*.ts", ":(glob)**/*.tsx"])) { if (f.startsWith(".🧬semio/")) continue; try { if (fs.readFileSync(R + f, "utf8").includes(needle)) cands.add(f); } catch {} }
const fromAbs = R + from;
let changed = 0, manual = [];
const rel = (importerRel, targetRel, explicit) => { let to = explicit ? targetRel : path.dirname(targetRel); let r = path.relative(path.dirname(R + importerRel), R + to).normalize("NFC"); if (!r.startsWith(".")) r = "./" + r; return r; };
for (const file of [...cands].sort()) {
  const abs = R + file; let text; try { text = fs.readFileSync(abs, "utf8"); } catch { continue; }
  if (file === from) continue;
  if (spec.scope && !spec.scope.some(p => file.startsWith(p.normalize("NFC")))) continue;
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  const resolves = s => { if (!s.startsWith(".") && !spec.bare) return false; if (!s.startsWith(".") && !s.startsWith(spec.bare)) return false; const r = ts.resolveModuleName(s, abs, opts, ts.sys).resolvedModule?.resolvedFileName; return r === fromAbs; };
  const litEdit = (lit, dest) => { const q = text[lit.getStart()]; const explicit = spec.bare ? true : /\.(ts|tsx)$/.test(lit.text); edits.push({ start: lit.getStart() + 1, end: lit.end - 1, text: rel(file, dest, explicit) }); };
  const leftmost = q => { let c = q; while (c && ts.isQualifiedName(c)) c = c.left; return c && ts.isIdentifier(c) ? c.text : null; };
  const visit = n => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]) && resolves(n.arguments[0].text)) {
      const lit = n.arguments[0]; let names = null;
      let up = n.parent; if (up && ts.isAwaitExpression(up)) { const aw = up; const par = aw.parent; if (par && ts.isVariableDeclaration(par) && ts.isObjectBindingPattern(par.name)) names = par.name.elements.map(e => (e.propertyName ?? e.name).getText()); else if (par && ts.isParenthesizedExpression(par) && par.parent && ts.isPropertyAccessExpression(par.parent)) names = [par.parent.name.text]; }
      else if (up && ts.isPropertyAccessExpression(up)) names = [up.name.text];
      const dests = names ? new Set(names.map(x => nameTo.get(x) ?? defaultTo ?? "__keep__")) : null;
      if (dests && dests.size === 1 && ![...dests][0].startsWith("__")) litEdit(lit, [...dests][0]);
      else manual.push(`${file}: dynamic import("${lit.text.slice(-40)}") names=${names ? names.join(",") : "?"}`);
    }
    if (ts.isImportTypeNode(n) && n.argument && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && resolves(n.argument.literal.text)) {
      const name = leftmost(n.qualifier); const dest = name ? (nameTo.get(name) ?? defaultTo) : null;
      if (dest) litEdit(n.argument.literal, dest); else manual.push(`${file}: import type("${n.argument.literal.text.slice(-40)}") name=${name}`);
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  for (const st of sf.statements) {
    const isImp = ts.isImportDeclaration(st), isExp = ts.isExportDeclaration(st) && st.moduleSpecifier;
    if (!(isImp || isExp)) continue;
    const s = st.moduleSpecifier.text; if (!resolves(s)) continue;
    const explicit = spec.bare ? true : /\.(ts|tsx)$/.test(s);
    let items = [], declType = false;
    if (isImp) { const c = st.importClause; if (!c) { manual.push(`${file}: side-effect import`); continue; } if (c.name || (c.namedBindings && ts.isNamespaceImport(c.namedBindings))) { manual.push(`${file}: default/namespace import of ${s.slice(-40)}`); continue; } declType = !!c.isTypeOnly; items = c.namedBindings.elements.map(e => ({ local: e.name.text, imported: (e.propertyName ?? e.name).text, type: !!e.isTypeOnly })); }
    else { if (!st.exportClause || !ts.isNamedExports(st.exportClause)) { manual.push(`${file}: export * of ${s.slice(-40)}`); continue; } declType = !!st.isTypeOnly; items = st.exportClause.elements.map(e => ({ local: e.name.text, imported: (e.propertyName ?? e.name).text, type: !!e.isTypeOnly })); }
    const groups = new Map();
    let any = false;
    for (const it of items) { const dest = nameTo.get(it.imported) ?? defaultTo; if (dest) any = true; const key = dest ?? "__keep__"; const g = groups.get(key) ?? []; g.push(it); groups.set(key, g); }
    if (!any) continue;
    const multiline = text.slice(st.getStart(), st.end).includes("\n");
    const im = multiline ? /\n([ \t]+)\S/.exec(text.slice(st.getStart(), st.end)) : null; const indent = im ? im[1] : "  ";
    const out = [];
    for (const [dest, its] of groups) {
      const parts = its.map(i => (i.type && !declType ? "type " : "") + (i.imported === i.local ? i.imported : `${i.imported} as ${i.local}`));
      const head = isImp ? `import ${declType ? "type " : ""}` : `export ${declType ? "type " : ""}`;
      const body = multiline ? `{\n${parts.map(p => indent + p).join(",\n")}\n}` : `{ ${parts.join(", ")} }`;
      const target = dest === "__keep__" ? s : rel(file, dest, explicit);
      out.push(`${head}${body} from "${target}";`);
    }
    edits.push({ start: st.getStart(), end: st.end, text: out.join("\n") });
  }
  if (!edits.length) continue;
  let next = text; for (const e of edits.sort((a, b) => b.start - a.start)) next = next.slice(0, e.start) + e.text + next.slice(e.end);
  changed++;
  console.log(`${write ? "WRITE" : "DRY"} ${file.replace(/^.*?engine\//u, "E/")} (${edits.length})`);
  if (write) { const cur = fs.readFileSync(abs, "utf8"); if (cur !== text) { console.log("  CHANGED UNDER US, skipped"); continue; } fs.writeFileSync(abs, next); }
}
console.log(JSON.stringify({ files: changed, manual: manual.length }));
for (const m of manual) console.log("MANUAL", m);
