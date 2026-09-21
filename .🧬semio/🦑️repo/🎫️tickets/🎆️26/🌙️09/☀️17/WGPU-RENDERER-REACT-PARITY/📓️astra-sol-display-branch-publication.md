# Display branch publication and fold ownership

## Runtime evidence

Checkpoint 16 Dock D mounted the Display Windows body in the bottom-left Up-flow panel. The screenshot paints only the bottom panel band, while the published registry reports section and child rows spanning `y=852.8..948.8`. It contains a label and transfer handle for `framework.display.windows.puzzle3d-main.projection.parallel`, but no dedicated `tree.chevron` target.

Evidence:

- `🗑️generated/astra-runtime/checkpoint-16-dock-d/wgpu/template-configuration-failed-state.json`
- `🗑️generated/astra-runtime/checkpoint-16-dock-d/wgpu/template-configuration-failed.png`

The missing chevron and the stale/off-viewport rows are separate faults. Adding a synthetic chevron for a row that was not painted would hide the publication problem.

## Source proof

React's `worldProjectionTemplatesToTreeItems` adds `items` only when a projection template has children and gives every branch `defaultOpen: false`. React `SortableTreeItem` then renders a real button containing `GroupFoldChevron` when `hasChildren`.

WGPU already paints a chevron for a `UiTreeItemNode` with nonempty `items`, but `register_retained_hit` publishes only `retained_hit_registration` and an optional drag handle. The item registration always uses `tree.label.<id>`; no retained branch-chevron registration exists. The event/tree state helpers also recognize section disclosure directly, while nested item expansion still depends on authored `default_open` in mounted layout and retained paint.

Hosted `frame_into_step` has a second source gap: paint is clipped by the caller-owned host viewport, but the hit walk only intersects each row with `retained_node_clip`. It does not intersect the host viewport before publishing the retained hit registry.

## Neutral contract and independent oracle

The shared `window-lifecycle-template-drag` fixture now owns `displayBranchPublication`:

- closed publishes only the painted section header;
- opened publishes only rows intersecting the host viewport;
- a visible childful row publishes both its label and a dedicated chevron;
- a stale accepted-layout generation publishes nothing.

Its JSON Schema requires every field. The independent Bun oracle computes viewport intersection and generation admission from the fixture. It also checks the actual React producer creates childful branches and the actual React Tree renders their fold button.

