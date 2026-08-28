# GIS 3D Config Direct 45

## Root Review — Source Gate Released, Native Unexecuted

Root independently replayed the frozen controller after reviewing the native integration, leaf implementations, actual generic codec, and `MutationKind` supertrait bounds. Result `🧪️gis3d-config-direct-45/🧫️run-AFdPB4/🔣️result.json`: 154 assertions, zero failures. Controller SHA256 remains `3835f48bbef23ed266ae46bf50d17df46fed4e4b824f7671c3ce5ba59241ea52`. The previous 155-check RED includes one missing-file assertion that disappears when its source is present; no assertion was removed from the controller. Five newly authored native tests and seven existing config tests remain uncompiled/unexecuted. This is source/schema/fixture acceptance, not GIS or Plugin native/publication readiness.

The earlier 22/45-check author runs below are retained but do not establish the full claimed source boundary. Root found that the 22-check controller never loaded the diff schema, which incorrectly required `deltas` while Rust used `steps`. The later 45-check controller read the repaired diff schema but still omitted aggregate Rust, the Rust schema sidecar, native fixture consumers, and command/editor source captures. Its reported RED was a stale source assertion after schema staging, not a pre-fix execution of the incorrect diff schema. No native test had run.

Root authored a schema-validated domain fixture, captured the missing actual sources and consumers, checked real descriptor identities and fixed diff/outcome/inverse expectations, and ran the expanded controller before native test mounting. The actual retained result is `🧪️gis3d-config-direct-45/🧫️run-tMaRtf/🔣️result.json`: 150/155 passed, with five exact missing native-consumer/mount/direct-leaf-law assertions. All source hashes were stable. The native integration is assigned to the independent Job-fixture executor; this report remains in progress until root replays it. The controller hash at this RED is `3835f48bbef23ed266ae46bf50d17df46fed4e4b824f7671c3ce5ba59241ea52`.

The domain fixture intentionally distinguishes valid camera JSON from opaque-string codec edge cases. It does not widen the editor's separate admission policy or claim that a hand-written JavaScript state model is a third-party Rust runtime.

## Earlier Author Packet — Limited Evidence

Implemented the approved two-leaf direct cutover under the GIS terrain editor config owner.

- `SetCamera` is now `🎥️set-camera`, tag 0, keyword `set-camera`.
- `SetLocale` is now `🗣️set-locale`, tag 1, keyword `set-locale`.
- The aggregate is a strict, transparent `dsl::Mutations + dsl::DslOps` enum. The previous inline enum, manual `Mutation` implementation, and handwritten binary codec are retired.
- `Gis3dConfigDiff` is sparse and ordered; each leaf only writes its own field and inverses restore the exact prior string. No whole-config diff remains.
- The config serde boundary is strict: both fields are required and unknown/null fields reject. `Default` construction remains explicit and unchanged.
- Removed stale `selectedIds` from TypeScript, GraphQL, protobuf, and JSON sidecars. Rust never contained the field; locale intentionally remains protobuf field 3.
- Updated the exact 3D view and locale command constructors plus terrain editor validation/preflight patterns to the wrapped leaf payloads. Renderer, runtime, lifecycle, gismap config, Plugin/publication, launch, and seed were untouched.

The retained pre-cutover schema/controller red is [result](../🧪️gis3d-config-direct-45/🧫️run-2uxCDO/🔣️result.json): aggregate references exposed the initial leaf-closure mistake and absent leaf-source keyword reads. The final actual-file Ajv 2020 + jsonc-parser gate is [result](../🧪️gis3d-config-direct-45/🧫️run-rxqxLT/🔣️result.json): 22 assertions, 0 failures, stable first/re-read hashes. It validates strict aggregate envelopes, strict config serde schema shape, stale-sidecar removal, canonical leaf keywords, binary-tag schema identity, and independent ordered sparse inverse laws.

Native Rust tests were authored in the owner and both leaves, including missing/null/unknown config serde rejection, sparse inverse restoration, unaffected-field preservation, and text codec round trips. They were not compiled or run: Cargo/native execution remains root-owned.

## Diff Contract Correction

The initial diff JSON incorrectly modelled `deltas`; actual Rust is `Gis3dConfigDiff { steps: Vec<Gis3dConfigDelta> }`, where both sparse fields are optional nullable `Option<String>`. The retained red gate [result](../🧪️gis3d-config-direct-45/🧫️run-EHiuSv/🔣️result.json) exposed the stale source assertion while the corrected schema/vector work was being staged. The fixed diff schema accepts only `steps`, empty/default steps, missing or null sparse fields, both fields, and ordered repeats; it rejects `deltas`, unknown fields, and wrong types. The Rust schema sidecar now also uses `deny_unknown_fields`, matching the strict snapshot boundary.

The final controller [result](../🧪️gis3d-config-direct-45/🧫️run-AkVAnm/🔣️result.json) has 45 assertions and no failures. It reads and rehashes actual descriptors, leaf/aggregate/diff/config schemas, Rust/sidecar sources, and ticket vectors; validates the shared descriptor schema; checks canonical tag/opcode/payload joins; and compares fixed expected sparse/no-op/absorb/inverse snapshots. This remains Ajv/jsonc schema/reference evidence only, not a native Rust execution or a claim that the JavaScript fixed-output model is a third-party algebra oracle.
