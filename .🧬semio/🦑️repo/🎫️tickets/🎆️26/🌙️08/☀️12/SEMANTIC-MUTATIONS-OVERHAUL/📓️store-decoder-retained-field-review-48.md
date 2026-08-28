# Store Decoder Retained Field Review 48

## Scope And Status

Read-only follow-up to the Flow partial-decode reuse audit. No Store production source, catalog, command, DSL, lifecycle, or compiler action was taken for this review. These are source findings, not executed native failure claims.

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`. The exact reviewed implementation names are more stable than line numbers while unrelated test work is active.

## Proven Reuse Boundaries

The outer `OwnedSchemaRecordCursor` reaches Complete only after its Trailing state receives token EOF. `ArtifactEnvelopeDecodeAuthority` calls `finish_record` only after that completion. In the fresh-field consumer, completed VCS state is held in the decoder's own `ArtifactEnvelopeFreshRecordTarget`, not published directly into an application during field parsing. `close_target_vcs` transfers that VCS value into `ArtifactStoreVcsRetirement`. A completed envelope is passed to the completed-record registry only from `finish_record`.

These are useful existing ownership mechanisms, but the current implementations do not yet prove retained error-path safety.

## Exact Error-Path Gaps

1. `ArtifactEnvelopeFreshFieldDecoder::accept_field_token` takes `self.active` into a local `active`, then uses `?` on both `authority.accept_token` and `authority.publish_reserved`. If either returns an error, the local active VCS authority is dropped rather than restored for the decoder's bounded `close_step`. The same local exits on unexpected RecordComplete. A live boxed authority can therefore leave the retained owner before cleanup; an enclosing retained lease cannot recover that removed field.
2. The same method accepts FieldComplete/TokenComplete after publication without checking `authority.terminal_is_empty()` before dropping the local VCS authority. A false completion needs an explicit retained-error law.
3. `ArtifactEnvelopeFreshVcsAuthority::accept_token` similarly takes its active enum before fallible nested-history acceptance. Edits, Changes, Checkpoints and Alternatives authorities are local to that enum; a `?` error exits without restoring them. Snapshot authority itself remains in a separate field, but the active reservation/publication phase is still lost.
4. Fresh VCS final assembly takes initial snapshot and history owners sequentially before later `ok_or_else(...)?` checks. Canonical required-field cursor admission may make those later absences unreachable for a correct catalog, but typed-owner safety should not depend on an untested false-completion assumption. Validate all required owner slots before taking any of them.

By contrast, the reviewed `ArtifactOwnedSprMutationArrayAuthority::accept` borrows its active mutation authority in place during fallible accept/publish, preserving it on those errors. Its remaining cancellation/false-terminal paths still require targeted tests; this comparison is not a blanket acceptance claim.

## Required Next Packet

Reuse the same Store authorities and add domain-owned injected-owner tests for accept error, publish error, unexpected/false completion, completed earlier field followed by later invalid field, and trailing-token rejection. Count real retained owners and prove no early Drop, then drain them through bounded zero/refusal/positive grants. Keep raw source pages, reservation state, and completed typed values separate in the assertions. Cover exact final-owner assembly before moving values.

Do not replace this with a serde/cold decoder, a fake completion certificate, or a second transaction API. The separate cursor grammar preparation does not fix these ownership paths. Production changes must be coordinated with runtime before mounting.
