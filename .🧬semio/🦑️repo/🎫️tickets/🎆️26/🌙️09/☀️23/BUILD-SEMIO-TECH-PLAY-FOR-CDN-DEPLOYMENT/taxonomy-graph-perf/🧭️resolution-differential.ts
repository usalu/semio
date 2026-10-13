/** 🧭️ Differential oracle for reference resolution: resolves every reference token of every corpus file against the real repository path
 * index with the pre-change snapshot's and the current `resolveReferencePath`/`lexicalOpaqueReferenceTarget`, and reports any difference.
 * argv: <oldSource> <newSource> <pathsListFile> <corpusListFile>; run from the repository root. */
import { readFileSync } from "node:fs";
import ts from "typescript";
import * as path from "node:path";

const [oldSource, newSource, pathsList, corpusList] = process.argv.slice(2);
const names = new Set(["resolveReferencePath", "referenceLeaf", "referenceLeafNames", "referenceLeafNameSets", "referencePathIndex", "addUniqueIndex", "ancestorReferenceCoordinateRoot", "splitTokenSuffix", "normalizeRelative", "sourceRelative", "lexicalOpaqueReferenceTarget", "opaqueLeafNames", "opaqueLeafNameLists", "isExcluded", "jsonTokens", "TOML_TABLE_HEADER", "tomlTokens", "embeddedArgumentTokens", "markdownInlineTokens", "markdownReferenceDefinitionTokens", "MARKDOWN_WHITESPACE_RUN", "MARKDOWN_TEXT_RUN", "MARKDOWN_LINE_TERMINATOR", "regexTokens", "htmlTokens", "lineLocation", "indexedLineContent", "indexedLineStarts"]);

/** 📜️ Transpiles the named declarations of one source into a callable bundle. */
function bundle(file: string): Record<string, (...args: never[]) => unknown> {
  const sources = [file, path.join(path.dirname(file), "🛣️path/🟦️.ts")];
  const code = sources.flatMap((source) => {
    const syntax = ts.createSourceFile(source, readFileSync(source, "utf8"), ts.ScriptTarget.Latest, true);
    return syntax.statements.filter((node) => (ts.isFunctionDeclaration(node) && node.name && names.has(node.name.text)) || (ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => names.has(declaration.name.getText(syntax))))).map((node) => node.getText(syntax).replace(/^export\s+/u, ""));
  }).join("\n");
  const javascript = ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  return new Function("basename", "dirname", "posix", `${javascript}\nreturn { resolveReferencePath, referencePathIndex, lexicalOpaqueReferenceTarget, jsonTokens, tomlTokens, markdownInlineTokens, regexTokens, htmlTokens, markdownReferenceDefinitionTokens: typeof markdownReferenceDefinitionTokens === "function" ? markdownReferenceDefinitionTokens : (content) => regexTokens(content, "markdown", "markdown-link", [/^\\s*\\[[^\\]]+\\]:\\s*(\\S+)/gmu]) };`)(path.basename, path.dirname, path.posix) as Record<string, (...args: never[]) => unknown>;
}

const before = bundle(oldSource!), after = bundle(newSource!);
const paths = readFileSync(pathsList!, "utf8").split("\n").filter(Boolean);
const directories = new Set<string>();
for (const file of paths) for (let parent = path.posix.dirname(file); parent && parent !== "."; parent = path.posix.dirname(parent)) directories.add(parent);
const known = [...paths, ...directories];
const taxonomy = { exclusions: [{ path: "compose" }, { path: "temp/compose" }, { path: "♻️mit-bestand/🔎️recherche" }] };
const oldIndex = (before.referencePathIndex as (paths: string[]) => unknown)(known), newIndex = (after.referencePathIndex as (paths: string[]) => unknown)(known);
type Token = { value: string; targetValues?: string[]; structuredLocation: string };
const tokensOf = (file: string, content: string): Token[] => {
  const lower = file.toLowerCase();
  if (lower.endsWith(".json")) return (after.jsonTokens as (p: string, c: string, a: string) => Token[])(file, content, "json");
  if (lower.endsWith(".toml")) return (after.tomlTokens as (p: string, c: string) => Token[])(file, content);
  if (lower.endsWith(".md")) return [...(after.markdownInlineTokens as (c: string) => Token[])(content), ...(after.markdownReferenceDefinitionTokens as (c: string) => Token[])(content), ...(after.htmlTokens as (c: string, a: string) => Token[])(content, "markdown")];
  return [];
};
let files = 0, tokens = 0, differences = 0, oldMs = 0, newMs = 0;
for (const file of readFileSync(corpusList!, "utf8").split("\n").filter(Boolean)) {
  let content: string;
  try { content = readFileSync(file, "utf8"); } catch { continue; }
  files++;
  for (const token of tokensOf(file, content)) {
    tokens++;
    for (const value of token.targetValues ?? [token.value]) {
      let t = performance.now();
      const left = JSON.stringify([(before.resolveReferencePath as (r: string, v: string, i: unknown) => unknown)(file, value, oldIndex), (before.lexicalOpaqueReferenceTarget as (r: string, t: Token, x: unknown) => unknown)(file, token, taxonomy)]);
      oldMs += performance.now() - t;
      t = performance.now();
      const right = JSON.stringify([(after.resolveReferencePath as (r: string, v: string, i: unknown) => unknown)(file, value, newIndex), (after.lexicalOpaqueReferenceTarget as (r: string, t: Token, x: unknown) => unknown)(file, token, taxonomy)]);
      newMs += performance.now() - t;
      if (left !== right) { differences++; if (differences <= 5) console.log(`[DEBUG] difference ${file} ${JSON.stringify(value).slice(0, 120)} old=${left} new=${right}`); }
    }
  }
}
console.log(`[DEBUG] resolution: files=${files} tokens=${tokens} differences=${differences} old=${(oldMs / 1000).toFixed(2)}s new=${(newMs / 1000).toFixed(2)}s`);
