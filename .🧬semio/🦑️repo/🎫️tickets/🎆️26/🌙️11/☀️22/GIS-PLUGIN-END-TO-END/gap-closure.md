# GIS Plugin End to End: Gap Closure (Continuation 2026-11-22 afternoon)

Status per item of `open-gaps-audit.md` and the handoff steps. "Verified" means a command ran in this continuation and its log was read.

## Rust tests
- mvt-decoder: 12/12 pass, including `vector_label_anchor_uses_line_midpoint_for_transportation_names` (gap 8, label anchor). Verified (log of 14:00).
- gismap (`semio-s-artifact-gis-gismap --lib`):
  - `repo cargo -- test -p semio-s-artifact-gis-gismap` from the root failed in preparation with "Unknown or ambiguous selected Cargo package". Root cause and fix: `cargo-package-ambiguity.md`.
  - Earlier failures (two label glyph tests, sqlite snapshot receiving port): root cause and fix in `gismap-test-failures.md`.
  - Cargo workspace tests after that fix: `🧪️tests/🟦️.ts` 20, `🪆️nested-owners` 4 (two new regression tests), `📤️arguments` 6, `🔁️replan` 9 pass. They first failed because `repo test-file` did not pin `RUSTUP_TOOLCHAIN` (fix in `🏗️build/🔬️test-file/🏃️run/🟦️.ts`, details `cargo-test-runner-toolchain.md`). `📥️original-control` (13) only runs under its project target `original-control-contract-check` with a live Original process; under `repo test-file` it reaches the live store by design.
  - The first `--manifest-path` run stopped on two stale type paths in the PDF page editor (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/✍️authoring/🦀️.rs:1154`: `semio_plugin::kernel::InteractionState` → `semio_plugin::InteractionState`, `semio_plugin::ArtifactOwnedToolJobContext` → `semio_plugin::app::ArtifactOwnedToolJobContext`, as the other plugins use).
  - Full run: pending.

## TypeScript tests
- All 26 GIS TypeScript test files under `✏️s/🔌️plugins/🌍️gis` exit 0 via `repo test-file`: 43 tests pass, 0 fail (`gis-typescript-tests.md`).
- 8 files run 0 tests under `repo test-file` because they are proof modules (for example `proveGisMapTilesV1` in `🗺️gismap/🗺️tiles/🧪️tests/🟦️.ts`), not `bun:test` files. Each package's `📜️script.ts test` target calls them, so they are covered by the six-project `test` sweep, not by these counts.

## Gap 7: 65 × 65 grid
- Independent GDAL check `🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/verify-terrain-maximum-grid.py` (run through `repo tools exec uv -- run --isolated --with rasterio`): "GDAL AAIGrid: 65 × 65, 65536 bytes, 2 picker chunks, exact signed millimetres". Verified.
- `s-plugin-integration.md` updated from "33 rows of 33 signed integer heights in meters" to the 2–65 grid in signed millimetres (code: `🧬️schema/📸️snapshot/🦀️.rs` `ELEVATION_MIN_SIDE`/`ELEVATION_MAX_SIDE`, schema `🏔️elevation/🔣️.json` `rowsMillimetres` 2–65).
- Browser maximum-grid exchange: pending (needs gis3d on 6083).

## Gap 11: stale docs
- `terrain-editor-product-gaps.md`: items 1 (click-only brush) and 2 (fixed 33 × 33, whole metres) marked resolved in source, with the code locations.

## Gap 13: "17 map tests"
- No GIS map suite of exactly 17 tests exists. The only "17" is `terrain-acceptance.md:34` ("the ordinary suite passed 17 cases"), a Terrain suite. The map suites are `🗺️gismap/🗺️tiles/🌍️gis-policy/🧪️tests/🔬️unit/🦀️.rs` (37 `#[test]`), `🗺️gismap/📦️packages/🟦️typescript/🧪️tests/` and `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests`. The claim in `MAP/🎫️ticket.json` is unsupported; the counts here come from the current gismap run.

## Gap 4: brush binding and archive reopen
- Fixed in source: the shell refresh now publishes window bodies before the reserved sections, so a section fault no longer drops the Terrain body and its `terrainBrush` binding (`gap4-terrain-brush-archive.md`). The TS test passes.
- Archive reopen: History operation frames now carry format and variant tags (commit `b4927d2c311`). A regression test was added.
- Still open: the `window_measures` trap (`gap4-window-measures-trap.md`) and the archive load that keeps reporting running after a rejected edit (`gap4-archive-load-fault.md`).

## Gap 5: native terrain suite
- The 32-of-42 suite is `semio-s-artifact-gis-gisterrain-editor --lib`, not the 21-test root crate (`gap5-native-terrain-suite.md`).
- Fixes are in progress for the press-preview settle stall and contour close (`gap5-press-contour-fix.md`) and for the History receive stall (`gap5-history-receive-fix.md`).

## Gap 6: 2D envelope gates
- All the gate tests still exist. The drawing live-load turn budget was raised to 1,000,000. The gate script `🌙️09/☀️16/GIS-2D-END-TO-END-BUILD/📜️verify-gis2d-envelope-gates.sh` was rewritten to explicit `--manifest-path` commands with the current crate names. Native `component_cold_map_patch` runs only through `repo run @semio-tech/gis-plugin:component-cold-map-patch-native-check` (`gap6-2d-envelope-gates.md`).

## Gaps 9 and 10
- Gap 9: no source emits `no-operator-graph` any more. A dead `BENIGN_RE` alternative in the time-travel console digest was removed.
- Gap 10: the Viewer role is registered like the working viewers. The earlier timeout was a stale component failing to mount. The test `gis_manifest_publishes_one_editor_viewer_pair_per_artifact_dialect` was added (`gap9-10-shell-diagnostics-viewer.md`).

## Unrelated compile blockers fixed on the way
- `semio-s-artifact-stdio-pdf-view-lane` (new PDF transient crate) had no mutation descriptor for `👁️set-view`, so `MutationLeaf` derivation failed. Fix: `pdf-view-lane-descriptor-fix.md`.

## Launchers
- The s shell on 6070 is `@semio-tech/framework-os-dev:dev-s-react-prototype` (port table `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts`); `dev-gis3d-react-prototype` serves gis3d on 6083/6183. The handoff named the wrong target for 6070.
