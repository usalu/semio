# r11-store-l4 execution report

Gate label `r11-store-l4`. All paths under `🧰️framework/` (`S` = `🛍️products/💻️os/🔨️modules/🏪️store`). Test runs: `cargo test -p semio-framework-os-kernel --lib -- --test-threads=1 <filters>` with `SEMIO_TEST_ARTIFACT_DIR` set (the pack/spr command-transport tests require the caller-authored artifact directory by design; without it they panic at `expect`, which is not a library defect).

## Result
Combined run (filters `os_dsl:: os_pack:: os_spr:: space_ canonical sqlite_snapshot document_codec replay_retirement runtime_seed`): before 660 passed / 8 failed (first run) and 138 passed / 10 failed (space/canonical/codec run); after **668 passed / 0 failed**. `cargo check -p semio-framework-os-kernel --lib` and `--tests` compile clean after the last edit (the only red moments were peers' in-flight value/pack edits).

## 1. replay_retirement corpus law
Root cause: the library already encodes before it applies (`replay_mutations` builds the operation identity from the installed wire before `apply_operation`). The test operation `FailClosedOp` delegated `ArtifactCanonicalJson` to its inner operation and ignored `Fault::Encode`, so the store never saw an encode fault and applied all three operations. Fix (test): the canonical wire of `FailClosedOp` refuses when the fault is `Encode`; the refusal surfaces as `VcsError::NativeEncoding` and the law maps it to `serialize-failed`; `FailClosedOp::RetireOwned` now forwards `controlled_retirement_supported` and `retirement_element_copy_bytes` (store close failed with "typed owner has no controlled retirement authority"). The law (applied 0 / 2, retired == operations + inverses, projections == applied + 1) is unchanged. Passes (1/1).

## 2. Production wire sources
- `S/🧵️operation-wire/🦀️.rs`: new `ArtifactCanonicalAuthoringFactory<P, M: ArtifactCanonicalJson>` (re-exported): borrows the canonical JSON wire from the original mutation, prepares no gesture, owns nothing, leaks nothing.
- `S/🦀️.rs`: `canonical_authoring_store_owners{,_birth_demand}` and `funded_canonical_authoring_store_owners` (bounded catalog + that factory). `space_history_store_owners()` now returns it.
- `S/🧬️schema/🧬️mutations/🔏️canonical/` (new, with `🧪️tests`): `ArtifactCanonicalJson for SpaceHistoryMutation` (all six leaves, `SpaceCheckpoint`, `SpaceAlternative`, `Author`, `HybridLogicalTimestamp`, pins) through one indexed navigator serving the node/key/borrowed-root views in exact serde order. Tests compare the borrowed root, the indexed traversal and the printed document (parsed by an independent serde_json) for every leaf shape.
- `HybridLogicalTimestamp` (`📡️replication/🆔️ids`): added `to_value_controlled` (the default refuses: "0.value owner has no controlled native encoding implementation"), required once a checkpoint timestamp is part of an authored operation.
- Codec reduction store: `ArtifactCodec::{of_authoring, bare_authoring}` plus a private `ArtifactCodecCatalog` strategy (`BoundedCodecCatalog` unchanged default, `CanonicalCodecCatalog` for mutations that name a wire); the thunks are generic over it, so `apply_ops_binary`/`replay_envelopes` install the chosen catalog. `of`/`bare` keep the bounded catalog, which retires but cannot author (a generic `M: OpBinary` has no borrowable wire; apps must opt in through `of_authoring` or their own catalog). The apply-ops law uses `of_authoring`.
- `test_support::plain_document_store_owners` (l1's leaking wire): replaced by `PlainOperationImage<M>`, a `repr(transparent)` view of the borrowed mutation that re-encodes the `OpBinary` image per bounded copy turn and copies it out by offset; no storage, no `Box::leak`. Needs `M: Sync` (added to `plain_*`, `assert_store_roundtrip`, `SubsetRoundtripSpec::Mutation`).
- Passing now: `space_checkpoint_commits_dirty_members...`, `space_checkout_checkpoint...`, `space_switch_alternative...`, `space_vcs_host_meta_document...`, `register_space_documents...`, `document_codec_apply_ops_binary_reduces...`.

## 3/4. Remaining module failures (root causes)
- `canonical_reader_error_after_partial_unicode_output...`: released bytes counted the string retirement's constant frame scaffold (4520 B) on top of the 7 payload bytes; the test now subtracts the measured scaffold of a one-byte text (`text_scaffold_release_bytes`), fixture value 7 kept.
- `canonical_runtime_seed_retains_duplicate_owners...`: a retirement frame frees whole page extents in one turn (5664 B > the fixture's 4096); the helper `drive_retirement_within` now bounds copy/capacity bytes and items by the fixture ceiling and the release by its quote (`fits(grant)`), docstring updated. Fixture unchanged.
- `sqlite_snapshot_framework_space_history_*` x2 (+ space oracle `edit`): Windows `CreateProcess` 32 K command line was exceeded by passing the 64 KiB plan JSON in argv; the plan/edit is now length-framed on stdin ahead of the database bytes (`space-history 🧪️tests`, `🪐️space/🧪️tests/🪶️sqlite/🔬️oracle`).
- `…native_codec_body_has_explicit_io_owner`: stale fixture owner path (store root is mounted as `os_store::component`); fixture `🚪️ownership` updated.
- `os_dsl::fixture_sweep::m5_*` x3: discovery chains still expected the old `🧬️schema/{📸️snapshot,…}/{📝️text,💾️binary}` layout; facets live at `🚪️io/{📝️text,💾️binary}/{📸️snapshot,🧬️mutations}` (constant `SCHEMA_DIR` -> `IO_DIR`, chains reordered nearest-first in `m5-auto-discovery`).
- `os_dsl::paged_dsl_operation_keeps_exact_header_whole_8194_text...`: the encoder now emits text in 256 B chunks, so the probe that waited for one 8194 B write never fired; it now cancels after 256 cumulative emitted bytes inside a progress turn (still asserts typed cancel, prefix >= 256 and < whole, exact prefix).
- `os_pack::control` x2 and `os_spr::io::native` x1: environment (`SEMIO_TEST_ARTIFACT_DIR` is mandatory by design); pass with it set.

## Notes
- `[DEBUG]` println in `…native_codec_body_has_explicit_io_owner` is a peer's, untouched.
- One whole-file `sed -i` on the store file was used for a single-line bound (`SubsetRoundtripSpec`); nothing else was script-written.
- Not touched: store-close deadlocks, hot-path livelock, process aborts (l3).
