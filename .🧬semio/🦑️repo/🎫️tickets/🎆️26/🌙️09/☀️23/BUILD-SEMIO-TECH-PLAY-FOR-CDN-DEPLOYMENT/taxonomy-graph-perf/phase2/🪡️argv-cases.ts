/** 🪡️ Prints the embedded-argv token values the production JSON adapter emits for each decoded command string (argv). */
import { readFileSync } from "node:fs";
import ts from "typescript";
const syntax = ts.createSourceFile("n.ts", readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts", "utf8"), ts.ScriptTarget.Latest, true);
const wanted = new Set(["jsonTokens", "embeddedArgumentTokens", "lineLocation", "indexedLineContent", "indexedLineStarts", "EMBEDDED_ARGUMENT_WORD", "EMBEDDED_ESCAPED_ARGUMENT_WORD", "EMBEDDED_ARGUMENT_PATH"]);
const code = syntax.statements.filter((n) => ts.isFunctionDeclaration(n) ? wanted.has(n.name?.text ?? "") : ts.isVariableStatement(n) && n.declarationList.declarations.some((d) => wanted.has(d.name.getText(syntax)))).map((n) => n.getText(syntax)).join("\n");
const jsonTokens = new Function(`${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn jsonTokens;`)();
for (const command of process.argv.slice(2)) {
  const content = JSON.stringify({ command });
  console.log(JSON.stringify(command), JSON.stringify(jsonTokens(".vscode/launch.json", content, "json").filter((t: any) => t.structuredLocation.startsWith("embedded-argv")).map((t: any) => t.value)));
}
