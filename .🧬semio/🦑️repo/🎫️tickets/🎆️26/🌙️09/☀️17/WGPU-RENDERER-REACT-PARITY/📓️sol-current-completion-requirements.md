# Current Renderer Completion Requirements

The requested end state is complete React/WGPU renderer parity, including placement, windows, all UI elements, interactions and customization. Passing the current implementation packets does not redefine that end state. This document records the execution requirements against the current source; it is not a completion claim.

## Evidence Required

| Requirement | Authoritative evidence | Current status |
| --- | --- | --- |
| Current source and artifact boot | Fresh renderer Wasm, generated browser isolates and selected guest component; activation receipt and live error-free accepted frames | Fresh React activation passed 4m53s; fresh WGPU activation passed 2m9s with source Wasm compilation executed. Both listeners serve. React normal browser workspace loaded. WGPU accepted-frame runtime remains under physical acceptance. |
| Baseline regression integrity | Complete browser and native WGPU gates, plus affected React laws and native kernel identity law | Full browser: fifty files / 540 tests passed, no skipped tests. Focused media: 27 passed. React typecheck passed. Complete native: 1599 executed, 1594 passed, five failed, eleven ignored. Focused repairs executing; kernel newly added malformed identity cases still require rerun. |
| Window topology and ownership | Thirteen neutral Dock destinations; normalized pointer drop; eight physical split/merge/reorder/cancel/template cases; independent app/window identity and actions | React Dock oracle: fourteen passed. Nine-vector spawned oracle: fifteen passed. All three focused native laws passed. Physical React: seven of eight cases passed; template active body remains skeleton. WGPU eight-case acceptance executing. |
| Chrome and panel placement | Full paired 57-step Puzzle journey with equal viewport/DPR/theme/session; divider displacement, pane body extents, focus/unfocus/close/reopen and immediate post-popup cap close | Previous checkpoints are historical. Current source routes exist; current physical runtime acceptance remains pending. |
| Every retained widget and layout | Paired pointer/keyboard/AX behavior and geometry for all current interpreter cases and six layout variants, including scroll/clipping/disabled/read-only/popups | Browser transport and source laws provide partial implementation evidence; a generic serializer pass does not prove each mounted interaction. |
| Customization | Both implementations consume authored compact spacing and named chrome/DOM metric multipliers; theme schema/vectors, independent CSS oracle, resulting live geometry | Shared fifteen-vector geometry model/schema/math and Rust twin implemented. Full styling: 71 tests / 2634 assertions passed. Mounted root law passed before final guards. Native valid vectors passed; two invalid array/draft laws being repaired. Localized named metric guidance has fail-first law pending repair. Physical paired customization still due. |
| Language and accessibility | Explicit English and German acceptance; semantic and physical activation produce the same mutations; focused editable Chat and widget controls; labels, focus and bounds are current | Current tests provide partial coverage. Fresh paired runtime checks remain due. |
| Progress, cancellation and presentation authority | Current bounded Worker/native laws and runtime input to accepted frames; stale generation/window/document events cannot publish or mutate retired targets | Browser gate passed for current transport snapshot. Full native and actual surface gestures remain due. |
| Multi-user/local-first behavior | Same app/document projection across React/WGPU/native, short disconnection/reconnection and event-driven edits with progress/cancellation | Existing registered collaboration/journey routes are source evidence until current execution and receipt inspection. |
| Developer integration | Bun/Nx commands and editor launch registrations, schema-owned generator/source closures, no new external runtime dependencies | Worker generation and check passed. Browser cache override law red then green, browser acquired and physical tests passed. Media, Dock and 57-step journey editor rows and required inputs registered in launch seed; registry regeneration passed and both JSONC files validated with zero missing inputs. Styling package target inference being corrected to preserve explicit router. Latest affected source closure checks still due. |

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
