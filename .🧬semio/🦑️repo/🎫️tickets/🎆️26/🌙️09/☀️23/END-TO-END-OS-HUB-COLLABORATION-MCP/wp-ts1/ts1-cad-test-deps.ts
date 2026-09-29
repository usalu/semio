/** 🧪️ TS1 one-off codemod: types the `registerTests1(vitest, dependencies: any, …)` bags the 09-08 test extraction left in cad +
 * spatial-kernel. Each source module exports `<Dir>TestDependencies` (`typeof` of every injected value), stops passing type-only
 * names as values, and its extracted suite imports the types it uses instead of `type X = any` aliases.
 * usage: bun ts1-cad-test-deps.ts <tsc-capture-with-TS2749> [--apply] */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, basename } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const [capture, flag] = process.argv.slice(2);
const apply = flag === "--apply";
const config = resolve(repo, "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json");
const parsed = ts.getParsedCommandLineOfConfigFile(config, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: () => {} })!;
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options });
const checker = program.getTypeChecker();
const valueAsType = new Map<string, Set<string>>();
for (const line of readFileSync(capture!, "utf8").split("\n")) {
  const match = /^(.*?)\(\d+,\d+\): error TS2749: '([^']+)' refers to a value/.exec(line);
  if (!match) continue;
  const file = resolve(dirname(config), match[1]!);
  (valueAsType.get(file) ?? valueAsType.set(file, new Set()).get(file)!).add(match[2]!);
}

type Edit = { readonly start: number; readonly end: number; readonly text: string };
const edits = new Map<string, Edit[]>();
const push = (file: string, edit: Edit) => (edits.get(file) ?? edits.set(file, []).get(file)!).push(edit);
const report: string[] = [];
const specifierFrom = (fromFile: string, spec: string, sourceFile: string): string => {
  if (!spec.startsWith(".")) return spec;
  const target = resolve(dirname(sourceFile), spec);
  const rel = relative(dirname(fromFile), target);
  return rel.startsWith(".") ? rel : `./${rel}`;
};
const pascal = (dir: string): string => dir.replace(/^[^A-Za-z0-9]+/u, "").split(/[^A-Za-z0-9]+/).filter(Boolean).map((part) => part[0]!.toUpperCase() + part.slice(1)).join("");

/** 🧭️ Where a name visible at `site` in the source comes from: its import specifier (re-relativized for `testFile`) and exported name. */
function origin(site: ts.Node, name: string, meaning: ts.SymbolFlags, sourceFile: ts.SourceFile, testFile: string): { spec: string; imported: string } | string {
  const local = checker.resolveName(name, site, meaning, false);
  if (!local) return `unresolved ${name}`;
  if (local.flags & ts.SymbolFlags.Alias) {
    const declaration = local.declarations?.[0];
    if (!declaration || !(ts.isImportSpecifier(declaration) || ts.isNamespaceImport(declaration))) return `alias-kind ${name}`;
    let cursor: ts.Node = declaration;
    while (!ts.isImportDeclaration(cursor)) cursor = cursor.parent;
    const spec = (cursor.moduleSpecifier as ts.StringLiteral).text;
    const imported = ts.isImportSpecifier(declaration) ? (declaration.propertyName ?? declaration.name).text : name;
    return { spec: specifierFrom(testFile, spec, sourceFile.fileName), imported };
  }
  const declaration = local.declarations?.[0];
  if (!declaration || declaration.getSourceFile() !== sourceFile) return `foreign-local ${name}`;
  const exported = (ts.getCombinedModifierFlags(declaration as ts.Declaration) & ts.ModifierFlags.Export) !== 0;
  if (!exported) return `private-local ${name}`;
  return { spec: specifierFrom(testFile, `./${basename(sourceFile.fileName)}`, sourceFile.fileName), imported: name };
}

