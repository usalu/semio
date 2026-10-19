# 🔎️ Audit verify-1019g: rs-sweep edits (sample of the 891 touched files)

Auditor `aud-verify7`, read-only (no source edits, no git writes, no cargo). Baseline for every diff: the sweep's first-touch backups in `🗑️generated/rs-sweep-x/backup/` (ledger `backup/ledger.json`). Scratch and classifier output: `🗑️generated/aud-verify7-x/` (`classify.py`, `classify.json`, `graph.py`, `edges.json`, `scc.py`, `layers.py`). Nothing compiled, so all findings are static.

## 0. Coverage

* Classified all 891 files: 222 manifests, 669 `.rs`.
* Counts: 21 files with visibility widenings (85 items); 3 files with `_cold` changes (2 definitions plus call-path edits in flow-host); 11 files with new `extern crate` aliases; 24 files with removed blocks (28 blocks); 205 manifests with 415 added dependency lines; 21 manifests with feature declarations.
* Sample of 60 was stratified by edit kind. Every `_cold`, extern and removed-block file was inspected. Widenings and dependency edges were checked in aggregate, with per-item spot checks.

## 1. Visibility widenings: verdict FAIL (partial)

* Justified: `pub use FemDof` (2d structure) and `pub use Fem3dGumballConfig` (fem3d config), `catalog_entry_label` (puzzle-3d panels), `ArtifactOutputChunks` (plugin declaration job). These are re-exports with real downstream uses (for example `🏭️bridge` consumes `FemDof`).
* Justified, same-crate sibling use: `valid_edit_identities` (durable-group codecs) to `pub(crate)`, and `flow_vcs_step_shift` (cursor-steps) to `pub(crate)`.
* Unjustified: pets crate, about 45 items. Only `watched` has a demonstrated cross-crate user (`🎯️choice/🦀️.rs:9` imports `semio_framework_pets_staging_draft::attention::watched`). Every other pets item's hits are same-crate or ticket scratch copies, so they should be `pub(crate)` again.
* Unjustified, zero external users: `snapshot_and_capture_stages` (`➡️directed/➕️normal/🪣️fill/🦀️.rs:33`, `pub use`), `path_codec` (`🚪️io/💾️binary/🧬️mutations/🦀️.rs:7`, `pub mod`), `pack_load_contract_and_value_frames` (`🪟️window/🎚️config/📥️retained/🦀️.rs:21`, `pub(super)`→`pub`), `reserve_exact_owner_page` and `cold_paged_owner` (`⚙️engine/🧮️analyses/🗂️model/🦀️.rs:31,62`, external hits only in ticket scratch).
* Not verified: `PageAllocation` (`🌱️value/📋️list/🦀️.rs:50`), `window_tab_rect` (`⌨️tui/🖥️chrome/🦀️.rs:421`), `ConfigurationWorker` (`👷️job/🏛️reserved/⚙️worker/🦀️.rs:9`), `write_lease_file` (`🛤️paths-and-lease-files/🦀️.rs:110`), `snapshot_wire_codec` (`📝️text/📸️snapshot/🦀️.rs:99`).

Fix list (widenings):
1. Pets, `👀️attention/🦀️.rs` lines 29, 40, 41, 46, 65, 84, 120, 188, 198, 214, 224, 247, 262, 281, 291, 302, 318, 326, 386, 391, 402, 418, 431: `pub` back to `pub(crate)`. Keep line 53 (`watched`) as `pub`.
2. Pets, `📏️spacing/🦀️.rs` lines 16, 17, 109, 138, 166; `📝️draft/🦀️.rs` lines 16–18, 24, 51, 67, 74–370 (the `pub fn` items listed in the sweep diff); `🗓️schedule/🦀️.rs` lines 26, 31, 55: `pub` back to `pub(crate)`, unless a cross-crate user is found for each.
3. `🚶️locomotion/🦀️.rs` lines 28, 37, 41, 65, 69: `pub use` back to `pub(crate) use`.
4. `➡️directed/➕️normal/🪣️fill/🦀️.rs:33`, `🚪️io/💾️binary/🧬️mutations/🦀️.rs:7`, `🪟️window/🎚️config/📥️retained/🦀️.rs:21`: revert to the pre-sweep visibility.
5. `⚙️engine/🧮️analyses/🗂️model/🦀️.rs:31,62`: revert to private.
6. Run a real cross-crate check (dependents plus `use` paths, not word search) for the five "not verified" items.

