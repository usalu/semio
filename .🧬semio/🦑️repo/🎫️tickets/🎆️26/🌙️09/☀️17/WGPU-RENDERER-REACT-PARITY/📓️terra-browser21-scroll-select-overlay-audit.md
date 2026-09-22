# Browser 21 Scroll and Select Overlay Audit

Read-only source audit on 2026-09-21. No build, browser run, or source change was made.

## Decision

The Browser 21 checklist's former Actions/Search terminal-row and escaped retained-Select *click* risks are covered by the current retained input path. The concrete remaining Select risk is ordinary **wheel** input: a long open popup cannot consume it in either the retained or immediate WGPU path. React gives the popup viewport ordinary `overflow-y-auto` scrolling.

This is a production-path finding. It is not an inference from an old screenshot.

## Already covered paths

`UiTree::child_walk_origin` subtracts a `SCROLLABLE` parent's live offset at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs:598-604`, and `absolute_rect` uses the same walk at `:611-629`. The reverse hit walk invokes that origin for every child at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:145-192`.

The existing neutral law `scrolled_select_popup_commits_a_row_outside_its_scroll_viewport` at `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs:134-151` opens a Select in a clipped, scrolled parent and commits the escaped option. `hit_test_and_absolute_rect_follow_nested_scroll_offsets` immediately precedes it.

The Shell law `the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:99-159` performs the actual retained wheel route, republishes, clicks the revealed terminal row, and asserts one exact action. These are native fixture laws, not a fresh browser acceptance result, so Browser 21 should still exercise them physically.

## Confirmed retained-Select wheel omission

Shell sends a wheel over a retained body to `interpreter::dispatch_ui_event(... UiEvent::Scroll ...)` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13413-13423`.

`EventRouter::dispatch_pointer` then invokes only generic `route_scroll` at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:2256-2263`. `route_scroll` does a normal clipped-tree hit test, finds `nearest_scrollable_ancestor`, and mutates that node's `WidgetState::scroll_offset` at `:1675-1689`. It neither resolves the topmost open `SelectPopup` nor writes the Select popup offset.

This makes two reachable failures.

1. A popup portaled outside a clipped ancestor cannot be found by generic `hit_test`: `hit_test_node` returns before descending when the point is outside a `CLIPS_CHILDREN` parent (`events:145-160`). Wheel there is ignored.
2. A popup still inside the ancestor's viewport resolves through generic hit testing, but the nearest scroll owner is the outer document. Wheel moves Actions/Search rather than the Select list.

The retained renderer already has the required bounded popup data. The interactive sync reads `Select.state.scroll_offset.1` as the popup pixel offset and `.0` as a chevron direction at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1764-1769`; `select_clamped_scroll` is the canonical clamp at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:147-153`.

## Confirmed immediate-Select wheel omission

Immediate `render_select_menu` materializes only option and chevron `DropdownItem` hit rows at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:496-545`; it publishes no scroll-viewport target. It owns two bounded map entries: `select.{id}.scroll` and a one-step pending direction (`:155-203`, `:551-570`).

At Shell ingress, after retained-body routing, `handle_pointer_wheel` accepts only `HitKind::ScrollRegion` (`Shell:13413-13443`). An immediate popup option or chevron is `DropdownItem`, so normal wheel returns `false` and changes neither entry. Click geometry remains correct: `handle_shell_hit` arms a chevron through `arm_select_scroll` (`Shell:13648-13654`) and resolves a selected option by an exact `widget_maps.select_metas` entry (`Shell:14001-14014`).

An open Select is a presented modal (`Shell:13323-13329`); `scene_pointer_target_at` refuses it at `Shell:13355-13361`. Thus an unhandled Select wheel is inert at this ingress, rather than reaching World3d. It is still visibly wrong because React's viewport scrolls.

React's authoritative popup is `<div data-slot="select-viewport" className="... overflow-y-auto ...">` at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:673-679`. Its existing component law only covers a chevron's `scrollBy({ top: 80, behavior: "auto" })` at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx:389-407`; it does not establish ordinary WheelEvent behavior.

## Smallest coherent repair

Give an open Select popup first refusal of wheel input, ahead of generic document-scroll routing. Its middle-range delta must alter the popup's own offset, mark only its popup dirty, keep the list open, and not emit an action, alter focus/capture, or scroll its ancestor.

For retained UI, add a bounded `route_open_select_wheel` before `route_scroll`. It must resolve the topmost open Select's popup/menu geometry rather than relying on clipped ordinary hit testing, apply the delta through `select_clamped_scroll`, write `.scroll_offset.1`, and preserve `.scroll_offset.0` for a separately armed chevron. It consumes only when its offset changes.

For the immediate Shell path, route the selected popup's exact owner before the generic `ScrollRegion` branch. Reuse the existing per-open-select two-key lifetime and clear it with `clear_select_scroll`. The clean common form is an explicit internal select wheel owner on the popup's rows/viewport, with a bounded pending pixel-delta consumed and clamped by `render_select_menu`, where the item count and resolved popup height are authoritative. This avoids deriving an owner from arbitrary `.item.` text at wheel time. If the existing identifier route is retained temporarily, it must first prove membership in `widget_maps.select_metas`, exactly as option commitment already does.

Whether a delta at the top or bottom should chain to an outer scroll container is not established by the current React law. Do not freeze that boundary policy from source inspection. The two fail-first laws below should cover a delta that changes the popup offset; a mounted React WheelEvent oracle must determine the clamp-boundary behavior before choosing consume versus bubble.

## Fail-first laws

1. **Retained popup wheel takes precedence over a clipped scroll parent.** Extend `targets-wgpu-events-unit` with a long Select inside `SCROLLABLE | CLIPS_CHILDREN`, opened and painted so a visible popup row lies beyond its parent clip. Dispatch a real `UiEvent::Scroll` on that row. Assert the Select's popup offset/first row changes, the outer offset does not, no `UiCommand::App` is emitted, and the popup remains open. Paint/publish, press the newly visible row, then assert one exact selection action and closure. This extends the existing `scrolled_select_popup_commits_a_row_outside_its_scroll_viewport` law rather than duplicating it.
2. **Immediate popup wheel owns a visible option row.** In a Shell input fixture with a long Select, render actual popup hits, wheel over a visible `DropdownItem`, and re-render. Assert `select.{id}.scroll` and the first visible row advance; selection still dispatches the exact value once; camera/underlying action and outer scrolling remain unchanged. A React component law should dispatch WheelEvent against `data-slot=select-viewport` and observe its scroll position, complementing the existing chevron-only oracle.

These laws cover the required Browser 21 physical probes: scrolling to a visible list option followed by activation, and a Select extending beyond its containing scroll root. They do not claim browser acceptance.

## Priority

Implement Select-popup wheel precedence before the generic scroll ancestor route. The Actions/Search terminal-row path needs only the scheduled physical browser confirmation; no current source defect was found there.
