# Presentation, Layout, and Architect Program Ownership Audit

## Scope and Decision Rule

This is a read-only source audit of the Animate Presentation, Layout, and Architect Program document roots, their native and external facets, and the editor state reached by their commands and renderers. No source or test was changed and no Cargo build was run.

A parent artifact persists its typed child links. The linked child document persists its own payload in a child store/envelope. A hash-only handle plus a process-local cache is not a child document, and a wrapper that serializes both a child handle and the full child payload duplicates ownership. Configuration used by an exact rendered window belongs to that window; live draft/results state belongs to an exact window transient scope.

| Artifact | Parent document fields | Child/link decision | Editor-state decision |
| --- | --- | --- | --- |
| `s.animate.presentation` | `schema` | Required `presentation` and `animation` child slots | engagement draft: tile-editor transient |
| `s.layout.layout` | layout pages, stories, styles, links, print/data fields | optional authored `backgroundDrawing` child; `referencedModel` is a forward link | blueprint/preview page and camera: exact window; drag ghost/input: transient |
| `s.architect.program` | program registers, including canonical `documents` | `knowledge` and `benchmarks` table children | register, graph, adjacency and report selections: exact windows; search/result caches: window transient |

## P0 — Presentation Persists Child Handles but Loses Their Payload After a Fresh Load

`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` and `📸️snapshot/🦀️.rs` correctly define two required child slots:

- `presentation: ArtifactChild<SemioPresentationSnapshot>`
- `animation: ArtifactChild<SemioAnimationSnapshot>`

They replaced the former parent-inline `source` and `tiles` fields. The parent must keep that shape; reintroducing source and tiles into the parent would restore duplicate ownership.

However, the actual presentation child content is only stored in the static `PRESENTATION_SCRATCH` map in `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs`, at `presentation_child_handle_and_cache` and `presentation_working_scene_for_handle`. The latter explicitly returns a default source and no tiles when its process cache misses. The parent text codec at `.../🚪️io/📸️snapshot/📝️text/🦀️.rs` writes and parses only `[child_id,target]`; the Pack codec follows the same child-handle-only ownership. A document loaded in a fresh process, on another peer, or after cache eviction thus has valid handles but renders a default empty deck. This is data loss hidden by in-process tests that have already populated the static map.

`animation_child_handle` has a presently empty child payload, but it is still a declared child slot. It should be materialized as the one canonical empty animation child entry until a mutation creates non-empty animation content.

### Required Presentation Repair

1. On import, mutation, and fixture construction, write `SemioPresentationSnapshot` and `SemioAnimationSnapshot` to a real child-store/envelope keyed by the emitted child handle. The root snapshot contains only the typed handles.
2. Replace `PRESENTATION_SCRATCH` as the source of truth with a child resolver supplied by the document/Pack load path. A local owner/cache can be a performance cache only; a missing cache must resolve the persisted child or report a missing child, never fabricate the default deck.
3. Update every root, snapshot, and nested-artifact diff facet in `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, and `🛰️.proto` under `.../🧬️schema/` from `source`/`tiles` to typed `presentation`/`animation` handles. The direct sparse diff may omit `animation` only while no mutation changes it; its nested artifact must always include both slots.

`PresentationConfig.engagement_input` in `.../✏️editor/🎚️config/🧬️schema/🦀️.rs` is an unsubmitted command draft, not shared document data. `🎮️commands/⌨️engagement-input/🦀️.rs` writes it and `📤️engagement-submit/🦀️.rs` clears it after emitting an actual tile mutation. Place it in the exact `tile-editor` window's transient state, so concurrent tile-editor windows do not overwrite one another and abandoned input does not become persisted app configuration.

## P0 — Layout's Claimed Drawing Child Serializes a Second Copy of the Child Payload

`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs` describes `background_drawing` as a genuine persisted Drawing child. But `LayoutDrawingChild` contains both `handle: ArtifactChild<SemioDrawingSnapshot>` and `content: SemioDrawingSnapshot`; `background_drawing_child_handle` clones the complete drawing into the parent wrapper, and `background_drawing_content` reads that clone. The layout snapshot printer in `.../🧬️schema/📸️snapshot/🦀️.rs` then JSON-encodes the whole wrapper. This leaves two authoritative locations for one drawing and defeats bounded child retirement and independently addressable child loading.

`background_drawing` is an optional, imported/authored child, not a derived export projection. It therefore belongs in `LayoutArtifact`, `LayoutSnapshot`, and the nested artifact form of `LayoutDiff`, but solely as `Option<ArtifactChild<SemioDrawingSnapshot>>`. Its content must be written to and resolved from the child store. `referenced_model: Option<ArtifactLink>` is different: it is a forward reference, not an owned child. It must remain a parent artifact/snapshot/diff link field and must never carry a copied model payload.

The same helper derives a purported content-addressed child ID from `source_tag` plus canonical drawing JSON. Equal child bytes imported through DWG and SVG receive different IDs. Either source provenance is semantic and must be an explicit child/parent field included in the child payload, or it is not semantic and `source_tag` must be removed from the identity calculation. A content-addressed handle cannot claim to name only the payload while adding unpersisted import-route input.

### Required Layout Facet Repair

The native artifact fields are already structurally correct except for the wrapper payload. After replacing that wrapper with the real child handle, synchronize all artifact, snapshot, and diff facets at:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️.rs,🟦️.ts,🔗️.graphql,🔣️.json,🛰️.proto}`
- the matching `📸️snapshot/` files
- the matching `🔺️diff/` files

