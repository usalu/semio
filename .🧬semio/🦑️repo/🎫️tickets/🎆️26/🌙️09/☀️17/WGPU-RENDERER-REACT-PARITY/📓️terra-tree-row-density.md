# Terra Tree Row Density Audit

## Scope and evidence

This is a read-only source audit of the canonical light/en checkpoint-15 Actions pane. It does not change the control geometry already agreed for the pane: the inline-control value column remains `50 × 3.2 = 160px`; a tree-inline `Select` remains `16px`; the other inline controls remain `22.4px`.

The filtered checkpoint evidence is decisive:

| Target | `clearSelection` | `selectAll` | Consequence |
| --- | --- | --- | --- |
| React | `y=126, h=24` | `y=150, h=24` | successive rows advance 24px; `engagementAbort` is at `y=990`, beyond the viewport |
| WGPU | `y=131.1181, h=12.042824` | `y=143.16092, h=12.042824` | successive rows advance 12.042824px; `engagementAbort` is at `y=564.66`, visible |

The source checkpoint is `🗑️generated/astra-runtime/window-physical-checkpoint-15b/steps.json`; the behavioural interpretation is already recorded in `📓️astra-window-physical-postconditions.md`. The React physical oracle subsequently completed three of three captures in `🗑️generated/astra-runtime/tree-style-checkpoint-15/tree-style.json`: its first eight tree rows are `height=minHeight=24px`, have zero padding, and the tree-section content is intrinsically `1344px` high with `scrollHeight=1344px`. There is no parent-height squeeze in React.

## Shared token contract that must govern both targets

The schema is [`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json:360`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json:360) and [`:372`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json:372). At compact spacing `3.2px`, the desired contract is:

| Concern | Token/input | Effective geometry |
| --- | --- | --- |
| Row pitch and section header | `treeRowUiSpacing = 7.5` | **24px**, exactly one band per visible row |
| Tree toggle slot | `treeToggleUiSpacing = 4.375` | 14px slot |
| Tree indent | `treeIndentPerLevelUiSpacing = 3.125` | 10px per level |
| Tree guide extension | `treeIndentLineExtraUiSpacing = 2.1875` | 7px |
| Inline gap | `propertyInlineGapUiSpacing = 2.5` | 8px |
| Default row icon / section chevron | CSS `size-tiny` / `size-small` | 9.6px / 16px, vertically centred in the 24px band |
| Leaf label | React `text-xs`, `leading-none` | 11.2px glyph size with its own tight label line box, vertically centred by the 24px flex row |
| Row shell | fixed height/min-height/max-height, no vertical padding or sibling gap | 24px regardless of label metrics |
| Scroll/clipping | content has intrinsic height; viewport clips and scrolls | never divide available height among rows |

The browser capture's inherited `font: 16px/24px Anta` is the surrounding section/shell style, not the leaf-label style. The React leaf label explicitly overrides it to `text-xs leading-none`, so the required label conclusion is centring inside a fixed 24px row, not changing its glyph size to 16px or its line box to 24px.

## React is already intrinsic, fixed-pitch flow

[`🌳️Tree/🟦️.tsx:212`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:212) derives `treeRowHeightPx` directly from the DOM token. The row shell at [`:215`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:215) sets `h-workbench min-h-workbench max-h-workbench`; the React styling mapping makes `size-workbench = 1.5 × size-small` at [`🎨️.css:731`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:731), hence 24px.

The implementation explicitly removes the alternate density inputs: vertical padding, branch gap, section/item top padding, sibling gap and subtree gap are all zero at [`🌳️Tree/🟦️.tsx:239`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:239). The leaf label is the centred `text-xs font-normal leading-none` slot at [`:248`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:248); section chevrons and item icons use the 16px and 9.6px slots at [`:250`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:250). The concrete `role=treeitem` leaf gets that shell at [`:2873`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:2873).

The Actions route is an ordinary React `Tree`: [`🛠️ShellHelpers/🟦️.tsx:4348`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:4348) renders `WindowActionPane`, whose body uses `<Tree ...>` at [`:4352`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:4352). The capture confirms its tree content grows to its intrinsic 1344px and scrolls rather than distributing an available parent height across children.

## WGPU source contract and exact divergence boundary

WGPU's intended retained-tree contract matches React:

* [`🧮️layout/🦀️.rs:143`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:143) constructs `TreeRowMetrics` from `Theme`; [`🎨️theme/🦀️.rs:264`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:264) derives `tree_row_height` from the same generated `TREE_ROW_UI_SPACING` token. Light/dark and en/de do not alter that scalar.
* The mounted classifier creates a `TreeRow` whose `height` is `tree_item_height(..., metrics)` at [`📌️mounted_layout/🦀️.rs:90`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:90) and [`:112`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:112).
* `flow_for` converts every `Tree`, `TreeSection`, and `TreeRow` to a fixed-length, clipped band at [`📐️flex/🦀️.rs:290`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:290) and [`:306`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:306). It is intentionally independent of an authored record's generic stack sizing.
* The live retained paint path again makes every item a `metrics.row_height` rectangle at [`🖌️paint/🦀️.rs:567`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:567) and increments the cursor by that same value at [`:692`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:692). The separate static `paint_tree_item` begins at [`:2759`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:2759); it is not the Actions pane production route.

