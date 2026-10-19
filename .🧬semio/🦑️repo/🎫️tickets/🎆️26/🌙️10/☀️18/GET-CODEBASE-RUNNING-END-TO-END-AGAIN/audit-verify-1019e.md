# Audit verify-1019e (aud-verify5, read-only)

Scope: the nine results `result-rg-plug`, `result-rg-fem3d`, `result-vcs-bound`, `result-course-img`, `result-zt-law`, `result-lay13`, `result-split-fw`, `result-docs-build2`, `result-fixups3`, checked against the working tree (`git diff HEAD`, `git show HEAD:`) on 2026-10-19 ~17:45 to 18:00. No builds, no cargo, no `bun repo`, no git-modifying command, no source edits. Test runs: two explicit-file `bun test` runs (logs in `🗑️generated/aud-verify-1019e/`). The tree is moving (peer edits at 17:22 to 17:46), so the verdicts describe the tree at the time of the audit.

## Verdict per result

| Result | Verdict | Basis |
| --- | --- | --- |
| rg-plug | Holds for lost functionality; gate and cargo claims not re-run | method-set comparison of the whole puzzle5d crate (below) |
| rg-fem3d | Holds, static only | no moved test references `super::`, `crate::` or a private item; cargo not run |
| vcs-bound | FAILS at the current tree: 112 pass / 5 fail, not 117 / 0. Two claimed law changes are absent | `bun test` run, explicit file |
| course-img | Partly wrong: Cargo.lock is not "additions only" | semantic lock comparison |
| zt-law | Holds | `bun test` explicit file: 22 pass, 0 fail |
| lay13 | Holds with caveats; 1 survivor is not on disk; counts differ by 7 | 90 seeded samples, see section 5 |
| split-fw | Holds | 47 of 47 titles found in course suites (substring check) |
| docs-build2 | Mostly holds; 2 to 3 statements not confirmed at call-site level | 10 statements, see section 7 |
| fixups3 | Holds | facade names have no remaining importer outside the two test targets |

## 1. rg-plug: deleted puzzle5d "duplicates"

Method: Python script (session scratchpad, not in the ticket) over every `.rs` file of `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d` at HEAD vs the working tree, comparing `(impl header, fn name)` pairs and top-level `struct/enum/trait/type/const/static/macro_rules` names. Limit: compares names and headers, not bodies. Byte equivalence of the paste/import copies is claimed by the executor and not re-diffed.

- HEAD: 984 (header, fn) pairs; working: 997.
- Pairs absent from the working tree: 1, `Puzzle5dAddBrushPartWork::from_board_payload`. It has no caller at HEAD (grep of tracked and tree code finds only its definition), so this is dead code, not lost functionality.
- Pairs with fewer copies: 40, all of them HEAD duplicates (2 copies, now 1): `PasteJob` (new, turn, checkpoint), `ImportJob` (new, turn, checkpoint, rows, push_mutation, snapshot_rows, snapshot_kind_compatibility_rows), `ProximityConnectWork`, `AddNodeWork`, `AddBrushPartWork`, `CreateFastenerWork`, `Puzzle5dCommand` (action_id, args, from_action, from_json, to_json, try_from_action, window_id), `Puzzle5dPlayApp::context_menu_body`.
- Top-level items at HEAD: 478; missing by name in the working tree: 0. Locations checked: `Puzzle5dBoardWindowConfigDiff` and `Puzzle5dWorldWindowConfigDiff` now in `🪟️window/🍓️config-diffs/🦀️.rs` (new file); `PUZZLE5D_IDENTITY_CEILING_BYTES` in `🌉️wasm/🌉️bridge`; `PUZZLE5D_IMPORT_*` in `🧩️entities` and `🖌️session/📥️snapshot-import`; `puzzle5d_app_manifest_for_tests` and `testkit` re-exported by the mirror crate under `cfg(any(test, feature = "testkit"))` from `semio_s_artifact_puzzle_5d_testkit`; `mod kit` replaced by `pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::kit`; `Puzzle5dPlayApp`, `handle_action_impl`, `with_puzzle5d_app` in `🍠️play-app/🦀️.rs`; `dispatch_puzzle5d_action` also in `🎬️dispatch/🦀️.rs`; `create_puzzle5d_app` in `🚀️app-assembly`.
- Not verified: the "1,150 lines" figure, the gate numbers (1.274 to 0.867), and the cargo state.

