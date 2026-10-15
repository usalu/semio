# Replication Layer Frontier — Current Audit

Read-only source audit on 2026-10-15. No builds, locks, source edits or git mutations were performed. Physical measurements below are not compiler Source census results.

## Result

The smallest immediately truthful cut is **replication identifiers**, with identifier definitions, their ToValue/FromValue implementations, and their RetireOwned implementations moved together. `🆔️ids/🦀️.rs` is 12,017 physical bytes and depends downward on Value; it has no codec, mutation, format, wire or transport dependency. Move the MutationId, ActorId, ArtifactId, SchemaId and HybridLogicalTimestamp retirement implementations out of `🧬️retirement/🦀️.rs` into this owner. Moving just definitions leaves illegal orphan implementations: both Value's trait and the identifier type would be foreign to replication. No compatibility barrel should remain; real consumers must import the new owner directly.

An even smaller independent source/sink crate is possible (`🚰️source/🦀️.rs`, 2,844 bytes, PackRefusal only), but it does not remove the identifier-only consumer frontier. Both are valid real packages, neither solves the large remaining replication SCC by itself.

## Concrete Cycles

- Codec → io: ByteStorage::Paged, ByteSpan::from_source and ByteReader::from_source name `io::binary::operation_bytes::OwnedOperationBytes`. IO → codec uses its readers/writers. Scalar → codec; retained codec code calls scalar. Extracting the whole current codec directory as a lower crate fails.
- Crypto → wire: RecordHasher/Signer/SignatureVerifier return ProtocolError imported from wire. Wire reexports crypto. A lower crypto contract must own/use a lower error contract rather than depending on frames.
- Mutation → format/wire: TransactionRef identity construction writes varints through wire and hashes using format::Blake3Hasher (mutation root around lines 1269–1273). IO consumes mutation contracts. Mutation's default operation contracts and canonical helpers must be checked together rather than moving public data alone.
- Causal → IO, mutation, format and wire; IO's causal codecs consume causal. Causal envelope/history vocabulary and fold algorithms need separate ownership from their encoded forms.
- Format → wire; wire/local-interaction → IO/causal; IO → wire. ProtocolError, limits, record constants and primitive helpers currently hidden under wire create false upward dependencies in addition to actual frame dependencies.

Consequently codec/scalar/io/mutation/causal/format/wire form a real cross-domain knot. The aggregate directory import list is not itself a type-checked SCC proof: root glob reexports hide many symbol edges. The explicit cycles above are direct source evidence.

## Canonical Ownership

RetireOwned implementations must live with each concrete type: identifiers in identifiers, map/message/prepared deltas in their mutation owner, envelope/history types in causal vocabulary, conflict types in conflict vocabulary, and local interaction root types in that owner. A separate retirement integration crate cannot implement Value's foreign trait for foreign types. The current root retirement file is 9,946 physical bytes and mixes these owners.

Keep generic extension traits in the lowest contract crate that defines them; keep their default methods only when those methods depend downward. Move canonical codec behavior to a codec implementation owner, or make the dependency explicit through repository-owned seams. Codec implementations of foreign traits must live with the local trait or local concrete type; an integration crate cannot arbitrarily collect foreign/foreign impls. This is why a data-only cut of mutation/causal followed by unchanged impls in the former giant crate is invalid.

## Eventual Acyclic Graph

Dependency arrows point to prerequisites:

1. Value, diagnostic, PackRefusal, hash, base64 are existing independent foundations.
2. Identifiers → Value; source/sink → PackRefusal; protocol errors/limits → diagnostic/PackRefusal; byte storage → Value retirement/control. Move OwnedOperationBytes into byte storage with its ownership/retirement behavior, rather than retaining codec → operation IO.
3. Codec primitives → byte storage/errors/Value; scalar codecs → primitives; dictionary → primitives; crypto contracts → lower protocol errors. Compression implementation remains separate from the primitive contract where appropriate.
4. Mutation contracts → identifiers/Value/mutation-diff; causal vocabulary → identifiers/mutation vocabulary; conflict vocabulary → causal/mutation. Give identity construction a direct hash+primitive owner rather than format dependency.
5. Operation and causal codecs → their vocabulary + primitives; history fold/index → causal vocabulary; format → source/sink + crypto + primitive codecs + record vocabulary.
6. Frame/local-interaction transport → causal/operation codecs and contracts; replica/server consumers select actual owners.

Each numbered stage can contain several narrowly scoped packages, not one crate per stage. Existing mutation-diff already demonstrates a real extracted owner. Nx should depend on these built artifacts and package-local source inputs: edits to a frame codec must not invalidate identifier or mutation-contract artifacts. Shared path-mounted source across crate manifests would not accomplish that ownership change.

## Physical Size Upper Bounds

All Rust files recursively beneath replication: 117 files / 1,648,710 bytes. Excluding any path containing the test marker: 63 files / 1,077,433 bytes. These include independently packaged mutation-diff and physically present but potentially unmounted sources; they are **upper bounds**, not reachable rustc inputs or active-line counts. They must not be used to declare the 262,144-byte / 32-file / 4,000-active-line compiler Source caps passing.

Non-test domain subtrees, in bytes: codec 61,725 (5 files); scalar 14,041 (1); dictionary 15,112 (2); source 2,844 (1); identifiers 12,017 (1); crypto 1,150 (1); IO 97,882 (10); mutation 259,524 (18); causal 187,629 (6); conflict 32,232 (3); frame/local interaction wire 286,709 (6); primitive wire 23,539 (2); format 54,580 (3); retirement 9,946 (1); peer overlay 8,769 (1). These are physical subtree counts including comments, not proposed package closure sizes. Frame/local interaction wire alone already exceeds the byte cap, so an eventual single “transport” crate is insufficient.

## Consumer Frontier

- Actual Actor → Value cleanup is already handled by the root task and was not edited here.
- UI contract non-test sources use only `protocol::value` paths in this audit. Their direct owner is semio-framework-value; redirect the package source imports and Cargo/Nx edges together. A package-level compiler check is still required before claiming the full dependency removed, including generated/derive paths.
- Tool-run receive limits use `protocol::codec::HistoryReadLimits`; this belongs in a small codec limits owner, not a full replication edge. It is not an identifiers-only consumer.
- Mesh-engine explicitly reexports causal HistoryFoldIndex/HistoryFoldSet and iterators. It requires a fold/index owner and cannot be honestly changed to Value alone.
- Surface node-graph was observed importing SelectionMethod and test DomainHover/DomainSelection. Those are local interaction vocabulary, not identifier/scalar ownership; coordinate current edits before changing its package edge.
- Pack mounts/reexports protocol codec primitives and uses codec/dictionary/source functionality. It needs the real codec/source decomposition; changing only a Value alias does not remove its replication dependency.

No compiler Source census or runtime test was run by this audit. The next implementation should land the complete identifiers owner first (including retirement), then split the byte-source/primitive/error cycle before attempting mutation/causal transport cuts. No cap increases or source exclusions are justified.

## Root Follow-up During Lease Recovery

The real source/sink owner has since been completed as semio-framework-pack-source, including canonical trait definitions, native implementations, direct consumers and both-mode native tests; see pack-source-owner-current.md. The next identifier extraction is still unimplemented. Read-only inspection confirms its current source retains serde derives on MutationId/ActorId/ArtifactId/SchemaId and a local crate::value path. A future extraction must carry the complete concrete-type retirement implementations, use the direct Value owner and resolve actual consumer serde requirements; it must not pretend the current code is runtime-dependency-free. No identifier crate, aliases, lock edits or new runtime dependencies were introduced by this follow-up.
