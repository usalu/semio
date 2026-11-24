# Panel adds: slow geometry and a stuck command lane (2026-11-24)

Found by hand on guest r36 after the journeys went green. The journeys do not reach it: they draw one small rectangle and never look at canvas pixels.

## Evidence (guest r36, headless Chromium, host load 10 to 14)
- `probe-latency.mjs` (Ellipse, Rectangle, Polygon, 6 s apart), 3 runs: layer rows after 1.3 to 3.5 s; ellipse on the canvas after 21, 21 and 32 s; the polygon never within 45 s, in all three runs.
- `probe-latency-trace.mjs` (patches the served host module inside the probe only): the ellipse geometry job needs 1,227 and 1,271 steps at about 16 ms each; the emblem alone needs about 90. Each step may already compute for 8 ms, so this is mostly work, in an unoptimised prototype guest.
- The polygon job stops after step 864 (twice) and 960; the drive waits for the actor's command lane. `probe-lane-trace.mjs`: the lane is held by a command turn (`enqueue` -> `runQueuedTurn`) that started at that moment and is still running 25 s later. In the run where it was named it is `LocalInteractionQuery`. A following "Add Line" never produced a row: the editor is stuck.
- With the lane probe two later runs never produced a row at all; the probe itself may disturb that timing, so they are not counted.
- The browser pane showed the panels stuck half filled, but the pane was hidden; not counted.
- Another agent's `[DEBUG] ladder zero blocks=true` is the guest's last line before the stall. It is also printed right after boot, so alone it proves nothing.

## Not known
Why the query does not complete, whether it waits on the geometry job (which cannot step while the query holds the lane) and whether the same happens in a release guest.

## Clipboard (same guest, `probe-clipboard.mjs`, one run)
Select the nested path layer in the panel, then Meta+C, V, X, V, D: no step changes the layer list within 12 s. Copy and the first paste print nothing; `cut` and the following `paste` are reported by the shell as stalled (`continuation 737/16, guest status more-work, ingress command-pending`). Meta+D is not bound (only undo and redo are in Draw's manifest), so its row proves nothing. The other agents' probes ran at the same time.

## Where the 20 s go (`probe-step-cost.mjs`, guest r36, one ellipse, load 7)
Last 1,100 steps: median 8.80 ms, p90 9.50 ms, 9.6 s inside `stepJob` of a 19.8 s span; 10.2 s are gaps, about 300 ms after each batch of 32 steps. Work counter about 3.1 million units, so about 3.7 us per unit. The host grants 100 ms per step (`DEFAULT_SHARD_BUDGET.wallMs`), so the guest's own 8 ms cap is what ends a step. The 2 ms / 12 % duty cycle in `geometry-cost-2026-11-24.md` is the native relay's budget and does not apply to the browser.

Edits (not built): `CANVAS_FLATNESS = 0.1/32` instead of 0.001 (picking needs tolerance/80 = 0.1/zoom; an ellipse goes from 1,024 to 512 segments by that file's own halving rule), and the step's own clock is read every 32 units instead of every unit. The gaps look like the full UI refresh that each progress publication triggers; not yet proven or changed.

## Resolution so far
- **Hang:** the actor's lifetime stock (`🔨️canvas/🧵️renderer-work/🔣️stock.json`) ran out (`local-query-stall-2026-11-24.md`). Measured spend in about 80 s of use: 9.2 M of 8.4 M items (4x run), 20.8 M of 50 M fuel, 308 of 512 MB capacity, 156 of 512 MB release, 207 MB of 2 GiB copy. Resized to 2^31 per retained axis (the value the copy axis already used; wasm32 usize limits it to 2^32), 2^40 fuel and 24 h per operation, with a pool for 128 operations. `probe-many-adds.mjs`, 2 runs of 6 adds: all rows and canvas updates arrive, no hang. This is sizing, not renewal: at the measured rate the capacity axis still ends after about 9 minutes of continuous heavy editing. The setting is shared by every plugin in the renderer.
- **Gaps:** a progress refresh is held back by four times the lane time the last one cost the job (`handle-turn-drive`). Gaps 10.2 s -> 3.8 s, ellipse span 19.8 s -> 13.3 s (one clean run), polygon 18.9 s. PluginRuntime suite 144 of 144 (vitest, level long).
- **Compute:** flatness and clock edits wait for the next guest build.

## Proposal: renew an actor's funds (not implemented, needs a framework owner)
The guest refuses a grant that exceeds what its context has left (`original actor context renewed its physical source authority`), by design. One operation per actor lifetime is therefore a hard end of every long session, and byte axes cannot exceed 4 GiB on wasm32. Options: (1) admit a successor operation when the actor is quiescent (no job slot, no held output, no outstanding turn) and rebind the context to it, kernel-issued and receipted like the first; (2) account the capacity axis as live (retained minus released) instead of cumulative; (3) issue per-command operations, as the native relay does for jobs. (1) keeps every existing law and needs a typed `original-stock-exhausted` fault for the non-quiescent case.

## Journeys after the stock and back-off changes (16:05 to 16:20, server restarted at 16:03, a Rust compile running, load 11 to 20)
The journeys were not rerun right after those two edits. Afterwards they are flaky: r36u en fails at `example`, r36v1 passes, r36v2 fails at `boot`. Alternating A/B (`journey-ab.sh`; `off` patches the back-off away in the test browser only): on 3 pass 2 fail (ab1, ab2 so far), off 2 pass 1 fail, each failure at another step (`example`, export download, `redo` with `actor-document-port.retired`). So the back-off is not shown to be the cause, nor cleared. Candidates: load, the fresh server, host edits of other agents since 13:35. To be repeated on a quiet machine.
