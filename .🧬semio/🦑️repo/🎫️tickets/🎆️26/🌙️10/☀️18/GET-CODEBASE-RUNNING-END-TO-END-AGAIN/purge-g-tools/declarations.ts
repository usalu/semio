import ts from "typescript";
import { readFileSync } from "node:fs";
const path = process.argv[2]!;
const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const names = source.statements.flatMap((statement) => {
  if ((ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) return [statement.name.text];
  if (ts.isVariableStatement(statement)) return statement.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : []));
  return [];
}).sort();
console.log(JSON.stringify(names));
