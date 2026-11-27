# GIS map lane results (interim)

Nothing has passed or failed in a test run yet. Evidence so far (logs `🗑️generated/gismap-lib.log`):
- Attempt 1 of `semio-s-artifact-gis-gismap --lib` (`--manifest-path` gismap workspace, `-j 3`, `--test-threads=2`) was killed by the host memory warden during dependency compilation ("cancelled 1 of its processes ... critically short of memory").
- Attempt 2 compiled 538 crates, then the warden cancelled rustc for `semio-s-artifact-stdio-pdf-model-graphics` (1661 MB unit) under memory pressure; no compile error was printed for that crate other than the warden cancellation.
- Runner `gismap-lane-run.sh` retries after warden cancellations (compiled artifacts persist) and waits on the shared compile lock; the lock was held by other lanes for long periods.
- Coordinator note applied: `repo run/test` takes the task first, flags after.

Suites/gates not yet run: gismap lib, mvt-decoder label test, manifest pair test, tiles crate, bridge crate, envelope gates script, component-cold-map-patch-native-check.

## Status at 18:16
- No test has run to completion; no source fix was made by this lane.
- Runner `gismap-lane-run.sh lib ...` is still alive in the background, queued on `🗑️generated/compile.lock` (held by conformance-lane since 17:57, terrain-lane before). When it gets the lock it retries the gismap lib run; result lands in `🗑️generated/gismap-lib.log` (last line `exit N`).
- Remaining after that: mvt label test, manifest pair test, tiles crate, bridge crate, `verify-gis2d-envelope-gates.sh` (task-first syntax for `repo run`), `component-cold-map-patch-native-check`.

## Takeover (gismap2-lane), 22:43
- Old runner `gismap-lane-run.sh` killed (the lock was owned by another lane, so it was not touched).
- Compile error fixed at the root (see `conformance-fixes.md`, section "Takeover"): the two map-config mutation report bridges no longer call `semio_os_store_test_support`.
- Verification is queued, unattended, under the shared compile lock (`-j 3`, `--test-threads=2`, memory_pressure >= 25 % free, 120 s pause after each release, up to 3 / 6 attempts after warden cancels):
  - `gismap2-all.sh` (runner `gismap2-run.sh`): `check --tests` for gis2d, gismap, gismap-viewer, gismap-config -> `🗑️generated/gismap2-check-all.log`, marker `gismap2-all.done`. Attempts 1 and 2 were cancelled by the host warden (cargo pid 1930 MB, host critically short of memory) after about 300 crates were checked, no compiler error (0 `^error` lines); attempt 3 is waiting for the lock.
  - `gismap2-rest.sh` (starts after `gismap2-all.done`, runner `gismap2-run-6.sh`): logs `gismap2-lib.log` (gismap lib tests incl. `shell`), `gismap2-tiles.log` (tile-coordinates crate incl. `the_overlays_value_carries_every_document_feature`), `gismap2-mvt.log` (`vector_label_anchor_uses_line_midpoint_for_transportation_names`), `gismap2-hub.log` (`gis_manifest_publishes_one_editor_viewer_pair_per_artifact_dialect`, package `semio-hub-gis --lib`), `gismap2-bridge.log`, `gismap2-gates.log` (2D envelope gates script), `gismap2-cold-map.log` (`repo run @semio-tech/gis-plugin:component-cold-map-patch-native-check --lang=en`); marker `gismap2-rest.done`. Each log ends with `exit N`.
- No pass/fail counts exist yet; none are claimed.
