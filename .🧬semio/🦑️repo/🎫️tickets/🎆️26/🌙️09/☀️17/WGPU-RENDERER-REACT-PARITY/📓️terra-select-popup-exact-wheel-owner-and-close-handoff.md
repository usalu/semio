# Exact Select Wheel Owner and Component-Close Handoff Audit

Read-only source audit on 2026-09-21. No build, browser run, or production edit was made.

## Exact Select wheel owner without expanding `HitTarget`

Do not add a public field to `HitTarget`. It is constructed throughout the renderer and is intentionally only the generic resolved target (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:19-26`). Do not derive the Select owner by parsing an option's `.item.` identifier.

The clean immediate-widget seam is the existing staged `WidgetInteractionMaps` authority.

* `render_select` first records the current Select action in the staging maps, then paints its popup (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:524-526`, `:708-711`).
* The Shell passes only `widget_maps_staging` to each immediate document walk (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4703-4711`).
* On the same accepted input witness, Shell swaps the maps, retained hit maps, geometry, and then `InputState`'s hit registry (`Shell:13028-13042`). A discarded candidate swaps none of them.
* `HitRegistry` has separately staged/resolved vectors and only exposes the completed one (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:217-257`, `:311-340`). Therefore a parallel scope stored in `WidgetInteractionMaps` is authoritative only if it refers to the corresponding resolved hit-vector indices.

Add one **private, hard-bounded** field to `WidgetInteractionMaps`, rather than a map:

```rust
select_popup_wheel_scope: Option<SelectPopupWheelScope>
```

`SelectPopupWheelScope` needs only the exact Select owner ID, painted menu rectangle, and half-open hit-index range `[first_hit, end_hit)`. It does not need an action clone, item list, or a public/wire field. The production opening transition closes all current Selects before setting one true (`Shell:14001-14005`; `close_open_selects` at `:14675-14684`), so an `Option` is the accurate bounded model. Assigning it while painting also follows visual order; an invalid multi-open state therefore cannot grow an unbounded registry.

`render_select_menu` already knows the menu rect and emits all popup rows and chevrons in a contiguous paint interval (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:496-545`). Capture the existing `InputState::staged_hits().len()` before and after those registrations, then register the scope through an `interaction_maps` helper. `staged_hits` already exists at `📥️input/🦀️.rs:337-340`; no `HitTarget` constructor changes are needed.

Add the narrow `InputState::hit_index_at(x, y) -> Option<usize>` helper beside `hit_at`. It returns the index of the same reverse-order resolved hit `hit_at` would select. On wheel, the current presented `widget_maps.select_popup_wheel_owner_at(x, y, top_hit_index)` accepts only when:

1. `menu.contains(x, y)`;
2. the top hit is in `[first_hit, end_hit)`, or it is an earlier underlying hit; and
3. no later hit is topmost.

This means option rows and chevrons retain their exact click targets, a blank menu gutter still has a popup wheel owner, and any later-painted context/menu control above the popup wins. It gives the requested exact owner/range/rectangle relation without changing generic hit data or performing a text-name parse.

The Shell wheel route can then resolve the scope **before** generic `ScrollRegion` routing, increment the existing `select.{id}.scroll` entry, and return true only when the popup route owns the delta. The next `render_select_menu` still has sole authority to clamp it using its actual item count and painted height. Its existing `clear_select_scroll` owns removal at every close (`Select:566-570`). Do not check live `open_selects` in the wheel handler: the scope belongs to the last GPU-accepted input snapshot, and a live close/open candidate must not change authority before its own ACK.

The staging cleanup must clear the `Option` in both `WidgetInteractionMaps::clear_frame` (`widgets:106-120`) and Shell's bounded manual staging drain alongside `select_metas` (`Shell:22809-22818`). It must be swapped at the existing ACK with `widget_maps`, never separately published. This preserves candidate cancellation and stale-scope retirement automatically.

### Exact fail-first immediate law

In the existing Shell input/wheel suite, paint a long actual immediate Select to the accepted registry, open it through the real trigger, and obtain a visible row from `input.hits()`. Wheel at that row, re-paint and ACK, then assert:

* the selected scope owner is the currently published Select and its first visible row advances;
* no outer `ScrollRegion` offset and no camera/action changes;
* a later-painted overlapping chrome row prevents the scope from consuming;
* after an unacknowledged candidate repositions or closes the popup, wheel still resolves only the old accepted scope; after ACK it resolves the new scope or none; and
* pressing a newly visible option invokes exactly its existing `select_metas` action once and clears the scope on the following ACK.

The retained-tree Select uses `EventRouter` rather than `WidgetInteractionMaps`; it still needs the separate topmost-`SelectPopup` route described in `📓️terra-browser21-scroll-select-overlay-audit.md`.

## Component-close handoff review

The current SolTree handoff is coherent in source.

`OsHost::advance_component_surface_close` refuses close admission while a presentation is pending, then asks the reusable frame handle to retire exactly one owner unit while a close request is pending (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694-715`). `FrameBuildHandle::retire_for_component_surface_close_step` cancels, closes one rejected/session unit, clears the submission generation only after there is no live session, and does not set permanent `closing` or take the installed completion waker (`🧵️frame-job/🦀️.rs:703-728`).

The cancelled build returns any sealed input authority before phase retirement: `ActiveFrameBuild::retire_cancelled_phase` calls the Build/Prepare candidate discard (`🧵️frame-job/🦀️.rs:230-239`), whose concrete frame transaction helper obtains the runtime lock, discards the exact Shell witness, and clears it (`🧊️renderer/🦀️.rs:14291-14327`). The presenter abort path has the same fence (`🧊️renderer/🦀️.rs:15243-15251`).

Native host scheduling takes one close step, invalidates `RESOURCE_READY` when it has work, drains independent input, and refuses new frame admission while the request is pending (`🪟️winit-app/🦀️.rs:245-263`). Once external close reaches terminal, the bridge is no longer pending, allowing a new frame so the UI retirement can acknowledge that terminal token; this is necessary, rather than an early-re-admission defect.

The existing source law `renderer_async_boundary` asserts these same barriers and reusable-handle conditions at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:2072-2091`. I found no additional source-proven close-handoff hole after this review.
