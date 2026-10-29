# 1028 result sweepvalue

Status: GREEN. 97 manifests / 87+ crates of value, replication, pack, dsl, trace, mesh-engine: native `--lib --tests` exit 0 (chk9), wasm32-wasip2 `--lib` exit 0 (chk10), both after all fixes.

Check recipe: manifest is `🧰️framework/Cargo.toml` (framework workspace), NOT the root Cargo.toml (root run hits a cargo resolver panic "did not find features for semio-framework-value-derive-expansion"). Package list: C/🗑️generated/1028/sweepvalue-pkgs.txt; runner C/🗑️generated/1028/sweepvalue-run.sh N "<args>".
All 97 manifests are members of the framework workspace.

## Fixes
- value-dictionary (lib): equal/🦀️.rs - RetainedEqualityStep import moved to semio_framework_value_persistent; verify_original discards check_binding bool.
- value-native-owned: sealed-shared-receiver - `I:?Sized` on NativeSharedReceiver and its impls (NativeReceive<I: ?Sized> design).
- value-snapshot-read tests: duplicate CLOSE_GRANT import removed.
- replication-edit tests: Edit.inverse is RetainedPublicationRoot - fixtures wrap it.
- replication-history tests: Arc<Vec<u8>> -> input.as_slice().
- value-native-decoding(+tests): ErasedSnapshotRetirement::terminal_is_empty ambiguity qualified in retirement test.
- replication-codec tests: ByteSpan::get returns Option<u8> (no pointer identity).
- replication-wire/conflict tests: payload is CausalPayload (PagedBytes).
- dsl-record typed_receiving: crate::RecordBytes -> semio_framework_dsl_record::RecordBytes.
- replication retained_diff_sequence: drain(post) -> drop(post) (PostFixture closes itself).
- replication-mutation-messages tests: next_capacity_byte_demand removed -> next_demand().
- pack-record-retained tests: RetainedDocumentDriver::one takes range.
- wire-core tests: OriginalWireSourceIdentity.range_start.
- pack-json tests: duplicate global allocator removed; close_step takes the NativeRetirementDecodeControl.

## Gate note
draw-drawing-model has 35 source files (limit 32) when reached through the dev-dep of os-store-composition-open-operation; blocks `--tests` checks that reach it (not in this slice; observed chk1).

## SHARED CHANGE
- 🧰️framework/Cargo.toml: registered member replay/🧪️tests/🔐️authority (peer's half-created crate was a member of the wrong workspace and broke every framework-workspace run with CargoLockRefused); its Cargo.toml workspace path fixed (9 levels, was 10).
- value sealed-shared-receiver: `I: ?Sized` (intended NativeReceive design).

## Draw-model gate follow-up (coordinator request) - CHECKPOINT 22:55 state
- NO split edits were made; tree is compile-atomic. Census (roots: draw-drawing-model + os-store-member-open-store, 376-crate closure) shows draw-drawing-model at 32 files / 239448 B / 2582 lines = 0 violators: a peer already inlined validation/{index,program,source}.rs into validation/🦀️.rs (35 -> 32). Zero file headroom remains.
- Planned clean split if wanted: new crate semio-s-artifact-draw-drawing-validation at 🧬️schema/✅️validation/📦️packages/🦀️rust (lib path ../../🦀️.rs), depends on drawing-model; model drops the `validation` mount (🦀️.rs lines ~818); replace `crate::` with the model crate path in validation; move vector/🧪️preparation.rs (Rust test using DRAWING_ADMITTER) into the new crate; consumers: editor/🪆️child (drawing ws), layout ws (drawing-composition re-export, composition tests, scene/🧵️drawing/🧪️tests/🧵️graph.rs); register in drawing ws Cargo.toml (member + workspace.dependencies), layout Cargo.toml deps, build-owners.json, boundary block like drawing-scene.
- Open-operation crate is really `semio-framework-os-store-member-open-store` in the ROOT workspace (not framework). Verification check chk12 (`--manifest-path Cargo.toml --lib --tests -p semio-framework-os-store-member-open-store`, pid 18858 wrapper) was still queued 58 min; result will appear in C/🗑️generated/1028/sweepvalue-chk12.{txt,rc}. Not killed (about to start).
- Next step: read chk12.rc; if 0 the gate is verified; else fix its errors.
