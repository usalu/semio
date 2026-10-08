# L-S4a: Library Launch Assertions, Table A Rows 1–27

Slice L-S4a of ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT` · 2026-10-07 · `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.
Every result line below is a `bun test <absolute file>` run of this session (runner `l-s4a-run.sh`); nothing was run through Nx.
"UNVERIFIED" = the edited assertion could not execute because the test fails earlier for a reason that predates this slice.

## 1. Outcome

- All 27 test files no longer read `.vscode/launch.json` or `.vscode/🧩️launch.seed.jsonc` and carry no launch name, command, group or order. `grep -i launch` over the 27 files returns nothing.
- No test file was deleted, so no `🥒️.feature`, Nx target, oracle ledger or registry entry had to go. Two launch-only cases inside files were deleted (rows 6 and 22).
- No regression: every file has the same or a smaller failure set than before the slice (section 3). 12 files were green and stay green; 4 files gained passing cases; the remaining red is pre-existing (section 5).
- No policy was lost and nothing was moved into an Nx target (section 2). No `📋️project.json` was edited by this slice.
- Three things are deliberately left and need a decision or a sibling (section 6).

## 2. Policy Decision (R7)

`l-s4a-launch-policy.ts` listed the real rows of both launch files for all 41 targets these tests pinned. No row carries `env`, a non-default `cwd` or arguments after `--`. The only policy is the Nx flag `--skip-nx-cache`, present on 27 of the 41 real rows (the fixtures of rows 1, 4 and 10 expected it as well; those rows had drifted).

| policy in the launch row | decision | where it lives now |
|---|---|---|
| `--skip-nx-cache` (rows 1, 2, 4, 5, 7, 10–16, 19–27 and the six handoff rows of row 22) | developer choice | dashboard global axis `cache` (`skip-local`); the Nx target and the `package.json` script never carried it |
| plain `bun nx run P:T` (rows 3, 6, 8, 9, 17, 18) | none | — |
| presentation `group` / `order` (4xx.xxx) | none | Run-panel ordering, dead with the panel |

Already intrinsic to the gate and untouched: `test-artifact-support` runs `artifact-support long` in its target command (row 7); the verbs `artifact-source-residue` and `artifact-source-commit` set `SEMIO_TEST_LEVEL=long` and `--timeout 120000` inside `LIB/📦️packages/🟦️typescript/📜️script.ts` (row 22).

Note for the axis default: the workspace Nx plugin infers `cache: true` for every `test-*` / `lint-*` target (`LIB/🟨️.mjs`, `cacheableFamily`), and several of these gates scan the whole repository beyond their declared inputs (row 27 `lint-artifact-io-ownership` is the clearest case). A replayed pass can therefore be stale unless the dashboard's `cache` axis defaults to `skip-local` for the verbs `test`, `gate` and `lint`. That default belongs to M-1a; it is not changed here.

## 3. Per File

Result = pass/fail before → after. "reg." = the registration case that held the launch block.

