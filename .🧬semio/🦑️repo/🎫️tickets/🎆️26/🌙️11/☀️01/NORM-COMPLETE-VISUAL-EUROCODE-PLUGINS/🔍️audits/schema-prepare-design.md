# Controlled Input Preparation Design

Source inspection confirms that the runtime requires an actual ActionArgDef descriptor, including recursive ArgSchema object, array and tagged-union metadata. InputSchemaReader currently recursively clones schema trees, resolves references through allocating document lookup, normalizes numeric transport shapes, and allocates descriptors synchronously. JsonSourceCursor funds JSON grammar and DslValue construction only. A wrapper grant around mutation_input_defs or a decoded-only cursor does not supply physical receipts for descriptor construction.

The renderer can consume an immutable shared descriptor catalogue and borrow its selected descriptor. Existing SealedShared already funds immutable shared birth, alias handoff and explicit retirement; SnapshotProjection handles original-bound borrowing. A cache seam can therefore avoid deep descriptor cloning, but a cache populated by mutation_input_defs during a different callback still lacks a genuine controlled construction owner. Such a seam must explicitly accept already-prepared original descriptors and cannot claim to compile schema metadata.

No implementation or test acceptance is claimed by this design note. Runtime and parent owners have been informed of this distinction.

## Native Receiving Reuse

The value native-owned module exposes NativeReceive/NativeReceiver, NativeReceiveSlot, NativeReceivingOriginal and NativeRecordMatcher. These already own typed string, vector, box, record, localized-label and default-value construction plus source binding, allocation receipts, cancellation and retirement. ActionArgDef and ArgSchema currently derive FromValue/RetireOwned but not NativeReceive. An initial compiler can use custom semantic receivers and forward actual copies to these retained slots instead of recreating string/vector retirement.

A fully correct runtime interpreter must also avoid unbounded schema keyword scans in demands(), label glossary OnceLock initialization, allocating numeric shape normalization, and registered_input_schema_document ambient parsing. Existing reader behavior includes allOf, nullable resolution, 32-hop refs, recursive-schema Any guards, numeric-transport normalization, vectors, references, union discrimination and per-locale labels. These are part of parity, not optional fast-path simplifications.

## Compilation Strategy Constraint

A static ahead-of-time descriptor artifact could retain exact source schema/document provenance and use paid JSON plus native typed receiving at runtime; this was raised with the parent as an alternative, not implemented or accepted. It would require a permanent schema compiler and declaration/schema closure coherence gates. It does not make existing arbitrary runtime mutation_input_defs resumable and must not be described as doing so.

## Current Execution Result

No production source was changed and no compiler was implemented in this execution. No tests or builds were run. This is an incomplete design investigation, not acceptance evidence. The full initial schema-to-ActionArgDef producer remains necessary before mounting the norm form render-work. Merely wrapping the old reader, using a decoded-only cursor, assuming an existing prepared cache, or advertising guessed original-construction receipts would not fulfill that requirement.