Verdict: no lost functionality found. Add one line to the report: `from_board_payload` removed as dead code.

## 2. rg-fem3d: moved unit tests

- The moved unit files (`🩹️change-*/🧪️tests/🔬️unit/🦀️.rs`) contain zero `super::` or `crate::` references (scanned in 3d and 2d).
- The per-kind fixtures mounted by the new `[[test]]` roots (`*-kinds/🦀️.rs`, 7 roots per aggregate) contain zero `super::`/`crate::`/`self::` references, and every `#[path]` target exists.
- Item paths used by the moved files (for example `semio_s_artifact_fem_3d_model::...::schema::diff::Fem3dSectionPatch`, `...::snapshot::Fem3dSnapshotValue`) resolve to `pub mod diff`/`pub mod snapshot` and `pub struct` items.
- `mutation_grammar_conformance`: the `[[test]]` path resolves (`../../../../🚪️io/...`), `protocol` is declared by `extern crate` in the file, `MUTATION_GRAMMAR_SEMIO` and `mutation_grammar_recognizer` are `pub`, and `semio-framework-dsl-record` is a dependency.
- Open: cargo check of both workspaces (`--lib --tests`) not run. Static evidence only. The `fixture!` macro's relative paths are unchanged because the test files did not move.

Verdict: holds, static.

## 3. vcs-bound: law changes (exact list, as found in the tree)

Test run: `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` (explicit file). Result: 112 pass, 5 fail, 117 tests. The result file claims 117 pass / 0 fail. Log: `🗑️generated/aud-verify-1019e/vcs-bound-run.log`.

Failures (one root cause):
- `neutral kernel cuts retain exactly one authored owner and producer-facing consumption` fails at `expect(producer).toBeDefined()`.
- `all new kernel owners enforce active source caps` fails 4 times (4 target modes) with `TypeError: undefined is not an object (evaluating 'producer.entry')`.
- Cause: `🧬️cuts.json` now names producer `semio-framework-value-native-owned`, and `🔣️.json` (corpus) has no row for it. The corpus still has `semio-framework-value-native-decoding` only. The new crate exists at `🛬️decode/🧳️owned/📦️packages/🦀️rust` (created 17:46) and is a facade (`pub mod native_decoding { pub use semio_framework_value_native_decoding::native_decoding::*; }`). This is an in-flight peer split that was not finished in the corpus.

Law changes, as found in the tree:
1. `🧬️cuts.json`, cut for the value list: producer `semio-framework-value-paged-list` changed to `semio-framework-value-persistent`, canonical `pub use semio_framework_value_persistent::{list, retained_equality};`. Present.
2. `🧬️cuts.json`, cut for native decoding: producer `semio-framework-value-native-decoding` changed to `semio-framework-value-native-owned`, canonical `pub use semio_framework_value_native_owned::native_decoding;`, definition `pub mod native_decoding;` changed to `pub mod native_decoding {`. Present, and it breaks the law (no corpus row).
3. `🔣️.json`: new row `semio-framework-value-persistent` (4000 lines, forbidden `os-kernel`, `plugin`, `semio-framework`, `semio-framework-value`). Present.
4. `🟦️.ts`: explicit `120000` ms timeouts on four closure tests. Present.
5. Claimed, NOT present: ceiling `semio-framework-os-store-testing-fixtures` 1700 to 1800. The corpus still says 1700. The row test for that crate passes in this run, so the change is not needed now; the report is wrong to claim it.
6. Claimed, NOT present: `testCaps` overrides for `value-native-decoding` (52/344064/2200) and `value-retained-clone` (32/270336/3800), and the schema for them. `testCaps` does not occur anywhere under the crate-boundaries test or fixture directories. The report is wrong to claim it.

Fixes:
- F1 (blocks the law): add a corpus row to `🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` for `semio-framework-value-native-owned` with `manifest` `../../🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml`, `entry` `../../🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust/🦀️.rs`, `maximumActiveLines` as measured (4000 is the usual cap), and the same forbidden dependencies. Alternatively revert the `🧬️cuts.json` producer to `semio-framework-value-native-decoding` until the owner's split lands. Then re-run the explicit-file test; expect 117 / 0.
- F2: correct the vcs-bound result (items 5 and 6 above, and the 112 / 5 status).

