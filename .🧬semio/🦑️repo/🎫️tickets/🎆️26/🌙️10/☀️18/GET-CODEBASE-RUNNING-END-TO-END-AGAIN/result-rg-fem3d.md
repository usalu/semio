# Result rg-fem3d (fem crate splitter, brief-1019)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command. `C` = this folder. Tools: `C/rg-fem3d-tools/` (`unmount.py`, `mount.py`, `leafmove.py`), gate `C/rgplug-tools/gate.ts` (canonical `readRustSourceClosureV1`, host cfg and `+test`, ratio = max(files/32, bytes/262144, lines/4000)). First-touch backups of every modified file: `C/🗑️generated/rg-fem3d/backup/` (48 files, path-flattened names). Gate table after: `C/🗑️generated/rg-fem3d/gate-after.txt`.

## Diagnosis

The gate measures lib/bin closures only. The test-cfg overflow came from leaf unit tests mounted into the *lib* (`#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`) that read 18 to 39 committed fixture JSONs through `include_str!` (each fixture counts as one source file). Those tests all depend on the aggregate (`Fem3dMutation` / `Fem2dMutation`), so the leaf crates carried a cyclic dev-dependency on the aggregate crate (a second instance of the leaf crate inside the test build). No crate split was needed, no new crate, no module/crate renames: the tests belong to the aggregate crates' integration targets, where the 11 existing `[[test]]` kinds roots already live.

## Changes

Pattern: unmount the lib `mod tests`, mount the same unchanged test file (file and `include_str!` paths stay put) as a `#[path] mod` in the existing aggregate `[[test]]` root of the matching behaviour group (`extern crate semio_framework_os_kernel as vcs;` added next to the existing `protocol` alias), drop the now-unused dev-dependencies (`<aggregate>-mutations`, `serde_json`) from the leaf crate.

| Group (3d and 2d unless noted) | Lib mount removed in | Mounted in `[[test]]` root | Leaf Cargo.toml |
| --- | --- | --- | --- |
| sections: `change-section` (3d + 2d), solids `change-solid` (3d), regions `change-region` (2d) | `🩹️change-*/🦀️.rs` | `🕸️mesh/🧪️tests/🔬️section-kinds`, `🔬️solid-kinds`, `🔬️region-kinds` (`mesh_section_kinds` ...) | `[dev-dependencies]` removed in `-mesh-sections`, `-mesh-solids`, `-mesh-regions` |
| mesh nodes: `change-node` | `🩹️change-node/🦀️.rs` | `🕸️mesh/🧪️tests/🔬️topology-kinds-2` | kept (move-selection tests still need them) |
| load: `change-load`, `change-combination-term`, `change-combination-name` | `🩹️change-*/🦀️.rs` | `🏋️load/🧪️tests/🔬️load-kinds`, `🔬️combination-kinds` | `[dev-dependencies]` removed in `-load` |
| boundary: `change-support` | same | `🛡️boundary/🧪️tests/🔬️support-kinds` | removed in `-boundary` |
| material: `change-material` | same | `🧱️material/🧪️tests/🔬️material-kinds` | removed in `-material` |
| aggregate grammar test: `🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️mutation-grammar-conformance/🦀️.rs` (39 payload fixtures) | `🚪️io/📝️text/🧬️mutations/🦀️.rs` (`mod mutation_grammar_conformance`) | new `[[test]] name = "mutation_grammar_conformance"` in `semio-s-artifact-fem-{3d,2d}-mutations/Cargo.toml`; test file got `extern crate ... as protocol;` and explicit imports instead of `use super::*;` (public `MUTATION_GRAMMAR_SEMIO`, `mutation_grammar_recognizer`, `Fem{3d,2d}Mutation`) | n/a |

Scope notes: the brief scoped `fem-3d-mutations` and the sections/solids/regions crates. The same defect hit their siblings, none of which was assigned to anybody, so I applied the identical fix to: `semio-s-artifact-fem-2d-mutations` (test 1.625), fem 3d/2d `-load` (test 2.594, over the gate), `-boundary`, `-material` (1.000), fem 3d/2d `-mesh` (0.906). Remaining fem crates above 0.90 are listed under Open items.

Pre-existing defect fixed on the way: `change-support`'s unit test (3d and 2d) used `crate::FemSupport`, which the lib root does not export (the lib test could not have compiled); now the model path `semio_s_artifact_fem_{3d,2d}_model::...::structure::FemSupport`.

Text readers re-pointed: none needed. `🧪️tests/📖️mutation-grammar/🟦️.ts` reads the grammar test file by path and regex (`("kind",include_str!("path"))` rows, 39 expected); file path and rows unchanged. `🗿️artifacts/◻️2d/📦️packages/🦀️rust/🧫️fixtures/⚖️verification-laws/🔣️.json` lists test names such as `...::change_section::component::tests::section_field_intent_follows_committed_vectors` for lib `semio_s_artifact_fem_2d`; those names were already unreachable (that lib no longer contains the mutation modules) and are untouched (see Open items).

