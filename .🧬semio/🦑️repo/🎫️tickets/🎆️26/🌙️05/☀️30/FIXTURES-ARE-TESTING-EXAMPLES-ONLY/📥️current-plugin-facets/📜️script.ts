import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import ts from "typescript";

const ticket = dirname(import.meta.dir);
let root = ticket;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const inputs = readdirSync(ticket).filter(name => /^📥️current-plugin-oct8-(?:corpus-facets|extended-.*)-before\.json$/u.test(name)).sort();
assert(inputs.length > 0);
const expected = new Map<string, string>(), absent = new Set<string>();
for (const name of inputs) {
  const input = JSON.parse(readFileSync(join(ticket, name), "utf8"));
  for (const row of input.sources) expected.set(row.path, row.after);
  for (const row of input.documents) absent.add(row.path);
}

if (process.argv[2] === "source-integrity") {
  for (const [path, after] of expected) {
    const source = readFileSync(join(root, path), "utf8");
    assert.equal(source, after, path);
    if (path.endsWith(".json")) JSON.parse(source);
    else assert.equal((ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true) as any).parseDiagnostics.length, 0, path);
  }
  for (const path of absent) assert(!existsSync(join(root, path)), path);
  console.log("[DEBUG] Exact authored current plugin facet sources", expected.size, "retired corpus documents", absent.size);
} else if (process.argv[2] === "tests") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const paths = [...expected.keys()].filter(path => path.endsWith(".ts") && path.includes("🧪️tests"));
  assert(paths.length > 0);
  const catalog = paths.filter(path => path.includes("✅️catalog-complete")), local = paths.filter(path => !catalog.includes(path));
  await runBudgetedTestCommand(process.execPath, ["test", ...local.map(path => join(root, path))], { cwd: root, env: process.env, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", ...catalog.map(path => join(root, path)), "-t", "validates the neutral contract and withholds every publication|admits only schema-owned module routes"], { cwd: root, env: process.env, budgetMs: 120_000, throwOnFailure: true });
  const { documentBackboneBindingOracle } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts"));
  const { testRetainedWindowInputOracle } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🟦️.ts"));
  const bindings = documentBackboneBindingOracle(root);
  assert(bindings > 0);
  testRetainedWindowInputOracle();
  const { compositionLawGroups, testCompositionOwnership } = await import(join(root, "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts"));
  assert(compositionLawGroups().length > 0);
  testCompositionOwnership();
  console.log("[DEBUG] Actual current plugin facet Bun test owners", paths.length, "binding oracle cases", bindings, "retained window helper", 1, "composition ownership helper", 1);
} else if (process.argv[2] === "catalog-tests") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  if (process.argv.includes("--injected-identity-only")) {
    await runBudgetedTestCommand(process.execPath, ["test", join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧩️composition/🟦️.ts"), join(root,"✏️s/🧑‍💻dev/🚀️entry/🧪️tests/🟦️.ts"), "-t", "canonical injected|canonical S entry|injected catalog portable laws"], {cwd:root,env:process.env,budgetMs:120_000,throwOnFailure:true});
    const {inventorySchemaScopes}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts")),inventory=inventorySchemaScopes(root),scopes=["os.plugin.registry","s.dev.entry"].map(id=>{const scope=inventory.catalog.scopes[id],diagnostics=inventory.diagnostics.filter((row:any)=>row.path===scope.path||row.path.startsWith(scope.path+"/"));assert.deepEqual(diagnostics,[],id);assert.equal(Object.keys(scope.exports).length,id==="os.plugin.registry"?7:1);if(id==="s.dev.entry")assert(scope.dependsOn.includes("os.plugin.registry"));return{id,scope,diagnostics};});writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"current-owned-inventory.json"),JSON.stringify(scopes,null,2)+"\n");console.log("[DEBUG] current canonical injected catalog scope exports7 and S entry exports1; owned diagnostics0; genuine registry dependency admitted");
    const catalogPath="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json",before=JSON.parse(readFileSync(join(ticket,"📥️oct8-lease-corpus-and-injected-identity-before.json"),"utf8")),original=JSON.parse(before.rows.find((row:any)=>row.path===catalogPath).text),current=JSON.parse(readFileSync(join(root,catalogPath),"utf8")),projection=structuredClone(current),names:Record<string,string>={PluginCatalogIdV1:"id",PluginCatalogTextV1:"string",PluginCatalogTextsV1:"strings",PluginCatalogTargetV1:"target",PluginCatalogHostV1:"host",PluginCatalogPlaygroundV1:"playground"};
    delete projection.$id;projection.title=original.title;projection.$defs=Object.fromEntries(Object.entries(projection.$defs).map(([key,value])=>[names[key],value]));
    const normalize=(value:any):void=>{if(Array.isArray(value)){value.forEach(normalize);return;}if(value&&typeof value==="object")for(const [key,child]of Object.entries(value)){if(key==="$ref"&&typeof child==="string"&&child.startsWith("#/$defs/"))value[key]="#/$defs/"+names[child.slice(8)];else normalize(child);}};normalize(projection);assert.deepEqual(projection,original,"canonical identity changed payload policy");
    const {default:Ajv}=await import("ajv"),prior=new Ajv({strict:false}).compile(original),canonical=new Ajv({strict:false}).compile(current),fixture=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/🧩️composition/🔣️.json"),"utf8")),verdicts=fixture.cases.map((row:any)=>{const before=!!prior(row.input),after=!!canonical(row.input);assert.equal(before,after,row.id);assert.equal(after,row.valid,row.id);return{id:row.id,before,after};});
    writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"injected-policy-oracle.json"),JSON.stringify({projectionExact:true,verdicts},null,2)+"\n");console.log("[DEBUG] injected identity original/current exact policy projection and independent Ajv verdicts="+verdicts.length);
    const {runPhysicalLeaseChecksV1}=await import(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🧾️publication/🧪️tests/🔐lease/🧪️tests/🟦️.ts"));await runPhysicalLeaseChecksV1(root);
  } else {
    const paths = [join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧪️tests/🟦️.ts")];
    if (!process.argv.includes("--descriptor-only")) paths.push(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts"));
    await runBudgetedTestCommand(process.execPath, ["test", ...paths], { cwd: root, env: process.env, budgetMs: 120_000, throwOnFailure: true });
  }
} else throw Error("Expected source-integrity, tests or catalog-tests");
