import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";

const repoRoot = "/Users/ueli/Documents/semio";
const specific = "✏️s/🧑‍💻dev/🎭️variants/🌍️gis";
const directory = join(repoRoot, specific, "🧩️service-composition", "🧪️tests");
let receivingFixtures = 0;
let importMetaReaders = 0;
let missing = 0;
for (const entry of readdirSync(directory, { withFileTypes: true }).filter((row) => row.isDirectory())) {
  const path = join(directory, entry.name, "🟦️.ts");
  const source = readFileSync(path, "utf8");
  const tree = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const declarations: Array<{ name: string; value: string; position: number }> = [];
  const visit = (node: ts.Node): void => {
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && ts.isStringLiteral(node.initializer)) declarations.push({ name: node.name.text, value: node.initializer.text, position: node.pos });
    if (ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "URL" && node.arguments?.[1]?.getText(tree) === "source.url") {
      const argument = node.arguments[0]!;
      let target: string | undefined;
      if (ts.isStringLiteral(argument)) target = argument.text;
      else if (ts.isBinaryExpression(argument) && ts.isIdentifier(argument.left) && ts.isStringLiteral(argument.right)) {
        const declaration = declarations.filter((row) => row.name === (argument.left as ts.Identifier).text && row.position < node.pos).at(-1);
        if (declaration) target = declaration.value + argument.right.text;
      }
      if (target) {
        const found = existsSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os", target));
        if (!found) { missing += 1; console.log("MISSING source.url", entry.name, target); }
        receivingFixtures += 1;
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(tree);
  for (const match of source.matchAll(/new URL\("([^"]+)",\s*import.meta.url\)/g)) {
    importMetaReaders += 1;
    const resolved = new URL(match[1]!, new URL(`file://${path}`));
    if (!existsSync(decodeURIComponent(resolved.pathname))) { missing += 1; console.log("MISSING import.meta.url", entry.name, match[1]); }
  }
}
console.log(JSON.stringify({ receivingFixtures, importMetaReaders, missing }));
