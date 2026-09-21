# Native107 Lifetime Checkpoint

Native107 completed 32 tests: 25 passed, seven failed, 1,286 outside the filter. Test time was 2.190 seconds and Nx took 1m35s. Receipt: `🗑️generated/astra-runtime/renderer-native107-host-and-retirement/run.log`.

The passing laws include 257 sequential Canvas window mounts and closes, silent active-gesture closure, fencing queued scene intents before tree retirement, nested component-resource retirement preserving its successor, and Ink commit refusal during closure. Seven generic presented-input laws also passed.

The remaining failures cover three native clipboard ownership/late-callback laws, presented keyboard focus, two Canvas replacement/deadline laws, and a sibling Canvas fixture rejected for multiple parents. The sibling fixture was corrected to the published document root and extended to route wheels independently to two same-document hosts, remove one, and verify the surviving camera. This changed test has not run yet.

Post-run source changes carry the actual runtime host identity in `UiRetiredComponentScene` and compare resource ownership independently of arena-local node identifiers. Presented pointer liveness still checks the exact arena node. The UI implementation owner is repairing component lifetime across candidate/presented arenas, deferring visible-host retirement until presentation acknowledgement, rebasing capture without freezing presentation, and closing focus/clipboard owners incrementally.

The external Engine close law and an exact-owner map terminal-witness law were added after this run. They must produce actual fail-first receipts before their production fixes. CPU/GPU/raster resource retirement is not yet integrated with generic component closure. Browser checkpoint 19 remains unbuilt and no visual acceptance is claimed.

The first exact-delay React oracle executed 35 tests: 34 passed and one failed at the incorrectly assumed 100 ms deadline (700 ms tests, Nx 12.6s). Inspection of the current React source establishes `CAMERA_SYNC_DEBOUNCE_MS = 120`. The shared fixture/schema now require 120 ms; the mounted React test observes no publication at 119 ms and one at 120 ms. The native law consumes the same bound rather than its prior permissive 400 ms observation. The corrected tests are awaiting rerun; WGPU production remains 350 ms pending a native fail-first receipt. The first React receipt is `🗑️generated/astra-runtime/react-canvas-settle/run.log`.

The corrected React oracle passed all 35 tests, including exact 119/120 ms timing, with 954 ms test time and Nx 14.5s. Receipt: `🗑️generated/astra-runtime/react-canvas-settle120/run.log`. Native timing parity is still unverified.