Focused command:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun test './🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪟️window-lifecycle-template-drag/🟦️.ts'
```

Receipt: 10 passed, 0 failed, 49 assertions.

## Native fail-first law

`an_expandable_display_template_publishes_a_real_gutter_toggle_and_retires_its_children_when_closed` lives in the renderer Display test module. It mounts the production Display World3d body with Up flow, requires exact `tree.chevron.framework.display.windows.main.projection.parallel`, uses the gutter for real pointer down/up, then requires Orthographic publication after open and retirement after close.

Native55 did not reach the chevron assertion. Its helper laid the document out before establishing the 300×240 viewport; the first frame then changed viewport generation and left the test's paint-only loop pending. The helper now establishes the viewport before its bounded layout loop. This is a fixture repair only.

Native57 reached the intended red. Its visible registry contained the section gutter, `kind`, Parallel and Perspective labels, and both transfer handles, but no `tree.chevron.framework.display.windows.main.projection.parallel`.

The bounded repair gives nested TreeItems the same live disclosure authority sections already had:

- mounted layout derives row/subtree height and child admission from the retained item's live `disclosure_open` state;
- retained paint uses the live state, paints an expanded Up-flow branch after its children, and traverses only admitted children;
- retained hit publication emits a dedicated gutter after the label/drag entries, so reverse hit resolution gives the gutter precedence;
- pointer release toggles a TreeItem only inside its exact depth-aware gutter; label activation and drag remain independent;
- the interactive-state synchronizer retains all authored nested rows so a closed branch can open without minting an unrelated generation;
- Up-flow label and gutter registrations share the painted bottom-row band.

`a_childful_up_flow_tree_item_registers_label_and_gutter_on_its_painted_bottom_row` covers the direct retained registration geometry. The real Display law still owns open, child publication, close, and child retirement. This repair is source-coherent for UI60 and Native58; no green receipt is claimed before root's run.

## Nested close invalidation

Full Native59 reached the complete physical interaction and reduced the Display failure to the last retirement assertion: the Parallel gutter opened, Orthographic published, and the same gutter closed, but the old child hit remained. Full UI61 was otherwise green at 649 passed and 0 skipped.

The language-agnostic retained-section law now reproduces the Display shape with a Tree nested below a panel Stack. It requires the first real gutter click to open, the child hit to end exactly at the stable header row, the second real gutter click to close immediately, the exact same header rectangle after both layout changes, no closed child hit, and a zero-height accepted child layout when closed.

UI62 established the focused red: after the close, the accepted child height remained 24 instead of returning to 0. UI63 added the immediate disclosure and non-overlap checks; both passed before the same stale-height failure. This separates pointer routing from layout invalidation: the physical gutter changes retained disclosure state correctly, but no closing layout generation is scheduled.

The accepted layout commit clears layout and subtree dirtiness on the root. Intermediate Section and TreeItem flags remain marked. The following close marks its branch, encounters the already-marked Section, and the previous bubbling rule stops there under the false assumption that every ancestor above is still marked. The root is already clean, so `layout_is_dirty` answers false and leaves both accepted child geometry and the hit generation stale.

The repair makes the root the explicit scheduling authority: an already-marked intermediate ancestor can stop bubbling only while the root still owns a layout obligation. A successful layout commit also marks the root paint-dirty so the accepted geometry is followed by a new paint and hit publication. Finally, the window's Up-flow bit remains attached to every admitted descendant, including a Tree nested below the Display panel Stack; this keeps child rows above the exact same header row.

Focused UI64 passed the strengthened Up-flow law (1 selected passed, 649 outside the filter) in 23.4 seconds. The paired Down-flow law owns the same physical open/re-close, stable header, zero-height collapsed child, and hit-retirement invariants, with its open child beginning exactly at the header's bottom edge. Full UI65 passed 651 of 651 tests with 0 skipped in 39.9 seconds, including both nested-close directions. The next native Display run owns the remaining renderer receipt.

## Checkpoint 17 panel resize interception

Checkpoint 17 rendered the Display parallel projection row, including its real nested-tree gutter, but WGPU published `panel.resize.bottom-left.outer` across the gutter. The evidence separated the completed tree repair from panel chrome geometry: the row was `[6.4, 876.8, 293.6, 24]`, its gutter was `[6.4, 876.8, 14, 24]`, and the WGPU resize hit was `[3.2, 849.6, 20, 124.8]`. React mounts left-column panel resize only on the right edge; WGPU mounted it on the left and widened it to 20 px.

The shared `panel-resize` fixture and draft-2020-12 schema declare all eight anchors, their physical edges, native suffixes, drag factors, and one compact-spacing width. The real React `Panel` oracle validates that fixture with Ajv, mounts every anchor, observes each real handle class, and applies a 10 px pointer delta. Native65 reached the intended resize RED in the production `render_panel_step`: `top-left/right uses one compact spacing unit` failed at the native law's line 217. This was part of a four-test focused run with 1 pass and 3 failures; the other selected assertions belong to separate root-owned window/reservation work.

The repair registers left-column anchors at their right/outer edge, right-column anchors at their left/inner edge, and middle-column anchors at both edges. Every rail is exactly `theme.panel_inset`; panel content receives no compensating padding. The focused actual React oracle passed through Nx with 1 selected pass, 565 skipped, in 6.82 seconds Vitest / 8.8 seconds Nx. The command was:

```sh
bun nx run @semio-tech/ui-react:test -- --run '../../🟦️.tsx' -t 'Panel realizes the schema-owned resize edge, width token, and drag factor for every anchor'
```

Receipt: `🗑️generated/astra-runtime/panel-resize-react-3.log`. A prior attempt addressed the extracted registration module directly; Vitest correctly collected no file because this suite is registered by the React target's in-source test entry. It is not a behavior receipt.
