/** 🚧️ Old-vs-new `unsupportedReferenceTokens` differential over every tracked textual file: byte-identical token lists and both timings.
 * A differing file is classified as an engine-limit case when every token only one side has is a quoted literal whose body needs at least
 * JSC's 638,976 regex iterations (where the original regex silently fails) or lies inside one. argv: <repoRoot> <oldSource> <newSource> */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import ts from "typescript";

const [repoRoot, oldSource, newSource] = process.argv.slice(2);
const names = new Set(["unsupportedReferenceTokens", "quotedLiteralMatches", "pathLiteralMatches", "PATH_RUN_EXCLUDED", "PATH_LOOKAHEAD", "PATH_PREFIXES", "REGEX_WHITESPACE", "LINE_TERMINATORS", "lineLocation", "indexedLineContent", "indexedLineStarts"]);
const load = (file: string): ((content: string, adapter: string) => unknown) => {
  const syntax = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
  const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => names.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  return new Function(`${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn unsupportedReferenceTokens;`)();
};
const before = load(oldSource!), after = load(newSource!);
const files = execFileSync("git", ["ls-files", "-z"], { cwd: repoRoot, maxBuffer: 1 << 30 }).toString("utf8").split("\0").filter((path) => /\.(?:rs|tsx?|jsx?|mjs|cjs|json|jsonc|toml|ya?ml|md|py|go|feature)$/u.test(path));
let compared = 0, different = 0, engineLimit = 0, unexplained = 0, oldMs = 0, newMs = 0;
for (const path of files) {
  let content: string;
  try { content = readFileSync(`${repoRoot}/${path}`, "utf8"); } catch { continue; }
  if (content.length > 16 * 1024 * 1024) continue;
  compared++;
  let started = performance.now();
  const left = JSON.stringify(before(content, "json"));
  oldMs += performance.now() - started;
  started = performance.now();
  const right = JSON.stringify(after(content, "json"));
  newMs += performance.now() - started;
  if (left === right) continue;
  different++;
  const oldTokens = JSON.parse(left) as { start: number; end: number }[], newTokens = JSON.parse(right) as { start: number; end: number; value: string }[];
  const huge = newTokens.filter((token) => /^["'`]/u.test(content[token.start - 1] ?? "") && [...token.value.replace(/\\[^]/gu, "e")].length >= 638_976);
  const inside = (token: { start: number; end: number }) => huge.some((literal) => token.start >= literal.start && token.end <= literal.end);
  const key = (token: { start: number; end: number }) => `${token.start}:${token.end}`, oldKeys = new Set(oldTokens.map(key)), newKeys = new Set(newTokens.map(key));
  const explained = newTokens.every((token) => oldKeys.has(key(token)) || inside(token)) && oldTokens.every((token) => newKeys.has(key(token)) || inside(token));
  if (explained) engineLimit++;
  else if (unexplained++ < 5) console.log(`[DEBUG] UNEXPLAINED ${path}`);
}
console.log(`[DEBUG] files=${compared} different=${different} engineLimit=${engineLimit} unexplained=${unexplained} old=${(oldMs / 1000).toFixed(1)}s new=${(newMs / 1000).toFixed(1)}s`);
