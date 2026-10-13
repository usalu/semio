/** 🔍️ First differing match between the original unsupported-path regexes and the linear scanners for one file. argv: <normalizationSource> <file> */
import { readFileSync } from "node:fs";
import ts from "typescript";
const [source, file] = process.argv.slice(2);
const names = new Set(["quotedLiteralMatches", "pathLiteralMatches", "PATH_RUN_EXCLUDED", "PATH_LOOKAHEAD", "PATH_PREFIXES", "REGEX_WHITESPACE", "LINE_TERMINATORS"]);
const syntax = ts.createSourceFile(source!, readFileSync(source!, "utf8"), ts.ScriptTarget.Latest, true);
const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => names.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax)).join("\n");
const scan = new Function(`${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn { quoted: quotedLiteralMatches, paths: pathLiteralMatches };`)();
const content = readFileSync(file!, "utf8");
const regexes = { quoted: /"((?:\\.|[^"\\\r\n])+)"|'((?:\\.|[^'\\\r\n])+)'|`((?:\\.|[^`\\\r\n])+)`/gu, paths: /(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s"'`),;\]])/gmu };
for (const kind of ["quoted", "paths"] as const) {
  const expected = [...content.matchAll(regexes[kind])].map((match) => [match.index!, match[0], match[1] ?? match[2] ?? match[3]]), actual = scan[kind](content);
  for (let index = 0; index < Math.max(expected.length, actual.length); index++) if (JSON.stringify(expected[index]) !== JSON.stringify(actual[index])) {
    const at = (expected[index]?.[0] ?? actual[index]?.[0]) as number;
    console.log(`[DEBUG] ${kind} #${index} expected=${JSON.stringify(expected[index])} actual=${JSON.stringify(actual[index])} context=${JSON.stringify(content.slice(Math.max(0, at - 40), at + 80))}`);
    break;
  }
}
