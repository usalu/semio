import { readFileSync, existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import ts from "typescript";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const output = join(ticket, "🗑️generated/coordinator");
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const paths = [...new Bun.Glob("**/🧪️tests/*/🟦️.ts").scanSync({ cwd: library, onlyFiles: true })];
const missing: { source: string; line: number; path: string; expression: string }[] = [];
let observed = 0;
for (const path of paths) {
  const absolute = join(library, path), source = ts.createSourceFile(absolute, readFileSync(absolute, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const resolveDeclaration = (name: string, node: ts.Node): ts.Expression | undefined => {
    for (let parent: ts.Node | undefined = node.parent; parent; parent = parent.parent) {
      if ((ts.isForOfStatement(parent) || ts.isForInStatement(parent)) && ts.isVariableDeclarationList(parent.initializer) && parent.initializer.declarations.some(declaration => ts.isIdentifier(declaration.name) && declaration.name.text === name)) return undefined;
      if (ts.isFunctionLike(parent) && parent.parameters.some(parameter => ts.isIdentifier(parameter.name) && parameter.name.text === name)) return undefined;
      if (!ts.isBlock(parent) && !ts.isSourceFile(parent)) continue;
      for (const statement of parent.statements) if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name) && declaration.name.text === name) return declaration.initializer;
    }
    return undefined;
  };
  const value = (node: ts.Expression, seen = new Set<ts.Node>()): string | undefined => {
    if (seen.has(node)) return undefined;
    seen = new Set([...seen, node]);
    if (ts.isStringLiteralLike(node)) return node.text;
    if (ts.isParenthesizedExpression(node)) return value(node.expression, seen);
    if (node.getText(source) === "import.meta.dir") return dirname(absolute);
    if (node.getText(source) === "import.meta.path") return absolute;
    if (ts.isIdentifier(node)) { const declaration = resolveDeclaration(node.text, node); return declaration ? value(declaration, seen) : undefined; }
    if (ts.isCallExpression(node)) {
      const args = node.arguments.map(argument => value(argument, seen));
      if (args.some(argument => argument === undefined)) return undefined;
      const name = node.expression.getText(source);
      if (["join", "resolve", "dirname", "posix.join"].includes(name)) return name === "dirname" ? dirname(args[0]!) : name === "resolve" ? resolve(...args as string[]) : join(...args as string[]);
    }
    return undefined;
  };
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ["readFileSync", "Bun.file"].includes(node.expression.getText(source)) && node.arguments[0]) {
      const target = value(node.arguments[0]);
      if (target && target.startsWith(root + "/") && !target.includes("/🗑️generated/")) {
        observed++;
        if (!existsSync(target)) missing.push({ source: relative(root, absolute), line: source.getLineAndCharacterOfPosition(node.getStart()).line + 1, path: relative(root, target), expression: node.arguments[0].getText(source) });
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
}
mkdirSync(output, { recursive: true });
writeFileSync(join(output, "computed-fixture-read-audit.json"), JSON.stringify({ files: paths.length, observed, missing }, null, 2) + "\n");
writeFileSync(join(ticket, "📓️computed-fixture-read-audit-2026-09-09.md"), "# Computed Fixture Read Audit\n\nTypeScript AST inspection resolves literal filesystem reads and lexical constants composed with join, resolve and dirname. Dynamic paths, parameters and runtime computations remain outside this bounded audit. Missing paths are candidates for review, not proof that every branch executes.\n\n```json\n" + JSON.stringify({ files: paths.length, observed, missing }, null, 2) + "\n```\n");
console.log(JSON.stringify({ files: paths.length, observed, missing: missing.length }));

