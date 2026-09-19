# 📓️ App Boot Defects Found by the Play Acceptance Suite

Acceptance run 3 (2026-09-19, 2 s settle after `data-shell-ready`, failing on page errors, console errors and
refused inputs): 48/59 passed. Full per-app console captures: `🗑️generated/console/<variant>.txt`
(probe: `🧪️probe-console.mjs <base> <outDir> <variants…>` against the play dev server on :6033).

| App (variant) | Symptom | Class |
|---|---|---|
| CAD (`cad`) | `input #1 setActiveExample refused … interactive-job classification BatchOnlyPendingRewrite` | action classification |
| Procedural 2D (`generation2d`) | `setContributions … rejected 62090 raw bytes before decoding; maximum is 12288` | contributions cap |
| Process 3D (`process3d`) | `setContributions … exceeds its exact output cap`, then `setActiveExample` closure-rejected (Incomplete) | contributions cap + closure |
| Architect (`architect`) | `setActiveExample` → `loadDocumentArchive` closure-rejected (Incomplete) | composed-child roster |
| Writer (`writer`) | same | composed-child roster |
| Animate (`animate`) | same | composed-child roster |
| Shooting (`shooting`) | `setActiveExample` → `artifact-store.persisted-initializer-refused` | store initialization job |
| Raster (`raster`) | guest panic `Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority` (`🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:369`) | owned-map serialization |
| Flow (`flow`) | guest panic `ordered-map root must be explicitly retired before drop` (`🌱️value/🗂️ordered/🦀️.rs:81`), then `runtime instance authority is busy` | value lifetime |
| Block 2D (`block2d`) | `typed-operation progress refresh failed … plugin.internal.prior-outcome` (origin fault unlogged) | typed operation |

Fixed in this ticket before run 3:
- Puzzle 5D: `Board2dHost` threw "no registered board session factory" — puzzle registered board factories for
  puzzle2d only (`◻️2d/…/✏️editor/🌉️wasm/🟦️.ts`); puzzle5d editor/viewer added.
- Terrain (`gis3d`): demo fixture mesh handle target `gisterrain-mesh` ≠ child id → `InvalidReference` panic.
- Eight apps without a `setActiveExample` action got a dead example picker and a dropped boot announcement:
  `appSwitchesExamples` (ShellHelpers) now gates `exampleOptions` in ShellHost.

## Resolutions
- Raster: `document_sync_json` (`✏️editor/🦀️.rs`) serialized the whole snapshot incl. the populated `assets`
  `RasterOwnedMap` on the first composite render → guard panic. Now built field by field; new test
  `raster_scene_projects_populated_owned_maps_without_wholesale_serialization`. Probe 0. Follow-ups: the
  composite canvas shows no visible content for the demo in the browser pane; JSON export serializer and
  mutation JSON bridge still serialize whole snapshots (not on the boot path).
- Procedural 2D: contributions route declared the old 12 288-byte ceiling; now `COMMAND_MAXIMUM_BYTES` like
  generation3d. Test `contributions_route_declares_a_reachable_wire_ceiling`. Probe 0.
- Process 3D: config-lane output cap 16 384 < retained contributions lane 24 576 → `PROCESS3D_RESUMABLE_OUTPUT_BYTES`;
  timber demo + drilled-plate fixtures carried stale content-hash child ids (old B-Rep serialization) → `Incomplete`;
  regenerated. Tests: `the_real_contributions_push_passes_the_host_configuration_output_gate`,
  `every_example_loads_through_the_member_less_archive_door`, `every_example_fixture_carries_its_canonical_child_handles`.
  Probe 0 (generation3d stays 0). Disk hit ENOSPC mid-run; pruned stale caches (+177 GB).
- Flow: render/eval/inference paths dropped `FlowHost`/`FlowHostSnapshot` (and `FlowEvalSession`) without
  retiring them → ordered-map drop guard panic inside the instance owner → "authority is busy". Helpers
  `with_host_from_snapshot`/`with_live_host_snapshot`; test `booting_renders_and_evaluates_without_dropping_a_live_flow_owner`.
  Probe 0. Follow-ups: flow test app never finishes closing (teardown); rename/patch-widget paths still leak.
- Block 2D (and 5D, same latent defect): store preflight declared `work_items: 1` (fold needs forward+inverse = 2)
  and no document-store retirement owners; added `♻️retirement` module + owners like block3d. Harness now binds
  the instance id. Block2d 34/34, block5d 33/33 editor tests. Probe 0 (block3d stays 0).
- Architect / Writer / Animate: composed `s.stdio.semio` child slots without `genesis_child_pack` + `type Members`
  on editor and viewer → archive closure `Incomplete` for the shell's member-less load. Added
  `genesis_program_child_pack` / `genesis_writer_child_pack` / `genesis_presentation_child_pack`, `SemioMembers`
  rosters in `dyn_enum_close!`; animate also gained config/draft/presence/transient owners. Test
  `demo_example_load_settles_through_the_host_document_archive_door` per app. Probe 0/0/0. Follow-ups: derived
  children load empty (content not persisted in the parent pack); writer demo asset child id ≠ target id.
