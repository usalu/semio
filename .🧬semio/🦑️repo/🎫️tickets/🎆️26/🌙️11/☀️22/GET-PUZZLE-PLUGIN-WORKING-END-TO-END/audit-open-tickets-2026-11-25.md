# 🔎️ Open Puzzle Tickets (read-only Haiku audit, 2026-11-25)

Reported by a subagent; only partly read in full, nothing re-run. Base: `🎆️26`.

## Open with substantial remaining work

- `09/02/PUZZLE-3D-END-TO-END`: reconcile reservations (32 MiB admits 3 of 13 surfaces, B56), import-distinct red (B57), Actions-pane fold toggle timeout, 61 BatchOnlyPendingRewrite actions.
- `09/06/PUZZLE-2D-END-TO-END`: battery 2026-09-17 33 pass, 4 fail (engagement-move, undo ledger, import round trip, inspector fresh).
- `09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D`: 2D 849 pass, 29 fail; 5D 35 BatchOnlyPendingRewrite verbs.
- `09/23/PUZZLE-5D-BACKGROUND-CLICK-CLEARS-3D-SELECTION`, `09/23/PUZZLE-5D-PART-DRAG-PREVIEW-IN-BOTH-WINDOWS`: no fix or evidence found.
- `11/04/ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END`: last 3D run 314 pass, 107 fail; disk full; deletions and killing stale graph hosts were denied by the permission classifier and need the dev.
- `11/04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END`: ten gates open (IO, media routes, SQLite, large export cancellation, wasm, 20 browser cases).
- `11/18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END`: official wasm build exit 1, candidate not applied.

## Likely fixed, verification not seen

Ten single-report 3D tickets of 09/13 to 09/16 without `🎫️ticket.json`; `09/16/PUZZLE-3D-ARTIFACT-TREE-PAGING` names tree virtualisation as future work.

## Not in `🗺️coordination-plan.md`

Interactive-job migration, large import and export, reconcile reservations, the 2D battery, the 09/23 bugs, artifact IO gates. `🌩�11/…` is a stray folder with a damaged name and one log.

## Single-report 3D tickets of 09/13 to 09/16 (second Haiku audit, source reading only, nothing run)

- All ten fixes have their decisive code in the tree, or a stronger replacement: frame overlay is now never offered; tree paging was replaced by virtualised streaming (no continuation rows).
- Open detail: `apply_reference_image_bytes` (world interaction rs line 165) has test callers only; the renderer pump uses `apply_decoded_reference_raster`. Dead production path plus a stale comment in `brush-lod-and-style` line 293: decide and remove.
- `[DEBUG] ` in `🧰️framework`: 24 files, 31 lines, 0 in `🌎️hub`; some are tests, fixtures or docstrings. Runtime ones may belong to agents that are debugging right now (one is lane p1's own), so none is removed before its owner is known.
- `INTERACTIVE_COMPONENT` grants 64 items, `INTERACTIVE_TURN_WORKER_WALL_US` is 50 ms: the two halves of the change in `continuation-2026-11-23.md` no longer match (read by the coordinator at 09:14).

## Artifact IO gates (read-only Haiku audit of `26/11/04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END`, 2026-11-25; nothing run, results of old matrices not verified)

- Tests and laws exist by name for hub surface (10 functions in `🌎️hub/…/🧩️puzzle/🧪️tests/🔬️surface`), physical SQLite, native text and pack, declared JSON (2D, 3D, 5D), media laws for 2D (SVG, page formats, DWG, TXT) and 5D (PNG, TXT, ZIP), 3D pane smoke and typed store, and the hub `guest-io` export and cancel test.
- Not located by name: a DXF law, a 2D PNG law, the 22 module laws of the 2D mutations, the `editor-gestures` target, the 2D inferences `declared_json_` selector.
- The 2D, 3D and 5D standards owners are excluded from the puzzle root workspace: they need explicit `--manifest-path` runs. `guest-io` panics unless SEMIO_TEST_ACTOR_COMPONENT, _RECEIPT and _DESCRIPTOR are set, so it needs the component build.
- Follow-up for p1 to p3 when they can run: include the standards-tree owners, not only the workspace crates.

## Interactive migration, resident limit, large import (read-only Haiku audit, 2026-11-25; nothing run)

- `BatchOnlyPendingRewrite`: no row left in the 2D, 3D, 5D editors (rows: 58, 63, 78 Migrated); only tests name it. Not checked: actions without any row.
- `UI_RESIDENT_AGGREGATE_BYTES` is now derived (slots times document bytes), and a surface reserves a floor instead of 8 MiB, so by arithmetic 13 surfaces fit; the sizes were not measured and the framework test `thirteen_mounted_surfaces_at_their_real_sizes_…` was not run.
- Import: total 262144 bytes, raw wire 532480, oversize and malformed answers publish a notice. `PUZZLE_COMMAND_WIRE_SCAN_STRIDE_BYTES` is reported as having no consumer: checked by the coordinator below.
- Coordinator check of the stride constant (grep and reading, nothing run): definition and one doc mention only, so it is dead; the paged step now lives in the framework wire code (`🎮️command/🎟️typed/📨️wire` line 30 steps by `TOOL_WIRE_PAGE_BYTES - page_len`). To do when 3D compiles: remove the constant and fix the doc line in `import-snapshot`, then run the import laws.
