# Board Host Constructor Allocation Inventory

Authority: admitted proposed closure-source-2, not a claim that these General paths are published. Exact tree-sitter source witnesses and file/function hashes are retained in generated/board-constructor-audit/source-witnesses-1.json. No allocator or native runtime measurement was run.

BoardHost::new calls Self::default unchanged; new_normal also constructs the complete default before changing port mode and handle selection. Guarding only the five entity maps therefore misses the actual nested construction.

| Constructor path | Exact source witness | Work beyond entity maps | Required control cut |
| --- | --- | --- | --- |
| BoardHost default → selection_options | normal/🦀️.rs:3669 | Owns rectangle and replace strings | Admit/cancel before each owned string allocation; retain exact ordinary values |
| BoardHost default → world_raster_tiling | normal/🦀️.rs:3677 | Owns world-clip string | Explicit string-byte credit and cancellation before creation |
| BoardHost default → builtin_edge_tips | directed/🦀️.rs:535 | Five ids, five id.to_string keys, five populated map inserts. builtin_for_id calls to_ascii_lowercase for each id, creating temporary owned strings even for canonical builtin literals | Separate bounded builtin-seed stage; details below |
| Same builtin phase | directed/🦀️.rs:524 | Five temporary lowercase strings plus owned keys and map backing growth; do not assume five distinct B-tree node allocations | Replace builtin lookup with explicit enum/constant defaults or control every temporary/key/insert; cancellable bounded seed cursor, fallible insertion/admission |
| BoardHost default → BoardEventQueue default | normal/🦀️.rs:2340 | Box of 256 Option<BoardOwnedEvent> slots via General Async boxed_fixed_slots | Admit table byte/item credit before reserve; cancellable slot initialization; do not allocate event payloads for None slots |
| BoardHost default → IconPaintCache::new → default → IconPaintRegistry default | directed/🦀️.rs:952,852 | Box of 256 IconPaintSlot entries, each starts key/value None | Admit backing bytes/items and initialize bounded pages; preserve empty registry epoch1 and retirement state |
| BoardHost default → BrushCandidatePage default | normal/🦀️.rs:249 | Box of 49,152 zero bytes plus Box of 1,024 Option<BrushCandidateEntry> slots. Box::new large inline array expressions also materialize temporaries | Separate byte-buffer and entry-table admissions; initialize bounded pages without large stack arrays; cancel between allocations/pages |
| General Async boxed_fixed_slots → boxed_slots | async/🦀️.rs:1560,1549 | Vec::with_capacity then resize_with over the whole length; no control, fallible reserve, or interruption in this helper | A controlled constructor must expose reserve/init/seal stages rather than call this whole synchronous helper behind one step |

The builtin row above identifies the five-key loop; its control cut is detailed in the following row rather than hidden behind one aggregate map credit. At least four independent boxed backing constructions occur before first scene admission: event slots, icon slots, brush bytes and brush entries. Three retained host strings and the seeded tip map are additional. Concrete total bytes for typed slot arrays require actual size_of/layout evidence; no architecture-dependent size is guessed here.

The five entity maps, four empty kind maps, five empty selection/highlight sets, empty compatibility Vec and two empty weight HashMaps have no explicit populated reserve/insert work in default. Future population still needs separate admission. RefCell/Cell wrappers, None fields, scalar Camera defaults, TransformGumballFlags and close_strings=[None;16] are inline state. CanvasPalette defaults copy fixed Color([f32;4]) values through Color::new; the examined constructor path contains no heap allocation. No rendered icon scene, raster image or event payload is constructed by the empty defaults.

A proper host-construction operation must own incomplete backing until success, publish the host only after all phases seal, and support bounded cleanup/cancellation without an admitted scene. Zero-credit/cancel-before-allocation, cancellation after each backing/page, failure after each backing, exact completed default equality, new_normal behavior, and worker-stack limits need language-neutral schedules plus an independent oracle. A single controlled wrapper around BoardHost::default does not satisfy those cuts.

This is a read-only source inventory. Native allocation counts, stack peaks, constructor-control implementation and whole engine equality remain unverified.
