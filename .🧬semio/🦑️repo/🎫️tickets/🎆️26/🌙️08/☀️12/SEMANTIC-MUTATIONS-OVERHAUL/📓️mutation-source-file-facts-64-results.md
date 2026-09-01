# 🧪️ Mutation Source File Facts 64 Results

## Authored Canonical Test Surface

The approved neutral files are now present under the canonical L owner:

- [`🟦️.ts`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts), SHA-256 `741bba309f3579e9a41f0b7fd4dffa55511e09e30ad1966a3ea3f18a9603610a`
- [`🧬️schema/🔣️.json`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🧬️schema/🔣️.json), SHA-256 `a7d62a7c289cf8eff26656f72aa4cac086fae8750c665245113bb3ed036c73c7`
- [`🔣️.json`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🔣️.json), SHA-256 `5672b8c0270a3a8a8c118f2c6e3ce177b8f966ef8dbf55eed5530326065f03ba`

The closed Draft 2020-12 fixture contains 40 cases: all 26 currently registered source extension chains, seven actual representative non-source roles, unknown `.jsx`, an executable regular source, synthetic generated and equal-longest tie catalogs, nonregular/absent/gitlink exclusions, and a rejected virtual `CoMpOsE` admission. Raw NFD path spelling is retained. There is no claim that the current catalog owns a generated kind: the one generated row uses a separately labeled synthetic catalog.

The independent reference selects suffixes with installed transitive `minimatch`, while Ajv validates the fixture. Both are test-only transitive tooling and neither was installed or added to a manifest. The test fails explicitly if their imports cannot resolve. The subject is the future root pure `mutationTaxonomySourceFileFacts` export only; it imports no classifier into production and performs no filesystem access for virtual admissions.

## Executed Reference

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts' --test-name-pattern='vectors|independent suffix'
```

This executed after a retained test-side null-guard correction and exited `0`: 4 tests passed, 4 subject tests were filtered, with 44 assertions in 395 ms. It validates the closed fixture, all registered source-chain coverage, full role-vocabulary coverage through actual or explicitly synthetic rows, and independent longest-suffix projection including the ambiguity case.

## Actual Missing-Export RED

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts' --test-name-pattern='subject'
```

This executed against root [`📜️script.ts`](../../../../../../../📜️script.ts) SHA-256 `5eb9cbfff2f505be52eef456cb6c26a310622f0fabff291a5277306c47d779e4` and taxonomy SHA-256 `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce`. It exited nonzero after loading the actual root module and produced the intended error: `missing mutationTaxonomySourceFileFacts export` (three catalog subject cases failed; five tests were filtered). This is an API-absence RED, not an implementation behavior failure.

The future subject test includes a rejected admission whose `observations` getter throws. Once the export exists, it must reject on status before observing that getter, proving no accidental row access. The virtual `CoMpOsE` case is solely this in-memory rejected admission; no compose path was traversed or probed.

No root/P implementation, source-index expansion, manifest, launch configuration, Cargo, or filesystem census was changed.

## Corrected Admission Boundary And Current Receipts

The initial authoring receipts above remain historical. The current test no longer calls `loadTaxonomy()`: it parses the exact L `🔣️taxonomy.json` as a catalog-only input, so the supplied-observation projection does not invoke workspace generator authority. The current artifacts are test `013b4e13623ebef2a39d82e7fd86a7935ec9c2bfd21be5983d3f02ebf5449f90`, schema `7c35594804d05e0defc7759f203bdf5aacccd70e148b0baebed5c4518ffb1b3a`, vectors `a27830505bf1a7b55a2b8f21fab684fd14f23a288a373e2a0b6c7d8004d18d73`, root script `5eb9cbfff2f505be52eef456cb6c26a310622f0fabff291a5277306c47d779e4`, and taxonomy `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce`.

`documentation-nfd` now uses the JSON `\u0301` escape, which parses as the U+0301 combining character; the reference asserts `sourcePath !== sourcePath.normalize("NFC")` and exact raw-path output preservation. Every supplied observation is now a full typed tuple with `explicitDirectory`, origins, index entries, and generator outputs. The Gitlink row has physical `directory`/`040000` facts and a distinct index stage-zero `160000` entry. The modeled complete rows remain explicitly labeled supplied observations rather than collector results.

The `CoMpOsE` rejection is no longer fabricated. The test calls actual pure `projectTaxonomySourceAdmission` with an opaque-prefix candidate and asserts its rejected `opaque-path` diagnostic before converting that result to an inventory. Its subject test then poisons the projected `observations` getter; a future projector must reject the noncomplete admission before reading it. No physical Compose path is touched.

The strict TypeScript diagnostic run is intentionally not represented as whole-package readiness. Its real program has exactly seven environment diagnostics: one `TS2307` for the intentionally type-free `bun:test` host and six `TS2339` instances for Bun's `ImportMeta.dir`; the test asserts there are no other diagnostics in the exact test source. No type stubs were introduced.