The current parity report records stale artifact facets that omit `backgroundDrawing` and `referencedModel`, and contradictory snapshot/diff facets whose field set is only `value`. Regenerate or hand-maintain each facet from the corrected root contract; do not make the Rust snapshot collapse to an opaque `value`, because page/style/frame fields are actual persisted document data.

### Layout Window Ownership

`LayoutConfig` currently combines `active_page_id`, `camera`, `preview_camera`, `drop_preview`, and `engagement_input` in one app-config record. The evidence is direct:

- `🎮️commands/📷️set-camera/🦀️.rs` chooses a camera from a caller-provided surface string and emits a global `LayoutConfigMutation`.
- `🎮️commands/📑️set-active-page/🦀️.rs` globally changes the active page.
- blueprint render reads `config.camera`, while preview render reads `config.preview_camera` in `🎭️modes/✏️edit/🪟️windows/📐️blueprint/🦀️.rs` and `👁️preview/🦀️.rs`.
- `🛬️canvas-drag-over/🦀️.rs` uses that global blueprint camera to calculate a drag ghost and persists the ghost through `SetDropPreview`.

Move `active_page_id` and camera to typed configurations keyed by the exact blueprint and preview window instances. Derive the receiving kind/id from trusted command `ViewModel` context, not `surface_id` supplied in the command payload. Keep the drag ghost and unsubmitted engagement input in a blueprint-window transient scope; both are short-lived interaction data and must vanish when that window closes. A preview may retain its camera in its own persisted window configuration, but it must never write the blueprint camera or another preview instance's camera.

## P0 — Architect's New Table Children Also Have No Durable Child Owner

`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs` converts the `knowledge` and `benchmarks` registers to `ProgramKnowledgeChild` and `ProgramBenchmarksChild`, each a typed Semio Table child. That is the correct parent contract: each child handle belongs in the root artifact, snapshot, and diff as a replacement handle, while the table document owns the row payload.

The current implementation writes the records only into `ProgramKnowledgeWorkingTable` / `ProgramBenchmarksWorkingTable` passed through `ArtifactChild::with_local_owner`. `program_knowledge` and `program_benchmarks` return an empty collection when the local owner is absent. The root snapshot codecs persist handles, so a text/Pack round trip that has no in-memory owner turns non-empty knowledge/benchmark registers into empty runtime data. This has the same cross-process data-loss failure as Presentation.

Write the actual `SemioTableSnapshot` to a child envelope/store at the child ID and resolve it when a root is loaded. Mutation diffs should replace only the handle after the new child entry is admitted. The root must not put the record vectors back inline merely to make text reload work.

## P1 — Architect Uses Three Names for One Canonical Documents Register

The native root and snapshot currently expose `artifacts: Vec<ArtifactRecord>` (`.../🧬️schema/🦀️.rs` and `📸️snapshot/🦀️.rs`), while `ProgramDiff` exposes `documents: Option<ProgramArtifactsDelta>`. Its apply path writes that diff field to `next.artifacts`. The external root and snapshot facets use `documents: DocumentRecord[]` instead.

The public program identity is **documents**, as shown by all three runtime surfaces:

