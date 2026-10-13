/** 🪡️ Old-vs-new embedded-argv differential over every tracked file the JSON and YAML adapters argv-tokenize: counts tokens each
 * version emits that the other does not, classifies the removed ones (escape-spanning, word-internal `/` fragment, other) and the
 * remaining partial overlaps. argv: <repoRoot> <oldNormalizationSource> <newNormalizationSource> */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import ts from "typescript";

type Token = { start: number; end: number; value: string; structuredLocation: string };
const [repoRoot, oldSource, newSource] = process.argv.slice(2);
const names = new Set(["jsonTokens", "embeddedArgumentTokens", "regexTokens", "lineLocation", "indexedLineContent", "indexedLineStarts", "EMBEDDED_ARGUMENT_WORD", "EMBEDDED_ESCAPED_ARGUMENT_WORD", "EMBEDDED_ARGUMENT_PATH"]);
const load = (file: string) => {
  const syntax = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
  const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => names.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  const api = new Function(`${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn { jsonTokens, embeddedArgumentTokens, regexTokens };`)();
  return (path: string, content: string): Token[] => {
    if (/\.jsonc?$/u.test(path)) return api.jsonTokens(path, content, path.endsWith(".jsonc") ? "jsonc" : "json").filter((token: Token) => token.structuredLocation.startsWith("embedded-argv"));
    const direct = api.regexTokens(content, "yaml", "yaml-value", [/^\s*(?:-\s*)?[\w.-]+\s*:\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu, /^\s*-\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu]);
    return direct.flatMap((token: Token) => api.embeddedArgumentTokens(content, token.value, token.start, "yaml", "embedded-argv", false));
  };
};
const before = load(oldSource!), after = load(newSource!);
const files = execFileSync("git", ["ls-files", "-z"], { cwd: repoRoot, maxBuffer: 1 << 30 }).toString("utf8").split("\0").filter((path) => /(?:^|\/)(?:launch(?:\.seed)?\.jsonc?|tasks\.json|project\.json|package\.json)$/iu.test(path) || /\.ya?ml$/iu.test(path) && /(?:workflow|action|launch|task|project|(?:^|\/)ci(?:\/|$))/iu.test(path));
const key = (token: Token) => `${token.start}\0${token.end}\0${token.value}`;
let removed = 0, added = 0, escape = 0, internal = 0, other = 0, partialBefore = 0, partialAfter = 0, changedFiles = 0;
const samples: string[] = [];
const partial = (tokens: Token[]) => { const sorted = [...tokens].sort((a, b) => a.start - b.start || b.end - a.end); let count = 0; for (let i = 0; i < sorted.length; i++) for (let j = i + 1; j < sorted.length && sorted[j]!.start < sorted[i]!.end; j++) if (sorted[j]!.end > sorted[i]!.end) count++; return count; };
for (const path of files) {
  let content: string;
  try { content = readFileSync(`${repoRoot}/${path}`, "utf8"); } catch { continue; }
  const left = before(path, content), right = after(path, content), rightKeys = new Set(right.map(key)), leftKeys = new Set(left.map(key));
  const gone = left.filter((token) => !rightKeys.has(key(token))), fresh = right.filter((token) => !leftKeys.has(key(token)));
  partialBefore += partial(left); partialAfter += partial(right);
  if (gone.length || fresh.length) changedFiles++;
  removed += gone.length; added += fresh.length;
  for (const token of gone) {
    if (token.value.includes("\\")) escape++;
    else if (token.value.startsWith("/") && token.start > 0 && !/[\s"'`([,;=:@]/u.test(content[token.start - 1]!)) internal++;
    else { other++; if (samples.length < 12) samples.push(`removed ${path} ${token.structuredLocation} ${JSON.stringify(token.value)}`); }
  }
  for (const token of fresh) if (samples.length < 24) samples.push(`added ${path} ${token.structuredLocation} ${JSON.stringify(token.value)}`);
}
console.log(`[DEBUG] files=${files.length} changedFiles=${changedFiles} removed=${removed} (escape-spanning=${escape} word-internal-root=${internal} other=${other}) added=${added} partialOverlaps old=${partialBefore} new=${partialAfter}`);
for (const sample of samples) console.log(`[DEBUG] ${sample.slice(0, 300)}`);
