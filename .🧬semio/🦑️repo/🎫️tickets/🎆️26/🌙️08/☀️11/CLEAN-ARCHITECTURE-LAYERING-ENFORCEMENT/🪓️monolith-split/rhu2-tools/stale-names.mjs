import fs from "node:fs";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const opts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"] };
const git = args => spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean).map(s => s.normalize("NFC"));
const targets = process.argv.slice(2);
const program = ts.createProgram(targets.map(t => R + t), { ...opts, noEmit: true, skipLibCheck: true, types: [] });
const checker = program.getTypeChecker();
for (const t of targets) {
  const sf = program.getSourceFile(R + t); const sym = sf && checker.getSymbolAtLocation(sf);
  const exp = new Set(sym ? checker.getExportsOfModule(sym).map(x => x.name) : []);
  const folder = t.split("/").slice(-2, -1)[0];
  const needle = folder + "/🟦️";
  const cands = git(["grep", "-l", "-F", needle, "--", ":(glob)**/*.ts", ":(glob)**/*.tsx", ":!.🧬semio"]);
  const T = R + t; let bad = 0;
  for (const f of cands) {
    const abs = R + f; let text; try { text = fs.readFileSync(abs, "utf8"); } catch { continue; }
    const s2 = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
    const res = s => s.startsWith(".") && ts.resolveModuleName(s, abs, opts, ts.sys).resolvedModule?.resolvedFileName === T;
    const check = (n, file) => { if (n !== "default" && n !== "*" && !exp.has(n)) { bad++; console.log("STALE", t.slice(-60), "|", f.slice(-70), "|", n); } };
    const visit = n => {
      if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && res(n.moduleSpecifier.text)) {
        if (ts.isImportDeclaration(n)) { const nb = n.importClause?.namedBindings; if (nb && ts.isNamedImports(nb)) nb.elements.forEach(e => check((e.propertyName ?? e.name).text)); }
        else if (n.exportClause && ts.isNamedExports(n.exportClause)) n.exportClause.elements.forEach(e => check((e.propertyName ?? e.name).text));
      }
      if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]) && res(n.arguments[0].text)) {
        const up = n.parent; if (up && ts.isAwaitExpression(up) && up.parent && ts.isVariableDeclaration(up.parent) && ts.isObjectBindingPattern(up.parent.name)) up.parent.name.elements.forEach(e => check((e.propertyName ?? e.name).getText()));
        else console.log("DYN?", f.slice(-70));
      }
      ts.forEachChild(n, visit);
    };
    visit(s2);
  }
  console.log("checked", t.slice(-50), "stale", bad);
}
