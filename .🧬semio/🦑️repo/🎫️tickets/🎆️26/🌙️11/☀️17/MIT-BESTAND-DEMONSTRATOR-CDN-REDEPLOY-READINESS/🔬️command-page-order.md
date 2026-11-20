# Boot failure: "command ingress fault: plugin.command-page-order"

## Cause (host TS side)
`createTurnPump.runQueuedTurn` (`📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts`) submitted every command page exactly once (`for (const commandPage of pages)`), whatever the guest answered. The 136-page `setContributions` pack makes the guest answer `backpressure` for some pages (slice ended / Source grant short); a backpressured page is NOT admitted, so the guest assembly stays at `pages.len()` while the host moves on to the next `page_index`; `continue_generic_assembly` then faults `plugin.command-page-order` (observed: page 0 accepted, 1 and 2 backpressure, 3 fault, 4 and 5 backpressure, 6 fault, ...). The guest is right: the protocol is "backpressure = offer the same page again", exactly what the native driver does (`CommandBatchDriver::observe`: Backpressure -> PageReady without advancing, only PageAccepted advances). The wgpu bridge driver had the same loop.

## Fix (TS only; the site must be rebuilt, no component rebuild)
- `handle-turn-drive/🟦️.ts`: page loop advances only on `page-accepted`, re-offers the same page on `backpressure` (yielding between offers, bounded by the 300 s command wall), stops on any other tag (`command-pending`/`command-complete`/`fault`) and hands it to the existing continuation loop. `terminal` is now folded per page; the wall constant moved above the loop.
- `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🔌️plugin-module-loader/🖼️render-and-turn-drivers/🟦️.ts`: same rule (bounded by 1024 offers).
Not changed: store worker `action-dispatch` / `snapshot-read` (single small commands, they throw on backpressure by design).

## Test
`📺️renderer/🧑‍🎨engine/🧪️tests/🧵️turn-dispatch-lifecycle/🟦️.tsx`: "offers a backpressured command page again instead of skipping it" (page 1 refused once, expects offers `[0,1,1]`). Run: `repo capacity run --memory=1024 -- repo tools exec bun -- test <...>/🧪️tests/🔌️plugin-runtime-suite/🟦️.tsx -t "command page"` (2 pass incl. the page census law).
