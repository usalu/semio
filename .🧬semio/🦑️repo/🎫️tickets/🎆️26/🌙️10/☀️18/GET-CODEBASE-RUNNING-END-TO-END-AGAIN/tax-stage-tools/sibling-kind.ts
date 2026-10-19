// 🔭 Classifies the sibling 🟦️.ts of every script of a list: real test module (imports a test runner) or helper module. usage: bun sibling-kind.ts <list>
import {existsSync, readFileSync} from "node:fs";
import {dirname, join} from "node:path";
import ts from "typescript";
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const m = join(dirname(p), "🟦️.ts");
  const short = p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "L/");
  if (!existsSync(m)) { console.log(`${short} | none`); continue; }
  const f = ts.createSourceFile(m, readFileSync(m, "utf8"), ts.ScriptTarget.Latest, true);
  const runner = f.statements.some((s) => ts.isImportDeclaration(s) && /^(bun:test|vitest|@jest\/globals)$/u.test((s.moduleSpecifier as ts.StringLiteral).text));
  const top = f.statements.filter((s) => ts.isExpressionStatement(s)).length;
  console.log(`${short} | ${runner ? "TEST-RUNNER" : "module"} topLevelExpr=${top}`);
}