| # | test (`LIB/🧪️tests/…/🟦️.ts`) | removed | remains | fixtures / schema touched | result |
|---|---|---|---|---|---|
| 1 | `↪️rust-divergence-callback` | launch loop per phase; `launchPaths`, `launchOrder`, `launchName`, `launchCommand` in the inlined expectation; `jsonc-parser` import | Nx target, package script, router branch, arguments and options per phase | `🧫️fixtures/↪️rust-divergence-callback/🔣️.json` (also stale `testPath` corrected to the real file) | source phase 41/3 → 44/0; native phase 3/0; combined 47/0. Syn phase (120 s Cargo build) NOT RUN |
| 2 | `♻️taxonomy-pattern-compiler-reuse` | launch loop; `launchOrder` mutation case | schema, Nx target, package script, compiled router | `…/🧪️registration/🔣️.json` | 18/3 → 18/3; reg. UNVERIFIED (registration schema file missing) |
| 3 | `⚙️root-script-compiler` | launch loop; import | Nx target command | `…/🔣️.json` | 6/0 → 6/0 |
| 4 | `✍️rust-writable-path-authority` | launch loop; launch keys in the inlined expectation; import | Nx target, router branch, package script | `…/🔣️.json` (stale `testPath` corrected) | 0/47 → 1/46; reg. now green |
| 5 | `🌐️registry-import-language` | launch loop; type fields | schema version, selection, Nx target | `…/🔣️.json` | 16/0 → 16/0 |
| 6 | `🌳️kind-only-basename` | whole case "derives every focused implementation command from the launch seed authority" (asserted three rows, nothing else) | all other cases | none (no launch keys) | 6/1 → 6/0 |
| 7 | `🍃️artifact-support-leaf-authority` | launch loop; type fields; import | Nx target command | `…/🔣️.json` | 16/3 → 16/3; reg. green |
| 8 | `🎚️tool-configuration-ownership` | launch `toContain` line; inlined `registration.name` / `.command` expectations; type | taxonomy laws, router, Nx target command and inputs, package script | `…/🔣️.json` (`registration` keeps `target`) | 6/3 → 6/3; reg. green |
| 9 | `🎚️vitest-configuration-ownership` | launch loop; type | Nx target command, package script | `…/🔣️.json` (`registration` keeps `target`) | 5/2 → 5/2; reg. green |
| 10 | `🎟️reference-coverage-selection` | launch loop | Nx target, package script, router branch, compiled route | `…/🔣️.json` | 3/2 → 4/1; reg. now green |
| 11 | `🎯️cargo-target-discovery-skip` | launch loop; type fields; import | Nx target command | `…/🔣️.json` | 2/0 → 2/0 |
| 12 | `🏺️historical-package-owner-identity` | launch catalogs map | project JSON parity, Nx target command | `…/🔣️.json` (`launchName`, `launchCommand`, `launchCatalogs`, `group`, `order`) | 26/0 → 26/0 |
| 13 | `👀️readme-reviewed-fixture-inputs` | launch map and its half of the registration object; JSONC mode of the helper | package name and script, Nx target, router branch | `…/🔣️.json` (launch keys; the two `.vscode` paths in `copies`) | 0/1 → 0/1; whole file UNVERIFIED (module throws `manifestSchemaPath is not defined`) |
| 14 | `💠️inventory-artifact-shards` | launch loop; import | Nx target command | `…/🔣️.json` (incl. `group`, `order`) | 8/0 → 8/0 |
| 15 | `💥️nested-cargo-collision-authority` | launch loop; import | Nx target command | `…/🔣️.json` (incl. `group`, `order`) | 25/0 → 25/0 |
| 16 | `📈️reference-coordinate-progress` | launch documents and loop | Nx target, package script, router | `…/🔣️.json` (`launchName`, `group`, `order`, `nxCommand`) | 21/0 → 21/0 |
| 17 | `📣️plugin-publication-source-ownership` | launch loop; type | Nx target, package script, router | `…/🔣️.json` (`registration` keeps `target`) | 6/1 → 6/1; reg. green |
| 18 | `📱️app-verification-source-ownership` | launch loop; type | Nx target, package script, router | `…/🔣️.json` (`registration` keeps `target`) | 1/5 → 2/4; reg. now green |
| 19 | `🔎️json-reference-owner-lookup` | launch loop; import | Nx target, router branch | `…/🔣️.json` (launch keys; corpus member `.vscode/🧩️launch.seed.jsonc`) | 7/0 → 7/0 |
| 20 | `🔖️readme-current-source-revision` | launch loop | Nx target, router branch | `…/🔣️.json`; `LIB/🧬️schema/🔖️readme-current-source-revision/🔣️.json` (`required` and `properties`); re-pin, section 4 | 11/0 → 11/0 |
| 21 | `🔤️taxonomy-leading-grapheme` | launch loop; `launchOrder` mutation case | schema, Nx target, package script, compiled router | `…/🧪️registration/🔣️.json` | 9/1 → 9/1; reg. UNVERIFIED (registration schema file missing) |
| 22 | `🔬️workspace-contract` | see below | see below | see below | targeted cases only, see below |
| 23 | `🕰️historical-json-source-encoding` | launch loop for both gates | project JSON parity, Nx target commands | `…/🔣️.json` (`execution` rows keep `id`) | 25/0 → 25/0 |
| 24 | `🗺️testing-readme-coordinates` | launch loop | Nx target, router branch | `…/🔣️.json` | 14/0 → 14/0 |
| 25 | `🚚️readme-move-source-authority` | launch map and its half of the registration object; JSONC mode of the helper | package name and script, Nx target, router branch | `…/🔣️.json` | 17/0 → 17/0 |
| 26 | `🚧️cargo-discovery-exclusions` | launch loop; type fields; import | Nx target command | `…/🔣️.json` | 3/2 → 3/2; reg. green |
| 27 | `🚪️artifact-io-ownership` | launch loop for `test-` and `lint-artifact-io-ownership`; import | compiler diagnostics, both Nx targets, both package scripts | none | 6/6 → 6/6; reg. green |

