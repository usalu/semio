/** 🧷️ Lists overlapping reference tokens that the JSON adapter emits for one file (optionally only around one offset), using the
 * production declarations compiled from the normalizer. argv: <repoRoot> <path> [offset] */
import { readFileSync } from "node:fs";
import ts from "typescript";

const [repoRoot, path, around] = process.argv.slice(2);
const source = readFileSync(`${repoRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`, "utf8");
const syntax = ts.createSourceFile("n.ts", source, ts.ScriptTarget.Latest, true);
const wanted = new Set(["jsonTokens", "embeddedArgumentTokens", "lineLocation", "indexedLineContent", "indexedLineStarts", "EMBEDDED_ARGUMENT_WORD", "EMBEDDED_ESCAPED_ARGUMENT_WORD", "EMBEDDED_ARGUMENT_PATH"]);
const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? wanted.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((d) => wanted.has(d.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
const jsonTokens = new Function(`${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn jsonTokens;`)() as (path: string, content: string, adapter: string) => { start: number; end: number; value: string; structuredLocation: string }[];
const content = readFileSync(`${repoRoot}/${path}`, "utf8");
const tokens = jsonTokens(path, content, path.endsWith(".jsonc") ? "jsonc" : "json").sort((a, b) => a.start - b.start || a.end - b.end);
let overlaps = 0;
for (let index = 1; index < tokens.length; index++) {
  const previous = tokens[index - 1]!, current = tokens[index]!;
  if (current.start < previous.end && !(current.start === previous.start && current.end === previous.end)) {
    overlaps++;
    if (around === undefined ? overlaps <= 5 : Math.abs(current.start - Number(around)) < 400) console.log(`[DEBUG] overlap ${previous.structuredLocation} ${JSON.stringify(previous.value)} | ${current.structuredLocation} ${JSON.stringify(current.value)}`);
  }
}
console.log(`[DEBUG] tokens=${tokens.length} overlapping-pairs=${overlaps}`);
