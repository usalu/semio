# Browser Journey Plan

This is a source-grounded execution plan. No current Lowpoly browser has run. Selectors below are candidates established by existing host test code, and must be confirmed in the first live DOM/accessible-tree observation before use.

## Prerequisites And Evidence

Wait for the coordinator's exact local URL after current catalog publication and declared Lowpoly activation. Use the retained probe's contained persistent profile and explicit locale. Require a successful host readiness beacon, loaded Lowpoly editor/viewer entries in the observed catalog, no failed plugin/resource requests, and visible nonzero scene canvases. Inspect the screenshot and semantic scene evidence separately: canvas presence and matrix structural digests do not prove rendered geometry.

The existing public helper owner is:
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts`.

Reuse exported awaitBeacon, dismissIntroduction, windowIds, readShell, witness, unfoldActionsRail, fillStagedArgument, submitStagedVerb, and seatLocale on the already-created contained browser context. Do not call runProgramMatrix directly: its own browser launch currently hardcodes Metal and does not preserve this ticket's profile/temp containment. Do not import its NOISE filter: this boot probe permits no hidden resource-error allowlist.

Physical host interaction patterns also exist in `🧪️testing/🚶️journey/🟦️.ts`. Use its observed controls and actual canvas bounds for trusted clicks/drags. Read-only catalog/input-ledger observations are evidence; do not invoke hidden dispatch functions or rewrite app state.

Before/after every mutation: save the action ledger cursor, history/check-in witness, object rows, relevant geometry or paint evidence, errors, and screenshot. Require the intended action and observable result. An action being dispatched alone is insufficient.

## Source-Established Host Controls

- Host readiness: documentElement data-semio-os-ready/error/not-found.
- Palette input: [role='dialog'] [data-slot='command-input']; rendered items expose data-command-item-id.
- Exact app spawn IDs: spawn.lowpoly.s.lowpoly.lowpoly@1/*#editor and corresponding #viewer; confirm both against the observed public catalog and visible palette.
- Window discovery: data-window-id; bodies: data-slot=window-body; action pane: data-slot=window-action-pane.
- Action rows: id=action.<verb>; staged argument controls end in .arg.<key>; exact execute controls end in .action.<verb>.execute.
- unfoldActionsRail handles engagement toggles. Confirm the matching pane/window before using it.
- fillStagedArgument can silently choose a live fallback option; acceptance must verify the requested value is actually seated. submitStagedVerb has broad fallbacks; bind the exact intended execute control before calling it.
- History helper reads framework.history.entry.* rows plus #s-checkin. ReadShell/witness track history and structural rendering, not mesh geometry.
- Locale seating must verify the actual document lang and localized controls, rather than trusting browser locale alone.

## Concrete Journeys

| Journey | Supported input/action | Required result |
| --- | --- | --- |
| Editor boot | Observed editor palette entry, or already-activated document | Real model visible; correct app identity; no boot errors; screenshot/semantic scene inspected |
| Viewer boot | Observed #viewer palette entry | Same initial geometry; read-only surface; absence of editor mutation affordances; locale-selected Mesh/Netz label |
| Create | Observed catalogue primitive or staged addPrimitive(kind=plane/cylinder) | One additional object with visible geometry, one history edit; initial object unchanged |
| Select | Actual document tree row or geometric pick in observed canvas | Intended object selection visible; face/edge/vertex granularity controls update as requested |
| Transform | Observed transform action with explicit dx/dy/dz, or physical gumball | Only selected object/components move; geometry evidence changes; one edit |
| Duplicate | Observed duplicateObject after selecting the new primitive | Fresh object identity, same topology, visible X offset; exact object count and independent selection |
| Edit | Pick a real face; observed extrude with explicit extrudeDistance | Side faces/shape visible; managed geometry changes; literal-source retention is a native assertion |
| Undo/redo geometry | Exact observed undo and redo controls | Shape and object count return to recorded states, then reapply; avoid matrix neutral selection dispatch that would perturb the journey |
| Paint | Observed brush and layer controls; one physical surface drag | Visible paint change, one committed edit; undo/redo restores/reapplies paint; aborted gesture adds no edit |
| UV | Observed Paint/Malen mode and UV window | Both Model and UV canvases visible; UV wireframe/paint correspond to active object; paint through observed UV input updates model |
| Layers | Observed addPaintLayer and layer row | New layer visible; selecting layer controls where subsequent paint is written |
| Reload | Reload same URL/context after committed geometry and paint | Same document identity and recorded edited geometry/paint survive; no replacement fixture silently seated |
| Import | Observed loadMeshRequest button plus Playwright filechooser event | Feed retained small OBJ/PLY/STL input via the real picker; matching imported geometry/name, valid app rendering, expected whole-document replacement |
| Export | Observed exportMesh with requested format plus download event | Correct extension/MIME as observable; nonempty downloaded geometry; independent parser validates object/face/position data; then real picker re-import reproduces geometry |
| English/German | seatLocale with explicit en/de and verify document lang | Locale-selected status, placeholder, failed-load labels and window labels; shared preset displays Paint / Malen in both locales |

Do not invent a named Model preset: current Lowpoly only has a named paint preset. The editor's Model/Modell and viewer's Mesh/Netz titles resolve from bilingual window-kind definitions without authored English title overrides.

For file input, prepare tiny retained fixtures under the ticket, and put downloaded files under generated/d. Use Playwright's actual filechooser/download events around the confirmed UI controls. Re-importing export output through the picker proves host wiring beyond native media tests. Validate exported format content with the existing independent IO oracle where applicable; do not count filename or file size alone.

## Completion Gates

Initial DOM observation may require adapting selectors to real composed window IDs, mode controls, panel scopes, canvas bounds, and accessible names. Do that only from observed current DOM. Preserve viewer, UV, both locales, real picker/download IO, and reload acceptance explicitly. Report absent controls/refused actions as failures or remaining gaps, never as skipped successful journeys. Keep generated evidence under the ticket, retain input scripts/fixtures and Markdown reports, and do not claim cross-platform execution beyond the host actually exercised.
