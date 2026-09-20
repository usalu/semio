# Terra Tree Final Audit

## Scope

Read-only final source audit of the Tree density and scroll packet. No test, build, browser activation, production edit, or Git operation was run. The checked surface is the retained WGPU Actions/Search document path and its event, paint, clipping, and Select-popup boundaries.

## Confirmed packet work

- `Tree`, `TreeSection`, and `TreeRow` use a fixed-height band with `shrink: 0.0`, so the generic negative-free-space branch cannot compress those bands (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:290-309, 593-627`).
- Both Actions and Search roots are projected as vertical, fill-sized `LayoutSpec::Scroll` records (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4613-4624, 18769-18789`). Reconciliation gives such records both `SCROLLABLE` and `CLIPS_CHILDREN` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:244-251, 1017-1022`).
- The retained paint walk subtracts the owning scroll offset before visiting a child (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:192-200`), and hit publication intersects every ordinary target with its accumulated ancestor clips (`:224-258, 385-414`). This preserves the requested clipped ordinary content.
- The Tree inline-control contract remains sourced through `TreeRowMetrics`: the shared value-column width and each control height are retained (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:143-153, 221-243`), and Tree-row controls remain absolute in that geometry (`📐️flex/🦀️.rs:281-288`). I found no source change that regresses the established 160 px / 16 px / 22.4 px policy.

## Actionable finding

### P0 — Event hit testing still ignores scroll offsets, so scrolled visible rows dispatch as unscrolled rows

`EventRouter::hit_test_node` accumulates only each parent's layout position when it descends (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:149-171`); unlike the paint and host-hit walk, it never subtracts `parent.state.scroll_offset`. `UiTree::absolute_rect` has the same omission (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs:507-524`).

The shell correctly routes a hit-ledger `TreeItem` back into that router (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12224-12240, 12898-12905`). After a wheel changes the scroll root, the ledger and painter put (for example) the terminal Actions row at its visible screen position, but the router resolves that screen position against the pre-scroll child locations. A click can therefore capture and activate the row that formerly occupied that coordinate, rather than the visible row. The existing dense Actions law verifies publication, clipping, and reverse-wheel geometry, but does not click the revealed terminal row (`🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:113-154`).

The same missing transform breaks the packet's Select-popup escape. The paint/hit path deliberately clears ancestor clipping for an open Select and gives its popup rows overlay priority (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:224-257, 375-429`). In contrast, event hit testing rejects a clipping ancestor before it tests its overlay children (`⚡️events/🦀️.rs:155-171`). A Select popup that extends outside a scroll viewport can paint and publish an overlay hit, yet its option press is rejected by `EventRouter` at the viewport boundary. This contradicts the required popup escape and makes the host's overlay target non-actionable.

Use one scroll-aware origin helper for `hit_test_node`, `UiTree::absolute_rect`, and the existing paint/hit walk: subtract a parent's live offset while descending ordinary children, reset both origin and clipping only at a real overlay boundary, and continue testing overlay children even when a clipped ordinary parent does not contain the point. Add a shell-level interaction law that scrolls to the terminal Actions row and activates it, plus a Select whose popup extends below a scroll root and commits an option outside that root.

Confidence: high. The source contains both coordinate rules and routes TreeItem/Button gestures through the incompatible event rule; the present law stops at geometry publication.

## Validation status

Static inspection only, as requested. The React-only Tree and palette receipts mentioned in the ticket were not rerun or relied on as a WGPU acceptance result.
