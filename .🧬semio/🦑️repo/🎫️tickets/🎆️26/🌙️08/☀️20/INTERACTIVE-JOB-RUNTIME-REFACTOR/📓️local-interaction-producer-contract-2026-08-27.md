# Local Interaction Producer Contract

## Logical Types

The new source of truth is `LocalInteractionState`, containing exactly three maps: `selection: Record<string, DomainSelection>`, `activeMode: Record<string, SelectionMode>`, and `activeGranularity: Record<string, string>`. `DomainSelection` preserves ordered ids, granularity, and optional anchor id. Hover remains current ephemeral-local state and is not replayed by a tutorial. No broadcast filtering is applied.

`LocalInteractionCapture` contains `identity` and `state`. Identity binds an app instance, the interaction-store generation, and its exact canonical revision. Transport encodes generation as the native unsigned 64-bit value and revision as 32 bytes; JSON/schema mirrors must represent the generation losslessly as decimal text, not a potentially rounded JavaScript number. The captured immutable root and identity remain fixed across every page.

`LocalInteractionRestore` is a strict union:

- `full`: exact replacement of all three maps. Any absent selection domain is cleared. No unrelated hover or artifact/config state is touched.
- `domains`: a map of domain patches. Each patch has `selection`, `activeMode`, and `activeGranularity`, each explicitly nullable. Null removes that field's domain entry; omitted domains remain untouched. An empty id vector and a missing selection entry remain distinct.

Both commands require the current base identity. A tutorial stores only the semantic three-map state; its historical capture identity must not be reused as authority when seeking later or in another app instance. The renderer uses the current queried/cached native identity for each restore. Staleness rejects without partial publication.

## Tutorial Schema Change

`TutorialUiSnapshot.localInteraction` replaces the incomplete selection-only field with `LocalInteractionState`. Sparse `TutorialUiChange` uses a typed `localInteractionDomain` change containing a domain id and the full nullable domain patch. This captures anchor changes, removed domains, modes and active granularity. Full snapshots are not turned into a sequence of normal `interactionSelect` events. Existing serialized tutorial assets and Rust/TypeScript schema mirrors must change together; no legacy field or dual-reading fallback is planned.

## Transport and Ownership

One `ReadLocalInteraction` request creates a bounded query owner. It emits ordered fixed-size pages, with instance/generation/revision, page ordinal and a terminal marker. This is a response stream, not a heartbeat poll or a renderer-maintained substitute state. Cancellation closes its exact captured Store read lease and page frontier. The same contract serves React and native Shell consumers.

Restore uses paged typed ingress and a retained builder/validation cursor. It validates declared domains, permitted granularities/modes, selection ids and anchor membership against an exact captured topology revision. Invalid or stale data rejects explicitly, without truncating selections to the current mode. One Interaction-lane Store publication adopts the prepared root only after all pages and validation complete; cancellation before that point changes nothing.

Publication authority also binds the captured topology/document revision. A graph change during validation must reject the restore even if the interaction generation and revision are unchanged. The topology source must be an existing immutable index or itself be constructed incrementally; calling the whole application topology callback inside initialization does not satisfy this contract. ACK and retry preserve tutorial sequence while obtaining a fresh current authority identity; they never reuse a historical capture identity.

## Necessary Native Work

The read side can traverse existing native BTreeMap iterators under a captured root, because iteration itself does not recompare long keys. The write side cannot repeatedly insert arbitrary long domain ids into BTreeMaps while reporting one bounded step. The protocol's four InteractionState maps therefore need persistent ordered roots and typed retirement, with ordinary synchronous callers explicitly treated as cold boundaries. The existing OrderedMap lookup/update/retirement primitives are native-approved, but InteractionState adoption itself is not yet implemented or tested.

Existing `validate_state` is a whole-state normalizer: it clamps multiple ids in Single mode and prunes unknown anchors/ids. It cannot be reused as an exact restore publication step. Native restore needs retained validation and a captured topology lookup interface; app topology construction itself must not be silently called as a whole-scene operation inside one step. This is a concrete authority requirement, not a reason to fabricate a renderer field.

## Required Fixture Cases

Full clear of an absent domain; sparse preservation of other domains; anchor-only change; ids containing commas; non-broadcast selection; restored Multiple mode while current mode is Single; Unicode ids and domain keys larger than 4,096 bytes; stale base revision; malformed/duplicate pages; invalid anchor/domain/granularity; cancellation at every ownership frontier; exact replay output and terminal emptiness at grants 1/64/4096. Independent TypeScript/third-party immutable patches verify semantic before/after states, while native tests establish actual producer/publication behavior.

This is the agreed logical design, not a claim that the query or restore routes are implemented.
