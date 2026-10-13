/** 🧮️ Proves the only difference between the original quoted-literal regex and the linear scanner is JSC's backtracking limit: an attempt
 * needing at least `LIMIT` iterations (one per escape or code point) makes `exec` return null, which silently ends `matchAll` for the rest of
 * the file. For every tracked textual file the regex's matches equal a copy of the scanner that stops there, while the production scanner's
 * path matches equal the path regex. argv: <repoRoot> <normalizationSource> */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import ts from "typescript";

const LIMIT = 638_976;
const [repoRoot, source] = process.argv.slice(2);
const names = new Set(["pathLiteralMatches", "PATH_RUN_EXCLUDED", "PATH_LOOKAHEAD", "PATH_PREFIXES", "REGEX_WHITESPACE", "LINE_TERMINATORS"]);
const syntax = ts.createSourceFile(source!, readFileSync(source!, "utf8"), ts.ScriptTarget.Latest, true);
const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => names.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax)).join("\n");
const paths = new Function(`${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn pathLiteralMatches;`)() as (content: string) => [number, string, string][];
const limited = (content: string): [number, string, string][] => {
  const rows: [number, string, string][] = [];
  scan: for (let index = 0; index < content.length; index++) {
    const quote = content[index];
    if (quote !== '"' && quote !== "'" && quote !== "`") continue;
    let cursor = index + 1, closed = false, iterations = 0;
    while (cursor < content.length) {
      const character = content[cursor]!;
      if (character === quote) { closed = cursor > index + 1; break; }
      if (character === "\r" || character === "\n") break;
      if (++iterations >= LIMIT) break scan;
      if (character === "\\") { const escaped = content.codePointAt(cursor + 1); if (escaped === undefined || escaped === 10 || escaped === 13 || escaped === 0x2028 || escaped === 0x2029) break; cursor += escaped > 0xffff ? 3 : 2; continue; }
      cursor += content.codePointAt(cursor)! > 0xffff ? 2 : 1;
    }
    if (!closed) continue;
    rows.push([index, content.slice(index, cursor + 1), content.slice(index + 1, cursor)]);
    index = cursor;
  }
  return rows;
};
const quoted = /"((?:\\.|[^"\\\r\n])+)"|'((?:\\.|[^'\\\r\n])+)'|`((?:\\.|[^`\\\r\n])+)`/gu, pathPattern = /(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s"'`),;\]])/gmu;
const oracle = (pattern: RegExp, content: string): string => JSON.stringify([...content.matchAll(pattern)].map((match) => [match.index!, match[0], match[1] ?? match[2] ?? match[3]]));
let files = 0, quotedDifferent = 0, pathDifferent = 0, limitCases = 0;
for (const path of execFileSync("git", ["ls-files", "-z"], { cwd: repoRoot, maxBuffer: 1 << 30 }).toString("utf8").split("\0").filter((file) => /\.(?:rs|tsx?|jsx?|mjs|cjs|json|jsonc|toml|ya?ml|md|py|go|feature)$/u.test(file))) {
  let content: string;
  try { content = readFileSync(`${repoRoot}/${path}`, "utf8"); } catch { continue; }
  if (content.length > 16 * 1024 * 1024) continue;
  files++;
  if (oracle(quoted, content) !== JSON.stringify(limited(content)) && quotedDifferent++ < 5) console.log(`[DEBUG] quoted regex differs from the limited scanner: ${path}`);
  if (oracle(pathPattern, content) !== JSON.stringify(paths(content)) && pathDifferent++ < 5) console.log(`[DEBUG] path regex differs from the scanner: ${path}`);
  if (/["'`][^\r\n]{638976}/u.test(content)) limitCases++;
}
console.log(`[DEBUG] files=${files} quotedDifferentFromLimitedScanner=${quotedDifferent} pathDifferent=${pathDifferent} filesWithLiteralsBeyondLimit=${limitCases}`);
