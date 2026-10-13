/** 🧬️ Differential oracle: evaluates the same named top-level declarations from the pre-change snapshot and the current source, calls one entry with (path, content) for every corpus file, and reports any output difference plus both timings.
 * argv: <oldSource> <newSource> <entry> <names,comma,separated> <corpusListFile> [extraGlobals json]
 * The corpus list holds repo-relative paths, one per line; the repo root is the current working directory. */
import { readFileSync } from "node:fs";
import ts from "typescript";
import * as path from "node:path";

const [oldSource, newSource, entry, namesArg, corpusList] = process.argv.slice(2);
const names = new Set(namesArg!.split(","));

/** 📜️ Transpiles exactly the named declarations of one source file into a callable closure. */
function closure(file: string): (...args: unknown[]) => unknown {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
  const picked = source.statements.filter((node) => (ts.isFunctionDeclaration(node) && node.name && names.has(node.name.text)) || (ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => names.has(declaration.name.getText(source)))) || (ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node)));
  const code = ts.transpileModule(picked.map((node) => node.getText(source).replace(/^export\s+/u, "")).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  return new Function("basename", "dirname", "posix", `${code}\nreturn ${entry};`)(path.basename, path.dirname, path.posix) as (...args: unknown[]) => unknown;
}

const before = closure(oldSource!), after = closure(newSource!);
let files = 0, differences = 0, oldMs = 0, newMs = 0;
for (const relative of readFileSync(corpusList!, "utf8").split("\n").filter(Boolean)) {
  let content: string;
  try { content = readFileSync(relative, "utf8"); } catch { continue; }
  files++;
  let t = performance.now();
  const left = JSON.stringify(before(relative, content));
  oldMs += performance.now() - t;
  t = performance.now();
  const right = JSON.stringify(after(relative, content));
  newMs += performance.now() - t;
  if (left !== right) { differences++; if (differences <= 5) console.log(`[DEBUG] difference ${relative}\n  old ${left.slice(0, 300)}\n  new ${right.slice(0, 300)}`); }
}
console.log(`[DEBUG] ${entry}: files=${files} differences=${differences} old=${(oldMs / 1000).toFixed(2)}s new=${(newMs / 1000).toFixed(2)}s`);
