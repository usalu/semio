/** 🔎️ TS1: classifies every name an in-source `registerTests1(import.meta.vitest, { … })` bag passes (cad + spatial-kernel). */
import ts from "typescript";
import { dirname, relative, resolve } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const config = resolve(repo, "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json");
const parsed = ts.getParsedCommandLineOfConfigFile(config, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: () => {} })!;
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options });
const checker = program.getTypeChecker();
const rows: string[] = [];
for (const file of program.getSourceFiles()) {
  if (file.fileName.includes("node_modules") || !/✏️s\//.test(file.fileName)) continue;
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "registerTests1" && node.arguments.length === 3 && ts.isObjectLiteralExpression(node.arguments[1]!)) {
      for (const property of node.arguments[1].properties) {
        if (!ts.isShorthandPropertyAssignment(property)) { rows.push(`${relative(repo, file.fileName)}\tNON-SHORTHAND\t${property.getText()}`); continue; }
        const local = checker.getShorthandAssignmentValueSymbol(property);
        if (!local) { rows.push(`${relative(repo, file.fileName)}\t${property.name.text}\tUNRESOLVED`); continue; }
        const target = local.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(local) : local;
        const declaration = target.declarations?.[0];
        const value = (target.flags & ts.SymbolFlags.Value) !== 0;
        const type = (target.flags & ts.SymbolFlags.Type) !== 0;
        const imported = (local.flags & ts.SymbolFlags.Alias) !== 0;
        let specifier = "";
        if (imported) {
          const importDeclaration = local.declarations?.[0];
          let cursor: ts.Node | undefined = importDeclaration;
          while (cursor && !ts.isImportDeclaration(cursor)) cursor = cursor.parent;
          if (cursor && ts.isImportDeclaration(cursor)) specifier = (cursor.moduleSpecifier as ts.StringLiteral).text;
        }
        const exported = declaration ? (ts.getCombinedModifierFlags(declaration as ts.Declaration) & ts.ModifierFlags.Export) !== 0 : false;
        rows.push([relative(repo, file.fileName), property.name.text, value ? "value" : "-", type ? "type" : "-", imported ? `import:${specifier}` : `local:${exported ? "exported" : "private"}`, declaration ? relative(repo, declaration.getSourceFile().fileName) : "?"].join("\t"));
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(file);
}
console.log(rows.join("\n"));
