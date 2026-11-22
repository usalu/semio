# GIS Plugin End to End: Open Gaps Audit

Read-only audit. No source files were edited and no build, test or launch command was run. Evidence comes from the ticket notes under `🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END`, `🌙️09/☀️16/GIS-2D-END-TO-END-BUILD`, `🌙️09/☀️23/*` and `🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END`, checked against the plugin at `✏️s/🔌️plugins/🌍️gis`.

Paths below use the short forms `TERRAIN/` for `🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/`, `2D/` for `🌙️09/☀️16/GIS-2D-END-TO-END-BUILD/`, and `MAP/` for `🌙️08/☀️29/GIS-MAP-END-TO-END/`.

## Prioritized open items

### 1. No sealed workspace graph blocks every launcher (blocker)
- Evidence: `TERRAIN/terrain-dev-integration-gates.md` says "Its graph currently has no sealed generation." `TERRAIN/native-terrain.md` reports "this machine holds no sealed generation of the whole workspace."
- Status: Every launcher (`dev-gis3d-react-prototype`, `run-gis3d-native-prototype`, `gis2d-wgpu`) waits for a sealed graph. Nothing in the notes shows this resolved.
- Next check: `repo show projects --graph-policy=current --lang=en` must exit 0 before any s or GIS launch is attempted. If it still fails, build the sealed generation first (`repo graph` or `repo show projects` without the current policy) and record the result in this ticket.

### 2. Full s shell has never run the terrain acceptance
- Evidence: `TERRAIN/s-shell-terrain-acceptance.md` says the runner "reported `ConnectionRefused` from `http://127.0.0.1:6070/`". `TERRAIN/terrain-results.md` says "Full Play activation did not reach the server."
- Status: Port 6070 (the standard s Dev row) was not serving. `verify-s-shell-terrain.mjs` (Catalogue → Editor drag, Workflow open, Editor/Viewer switch, persisted Terrain reload) has no passing run recorded.
- Next check: start `repo run @semio-tech/framework-os-dev:dev-gis3d-react-prototype --lang=en`, confirm HTTP 200 on 6070, then run `TERRAIN/verify-s-shell-terrain.mjs`.

### 3. The umbrella ticket has no verification evidence
- Evidence: `🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🎫️ticket.json` contains only `ticket.open`, with no `summary`. Its description requires "it builds, all its tests pass, it loads in s".
- Status: Nothing has been run or recorded under this ticket. The Rust crates have 111 `#[test]` annotations in gismap and 57 in gisterrain. The gis-js TypeScript package has two `*.test.ts` files and its `test` target is `repo ./📜️script.ts test`.
- Next check: after item 1 is clear, run per crate: `repo run @semio-tech/gis-js:test`, the gismap and gisterrain Rust test targets, and `repo run` for `bridge`. Record the outputs in `🗑️generated` and then summarize in this ticket.

### 4. Terrain brush in the live browser viewport and archive reopen are not working
- Evidence: `TERRAIN/terrain-acceptance.md` says "The browser viewport still fails to receive the brush binding after the broad Measures refresh." `TERRAIN/dev-preview.md` says importing an archive back "failed to restore the previous overlays on the current component."
- Status: Viewport brush drawing, Undo of contour overlays, and archive reopen are reported as failing or unverified.
- Next check: live Chrome run on the Terrain editor at `http://127.0.0.1:6083/?example=demo` (brush press, drag, Undo/Redo), then export and re-import an archive and compare overlay labels. Relevant files: `✏️s/.../🗿️artifacts/🏔️gisterrain/.../✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/` and `🧰️framework/.../🧱️elements/🗺️WorldTerrainLayer/🟦️.tsx`.

### 5. Native terrain test suites are not all green
- Evidence: `TERRAIN/terrain-acceptance.md` says "The full library run passed 32 of 42" and "still reported press preview and teardown failures, History metadata and replay failures, a contour lifecycle failure". A later note says "The newest completed mounted suite passed 22 of 22", which covers a different scope.
- Status: The mounted suite (22/22) and the full native library (32/42, with later fixes not re-run) disagree. "The native pointer journal and native registered-command test remain in progress."
- Next check: run the full native gisterrain library suite on the current checkout and record one number. Do not cite 22/22 as the full-library result.

