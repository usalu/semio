# Broad Test Suite Verification

The actual Nx repository-test long suite executed353tests:338passed,15failed,7946assertions,297seconds. This is not reported as an overall pass. Failures include live unrelated mutation vocabulary/oracle coverage/dependency debt and concurrent repository topology changes, alongside obsolete fixture-schema vectors and stale schema catalogs addressed by this refactor. The exact failures were reviewed from the execution log. Native jobs remain pending shared preparation and no native pass is inferred.

```text
pillow is linked by oracle lowpoly-io-png-pillow but is absent from the dependency baseline
```

```text
js:ignore is on a generated host's import path but is absent from the dependency baseline
```

```text
Expected: false
Received: true
```

```text
Test discovery requires real workspace ancestry: /var
      at workspaceAdmission (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:56:137)
      at taxonomy (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:46:21)
      at discoverCaseDirs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:362:22)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1515:14)
✗ 🧪️ projected vector storage > profile storage never becomes an executable Nx test project [14.32ms]
✓ 🔬️ subject selection > an oracle implemented in the case's own adapter is never also dispatched as that case's subject [126
```

```text
Expected: true
Received: false
```

```text
Expected: "schema-fixture-defines-schema:🌎️hub/💡️inference/🧫️fixtures/🧫️approval/🧬️.schema.json"
Received: "schema-fixture-defines-schema:🌎️hub/💡️inference/🧫️fixtures/🧫️approval/🧬️.schema.json,🌎️hub/💡️inference/🧫️fixtures/🧫️approval/🧬️schema/🔣️.json"
```

```text
Nested Cargo catalog digest drift
      at semanticPackageProjectionCatalog (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5448:144)
      at projectNestedCargoPackages (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:4266:19)
      at inventoryTaxonomyWithSourceParentPruning (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:4823:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts:47:21)
✗ HTML source pair normalization accepts the primary leaf of a multi-extension kind [14831.91ms]
✓ HTML source pair manifests retain exact native-reader and parser inputs [1009.20ms]
✓ HTML source pair readers declare every exter
```

## Final Broad Repository Suite

Actual existing repository long-suite task rerun through isolated Nx: 353 tests, 341 passed, 12 failed, 8,000 assertions, 734.32 seconds. This is not an overall pass. Exact observed failure summaries follow; fixture-boundary-related failures are under review and no unrelated baseline attribution is made.


## Observed Final Failure Details

### (fail) 🔍️ discovery and contract > discovery is idempotent [68507.31ms]

```text
(pass) ⚖️ comparison profiles > floating-point-v1 tolerates representation noise that ordered-json-v1 does not [0.06ms]
```

### (fail) 🔍️ discovery and contract > every committed case satisfies the frozen contract's blocking rules [21389.21ms]

```text
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:296:188)
```

### (fail) 🔒️ dependency ratchet > the committed baseline classifies every ecosystem it tracks and keeps oracles out of production [12066.03ms]

```text
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:296:188)
381 |     for (const oracle of repoRegistry.oracles) {
382 |       for (const linked of oracleLinkedPackages(oracle)) {
383 |         const entry = baseline.entries.find((candidate) => candidate.name === linked.package);
384 |         // 🔒️A package present in the baseline at all is the invariant; being absent is what makes the
385 |         // gate blind to its own subject.
386 |         expect(entry, `${linked.package} is linked by oracle ${oracle.id} but is absent from the dependency baseline`).toBeDefined();
error: pillow is linked by oracle lowpoly-io-png-pillow but is absent from the dependency baseline
Received: undefined
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:386:120)
```

### (fail) 📈️ non-aggregate metrics > oracle coverage counts every discovered case as backed by an oracle or a recorded decision [1454.13ms]

```text
470 |   });
471 | 
472 |   test("oracle coverage counts every discovered case as backed by an oracle or a recorded decision", () => {
473 |     const metrics = computeCoverageMetrics(repoRoot, cases, [], [], []);
474 |     expect(metrics.oracleCoverage.unbacked).toEqual([]);
error: expect(received).toEqual(expected)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:474:45)
```

### (fail) 🌱️ native second implementation > every registered verified-native-second-implementation entry in the live registry is earned [8.40ms]

```text
+     "solution": "Point format at the artifact id this same contribution's own mutationManifests entry declares.",
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:716:130)
```

### (fail) 🧫️ mutation without fixture > the live registry retains the independent Stdio declaration census and has no declared fixture debt [13527.60ms]

```text
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:888:51)
```

### (fail) 🧩️ cross-language oracle hosts > the committed baseline classifies every external host package as a test-only dependency [3.04ms]