Renamed case titles (rows 3–5, 7, 9, 11–20, 23–26) only lose the words about launch catalogs. No ledger, feature file or router pattern names the old titles (repository search).

### Row 22, `🔬️workspace-contract` (7.6k lines; also listed in table B)

The whole file was NOT RUN. Targeted runs: default level for the handoff cases, `SEMIO_TEST_LEVEL=long` for the Draw cases (they live in `describe.if(testLevelAtLeast("long"))`).

| part | change | fixtures | result |
|---|---|---|---|
| "resident native metadata binds…" | launch loop removed | `🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json`: `launch` array and `existingTs.launch` | pass → pass |
| "UI-host metadata binds…" | launch loop removed | `…/🖥️ui-host-package/🔣️.json`: `launch` array (4 rows) | fail → fail, UNVERIFIED (handoff schema file missing) |
| `assertArtifactProjectionSingleCaseRoute` | launch loop and the two vector fields removed | `🧫️fixtures/🖍️draw-source-scenario/🛣️commit-route/🔣️.json`, `…/📨️submitted-proof/🔣️.json` | residue route pass → pass; commit route fail → fail (schema file missing) |
| "authored Draw source launch seed matches its host…" | case deleted (it called `generateLaunchJson`; already red: "Isolated producer module is outside the captured closure"); its only helper `artifactProjectionIsolatedProducerModule` deleted | — | fail → gone |
| authored launch seed as a producer input (`DrawSourceScenario.launchSeed`, the authored map in "producer context…", `artifactProjectionProducerInputs`) | removed with `l-s4a-draw-seed-removal.py --apply` | `🧫️fixtures/🖍️draw-source-scenario/🔣️.json`: `launchSeed` block | "schema and parser oracles" pass → pass; "producer retains imported JSON bytes" pass; "producer context…" fail → fail (unrelated `🎠️kernel` assertion) |

The last part was coupled to L-S2: the scenario may only supply inputs that `generatorContracts["plugin-registry"].inputPatterns` declares. It was applied at about 19:57, right after L-S2 removed the seed from `LIB/🔣️taxonomy.json`; between the two edits three producer cases and `❄️frozen-markdown-coordinates` threw "Authored producer inputs must be exact declared producer inputs". `❄️frozen-markdown-coordinates` (extracts three functions of this file) is 36/0 again.

## 4. Sealed Pins Changed

Removing the launch keys from the revision gate fixture changed its digest `5eb7fd80…` → `2db36a01…`. Re-pinned in:

- `LIB/🔣️taxonomy.json`: `frozenCoordinateEvidenceContracts["readme-current-source-revision-input"].sha256` (one line).
- `LIB/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json`: `revisionInputSha256` (one line; the file belongs to row 32 of L-S4b).
- `LIB/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json`: one new `reseals` entry for that contract, seal `5c63712d…`, coordinates unchanged (`fcb1623a…`), `recordedBy` this ticket. Computed with `l-s4a-seal.ts`, which reproduces the recorded seal of the untouched neighbour contract.

Verified by rows 20 and 23 (green) and by `☂️frozen-coordinate-wildcard-coverage` (5/0).

## 5. Red That Predates This Slice

- Commit `d4597d0410a` (2026-10-06, 26,114 files) deleted most of `LIB/🧬️schema/**`. Tests that load those schemas throw `ENOENT` or reference constants that went with them: rows 2, 10, 13, 21, 22 (UI-host, commit route), and row 32 of L-S4b (`fixtureSchema is not defined`). If the schemas are restored from history, the registration and execution schemas must lose their launch properties first.
- Row 4: `preservedFiniteCheckpoint` pins the digest of `🥤️rust-finite-target-consumption/🟦️.ts` (row 34, L-S4b); it already mismatched and will move again with their edit. 42 further cases fail with `canonicalJson is not defined`; the last 3 failures were not examined.
- Rows 7, 8, 9, 17, 18, 26, 27: failures in cases that never touched launch data (owner maps, Vite/Vitest loaders, consumer closures, Cargo recipes, IO ownership).
- Row 13 fixture `execution.source` still names the pre-rename path `🧪️tests/🟦️readme-reviewed-fixture-inputs.ts`.

