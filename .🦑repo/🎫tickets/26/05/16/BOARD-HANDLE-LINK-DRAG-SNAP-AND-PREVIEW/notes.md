# Board handle link (2026-05-16)

- Rust `BoardHost`: `LinkFromHandle` / `LinkDragging`, `link_screen_preview` stroke, snap within `(HANDLE_HIT + LINK_SNAP_EXTRA)/zoom + radius`, commit `edge-link-{n}` + `edgeCreate` JSON.
- TS: `BoardScene.ingestWasmEdge` + `applyWasmDrainToScene` `edgeCreate` case (single emit).
- `defersDescriptorSyncFromJs` + `lastDescriptorPushDeferred`: skip `syncDescriptorJson` during link drag so `sync_descriptor` does not reset the gesture.
- Vitest: pointer drag link integration (MouseEvent `pointer*` types).
