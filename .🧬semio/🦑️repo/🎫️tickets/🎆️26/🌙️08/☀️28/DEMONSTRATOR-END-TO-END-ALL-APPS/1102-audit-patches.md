STATUS: AUDIT DONE, 25 patch files: 8 APPLIED, 2 PARTIAL, 10 NOT APPLIED, 3 UNVERIFIED, 1 OBSOLETE, 1 not a patch; 67 files modified after 01:00 Nov 2 checked, unmapped ones listed in the stray table

Method: read-only. Unified diffs checked hunk by hunk (old and new block searched in the target file, Python helper in `/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/aud/hunks.py`). Python apply scripts checked for their old/new anchor strings with grep. Markdown plans checked item by item. `git diff HEAD` used only for reading (no modifying git commands). Where a target could not be located, the row says UNVERIFIED.

## Patch verdicts

| Lane | File (under 1101-tools/<lane>/patches/) | Verdict | Evidence |
|---|---|---|---|
| closure | closure-manifest-deps.patch | APPLIED | All ~40 Cargo.toml file hunks match the new (post-patch) text in the tree (the `NEW` check). Removed dependency lines absent. |
| compute | trapped-guest-test.md | UNVERIFIED | Test-only plan. The shared `pump` helper it says was applied could not be located under 🧰️framework within the search depth. Production code unchanged per plan. |
| describe | APPLY-AFTER-FREEZE.md | PARTIAL | Item 4 NOT APPLIED: `🪆️subsets/🧱️base/🚪️io/📐️geometry/📦️packages/🦀️rust/Cargo.toml` (step) has no `semio-framework-async-macros` line. Item 1 (`.result` in stdio-semio unit test) UNVERIFIED: no `retire(factory` call found in the first unit test files searched. Items 2 and 5 (test-dir edits) UNVERIFIED. |
| energy | bridge-construction-recipient.py | APPLIED | `StepRecipient` struct and `NativeDecodeConstructionRecipient` impl present in `📸️snapshot/🧬️original/🌉️bridge/🦀️.rs` (3 and 2 hits). Call-site replacement not grep-checked. |
| fem | PLAN.md | UNVERIFIED | Targets (3d `editor-panels` `🗿️artifact` and `🔍️inspection` files) not located by bounded find. 2d `editor` derive: `🎛️play-app-editor/🦀️.rs:25` uses `FemGumballTransientMutation` but its definition/derive not located. |
| flowext | geometry-close-ticket.py | APPLIED | Prefixed `semio_framework_value::geometry_close_ticket(` count 0 in `🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs`; bare call form present. |
| flowext | retirement-tests.md | PARTIAL | Applied: `#[cfg(any(test,feature="testkit"))]` on FlowHost `recorded` field (`🖥️host/🏠️flow-host/🦀️.rs:127`) and push site (`⏪️undo-redo-history/🦀️.rs:85`); retirement-tests Cargo.toml has `features=["testkit"]` and `unit_tests` dep. Not checked: `[features] testkit` in model crate Cargo.toml, host-authoring and retirement recorded sites (both modified 01:03 but not diffed), window-config test (item 2, UNVERIFIED). |
| gen | brep-boolean-funded-phases.patch | NOT APPLIED | 5 files, 0 of 6 hunks applied. `run_to_completion` still calls `step_unbudgeted` (resumable-boolean-job line 252), `step_unbudgeted` still `pub(crate)`, no `is_funded_validation`. `BrepSession` enum has no `Failed` variant; `step_boolean_job_sync` still the old body; `analysis-check` and `brep-boolean` have no `Failed` arm. Obsolete-risk: none, anchors still match. |
| mutpolicy | gen-host-displaced.py | NOT APPLIED | `🧊️generation3d/🔨️modules/🏠️host/🦀️.rs` still has `owner.close_step(grant).expect("cold displaced replay close")` (no `.result`). The generation2d host file has no matching line (no-op). |
| mutpolicy | raster-model.py | NOT APPLIED | All 3 edit sites still old: `🗺️owned-map/🦀️.rs` line 442 `return RetirementStep::Complete;`; `🖼️asset-content-and-patch/🦀️.rs` line 226 `match ...admit_owned_retirement(value, birth) {` without `.result`; line 238 `owner.close_step(grant).expect(` without `.result`. |
| rtpanes | process3d-pane-smoke-target.md | NOT APPLIED | `✏️editor/📦️packages/🦀️rust/Cargo.toml` has no `process3d_pane_smoke` `[[test]]` entry. |
| runtime | apply-lifecycle-reserved-return.py | APPLIED | `🏛️ownership/⚠️failure/🦀️.rs`: `reserved_before` and `original_lifecycle_net_after(` present (2 hits each); `🏛️ownership/🦀️.rs:10` re-export includes `original_lifecycle_net_after`. |
| runtime | apply-ack-branch-receipt.py | APPLIED | `🧵️retained-command/🔄️full-operation/🦀️.rs:259-262` contains the new `worker_outcome_pending` branch (`reserved`, `original_lifecycle_delta(before,after)`). |
| stdio3 | decode-denied-close-grant.patch | APPLIED | Real target is `🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs` (the path in the patch is relative to it; my first lookup under 📡️spr was wrong). Line 60 has the new guard `if grant.maximum_items==0||grant.maximum_depth==0`. `git diff HEAD` shows exactly this one-line change. |
| testport | README-dbg.txt | N/A | Debug note, not a patch. Refers to `dbg.py`, which is not in the patches folder. |
| testport | cold-reserve-output.py | APPLIED | `🔁️replay-owned-state-operations/🦀️.rs`: old `ok_or_else(||VcsError::HistoryReplaying)?;control.admit` pattern absent; `None if grant==Self::cold_authority_grant()` present (3 hits). |
| testport2 | reactor-jobs-cargo-protocol-laws.md | NOT APPLIED | `⚛️reactor/💼️jobs/📦️packages/🦀️rust/Cargo.toml` has no `protocol-laws` in the dev-dependency feature list. |
| testport2 | time-travel-instance-jobs-lib-test-cycle.md | UNVERIFIED | Gating of `assert_discard_publication_original` (`♻️retirement/🧪️tests/🦀️.rs`) and of the staging test modules not located. Nothing shows it applied. |
| testport2 | instance-render-tool-clock-feature.md | NOT APPLIED | `🖼️render/📦️packages/🦀️rust/Cargo.toml` line 25 still `artifact-app-testing = []`. |
| ts2d | admission-within-workspace.diff | APPLIED | Both hunks match the tree (`NEW@-1,4` and `NEW@-10,4`): `mkdir` import and `.cache/test-admission` scratch area present in `🔏️admission/🟦️.ts`. |
| ts2d | apply.py | NOT APPLIED | Target `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/🟦️.ts` and top-level `🟦️.ts` still contain `booleanRegions, ` / `BooleanOptions, ` (1 and 2 hits). |
| ts2d | booleans.ts | NOT APPLIED | `🔀️booleans/🟦️.ts` (26655 bytes, Oct 18) differs from patch copy (27095 bytes). |
| ts2d | booleans.test.ts | NOT APPLIED | `🔀️booleans/🧪️tests/🟦️.ts` (15358 bytes, Oct 17) differs from patch copy (16744 bytes). |
| ts2d | paths.ts | NOT APPLIED | `🔀️booleans/🛤️paths/🟦️.ts` (11604 bytes, Nov 1 15:00) differs from patch copy (11519 bytes). Changed before the window, not by a patch. |
| actinfra | component-scheduler-flag-views.patch | OBSOLETE | Patch header targets `🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` (its `+++` line names /tmp/sa-dep.ts, so the diff was made from a temp copy). The file (modified 01:03) now has `export function requireCompatibleCargoFlagViewsV1` (line 75) called from `hold` (line 82). The inline check of the patch is replaced by that function (the brief's fix). Patch not to be re-applied. |

Summary by lane: closure, energy, flowext(geometry), runtime(x2), stdio3, testport(cold), ts2d(admission) are applied. gen, mutpolicy(x2), rtpanes, testport2(x2 of 3), ts2d(apply, booleans, booleans.test, paths) and describe item 4 are not applied. actinfra is obsolete. fem, compute, testport2 time-travel, and most of describe/flowext test-file edits are unverified.

## Stray edits (modified after 2026-11-02 01:00, in 🧰️framework/, ✏️s/, 🌎️hub/, ♻️mit-bestand/; excludes target, dist, node_modules, .cache)

Mtime is the file modification time. Brace balance uses a naive count (strings and comments stripped, so only a rough signal). `git diff HEAD --stat` was read per file where it mattered.

| File (repo-relative, short) | mtime | Matches a patch? | Diff / completeness |
|---|---|---|---|
| 🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs | 01:06 | stdio3 decode patch (APPLIED) | 1 line changed, complete. Brace count shows -1 with naive counter, which the one-line diff does not support: likely a false alarm from the counter. |
| 🧰️framework/.../🔌️plugin/🏛️ownership/🦀️.rs | 01:02 | runtime py (APPLIED) | 1 line (re-export), complete. |
| 🧰️framework/.../🧵️retained-command/🔄️full-operation/🦀️.rs | 01:02 | runtime py (APPLIED) | Complete (branch present). |
| 🧰️framework/.../🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs | 01:07 | testport cold py (APPLIED) | Complete. |
| 🧰️framework/.../🌊️flow/🖥️host/🏠️flow-host/🦀️.rs | 01:03 | flowext md (PARTIAL, applied) | 1 line (`cfg(any(test,feature="testkit"))`), complete. |
| 🧰️framework/.../🌊️flow/🖥️host/⏪️undo-redo-history/🦀️.rs | 01:03 | flowext md (applied) | 1 line, complete. |
| 🧰️framework/.../🌊️flow/🖥️host/🧹️retirement/🦀️.rs | 01:03 | flowext md (not diffed) | Not diffed, brace balance 0. UNVERIFIED. |
| 🧰️framework/.../🌊️flow/🖥️host/✏️host-authoring/🏗️construction-and-registry/🦀️.rs | 01:03 | flowext md (not diffed) | Not diffed, brace balance 0. UNVERIFIED. |
| 🧰️framework/.../🌊️flow/🖥️host/📦️packages/🦀️rust/Cargo.toml | 01:03 | flowext md (testkit features) | Not diffed. UNVERIFIED whether `[features] testkit` is complete. |
| 🧰️framework/.../🌊️flow/🖥️host/🧱️flow-host-model/📦️packages/🦀️rust/Cargo.toml | 01:03 | flowext md (testkit features) | Not diffed. UNVERIFIED. |
| 🧰️framework/.../🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs | 02:20 | flowext md test-file edits (partly listed) | Brace balance 0. Modified after the window, so possibly the coordinator or a live agent. Not diffed. |
| 🧰️framework/🛍️products/🦑️repo/.../📥️dependencies/🟦️.ts | 01:03 | actinfra patch (OBSOLETE) | Holds the brief's fix `requireCompatibleCargoFlagViewsV1`. Brace balance 0. Not a patch. Not covered by a unit row yet (brief). |
| 🧰️framework/🛍️products/🦑️repo/.../📥️dependencies/🧪️tests/🟦️.ts | 02:18 | none | Brace balance 0. Unmapped; likely the unit test for the fix. Review needed. |
| 🧰️framework/🛍️products/🦑️repo/.../📥️dependencies/🧪️tests/📥️consumption/🟦️.ts | 02:18 | none | Brace balance 0. Unmapped. Review needed. |
| 🧰️framework/🛍️products/🦑️repo/.../📥️dependencies/🧪️tests/🧫️fixtures/🎯️flag-views/🔣️.json | 02:18 | none | JSON fixture, unmapped. Review needed. |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/.🧾️store/support.json | 02:21 | none | Hidden store file, unmapped, most recent. Looks like tool output, check before close. |
| ✏️s/.../🪵️sourcing/... (closure Cargo.toml, ~17 files) | 01:0x-01:1x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../🌍️gis/... (closure Cargo.toml, ~17 files) | 01:0x-01:1x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../🎬️sequence/... (closure Cargo.toml, 5 files) | 01:0x-01:1x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../🏭️process/... (closure Cargo.toml, 5 files) | 01:0x-01:1x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../🗄️stdio/... (closure Cargo.toml, 8 files) | 01:0x-01:1x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../🌀️procedural/.../🧊️generation3d/📦️packages/🦀️rust/Cargo.toml | 01:0x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../🌊️flow/.../♻️retirement/🧪️tests/📦️packages/🦀️rust/Cargo.toml | 01:0x | flowext md (APPLIED) | Has testkit and unit_tests dep. Complete. |
| ✏️s/.../🌊️flow/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs | 01:03 | describe/flowext test edits (not mapped exactly) | Brace balance 0. Not diffed. UNVERIFIED. |
| ✏️s/.../🌍️gis/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs | 01:08 | none exact | Brace balance 0. Unmapped test edit. |
| ✏️s/.../🌍️gis/.../🧰️owned/🧪️tests/🔬️unit/🦀️.rs (gis host) | 02:23 | none | Brace balance 0. Unmapped and modified after the window. |
| ✏️s/.../🏭️process/.../🏠️host/🧰️owned/📦️packages/🦀️rust/Cargo.toml | 01:0x | closure patch (APPLIED) | Complete per hunk check. |
| ✏️s/.../📐️cad/🧩️extensions/🔥️aec-building-energy/🧪️tests/🔬️unit/🦀️.rs | 02:19 | none | Brace balance 0. Unmapped, modified during this audit. |
| ✏️s/.../📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🔬️unit/🦀️.rs | 02:19 | none | Brace balance 0. Unmapped. |
| ✏️s/.../📐️cad/🧩️extensions/📐️spatial-shape/🧪️tests/🔬️unit/🦀️.rs | 02:19 | none | Brace balance 0. Unmapped. |
| ✏️s/.../🗄️stdio/.../🏗️ifc/🔎️details/contextual_render_tests.rs | 01:10 | none | Brace balance 0. Unmapped (ifc, outside freeze closure). |
| ✏️s/.../🗄️stdio/.../🏗️ifc/🔎️details/✏️edit/contextual_render.rs | 01:10 | none | Brace balance 0. Unmapped. |
| ✏️s/.../🔋️energy/.../🌉️bridge/🦀️.rs | 01:06 | energy py (APPLIED) | Struct and impl present, complete by grep. |
| ✏️s/.../🧬️schema/.../🌉️... (closure etc.) | 01:0x | closure patch (APPLIED) | See closure rows. |
| ✏️s/.../Cargo.lock (fem 3d, space, procedural generation3d, cad, gismap, gltf, sourcing curation) (7 files) | 01:0x | none (lock churn from closure Cargo.toml edits) | Lockfile follows the closure Cargo.toml changes. Consistent, but not cargo-checked (no cargo runs). |
| ✏️s/.../🧊️process3d/.../📦️packages/🦀️rust/Cargo.toml (schema, io, host owned) | 01:0x | closure patch (APPLIED) | Complete. |
| 🌎️hub/Cargo.lock | 01:0x | none (lock churn) | Unmapped, consistent with closure edits. |
| ♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorserveclosure/🟦️.ts | 02:34 | none | Brace balance 0. Test dir (allowed), unmapped, modified during the audit. |

Notes for the coordinator:
1. No file in the stray set has unbalanced braces by the naive count except the decode file, which the one-line diff clears.
2. The mtimes after 02:07 (support.json 02:21, dependencies tests 02:18, cad unit tests 02:19, gis host test 02:23, demonstratorserveclosure 02:34, flow host unit test 02:20) are newer than the dead 1101 coordinator. They may be live work from the current session and need an owner check before any cleanup.
3. Not checked: no cargo or bun run, no test run. All verdicts are text-level.
