# Drivers disclosure Up-flow hit geometry and retirement

## Runtime evidence

Checkpoint 16 proves a generic retained Up-flow geometry mismatch.

The closed Drivers disclosure publishes its exact qualified header hit at about `y=612.8`. After the section opens, that same qualified hit moves to about `y=396.8`, although the rendered header remains at the bottom near `y=612.8`. The incorrectly translated hit overlaps the Save Driver row. A physical close attempt at the published header center therefore resolves the Save button and dispatches `saveDriver`.

Evidence:

- `🗑️generated/astra-runtime/checkpoint-16-full/wgpu/console.txt:6167-6194`
- `🗑️generated/astra-runtime/checkpoint-16-full/wgpu/console.txt:162051-162076`
- `🗑️generated/astra-runtime/checkpoint-16-full/wgpu/17-settings-drivers-open.png`
- `🗑️generated/astra-runtime/checkpoint-16-full/wgpu/18-settings-drivers-close.png`
- `🗑️generated/astra-runtime/checkpoint-16-full/steps.json`

React keeps the expanded section header in its bottom band. Its `TreeSection` renders `sectionContent` before `sectionTrigger` when direction is `up`. WGPU paint already mirrors that: `retained_tree_node_step` derives the header Y from the full section height when block flow is reversed. The retained hit translator instead always used the expanded section container’s top.

## Fail-first and repair

`an_open_up_flow_disclosure_keeps_its_header_below_its_children_and_retires_them_when_closed` mounts the production General Tree in the real bottom-right panel. It opens Drivers through the retained pointer path, settles accepted layout and paint, requires the expanded header below its real save-label child, requires `hit_at` at the header center to return the exact qualified disclosure id, physically closes the section, and verifies child retirement from the host registry and retained owner map.

Native54 supplied the intended red receipt: the expanded Up-flow header hit was `y=516.8 h=24`, above the save-label child at `y=544.8 h=16`.

The repair passes each retained window’s block-flow direction through both normal `frame_step` and hosted `frame_into_step` hit walks. `retained_tree_section_header_band` now places a reversed section header at `rect.y + rect.h - headerHeight`, matching paint, while Down flow remains at `rect.y`.

The schema-owned `upFlowDisclosure` vector pins a normalized expanded section, child, and bottom header band. The React oracle validates that geometry and independently checks the actual `TreeSection` source orders content before trigger for Up flow. Focused React results: 10/10 passed via Nx/Vitest. Rust verification is root-owned: UI59 covers the direct header-band law and Native55 covers the real Shell pointer/retirement law.

Native55 executed 1,238 renderer tests. The geometry and flat-registry ownership assertions passed, but the final physical close still left the Drivers child published. The remaining mismatch was inside `EventRouter`: its recursive target resolver continued to test every disclosure against a top-edge `SECTION_HEADER_HEIGHT` rectangle. The press therefore reached the correct host registry entry and then captured the panel root instead of the Tree section row.

The follow-up repair moves the section-band formula into the shared layout module and passes the router's actual `UiFlow` plus `TreeRowMetrics` through its whole recursive hit walk. Press, release, hover, drag/drop, scroll, and overlay subtree resolution now use the same bottom header band as paint and retained host publication. `an_up_flow_tree_section_routes_its_bottom_header_through_the_event_router` pins the direct retained route: a pointer at the bottom header must capture the section and close it on release.

Native57 reached that path. The bottom-header capture and current host-registry child retirement passed. Its sole Drivers assertion found the child only in `retained_hit_windows`, the Shell test helper's host-side staging map. The synthetic helper had omitted the `FrameSetup` clear that production `render_chrome_step` performs before every chrome walk, so the prior synthetic walk's staging entries were being appended to the next publication. The helper now clears `retained_hit_windows_staging` at the same boundary. No production retirement workaround was added.

## Display branch follow-up

The checkpoint-16 Dock Display failure is related but not fixed by the section-header translation alone. Its published registry contains:

- section header `y=852.8`;
- kind row `y=876.8`;
- parallel row `y=900.8`; and
- perspective row `y=924.8`.

Native57's corrected mounted Display law settled its real 300×240 accepted layout and reached the intended missing-gutter assertion. The published visible branch had a label and transfer handle for Parallel but no `tree.chevron` entry. That receipt separates the nested disclosure omission from the earlier stale screenshot timing.
