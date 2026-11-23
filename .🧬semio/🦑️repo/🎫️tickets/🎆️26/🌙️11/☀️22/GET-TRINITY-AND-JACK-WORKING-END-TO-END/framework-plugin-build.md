# Framework Plugin Build and Puzzle 5D Check (draft, in progress)

## Method
- `repo` is broken, so cargo runs natively through `cargo-native.sh` (ticket folder): pinned nightly toolchain, `CARGO_BUILD_RUSTC=rustc`, empty rustc wrapper, `CARGO_INCREMENTAL=0`, target/build/tmp dirs on `G:` (`G:\t\nrp`, `G:\t\nrpb`, `G:\t\tmpp`), `-j 8`.
- Generated inputs are produced natively with `generate-styling.ts` and `generate-graph.ts` (ticket folder).

## Task 1: framework crates (verified)
- `semio-plugin`, `semio-plugin-instance-render`, `semio-plugin-instance-commands`: `cargo check --all-targets` passes.
- Only error seen: lib test import of removed `assert_latest_history_preview`/`assert_mounted_history_overwrite` in `🔌️plugin/🦀️.rs:108`; another agent removed it within minutes (no edit by me).

## Task 2: Puzzle 5D (workspace `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d`) - not yet green
Root causes found and fixed:
1. Stale generated files: `ui-styling` tokens (motion constants) regenerated; graph manifest registries generated for puzzle `2d`, `3d`, `5d` (`🤖️generated` is git-ignored).
2. Wire API drift: editors in `2d`, `3d`, `5d` called the non-existent `with_original_binary_receiving_and_comparison(...)`; replaced by `with_original_pack_value_receiving()` (5d: also dropped the unused `prepared_operation_wire_source` import).
3. PDF plugin blockers on the dependency path: borrow error in `🔆️image-preview` (`inferred` hoisted before the match); `page` module lost its public re-export (`pub use ..._page_editor_base::*`).
4. Document records: `Puzzle5dFastener` and `Puzzle5dKindCompatibility` missed `DslReceiving`.
5. Test-only serde: record and mutation types derived serde under `cfg(test)` only, so dependent test crates failed. Now `any(test, feature = "testkit")` in `document-records`, `kind-catalogs` and 42 mutation leaf files; `testkit = []` features added to document-records, kind-catalogs, mut-any, mut-any-2/-3/-4/-part2d, diff-2, diff, mut-any-5; dev-dependencies enable them in the affected test crates.
6. Test code drift: window unit test retirement helper (`drive::drive`), proximity-connect test imports, `serde_json` vs pack `Value` comparison, `Op` inference annotations (`::<Puzzle5dMutation>`) in the mutations unit test.

7. Further test drift in the 5d host-admission tests: `ColdMutation` calls, `pending_effects` (async, needs the fixture cx), `host_turn` result pages via `take_fixture_result_page`, `testkit::FIXTURE_INSTANCE_ID` constant shared with `bind_instance_id`.

## Verification
- `cargo check -j 8 --workspace --all-targets` in the 5d workspace: `Finished`, `EXIT 0` (`puzzle-full9.log`).
- Transient failures by other agents' in-flight edits (instance-app Boxing, instance-construction dependency, layout contract, math exports) cleared on retry; not edited by me.

## Open
- `cargo test -j 8 --workspace --no-fail-fast` for the Puzzle workspace (started 17:24, wrapper `sh` PID 13544, cargo PIDs 33996/38824, target `G:\t\nrp`) is still compiling test binaries under heavy host load as of 22:53 (log `🗑️generated/puzzle-test1.log`, ends with `EXIT <code>` and `DONE`). Not killed by the other agent's accident (verified alive). No test result yet; no compile errors in it so far.
- Tests must be read from that log once `DONE` appears; failures (if any) are not yet known.
- Standalone `-p` runs of other mutation crates may need `testkit` dev-dependencies (feature unification hides it in workspace runs).
