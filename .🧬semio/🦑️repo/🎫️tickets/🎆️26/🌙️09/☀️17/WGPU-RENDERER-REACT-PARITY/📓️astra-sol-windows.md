# Astra Sol Window Lifecycle and Template Drag

## Scope

This slice repairs three WGPU shell differences from the React renderer:

1. Closing the last dock window leaves a full-size empty drop body, and opening a Display entry creates a usable active root window.
2. Closing a `World3d` window removes its pointer, wheel, status, projection, overlay, settle, and action authority at the layout-to-input synchronization boundary.
3. Display window-kind and projection rows publish React's window-template MIME payload and can land at tab, left, right, top, bottom, and empty-root targets.

Escape cancels pending and promoted template drags without changing the dock. A pointer release outside a target abandons the drag through the existing cancellation path.

## Contracts and Oracles

The language-neutral vectors live at `engine/🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json`; their schema lives at `engine/🧬️schema/🪟️window-lifecycle-template-drag/🔣️.json`.

The TypeScript law uses Ajv as the third-party schema oracle and an independent layout projection for the six target vectors. It also checks the existing React `Canvas`/`ShellHost` sources and React drag-cancellation tests for the shared MIME, targeted insertion, and cancellation behavior. Rust Dock and Shell laws consume the same JSON fixture.

The React behaviors used as the implementation oracle are:

- `COMPOSE_WINDOW_TEMPLATE_MIME` and template payload publication in `🎨️Canvas`.
- `insertWindowAtDropZone` for target-sensitive layout insertion.
- `handleTemplateDrop` in `🏛️ShellHost`.
- `cancelWindowTemplatePointerDrag` for keyboard cancellation.

## Final Ownership

| Owner | Live responsibility | Close transition | Pressure behavior |
| --- | --- | --- | --- |
| `DockState` | Window topology, active tab, instance kind, optional projection template | Removes the tab and commits the resulting tree, including an empty stack | A new template is refused if its instance id already exists |
| `AdmittedSurfaceMap<World3dState>` | Active, input-addressable scene generation | Removes the exact state and immediately invalidates its `AdmittedSurfaceToken` | Active owners and draining owners share `SCENE_SURFACE_CAPACITY` grants |
| Shell retired World3d queue | Exact removed scene owner while dynamic resources close | Advances one bounded retirement step in the existing maintenance phase | Preallocated for 256 owners; each queued owner reserves one map admission grant until terminal release |
| `RendererAssetFetchOwner::World` | In-flight fetch/decode owner | Carries the active surface generation token from checkout through finish/return | Missing or changed generation closes benignly and cannot publish into a same-id reopen |

This grant sharing gives the retired queue a fixed admission policy. If close/reopen occurs faster than resource retirement, new scene admission receives the ordinary item-capacity refusal once active plus draining owners reaches 256. Existing live generations remain available. A terminal retirement releases one grant and a later paint can admit the waiting window as a new generation.

## Repairs

### Dock and Display

- `DockStackTab` retains `template_id` through layout import/export, tab moves, split moves, mobile flattening, and persistence.
- `DockDragKind::NewWindow` distinguishes palette templates from moves of existing tabs/stacks, so rendering the drag ghost never docks out a source window.
- Empty-stack insertion replaces the empty root with the incoming window instead of creating an empty-id tab.
- Display window-kind and projection rows are draggable, publish `application/x-compose-window-template`, and carry no click action.
- Pointer-down checks drag data before the ordinary tree-hit action consumes the row.
- `open_display_window` uses the same drop operation: right of the active stack when populated, or the empty root when the final window was closed.
- Phase-six chrome draws the existing target indicator and drag ghost for template drags.

### World3d lifecycle

- `sync_engine_surface_states` derives live window ids from the dock/panel plan, removes closed World3d generations, begins dynamic retirement, and clears shell caches at the existing layout-to-input boundary.
- The removed state is retained in FIFO ownership and advances with one unit of maintenance fuel. Renderer shutdown drains this queue before closing active World3d states.
- The cleared projections include status and pill traces, settle watches, utility/pane folds, projection templates, expanded actions, search state, measures, engagement state, content rectangles, silhouettes, and staged action arguments.
- Pointer and wheel hit tests only see the active map after synchronization.

### Late async completions

- Fetch checkout and completed-decode checkout snapshot `AdmittedSurfaceToken` with the surface metadata id.
- Response reservation, sealing, terrain apply, reference-image apply, mesh publication, cancellation, finish, and return resolve by token rather than string id.
- Runtime-lock contention remains retryable. A missing token means the generation retired; the probe closes and finish consumes the old owner without applying it.
- Reopening the same window id receives a different epoch token, so an old completion cannot recover or mutate the new state.
- Native blocked-owner cancellation advances its own close state per pump instead of retaining a cancelled old-generation owner indefinitely.

## Test Development and Verification

Tests and the neutral fixture were written before the production repair. Two focused Nx attempts reached the renderer unit target but waited behind the repository's shared Cargo artifact locks; they were interrupted, so no red test result was observed. A TypeScript Nx attempt stopped before the law because the then-current project graph reported the unrelated cycle `framework-os-kernel -> value-derive -> framework-os-kernel`.

Source validation completed:

- `git diff --check` passes after the implementation and pressure-policy changes.
- `jq empty` accepts both the neutral fixture and its JSON Schema.
- The Rust laws cover all target vectors, empty-root restoration, persisted projection metadata, Escape cancellation, immediate input-authority removal, cache removal, token invalidation, same-id generation renewal, and fixed retirement pressure.
- The TypeScript law covers Ajv fixture validation, independent drop projection, and the existing React source/test oracle.

Root owns the fresh integrated runs because renderer builds share fine-grained Cargo compilation locks. Current integrated output is recorded under `🗑️generated/astra-runtime/wgpu-wasm-build.log` and `🗑️generated/astra-runtime/renderer-native-tests.log`. No focused or integrated pass is claimed here until those runs finish.

## Remaining Measured Receipt

Adding the external reservation count changes the in-memory shape of `AdmittedSurfaceMap`. The boxed-fixed-slot law will report the measured `ownerSizeBytes` if its committed receipt needs an update. That value must be taken from the integrated test result; it was not guessed in this slice.
