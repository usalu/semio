// 🔎 c5d TS importer census: every static import, export-from and dynamic import of a module specifier that resolves to one of the given target files, with the bound names and the exact source range.
// usage: bun tsimporters.ts <target-abs-path>...   (files from `git ls-files` plus untracked; the ticket tree is skipped)
import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const REPO = "/Users/ueli/Documents/semio";
const targets = new Set(process.argv.slice(2).map(p => resolve(p)));
const list = (args: string[]) => execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", ...args], {cwd: REPO, encoding: "utf8", maxBuffer: 1 << 28}).split("\n").filter(Boolean);
const files = [...new Set([...list(["ls-files", "--", "*.ts", "*.tsx", "*.mts", "*.js", "*.mjs"]), ...list(["ls-files", "-o", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mts", "*.js", "*.mjs"])])].filter(f => !f.startsWith(".🧬semio/") && !f.includes("node_modules/"));
let count = 0;
for (const rel of files) {
  const abs = resolve(REPO, rel);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  if (!ts.preProcessFile(text, true, true).importedFiles.some(f => f.fileName.startsWith(".") && targets.has(resolve(dirname(abs), f.fileName)))) continue;
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true);
  const hit = (spec: string) => targets.has(resolve(dirname(abs), spec));
  const line = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && hit(node.moduleSpecifier.text)) {
      count++;
      console.log(`${rel}:${line(node.getStart())} import ${node.getText().replace(/\s+/g, " ").slice(0, 400)}`);
    } else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && hit(node.moduleSpecifier.text)) {
      count++;
      console.log(`${rel}:${line(node.getStart())} export ${node.getText().replace(/\s+/g, " ").slice(0, 400)}`);
    } else if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]!) && hit((node.arguments[0] as ts.StringLiteral).text)) {
      count++;
      let stmt: ts.Node = node;
      while (stmt.parent && !ts.isVariableStatement(stmt) && !ts.isExpressionStatement(stmt) && !ts.isReturnStatement(stmt)) stmt = stmt.parent;
      console.log(`${rel}:${line(node.getStart())} dynamic ${stmt.getText().replace(/\s+/g, " ").slice(0, 300)}`);
    }
    ts.forEachChild(node, visit);
  };
  visit(sf);
}
console.log("importers", count);