## 6. Left Open

1. `LIB/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json` keeps `launchName`, `launchCommand`, `launchGroup`, `launchOrder` (table C lists it). It is the byte copy of the reviewed expectation of an approved README revision; no code reads those keys, and they match none of the final acceptance spellings (`launch.json`, `launch.seed`, `🚀️launch`). Stripping them changes the approved revision identity: new expectation digest and size in `📋️manifest/🔣️.json`; the manifest digest in two fixtures; `expectationsSha256` twice plus `expectedRevisionDigest` in the revision fixture and in the activation fixture of L-S4b; three digests in `🔣️taxonomy.json`; two ledger reseals. The gate that guards the copy (row 13) cannot run today. Owner decision.
2. Row 19: the launch seed was the only tracked `.jsonc` document outside tickets, so the corpus case now covers JSON documents only. JSONC parsing stays covered by the synthetic cases of the same fixture.
3. Row 17: the `🚀️launch` directory context and the `🧪️tests/🚀️launch/🟦️.ts` consumer in its fixture were removed by L-S2 at 19:58 together with the taxonomy kinds; not edited here.

Fixture ownership against L-S4b: the readme family (`👀️readme-reviewed-fixture-inputs/**`, `🔖️readme-current-source-revision`) is read by rows 13, 20 and 32 and was handled here (lower row). In `🟢️readme-current-source-activation/🔣️.json` only the digest line was touched.

## 7. Files

Created (ticket folder, kept as input scripts): `l_s4a_edit.py`, `l-s4a-run.sh`, `l-s4a-seal.ts`, `l-s4a-launch-policy.ts`, `l-s4a-draw-seed-removal.py` (applied; refuses a second run), this report. Generated logs under `🗑️generated/launch-s4a/` were deleted.

Updated, tests (27): `LIB/🧪️tests/<name>/🟦️.ts` for every row of section 3.

Updated, fixtures and schema (33):
- `LIB/🧫️fixtures/`: `↪️rust-divergence-callback/🔣️.json`, `♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json`, `⚙️root-script-compiler/🔣️.json`, `✍️rust-writable-path-authority/🔣️.json`, `🌐️registry-import-language/🔣️.json`, `🍃️artifact-support-leaf-authority/🔣️.json`, `🎚️tool-configuration-ownership/🔣️.json`, `🎚️vitest-configuration-ownership/🔣️.json`, `🎟️reference-coverage-selection/🔣️.json`, `🎯️cargo-target-discovery-skip/🔣️.json`, `🏺️historical-package-owner-identity/🔣️.json`, `👀️readme-reviewed-fixture-inputs/🔣️.json`, `💠️inventory-artifact-shards/🔣️.json`, `💥️nested-cargo-collision-authority/🔣️.json`, `📈️reference-coordinate-progress/🔣️.json`, `📣️plugin-publication-source-ownership/🔣️.json`, `📱️app-verification-source-ownership/🔣️.json`, `🔎️json-reference-owner-lookup/🔣️.json`, `🔖️readme-current-source-revision/🔣️.json`, `🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json`, `🕰️historical-json-source-encoding/🔣️.json`, `🗺️testing-readme-coordinates/🔣️.json`, `🚚️readme-move-source-authority/🔣️.json`, `🚧️cargo-discovery-exclusions/🔣️.json`, `🤝️package-language-kind-handoff/💾️resident-package/🔣️.json`, `🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json`, `🖍️draw-source-scenario/🔣️.json`, `🖍️draw-source-scenario/🛣️commit-route/🔣️.json`, `🖍️draw-source-scenario/📨️submitted-proof/🔣️.json`, `🧫️frozen-seal-ledger/🔣️.json`, `🟢️readme-current-source-activation/🔣️.json`.
- `LIB/🧬️schema/🔖️readme-current-source-revision/🔣️.json`.
- `LIB/🔣️taxonomy.json` (one digest).

Removed: none.
