# Current Product Seven Slot Budget Law Read-Only Audit

The original current EngineCanvas law and actual async fixed-slot guard were read directly after the failed owning run. Production and fixture writes remain prohibited pending causal layout diagnosis.

```rust
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "renderer::engine_canvas")
        .map(|table| {
            semio_framework_async::FixedSlotTableBudget::new(
                table["owner"].as_str().expect("owner"),
                table["capacity"].as_u64().expect("capacity") as usize,
                table["elementSizeBytes"].as_u64().expect("element bytes") as usize,
                table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize,
            )
        })
        .collect();
    let measured = vec![
        semio_framework_async::FixedSlotTableBudget::new("engine_canvas::EngineSurfaceRegistry", ENGINE_SURFACE_CAPACITY, size_of::<EngineSurfaceSlot>(), size_of::<EngineSurfaceRegistry>()),
        semio_framework_async::FixedSlotTableBudget::new("engine_canvas::StagedEngineScenes", ENGINE_CANVAS_FRAME_PACKET_CAPACITY, size_of::<Option<StagedEngineScene>>(), size_of::<StagedEngineScenes>()),
        semio_framework_async::FixedSlotTableBudget::new("engine_canvas::EngineCanvasBuildContext", ENGINE_CANVAS_FRAME_PACKET_CAPACITY, size_of::<Option<EngineCanvasPacket>>(), size_of::<EngineCanvasBuildContext>()),
    ];
    semio_framework_async::assert_fixed_slot_tables(
        "renderer::engine_canvas",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(EngineSurfaceRegistry::default());
            drop(StagedEngineScenes::default());
            drop(EngineCanvasBuildContext::default());
        },
    );
}

/// ⚖️ Law: painting a tiled-map surface ADMITS its visible tiles into the bounded renderer-asset
/// pipeline. `apply_map_tile_bytes` existed with zero production callers and nothing reserved a
/// `WorldAssetRequestKind::MapTile` anywhere in the repo, so raster and vector tiles were dead on
/// both browser and native — the map painted its vector fixture over an empty basemap forever.
/// React's `MapRenderer.uploadTileRow` (`🧭️TiledMapHost/🟦️.tsx:608-631`) is the reference: fetch
/// every visible tile the session does not already hold, skip the ones it does, never re-request a
/// miss.
```

```rust
/// `native_pool` now states explicitly instead of inheriting Rust's 2 MiB default.
/// 🧱️ One row of the [fixed-slot fixture](🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json), measured or declared — see
/// [`assert_fixed_slot_tables`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedSlotTableBudget {
    pub owner: String,
    pub capacity: usize,
    pub element_bytes: usize,
    pub owner_bytes: usize,
}

impl FixedSlotTableBudget {
    pub fn new(owner: &str, capacity: usize, element_bytes: usize, owner_bytes: usize) -> Self {
        Self { owner: owner.to_string(), capacity, element_bytes, owner_bytes }
    }
}

/// 🧱️ The whole `boxed_fixed_slots` law, in one place: the crate's measured rows equal the
/// committed fixture's rows, every listed table is big enough to be worth listing
/// (`capacity * element_bytes` over the conversion threshold), every owner is **smaller than the
/// table it owns** — which is exactly what an inline `[T; N]` field cannot be, so it is the
/// structural proof that the slots live on the heap — and `build` (the owners' `Default`/`new`) runs
/// to completion on a thread holding only `stack_bytes`.
///
/// Callers supply `declared` from the [fixed-slot fixture](🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json) and `measured` from `size_of` at the
/// one site where the private slot types are nameable; each caller owns its physical descriptor.
/// This function owns the reusable assertions without enumerating callers.
#[cfg(not(target_arch = "wasm32"))]
pub fn assert_fixed_slot_tables(guard: &str, stack_bytes: usize, threshold_bytes: usize, declared: &[FixedSlotTableBudget], measured: &[FixedSlotTableBudget], build: impl FnOnce() + Send + 'static) {
    assert!(!declared.is_empty(), "🧱️ guard '{guard}' owns no row in the committed fixed-slot-table budget");
    assert_eq!(measured, declared, "🧱️ guard '{guard}': measured slot tables differ from the committed budget");
    for row in measured {
        let table_bytes = row.capacity.checked_mul(row.element_bytes).expect("🧱️ slot table size fits usize");
        assert!(table_bytes > threshold_bytes, "🧱️ {}: {table_bytes} B is under the {threshold_bytes} B conversion threshold — drop the row instead of carrying it", row.owner);
        assert!(row.owner_bytes < table_bytes, "🧱️ {}: the owner is {} B against a {table_bytes} B slot table, so the table is still inline — build it through boxed_fixed_slots", row.owner, row.owner_bytes);
    }
    on_bounded_stack(guard, stack_bytes, build);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn on_bounded_stack(name: &str, stack_bytes: usize, build: impl FnOnce() + Send + 'static) {
    let lane = std::thread::Builder::new().name(name.to_string()).stack_size(stack_bytes).spawn(build).expect("🧵️ the bounded-stack lane spawns");
    if let Err(panic) = lane.join() {
        std::panic::resume_unwind(panic);
    }
}
//#endregion 🧱️BoxedSlots

//#region 🧵️WorkerPool
/// 🧵️ One submitted unit of pool work — a plain closure, never a `Future`. `WorkerPool` is the CPU
/// substrate every subsystem's OS-thread work collapses onto (Phase 0 census: shard executors, shard
/// outcome forwarders, DB actor threads, the epoch ticker, HTTP fetch threads); polling `Future`s to
/// completion remains the future-polling executor's job (packet R2/P1b), built ON TOP of this pool.
pub type Job = Box<dyn FnOnce() + Send + 'static>;

```
