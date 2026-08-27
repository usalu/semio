# Store Incremental Canonical Edit Sealer

## Status

Source packet delivered on 2026-08-27. The Flow r6 native check compiled the Store canonical module without diagnostics, then stopped on two upstream stdio errors before reaching Flow. The coordinator subsequently ran the exact `canonical_edit::tests::` filter: seven passed, zero failed, 849 filtered, 0.32 seconds execution and 35.84 seconds compilation. Complete app interactivity is not claimed. The subsequent rooted borrowed-map extension has a separate handoff and native gate.

Five implementation/test-source files changed in this packet: Store parent component, new canonical-edit Rust component, strict schema, language-neutral fixture, and the existing root script. No runtime dependency or new executable script was introduced. The source report and evidence are ticket-local.

## Actual Ownership And API

Store exports `ArtifactCanonicalJson`, its typed node enum, fixed-state `ArtifactCanonicalJsonCursor`, `ArtifactStoreOneItemSealer<P, M>`, and its portable checkpoint. Mutation implementations return borrowed typed fields in exact serde order; they cannot submit encoded JSON or a digest. The Store owns the Edit/MutationMeta wrapper traversal and all encoding/hash state.

`Arc<ArtifactStoreOneItemLiveAuthority>::begin_one_item_seal(edit, post, mutation_retirement, snapshot_retirement)` moves the exact edit and immutable post-root into the retained owner without serializing. The owner advances, exposes/takes its prepared candidate, cancels, begins close, retires incrementally, and reports terminal emptiness. `authority.retire()` handles pre-sealer cancellation, including final actor/group strings, under one-byte grants.

The edit is boxed before sealing. A private Store token captures the exact authority Arc, boxed-edit allocation address, post-root Arc allocation address, and the Store-computed digest. Commit validates this token and the immutable semantic authority instead of serializing the edit again. The existing bounded `prepare_one_item` helper mints the same token; its whole-edit oracle remains suitable only for explicitly bounded callers. It is not a large-value migration mechanism.

## Bounded Canonical Work

Each advance emits at most min(grant bytes, 256) actual stream/copy bytes. The canonical maximum is 16 MiB, sufficient for the currently admitted 1 MiB typed footprint even with JSON escaping and forward/inverse overhead; existing 16 KiB and 64 KiB payload domains are not reduced.

The phases are semantic authority validation; canonical length measurement; exact framed digest header; canonical encoding/hash; actor/applied/tail identity byte copies; private sealing. Progress counts both canonical passes, framing, and identity copies. No accumulated-credit gate permits a later whole serialization. Scalars alone use serde_json into a fixed 64-byte buffer to preserve numeric formatting. Strings are emitted bytewise with retained escape state. The stack has 64 fixed frames; excessive depth fails explicitly.

Canonical digest parity is preserved:
`semio.artifact.cursor.v2 | u64be(4) | edit | u64be(edit-id bytes) | edit-id | u64be(canonical bytes) | canonical Edit JSON`.

Checkpoint restoration does not trust supplied hash state. It replays prior bytes with fresh Store-owned state and verifies the exact prefix and position before continuing. Operation, generation, base revision, and a fixed-size fingerprint of actor/group/clock/sequence bind restoration. A moved live owner can continue on another worker without replay; a serialized checkpoint uses bounded replay, not constant-time restoration. The maximum checkpoint overhead is derived from domain framing and four maximum identity slots, not an incorrect hard-coded 1,024-byte allowance.

Store minting already caps actor and group IDs at 256 bytes each. Equal-length semantic identity comparisons are therefore bounded. The sealer also copies its three publication strings bytewise before moving them into the prepared candidate. Cancellation and close retire mutations, snapshots, copied metadata, and final authority strings through exact retained factories.

## Tests And Evidence

The strict draft-07 fixture contains an edit with a string larger than 4,096 bytes; nested mutation values; quotes, backslashes, control characters, Unicode; grants 0/1/2/7/256/4096; expected canonical JSON and SHA-256; all three metadata origins; and 16,384/65,536-byte domains.

Supplemental isolated execution of the actual exported `storeCanonicalEditSealerSelfTests` passes 21 checks: five positive-grant byte oracle cases, nine strict Ajv hostile shapes, six exact live Store source-hostile substitutions, and one Node crypto digest oracle. JSON.stringify is the independent JSON oracle. This does not substitute for Rust runtime execution.

Seven Rust tests passed in the coordinator's exact native run:

1. Large Unicode canonical bytes match serde_json and the language-neutral expected digest.
2. Tiny grants, serialized checkpoint replay, cross-worker owner transfer, and exact digest/byte totals.
3. Stale operation/generation/base/authority, forged prefix/digest, rebound edit/post, and distinct authority Arc rejection.
4. Cancellation at all seven phases, terminal retirement under one-byte grants, and retry.
5. Maximum checkpoint framing/identity overhead acceptance plus one-byte overflow rejection.
6. 16 KiB/64 KiB payloads across owner/contributed/transaction metadata origins, optional fields, and serde_json parity.
7. Final authority actor/group Unicode retirement under one-byte grants.

Targeted `git diff --check` passed. The canonical coordinator-requested command was executed:
`NX_DAEMON=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'`.
The first run failed at the concurrent factory-witness fixture, `owner factory missing-compiler-witness: []`. After its owner repaired the guard, the coordinator reported the canonical Nx self-test PASS 645, including the new sealer checks. The earlier failure remains part of the recorded sequence.

An earlier direct root import also failed before tests during taxonomy validation of `artifact-empty-facet-primary-markdown-v1`, reported at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:764` through `.storybook/scopes.ts:199/226`. Diagnostics rejected contractKind, missing ownerFixedDirectoryContractId, sourceFilename/sourceFileKindId resolution, ticket-scoped destination projection, exact Markdown leaf, NFC destination names, and required projection contract inventory. No unrelated taxonomy was changed.

## Explicit Remaining Boundaries

Typed traversal correctness is a per-domain code obligation, validated against serde_json; this trait cannot prove an arbitrary implementation is faithful or constant-time. Hidden serialization, cloning, collection, or BTreeMap nth scans are forbidden in its methods. The subsequent rooted borrowed-map extension supplies retained native iterators; concrete dictionary-heavy app visitors still require integration and oracle tests.

Flow owns its Config visitor, typed root preparation, mutation inverse construction, and application integration. The shared sealer does not make CAD/Writer/Sequence root cloning bounded; they must adopt the same retained preparation and retirement lifecycle. Child publication is owned by the separate child-publication executor. Native compilation, actual live publication, renderer/runtime verification, and complete large-domain app coverage remain coordinator gates.
