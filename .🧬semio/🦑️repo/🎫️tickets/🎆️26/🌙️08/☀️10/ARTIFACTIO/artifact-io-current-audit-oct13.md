# Artifact IO Current Audit — 2026-10-13

Read-only source audit; no test gates or runtime execution performed. Root, framework-products, s, and procedural AGENTS.md were read. Existing ticket lifecycle remains with the coordinating agent. Findings describe the observed working tree, which is changing concurrently.

## Source Coordinates

- `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`
- `GEN` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`

## Confirmed Progress

The original `GEN/🧬️schema/🧬️mutations/💾️binary` owner no longer exists in the inventory. `GEN/🚪️io/💾️binary/🧬️mutations/🦀️.rs` owns actual `OpBinary` implementations for `Generation3dOperationDsl` and `Generation3dMutation`, plus `encode_op`/`decode_op`; it calls `dsl::variants_binary::{encode_tagged_op,decode_tagged_op}` directly. Its semantic enum is in `GEN/🧬️schema/🧬️mutations/🦀️.rs`, with leaf dispatch, inverse/diff semantics, and store integration. This is a real implementation move, not a facade forwarding to an implementation remaining in mutations.

The binary implementation currently imports the lowering carrier and `generation3d_operation_{to,from}_dsl` from `io::text::mutations`. This is physical-to-physical reuse, not semantic codec leakage; the Gen execution owner should decide whether the lowering carrier should become representation-neutral. Do not infer a required duplicate implementation from that import alone. JSON references observed under semantic mutation test folders are test oracle/fixture usage, not runtime violations.

## Highest-Impact Remaining Gaps

1. **Store still owns concrete native codecs outside representation owners.** `OS/🏪️store/🦀️.rs:2177–2193` implements `ArtifactPack` for `serde_json::Value` and `DslValue`; `:12019–12036` implements text and binary codecs for `SpaceHistoryMutation`; `:13094–13104` implements `ArtifactPack` for `protocol::InteractionState`. These are concrete serialization implementations, not intrinsic semantic byte operations. Move their bodies to the actual text/binary IO owners and mount them as IO, preserving store host/application authority separately. In particular, relocating a wrapper that continues calling these old bodies would not complete extraction.

2. **Semantic declaration macros still generate text and binary codec implementations.** `OS/🏪️store/🦀️.rs:3070–3087`, inside `field_set_mutations!`, emits both `OpText` and `OpBinary`; the latter serializes JSON text straight into UTF-8 bytes. `config_record!` invokes this macro at `:3109`, so the mixed ownership is transitive. Split semantic macro expansion from representation-specific codec expansion and explicitly invoke wire expansions in IO owners. The immediate observed consumer is `OS/📡️spr/🧪️tests/⚖️protocol-laws-unit/🦀️.rs:449`; no production consumer was confirmed during this bounded audit. The macro is still a public architectural regression vector even when current invocations are probes.

3. **Domain-neutral vocabulary extraction remains incomplete.** `OS/🏪️store/🚪️io/🦀️.rs` defines `ArtifactDsl`, `ArtifactPack`, document pack framing, and erased `ArtifactCodec` vocabulary in an OS/store owner. `ArtifactPack` embeds registration and SQLite-codec capability hooks. `OS/🏪️store/🦀️.rs:2117` mounts this as `serialization`, and store consumers remain expressed through that namespace. Meanwhile `🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust/Cargo.toml` describes product-neutral identities/payloads/outcomes and has no OS dependency; its source defines `IoError`, `IoOutcome`, `IoEntryDescriptor`, and `IoRoute`, not the snapshot codec contracts. This proves shared neutral IO exists, but does not prove native codec vocabulary has reached it. Extract first-party snapshot/operation capability contracts to neutral framework owners and leave host codec construction/publication/store application in the OS. Orphan-rule comments beside `InteractionState` explain crate co-location, not physical file ownership; moving implementations within the same crate remains possible.

4. **Current ownership checker cannot prove all these gaps are closed.** `LIB/🚪️io/🏛️architecture/🟦️.ts:120–143` discovers framework roots through `🗿️artifacts` or paired schema+IO+snapshot roots. Schema checking examines semantic source and assembly mounts; it does not prohibit concrete codec implementations in an owner's root engine file such as `OS/🏪️store/🦀️.rs`. Its checks recognize explicit impls/macros from a fixed vocabulary, but do not model semantic macros that emit codec impls. VCS `Author` at `OS/🌿️vcs/🦀️.rs:41–42` also derives `CanonicalJsonTree` with `canonical_json(owner = semio_framework_pack_json)` in a file describing itself as pure algebra. Treat that as a specific generated physical dependency to inspect, not proof that every ToValue/FromValue derive is IO. The checker currently does not identify that derive/attribute pair in `schemaRustWireDependencies`.

## Focused Proof Required

- Extend `LIB/🧪️tests/🚪️artifact-io-ownership/🟦️.ts` and its existing language-neutral fixtures to detect root engine codec implementations and semantic macros producing codecs. Existing tree-sitter, TypeScript and picomatch oracles are useful independent controls; current fixture tests mainly establish rule parsing and discovery on miniature trees, not runtime correctness of the moved native consumer.
- Preserve and actually run the Store SpaceHistory wire-witness fixtures (`OS/🏪️store/🧪️tests/🔬️unit/🦀️.rs:269–274`) and semantic/store round trips (`:8672–8687`). Add an exact representation witness for InteractionState and the schema-less value codec after relocation; ownership grep alone is inadequate.
- For `field_set_mutations!`, keep the existing protocol law probe and its text/binary round-trip test beginning after `:485`, but add independently specified bytes/text expectations and an oracle. A round trip can pass when both faces incorrectly share JSON byte transport.
- Validate neutral consumer crate manifests and direct imports after vocabulary extraction. Compilation through `store::serialization` aliases alone does not demonstrate domain-neutral consumer reachability.

No broad tests were run; no passing gates or runtime behavior are claimed. Gen and Actor/Host/Media implementation are owned by other execution agents; replication/snapshot controls remain with the coordinator.
