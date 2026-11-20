import {join} from "node:path";

const root = process.cwd();
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const routing = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const {createBuildBootstrapCapabilitiesV1} = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const {issueRepositoryScriptProvidersV1} = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));
const {CacheTestScript} = await import(join(library, "⚡️caching/📜️script.ts"));

issueRepositoryScriptProvidersV1(process.env);
const capabilities = await createBuildBootstrapCapabilitiesV1(root, ["tools", "setup", "all"], process.env);
const envelope = routing.createScriptProcessEnvelope({version: 1, owner: "repository:closure-suite", maximumElapsedMilliseconds: 300000}, capabilities, Date.now());
await routing.withScriptProcessEnvelope(envelope, async invocation => {
  const [filePart, testNamePattern] = process.argv.slice(2);
  const suite = "storage-graph-processes";
  if (!filePart) return new CacheTestScript(join(library, "⚡️caching"), root, invocation).run([suite]);

  const {cachingTestRootV1, readCachingTestTableV1} = await import(join(library, "⚡️caching/🗂️suites/🟦️.ts"));
  const {runProcessTestSuiteV1, withProcessTestOutputV1} = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🏃️run/🟦️.ts"));
  const table = readCachingTestTableV1();
  const declared = table.suites.find(row => row.id === suite)!;
  const files = declared.files.filter(file => file.includes(filePart));
  if (files.length !== 1) throw Error("Exactly one declared test file required");
  await withProcessTestOutputV1(root, "cache-contract", output => runProcessTestSuiteV1({
    invocation, root: cachingTestRootV1, checkout: root,
    table: {...table, suites: [{...declared, files, testNamePattern}]},
    suite, owner: "cache", output, environment: process.env,
  }), declared.scratch);
});