## 2. `_cold` to plain renames: verdict FAIL (correctness/naming)

Evidence (git history is path-limited because `git log -S` over the whole repo took minutes):

* At HEAD (`bd9e4e02bd0`) the only definitions are `pub fn inverse_fem2d_mutation_cold(snapshot: &Fem2dSnapshotValue, …)` (2d mutations `🧬️mutations/🦀️.rs:125`) and `inverse_fem3d_mutation_cold(snapshot: &Fem3dSnapshotValue, …)` (3d `🧬️mutations/🦀️.rs:182`).
* The commit `bd9e4e02bd0` renamed the plain function to `_cold` and changed its parameter from the persisted `Fem2dSnapshot` / `Fem3dSnapshot` to the value type `Fem2dSnapshotValue` / `Fem3dSnapshotValue`. The `_cold` suffix is the repo's marker for the value-level variant (`ColdMutationKind`, `ColdMutationDiff`, `assert_*_cold`, `update_field_cold`, `retain_invocation_reply_cold`).
* The plain name existed only before that commit (17 earlier commits in path history). It is not a second function that the sweep merged. There was never a plain function with the value signature, so no two functions with different semantics collide.
* 253 importers pass a value base (`fn before() -> Fem2dSnapshotValue` or `Fem3dSnapshotValue`, about 127 + 126 files). Those calls match the `_cold` definition. They were left on the plain name by the author's commit, so they are the stale side.
* Two importers pass the persisted type and break after the sweep's rename: `✏️s/…/🎮️commands/🩹️patch-material/🧪️tests/🔬️unit/🦀️.rs:44` (2d; `demo()` at line 18 returns `Fem2dSnapshot`), and the 3d patch-material unit test with the same shape. These will fail to type-check against `&Fem2dSnapshotValue`.

Verdict: the rename direction is wrong. It keeps a suffix-less name for a function whose semantics were deliberately marked as the value variant, and it breaks two callers. No merge bug, but a naming and type regression.

Fix list (cold):
1. Restore the definitions to `pub fn inverse_fem2d_mutation_cold` (2d `🧬️mutations/🦀️.rs:125`) and `pub fn inverse_fem3d_mutation_cold` (3d `🧬️mutations/🦀️.rs:182`).
2. Rename the 253 value-base importers' call sites and `use` lines to `_cold` (the 2d and 3d `inverse_fem*_mutation` imports, including the `#[cfg(test)] use … {inverse_fem2d_mutation}` in `🚪️io/💾️binary/🧬️mutations/🦀️.rs:39`).
3. For the two persisted-type callers, decide on purpose: convert the base to the value snapshot (the `🩹️patch-material` tests only need `apply_fem2d_mutation` semantics), or call the persisted inverse if one still exists. Do not rename it to the plain name.

## 3. Added dependencies: verdict PASS (static, with caveats)

* Built the normal-dependency graph from all 4,202 manifests (path and `workspace = true` resolution). Result: 0 cycles among normal dependencies. None of the 230 sweep-added edges (matched by manifest and name) lies inside a cycle. Dev-dependency cycles are allowed by cargo and were not assessed.
* Layering by path category: 0 framework-core to product, 0 framework-core to os-plugin edges. The 116 product to framework-core, 49 s-plugin to framework-core and 21 framework-core to framework-core edges all point in the normal direction. 22 sweep-added edges were classified "unknown" (crate paths outside the classified roots), not violations.
* Caveats: layering is judged by directory category, not exports, so "plugin to framework internals" was not checked at symbol level. Dep names were not compiled. The 4 refused cycles listed in the sweep report are not in the graph (correctly).
* Not verified: that each added dependency for the 11 `extern crate`-aliased files is actually present in its manifest (see section 4).

## 4. `extern crate` aliases: verdict PASS

