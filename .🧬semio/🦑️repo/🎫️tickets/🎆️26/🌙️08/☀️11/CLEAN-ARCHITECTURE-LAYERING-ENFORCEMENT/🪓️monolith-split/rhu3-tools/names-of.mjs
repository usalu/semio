import fs from "node:fs";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const [target, needle] = process.argv.slice(2);
const opts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"] };
const git = args => spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean).map(s => s.normalize("NFC"));
const cands = git(["grep", "-l", "-F", needle, "--", ":(glob)**/*.ts", ":(glob)**/*.tsx", ":!.🧬semio"]);
const T = R + target; const byName = new Map();
for (const f of cands) {
  const abs = R + f; let text; try { text = fs.readFileSync(abs, "utf8"); } catch { continue; }
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const res = s => s.startsWith(".") && ts.resolveModuleName(s, abs, opts, ts.sys).resolvedModule?.resolvedFileName === T;
  const add = (n, file) => { const a = byName.get(n) ?? new Set(); a.add(file); byName.set(n, a); };
  const visit = n => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && res(n.moduleSpecifier.text)) {
      if (ts.isImportDeclaration(n)) { const c = n.importClause; if (!c) add("<side-effect>", f); else { if (c.name) add("default", f); const nb = c.namedBindings; if (nb) { if (ts.isNamespaceImport(nb)) add("*", f); else nb.elements.forEach(e => add((e.propertyName ?? e.name).text, f)); } } }
      else if (n.exportClause && ts.isNamedExports(n.exportClause)) n.exportClause.elements.forEach(e => add((e.propertyName ?? e.name).text, f)); else add("export*", f);
    }
    if (ts.isImportTypeNode(n) && n.argument?.literal && ts.isStringLiteral(n.argument.literal) && res(n.argument.literal.text)) { let q = n.qualifier; while (q && ts.isQualifiedName(q)) q = q.left; add(q?.text ?? "?", f); }
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]) && res(n.arguments[0].text)) add("<dynamic>", f);
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
for (const [n, fs_] of [...byName].sort((a, b) => a[0].localeCompare(b[0]))) console.log(n.padEnd(40), fs_.size, [...fs_].slice(0, 2).map(x => x.slice(-60)).join(" | "));
