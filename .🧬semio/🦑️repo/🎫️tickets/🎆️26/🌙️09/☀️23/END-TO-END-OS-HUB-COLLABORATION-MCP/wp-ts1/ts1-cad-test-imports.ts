/** 🔁️ TS1 one-off codemod: static imports the 09-08 extraction left inside a source's `// #region 🧪️Tests` are hoisted into the
 * production module graph (spatial-kernel `📐️geometry` → cad `🎬️actions` closes the `📔️registry` cycle). Test-only names move into
 * the extracted suite as its own imports and leave the source, its bag and its `<Dir>TestDependencies`; names production also uses
 * stay in the source and are reported. usage: bun ts1-cad-test-imports.ts <source…> [--apply] */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const args = process.argv.slice(2);
const apply = args.includes("--apply");
const sources = args.filter((arg) => arg !== "--apply").map((arg) => resolve(repo, arg));
type Edit = { readonly start: number; readonly end: number; readonly text: string };
const edits = new Map<string, Edit[]>();
const push = (file: string, edit: Edit) => (edits.get(file) ?? edits.set(file, []).get(file)!).push(edit);
const report: string[] = [];
const respec = (fromFile: string, spec: string, sourceFile: string): string => {
  if (!spec.startsWith(".")) return spec;
  const rel = relative(dirname(fromFile), resolve(dirname(sourceFile), spec));
  return rel.startsWith(".") ? rel : `./${rel}`;
};
const parse = (file: string) => ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);

for (const file of sources) {
  const source = parse(file);
  const text = source.text;
  const start = text.indexOf("// #region 🧪️Tests");
  const end = text.indexOf("// #endregion 🧪️Tests", start);
  const testSpec = /await import\("(\.\/🧪️tests\/[^"]+)"\)/.exec(text.slice(start, end))?.[1];
  if (start < 0 || !testSpec) { report.push(`${relative(repo, file)}: no tests region / suite`); continue; }
  const testFile = resolve(dirname(file), testSpec);
  const test = parse(testFile);
  const moved = new Map<string, string[]>();
  for (const statement of source.statements) {
    if (!ts.isImportDeclaration(statement) || statement.getStart() < start || statement.getStart() > end) continue;
    const bindings = statement.importClause?.namedBindings;
    if (!bindings || !ts.isNamedImports(bindings) || statement.importClause?.name) { report.push(`${relative(repo, file)}: unsupported import ${statement.getText()}`); continue; }
    const names = bindings.elements.map((element) => element.name.text);
    const productionUse = new Set<string>();
    const visit = (node: ts.Node): void => {
      if (ts.isIdentifier(node) && names.includes(node.text) && (node.getStart() < start || node.getStart() > end) && !ts.isImportSpecifier(node.parent)) productionUse.add(node.text);
      ts.forEachChild(node, visit);
    };
    visit(source);
    const keep = bindings.elements.filter((element) => productionUse.has(element.name.text));
    const move = bindings.elements.filter((element) => !productionUse.has(element.name.text));
    const spec = (statement.moduleSpecifier as ts.StringLiteral).text;
    const lineEnd = text.indexOf("\n", statement.getEnd()) + 1;
    const lineStart = text.lastIndexOf("\n", statement.getStart() - 1);
    push(file, keep.length ? { start: statement.getStart(), end: statement.getEnd(), text: `import { ${keep.map((element) => element.getText()).join(", ")} } from ${JSON.stringify(spec)};` } : { start: lineStart, end: lineEnd - 1, text: "" });
    if (keep.length) report.push(`${relative(repo, file)}: production also uses ${keep.map((element) => element.name.text).join(", ")} from ${spec} (kept in the tests region — move by hand)`);
    moved.set(respec(testFile, spec, file), move.map((element) => element.getText()));
  }
  const movedNames = new Set([...moved.values()].flat().map((text) => text.split(" as ").pop()!));
  if (!movedNames.size) { report.push(`${relative(repo, file)}: nothing to move`); continue; }
  let bag: ts.ObjectLiteralExpression | undefined;
  let alias: ts.TypeAliasDeclaration | undefined;
  const find = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "registerTests1" && ts.isObjectLiteralExpression(node.arguments[1]!)) bag = node.arguments[1] as ts.ObjectLiteralExpression;
    if (ts.isTypeAliasDeclaration(node) && node.name.text.endsWith("TestDependencies")) alias = node;
    ts.forEachChild(node, find);
  };
  find(source);
  const kept = bag!.properties.map((property) => property.getText()).filter((name) => !movedNames.has(name));
  push(file, { start: bag!.getStart(), end: bag!.getEnd(), text: `{ ${kept.join(", ")} }` });
  const literal = alias!.type as ts.TypeLiteralNode;
  for (const member of literal.members) if (member.name && movedNames.has(member.name.getText())) push(file, { start: member.getFullStart(), end: member.getEnd(), text: "" });
  const register = test.statements.find((statement): statement is ts.FunctionDeclaration => ts.isFunctionDeclaration(statement) && statement.name?.text === "registerTests1");
  const destructure = register!.body!.statements.find((statement) => ts.isVariableStatement(statement) && statement.declarationList.declarations[0]?.initializer?.getText() === "dependencies") as ts.VariableStatement;
  const pattern = destructure.declarationList.declarations[0]!.name as ts.ObjectBindingPattern;
  push(testFile, { start: pattern.getStart(), end: pattern.getEnd(), text: `{ ${pattern.elements.map((element) => element.getText()).filter((name) => !movedNames.has(name)).join(", ")} }` });
  for (const statement of test.statements) {
    if (!ts.isImportDeclaration(statement) || !statement.importClause?.isTypeOnly) continue;
    const bindings = statement.importClause.namedBindings;
    if (!bindings || !ts.isNamedImports(bindings)) continue;
    const rest = bindings.elements.filter((element) => !movedNames.has(element.name.text));
    if (rest.length === bindings.elements.length) continue;
    const lineEnd = test.text.indexOf("\n", statement.getEnd()) + 1;
    push(testFile, rest.length ? { start: bindings.getStart(), end: bindings.getEnd(), text: `{ ${rest.map((element) => element.getText()).join(", ")} }` } : { start: statement.getStart(), end: lineEnd, text: "" });
  }
  const importText = [...moved].filter(([, names]) => names.length).map(([spec, names]) => `import { ${names.join(", ")} } from ${JSON.stringify(spec)};\n`).join("");
  push(testFile, { start: 0, end: 0, text: importText });
  report.push(`${relative(repo, file)}: moved ${movedNames.size} test-only names into ${relative(repo, testFile)}`);
}

for (const [file, list] of edits) {
  let text = readFileSync(file, "utf8");
  for (const edit of [...list].sort((left, right) => right.start - left.start || right.end - left.end)) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
  if (apply) writeFileSync(file, text);
  report.push(`${apply ? "wrote" : "would write"} ${relative(repo, file)} (${list.length} edits)`);
}
console.log(report.join("\n"));
