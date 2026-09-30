# Current Renderer Completion Requirements

The requested end state is complete React/WGPU renderer parity, including placement, windows, all UI elements, interactions and customization. Passing the current implementation packets does not redefine that end state. This document records the execution requirements against the current source; it is not a completion claim.

## Evidence Required

| Requirement | Authoritative evidence | Current status |
| --- | --- | --- |
| Current source and artifact boot | Fresh renderer Wasm, generated browser isolates and selected guest component; activation receipt and live error-free accepted frames | Wasm source preflight passed; fresh published binary pending theme projection completion. Old staged binary faults before app admission. Prior listeners disappeared during app restart and are being rebuilt. |
| Baseline regression integrity | Complete browser and native WGPU gates, plus affected React laws and native kernel identity law | Browser snapshot: 50 files / 536 assertions passed. Latest focused media snapshot: 26 passed. React media snapshot: eight passed. Full native green: 550 passed, two media assertions failed, remaining tests not executed. Repairs and rerun pending. |
| Window topology and ownership | Thirteen neutral Dock destinations; normalized pointer drop; eight physical split/merge/reorder/cancel/template cases; independent app/window identity and actions | React Dock oracle: fourteen passed. Nine-vector spawned oracle: fifteen passed. Focused native green is running. Physical accepted-frame receipts remain pending fresh binary. |
| Chrome and panel placement | Full paired 57-step Puzzle journey with equal viewport/DPR/theme/session; divider displacement, pane body extents, focus/unfocus/close/reopen and immediate post-popup cap close | Previous checkpoints are historical. Current source routes exist; current physical runtime acceptance remains pending. |
| Every retained widget and layout | Paired pointer/keyboard/AX behavior and geometry for all current interpreter cases and six layout variants, including scroll/clipping/disabled/read-only/popups | Browser transport and source laws provide partial implementation evidence; a generic serializer pass does not prove each mounted interaction. |
| Customization | Both implementations consume authored compact spacing and named chrome/DOM metric multipliers; theme schema/vectors, independent CSS oracle, resulting live geometry | Confirmed split authority being repaired by coordinated React/theme and native-shell lanes. |
| Language and accessibility | Explicit English and German acceptance; semantic and physical activation produce the same mutations; focused editable Chat and widget controls; labels, focus and bounds are current | Current tests provide partial coverage. Fresh paired runtime checks remain due. |
| Progress, cancellation and presentation authority | Current bounded Worker/native laws and runtime input to accepted frames; stale generation/window/document events cannot publish or mutate retired targets | Browser gate passed for current transport snapshot. Full native and actual surface gestures remain due. |
| Multi-user/local-first behavior | Same app/document projection across React/WGPU/native, short disconnection/reconnection and event-driven edits with progress/cancellation | Existing registered collaboration/journey routes are source evidence until current execution and receipt inspection. |
| Developer integration | Bun/Nx commands and editor launch registrations, schema-owned generator/source closures, no new external runtime dependencies | Media acceptance and browser/Wasm check metadata registered and JSONC parsed. Source closure generation/check passed at its previous snapshot; latest edits require revalidation. |

## All Surface Families

The current UI contract declares exactly fifteen surface tags in `🖱️ui/🧬️contract/🗺️surface/🦀️.rs`. Every family needs a normal app-backed producer, accepted host, visible content, physical pointer/wheel/keyboard action and its exact owning app/window consequence. Renderer-only native scenes do not replace that receipt.

| Contract tag | Current production route to validate |
| --- | --- |
| world-3d | Puzzle3D Top/Perspective and the five existing World gestures |
| board-2d | Puzzle2D |
| node-graph | Flow authored Demo |
| canvas-2d | Draw and Layout authored Demo |
| text-editor | Playbook Source, including real caret/typing/commit |
| table | Playbook Steps |
| paint-2d | Raster authored Demo |
| virtual-file-system | Space: establish an ordinary app-backed hierarchy and navigation/selection authority; the historical root-only specimen is insufficient |
| tiled-map | GIS2D authored Demo, recording tile readiness separately |
| icon-render | Shooting preview, actual app save/export |
| ink-canvas | Note authored Demo and cancellation/next-commit |
| graph-timeline | VCS History |
| block-list | Playbook Builder |
| diff-view | Playbook Changes |
| event-feed | Playbook Activity |

The earlier proposal to add a synthetic DiffView/EventFeed fixture app is superseded by the current Playbook producer. The fifteen-family list describes renderer capabilities, not a restriction to fifteen plugin catalog rows. Current registry variants must be inspected for representative routes and relevant boot coverage.

## Acceptance Boundaries

The complete parity verdict already refuses zero compared pixel regions, missing or zero structural nodes, skipped behavioral steps and any failed axis. Its default behavior suite exercises an app-declared state change; it does not cover the full chrome journey or every surface gesture, so additional paired receipts are required.

Media-app viewers currently emit an explicit unsupported export capability in both renderer paths. Actual app-backed unsupported presentation must match; separately, the production browser media host must physically decode WAV/MP4 and preserve player/time/object URL through occlusion. This shared app capability boundary is not permission to claim fabricated app playback or omit renderer-host playback.

Embedded multi-shell introduction suppression remains an explicit source difference mentioned by the WGPU Shell. The ordinary single-shell journey cannot prove that topology. Its relevant host contract and a concrete acceptance route must be investigated before making an unrestricted renderer-equivalence claim.

No item above is accepted solely because an implementation exists, a locator call returns, a test process exits before assertions, a previous checkpoint passed, or another logical workstream is active.