The corrected independent-reference command and complete raw output are:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts' --test-name-pattern='^mutation source-file facts (vectors|independent suffix reference|reference source)'

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
(pass) mutation source-file facts vectors are closed and cover the registered source chains [33.08ms]
(pass) mutation source-file facts reference source has strict standalone types [1656.47ms]
(pass) mutation source-file facts independent suffix reference: current [17.45ms]
(pass) mutation source-file facts independent suffix reference: synthetic-generated [0.61ms]
(pass) mutation source-file facts independent suffix reference: synthetic-tie [0.50ms]

 5 pass
 4 filtered out
 0 fail
 48 expect() calls
Ran 5 tests across 1 file. [1.96s]
```

The corrected actual missing-export command and complete raw output are:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts' --test-name-pattern='^mutation source-file facts subject'

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
error: missing mutationTaxonomySourceFileFacts export
  at subject (🟦️.ts:93:50)
  at subject rejects projected opaque Compose before poisoned observations (🟦️.ts:131:27)
(fail) mutation source-file facts subject rejects projected opaque Compose before poisoned observations [60.13ms]
error: missing mutationTaxonomySourceFileFacts export
  at subject (🟦️.ts:93:50)
  at subject matches reference: current (🟦️.ts:138:128)
(fail) mutation source-file facts subject matches reference: current [1.83ms]
error: missing mutationTaxonomySourceFileFacts export
  at subject (🟦️.ts:93:50)
  at subject matches reference: synthetic-generated (🟦️.ts:138:128)
(fail) mutation source-file facts subject matches reference: synthetic-generated [1.62ms]
error: missing mutationTaxonomySourceFileFacts export
  at subject (🟦️.ts:93:50)
  at subject matches reference: synthetic-tie (🟦️.ts:138:128)
(fail) mutation source-file facts subject matches reference: synthetic-tie [1.68ms]

 0 pass
 5 filtered out
 4 fail
Ran 4 tests across 1 file. [235.00ms]
Nx exec propagated the child status 1.
```

This remains the intended API-absence RED. It has not exercised a root projection or claimed runtime behavior.

## Strict Oracle Correction

The prior seven-diagnostic filtered check was not a strict success and remains only historical evidence. It has been removed. The moved pure classifier and its owned types are now in canonical [`🧪️oracle/🟦️.ts`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🧪️oracle/🟦️.ts), SHA-256 `5f743f901d2c1245bdcc6e4548e46e11c30c50babd0a36e7429771504ba17309`. It has no Bun host or `ImportMeta.dir` dependency, uses the actual installed `minimatch` test-only import, and is checked by `strictSourceDiagnostics` with an exact `[]` assertion and no filtering, stubs, or suppressed categories. The current test SHA-256 is `98e236ebffef51934046b2b58915e0995926a8afc45cb56cbf61c60d7031ce0d`; vector/schema/root hashes remain `a27830505bf1a7b55a2b8f21fab684fd14f23a288a373e2a0b6c7d8004d18d73`, `7c35594804d05e0defc7759f203bdf5aacccd70e148b0baebed5c4518ffb1b3a`, and `5eb9cbfff2f505be52eef456cb6c26a310622f0fabff291a5277306c47d779e4`.

Corrected reference raw output:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts' --test-name-pattern='^mutation source-file facts (vectors|independent suffix reference|reference oracle)'

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
(pass) mutation source-file facts vectors are closed and cover the registered source chains [32.00ms]
(pass) mutation source-file facts reference oracle has strict standalone types [1159.53ms]
(pass) mutation source-file facts independent suffix reference: current [19.28ms]
(pass) mutation source-file facts independent suffix reference: synthetic-generated [0.63ms]
(pass) mutation source-file facts independent suffix reference: synthetic-tie [0.56ms]

 5 pass
 4 filtered out
 0 fail
 47 expect() calls
Ran 5 tests across 1 file. [1.56s]
```

Corrected missing-export RED raw output:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts' --test-name-pattern='^mutation source-file facts subject'

bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:
error: missing mutationTaxonomySourceFileFacts export
      at subject (🟦️.ts:63:50)
      at subject rejects projected opaque Compose before poisoned observations (🟦️.ts:99:27)
(fail) mutation source-file facts subject rejects projected opaque Compose before poisoned observations [66.91ms]
error: missing mutationTaxonomySourceFileFacts export
      at subject (🟦️.ts:63:50)
      at subject matches reference: current (🟦️.ts:106:128)
(fail) mutation source-file facts subject matches reference: current [2.11ms]
error: missing mutationTaxonomySourceFileFacts export
      at subject (🟦️.ts:63:50)
      at subject matches reference: synthetic-generated (🟦️.ts:106:128)
(fail) mutation source-file facts subject matches reference: synthetic-generated [1.94ms]
error: missing mutationTaxonomySourceFileFacts export
      at subject (🟦️.ts:63:50)
      at subject matches reference: synthetic-tie (🟦️.ts:106:128)
(fail) mutation source-file facts subject matches reference: synthetic-tie [1.84ms]

 0 pass
 5 filtered out
 4 fail
Ran 4 tests across 1 file. [275.00ms]

error: Command failed: "bun" "test" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts" "--test-name-pattern=^mutation source-file facts subject"
 status: 1
```

