# Flow Store Owned Decoder Audit 46

## Existing Reusable Store Seams

`OwnedSchemaRecordCursor` in [Store](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs) is the fixed-schema outer-record cursor. It validates nonempty unique ASCII field specs (at most 64), enforces required fields and duplicate rejection, emits terminal field-token spans, and reaches `Complete` only after its `Trailing` state receives token-cursor EOF.

The existing ownership stack provides concrete reuse seams for a Flow owner implementation. Sufficiency for every typed partial-field, text, binary, and command path is not yet proven; no duplicate transaction API is justified by this audit:

- `ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>` supplies owner-specific VCS, snapshot, mutation, SPR conflict, and history-entry authorities.
- `ArtifactEnvelopeVcsFieldAuthority`, `ArtifactEnvelopeSnapshotFieldAuthority`, and `ArtifactEnvelopeMutationFieldAuthority` accept bounded source tokens, publish only through a matching target reservation, and expose bounded close/terminal state.
- `ArtifactEnvelopeMutationFieldTarget` reserves, publishes, or cancels a concrete mutation exactly once. `ArtifactOwnedSprMutationTarget` keeps a `ManuallyDrop<Option<Mutation>>` and rejects a live reservation/value at `Drop`.
- `ArtifactEnvelopeFreshFieldDecoder` is the current fresh-envelope consumer. It creates the VCS authority through the catalog, tracks the target reservation, then releases every owner through bounded close steps and verifies terminal state before returning its lease.

## Flow Mount Status

A constrained read of the actual framework Flow and plugin Flow source roots found no `ArtifactEnvelopeOwnedFieldCatalog` implementation, no `begin_vcs`/`begin_snapshot`/`begin_mutation` catalog mount, and no Flow-specific history-entry decoder registration. Therefore Flow can reuse the Store contracts but has not yet supplied the domain authorities that decode its snapshot/mutation shapes. Generated partial-field ownership is absent: the Store only calls catalog-owned boxed authorities; it does not generate one from a Flow serde type or schema.

## Grammar And Safety Gaps

1. The outer cursor has an exact EOF rule: after a valid object end it enters `Trailing`; a further token faults as `schema-json.trailing-token`, while `OwnedSchemaTokenStep::Complete` in `Trailing` alone yields `Complete`.
2. A trailing comma is nevertheless accepted. `Separator + Comma` transitions to `Key`, and `Key + ObjectEnd` accepts a complete record. Thus `{"schema":"x",}` can complete if required fields are already seen. An explicit `AfterCommaKey` state is absent.
3. Schema key comparison is lexical raw-byte matching, not JSON-string semantic matching. `string_token_equals` requires token byte length `expected.len() + 2` and compares bytes between quotes. Escaped-but-equivalent keys such as `"sch\\u0065ma"` fail `schema-json.unknown-field`; no unescape/key-normalization path exists.
4. Nested values use delimiter depth and terminal field tokens, but the outer cursor does not make an owner decoder responsible for parsing field-key escapes or trailing-comma legality.

## Retained Ownership On Refusal Or Error

The catalog design keeps ownership reservation-based: a target must be reserved before authority publication, cancellation must use the same reservation, and release paths require terminal emptiness before a borrowed decoder lease is returned. This is the correct reuse seam for Flow refusal/error cleanup. A Flow implementation must still provide each authority's concrete cancellation/retirement behavior; Store cannot infer it from Flow serde metadata.

## History Decoder Boundary

`artifact_bounded_history_entry_decoder<T>()` is explicitly source-bounded. `ArtifactRepositoryHistoryEntryAuthority` copies each token span into a fixed raw byte buffer, then calls `serde_json::from_slice` on the terminal token. Its "retained" name refers to bounded raw source bytes and later retirement of the completed typed `T`; it is not generated or incremental typed partial-field ownership. The helper itself is not fail-closed: it actually invokes typed deserialization. Its documentation's cohort restriction must not be read as an ownership guarantee, and it must not receive partial Flow cleanup credit.

## Recommended Flow Reuse Scope

Implement one Flow-owned `ArtifactEnvelopeOwnedFieldCatalog<FlowFixture, FlowMutation>` and its VCS/snapshot/mutation/history authorities against the existing reservations and terminal-close protocol. Keep Store changes separate: the trailing-comma and escaped-key issues are shared `OwnedSchemaRecordCursor` grammar fixes, not Flow-local work. Do not claim partial typed decoding until Flow supplies concrete authorities for each generated/direct field shape.
