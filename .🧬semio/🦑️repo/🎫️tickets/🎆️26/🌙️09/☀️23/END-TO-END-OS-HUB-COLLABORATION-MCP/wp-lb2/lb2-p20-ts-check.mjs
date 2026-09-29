// 🌐️ LB2 p20 TS pre-proof (scratch): the hub publisher's linked-ownership rule lifted out of its source exactly as the vitest law
// `🌎️hub/🧪️tests/⛓️linked-codec-ownership` does, replayed over the fixture's cases, plus that law's new check (every stdio editor's
// `(kind, schema)` is owned, one creation surface per dialect) over natively described descriptors.
// usage: node lb2-p20-ts-check.mjs <scratch root> <descriptor dump dir>
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const [root, dumps] = process.argv.slice(2);
const script = readFileSync(`${root}/🌎️hub/📦️packages/🦀️rust/📜️script.ts`, "utf8");
const syntax = ts.createSourceFile("script.ts", script, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "trustedBootstrapLinkedUnownedKindsV1");
const javascript = ts.transpileModule(declaration.getText(syntax).replace(/^export /u, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
const classify = new Function(`${javascript}\nreturn trustedBootstrapLinkedUnownedKindsV1;`)();
const fixture = JSON.parse(readFileSync(`${root}/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/⛓️linked-codec-ownership/🔣️.json`, "utf8"));
let failures = 0;
for (const row of fixture.cases) {
  let outcome;
  try {
    outcome = { unowned: [...classify(row.declared, row.linked)].sort() };
  } catch (error) {
    outcome = { refusal: String(error.message) };
  }
  const ok = row.refusal ? outcome.refusal?.includes(row.refusal) : JSON.stringify(outcome.unowned) === JSON.stringify([...row.unowned].sort());
  console.log(`${ok ? "ok  " : "FAIL"} ${row.id} ${JSON.stringify(outcome)}`);
  failures += ok ? 0 : 1;
}
const registry = JSON.parse(readFileSync(`${root}/✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json`, "utf8"));
const owned = new Set(registry.receipts.map((row) => `${row.artifact_kind} ${row.artifact_schema}`));
const editors = readdirSync(dumps).filter((file) => file.endsWith(".json")).flatMap((file) => JSON.parse(readFileSync(`${dumps}/${file}`, "utf8")).manifest.apps.filter((app) => app.role === "editor"));
const unowned = editors.filter((app) => !owned.has(`${app.dialect.artifactKind} ${app.io.artifactSchema}`)).map((app) => `${app.dialect.artifactKind} ${app.io.artifactSchema}`);
const coordinates = editors.map((app) => `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`);
console.log(`editors ${editors.length}, registry rows ${registry.receipts.length}, unowned editor rows ${unowned.length} ${JSON.stringify([...new Set(unowned)])}, duplicate coordinates ${coordinates.length - new Set(coordinates).size}`);
failures += unowned.length + coordinates.length - new Set(coordinates).size;
console.log(`TS-CHECK failures=${failures}`);
process.exit(failures ? 1 : 0);
