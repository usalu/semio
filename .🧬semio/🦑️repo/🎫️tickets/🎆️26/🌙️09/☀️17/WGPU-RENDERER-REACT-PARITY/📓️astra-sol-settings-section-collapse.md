# Sol Settings Section Collapse and Panel Ownership

## Scope

This packet implements P2 from `📓️astra-terra-settings-visual-audit.md` and the concrete paired
checkpoint 11 repair:

- retained generic `Section` and `Tree` section disclosure governs layout, paint, pointer hits,
  keyboard focus, and accessibility;
- an upward bottom panel reverses Tree section and row geometry;
- navbar/footer chrome retains the root panel toggle while the open panel owns only nested path
  rows;
- the footer root toggle and open panel row have disjoint bounds, without a Settings-specific
  pointer exception.

Select overlay origin and AX overlay visibility remain separate P3/P4 packets.

## Runtime evidence and failure mechanism

Checkpoint 11 measured the WGPU Settings `PanelTab` and footer `Toggle` on the same y band:

- panel root `framework.settings`: `[1300, 974.39996, 67.69668, 22.4]`;
- footer `framework.settings`: `[1285.0319, 974.4, 67.696655, 22.4]`.

The browser probe clicked the footer toggle's centre, but retained panel authority won the overlap.
Only child tab hits disappeared; General remained visible. React keeps the Settings root in the
footer row at y975 and places the selected General row above it at y952.

The same checkpoint showed every Driver child despite its authored `defaultOpen: false`, and put
the Tree in top-down/full-height order. The retained reconciler mounted every descendant, while
layout, paint, hit testing, focus collection, and accessibility continued to walk those mounted
children unconditionally. `defaultOpen` was therefore presentation data rather than interaction
authority.

## Product repair

`UiTree` now owns live disclosure state per generic labelled `Section` and synthesized Tree section
row. Pointer, keyboard, and accessibility activation toggle that state through the same event
router. A toggle marks layout and paint dirty and stamps a one-shot `NodeFlags` signal on the root.
The engine consumes that signal immediately, increments the surface layout generation, invalidates
the previous intrinsic height, and requeues layout. The signal uses a spare bit in the existing
fixed-width flag word; it does not widen `EventRouter`, `UiWindow`, `UiTree`, or the fixed surface
registry.

That explicit signal matters when a user opens and recloses a section before an in-flight layout
settles. Every toggle advances the generation. A late job cannot publish geometry for the
intermediate open state over the newer closed state.

Closed descendants are absent from:

- mounted-layout admission and intrinsic height;
- the retained paint walk;
- retained hit registration and recursive hit testing;
- Tab focus collection;
- accessibility projection.

The arena still retains their keyed identity, so reopening does not reconstruct unrelated state.

Tree layout receives `UiFlow.block.is_reversed()` at bounded job admission without retaining a new
per-surface field. Up flow arranges authored sections and rows from the block end and moves a
section's trigger padding to that end. The streaming Tree painter traverses the same reversed flat
section/item order used by General Settings.

Chrome-hosted top and bottom panel bounds now reserve the root control row already owned by the
navbar/footer. The floating panel's tab rows omit that root and begin at the first selected branch
row. Side-middle anchors, which have no bonded shell row, keep their root inside the panel. This
makes the General row end at the footer root row's start and removes the competing `PanelTab` hit at
the Settings toggle.

## Schema and laws

The neutral disclosure schema and fixture define closed and open expectations for layout, paint,
pointer reachability, and accessibility.

The independent React oracle uses the production `Collapsible`, `CollapsibleTrigger`, and
`CollapsibleContent`. It validates the fixture through Ajv, checks `aria-expanded`, checks that the
closed child is absent from ordinary role queries, then activates the real trigger and confirms the
child becomes visible and reachable.

Rust laws cover:

- the real retained layout and frame pipeline from a closed Section through rapid
  open→close-before-settle and a final reopen;
- intrinsic height, paint census, and hit-registry membership in both states;
- a published two-record accessibility document, including `expanded` and child reading-order
  reachability;
- upward Tree section and flat-row geometry through the production mounted layout job;
- Shell root-row ownership and the bottom panel/footer non-overlap boundary.

## Validation

- Focused Bun+Nx React/Ajv oracle with `SEMIO_TEST_LEVEL=standard`: **1 file passed, 2 tests passed**.
- Both neutral JSON files parse successfully.
- All touched Rust sources and new laws parse with `rustfmt --edition 2021 --emit stdout`.
- Scoped `git diff --check`: clean.

The first focused Vitest invocation omitted `SEMIO_TEST_LEVEL=standard`; the existing config then
selected only its fundamental quick suite and reported no matching file. The corrected invocation
above is the relevant result.

Root owns UI/native/wasm/browser gates. At this source checkpoint, UI18, Native20, and a fresh
paired browser run had not completed, so this report does not claim runtime parity. UI18 reached
the new law and stopped at one test-only missing `Label` import before executing tests; the explicit
crate import is repaired for UI19. No production diagnostic was reported by that compile.

## Boundaries

The General Settings Tree contains flat item rows, and this packet validates the complete upward
geometry for those rows and their section triggers. Nested Tree item disclosure remains governed by
its existing authored `defaultOpen` contract; adding user-controlled nested item disclosure was not
part of this Settings packet.

P3 must still make Select popup placement originate from the solved upward control rect. P4 must
validate the final browser accessibility mirror and overlay visibility after the panel geometry is
active.

## Checkpoint 12 Up-paint regression

The sealed checkpoint 12 browser run reached the new upward General Tree and panicked in
`paint::retained_tree_index` before it could validate Settings behavior. The function used
`(index < len).then_some(if reversed { len - index - 1 } else { index })`. Rust evaluates the
`then_some` argument eagerly, so the normal terminal cursor (`index == len`) executed the reversed
subtraction and underflowed. Every later poisoned-mutex console entry was a cascade from that first
panic; checkpoint 12 is therefore runtime red for P2 and cannot validate the later root-Toggle
repair.

The index mapper now returns `None` before doing reverse arithmetic whenever `index >= len`. A new
law drives the actual bounded `frame_into_step` pipeline with `UiFlow::for_anchor(Bottom)`, an empty
section, an open item whose nested list is empty, and a populated sibling subtree. It requires the
frame to reach `Ready`, retire its paint cursor, and publish the empty section, empty-nested item,
and populated child hits. This covers both the terminal section cursor and the empty nested-list
ascent that layout-only laws did not execute. Root owns the pending UI/native/browser receipt for
this source-coherent repair.

The generic Shell close repair is in the `panel_tab_anchor` `HitKind::Toggle` branch. The root
`framework.settings` control previously called nested-tab selection, which could remove child tab
hits while leaving the General body active. It now calls `toggle_anchor_tab`; its native
postcondition requires the anchor closed, General no longer selected, and no dock or input drag
armed. Because checkpoint 12 predates this semantic change, a close failure in that sealed bundle
does not assess the repair.

UI24 subsequently passed the bounded Up `frame_into_step` law, including terminal traversal, empty
section, empty nested list, and populated sibling hits. Checkpoint 13 then passed the browser's
General open, Drivers closed→open→closed, and physical Settings footer-close postconditions without
the checkpoint 12 overflow. Native21 also passed the real Shell footer-close law. These receipts
cover the P2 runtime path; P3 Select popup origin remains separately tracked in
`📓️astra-sol-select-origin.md`.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/📂️retained-section-collapse/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📂️retained-section-collapse/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📂️retained-section-collapse/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📂️retained-section-collapse/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📂️retained-section-collapse/♿️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs`
