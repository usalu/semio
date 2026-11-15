# Handoff to the Canonical Tree

Ticket `26/11/12/GET-ALL-PRINT-PRODUCTS-RUNNING-END-TO-END`, goal `runningframework/runningproducts/runningprint`. Written 2026-11-13 about 19:30 PST from the old worktree `/Users/Shared/semio`. The ticket is **not closed**. Nothing of this session is running any more.

Session goal (still open): get all print products running end to end; run commands with `repo run <project>:<target> --lang=en --output=stream --skip-remote-cache`.

## Target state

Green, verified by a real run in the old tree:

- `@semio-tech/print`: `check`, `build`, `build-viz`, `test`, `test-pdf`, `test-measurement`, `test-assignment`, `test-thesis`, `test-thesis-render`, `test-macro-staging`, `test-toolchain`, `test-viz`, `preview-generated`, `generate`, `fonts`, `deps-tectonic`, `deps-tex`
- `@semio-tech/print-rs`: `build`
- `@teaching/architecture-course`: `build-parametrization-en|de`, `build-typology-en|de`, `test-assignment`
- `@semio-tech/mit-bestand-bericht`: `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht`, `build`, `test`, `check-generated`, `akteursnetz-validate`
- PDF audit: 182 of 182 expected PDFs present and valid (`pdf-audit.md`), prototype mode only.

Not green:

| Target | State | Report |
|---|---|---|
| `print:test-viz-full` | Test and all 87 tasks pass; last complete run exited 1 only in teardown (native borrower admission). Borrower fix is in, light tests pass, **no confirming run yet** (three attempts were interrupted by disk, service and this halt). | `big-number-display.md`, `native-borrower-admission.md` |
| `print:test-native-grammar` | Hull assertion fixed. Fails later in the Rust grammar emission: cargo lint warnings overflow the capture cap (65,546 of 65,536 lines). Last attempt (cargo `build.warnings="allow"` for that invocation) was interrupted, **unverified**. An earlier "passing" emission run executed 0 tests and proves nothing. | `native-grammar-hull.md` |
| `print-rs:test` | Compiles. 39 tests, 27 pass, 2 fail (`sqlite_snapshot_chart_canonical_logical_owner_assets_match_actual_producers`, `sqlite_snapshot_chart_control_and_closed_domain_refusals_are_real`, both `OriginalSnapshotRefusal("operation")` at the first `encode_native`), 10 never run. Suspect: the `preflight` call kept in `encode` in `📓️print/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`. A diagnostic label was added; the rerun was interrupted. | `print-rs.md` |
| `print:test-viz-fixtures` | Dependency fix in (`dependsOn` fonts, deps-tectonic, deps-tex, generate). 4-compile proof `-- viz-1` passed. Full run (320 compilations) never completed: about 90 s per compilation under memory pressure, roughly 8 h on that host. Concurrency now comes from the compiler capacity lease; no resumability. | `test-viz-fixtures.md`, `viz-fixtures-sizing.md`, `viz-fixtures-concurrency.md` |
| Never run to completion | `architecture-course`: `build`, `publish`, `test`, `typecheck`, `check`, `preflight`; `print-viz-inference`: `build`, `test`; `print-rs`: `test-logical-owner`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-native-emission:prototype|release`. All earlier attempts failed on infrastructure (full disk, graph service), not on their own code. | `target-inventory.md` |
| Deliberately not run | watch/dev servers, docker targets (one pushes externally), `test-e2e`, `deploy-check`, workspace-mutating regenerators (`generate-viz`, `generate-font-metrics`, `produce-logical-owner`, `generate-actor-network`). | `target-inventory.md` |

## Exact next step

In the canonical tree, one run at a time (16 GB host, the capacity gate queues):

1. `repo run @semio-tech/print:test-viz-full --lang=en --output=stream --skip-remote-cache` — confirms the borrower fix and the glyph-kind contract fix.
2. `repo run @semio-tech/print-rs:test …` — read the `operation:` label, fix the two Chart snapshot failures, then the 10 unrun tests.
3. `repo run @semio-tech/print-rs:test-native-emission …` — confirm a non-zero test count and `SEMIO_PRINT_NATIVE_V1` rows, then `@semio-tech/print:test-native-grammar`.
4. The never-run targets in the table, then full `print:test-viz-fixtures` last.
5. Rerun `check` and one target without `--skip-remote-cache`; verify `repo run-many`.

## Edits made in the old tree

The other session copied edits up to about 17:21. **Edits after 17:21 that may not be copied yet** (all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/` unless noted):

- Closure pinning and missing-closure recovery (about 17:10–17:20, verify): `🗄️cas/🧹️collector/🟦️.ts`, `🗄️cas/📌️pins/🟦️.ts`, `🗄️cas/📂️areas/🟦️.ts`, `🕸️graph/📁️workspace/🟦️.ts`, `🕸️graph/🕰️generations/🔌️load/🟨️.mjs`, plus tests `🗄️cas/🧪️tests/🟦️.ts`, `🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts`. Report `closure-gc-dangling.md`.
- Graph-plan starvation fix (about 18:35–18:55): slice pool `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/…` (`sliceAdmissionV1`, `scaledOwnerWaveV1`, new text `host-slice-owners-reduced`), gate `withHostCapacityV1` / `unsatisfiableHostRequestV1`, new tests `🏡️resident/🧪️tests/🛟️reservation/`, `🌡️host-capacity/🧪️tests/🛟️unsatisfiable/`. Exact paths in `graph-plan-starvation.md`.
- Emission script (hull agent, until about 19:20): `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️rust-package-commands/🟦️.ts` (message format reverted to human, cargo `build.warnings="allow"` attempt, in flux when stopped — re-read before trusting).
- print-rs agent (until about 19:20): possibly `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` and `🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs` (diagnostic label).

