/** Z2 runner: the cache-contract laws that parse or execute the Nx bootstrap wrapper, run one by one. */
const C = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests";
const workspace = "/Users/ueli/Documents/semio", output = process.argv[2]!;
const laws: [string, () => Promise<unknown>][] = [
  ["testNxCoordinator", async () => (await import(`${C}/🛑️coordinator/🟦️.ts`)).testNxCoordinator(workspace)],
  ["testWatcherReadiness", async () => (await import(`${C}/🕸️daemon/🟦️.ts`)).testWatcherReadiness(workspace)],
  ["testWorkspaceWatchIgnores", async () => (await import(`${C}/🕸️daemon/🟦️.ts`)).testWorkspaceWatchIgnores(workspace, output)],
  ["testDependencyBootstrap", async () => (await import(`${C}/📦️dependencies/🟦️.ts`)).testDependencyBootstrap(workspace, output)],
];
for (const [name, run] of laws) {
  const started = Date.now();
  try { await run(); console.log(`PASS ${name} ${Date.now() - started}ms`); }
  catch (error) { console.log(`FAIL ${name} ${Date.now() - started}ms :: ${String(error instanceof Error ? error.stack ?? error.message : error).slice(0, 1500)}`); }
}
