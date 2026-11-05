# E2E Source Gaps

Read-only acceptance inventory against browser-journey-plan.md, 2026-11-05. No browser/native run.

## Confirmed Identity And Layout Wiring

Kit lowpoly_controller_id derives canonical editor ID once with surface_app_id(LOWPOLY_DIALECT,Editor), used by lowpoly_action and lowpoly_window_action. Mounted progress fixture independently compares actual assembled definition and EditorApp surface identity. These remove prior lowpoly-play literal mismatch in inspected paths.

Paint mode defines localized Paint/Malen and binds its named paint layout; layout is Model+UV, shared title Paint / Malen. Journey plan correctly warns there is no named Model preset. Exact browser spawn palette IDs still need observed catalog/DOM verification; source plan is not executed spawn evidence.

## Media Coverage

Manifest offers exactly OBJ/PLY/STL export options, required format with obj default. Args bridge maps value to format and defaults obj, imports filename/contents aliases into name/payload. Media serializer/deserializer helpers support same three extensions. No extra advertised UI format found in this scope.

Mounted import.rs now exercises public kernel chunks for padded OBJ, raw and dataUrl forms, and captures exactly one LoadDocument without discarding effects. It asserts no partial replacement, source snapshot/history mutation count unchanged, decoded pack objects/faces/positions. It does not apply real host archive loader or prove persisted replacement/reload. PLY/STL imports are helper coverage, not public picker/typed staged journeys.

Media unit export_round_trips_through_every_format calls export_mesh_text/import_mesh_bytes directly for each format. Only export command is dispatched using context::dispatch, whose settle loop drains and discards effects. Thus current tests do not verify public string export action -> exact DownloadMediaExport payload -> browser download -> independent parse -> picker reimport. Browser journey plan retains this valid gap.

## Host Download Path

React ShellHost host-effects main dispatch:130 routes downloadMediaExport to applyDownloadMediaExportEffect. Export-effects:17 reads filename,mimeType,data,encoding and calls direct downloadMediaExport for inline data; segmented marker follows plugin chunk drain. Lowpoly emits inline DownloadMediaExport with encoding None. WGPU settings-and-dock-persistence:113 also handles DownloadMediaExport. No missing host download dispatch arm is established by this source audit.

## Concrete Metadata Defect

Lowpoly manifest:208 marks exportMesh destructive. Actual export handler only emits DownloadMediaExport; native unit explicitly checks unchanged document/history. The destructive metadata contradicts the operation's source behavior and may affect user-facing destructive action presentation. No live confirmation/dialog behavior was observed, so that UI consequence is not asserted.

## Remaining Browser Acceptance

Unproven current host behaviors remain editor/viewer spawn, visible real geometry, trusted selection/gumball/paint/UV input, aborted gesture zero trace, layer interaction, reload persistence, both explicit locales, actual chooser import and actual download/reimport. Native helper assertions and projected controls cannot stand in for those journeys. No new runtime defect beyond metadata mismatch is inferred merely from missing execution.

## Cache Correction

Root's explicit command-crate source glob is now present as fourth test-runtime input and helper selects semio-framework-plugin-command. target-cache-audit.md amended with verified correction.

## Destructive API Follow-Up

Builder /🏗️builder/🧩️app/🎬️actions-commands-and-tools/🦀️.rs:101-113 defines destructive as discarding unreconstructable content; it sets effects.destructive and policy.approval WhenDestructive unless Always, affecting MCP approval. shell_action:16-18 expressly covers export with no document mutation. Thus exportMesh marker is a functional policy mismatch, not only visual styling. Regression home is editor /🧪️tests/🔬️unit/🦀️.rs manifest sanity at129; check actual assembled export action kind Shell, destructive false and approval Never. Peer raster editor /🔌️app-assembly/🦀️.rs:139 declares shell_action exportPng with no export destructive marker found.

Prospective public download regression: dispatch_fixture_action exportMesh format for OBJ/PLY/STL; reuse import.rs custom effect capture pump, require one DownloadMediaExport and exact metadata, independent parse/roundtrip plus unchanged snapshot/history. Do not use context settle helpers which discard the effect. Root informed; no extra source ownership assumed.