Edits before 17:21, by area (details and exact paths in the named reports):

- Print product (`🧰️framework/🛍️products/📓️print/`): `📦️packages/🟦️typescript/📋️project.json` (artifactPackage declaration, `test-viz-fixtures` dependsOn); `🎮️commands/🧪️print-pipeline-verification/🟦️.ts` (Chromium preparation); `…/🧪️tests/🧫️fixtures/🧫️command-boundaries.json`; `…/🧪️tests/🧫️fixtures/🖌️kind-paint/🔣️.json` (five glyph kinds); `📦️artifacts/📖️pdf/🟦️.ts` (undefined operator arguments) and `📜️script.ts` (diagnostic); `📦️artifacts/🟦️.ts` (worker count); `🧪️tests/📜️chart-inference-result/🟦️.ts` (draft-07 meta-schema); `🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🟦️.ts`, new `…/🖼️resources/🖼️raster/🟦️.ts` and `…/🖼️raster/🔌️sharp/🟦️.ts`, `…/🪟️panels/📜️script.ts` (sharp behind an interface); `🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/🟦️.ts` and new `…/📏️measurement/⏱️schedule/` (bounded concurrency, progress); `🧪️tests/🧬️native-chart-grammar/🔣️.json` and `…/🏭️domain-effects/🟦️.ts` (hull oracle); Rust: `🚪️io/🪶️sqlite/📸️snapshot/…`, `🚪️io/💾️binary/🧬️mutations/🦀️.rs`, `🧪️tests/🧬️chart-mutations/🦀️.rs`, `🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`.
- OS product Rust (compile fixes): `💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/🎬️media-export/🦀️.rs`, `…/🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs`, `…/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`.
- Framework process: `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json` (32 MiB bound, plus tests); `…/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts` plus tests and fixture (queue when full).
- Repo product: remote-cache probe (`🏗️build/🌐️remote-cache/…`), scheduling domain (`🗓️scheduling/🔣️.json`, `nx.json`, `🏗️build/🔣️policy.json`), run-many plan dedupe (`🏗️build/🧭️invocation/🛰️plan/🟦️.ts`), slice waves and macOS memory reading, dependency lock NFC normalisation (`⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🟦️.ts`, `…/🔒️lock-sync/🟦️.ts`, tests and Python oracle), retirement type fix (`…/💾️storage/🧹️retirement/🟦️.ts:25`), plan retention (`🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.mjs`, new `…/🧹️retention/`, schema, policy types, fixture).
- `♻️mit-bestand/package.json`: one workspace entry restored from NFD to NFC (identical to HEAD).

## State outside tracked files

- `🗑️generated/` in this ticket folder (110 MB of logs, exit files, `summary-r8.txt`, `summary-r9.txt`): disposable, must be deleted when the ticket closes. Not needed in the new tree.
- `run.lock` (empty directory in the ticket folder): obsolete, delete.
- Ticket scripts to keep: `compute-boundaries.ts`, `paint-oracle.ts`, `real-paint-check.ts`, `glyph-kinds-check.ts`, `lock-check-probe.ts`, and the agents' `ps-rs-run*.sh` under `🗑️generated/` (regenerate if wanted).
- `/Users/Shared/semio/.cache` (113 GB): build outputs, including the audited PDFs under `.cache/b/p/m/print/prototype/documents/`. All reproducible; the new tree will rebuild cold.
- Host-level, shared by both trees: I deleted 59 stale plan files (13 GB) older than 3 h from `$TMPDIR/semio-repo/501/plans-89ee733bc283/`; the learned slice peak file `$TMPDIR/semio-slice-peak-501.json` still holds 8607 MB (the clamp now bounds it). The pinned Chromium headless shell may have been provisioned into the host tool store by `test-viz`.
- `status.md` in the ticket folder is outdated (state of 2026-11-12 evening); this note supersedes it.

## Known open issues

- `repo doctor` and `repo cache gc` do not know the plan temp directory; each plan embeds the whole project graph (about 430 MB for 674 projects).
- Missing-closure recovery re-infers every owner; no bilingual notice.
- Failing tests not caused by this work, unverified origin: lock-sync "every install path asks the lock check…", resident host "held connection keeps the idle rule", `🏊️farm` / `👥️agents` / `🧾️label` (5 tests, a 768 MB vs 2048 MB default), 4 Bazel-scope walk-view tests, one run-many "real client" test.
- About 660 graph owners still hold a raw analysis row, so edits to repo implementation stale them (`owner-staleness.md`).
- Behaviour change in print-rs: whole-number chart floats (`1.0`) are received as integers through the pack-value path.
- `test-viz-fixtures` has no resumability; a cancelled run restarts from zero.

## Added by the worker-setup session (2026-11-13 19:15)

All edits of the old tree are now in `/Users/ueli/Documents/semio`, except seven files that the fleet changed in the same hour (host-capacity gate, decision, schema and facade; slice-pool and its limit). Their old-tree versions are in `unmerged-from-old-tree/` with a table of the paths. The canonical tree holds the fleet version of those seven: re-apply the starvation fix there first.
