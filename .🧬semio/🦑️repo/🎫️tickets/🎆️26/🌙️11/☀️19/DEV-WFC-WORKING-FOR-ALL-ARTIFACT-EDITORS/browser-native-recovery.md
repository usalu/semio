# WFC native browser recovery on November 20

The five assigned ports had no listeners after the interrupted turn. All five OS dev variants were prepared and activated again using the ticket's native invocation envelope and the generated S plugin catalog. The WFC component was produced by native component run `wIRDTOG`, with component receipt SHA-256 `abe27718cd8f326dc27f0f50f7f88463cca84f4e42f561e40632d9efc42ddd55` and activation artifact SHA-256 `00bc5983df884a7bc517c88ac7e548cd22688cae758b5d350d89ab199327adad`. Each activation completed successfully. All five native fallback URLs returned HTTP 200, with the serve logs reporting a staged component matching its activation receipt.

| Variant | Port | Default example shown on mounted editor | Alternate example dispatched |
|---|---:|---|---|
| Bitmap | 6041 | Rooms 16 | Flowers 24 |
| Grid 2D | 6042 | Pipes | Terrain |
| WFC 2D | 6043 | Two Rooms And A Corridor | Wall And Roof Facade Strip |
| Grid 3D | 6044 | Building Blocks | 3D Pipes |
| WFC 3D | 6045 | Two Rooms And A Corridor | Wall And Roof Facade Strip |

Chromium observed each initial editor with two mounted windows and the corresponding canvases. Each alternate selection opened a pending Load Example command and left the picker showing the initial example. The guarded run requires the alternate example to appear in the controlled picker with the editor mounted and no command stall before it attempts Solve.

The browser probe retries only transient navigation context loss during snapshots. It now identifies visible collapsed action toggles by stable IDs so opening one cannot invalidate the locator for another. The earlier Bitmap probe clicked Solve too early while the picker still showed Rooms 16. Its Note Shell Command stalled and no Tool Run appeared. The earlier Grid 2D, WFC 2D and both 3D probes hit harness locator or navigation errors. Those runs do not establish Solve behavior.

The per-variant native logs use the prefix `[DEBUG] abe27718-` and the guarded browser logs use `[DEBUG] abe27718-verified-browser-` in the ticket's generated directory.

## Guarded browser results

All five guarded runs exited 1. Bitmap and Grid 3D displayed “The input could not be delivered.” Grid 2D, WFC 2D and WFC 3D still showed the Load Example stall at the six-minute harness deadline. The alternate example appeared in none of the controlled pickers. The tests did not reach Solve. All five guarded browser processes exited.

A separate Bitmap browser kept the default Rooms 16 example, opened the Input and Output action panes, and clicked Solve. The WFC actor reported a stalled Note Shell Command, and Chromium observed no Tool Run. A page timing read contained 21,744 worker turns. The newest 1,000 answered MoreWork with no events or patches, a carried result and zero internal drive polls.

## Bitmap command ingress evidence

The Chromium CPU profile showed active guest Wasm in the WFC shard while the Load Example command was pending. The Store worker and the three other shards were idle. The recurring Wasm stack was rooted at function 5692, the exported reactor poll. The component and browser core contain no Wasm names section.

The runtime trace reported `command_ingress` as the only MoreWork source after the alternate selection. Conditional CDP logpoints around the bridge poll captured one 498-byte command page with sequence 41 and 9,031 empty continuations. The poll kept returning CommandPending for sequence 41 at page index 0 or 1. There were no patches, effects, presence updates, wakes, acknowledgements or terminals in the observed window. The most common receipt appeared 2,818 times and charged 10 copied items, 4,096 copied bytes and 20 work units with no release or capacity growth. Other recurring groups released or reallocated 1,632 bytes.

The raw diagnostic logs are `[DEBUG] abe27718-profile-bitmap-select.log`, `[DEBUG] abe27718-profile-bitmap-solve-retry.log`, `[DEBUG] abe27718-bitmap-solve-turn-measures.log`, and `[DEBUG] abe27718-bitmap-poll-select.log` in the ticket's generated directory.

Wasm entry breakpoints after dispatch showed that the command passed manifest decoding and entered `OriginalSurfaceRefresh::advance`. During consecutive samples it stayed in phase 2 with no cached-capture lease. Later, spaced samples on one refresh owner observed slot 3 advance from phase 2 (ViewModel clone) to phase 6 (displaced view retirement) over 1.5 seconds. A later sample showed another refresh owner at slot 64 in phase 2. The old build made progress through this path, but the refresh took enough turns to stall the user command. The targeted evidence is in `[DEBUG] abe27718-bitmap-surface-refresh-phase.log` and `[DEBUG] abe27718-bitmap-surface-cursor-time-retry.log`.

A watcher waits for a staged component receipt different from `abe27718`, then re-prepares and activates all five variants and reruns the guarded probes on the existing native listeners.

## Direct Solve controls on the default examples

The shortest pane path observed in all five browsers is to expand **Actions** in the window listed below and click **Solve**. None of these buttons takes arguments. The guarded browser probe now expands only that pane and opens Tool runs after the click.

| Variant | Window with Solve | First pending command observed |
|---|---|---|
| Bitmap | Output | Note Shell Command |
| Grid 2D | Preview | Note Shell Command |
| WFC 2D | Preview | Note Shell Command |
| Grid 3D | Grid | Solve |
| WFC 3D | Preview | Note Shell Command |

Each click was verified with one browser inspecting variants serially. No Tool Run appeared during the seven-second observation. See `[DEBUG] abe27718-solve-pane-matrix.log`. A separate Bitmap WIT capture over 33 seconds recorded five LocalInteractionQuery pages, two surface-visible events and one `noteShellCommand` page for `shell.windowActivate`. It showed no Solve page behind the pending focus journal. See `[DEBUG] abe27718-bitmap-solve-command-witness.log`.

A longer Grid 3D differential ran Solve on the default Building Blocks example without an earlier shell note. The command failed at its approximately 300-second deadline with `plugin.command-ingress-stalled`. For action `solve`, sequence 39, the runtime recorded 47,185 continuations and reported only CommandPending. The Preview canvas had the same SHA-256 before and after the attempt, and no Tool Run appeared. See `[DEBUG] abe27718-grid3d-default-solve.log`.

Sparse Grid 3D Wasm samples followed the same refresh owner during a separate direct Solve. It advanced from slot 0 in displaced-view retirement at five seconds to slot 1 in retirement at eleven seconds, then to slot 2 in view cloning around 25 seconds and slot 2 in clone retirement around 50 seconds. At each sample, the stack showed both command ingress and manifest advancement. None of those steps held the cached-capture lease. The browser later reloaded and its old worker detached, so the late wire histogram was unavailable. See `[DEBUG] abe27718-grid3d-solve-wire-late.log`.
