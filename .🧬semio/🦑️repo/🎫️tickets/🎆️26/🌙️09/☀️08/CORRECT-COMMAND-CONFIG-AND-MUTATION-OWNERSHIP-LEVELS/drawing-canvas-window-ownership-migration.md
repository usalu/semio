# Drawing Canvas Window Ownership Migration

## Ownership decision

The exact registered Drawing Canvas window owns navigation in `DrawingCanvasWindowConfig` and pointer/trace progress in `DrawingCanvasWindowTransient`. The application uses `NoConfig`; its document schema and nonempty `DrawingPresence` remain document/collaboration concerns. Presence reuses the shared `Viewport2d` schema because it communicates a peer viewport, while each local Canvas instance keeps its independent `Viewport2d` in `WindowConfig`.

`SetCamera`, `SetCameraZoom`, and gesture completion resolve the caller's concrete `window_id`, verify that it is registered as the Canvas kind, and publish only addressed `WindowConfig` or `WindowTransient` mutations. Rendering and chrome read those exact captured owners. Parent document commands keep their existing document lane.

## Schema and implementation

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config` defines the Rust owner, closed replacement mutation, exact Pack/text codecs, owner registration, address helpers, five schema facets, neutral fixture, independent TypeScript oracle, and native law.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🫧️transient` defines the exact local transient owner and its five facets.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` registers both owners, captures them in retained work, routes exact emissions, supplies lifecycle factories/disposers, and reads concrete-instance state for render and chrome.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence` uses the shared `Viewport2d` in all five facets and retains the established nonempty collaboration contract.
- The obsolete app camera/config facets were removed. `✏️editor/🎚️config/🦀️.rs` now exposes the empty app config plus the real presence descriptor.

## Validation

The registered neutral route passed in `🗑️generated/drawing-canvas-window-ownership-oracle-3.log`. Ajv validated Canvas config, Canvas transient, and Drawing presence against their real JSON Schemas including the shared `Viewport2d`; `fast-json-patch` independently reproduced the mutation result; strict TypeScript compiled the exact schema and oracle. The final runtime line was `[DEBUG] drawing-canvas-window-ownership configs=2 transients=1 reload=config-restored-transient-reset`.

Native r1 passed its neutral/Ajv/strict-TypeScript stages, then stopped before the focused tests while compiling the Drawing crate. The package root still included the deleted app-config Rust schema; after that include was removed, Rust parsing found an empty generated presence reexport region whose doc comment no longer documented an item. Both stale references are removed while the empty app descriptor and real nonempty presence facet remain registered. The exact failure is `🗑️generated/drawing-canvas-window-ownership-native-1.log`; the corrected neutral route passes in `🗑️generated/drawing-canvas-window-ownership-oracle-fix.log`. The native law remains queued for a coordinated rerun.

Native r2 compiled through the deleted-schema repair and exposed seven Drawing-local API/lifetime errors before its focused tests: the two presence roots used an owned-value factory where the Store requires an `Arc` snapshot retirement factory, one legacy fixture expected a `Result` from the now-infallible digest close step, and four gesture completion branches retained their mutable session borrow while cancelling the containing fixed-operation registry. The editor now uses `SharedValueRetirementFactory<DrawingPresence>`, consumes the direct `SnapshotRetirementStep`, and clones the returned window transient before each registry cancellation so the exact retained owner can enter closure. The 52.3-second result is `🗑️generated/drawing-canvas-window-ownership-native-2.log`; the corrected files parse with Rustfmt and await the next coordinated native batch.

## Executable routes

Root and ticket scripts/projects register `drawing-canvas-window-ownership-oracle` and `drawing-canvas-window-ownership-native`. Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` register them at orders `311.208` and `311.209`.
