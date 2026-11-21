import { dirname, join } from "node:path";
const checkout = process.cwd();
const library = join(checkout, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const processRoot = join(checkout, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript");
const { Script, ScriptRouter } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(library, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));
const { readCachingTestTableV1, cachingTestRootV1 } = await import(join(library, "⚡️caching/🗂️suites/🟦️.ts"));
const { runProcessTestSuiteV1 } = await import(join(processRoot, "🧪️testing/🗂️suites/🏃️run/🟦️.ts"));

class PruneContentionScript extends Script {
  async run(): Promise<void> {
    const authored = readCachingTestTableV1();
    const suite = authored.suites.find((row: { id: string }) => row.id === "pruning");
    if (!suite) throw Error("Missing declared pruning suite");
    const test = "🧹️pruning/🔒️store/🧪️tests/🟦️.ts";
    const table = { ...authored, suites: [{ ...suite, files: [test], testNamePattern: "opportunistic compiler recovery" }] };
    await runProcessTestSuiteV1({ invocation: this.invocation, root: cachingTestRootV1, checkout, table, suite: suite.id,
      owner: "cache", output: join(dirname(process.argv[1]!), "🗑️generated"), environment: process.env });
  }
}
const args = process.argv.slice(2);
const router = new ScriptRouter(checkout, checkout).register("test", PruneContentionScript);
await runWorkspaceScriptMain(router, { freshCapabilities: async () => {
  issueRepositoryScriptProvidersV1(process.env);
  return createBuildBootstrapCapabilitiesV1(checkout, args, process.env);
} });
