# Actual Editor Algorithm Activation Boundary

This is current-source research for the active complete-editor goal. The resource stage is concrete progress, with real trace/Boolean document production and passing TypeScript/native pixel owners. The full editor goal remains unfinished.

## Authoritative Current Consumers

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` still recursively flattens the document for editor scene nodes. Its Boolean helper applies only child-local transforms, eagerly flattens curves, filters empty operands and returns empty geometry on kernel errors. Its trace helper eagerly decodes PNG, optionally resizes to asset dimensions, traces synchronously, scales to inferred artboard size and removes contours below area six. These paths still control actual scene output despite the bounded document jobs.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs` calls that synchronous flattener while building the canvas, with gesture preview transforms. Canvas rendering therefore recomputes algorithm geometry in the rendering path; merely adding the document-raster owner did not activate it.
- The same canvas builds path anchor/control overlays from published scene paths and constructs selection transform overlays from source bounds. Geometry publication must also supply picking/bounds/overlays consistently rather than rendering one derived result while selection uses another.
- Existing schema unit tests still deliberately expect swallowed/missing Boolean/trace dependencies to return empty geometry. Replace these expectations with explicit retained-job failure semantics when replacing the actual helper, rather than preserving a legacy fallback.
- Actual Draw export currently exposes PDF/SVG command routes; SVG uses the stdio document bridge and production PNG remains separate/reduced. Complete-document raster verification alone does not prove editor export activation.

## Required Next Integration

Use one retained snapshot-owned preparation→trace→Boolean geometry producer and publish its complete result under current document/revision authority. Keep semantic text records for the existing genuine canvas text renderer; this geometry producer can serve editor algorithm activation before arbitrary-text raster export has real outlines. The raster/export text boundary still needs explicit fonts/shaping; it must continue to refuse unsupported text rather than synthesize glyph boxes.

Schedule actual source/contour/curve/Boolean work under the existing interactive-job policy, retain real progress/cancellation, keep the last complete scene while a current candidate advances, and prevent stale candidates from replacing a newer revision. Success/error/cancel must release every candidate. Transform previews must derive consistent complete world geometry without cancelling independently movable Boolean reference semantics. Selection/picking/bounds and conversion commands must consume the same complete geometry and preserve one undoable logical edit; malformed assets/operands remain visible, named failures.

Do not add a synchronous drain of the bounded jobs inside the canvas projection, fabricate a completed progress value, silently omit unsupported operands, or replace typed source documents with irreversible derived paths just to render them. The desired end state retains editable algorithm parameters and original operands.

Native Draw verification currently has a fresh capacity-recovery attempt **82259**; poll that exact live handle. Prior **89201** failed on filesystem capacity before Draw tests, and the authoritative resource report records its errors and safe ticket-cache cleanup. Browser/editor behavior remains unverified. No new production integration is claimed by this research note.

## Native Canvas Scheduling Audit — 2026-10-03

Draw editor render hooks remain synchronous (`ArtifactEditor::render_with_request_context`). `ArtifactView::take_snapshot_read` transfers one genuine immutable read lease without a document clone; `AppRenderOperationContext` carries the full canonical revision, generation and concrete instance. A pinned async future can soundly own that read and borrow it for `DocumentSceneJob`, but only an actual mounted bounded worker adapter can drive the future: wrapping render in an async signature or draining it with `resolve_ready` supplies no scheduling.

The existing instance maintenance route cannot alone run a new geometry producer: `VcsArtifactApp::maintenance_step` skips the instance owner when its known queues and mounted jobs are empty. Do not install computation only in `DrawingInstanceOperationOwner::maintenance_step` and assume the reactor wakes it.

There is a real first-party reference in FEM2D editor `🧵️session/🦀️.rs`: its `reconcile` takes the snapshot read, records a concrete instance/revision job, emits `CancelJob` for superseded work and `SpawnJob` with isolated placement, and registers a `BoundedJobFactory`. Its `with_live_visual` borrows only a matching completed lease. Editor hooks publish pending effects and explicit mounted job maintenance/terminal laws. This is evidence of the scheduler mechanism, not authorization to copy FEM-specific analysis or expose FEM types in Draw. A neutral retained derived-scene owner should use these exact host capabilities, preserve prior complete geometry during recomputation, and supply bounded admission/retirement and stale publication laws.

Native compile handle 82259 is terminal exit 1 (104 errors), superseding the earlier live/capacity-only status. Typed boundaries and graph catalog are now repaired; full native handle 87382 is live and TypeScript handle 92470 completed with exit 0 (537 tests and strict typing, independent PDF/SVG and field-patch/publication/scheduler checks). Native acceptance remains pending until its actual result.

## Retained Canvas Carrier Check

The existing `Canvas2dScene.snapshot` is a first-party `Canvas2dSnapshotLease`, so a renderer has an immutable packet carrier, but it is not currently sufficient for arbitrary drawings/images: its authoritative capacities are 8 snapshots, 4 pages per snapshot, and 4096 bytes per page. `Canvas2dSnapshotPage::push` can stream byte slices; a published snapshot therefore owns at most 16 KiB. Do not infer that this API solves complete image/path resource publication or silently truncate to its current fixed pages. Determine the actual React/native packet readers and provide an admitted scalable resource/scene owner before routing complete Draw output to it. No UI carrier changes were made in this audit.

Current native handle is 51592. Native 68270 terminated before Draw on shared plugin genesis errors that are already corrected in the current checkout; details and exact terminal evidence are in the native boundary repair report. Do not poll older terminal handles.

## Current Vector Producer Stage

The raster pipeline now shares a complete document vector producer in Rust and TypeScript. Its schema preserves semantic text/images/group/paint metadata while admitting only fully resolved algorithm geometry. Full TypeScript verification passed 551 tests. Native gate 99274 executed 514 tests: 505 passed, 9 failed; the demo DSL map syntax and JSON numeric fixture comparisons were repaired. Native repair gate 49514 finished successfully: 514 tests passed, zero failed or skipped. Canvas activation and full end-user acceptance remain unfinished. See [the complete stage report](🎬️document-vector-producer.md).

