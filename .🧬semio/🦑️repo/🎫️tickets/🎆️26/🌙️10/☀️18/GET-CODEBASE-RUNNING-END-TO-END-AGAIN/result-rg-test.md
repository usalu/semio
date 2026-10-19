# Result rg-test (executor rg-test): test-cfg closures under the source gate, reader refusals resolved

Sonnet 5.5, foreground, no sub-agents, no modifying git command. `C` = this ticket. Tools in `C/rg-test-tools/` (`runtime.py` = `include_str!` to `fixture_text!` rewriter + dev-dependency adder, `convert_closure.py`, `gate-full.ts` = single-crate canonical gate with full error text, `priv.py` = private-item scan, `inc.py` = include path checker). Gate = `readRustSourceClosureV1`, host cfg (+`test`), 32 files / 262,144 bytes / 4,000 lines, ratio = max of the three. Generated output (`🗑️generated/rg-test`) deleted.

## Gate before / after (test cfg; prod unchanged or lower)

| Crate | Before | After | Method |
| --- | --- | --- | --- |
| `teaching-proctor-base` | 2.876 (bytes) | 0.760 | two unit tests read the 309 KB and 262 KB course fixtures at run time (`semio_framework_test_fixtures::fixture_text!`, the crate already has the dev-dependency) |
| `semio-s-plugin-energy-engine-precompute` | 1.376 | 0.293 | exchange corpus (291 KB) read at run time; dev-dependency `semio-framework-test-fixtures` added |
| `...-sim-unit-tests` | 1.311 | 0.221 | chronology (285 KB) + retirement corpora read at run time; dev-dependency added |
| `...-sim-restore` | 1.284 | 0.202 | same chronology corpus; dev-dependency added |
| `...-site` | 1.177 | 0.125 | same chronology corpus; dev-dependency added |
| `semio-framework-repo-dashboard-terminal` | 1.156 (bytes, 4131 lines) | 0.824 | all 17 `include_str!` of fixtures/features/sources in its tests become run-time reads (-43 KB); the pure state machine `🚀️launcher` (614 lines, 39 KB, no sibling dependency, only `run`/`run_with` are used outside) becomes the new crate `semio-framework-repo-dashboard-launcher` (0.153 / 0.208); terminal keeps `use dashboard_launcher as launcher;` so `super::launcher::..` paths are unchanged |
| `semio-framework-replication-history` | 1.131 | 0.788 | the unit file (729 lines, 4 fixtures = 38 KB) is now `[[test]] history_transition_unit` over the public API; `HistoryFoldJob::close_step` and `terminal_is_empty` became `pub` (the only two private names it used); `RequestedAllocator` global allocator copied into that test root; the retirement/drafts/birth private tests stay in-crate; exact-law row in `🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts` re-pointed to the new target |
| `semio-framework-plugin-window-config` | 1.025 | 0.891 | apply-edit tests (22 KB + fixture) are now `[[test]] window_config_apply_edit` (the edit is a pure re-export, public API only); the self-contained whole-record replace edit (98 lines) is the new crate `semio-framework-plugin-window-config-replace` (re-exported by `pub use`, no path change for consumers) |

Census re-run (copy of `aud-census8-tools/crate-gate8-disk.ts`, 8 shards, working tree, 3,468 targets, one shard repeated after a transient `rustup which rustc` failure): **0 refused (prod and test), 0 over 1.00, 141 in 0.90-1.00** (audit before: 8 over 1.00, 12 refused, 151 in band). The two new crates measure 0.153 / 0.208 (launcher) and 0.031 (replace).

## The 12 reader refusals (all now measured)

| Crate | Cause | Fix | Ratio now |
| --- | --- | --- | --- |
| `semio-s-fem-app-surface` | `include_str!` of a fixture moved to `🧬️contract/🧪️tests/🧫️fixtures/🧫️color-input` | path corrected | 0.219 |
| `...-plugin-decl-surface` | `#[path]` of `snapshot_builder_ownership_tests` missed the sibling directory | `../🏗️snapshot-builder/...` | 0.641 |
| `...-os-store-testing-fixtures` | two leaf fixtures mounted `mod tests` from files deleted by the consolidation into `🧪️fixture-timestamped` | stale `#[cfg(test)] mod tests` removed from `set-n` and `assign-n` | 0.719 |
| `...-puzzle-2d-connect-handles-preparation`, `...-puzzle-2d-tests`, `...-puzzle-2d-text-inverse` | `include_str!(concat!(.., $kind, ..))` inside a `macro_rules!` (not evaluable) with stale relative paths; earlier `fixture_text!` paths had one `../` too many | six `↩️inverse/🧪️tests/🦀️.rs` files (text, connect-handles, delete-node, create-node, flag, move-node) read through `repo_text!` with repo-root-relative paths (these files are mounted by several crates, so manifest-relative reads would break); every `vector!(kind, case)` checked to exist on disk | 0.887, 0.932, 0.776 |
| `...-os-store-tests-05` | `#[path]` one `../` short in `bin-05/🔬️unit/🦀️.rs` | fixed | 0.969 |
| `...-os-store-tests-04` | two `include_str!` one `../` short | fixed | 0.834 |
| `...-os-store-durable-group-tests` | two `include_str!` paths wrong (4 ups instead of 5; `../` instead of `../../`) | fixed, then 266 KB (1.015) so the 56 KB of JSON/sources are read at run time (6 files, dev-dependency added) | 0.802 |
| `...-os-spr-laws-tests` | five mutation-law leaves mount `mod tests` from `🧪️tests/🔬️unit/🦀️.rs` that no longer exists (tests live in the `🧬️mutation-laws-mutations-*` crates) | stale mounts removed | 0.406 |
| `...-plugin-tests-14-2` | `CARGO_MANIFEST_DIR + "/../../🧫️fixtures/..."` two levels short | `/../../../../🧫️fixtures/...` | 0.772 |
| `...-os-kernel-db-sync-laws` | `include_str!` one `../` short | fixed | 0.272 |