The Actions pane does use the mounted retained document route: `build_window_actions_ui` authors the `UiNode::Tree`; `PanelProjection` maps the tree, sections, and items to record components at [`🐚️Shell/🦀️.rs:4904`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4904) and [`:4923`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4923); reconcile mounts the corresponding identity rows as `Stack`s at [`🔀️reconcile/🦀️.rs:856`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:856). Their keys are consistently surface-prefixed in both the record and reconstructed tree spec ([`🐚️Shell/🦀️.rs:4638`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4638), [`🔀️reconcile/🦀️.rs:837`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:837)).

The only compression boundary in this source is the fallback at [`📌️mounted_layout/🦀️.rs:517`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:517): an item that `tree_row_kind` cannot associate with the owning `Tree` becomes a normal authored `Stack`, whose `LayoutSpec` is auto-sized. That is the only legitimate route that can stop the fixed `TreeRow` band from reaching the solver. It must be treated as a failed identity/classification invariant, not as an alternate compact density. The 12.042824 value has no token or constant source in the current tree metric path, and conflicts with the current painter's 24px cursor. Therefore source inspection alone cannot prove whether the captured runtime was a stale artifact or which of the classifier's four `None` exits occurred; it does prove the capture is not an intentional theme, locale, icon, padding, or label-line-height setting.

The pointer ledger reports the accepted mounted layout verbatim (`register_retained_hit` at [`⚙️engine/🦀️.rs:334`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:334)), so the 12.042824px `TreeItem` entries are real published row geometry, not an accessibility-mirror transform. This is why it can cause the Abort visibility error even if a static painter seems correct.

## Parent fitting, intrinsic flow, and clipping

React's result is intrinsic content (`1344px`) in a scrollable section; its fixed rows do not shrink. WGPU's generic authored `LayoutSpec` also does not assign leftover child growth: the flex dialect distinction is explicit at [`📐️flex/🦀️.rs:7`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:7). Legacy `UiNode` stacks do grow children; authored records do not. A correctly classified `TreeRow` is stronger than either rule because it has a fixed height.

There is a second, independent shell discrepancy to repair after classification: the Actions body is currently given the entire remaining window height at [`🐚️Shell/🦀️.rs:18757`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:18757), whereas the Search body already reads its retained intrinsic content height at [`:18788`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:18788). The retained engine exposes that intrinsic height at [`⚙️engine/🦀️.rs:1954`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1954). The repair should make Actions use the same one-edge, bounded-height, clipped-and-scrollable contract as React, while retaining the 24px pitch. Merely shrinking the pane or changing control metrics would hide the row defect and break the agreed controls.

## Minimal coherent implementation and validation packet

1. Make `TreeRowMetrics` the sole schema-to-runtime interpretation of `treeRowUiSpacing`, toggle/indent/gap, icon slots and inline-control slots. Keep the current `160/16/22.4` control policy in [`🧮️layout/🦀️.rs:221`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:221).
2. Add an invariant in the mounted Actions document path: every published `Component::TreeItem` must resolve to `LayoutNodeKind::TreeRow`; reject or diagnose a missing owning-tree/key/parent association before it can silently fall through to generic `Stack`. Assert the mounted rectangles for `clearSelection` and `selectAll` have height 24 and a 24px delta, then assert the painter, hit registry and accessibility projection publish the same rectangles.
3. Use intrinsic Actions content height, capped by the React-equivalent available pane height, with a scroll offset and one clipping rect. `engagementAbort` must remain outside the initial viewport at the canonical checkpoint; it must become reachable by scrolling, never by compressing rows.
4. Extend the language-neutral fixture `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json` with an Actions-category vector. Its Rust law [`🧪️tests/🌳️tree-row-rects/🦀️.rs:125`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs:125) already verifies mounted rows, painter-compatible metrics and hit testing; add the real shell document route in [`🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:100`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:100), which currently verifies rows and identifiers but not their accepted geometry.
5. The TypeScript twin `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️tree-row-rects/🟦️.ts` currently re-derives the token arithmetic rather than mounting React. Keep it as schema parity, but use the existing Playwright dependency for a real-browser Actions story/journey measurement of `getBoundingClientRect`, computed label style, parent `scrollHeight/clientHeight`, and clipping. The existing React component suite ([`🌳️Tree/🧪️tests/🧩️component/🟦️.tsx:1`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx:1)) uses Testing Library and is suitable for structure/interaction but JSDOM is not a physical-layout oracle. The completed `tree-style.json` capture is the immediate real-browser oracle for this checkpoint.

## Confidence and remaining unknowns

Confidence is high that the intended shared row contract is 24px and that React obeys it without vertical squeeze; high that the captured WGPU 12.042824px hit rectangles violate the current retained-tree metric contract; and high that the correct repair must preserve fixed intrinsic rows then clip/scroll. Confidence is medium that `tree_row_kind` fallback is the live compression mechanism: it is the exact source boundary that permits a non-`TreeRow` layout, but this read-only audit cannot inspect the runtime arena/classifier result or prove whether the captured artifact predates the present source. The first new mounted Actions geometry test should expose that remaining binary question directly.
