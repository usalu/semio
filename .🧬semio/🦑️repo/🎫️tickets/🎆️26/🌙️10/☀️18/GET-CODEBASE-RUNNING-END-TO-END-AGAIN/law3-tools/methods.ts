import { readFileSync } from "node:fs";
import ts from "typescript";
const base = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/";
for (const path of ["🍎️image/🟦️.ts", "🧱️fixed/🟦️.ts", "🧹️cleanup/🎛️command/🟦️.ts"]) {
  const file = ts.createSourceFile(path, readFileSync(base + path, "utf8"), ts.ScriptTarget.Latest, true);
  const visit = (node: ts.Node): void => { if (ts.isPropertyAssignment(node) && node.name.getText(file) === "withExclusiveMaintenance" && ts.isArrowFunction(node.initializer)) console.log("=====", path, "\n" + node.initializer.getText(file)); ts.forEachChild(node, visit); };
  visit(file);
}
