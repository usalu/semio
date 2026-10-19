// 🔬 Evaluates the command-router validator on a script and on variants with one top-level statement removed, to find the rejecting statement. usage: bun router-variants.ts <script.ts>
import {readFileSync} from "node:fs";
import {ecmaCommandRouterModule} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🛣️ecma-command-routes/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import ts from "typescript";
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const source = readFileSync(process.argv[2]!, "utf8");
console.log("whole:", ecmaCommandRouterModule(source, grammar));
const file = ts.createSourceFile("s.ts", source, ts.ScriptTarget.Latest, true);
for (const [i, st] of file.statements.entries()) {
  const cut = source.slice(0, st.getFullStart()) + source.slice(st.getEnd());
  const ok = ecmaCommandRouterModule(cut, grammar);
  if (ok) console.log("removing statement", i, ts.SyntaxKind[st.kind], JSON.stringify(st.getText().slice(0, 100)), "=> accepted");
}
