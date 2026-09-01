import { execSync } from "node:child_process";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import * as ts from "typescript";

const root = "/Users/ueli/Documents/semio";
const normPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
const normRel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

function extractClassifier(source: string): (content: string, max: number) => string {
  const syntax = ts.createSourceFile(normPath, source, ts.ScriptTarget.Latest, true);
  const functions = new Set(["classifyGlue", "isRustDeclarativeStatementSequence", "stripDeclarativeRustModuleBlocks", "stripStringLiterals", "isConfigDelegationModule", "splitTopLevelStatements"]);
  const constants = new Set(["RUST_DECLARATIVE_STATEMENT_SEQUENCE"]);
  const extracted = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? functions.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((d) => constants.has(d.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  const compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(extracted);
  return new Function(compiled + "\nreturn classifyGlue;")();
}

const currentSource = readFileSync(normPath, "utf8");
const oldSource = execSync(`git show HEAD:"${normRel}"`, { cwd: root, maxBuffer: 1024 * 1024 * 64 }).toString();
const oldClassify = extractClassifier(oldSource);
const newClassify = extractClassifier(currentSource);

function walk(dir: string, out: string[]) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    const st = statSync(full);
    if (st.isDirectory()) walk(full, out);
    else if (entry.name.endsWith(".rs")) out.push(full);
  }
}

const files: string[] = [];
for (const plugin of readdirSync(join(root, "✏️s/🔌️plugins"), { withFileTypes: true })) {
  if (!plugin.isDirectory()) continue;
  const pkgDir = join(root, "✏️s/🔌️plugins", plugin.name, "📦️packages/🦀️rust");
  try { walk(pkgDir, files); } catch {}
}
// also framework-wide 📦️packages/🦀️rust
try { walk(join(root, "🧰️framework"), files); } catch {}

let changed = 0, sameCount = 0;
for (const f of files) {
  if (!f.includes("📦️packages/🦀️rust")) continue;
  const content = readFileSync(f, "utf8");
  const before = oldClassify("rust", content, 32);
  const after = newClassify("rust", content, 32);
  if (before !== after) {
    changed++;
    console.log("CHANGED:", f.replace(root + "/", ""), before, "->", after);
  } else {
    sameCount++;
  }
}
console.log("\nTotal files checked:", files.filter((f) => f.includes("📦️packages/🦀️rust")).length, "changed:", changed, "same:", sameCount);
