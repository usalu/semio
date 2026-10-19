import { resolve } from "node:path";
import ts from "typescript";

const file = resolve("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts");
const program = ts.createProgram([file], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, allowImportingTsExtensions: true, allowJs: true, skipLibCheck: true, noEmit: true, types: ["node"] });
const checker = program.getTypeChecker();
const source = program.getSourceFile(file)!;
const names = new Set(["graph", "registry", "declared", "resolved", "expected", "contribution", "project", "target"]);
const visit = (node: ts.Node): void => {
  if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && names.has(node.name.text)) console.log("[DEBUG]", node.name.text, checker.typeToString(checker.getTypeAtLocation(node.name)).slice(0, 300));
  if (ts.isBindingElement(node) && ts.isIdentifier(node.name) && names.has(node.name.text)) console.log("[DEBUG]", node.name.text, checker.typeToString(checker.getTypeAtLocation(node.name)).slice(0, 300));
  ts.forEachChild(node, visit);
};
visit(source);