The aliases added are the repo's established crate-root convention, not compatibility shims for renamed crates. HEAD already uses `extern crate semio_framework_os_kernel as protocol` (1955 files), `as store` (1384), `as dsl` (1330), `as vcs` (173), `as pack` (87), and `extern crate semio_framework_schema as schema` (317). The sweep restored the lost aliases of this same pattern, plus energy aliases such as `semio_s_artifact_energy_model_mutations as mutations` (53 at HEAD). Their presence is what makes HEAD's `store::`/`protocol::`/`vcs::` paths resolve.

Open item: confirm `semio-framework-os-kernel` (and the energy crates) appear in the manifests of the 11 files that gained aliases. Not checked.

## 5. Removed duplicates: verdict MIXED

* PASS, identical: `🔋️energy/…/🧪️tests/🔬️unit/🦀️.rs`. The two 42-line copies in the backup are byte-identical. The sweep removed the second copy. The retained line 23 was repointed to `semio_s_artifact_energy_model_snapshot::energy_snapshot_with_state`, which is `pub fn` at `…/📸️snapshot/🦀️.rs:464`. Dependency presence needs confirming.
* PASS, empty impls: `ColdMutationKind` blocks with empty bodies (`✋️drag-selection2d/🦀️.rs` around line 44, `➕️add-part-grip/🦀️.rs` around line 48, plus about 8 more in puzzle 5d mutations). The retained impl is kept.
* NOT identical, decided by copy choice: `🛡️guards/🦀️.rs` (fem3d, HEAD 467 lines). The file held two concatenated copies. The sweep kept the second, which adds `entity_identity_breach` checks to node, material, section, solid, element, load and combination, and uses value types. The removed first copy used persisted types (`node_breach(node: &FemNode)`, `material_breach(&FemMaterial)`) and lacked those checks. Semantics changed. This is defensible (the kept copy is the superset, newer by position), but it must be recorded as a deliberate choice. Callers of `node_breach`/`material_breach` were not confirmed (the word search was inconclusive).
* FAIL, broken imports: the import dedup removed live `use` lines in 6 files. Confirmed in 3 of them that the leaf is still used and no glob import covers it:
  1. `🏪️store/🔄️sync/🧪️tests/🔬️unit/💾️folder-storage-round-trips/🦀️.rs`: line 81 `native_actor::ArtifactActor::new(` has no import. The sweep backup had `use semio_framework_os_store_sync_native::native_actor;` at line 7. HEAD also lacks it, so this was broken at HEAD and the sweep's edit did not fix it.
  2. `🏪️store/🔄️sync/🧪️tests/🔬️unit/🕸️hostile-binding-heartbeat-and-wire-fixtures/🦀️.rs`: `native_actor` (same pattern).
  3. `🏪️store/🔄️sync/🧪️tests/🔬️unit/♻️rebootstrap-supersession-refusal-and-presence-corpus-laws/🦀️.rs`: `native_actor` (same pattern).
  4. `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/⚖️slot-and-publication-fairness/🦀️.rs`: `VcsArtifactApp`, `Emit`, `EphemeralEmit`, `ChildContentView` (import removed, leaf still used).
  5. `🛢️db/🔄️sync/🧪️tests/🔬️unit/🔁️wal-command-decoder-and-replay-laws/🦀️.rs`: `ArtifactId`.
  6. `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🥩️member-load-scalar-average-and-job-graph-laws/🦀️.rs`: `Seen` (`use semio_s_fem_test_support::Seen;` removed).

Fix list (dedup):
1. Re-add the imports above. Check each target crate is a dependency of that test crate first. Items 1–3 need `semio-framework-os-store-sync-native` in their manifests.
2. Items 4–6: confirm each leaf is still used, then restore the import.
3. Re-run the removed-import leaf check over all 413 removed `use` lines once the tree compiles. The static check had 13 hits, 7 of them real defects above and the rest likely false positives (for example `Binary`, `sqlite_control`, `birth` appear as string or binding names).
4. Record the `guards` copy choice in the sweep report.

## 6. Open items

* Nothing was compiled. Every verdict is static.
* Widening cross-crate checks used word search, which also hits ticket scratch copies; the "not verified" widenings need a manifest-level dependents check.
* The `_cold` history used path-limited `git log`. Full `git log -S` over the whole repo was too slow to run.
