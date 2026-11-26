# F62 result: real-launchd tests

Cause: macOS privacy (TCC), not the product. The helper job (`/bin/sh <dir>/reload.sh`) exited 126 `Operation not permitted`: the hermetic runner's tmpdir is `~/Documents/semio/.cache/scratch/...`, and a launchd-started process cannot read `~/Documents`. `launchctl bootstrap` accepted both plists, the main job ran; only the helper never wrote `result.json`.
Live fleet services avoid it by running `boot.sh` from `~/Library/Application Support/semio/fleet/entry` (generated plist verified: ProgramArguments there, SEMIO_WORKSPACE_ROOT only as env). The product maintainer writes reload script and plists to the entry dir, so no product change; F44's SEMIO_FLEET_* removal is unrelated.
Fix (test only): probe dir `~/Library/Application Support/semio/launchd-probe/e27-*` (removed afterwards); `priority()` waits for a stable reading (one earlier run read 31 as baseline under load); real tests default on for macOS with a reachable gui domain, opt out `SEMIO_REAL_LAUNCHD_TESTS=0`, so the coordinator Mac runs them in the normal suite.
Proof: 15/15 twice at load ~16-20, 14-20 s; no `e27probe` jobs or probe dirs left.
Not verified: other Macs (needs a GUI session; skipped without one); the baseline-31 flake cause not reproduced.