```text
1026 | 
1027 |   test("the committed baseline classifies every external host package as a test-only dependency", () => {
1028 |     const baseline = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { ecosystem: string; name: string; kinds: string[]; productionReachable: boolean }[] };
1029 |     for (const host of externalOracleHostPackages(repoRegistry)) {
1030 |       const entry = baseline.entries.find((candidate) => candidate.ecosystem === host.ecosystem && candidate.name === host.name);
1031 |       expect(entry, `${host.ecosystem}:${host.name} is on a generated host's import path but is absent from the dependency baseline`).toBeDefined();
error: js:ignore is on a generated host's import path but is absent from the dependency baseline
Received: undefined
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1031:135)
```

### (fail) 🔒️ recorded production debt > an oracle claiming testOnly while already production-reachable must record the debt, not hide it [1.93ms]

```text
error: js:ignore is on a generated host's import path but is absent from the dependency baseline
Received: undefined
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1031:135)
1041 |     const registry = repoRegistry;
1042 |     const baseline = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { name: string; productionReachable: boolean }[] };
1043 |     for (const oracle of registry.oracles) {
1044 |       const entry = baseline.entries.find((candidate) => candidate.name === oracle.package);
1045 |       if (entry?.productionReachable !== true) continue;
1046 |       expect(oracle.productionDebt, `${oracle.package} is production-reachable but records no debt`).toBeDefined();
error: xstate is production-reachable but records no debt
Received: undefined
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1046:102)
```

### (fail) 🧩️ open/closed > the root script names neither the test module's location nor its phase vocabulary [2.92ms]

```text
(pass) 🧭️ contribution directory ownership > the handpicked kernel oracle remains discoverable at runtime [1.03ms]
(pass) 🧩️ open/closed > the framework Rust host declares no dependency at all [0.36ms]
1325 |     }
1326 |   });
1327 | 
1328 |   test("the root script names neither the test module's location nor its phase vocabulary", () => {
1329 |     const root = readFileSync(join(repoRoot, "📜️script.ts"), "utf8");
1330 |     expect(root.includes(testTaxonomy(repoRoot).testDomainPath)).toBe(false);
error: expect(received).toBe(expected)
Expected: false
Received: true
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1330:66)
```

### (fail) 🧪️ projected vector storage > profile storage never becomes an executable Nx test project [16.36ms]

```text
51 | /** 🛡️ Admits fresh workspace ancestry and relative real-file candidates without normalization. */
52 | function workspaceAdmission(input) {
53 |   if (typeof input !== "string" || !input || input.includes("\0") || input.split(/[\\/]/u).some(part => part === "." || part === "..")) throw new Error("Test discovery requires an unnormalized real workspace root");
54 |   const root = resolve(input), ancestry = [];
55 |   for (let current = root; ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
56 |   for (const path of ancestry.reverse()) { const value = lstatSync(path); if (!value.isDirectory() || value.isSymbolicLink()) throw new Error(`Test discovery requires real workspace ancestry: ${path}`); }
error: Test discovery requires real workspace ancestry: /var
      at workspaceAdmission (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:56:137)
      at taxonomy (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:46:21)
      at discoverCaseDirs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs:362:22)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts:1515:14)
```

### (fail) 🩺️ the committed tree is measured by the same checkers > the test platform's own subtree carries no schema-contract finding [2151.83ms]

```text
811 |   });
812 | 
813 |   // 🧭️Scoped to the module this worker owns. The rest of the tree is mid-migration under other
814 |   // partitions, and asserting over it here would report their work as this module's breakage.
815 |   test("the test platform's own subtree carries no schema-contract finding", () => {
816 |     expect(schemaContractDiagnostics(repoRoot, TEST_DOMAIN_REL_PATH).map((entry) => `${entry.code} ${entry.path ?? ""}`)).toEqual([]);
error: expect(received).toEqual(expected)
+   "schema-dialect-not-draft-07 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧬️schema/🔣️.json",
+   "schema-catalog-stale 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧬️schema",
+   "schema-catalog-stale 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers/🧬️schema",
+   "schema-catalog-stale 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema",
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts:816:123)
```

### (fail) HTML source pair normalization accepts the primary leaf of a multi-extension kind [6393.38ms]

```text
5476 |   const contract = taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
5477 |   const state = exactOwnerRegularFile(repoRoot, contract.authorityCatalogPath);
5478 |   if (state === "absent") return null;
5479 |   if (state !== "file") throw new Error("Nested Cargo catalog must be a no-follow regular file");
5480 |   const bytes = readFileSync(join(repoRoot, contract.authorityCatalogPath));
5481 |   if (createHash("sha256").update(bytes).digest("hex") !== contract.authorityCatalogSha256) throw new Error("Nested Cargo catalog digest drift");
error: Nested Cargo catalog digest drift
      at semanticPackageProjectionCatalog (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5481:144)
      at projectNestedCargoPackages (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:4266:19)
      at inventoryTaxonomyWithSourceParentPruning (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:4823:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts:47:21)
```