The retained detailed runner continuation above records the complete Node/Nx stack for the same status-one propagation. The RED continues to establish export absence only; it does not claim classifier behavior.

### Literal RED Runner Continuation

The following is the unabridged error body emitted between the RED command line and its summary above; it is retained separately so the missing-export result is not represented by a summary alone.

```text
88 | 
89 | /** 🧭️ Resolves the proposed root projector dynamically so absence is a real test failure. */
90 | async function subject(): Promise<(admission: TaxonomySourceInventory, taxonomy: Taxonomy) => readonly Omit<Expected, "extensionChain">[]> {
91 |   const module = await import(`${pathToFileURL(rootScriptPath).href}?source-file-facts=${createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex")}`);
92 |   const projector = Reflect.get(module, "mutationTaxonomySourceFileFacts");
93 |   if (typeof projector !== "function") throw new Error("missing mutationTaxonomySourceFileFacts export");
                                                      ^
error: missing mutationTaxonomySourceFileFacts export
      at subject (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:93:50)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:131:27)
(fail) mutation source-file facts subject rejects projected opaque Compose before poisoned observations [60.13ms]
88 | 
89 | /** 🧭️ Resolves the proposed root projector dynamically so absence is a real test failure. */
90 | async function subject(): Promise<(admission: TaxonomySourceInventory, taxonomy: Taxonomy) => readonly Omit<Expected, "extensionChain">[]> {
91 |   const module = await import(`${pathToFileURL(rootScriptPath).href}?source-file-facts=${createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex")}`);
92 |   const projector = Reflect.get(module, "mutationTaxonomySourceFileFacts");
93 |   if (typeof projector !== "function") throw new Error("missing mutationTaxonomySourceFileFacts export");
                                                      ^
error: missing mutationTaxonomySourceFileFacts export
      at subject (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:93:50)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:138:128)
(fail) mutation source-file facts subject matches reference: current [1.83ms]
88 | 
89 | /** 🧭️ Resolves the proposed root projector dynamically so absence is a real test failure. */
90 | async function subject(): Promise<(admission: TaxonomySourceInventory, taxonomy: Taxonomy) => readonly Omit<Expected, "extensionChain">[]> {
91 |   const module = await import(`${pathToFileURL(rootScriptPath).href}?source-file-facts=${createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex")}`);
92 |   const projector = Reflect.get(module, "mutationTaxonomySourceFileFacts");
93 |   if (typeof projector !== "function") throw new Error("missing mutationTaxonomySourceFileFacts export");
                                                      ^
error: missing mutationTaxonomySourceFileFacts export
      at subject (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:93:50)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:138:128)
(fail) mutation source-file facts subject matches reference: synthetic-generated [1.62ms]
88 | 
89 | /** 🧭️ Resolves the proposed root projector dynamically so absence is a real test failure. */
90 | async function subject(): Promise<(admission: TaxonomySourceInventory, taxonomy: Taxonomy) => readonly Omit<Expected, "extensionChain">[]> {
91 |   const module = await import(`${pathToFileURL(rootScriptPath).href}?source-file-facts=${createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex")}`);
92 |   const projector = Reflect.get(module, "mutationTaxonomySourceFileFacts");
93 |   if (typeof projector !== "function") throw new Error("missing mutationTaxonomySourceFileFacts export");
                                                      ^
error: missing mutationTaxonomySourceFileFacts export
      at subject (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:93:50)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts:138:128)
(fail) mutation source-file facts subject matches reference: synthetic-tie [1.68ms]

55 |     }
56 |     const projects = getProjects(projectGraph, nxArgs);
57 |     const projectsToRun = (0, get_command_projects_1.getCommandProjects)(projectGraph, projects, nxArgs);
58 |     projectsToRun.forEach((projectName) => {
59 |         const command = argv.reduce((cmd, arg) => cmd + `"${arg}" `, '').trim();
60 |         (0, child_process_1.execSync)(command, {
                                 ^
error: Command failed: "bun" "test" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts" "--test-name-pattern=^mutation source-file facts subject"
 signal: null,
 status: 1,
 output: [ null, null, null ],
    pid: 61659,
 stdout: null,
 stderr: null,

      at genericNodeError (node:child_process:998:13)
      at checkExecSyncError (node:child_process:458:27)
      at execSync (node:child_process:278:31)
      at <anonymous> (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:29)
      at forEach (1:11)
      at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
      at async <anonymous> (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:75)
```
