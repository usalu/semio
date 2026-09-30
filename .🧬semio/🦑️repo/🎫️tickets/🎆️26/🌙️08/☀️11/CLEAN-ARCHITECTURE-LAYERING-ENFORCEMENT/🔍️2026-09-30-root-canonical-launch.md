# Canonical Aggregation and Launch Audit

Read-only source/metadata inspection on 2026-09-30; no tests, builds or generator runs. Scanned authored `📋️project.json` files using `rg --files`, parsed their canonical target contributions and compared the canonical launch picker. Console result: `missing [] extra [] count 14`.

## Registration

Root `📜️script.ts:8946–8949` aggregates all canonical owner targets with Nx `run-many -t canonical-architecture --all --exclude workspace`. Excluding workspace avoids recursive root orchestration. It applies no test-name/file exclusion to the contributed owners.

`.vscode/launch.json:11632–11635` exposes the canonical owner target through the project picker; `.vscode/launch.json:20231–20252` lists exactly all 14 current authored contributors. The root aggregate has its own launch command at line 5093. No canonical contributor is missing launch registration.

Contributors: CAD artifact, framework Job, OS TypeScript, OS dev, OS kernel, Layout, demonstrator, presentation deck, Procedural, Puzzle, Raster, repo library, Sequence, and Hub. Picker-based registration is intentional; absence of 14 separate literal project:target rows is not a defect.

## P2: Two Native Contributions Can Accept an Empty Test Selection

`🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts:16` invokes `runCargoTestBudgeted` filtered to `neutral_reconcile_envelopes_match_language_agnostic_vectors`. `🌎️hub/📦️packages/🦀️rust/📜️script.ts:7538` invokes that helper for library tests filtered to `plugin_module`.

The shared helper at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1733–1734` explicitly supplies Nextest `--no-tests warn`. Binary listing before execution is binaries-only, so it does not prove these law filters select tests. Removal/rename of matching native laws can therefore leave the canonical contribution green while executing none; the independent TypeScript proofs still run, but they do not replace the required native laws. Coverage uses the same warn choice at line 1689, and Cargo's fallback likewise lacks a selected-law count assertion.

Use the existing `runExactCargoLaws` route for the intended named native laws, or require a nonempty selected inventory and exact receipts. This is a source-confirmed regression-detection defect, not a claim that recorded current runs selected zero tests. The parent's current native runs reported real executed laws.

## Other Coverage Routes

OS kernel uses `runExactCargoLaws` with its 13 declared laws and explicit target identity, avoiding a zero-test filtered pass. Repo library executes the neutral graph fixture oracle plus live graph validation. Layout/Puzzle renderer configs explicitly set `passWithNoTests: false`, reset inherited test-name filters and select the relocated concrete suites. Raster/Procedural routes select authored concrete test files. OS TypeScript's policy/publication/program filter intentionally selects contributed suites; the parent's recorded 74 tests establish execution for the current filter, while future exact inventory checking remains stronger than a name pattern alone.

No concrete test-hiding change was identified in root aggregation or the reviewed owner routes. The native empty-selection defect above prevents an unconditional claim that every contributed canonical route rejects zero-test execution.
