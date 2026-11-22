# Gap 4: Terrain brush binding and archive reopen

Static analysis only. No build, cargo or browser run. One TypeScript file ran: `browser-actor-panels` (5 pass, 0 fail).

## (a) Brush binding after the broad Measures refresh

Binding flow:
- `terrain::render` (`.../🪟️windows/🏔️terrain/🦀️.rs`) writes `interaction.terrainBrush` from `view.active_utility_id`.
- That field comes from the host's per-window utility register, via `windowViewContext`.
- `handleSetActiveUtilityAction` (`ShellHost/🖱️shell-actions/.../🧰️utility-and-tool-activation`) writes the register, then asks for a partial body refresh of the addressed window.
- `World3dHost` parses `terrainBrush` with `parseTerrainBrushAction` and hands it to `WorldTerrainLayer`, whose `TerrainBrushStroke` pointer capture is correct.

Cause:
- `runUiRefreshPass` (`ShellHost/🔁️contributions-refresh/🟦️.ts`) sent window bodies and the reserved sections (engagements, measures, tools, catalogue) in one `refresh-ui` guest turn.
- Any fault while projecting a section (the `window_measures` trap reproduced in `terrain-browser-utility.md`) failed the whole turn. The already rendered Terrain body never reached the host cache or the live stores, so `data-interaction-json` kept no `terrainBrush`.
- That is why restricting the request to the addressed body returned the binding while the broad request did not.
- The Terrain editor itself returns no `setActiveUtility` effect, so the broad scope came from other passes that coalesce onto the utility press.
- I could not identify the `window_measures` trap itself statically. The refresh pass is now robust to it.

Fix:
- New `uiRefreshSectionsOnlyV1` in `ShellHelpers/🛣️route-and-chrome/🟦️.tsx`, sibling of `withoutUiRefreshSectionsV1`.
- `runUiRefreshPass` crosses bodies first and applies them to the cache. A second crossing fetches the sections inside a try/catch. The section fault is rethrown only after bodies, stores and owed effects were published, so the lane still records the failure.
- Actor-served sessions are unchanged (their sections were already stripped).

Test (run, passes): `SEMIO_LANGUAGE=en repo test-file "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🎭️browser-actor-panels/🟦️.ts"`. New case: "crosses window bodies and reserved sections as two requests so a section fault cannot discard the bodies".

Still needed: live Chrome run with `verify-terrain-browser.mjs` (waits for `terrainBrush.arguments.operation === "raise-lower"` after pressing Raise Terrain), and a root-cause fix for the Measures trap once a native log names it. The guest's `ScopedWindowView::drop` -> `return_scoped_alias` asserts in `window_measures` / `window_engagements` are the first suspects.

## (b) Archive reopen does not restore overlays

Cause (from `terrain-archive-mutation-wire-witness.md` and `terrain-browser-archive-execution.md`):
- The component that exported the archive wrote History operations as a container-less generic record, with the historical external variant name and no format or tag prefix.
- Its original receiver (`original_pack_value_operation_receiving!` without `tagged_key`) expected a complete Pack container. It read ASCII bytes as the header CRC at offset 20 and refused: `checksum mismatch in header at offset 20`.
- Hydration therefore rejected at `DecodeForward` of the first edit. The archive load stayed `running` at 483/484 and never reached document adoption, so no overlays were restored.
- Writer and receiver disagreed inside one component. The archive was not corrupt.

Fix (already in source, commit `b4927d2c311`, verified by reading):
- `GisTerrainMutation::encode_op` uses `tagged_value_binary` with `VariantTag::Key`.
- `original_pack_value_operation_receiving!(GisTerrainMutation, tagged_key = COMPONENT_PROTOCOL_SEMIO)` uses the same protocol.
- No compatibility path was added (greenfield rule). Archives from the old component must be re-exported.
- This session added a Rust regression pinning the frame shape: `operation_frames_carry_format_and_variant_tag_before_the_record_body` in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`. It checks that bytes 0..2 are `[1, tag]` for all three variants and that decode round-trips. The format byte `1` is taken from the witness note; the first run confirms it.

Open defect (not fixed): after `reject(MemberOpenDiagnostic::...)` in hydration, the archive load keeps reporting `running` instead of failing. A bad archive stalls instead of surfacing a fault. It needs a native law (rejected hydration step ends the load with a typed fault) in `🧰️framework/.../🏪️store/🧾️document/📜️history/💧️hydration` and the archive-load owner. I did not edit it without a way to run it.

## Tests to run (not run here)
Rust (use the coordinator's serialized runner):
- `operation_frames_carry_format_and_variant_tag_before_the_record_body` (gisterrain binary mutations, new)
- `authored_elevation_operation_and_snapshot_round_trip`, `op_binary_round_trips_and_agrees_with_text` (same file)
- `edited_terrain_archive_reopens_with_replayable_raster_and_overlay` (`✏️editor/🧪️tests/🧩️mounted/📦️archive`): the end-to-end archive law
- `terrain_brush_diff_applies_reverses_and_refuses_an_invalid_change` and the unit test that asserts `interaction["terrainBrush"]["action"] == "editElevation"` for `terrainSmooth` (editor window unit tests)

Browser: `verify-terrain-browser.mjs`, then `verify-terrain-exchange-browser.mjs --archive` against a freshly built component. Export the archive from that same build before reopening it.

## Files
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔁️contributions-refresh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🎭️browser-actor-panels/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