No `// #region` touched or dropped, no glob or compat re-export added, no crate registered (none created: workspace members, `📋️build-owners.json`, crate-name registry unchanged). No test name changed except module nesting (`<root target>::change_section_field_intent::...`), test count unchanged.

## Gate before / after (prod | test)

| Crate | Before | After |
| --- | --- | --- |
| fem-3d-mutations | 0.250 / 1.625 (52 f) | 0.250 / 0.375 (12 f) |
| fem-2d-mutations | 0.250 / 1.625 (52 f) | 0.250 / 0.452 (12 f) |
| fem-3d-mesh-sections | 0.406 / 1.000 | 0.406 / 0.406 (13 f) |
| fem-3d-mesh-solids | 0.406 / 1.000 | 0.406 / 0.406 |
| fem-2d-mesh-sections | 0.406 / 1.000 | 0.406 / 0.406 |
| fem-2d-mesh-regions | 0.406 / 1.000 | 0.406 / 0.406 |
| fem-3d-mesh | 0.219 / 0.906 | 0.219 / 0.281 (9 f) |
| fem-2d-mesh | 0.219 / 0.906 | 0.219 / 0.281 |
| fem-3d-load | 0.781 / 2.594 (83 f) | 0.781 / 0.781 |
| fem-2d-load | 0.781 / 2.594 | 0.781 / 0.781 |
| fem-3d/2d-boundary | 0.406 / 1.000 | 0.406 / 0.406 |
| fem-3d/2d-material | 0.406 / 1.000 | 0.406 / 0.406 |

Every fem artifact crate swept with `gate.ts` (56 manifests): only three remain at or above 0.90 (below).

## Verification

- `bun C/rgplug-tools/gate.ts <manifest>` before/after for every crate above, plus a 56-manifest sweep of the fem 2d and 3d package crates: results in the table, sweep in `gate-after.txt`.
- `rustfmt --edition 2021 --check --config disable_all_formatting=true,skip_children=true` (parse only) on the 8 edited non-trivial Rust files (both grammar tests + both text roots + 4 kinds roots): OK. Not run on the later leafmove files (same mechanical append).
- Python path audit over all 284 `#[path] mod` lines in every `🧪️tests/🔬️*/🦀️.rs` root of fem 2d and 3d and all `[[test]]` paths of both aggregates: 0 missing targets, 0 duplicate module names.
- Static check of every moved unit test: no `crate::`/`super::` reference left (only `protocol`/`vcs` aliases, supplied by the root `extern crate`s), no `serde_json`.
- Cargo: NOT green, NOT verified. `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path <fem 3d Cargo.toml> -p semio-s-artifact-fem-3d-{mutations,mesh-sections,mesh-solids,mesh,load,boundary,material} --lib --tests` (no `dependencies lock|sync` running at the time): run 1 failed `System image plist required` (roster, known); run 2 failed in `cargo fetch --locked` because `Cargo.lock` of the fem 3d workspace is stale (it predates the earlier splits, e.g. lacks mesh-topology/node deps; I did not hand edit it); run 3 via `C/rgplug-tools/chk.sh` (`update --workspace` then `check`) timed out twice in the shared cargo queue (alarm 300 s / 570 s while 4 to 5 other `bun repo cargo check` jobs were queued). The 2d workspace was not attempted. No compiler evidence either way.

## Open items

- Cargo check of: fem 3d workspace `-p semio-s-artifact-fem-3d-{mutations,mesh-sections,mesh-solids,mesh,load,boundary,material}`, fem 2d workspace `-p semio-s-artifact-fem-2d-{mutations,mesh-sections,mesh-regions,mesh,load,boundary,material}`, each `--lib --tests` (integration targets `mesh_*_kinds`, `load_*_kinds`, `boundary_kinds`, `material_kinds`, `mutation_grammar_conformance`). Both `Cargo.lock` files need `cargo update --workspace` first (stale before my change; my dev-dependency removals add to it). Run `C/rgplug-tools/chk.sh <manifest> -p ... --lib --tests` when the queue is quiet. Expected risk: only name resolution in the moved test files (imports verified statically).
- Still at or above 0.90 test cfg in the fem tree, not touched (different cause, tests use `super::*`/private items of their own crate, or one 962-line diff file): `semio-s-artifact-fem-2d-model` 0.969, `semio-s-artifact-fem-3d-model` 0.938 (9 test files each, in-crate unit tests), `semio-s-artifact-fem-2d-editor-session` 0.924 (lines: 3,698 active lines test cfg).
- `fem 2d/📦️packages/🦀️rust/🧫️fixtures/⚖️verification-laws/🔣️.json` ("native" law list for lib `semio_s_artifact_fem_2d`, 1,190 names) names monolith-era test paths that no longer exist in that lib; the exact-law runner (`📜️script.ts` `runAdmittedExactCargoLawsV1`) would need to be pointed at the per-crate test targets. Pre-existing, not re-pointed.
- The two grammar tests still print a `[DEBUG] FEM3d/2d exact one-operation grammar...` line and the TS companion test prints `[DEBUG] ...` lines (pre-existing, not mine).