## 4. course-img: Cargo.lock "additions only"

`git diff --numstat`: 181 insertions, 9 deletions. Semantic comparison (`[[package]]` blocks, HEAD vs working):
- 16 packages added; 0 removed; 0 version changes.
- 1 dependency edge removed: `semio-framework-replication-codec` no longer lists `semio-framework-dsl-record`. Its manifest at `🧰️framework/🔨️modules/📡️replication/⚙️codec/📦️packages/🦀️rust/Cargo.toml` has no such dependency, at HEAD or now, so HEAD's lock was stale and the removal is correct.

Verdict: the regeneration is sound, but "additions only" is false. Fix: change the wording to "16 packages added, one stale edge removed (replication-codec to dsl-record, not in the manifest)". The two code fixes (`write_float_to` exists in `🎯️float-writer/🦀️.rs`; the `value-derive-expansion` dependency line is present) were confirmed.

## 5. lay13: 1,017 rewrites

Method: `lay13-tools/ncpr-plan-applied.json` edits (1,275 entries; `applied` says 1,274, one `stale`), minus the unique keys in `reverted-regress.json` and `reverted-closure.json` (250 unique keys), gives 1,017 survivors. The result claims 1,274 applied minus 257 reverted = 1,017. The reverted list has 250 unique keys; the two numbers agree on the total only.

Sampling (`lay13verify.py`, seeded): for each sampled survivor, the import or export statement with the new specifier is read from the file, the specifier resolves through the package `exports` map to its glue, and each imported name's origin in the glue must equal its origin in the original deep file.
- seed 1019, 30 draws: 16 verified. 1 flagged (`@semio-tech/framework/ui/testing/fixtures`): a false positive, since the glue and the deep file both re-export the same JSON origin. 13 not checkable (dynamic `import()` with no named bindings).
- seed 1019, 60 draws: 22 verified; 3 flagged, all checked by hand and correct: `framework-replication` (`orbitPointToScreen` reaches the glue through `export *` to the replication root barrel, which re-exports it from `peer-overlay`), `framework-plugin/build/execution` (glue is `export * from` the right module), `config/mutations` (the deep file does export the name in a multi-line `export type`); 1 absent; 9 relative specifiers not resolved; 26 dynamic.
- seed 7, 60 draws: 26 verified; 1 flagged (`framework-plugin/build/execution`, a false positive as above); 9 relative; 24 dynamic.
- Absent on disk: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📬️typed-result/🟦️.ts` lists `@semio-tech/value-ts/decode` as a survivor, but the file does not contain that specifier. The file is modified in the working tree (6 insertions, 11 deletions) by someone else. Real mismatches found: 0.

Verdict: holds. Fixes: F3, recount the reverted set (257 vs 250 unique keys) and re-run `ncpr-all.ts` once the peer edit settles, so the absent survivor is reconciled.

Not inspected: `exports-needed.json` and the `renderer-hosts-only-ui` policy conflict.

## 6. split-fw: deleted `learner-journey`

- HEAD file `🧰️framework/🛍️products/📚️course/🧪️tests/🚶️learner-journey/🟦️.tsx` has 47 `it`/`test` titles, all unique.
- Each title (exact string) occurs in the 77 `.ts`/`.tsx` files under `🧰️framework/🛍️products/📚️course/🧪️tests/` outside `learner-journey`: 47 of 47 found, 0 missing. This is a substring presence check, not a same-suite check.
- Verdict: holds. Note the substring method is weak for duplicate-like titles.

## 7. docs-build2: 10 statements checked against source

| # | Statement | Check | Result |
| --- | --- | --- | --- |
| 1 | `bazelCacheDecisionV1` decides Bazel cache classes | defined in `🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | confirmed |
| 2 | `.bazelrc` sets no sandbox or strategy flags | grep `sandbox|strategy` in `.bazelrc` | confirmed (none); a test in `💾️storage/🧪️tests` reads the layout, the pin test not read in full |
| 3 | `requireBazelHostPathsV1` guards install, outputs and server on darwin | defined in `💾️storage/🟦️.ts`, tested there | defined; call-site in `install`/`outputs`/`server` not confirmed |
| 4 | `verify build-owners [sync]` route | `🎛️commands/✅️verify/🟦️.ts:47` | confirmed |
| 5 | CI jobs `smoke` and `products` in `build-system.yml` | `.github/workflows/build-system.yml` lines 41, 177 | confirmed |
| 6 | Devcontainer runs unprivileged (stale `privileged: true` removed) | `docker-compose.yml` comment, no `privileged: true` | confirmed |
| 7 | Windows readiness refuses while `LongPathsEnabled` is off | Win32 reader in `🗂️system/🟦️.ts` | reader exists; refusal in `bazelPlatformProblemsV1` not read |
| 8 | `SEMIO_TASK_RECEIPT` only for the declaring task | string occurs in README and `🧾️store-receipt/🧬️schema/🔣️.json`, not in any `.ts` | not confirmed in code |
| 9 | `workspaceCommand` rule (at least 2 words, unique) | not read | not checked |
| 10 | Bazel 8.4.2 is the pinned version handled on macOS | README only | not checked |

