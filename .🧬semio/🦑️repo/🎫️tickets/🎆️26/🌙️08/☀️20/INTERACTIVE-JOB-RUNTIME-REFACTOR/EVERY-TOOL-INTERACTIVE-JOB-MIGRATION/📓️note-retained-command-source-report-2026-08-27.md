# Note Retained Command Source Report

## Result

The Note live-action census is exactly 36 routes. Nine routes have source-level retained migration credit: `setGridVisible`, `setGridSpacing`, `setCamera`, `setCameraZoom`, `setActiveUtility`, `setLocale`, `engagementInput`, `navigatorEngagementInput`, and `loadRequest`. Twenty-seven routes remain `BatchOnlyPendingRewrite`; the fixture records their exact blockers rather than granting false credit.

The migrated routes use one Note-owned retained factory with exact route keys and publication lanes, resumable wire/checkpoint ownership, bilingual progress/replay previews, cancellation, incremental close, and terminal-empty witnesses. Config publications use an app-owned preparation factory. Artifact preparation seals only through `ArtifactStoreOneItemLiveAuthority::prepare_one_item`; Note does not construct a prepared envelope, duplicate Store digest logic, or depend on `semio-framework-hash`.

The editor now publishes nine exact `bounded_first_step_tool_proofs!` entries matching those nine retained declarations, the framework-required `BoundedFirstStepCommandJobFactory` proof authority, `s.note.note@1/*#editor`, and `note.document`. The official verifier's previous “Migrated declaration has 0 exact bounded reducer proofs” discovery gap is repaired in source without granting proof rows to the 27 pending routes.

`setGridVisible` and `setGridSpacing` are exact Artifact routes integrated through one shared root-scalar engine. Their final command units no longer invoke the generic projection apply triad. The Artifact factory admits only `ChangeGridVisible` and valid `ChangeGridSpacing` on `HistoryLane::Document`; validation, mutation-specific inverse construction, sparse diff extraction, nested post-root materialization, mutation-specific scalar apply, and Store sealing advance as separate bounded phases. Grid spacing preserves the source mutation's positive finite invariant and diagnostic text. The preparation checks operation/generation/base-revision freshness on every turn, emits replay checkpoints after every accepted unit, cancels into the root cursor, retires every partial owner incrementally, and seals only with Store's live authority.

The former process-global `static NEXT: AtomicU64` identifier owner is removed. `NoteIdOwner` is serializable and is scoped to the exact operation, document child, or importer child. Create, duplicate, DWG, PDF, PNG, SVG, and DXF paths now receive an explicit mutable owner.

## Exact Pending Blockers

- The bounded nested `NoteSnapshot` materializer is complete in source. Every Artifact route except `setGridVisible` and `setGridSpacing` still reaches publication only through generic monolithic mutation semantics or has no exact factory admission. Per-route semantic apply must be cursorized and connected to one-item preparation before further Artifact migration credit is honest.
- `moveBlock`, duplicate, patch, and nudge reducers still contain whole-tree lookup/clone traversal below the retained input-unit boundary.
- `inkApplyEvents` splits its nested event input but still clones and diffs the document per event.
- `setActiveExample`, `setFixtureJson`, and `saveDownload` still parse or serialize a complete document in one reducer turn.
- The Store seam is released and Note calls only `authority.prepare_one_item(edit, Arc::new(post_snapshot))`, then reads `prepared.edit_digest()`. No Store API seam remains blocked; Note compiler validation is still waiting for the separate exclusive compiler lease.

## Materialization Cursor Progress

The Note-owned materializer has a 1,024-byte string cursor and an exact typed text-child metadata cursor. It cursor-copies `child_id`, artifact id, artifact kind, standard, and subset; retains `Arc<SemioTextSnapshot>` through `local_owner::<SemioTextSnapshot>()`; rebuilds the handle; and reattaches the same Arc. Wire-only handles preserve `None`. Cancellation retires partial/completed metadata incrementally before releasing the typed owner. A paragraph/run content cursor copies run text and links through the same bounded byte owner and incrementally abandons partial output.

The root cursor now rebuilds the full `NoteSnapshot`: root strings and options; recursively boxed block cursors for text, image, table, math, ink, and group variants; table columns/rows/cell strings; ink points; ordered `BTreeMap` asset traversal without pre-collecting or cloning a key; asset mime/data strings; and every `ArtifactLink` target, role, checkpoint/blob string. It exposes phase/block/asset progress, rejects source-shape changes, supports cancellation at every nested phase, and feeds an incremental retirement owner that empties strings, vectors, maps, links, and typed text Arcs one bounded step at a time.

This is deliberately scoped to Note's typed `SemioTextSnapshot` child contract and is not a generic `ArtifactChild` clone claim. Per-mutation semantic reducers and preparation integration remain the Artifact-lane blocker.

## Language-Neutral Evidence

- Strict fixture: `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️component.json`
- Draft-07 schema: `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️schema.json`
- Test-only third-party oracle: `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧪️component.test.ts` using Ajv and Bun's test runner
- Hostile checks reject process-global IDs, `BoundedArtifactCommandWork`, app-owned digest functions, `semio-framework-hash`, and direct `ArtifactStoreOneItemPrepared` construction. They also pin the typed owner-present, owner-absent, and cancellation witnesses.

Direct command run:

```text
bun test ✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧪️component.test.ts
3 pass, 0 fail, 36 expect() calls
```

No Cargo, Nx, rustfmt, native, or Wasm command was run because the exclusive compiler lease was not transferred. Runtime/compiler status is therefore unverified.

## Files

- `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- Note-only command, mutation-test, diff-test, and importer call sites receiving `NoteIdOwner`
- `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️component.json`
- `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️schema.json`
- `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧪️component.test.ts`
