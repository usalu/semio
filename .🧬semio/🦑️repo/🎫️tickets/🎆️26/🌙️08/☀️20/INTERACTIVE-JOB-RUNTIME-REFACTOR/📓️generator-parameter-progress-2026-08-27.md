# Generator Parameter Edit Progress

## Completed Source Prerequisites

The shared canonical reader handoff is complete; canonical Nx passed702. The subsequent Procedural3d immutable generation-root schema/source checks bring canonical Nx to711. Two generation-root native tests are authored, not yet executed.

ProceduralGenerationRoot transparently serializes the same GenerationPlayState wire value but internally shares its Arc allocation. Snapshot, artifact, and diff now use this wrapper. Cold decoder/initializer/replay accesses unique roots through crate-private Arc::get_mut, never Arc::make_mut. Existing generation editing commands explicitly clone as_state outside the future retained slider path. Shared mutation is refused. A final-owner retirement cursor walks owned JSON map/array iterators and retires keys/strings bytewise; tests include arbitrary nested JSON larger than16KiB and keys larger than4096bytes.

The shared FlowGraphCanvasHost slider now dispatches the small setGraphParameter action with widgetId/value and preserves immediate wasm preview. Slider/end no longer obtains documentJson or schedules the80ms whole-fixture commit. Other graph edits still use their existing whole-fixture path and are not included in this migration. Backend setGraphParameter registration/publication is not yet wired; this is an in-progress source change, not a working runtime claim.

## Active Dependencies And Decisions

The coordinator chose shared immutable generation ownership rather than converting all foreign JSON maps. The existing cold mounted codec and whole-root retirement still have independent boundedness gaps; no new interactivity credit is granted to generation-edit routes or whole-fixture imports.

The Flow owner owns the new OrderedMap primitive and map-type adoption. Its immutable persistent root allows unchanged map fields to share O(1); retained updates compare bytes and copy one path level per step. The reusable frameworkFlow typed copier/retirement extraction will use that API, not BTreeMap long-key insertions. The Flow owner agreed that the current app-local copier remains untouched until the replacement API is ready.

Latest-wins needs a runtime cancellation authority seam, not just a UI timer or producer-local sequence: concurrent typed jobs share the same captured Store generation until one publishes, and ArtifactOwnedToolJobRequest does not currently expose the live cancellation lease. The coordinator has been asked to assign exact app-instance supersession-key cancellation through publication.

## Remaining Work

Implement the exact typed command and registered concrete ArtifactOwnedToolJobFactory, reusable bounded fixture copying/retirement, targeted inverse, canonical mutation visitor and Store preparation, latest-wins cancellation through publication, schema-first DOM/native gesture/stale tests, actual compiler/runtime verification, and classify unsupported generation/fixture routes honestly. None of these remaining items is claimed complete.

