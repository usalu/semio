# 🧹️ 1030 Cleanup Lane Result

STATUS: edits done and parse-checked (rustfmt syntax pass over every edited file); type-check BLOCKED by a peer-owned Cargo Source admission refusal (see Blocker). No sub-agents, no git writes.

Scanner: `1030-tools/cleanup/scan.py` (policy2 copy, output `🗑️generated/1030/cleanup-findings.tsv`). Refinements: `cfg(... test ...)` variants, `#![cfg(test)]`, `testing`/`testkit` crate dirs counted as test code, and a fix for single-line `#[cfg(test)]mod tests{...}` (line test flag was taken at line end). Re-measure raised exclusion: NEW unwrap/expect 400 -> 374 before any edit.

## Counts (NEW vs HEAD)

| rule | before | after | remaining cause |
|---|---:|---:|---|
| unwrap_expect | 400 (374 refined) | 244 | 242 inside peer migration dirs (value 121, 2d 59, job 36, pixels 19, rest value/job subpaths), 2 deferred (below) |
| unsafe | 71 | 57 | all 57 in 🌱️value (derive, decode, retained-clone, projection, testing allocator) |
| debug | 7 (+1 ladder fn) | 0 | - |
| allow | 2 | 0 | - |
| test_threads | 1 | 0 | - |

Scope exclusions: whole 🌱️value subtree (not only ♻️retirement: every file there was edited <60 min ago by the peer), ◻️2d, 🔲️pixels, 🧵️job.

## What changed

Unwrap/expect to typed errors (`ok_or_else(..)?` / let-else / `filter`), compile-atomic per file:
- draw scene: added `DocumentSceneError::held()` (prepare) and `PathRasterError::held()` (geometry raster); used in scene prepare, paint, paint/prepare, text, placement, booleans, raster, trace, geometry raster (about 150 sites). `settlement_pending`/`active` (bool fns) rewritten with `is_some_and`/`is_none_or` instead of `?`. `fill/sampling` use the existing `&'static str` error.
- draw editor: geometry job (JobStep::Failed on missing owner), completion payload admit (let-else), export (`held()->Fault`).
- raster editor export, ifc tessellation, generation3d original compute, puzzle5d entities (`map_or(DslValue::Null, ..)`), flow wasm trace, surface compositing, async cancel retirement, pack-json encode tree + wire, plugin lifted retirement, retained-command emission, window typed-pack-load (Copy diagnostic via `*get_or_insert`), window preview (`match` + `unreachable!` inside a reference-returning projection closure, no safe error channel).

Unsafe: `allocate_scope()` helper with SAFETY docstring (diagnostic message), `Slot<T>` allocate/seat helper with docstring (tool-machine typing edit), docstrings on text-splice `admit`, replication set-field `Drop`, and both test `unsafe impl GlobalAlloc for Refusing`.

Debug: removed `[DEBUG]` printlns (layout owned tests: also `event()` reduced to `(ordinal, maximum)`; draw export font and original font tests), `[DEBUG]` panic text in stdio semio-base sqlite test, `DBG1030` file write in puzzle3d action-producer test, and `dbg1030_terminal_clauses` in plugin maintenance-ladder.

Allow: removed `#[allow(unused_imports)]` on the glob import and deleted the duplicate `use crate::raster::*;` in draw raster unit tests. The glob `use crate::{...}` may now warn on unused modules; trim after the first compile.

test-threads: md sqlite final-SQL cancellation test re-executes itself with `--exact`; the extra `--test-threads=1` was redundant (isolation is the child process with exactly one selected test), removed. Other `--test-threads=1` hits repo-wide are old harness conventions outside the modified-since scope.

## Deferred (need a decision / outside lane)

- `dsl-mirror` generation2d and generation3d `print_dsl` (`from_envelope_id(..).expect("valid envelope_id")`): `ArtifactDsl::print_dsl(&self)->String` is infallible and has ~12 implementors (flow host and the `wp1-window` macro share the pattern). Needs a trait change to `Result<String, ValueError>`; not started.
- Peer dirs listed above: re-run `scan.py` after the work-API migration lands.
- `typing/edit` line 39 `request.as_ref().unwrap()` is pre-existing (not new vs HEAD), untouched.

## Blocker (type-check)

Ten batched checks (`1030-tools/cleanup/all.sh`, one per workspace: drawing, framework, root, layout, generation3d, raster, ifc, semio-base, puzzle5d, puzzle3d; `-p <touched crates> --tests --keep-going`) all fail before the compiler with
`Rust crate Source refused ... semio-framework-job ... bytes ... 🧵️job/⏱️context/🎟️source/👶️child/🦀️.rs observedBytes 14728 maximumBytes 4967`.
The file is unchanged since 04:46, retry (`cleanup-fw2`) fails identically, so it is a deterministic job-crate source extent plan problem owned by the job migration peer, not churn. Logs: `🗑️generated/1030/cleanup-*.txt`. Rerun with `bash 1030-tools/cleanup/all.sh` once the job crate admits (wait on `🗑️generated/1030/cleanup-all.rc`). Expect to fix: possible unused-import warnings (raster unit tests, layout owned test unused locals after the println removal), possible `?` inside closures or non-Result fns that the regex pass converted (the scanner pass found no bool-fn cases left).