## Files changed (grouped)

- proctor: `🎓️teaching/🛂️proctor/🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs`, `.../👪️crowd/🧪️tests/🔬️unit/🦀️.rs`.
- energy: `exchange/🧪️tests/🦀️.rs`, `🧪️sim/🧪️tests/🔬️unit/🦀️.rs`, `🧪️sim/♻️restore/🧪️tests/🔬️unit/🦀️.rs`, `📍️site/📅️chronology/🧪️tests/🦀️.rs` and the four crate `Cargo.toml`.
- dashboard: terminal `🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, 6 test files, `🚀️launcher/🧪️tests/🔬️unit/🦀️.rs`; new `🚀️launcher/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`; `🎛️dashboard/📦️packages/🦀️rust/📜️script.ts` (crate list) and `🧪️tests/🧫️fixtures/🎛️dispatch/🔣️.json` (unit list); root `Cargo.toml` (member); `📋️build-owners.json` (`@semio-tech/repo-dashboard-launcher-rs`).
- replication: `🔀️transition/🦀️.rs`, `🔁️fold/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml` (`[[test]]`), `📡️replication/📦️packages/🦀️rust/📜️script.ts`.
- window-config: `🎚️config/🦀️.rs`, `🧬️preparation/✏️apply/🧪️tests/🦀️.rs`, `🧬️preparation/🔁️replace/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`; new `🧬️preparation/🔁️replace/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`; root `Cargo.toml`, `📋️build-owners.json` (`@semio-tech/plugin-window-config-replace-rs`).
- refusals: fem `🖥️app-surface/🧪️tests/🔬️unit/🦀️.rs`; `📜️declaration/🪟️surface/🏗️surface-builders/🦀️.rs`; store timestamped `🔢️set-n/🦀️.rs`, `↩️assign-n/🦀️.rs`; the six puzzle `↩️inverse/🧪️tests/🦀️.rs`; `🏪️store/🧪️tests/🌅️bin-05/🔬️unit/🦀️.rs`; two `🌌️bin-04/🔬️unit/*` files; durable-group tests (3 unit files, graph group tests, outbound-announcement, deferred-reprojection, companion `Cargo.toml`); five `📡️spr/🧪️testing/🧬️mutation-laws/🧬️mutations/*/🦀️.rs`; `🛢️db/🔄️sync/🧪️tests/🔬️unit/🪢️retained-sync-hello-laws/🦀️.rs`; plugin `...-part-4/🎞️host-events-media-cancel-and-download-seam/🦀️.rs`.

## Verification

- `bun C/rg-test-tools/gate-full.ts <manifest>` before/after for every crate above (numbers in the tables); full census re-run as described (0 refused, 0 over 1.00).
- `rustfmt --edition 2021 --check --config disable_all_formatting=true,skip_children=true` (parse only) on 40 edited Rust files: no parse failure. All path dependencies of the 11 edited/new `Cargo.toml` resolve to an existing manifest; `Bun.TOML.parse` OK for the 6 central manifests; `📋️build-owners.json` and the dispatch fixture parse; `bun build --target=bun` OK for the two edited `📜️script.ts`.
- Include/mount audit: every runtime fixture path is derived by script from the real target (`os.path.relpath`) and asserted to exist; every puzzle `vector!` case exists.
- **Cargo: not run.** Other fleets held the shared cargo queue with 37-minute checks for the whole session, and the new crates and dev-dependencies need `Cargo.lock` refreshes first (see below). No compiler evidence for any change here; name resolution in the moved tests (replication unit file, window-config apply test) was verified only by a private-item scan.

## Open items

- Cargo checks (after lock refresh): `-p teaching-proctor-base`, the four energy engine crates (new dev-dependency on the fixture reader; energy workspace lock has no entry yet), `-p semio-framework-repo-dashboard-terminal -p semio-framework-repo-dashboard-launcher`, `-p semio-framework-replication-history --tests`, `-p semio-framework-plugin-window-config -p semio-framework-plugin-window-config-replace --tests`, plus the companion crates of the refusal list (`os-store-tests-04/05`, `durable-group-tests`, `puzzle-2d-*`, `plugin-tests-14-2`, `decl-surface`). `🔒️dependencies.json` needs `dependencies lock` for the two new crates and the new dev-dependencies.
- `semio-framework-plugin-window-config`: `🧬️preview/🧪️tests` and `🏠️catalog/🧪️tests` (still mounted in the lib, 9.7 KB) reference `pack_identity_tests`/`retained_pack_load_tests`, which only exist in the test host crate `plugin-tests-19`; they cannot compile in the lib build (pre-existing, same at HEAD). Left in place because they are what keeps the crate at 0.891; the clean fix is mounting them in `🪝️mounts/.../🎚️window-config/🦀️.rs` once `WindowSemanticPreview` is reachable.
- Surface-builders' `snapshot_builder_ownership_tests` and bin-05's `shared_semantic_source_tests` were unreachable before (wrong `#[path]`); now measured and mounted, they may reveal compile errors of their own.
- Newly measured by the reader (previously refused, now scored, still in the 0.90-1.00 band): `puzzle-2d-tests` 0.932, `os-store-tests-05` 0.969.
- Old exact-law row for `history_invariants_follow_portable_ownership_and_utf8_vectors` named a nonexistent lib target of `semio-framework-replication`; it now names the new test target (unverified).
- 141 crates remain in the band (see `audit-gates-1019.md` section 3 for owners); not touched here.
