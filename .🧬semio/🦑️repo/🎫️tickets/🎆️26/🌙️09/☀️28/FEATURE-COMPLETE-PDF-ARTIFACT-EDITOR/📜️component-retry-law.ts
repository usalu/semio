/** 🧪 Runs the staged two-component retry through the repository test owner. */
import {mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const workspace = process.cwd();
const root = join(import.meta.dirname, "🗑️generated", "component-retry");
const processRoot = resolve("🧰️framework/🔨️modules/🏃️process");
const component = join(processRoot, "📦️artifacts/🏗️native-build/🧩️component-pass");
const sourcePath = join(root, "driver.ts");
const retryPath = join(root, "failure-churn.ts");
const fixturePath = join(root, "fixtures.json");
writeFileSync(fixturePath,readFileSync(join(root,"0.json")));
const testName = "the held native lane retries a stable locked dependency change for two components";
const rewrite = (file: string, input: string, output: string, overrides: Record<string, string>) => {
  const source = readFileSync(join(root, input), "utf8");
  writeFileSync(join(root, output), source.replace(/(from\s*")((?:\.{1,2}\/)[^"]+)(")/g,
    (_, before, path, after) => before + (overrides[path] ?? resolve(dirname(file), path)) + after));
};
rewrite(join(component, "🟦️.ts"), "3.ts", "driver.ts", {"./🔁️failure-churn/🟦️.ts":retryPath});
rewrite(join(component, "🧪️tests/🟦️.ts"), "4.ts", "driver-test.ts", {
  "../🟦️.ts":sourcePath,
  "../🔁️failure-churn/🧪️tests/🔣️.json":fixturePath,
  "../🔁️failure-churn/🟦️.ts":retryPath,
});
const runner = join(processRoot, "🧪️testing/🗂️suites");
const [{runProcessTestSuiteV1},{readProcessTestTableV1},{createScriptProcessEnvelope,withScriptProcessEnvelope}] = await Promise.all([
  import(join(runner, "🏃️run/🟦️.ts")),
  import(join(runner, "🟦️.ts")),
  import(join(processRoot,"📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),
]);
const table = readProcessTestTableV1();
const authored = table.suites.find((row: {id: string})=>row.id==="native-artifacts");
const cut = {...authored,files:["driver-test.ts"],testNamePattern:testName};
const output = join(dirname(root), "h");
mkdirSync(output,{recursive:true});
await withScriptProcessEnvelope(createScriptProcessEnvelope({
  version:1,owner:"pdf:component-retry-law",maximumElapsedMilliseconds:authored.budgetMs,
}, {}, Date.now()), invocation=>runProcessTestSuiteV1({
  invocation,root,checkout:workspace,table:{...table,suites:[cut]},suite:authored.id,owner:"process",output,environment:{...process.env,RUSTC:process.argv[2],RUSTC_WRAPPER:"",RUSTC_WORKSPACE_WRAPPER:""},
}));
