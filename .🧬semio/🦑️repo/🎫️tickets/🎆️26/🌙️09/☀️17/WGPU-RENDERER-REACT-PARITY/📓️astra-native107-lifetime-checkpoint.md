# Native107 Lifetime Checkpoint

Native107 completed 32 tests: 25 passed, seven failed, 1,286 outside the filter. Test time was 2.190 seconds and Nx took 1m35s. Receipt: `🗑️generated/astra-runtime/renderer-native107-host-and-retirement/run.log`.

The passing laws include 257 sequential Canvas window mounts and closes, silent active-gesture closure, fencing queued scene intents before tree retirement, nested component-resource retirement preserving its successor, and Ink commit refusal during closure. Seven generic presented-input laws also passed.

The remaining failures cover three native clipboard ownership/late-callback laws, presented keyboard focus, two Canvas replacement/deadline laws, and a sibling Canvas fixture rejected for multiple parents. The sibling fixture was corrected to the published document root and extended to route wheels independently to two same-document hosts, remove one, and verify the surviving camera. This changed test has not run yet.

Post-run source changes carry the actual runtime host identity in `UiRetiredComponentScene` and compare resource ownership independently of arena-local node identifiers. Presented pointer liveness still checks the exact arena node. The UI implementation owner is repairing component lifetime across candidate/presented arenas, deferring visible-host retirement until presentation acknowledgement, rebasing capture without freezing presentation, and closing focus/clipboard owners incrementally.

The external Engine close law and an exact-owner map terminal-witness law were added after this run. They must produce actual fail-first receipts before their production fixes. CPU/GPU/raster resource retirement is not yet integrated with generic component closure. Browser checkpoint 19 remains unbuilt and no visual acceptance is claimed.

The first exact-delay React oracle executed 35 tests: 34 passed and one failed at the incorrectly assumed 100 ms deadline (700 ms tests, Nx 12.6s). Inspection of the current React source establishes `CAMERA_SYNC_DEBOUNCE_MS = 120`. The shared fixture/schema now require 120 ms; the mounted React test observes no publication at 119 ms and one at 120 ms. The native law consumes the same bound rather than its prior permissive 400 ms observation. The corrected tests are awaiting rerun; WGPU production remains 350 ms pending a native fail-first receipt. The first React receipt is `🗑️generated/astra-runtime/react-canvas-settle/run.log`.

The corrected React oracle passed all 35 tests, including exact 119/120 ms timing, with 954 ms test time and Nx 14.5s. Receipt: `🗑️generated/astra-runtime/react-canvas-settle120/run.log`. Native timing parity is still unverified.

## Native108

Native108 executed 38 tests: 30 passed, eight failed, 1,284 outside the filter; 1.259 seconds test time and Nx 5m40s. Receipt: `🗑️generated/astra-runtime/renderer-native108-component-lifetime/run.log`. All three repaired clipboard ownership laws passed, and the corrected presented keyboard fixture passed. Independent sibling Canvas cameras and the ordinary two-window Table transfer passed.

Five new laws reached their intended failures: exact Engine close left A staged beside B; the surface map reported terminal while a retired owner remained; no Canvas action was due at the React 120 ms deadline; a same-host Table refresh lost its drop action; and World camera diagnostics queried the shared wire address instead of the host. Three Canvas replacement/removal deadline failures remain, including reuse of an inactive arena's old mount after accepted removal. The full shared UI suite is running next.

After these actual failures, the root repaired the surface-map terminal witness, Canvas 120 ms deadline, passive-list window generation, and World diagnostic host lookup. Sol is implementing exact Engine/World/GPU/raster component retirement. These repairs still require runtime verification.

## UI7

The full shared UI engine suite executed 664 tests: 658 passed, six failed, zero skipped; 2.479 seconds tests, Nx 44.7s, run `10cbbe42-c692-48aa-bd71-b772df3ad7d0`. Receipt: `🗑️generated/astra-runtime/ui-engine7-presented-lifetime/run.log`.

The new deferred-retirement-after-presentation, held-pointer capture transfer, and superseded candidate retirement laws passed. The re-add law proved that accepted removal followed by same-key re-add reuses the inactive arena's old `scene.1.1` host. Sol is repairing that real identity defect. Remaining failures concern accessibility generation, retirement-ledger backpressure, two reconciliation completion assumptions, and the fixed-memory census: actual UI slot 164,120 bytes versus prior 162,000, fixed capacity 64 and owner 520 bytes unchanged. None is treated as a pass.

Flow21 full verification is now running with the cold fixture-owner repair and retained publication checkpoint. No new WGPU browser bundle has been activated.
