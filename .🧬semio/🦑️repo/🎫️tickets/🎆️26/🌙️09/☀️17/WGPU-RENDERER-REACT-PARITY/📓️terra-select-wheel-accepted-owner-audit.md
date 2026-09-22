# Select Wheel Accepted-Owner Audit

**Scope:** Read-only source audit of the current WGPU Select-popup wheel scope and its immediate Shell route. No build or test was run.

## Result

The current implementation has the required accepted-frame ownership boundary. I found no additional source-proven production defect in the requested geometry, clipping, finite-delta, occlusion, or scope-swap paths.

The Shell wheel binding is present in the current shared source. It may have landed after the request that described it as intentionally pending, so this report does not treat its runtime behavior as verified.

## Ownership and publication

- The only popup wheel witness is private: WidgetInteractionMaps.select_popup_wheel: Option<SelectPopupWheelScope> stores owner, menu, max_scroll, and end_hit in [widgets](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs#L74-L82). It adds no public HitTarget field or item-ID parsing path.
- render_select_menu registers the scope only after it has registered every visible row and both chevrons. end_hit is therefore the exact first index that a later-painted target would receive. See [Select painter](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs#L515-L550).
- InputState.hit_index_at selects the last matching resolved target, the same paint-order winner as hit_at. A point covered by a later target returns an index at or above end_hit and the scope declines it. A menu gutter with no target has no index and remains owned by the popup. See [input registry](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs#L320-L355).
- On accepted presentation, Shell swaps widget_maps and widget_maps_staging in the same critical section as the retained hit maps and input.publish_hits. This keeps the scope and its hit ordering on one presented epoch. See [Shell acknowledgement](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L13028-L13040).
- The next frame’s setup retires the staging scope before it is rebuilt. Until an acknowledgement swaps that rebuilt map, the old accepted scope remains authoritative. See [Shell frame setup](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L22767-L22865).

## Geometry, clipping, and numeric bounds

- The maximum is the actual popup content extent minus the viewport interior: rows use the body line box plus two standard paddings; the viewport subtracts the popup’s own two standard paddings. The scope takes that same popup.max_scroll. See [Select geometry](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs#L61-L72) and [extent calculation](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs#L147-L152).
- The scope menu, scissor, and row geometry derive from the same SelectPopupGeometry. Item hit rectangles are clipped to the menu and exclude the chevron bands; chevrons are registered after rows and thus win their overlapping bands. See [geometry and clipped row hits](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs#L237-L273) and [painter](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs#L515-L549).
- Incoming vertical deltas are admitted only when finite and non-zero, then clamped to [0, max_scroll]; the scope still claims a finite-zero or non-finite event at an owned point, so no underlying region receives it. Generated immutable theme tokens supply the dimensional inputs; this audit found no source-reachable non-finite max_scroll producer. See [scope scroll](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs#L120-L134) and [theme construction](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs#L235-L281).

## Shell route and single-scope validity

- Shell.handle_pointer_wheel now checks the accepted Select scope before scene routing, retained-document dispatch, and generic ScrollRegion mutation. Thus a popup owns its menu and gutter while a later overlay at that point still wins through the end-hit gate. See [Shell wheel dispatch](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L13413-L13446).
- A single scope is consistent with the concrete Shell interaction authority: selecting a trigger first closes every currently open Select, then opens just the selected one. close_open_selects clears both scroll slots for every prior owner. See [trigger routing](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L14004-L14018) and [close authority](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L14678-L14688). A fixture may manually set several open_selects entries, but no normal trigger path admits that state, so it is not evidence that this private one-scope representation loses a reachable owner.

## Existing law coverage and one useful addition

The widget law already covers the important local protocol: before publication no scope owns input, after the paired hit/map publication item and gutter wheel mutate only the Select offset, non-finite deltas do not mutate it, the extent clamps at both ends, a later overlapping hit wins, and clearing the scope removes ownership. See [widget law](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-widget-metrics/🦀️.rs#L120-L155).

The new Shell law verifies the route’s precedence over an outer scroll region and preserves the accepted popup after the candidate open-state changes. It currently builds its map directly as accepted rather than driving the real Shell candidate/acknowledgement seam. See [Shell law](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs#L5-L33).

A narrowly stronger native regression, if Native146 does not already cover it, should use the real candidate paint/seal/ack path:

1. Present an open Select above a ScrollRegion; wheel its clipped menu gutter and assert only select.<owner>.scroll changes.
2. Build a candidate with that Select closed but do not acknowledge; assert the old popup still owns the gutter.
3. Acknowledge the replacement; assert the same point reaches the new underlying ScrollRegion, and add a later overlapping hit before the acknowledgement in a separate candidate to prove it blocks the popup.

That law targets the only cross-owner coupling at issue—the actual paired map/hit swap—without adding metadata to generic hits or parsing Select item IDs.

## Confidence and limits

**High confidence** for source ownership and geometry conclusions. **No runtime conclusion:** this audit did not run Native146, React, or a browser journey.

