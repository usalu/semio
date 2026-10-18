// 🧪 c5a: runs the exported source-text oracles that read the cut plugin sources (no bun:test runner involved) and prints their result or error.
const R = "/Users/ueli/Documents/semio";
const PL = `${R}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`;
const run = async (name: string, fn: () => unknown | Promise<unknown>) => {
  try { console.log("OK  ", name, await fn()); } catch (error) { console.log("FAIL", name, String((error as Error).message ?? error).slice(0, 300)); }
};
const ro = await import(`${PL}/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts`);
await run("guestLifecycleOracle", () => ro.guestLifecycleOracle());
await run("issuedPatchOracle", () => ro.issuedPatchOracle());
await run("coldDocumentPairIngressOracle", () => ro.coldDocumentPairIngressOracle(R));
await run("documentBackboneBindingOracle", () => ro.documentBackboneBindingOracle(R));
const tt = await import(`${PL}/🧪️tests/🧪️time-travel/🟦️.ts`);
await run("timeTravelScenarioOracle", () => tt.timeTravelScenarioOracle(R));
const lw = await import(`${PL}/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts`);
await run("toolJobLatestWinsSelfTests", () => lw.toolJobLatestWinsSelfTests());
const inv = await import(`${PL}/🧩️extension/🧪️tests/📨️invoke/🟦️.ts`);
await run("extensionInvokeCallerOracle", () => inv.extensionInvokeCallerOracle(R));
