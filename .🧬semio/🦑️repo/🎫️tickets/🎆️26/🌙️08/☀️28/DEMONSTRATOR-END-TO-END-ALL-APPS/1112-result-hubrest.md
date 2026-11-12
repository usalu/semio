# 1112 hubrest
## Round 1 (verified)
- stdio-contract-editing-patch lib: decode cursor effect-shaped (`step(effect)`, `into_retirement(effect)` via admit_owned_retirement); canonical JSON: Float32->F32, QuotedUnsigned->U64Text when it fits u64 else refused (ArtifactCanonicalJsonNode, 🛫️encode/🔣️scalar/🦀️.rs:6-22, has no 128-bit quoted node; the earlier blanket refusal was a guess, now fixed). W-hubrest-1 rc0, hub W-hubrest-hub-1: the three crates compile.
- process3d-presence: `process3d_presence_retirement_factory()` (SharedValueRetirementFactory); editor-impl uses it.
- hub-inference-binding: StoreMutationFor bounds.
## Round 2 (VERIFIED)
- W-hubrest-10 wasm hub rc0: stdio epw-snapshot, html-snapshot, tsv-snapshot, wav-inferences, mp4-inferences, avi-snapshot, mp3-schema, contract-editing-patch.
- W-hubrest-11 native `--tests` (stdio workspace manifest; the hub workspace has no dev-deps for non-members) editing-patch rc0. Tests compiled, NOT run.
- store space-history settlement: appended `decode_typed` / `encode_typed` (TypedFileSettlement + owner.receive, same shape as binary-snapshot 🚦️native). Runtime receipt accounting of html encode payload capacity (admit_frontier/record_progress in encode_typed) is unexercised.
- wav/mp4/avi: decode/encode_sqlite_snapshot_native on record_native output-slot shape.
- html: native::decode/encode via decode_typed/encode_typed.
- tsv: record_receiving decode (admission::bind via owner.admit_constructed/record_constructed_progress), record_native encode, validate_sqlite_snapshot_subset ("*" only); preflight uses the loan directly; removed 🫴️receiving module+dir.
- epw: dropped to/from_sqlite_database_receiving + validate_*_decoding/encoding, removed 🫴️receiving module+dir and its test-package mirror.
- mp3: `dsl::original_pack_value_operation_receiving!(Mp3Mutation)`.
- patch tests: added ⏱️effect harness (copy of presence-retirement effect.rs), closing()/stepping() helpers; step/into_retirement/close_one/close_factory_ticket/ControlledRetirement::step ported to effects.
- QuotedUnsigned: now U64Text when it fits u64, refused beyond (no 128-bit quoted node in ArtifactCanonicalJsonNode, 🛫️encode/🔣️scalar/🦀️.rs:6-22).
## Remaining red crates (last full hub map W-hubrest-hub-1; not mine; may have changed since)
| crate | errors |
|---|---|
| stdio-binary-mutations | 1 |
| stdio-semio-base | 17 |
| os-flow-host-read | 97 |
| puzzle-3d-precompute-fill | 117 |
| stdio-dwg | 3 |
| stdio-step-snapshot | 12 |
| fem-2d-editor-model-window | 4 |
Mine from that map: all resolved. Full hub map not re-run after round 2.
