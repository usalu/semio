// 🧪️ DEMO 8 — script.ts's DependencyScript (bun ./📜️script.ts dependency) calls, verbatim:
//     const sorted = loadClassifiedBaseline(this.repoRoot);
//     const verdict = ratchetDependencies(sorted, sorted, registry);
// i.e. it ratchets the committed 🔒️dependencies.json against ITSELF. `goProductionClosure`,
// `pythonRuntimeImports` and `dotnetPackageReferences` — the exact functions index.ts provides to
// derive what a language's production code ACTUALLY imports today — are imported into script.ts
// (lines 38, 81) and never called anywhere. No candidate is ever computed from the live source tree.
//
// Consequence: ratchetDependencies structurally CANNOT observe a new production-reachable dependency,
// because baselineKeys and candidateKeys are built from the identical array. This demo proves the
// function itself works correctly (it DOES catch a real new production dependency when given a
// genuinely different candidate) — the vacuous pass is entirely in how script.ts calls it.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const registry: T.OracleRegistry = { schemaVersion: 2, oracles: [], probes: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], fixtureManifests: [], contributions: [] };

const committedBaseline: T.ClassifiedDependency[] = [{ ecosystem: "js", name: "left-pad", version: "1.0.0", kinds: ["test-runner"], users: ["x"], productionReachable: false }];

// What script.ts ACTUALLY does — call with the SAME array as both baseline and candidate:
const verdictAsWritten = T.ratchetDependencies(committedBaseline, committedBaseline, registry);
console.log(`ratchetDependencies(sorted, sorted, registry) — as script.ts calls it: ok=${verdictAsWritten.ok}, newProduction=[${verdictAsWritten.newProduction.join(", ")}]`);

// What a REAL candidate would look like if script.ts actually called goProductionClosure /
// pythonRuntimeImports / dotnetPackageReferences (imported, never invoked) to see what production
// code imports TODAY — say, someone just added a brand-new production-reachable npm package:
const realCandidateIfItWereComputed: T.ClassifiedDependency[] = [...committedBaseline, { ecosystem: "js", name: "sneaky-eval-based-plugin-loader", version: "9.9.9", kinds: ["production-runtime"], users: ["🧰️framework/some/production/file.ts"], productionReachable: true }];
const verdictIfWired = T.ratchetDependencies(committedBaseline, realCandidateIfItWereComputed, registry);
console.log(`\nratchetDependencies(sorted, REAL-candidate, registry) — what SHOULD run: ok=${verdictIfWired.ok}, newProduction=[${verdictIfWired.newProduction.join(", ")}]`);

console.log(
  verdictAsWritten.ok && !verdictIfWired.ok
    ? "\n>>> CONFIRMED: the ratchet function itself is sound — it correctly blocks a brand-new production-reachable dependency when given a real candidate. But `bun ./📜️script.ts dependency` never builds that candidate: `goProductionClosure`, `pythonRuntimeImports` and `dotnetPackageReferences` are imported into script.ts and never called. DependencyScript.run() literally passes the same `sorted` array as both `baseline` and `candidate`, so `newProduction` and `unregisteredTestDeps` are provably always empty — the command can never fail on a new production dependency no matter what a developer adds to production source, regardless of language."
    : "\n>>> could not reproduce",
);