### 6. The 2D gate suite is unfinished, and the Xcode blocker is only worked around
- Evidence: `2D/📓️verification.md` lists open gates: `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`, `drawing_live_envelope_submit_*`, `component_cold_map_patch` (native), and "Full `semio-s-artifact-gis-gismap` 240/240 | Blocked on live envelope test above". `2D/📓️status.md` names "Unaccepted Xcode license" as the current blocker.
- Current state (checked): `xcode-select -p` returns `/Library/Developer/CommandLineTools` and `xcrun --sdk macosx --show-sdk-path` resolves. The gate script `2D/📜️verify-gis2d-envelope-gates.sh` (lines 6–14) falls back to CLT, so the license blocker no longer blocks the native build.
- Status: Earlier full runs were "aborted (SIGTERM) after ~26 min" and the one-turn test passed only after a drain fix. The full gismap run has not been recorded as passing.
- Next check: run `verify-gis2d-envelope-gates.sh` with the fast gates first (the script already orders them that way) and capture the log under `🗑️generated`. Record the native `component_cold_map_patch` result separately.

### 7. The 65 by 65 rectangular elevation grid is in code but not verified end to end
- Evidence: `🗿️artifacts/🏔️gisterrain/.../🧬️schema/📸️snapshot/🦀️.rs:56` says "a rectangular grid with 2 to 65 samples per axis". `TERRAIN/s-plugin-integration.md` still says "33 rows of 33 signed integer heights". `TERRAIN/terrain-acceptance.md` says "Native and browser compilation and mounted maximum-grid exchange are outstanding."
- Status: The schema and Rust validator accept 2 to 65 samples. Staged patches (`terrain-rectangular-*.patch`) are in the ticket folder. The browser and native builds of this cut and the maximum-grid exchange are unverified.
- Next check: run `TERRAIN/verify-terrain-maximum-grid.py` and `TERRAIN/verify-terrain-maximum-grid-browser.mjs` after the native and browser builds. Update `s-plugin-integration.md` from 33×33 to the 2–65 range.

### 8. 2D LOD and vector-label tests have never run in the crate
- Evidence: `🌙️09/☀️23/GIS-2D-LOD-MODE-AUTOMATIC-SUFFIX/📓️summary.md` says "full crate `cargo test --lib` blocked by pre-existing `Arc<GisMapSnapshot>: ArtifactCompositionFields` errors". `🌙️09/☀️23/GIS-2D-VECTOR-TILE-LABEL-ANCHOR/📓️root-cause.md` gives `cargo test -p semio-framework-surface vector_label_anchor_uses_line_midpoint_for_transportation_names`, which is not recorded as run.
- Status: Both fixes are reported as `cargo check` only. The label-anchor test is named but no pass is recorded.
- Next check: run that one `cargo test -p semio-framework-surface` filter, then check whether the `ArtifactCompositionFields` errors in `semio-s-artifact-gis-gismap` still exist with `cargo check --tests`.

## Further items (lower priority)

9. **2D middle-button pan and console are unverified.** `MAP/📓️startup-verification.md` says "Browser middle-button panning has not been exercised" and "The shell still emits `no-operator-graph` contribution diagnostics". Check: middle-drag through CDP or Playwright and a clean console on port 6040.
10. **Viewer role in the live shell is unverified.** `TERRAIN/dev-preview.md` says the component "did not publish the Viewer role after 90 seconds". The viewer code exists at `✏️s/.../🏅️standards/.../👁️viewer/🦀️.rs`. Check: Editor → Viewer → Editor switch in `verify-s-shell-terrain.mjs`.
11. **Stale docs contradict the code.** `TERRAIN/terrain-editor-product-gaps.md` item 1 says the brush is "click-only", but `🧰️framework/.../🗺️WorldTerrainLayer/🟦️.tsx` now uses `TerrainBrushStroke` with pointer capture (lines 304 to 446). Item 2 (fixed 33×33) is also superseded by the 2–65 schema. Check: update both docs once items 4 and 7 are verified.
12. **Ticket summaries overstate the state.** `TERRAIN/ticket.json` summary says "The registered editor, live Chrome viewport, and focused native terrain host checks passed", while `terrain-acceptance.md` says the viewport still fails to receive the brush binding. Check: reconcile the summary once item 4 is resolved.
13. **The "17 map tests" claim cannot be located in the plugin.** `MAP/🎫️ticket.json` says "All 17 map tests and focused infrastructure checks pass", but no TypeScript map test file was found under `✏️s/🔌️plugins/🌍️gis`. Check: find the suite in the framework and record its path.
14. **Path length is within limit.** The longest file under the plugin is 221 characters, below the 256-character limit in CLAUDE.md. No action needed.

## Summary of verified versus unverified
- Verified in code: the 2–65 schema, the stroke pointer handling, the viewer source, and the CLT fallback in the 2D gate script.
- Not verified in this audit: any build, test, or browser run. The docs report that the sealed graph is missing, so the launchers cannot run in the current state.