Verdict: no contradictions; statements 3, 7, 8, 9 and 10 need the call-site check before the docs can say "verified". Fix: either confirm them or soften the wording.

## 8. fixups3: mesh facade and other claims

- Removed facade names (`MeshSurfaceJob`, `analyze_polygon_soup`, `MeshQualityReport`, the journal names) have no importer in `🥽️mesh/🧪️tests/**`; the only importers are the two `surface`/`quality` unit roots, which now import directly.
- `FileStamp` narrowing: no outside users (`git grep` in `🧰️framework` and `✏️s`). Consistent with the claim.
- `[DEBUG]` line in `🛂️host.ts`: not re-checked in this audit.

Verdict: holds.

## 9. AGENTS compliance of new code (sampled)

- `AGENTS.md` unchanged (not in `git status`).
- No new script file (`.sh`, `.py`, `.ps1`, `.bat`, `.cmd`, `.pl`, `.rb`, or `*script*.ts` other than `📜️script.ts`) outside the ticket folder; no new `.md` outside the ticket folder.
- No untracked path of 256 characters or more.
- No `@deprecated`, compatibility layer or legacy code added in repo code (the only hit was this brief's own text).
- The zero-touch core and law adapter: docstrings start with an emoji; no `//` comments inside definitions (148 and 16 lines checked).
- Console and `println!` lines added outside tickets (28) are permanent CLI or verify output (for example `[verify build-owners] ...`, `[semio-os-mcp] ...`), not temporary logs, so the `[DEBUG]` rule does not apply. One test-only `console.log("Flow source bootstrap ... PASS")` may be a leftover; not verified whether it is new.
- Rust `///` lines: continuation lines of multi-line docstrings lack an emoji; the rule applies to the first line only, and a per-first-line check was not run.

## Concrete fixes (summary)

- F1 (vcs-bound, blocks the law): add the `semio-framework-value-native-owned` corpus row, or revert the cut producer; then re-run the explicit file test (expect 117 / 0). Owner: the peer editing `🛬️decode/🧳️owned`.
- F2: rewrite the vcs-bound result: 112 / 5 at 17:45; remove the ceiling 1700 to 1800 and `testCaps` claims (absent from the tree).
- F3: rewrite the lay13 result: 1,017 survivors but one absent on disk; reverted count 257 vs 250 unique keys; re-run `ncpr-all.ts` after the peer edit.
- F4: rewrite the course-img lock claim: 16 added, 1 stale edge removed.
- F5: rg-plug result: add "`from_board_payload` removed as dead code"; state that cargo and gates were not re-run.
- F6: docs-build2: confirm or soften statements 3, 7, 8, 9, 10 in section 7.
- No change needed: rg-fem3d, zt-law, split-fw, fixups3.

## Not run

- Cargo checks of any Rust change in the nine results (no builds allowed).
- Gate tool numbers (`gate.ts`), the fem `verify` lanes, the zero-touch `bun repo --help` without node_modules.
- Any test other than the two explicit files listed here.

## Runs

- `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` (SEMIO_TEST_ARTIFACT_DIR under `🗑️generated/aud-verify-1019e/`): 112 pass, 5 fail, 49.99 s. Log: `🗑️generated/aud-verify-1019e/vcs-bound-run.log`.
- `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts`: 22 pass, 0 fail, 20.63 s. Log: `🗑️generated/aud-verify-1019e/zt-law-run.log`.
