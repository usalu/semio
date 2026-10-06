# Global Fixture And Asset Audit

## Scope And Method

This read-only audit inspected the current working tree across `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand`, and root-level content. The latest full-path census ran at 2026-09-09T10:15:05+02:00 in the live shared working tree. It excluded dependencies, Git internals, ticket history, and ticket generated output. It counted both tracked and present working-tree paths. The latest values are 4,567 fixture-path files, 57 fixture files below test-case paths, 12,747 non-code test-case files, and 51 direct test-root non-code files. An earlier supplementary untracked pass, excluding standard output/dependency roots, found 444 paths, including 392 beneath fixture directories and 4 non-code paths below test-case trees. A permanent check must therefore include present, non-output untracked files as well as tracked paths. Concurrent edits explain the difference from earlier working-tree observations; neither is a timeless repository baseline.

The full numeric census and confirmed source edges are in `🗑️generated/global-audit/fixture-census.md` and `🗑️generated/global-audit/uri-and-production-edge-census.md`.

## Findings

### 1. Test cases still own fixtures and other test data

57 files in fixture-named directories sit below `🧪️tests` paths. This is directly contrary to the target ownership model. The owners need a shared language-neutral `🧫️fixtures` subtree outside test case directories, keyed by the smallest semantic owner and case/vector identity.

The wider census finds 12,747 non-code files in test-case subtrees. It is a physical migration census, not a finding that every member is a fixture: the population includes candidate JSON inputs, expected outputs, snapshots, diffs, schemas, contracts, and possible build metadata. Every member needs a semantic owner and consumer classification before relocation. Example input/output needs shared fixture ownership; schemas, contracts, and build metadata belong with the owner that interprets or executes them. Their physical location still makes all non-metadata sample content case-owned and subject to the migration rule, even when its directory is named `📸️snapshot`, `🦠️mutation`, `🎯️outcome`, or `🛂️schema` rather than fixtures.

51 non-code files sit directly in a test root. Some can be metadata that defines the test root itself. This needs a schema/semantic-kind allowlist. Treating every non-code file directly under `🧪️tests` as metadata would create an escape hatch for fixtures.

### 2. The test platform currently permits the forbidden layout

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` discovers `caseDir/🧫️fixtures`, emits `localFixtureDir`, resolves `local://`, and checks that local fixture tree for orphans. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` also hashes case-local fixture paths as target inputs. Removing only the directory data leaves an invalid production path through discovery, plan construction, resolution, cache inputs, protocol fixtures, and cross-language runner APIs.

`local://` occurs in 213 files. Its only behavior-bearing source is this test platform; the other source-like occurrences are oracle descriptions. `asset://` occurs in 471 files and is principally a test declaration scheme. It must resolve through the owner asset root only, never the owner root generally and never fixture paths.

### 3. Production code has direct fixture dependencies

The confirmed compilation edges listed in the generated census use `include_str!` to bind files below fixture directories into runtime constants. These are production/build dependencies, not merely test helper names. Their data must be re-owned under `🖼️assets` if it is runtime static data. If it is only an expected test vector, the constants and consuming runtime API must be removed or moved to test-only code.

### 4. Fixture paths also hide executable build support

163 compiled-or-script-language files and 13 build or execution configuration files currently live below fixture directories. This path census does not itself prove that each source file is executed or imported. It does prove the path-name exemption is unsafe, and focused consumer inspection has established the following classes. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5401` hard-codes the JCO guest fixture tree as a semantic package owner, reads its Cargo manifest, lockfile, Rust, WIT, and adapter inputs, and feeds them to package-adapter logic. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:8` and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:3` import the Scale fixture's component-artifact declaration. Its subtree declares an Nx project and a Rust component package. These are confirmed production/build-source edges, while the JCO workspace-contract consumer is separately confirmed test-only.

The browser-bundle actor-import subtree contains a guest Cargo package and a project script. The OS store, SPR, and plugin mutation fixture trees contain Rust modules and tests. Replication, ordered value, neural retirement, and Flow fixture trees contain executable scripts. Their exact reverse consumers have not all been resolved in this audit, so they are classification candidates rather than claimed live dependencies. Resolve each through imports, package/build inputs, and invocation graphs before migration. They need a test-support, generator, schema, or product/build semantic owner according to that consumer; a fixture path is valid only for finite example input/output content.

Source code can remain as fixture payload only when an explicit fixture manifest classifies it as opaque input/output text and a reverse dependency closure proves it has only test readers. A folder name or a source-file extension cannot establish that exception.

### 5. Mislabelled data needs semantic classification before migration

Some files labelled fixture are actual executable support or contracts: fixture directories can contain `📜️script.ts`, tests, schemas, or generators. They are not fixtures merely because of their parent name. Move examples and expected outputs to the semantic owner fixture tree; retain executable generators, schemas, and production static artifacts in their respective semantic owners. Do not move a schema or script under fixture ownership solely to eliminate a path match.

## Exact Backlog Recommendation

1. In the repository test platform, delete `localFixtureDir` and the `local` URI scheme from discovery, schemas, plans, TypeScript/Python/Go/Rust runner contexts, orphan checks, and Nx input derivation. Make the owner `🧫️fixtures` subtree the only testing-data file source. Make `asset://` resolve only at `<owner>/🖼️assets` with traversal protection and a plan-time digest.
2. Add a schema-first fixture manifest that declares each fixture's semantic owner, input/output role, test consumers, and source class. The permanent gate must reject fixture paths beneath any test case, fixture entries with no test consumer, executable/script/schema members in fixture trees, and `asset://` targets outside the owner asset subtree. A source-language sample needs an explicit opaque-payload declaration, not an extension exemption.
3. Add an independent production-edge check over production, package, generator, build, and runtime sources. Reject `include_str!`, `include_bytes!`, imports, runtime filesystem reads, copy operations, and build/config globs whose resolved path reaches a fixture directory. Examine Nx project roots/inputs, Cargo manifests, generator inputs, and dynamically resolved paths as well as source imports. Exclude declared test runner sources only after establishing their test-only execution boundary.
4. Classify and migrate the 12,747 test-case non-code files in semantic-owner batches. Preserve test-root metadata only when it has an approved metadata kind; migrate its sample content otherwise. Recompute all adapter includes, feature URIs, runner plans, cache inputs, and fixture manifests.
5. Verify with (a) static path and dependency closure audits over tracked plus non-output untracked files, (b) test-plan resolution proving no `local://` / case fixture directory can resolve, (c) production builds that compile formerly fixture-backed modules from assets, (d) negative checks showing JCO/Scale-style package, project, Cargo, and script trees cannot reside under fixtures, and (e) one actual runner per language that reads a shared fixture through the new plan.

## Verification Gaps

This report did not run the full repository test/build matrix because it is a design audit, and concurrent edits can change the live shared tree during a census. A stable checkout was neither created nor required. Post-migration verification must record timestamped full raw-working-tree scans and the affected-consumer results under this ticket's generated directory, then clean generated output before ticket closure. It must inspect indirect dependencies introduced through generated project manifests and Nx cache inputs; a simple source-text search cannot prove those edges absent.