- the editor catalog's `REGISTER_IDS` and register dispatch in `.../✏️editor/🗂️catalog/🦀️.rs` use `"documents"` and map it to the native field;
- inference/search/status surfaces label the same native collection `"documents"` in `.../🧬️schema/💡️inferences/🦀️.rs`;
- the mutation tree is `🧬️schema/🧬️mutations/📃️document/`.

In this greenfield tree, make `documents: Vec<DocumentRecord>` the sole name. Rename the native field/type/delta and all snapshot, diff, mutation, text/Pack, inference, catalog, and fixture references together. The external facets are correct about the public field name but must be checked against the native `ArtifactRecord` fields before a mechanical type rename. Do not preserve `artifacts` as an alias: two public names for one register make field-diff and command contracts ambiguous.

The external Architect Program artifact/snapshot facets also still model `knowledge` and `benchmarks` as record arrays. Replace them with typed Table child handles after durable child-store ownership exists. The external and native nested-diff artifact forms must follow the same field set.

## P1 — Architect Config Mixes Per-Window View State With Transient Derived Output

`ArchitectConfig` stores `active_register`, search state/history, report and analysis JSON, adjacency filter, and flattened graph camera in one app-config record. The window and command paths prove these are not one shared document-wide view:

- the graph window rebuilds `GraphCamera` from `graph_camera_{x,y,zoom}` and `node_graph_viewport` globally replaces it (`🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs` and `🎮️commands/🕸️graph/🦀️.rs`);
- the register window renders the `active_register` selection;
- the adjacency window consumes `adjacency_kind_filter`;
- the report window renders parsed `active_report_json`;
- search, validation, analysis, and report commands write JSON caches, while analysis/report commands separately create persisted `AnalysisRecord` and `ReportRecord` document mutations.

Replace the monolithic app record with these ownership scopes:

| State | Correct owner |
| --- | --- |
| active register | persisted configuration of the exact register window |
| graph camera | persisted configuration of the exact graph window |
| adjacency filter | persisted configuration of the exact adjacency window |
| selected report record/id | persisted configuration of the exact report window; render from the document's `ReportRecord` |
| search query/history, last result, last analysis, materialized report view | exact-window transient state, bounded and discarded on close/replacement |

Commands must receive/use exact trusted window context and emit exact-window config/transient mutations. The result JSON must not be duplicated between a persisted document record and app config. A report/analysis command continues to emit its document mutation for authored provenance, then publishes a bounded window-local result for immediate UI feedback.

## Native and Cross-Runtime Tests Required

1. **Presentation child restart:** create a non-default source and tile collection, save root plus child entries through text and Pack, clear all process-local presentation caches, reload, resolve the child, and assert an identical canvas scene. Assert the root has only the handles and no source/tile payload. Repeat for the empty animation child.
2. **Layout child/link separation:** import a non-empty drawing, encode the parent and child independently, clear local state, and resolve the background via its handle. Assert the root never serializes drawing content, `referencedModel` remains a link with no owned child, and equal canonical drawing payloads receive equal child IDs. Add a separate provenance assertion only if provenance becomes a serialized semantic field.
3. **Layout multiwindow trace:** two blueprint windows and two preview windows set separate cameras and pages. Assert each render sees only its own configuration, drag preview is visible only to its source blueprint window and disappears on close, and document text/Pack bytes do not change.
4. **Architect child restart:** create non-empty knowledge and benchmark tables, save root and child envelopes, clear local owners, reload, and verify commands/inferences read the same rows. Change one table and assert only that child handle/content changes. Test text, Pack, and the independent TypeScript/Ajv schema form.
5. **Architect documents contract:** apply create/replace/delete/reorder to `documents`, then round trip root and diff through native, text, Pack, TypeScript, GraphQL, and Ajv fixtures. Assert there is no `artifacts` field in any public form or operation.
6. **Architect window scopes:** open two graph, register, adjacency, and report windows. Set different camera/register/filter/report selections and run search/analysis in one. Assert all persisted window configurations are independent, result caches do not serialize with the document or leak to the other window, and the authored analysis/report record is still present after reload.

The existing field-parity target should become a blocking test for these three roots after this migration: each artifact, snapshot, and diff must agree on parent fields and typed child/link slots. Passing a field-set checker alone is insufficient; each child test must also prove that a fresh process can resolve the child payload named by the parent handle.
