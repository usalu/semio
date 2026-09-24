import * as t from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const time = <T>(label: string, run: () => T): T => { const s = performance.now(); const v = run(); console.log(label, Math.round(performance.now() - s)); return v; };
const registry = time("registry", () => t.loadOracleRegistry(root));
const cases = time("discover", () => t.discoverTestCases(root));
time("cases", () => cases.flatMap((c) => t.validateCaseContract(root, c, registry)));
time("oraclePurity", () => t.oracleImportsInProduction(root));
for (const name of ["mutationInventoryBreaches", "binaryProtocolDriftBreaches", "stubSerializerBreaches", "stubDeserializerBreaches", "reimplementationOracleBreaches", "fixtureProvenanceBreaches"]) time(name, () => (t as any)[name](root, registry));
for (const name of ["capabilityManifestBreaches", "nativeSecondImplementationBreaches", "registryRecordBreaches", "noOracleMisuseBreaches", "mutationFixtureBreaches", "isolationBreaches"]) time(name, () => (t as any)[name](registry));