for (const sourceFile of program.getSourceFiles()) {
  if (!sourceFile.fileName.includes("/✏️s/") || sourceFile.fileName.includes("node_modules")) continue;
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && node.expression.getText(sourceFile) === "import.meta.vitest" && ts.isBlock(node.thenStatement)) {
      const statements = node.thenStatement.statements;
      const importStatement = statements.find((statement) => /await import\("(\.[^"]+)"\)/.test(statement.getText(sourceFile)) && statement.getText(sourceFile).includes("registerTests1"));
      const call = statements.map((statement) => statement.getText(sourceFile)).join("\n").includes("registerTests1(import.meta.vitest") ? statements : null;
      if (importStatement && call) {
        const testSpec = /await import\("(\.[^"]+)"\)/.exec(importStatement.getText(sourceFile))![1]!;
        const testFile = resolve(dirname(sourceFile.fileName), testSpec);
        let callNode: ts.CallExpression | undefined;
        const find = (inner: ts.Node): void => {
          if (ts.isCallExpression(inner) && ts.isIdentifier(inner.expression) && inner.expression.text === "registerTests1") callNode = inner;
          ts.forEachChild(inner, find);
        };
        find(node.thenStatement);
        const bag = callNode!.arguments[1]! as ts.ObjectLiteralExpression;
        const typeName = `${pascal(basename(dirname(sourceFile.fileName)))}TestDependencies`;
        const kept: string[] = [];
        const typeOnly: string[] = [];
        for (const property of bag.properties) {
          if (!ts.isShorthandPropertyAssignment(property)) { report.push(`${relative(repo, sourceFile.fileName)}: non-shorthand ${property.getText(sourceFile)}`); kept.push(property.getText(sourceFile)); continue; }
          const local = checker.getShorthandAssignmentValueSymbol(property)!;
          const target = local.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(local) : local;
          if (target.flags & ts.SymbolFlags.Value) kept.push(property.name.text);
          else typeOnly.push(property.name.text);
        }
        push(sourceFile.fileName, { start: bag.getStart(sourceFile), end: bag.getEnd(), text: `{ ${kept.join(", ")} }` });
        const members = kept.map((name) => `  readonly ${name}: typeof ${name};`).join("\n");
        push(sourceFile.fileName, { start: node.getStart(sourceFile), end: node.getStart(sourceFile), text: `/** 🎒️ The values this module hands its extracted suite \`${testSpec}\`. */\nexport type ${typeName} = {\n${members}\n};\n\n` });
        const testText = readFileSync(testFile, "utf8");
        const testSource = ts.createSourceFile(testFile, testText, ts.ScriptTarget.ESNext, true);
        const typeImports = new Map<string, Set<string>>();
        const addImport = (spec: string, name: string) => (typeImports.get(spec) ?? typeImports.set(spec, new Set()).get(spec)!).add(name);
        addImport(specifierFrom(testFile, `./${basename(sourceFile.fileName)}`, sourceFile.fileName), typeName);
        const wanted = new Set([...typeOnly, ...(valueAsType.get(testFile) ?? [])]);
        const anyAliases: ts.TypeAliasDeclaration[] = [];
        const scan = (inner: ts.Node): void => {
          if (ts.isTypeAliasDeclaration(inner) && inner.type.kind === ts.SyntaxKind.AnyKeyword) anyAliases.push(inner);
          ts.forEachChild(inner, scan);
        };
        scan(testSource);
        for (const alias of anyAliases) wanted.add(alias.name.text);
        const resolved = new Set<string>();
        for (const name of wanted) {
          const found = origin(callNode!, name, ts.SymbolFlags.Type, sourceFile, testFile);
          if (typeof found === "string") { report.push(`${relative(repo, testFile)}: ${found}`); continue; }
          addImport(found.spec, found.imported === name ? name : `${found.imported} as ${name}`);
          resolved.add(name);
        }
        for (const alias of anyAliases) if (resolved.has(alias.name.text)) {
          const start = alias.getFullStart();
          push(testFile, { start, end: alias.getEnd(), text: "" });
        }
        const register = testSource.statements.find((statement): statement is ts.FunctionDeclaration => ts.isFunctionDeclaration(statement) && statement.name?.text === "registerTests1");
        const parameter = register?.parameters[1];
        if (!parameter?.type || parameter.type.kind !== ts.SyntaxKind.AnyKeyword) report.push(`${relative(repo, testFile)}: dependencies parameter not \`any\``);
        else push(testFile, { start: parameter.type.getStart(testSource), end: parameter.type.getEnd(), text: typeName });
        const destructure = register?.body?.statements.find((statement) => ts.isVariableStatement(statement) && statement.declarationList.declarations[0]?.initializer?.getText(testSource) === "dependencies") as ts.VariableStatement | undefined;
        const pattern = destructure?.declarationList.declarations[0]?.name;
        if (pattern && ts.isObjectBindingPattern(pattern)) {
          const names = pattern.elements.map((element) => element.getText(testSource)).filter((text) => !typeOnly.includes(text));
          push(testFile, { start: pattern.getStart(testSource), end: pattern.getEnd(), text: `{ ${names.join(", ")} }` });
        } else report.push(`${relative(repo, testFile)}: no dependencies destructure`);
        const importText = [...typeImports].map(([spec, names]) => `import type { ${[...names].sort().join(", ")} } from ${JSON.stringify(spec)};`).join("\n");
        push(testFile, { start: 0, end: 0, text: `${importText}\n\n` });
        report.push(`${relative(repo, sourceFile.fileName)} → ${typeName}: kept ${kept.length}, type-only ${typeOnly.length}, any-aliases ${anyAliases.length} (resolved ${[...resolved].filter((name) => anyAliases.some((alias) => alias.name.text === name)).length})`);
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
}

for (const [file, list] of edits) {
  let text = readFileSync(file, "utf8");
  for (const edit of [...list].sort((left, right) => right.start - left.start || right.end - left.end)) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
  if (apply) writeFileSync(file, text);
  report.push(`${apply ? "wrote" : "would write"} ${relative(repo, file)} (${list.length} edits)`);
}
console.log(report.join("\n"));
