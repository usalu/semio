# FEM engine module crates: check and test report

Workspace: all 17 crates are members of `C:\semio\✏️s\Cargo.toml` (`✏️s/🔨️modules/🏗️fem/⚙️engine/**`).

## What ran
- `repo cargo --store=shared` check parked 53 min in `cargo:build-entry` (and earlier at `door.capacity` on a full C:). Per coordinator, fell back to native cargo.
- Native env: `$T/engine-modules-env.sh` (target `C:\t\engmod`, build-dir `C:\t\engmodb`, TMP on `C:\t\tmp`, `CARGO_INCREMENTAL=0`, rustc/wrapper overridden because `.cargo/config.toml` routes rustc through the repo ingress gate). Earlier G: variant: `$T/native-env.sh`.
- `cargo check -p <17> --all-targets --keep-going`: EXIT 0 (after the two analyses test fixes below).
- `cargo test -p <17> --lib --tests --no-fail-fast` (log `🗑️generated/engine-modules-all.log`): EXIT 0, 18 result lines all `ok`, no failures. Crate counts: static 66, sparse 12, ldlt 11, pcg 27, subspace 17, mesh 18, mesh-derived 13, analyses 30, analyses-assembly 15, analyses-model 8, formulation 10, elements2d mounted-cells 1; algebra, model, payload-turns, elements3d, elements2d lib, test-support have 0 unit tests.
- `SEMIO_FEM_NEUTRAL_RESULTS` must be set for mesh-derived `complete_portable_region_surface_corpus` (the owning `test-surface` command normally sets it); run with it pointing at `C:\t\engmod-neutral\results.json`.
- Oracles: the scikit-fem/numpy/scipy comparisons are Rust tests with embedded reference values (static continuum/plate/elements2d, analyses solid, subspace shift-operator); they all passed. The only live Python oracle, `elements2d/🧪️tests/🌀️mounted-cells/🐍️.py`, has no project target; ran via `repo tools exec python -- -I 🐍️.py`: "Independent NumPy oracle verified 5 cases".
- TS package `⚙️engine/📦️packages/🟦️typescript` is an exports-only shell (package.json + symlinks); TS tests are owned by the TS agent and were not run here.

## Root cause of the 3.5 h hang
`fem_job_graph_checkpoint_resume_preserves_stage_order` drove `FemJobGraph::step_plain` with a bare `StepContext::new(..)`. That context has no original parent source supplier, so the publication step's `with_original_effect` fails ("original caller has no parent source supplier"), the graph yields `Seen::Fault` every turn, and the test loop (which only asserted "not Complete") spun forever. The same cause broke 5 subspace tests (restore close, funded restore, close ladder). Fix: drive them through the framework fixture caller (`OriginalNumericalFixture`), and bound/assert the loops so a regression fails instead of hanging.

## Files changed (all under `C:\semio\✏️s\🔨️modules\🏗️fem\⚙️engine`)
- `🧮️analyses/🧪️tests/🔬️unit/🥩️member-load-scalar-average-and-job-graph-laws/🦀️.rs`: graph tests use `step_plain` for stale/cancel; checkpoint-resume test rewritten on `OriginalNumericalFixture` with `GRAPH_TURN_BOUND = 10_000` and Fault/Cancelled assertions. (An accidental no-op `sed -i '7{/close_job/d}'` also rewrote this file with unchanged content.)
- `🧮️analyses/🧪️tests/🔬️unit/🦀️.rs`: dropped stale `close_job` import.
- `🧮️analyses/🧪️tests/🧮️triplet-csr/🦀️.rs`: `reserve_one_funded` now called through `semio_s_fem_test_support::granted`.
- `🔢️sparse/🎵️subspace/🧪️tests/🔬️unit/🦀️.rs`: thread-local leaked `OriginalNumericalFixture` behind `restore_close` / `test_funded_restore`; publication-fuel test and factor-cursor test use the fixture; removed stale imports.
- `🔢️sparse/🔁️pcg/🧪️tests/🔬️unit/🦀️.rs`: removed stale imports of deleted helpers.
- `🧪️tests/🧰️support/🦀️.rs`: removed legacy bare-context helpers (`close_job`, `close_turn`, `close_payload`, `close_writer`, `close_payload_page_bytes`, `payload_from_pages`, `checkpoint_payload`, `with_context`, `until_done`, `payload_close`) and the unused `StepBudget` import; `OriginalNumericalFixture` is the single mechanism.
- Not mine: TS agent's fuel accounting edits in `🔢️sparse/🔺️ldlt` and `🔢️sparse/🎵️subspace` (included in the runs, pass).

## Open
- Doctests (`cargo test --doc`) were not part of the final run: while another agent's build shared the target dir, rustdoc could not find dependency rlibs (E0463 / "extern location ... does not exist"). All crates have 0 doctests of substance, so not a code defect.
- A `taskkill` by command-line pattern may have ended another agent's cargo run that listed `semio-s-fem-static`.
- `repo cargo` itself remains stuck in `cargo:build-entry`; the native path was used.
- Temporary logs live in `$T/🗑️generated` (not deleted: other agents share it).

## Doctests (final)
`cargo test -p <17 crates> --doc --no-fail-fast` alone on `C:\t\engmod` (log `🗑️generated/engine-modules-doc.log`): EXIT 0, 17 `Doc-tests` sections, all `ok` (0 doctests each, nothing to fail). The earlier E0463 / missing-rlib doctest errors came from concurrent builds replacing artifacts in the shared target dir, not from code. The first Open item (doctests not run) is resolved.
