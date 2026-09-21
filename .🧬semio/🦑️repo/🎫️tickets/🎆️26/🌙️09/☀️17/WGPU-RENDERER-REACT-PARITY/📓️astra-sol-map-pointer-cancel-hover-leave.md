# Tiled Map Pointer Cancel and Hover Leave

## Contract and fail-first boundary

The scene pointer cancellation fixture and schema are version 2. They retain the existing generic cancellation rows and add a closed TiledMap contract with distinct window, surface, and peer-surface ids. Pan cancellation publishes exactly one camera settlement and clears the drag. Marquee cancellation clears local gesture state and publishes no selection. Cancellation without a drag is inert. Every row admits the next fresh down; duplicate cancel and the old outside-up remain inert.

Hover leave starts from a non-null position witness and publishes exactly one `interactionHover` with empty targets. A repeat leave and the peer Map remain silent, and hover retirement has no gesture fanout.

The canonical source law was executed before production changes. Five assertions passed and the exact route assertion failed because `Shell::handle_pointer_cancel_for` still called `tiled_map_pointer_up_into`. Native80 then provided the production-absent native receipt: seven `E0425` errors for five cancel and two leave references, no other renderer errors, and Nx ended after 56.6 seconds. This compile RED is distinct from a behavioral runtime RED.

The registered native filters are:

```text
tiled_map_cancel_separates_pan_camera_settle_from_marquee_selection
tiled_map_hover_leave_publishes_one_empty_owner_transition
```

They use an attached TiledMap host, bounded action reservations, real pointer down/move/up entry points, and the fixture's distinct owner identity. No Cargo command was run locally.

## React oracle

The existing component-source oracle executes React's actual `onPointerCancel` callback for both pan and marquee state. It observes host pointer-up settlement, marquee reset, pointer-capture release, and camera mirror, and proves `emitFeatureSelection` remains uncalled in both modes.

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-react:test-long' -- '../../../../🧱️elements/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts' --silent=false --reporter=verbose
```

Receipt: one file, 22/22 tests passed in 4.49 seconds; Nx completed in 5.4 seconds.

The independent neutral/source law command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-react:test-long' -- '../../../../🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts' --silent=false --reporter=verbose
```

The pre-production receipt was five passed and one failed on the legacy Map-up cancellation route. After the dedicated APIs and Shell route were implemented, the same file passed 6/6 in 901 ms; Nx completed in 1.5 seconds.

## Production boundary

`tiled_map_pointer_cancel_into` distinguishes the retained drag mode. Pan sends one `PointerUp` into the Map host and clears gesture state only after its bounded action publication succeeds. Marquee clears drag, points, and active state locally and emits no action. No active Map gesture returns false.

`tiled_map_pointer_leave_into` checks the cached non-null hover witness, publishes one empty `interactionHover` using the existing domain and pointer channel, and commits the null witness only with successful publication. This retains a retryable witness after a bounded fault. Shell calls the cancel API only when the retained Map surface's `window_id` matches the cancelled target's window.

The renderer move hook is root-owned. Its bounded integration calls the leave API only for a previously hovered Map that differs from the selected retained TiledMap target, before selected-target move dispatch. No Graph behavior is part of this packet.

Renderer Native81 executed the two registered laws in the full renderer census. Both Map laws passed. The full receipt was 1,272 run, 1,268 passed, four failed; the remaining failures belonged to the concurrent per-pointer and stale World work. The peer-leave renderer hook was present in this receipt.
