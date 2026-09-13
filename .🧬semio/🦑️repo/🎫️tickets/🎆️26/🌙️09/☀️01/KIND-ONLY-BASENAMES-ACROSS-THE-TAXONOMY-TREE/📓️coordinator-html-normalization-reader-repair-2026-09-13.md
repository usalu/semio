# HTML Primary-Leaf Normalization Reader Repair

This repair is complete following the accepted HTML source-pair relocation. It corrects a missed normalization reader; the earlier native fixture and discovery checks did not exercise this reader. The repository-wide goal remains active.

The coordinator reran all 121 package-boundary tests with private temporary and artifact roots. The result was 117 passing and four failing tests, 508 assertions, 27.10 seconds. Three failures were caused by `normalization/🟦️.ts` rejecting the valid `stdio-html-source-pair-v1` contract because the HTML kind owns both `.html` and `.htm`. The discovery renderer already selects the schema-ordered primary `.html` leaf, but the normalization parser still required exactly one extension. The cancellation assertion received the taxonomy error before it could check cancellation; the source-inventory and captured-read tests threw before their row assertions. The fourth failure was the fixed-script compiler-vector test exceeding Bun's default five-second timeout at 5.52 seconds.

A new durable regression test uses the existing language-neutral, Ajv-validated HTML source-pair fixture and actual normalization inventory of all sixteen native source files. Before the repair it failed with the same normalization error in 4.17 seconds, with no assertions reached. The test expects the exact declared source identities, HTML kind, unchanged normalized paths, and no violations. The parser now delegates primary filename rendering to the existing shared renderer. The fixed-script compiler-vector test has an explicit bounded 30-second budget.

The first post-repair inventory assertion also caught six unrelated sibling fixture leaves in the test's broad fixture scope. Admission now uses the eight declared owner-directory prefixes and compares every file within those owners to the exact sixteen declared leaves. The full mutation-fixtures suite subsequently passed 12/259. A new source-input control then failed before the three external reader inputs were added to the four package test targets. The original 17-source `htmlSourcePairs` input set remains exact and unchanged; the separate normalization, discovery and taxonomy paths are explicit target inputs. With this closure, the full suite passes 12/271 in 26.79 seconds.

The repaired full package-policy direct route passed 121/530 in 68.16 seconds. That exceeds its existing 60-second outer command cap, which now has an explicit 240-second bound. The first actual registered isolated Nx attempt ran all assertions and passed the three HTML-affected cases, but finished 120/121 in 133.54 seconds because the native Go compiler bootstrap row took 5.84 seconds and exceeded Bun's implicit five-second per-case timeout. Native compiler-oracle rows now have a 30-second case budget; TypeScript-only rows retain five seconds. No role or semantic assertion was weakened. The registered retry remains pending.

The actual registered HTML route is now green: `@semio-tech/repo-test:test-long --args='-t HTML'` passed four tests and 256 assertions with 325 tests filtered out across the existing five files, 86.74 seconds (Nx 1m28s), cache skipped. Its preceding quick-route attempt hit the correct 30-second outer budget during the existing test-platform module's repository contribution scan, before any HTML assertion. The longer existing level completed without an override or extra command registration. This is focused HTML coverage, not the entire repository-test suite.

The actual Nx project graph generated in the private workspace data contains all four package test targets, the unchanged exact 17-source set, the three explicit reader inputs, unchanged commands/cwd, and `cache: true`. Four changed TypeScript sources have zero installed-compiler syntax diagnostics, and the scoped whitespace check passes. Terra independently reviewed the parser, eight-owner admission, and exact reader-input delta.

The final full registered `@semio-tech/repo-lib:test-package-body-policy` retry passes all 121 tests and 530 assertions in 127.51 seconds (Nx 2m8s), cache skipped. It includes the actual root/domain/package inventory, cancellation, captured read failure, compiler/parser oracles and registration checks. This is the final current result after the native-case budget repair; the earlier 120/121 run remains failed evidence. No live normalization/mutation, cleanup, service, browser or native HTML mutation operation was run.

## Exact Product Attribution

Seven current files are updated, with no removed source identity in this repair. Their owned changes are the shared normalization renderer delegation, durable HTML inventory/input control, explicit three-source cache closure, and bounded compiler test/command budgets:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🌐️html-source-pair/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🌐️html-source-pair/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📋️project.json`

This report and the earlier HTML report's follow-up notice are retained ticket documentation. All owned processes completed. Private `🗑️generated/coordinator/package-policy-recheck` output was removed after the final evidence above was retained; product fixtures/schema, reports and authored ticket inputs remain.
