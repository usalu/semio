# Window Measures Retained Height and Tree Window Extent Audit

Read-only source audit on 2026-09-21. No build, native test, or browser run was performed.

## Window Measures result

I found no source-supported initial-paint, accepted-height reuse, or folded-rail hit-retirement defect in the current path.

`ShellState::paint_window_measures_step` returns early for a missing document or a folded rail before it registers body hits. On every new chrome walk, `FrameSetup` clears `retained_hit_windows_staging` and `retained_scene_hits_staging`, and the interpreter clears its accessibility staging list. The later accepted candidate can therefore contain no previous Measures target. The prior accepted registry stays live only until that replacement is acknowledged, which is the intended presented-input rule.

The apparent `surface_id`-only height lookup is also not an initial-paint feedback bug. `window_measures_rect` initially supplies the available body height only until layout exists. The first opportunity that pushes the panel solid is guarded by `cursor.document.phase_name() == "paint"`; the document has already traversed ingress, reconcile, viewport, and layout. The outer stepped caller recomputes the rect on each opportunity. At that paint opportunity, `surface_content_height` is the newly solved intrinsic height, so the solid, clipped document, border, and registered body hits all use the current accepted layout.

A same-surface replacement may use the prior intrinsic height as an input viewport during its new ingress/layout work, because `surface_content_height` is keyed by surface id. That is safe for this owner: the engine computes intrinsic content height bottom-up and independently of the supplied viewport. Before paint/hit registration, the next opportunity re-reads the current intrinsic height. It is not evidence that a previously short Measures document permanently clamps a longer successor.

The current unit law does not directly prove the timing boundary: it paints to completion and only then reads `window_measures_rect`. Add one native fail-first law that starts with a short accepted Measures tree, replaces it under the same Measures surface with a longer tree, and drives exactly one real stepped document paint to completion. It should assert in that same candidate:
- the one panel solid has the new `retained_content_height`;
- the new final control is registered and inside the clipped rect;
- the old final control is absent; and
- a folded rail publishes no Measures accessibility item or hit after the next acknowledged candidate, while its retained lease remains available for re-opening.

That exercises `paint_window_measures_step`, `seal_presented_input_candidate`, and ACK, rather than a direct geometry helper.

Tree group folding has its separate established rule. The projection emits a real `TreeSection` / nested `TreeItem`; collapsing changes retained tree disclosure and layout rather than retiring the whole Measures lease. The existing retained section-collapse law demonstrates that an accepted collapsed frame omits descendants from hits and content height, including an open/close race while layout is checked out. For Measures controls this is the correct scope. I found no evidence that folding a group needs synchronous lease retirement.

## Exact source anchors

| Concern | Evidence |
|---|---|
| Measures records are a finite compact tree with no virtual window | [Shell WGPU source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4372), particularly the explicit `window: None` section at 4389. |
| Existing lease retirement and fresh publication | [Shell WGPU source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4593). It removes exactly the old lease, retires it, then inserts a newly minted one. |
| Dynamic rect and stepping order | [Shell WGPU source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4690) and [the rect resolver](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4739). It recomputes `rect` every opportunity; the first background write is gated to `Paint`; hit registration follows completion. |
| New candidate starts clean | [Shell WGPU source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22747). Both shell staging maps are cleared before the walk. |
| Height semantics | [UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2751). `surface_content_height` returns the root's intrinsic extent when available, otherwise the accepted root layout. Its existing engine test requires viewport-independent intrinsic height. |
| Folded child behavior | [retained section-collapse test](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/📂️retained-section-collapse/🦀️.rs:193). It already asserts the hidden child has no hit and the later reopened tree increases intrinsic height. |
| Current Measures law limitation | [Measures test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs:33). It validates final compact geometry and group-hidden child input, but does not test a same-surface size-changing replacement. |

## Uniform virtual-window extent

The proposed required authored `TreeWindowRowExtent` is sound only if “uniform” means the **closed base reservation** of every logical row, including each materialized row and each absent spacer row.

It contradicts current generic `TreeWindow` semantics if it instead means the complete rendered height of every materialized row. A materialized item may legally be open and own nested descendants:
- the production panel builder explicitly supports a windowed item inside a windowed section, and its test asserts nested `TreeWindow { total: 25, offset: 0 }`;
- WGPU `live_tree_item_height` computes the base row and recursively adds every open child;
- React’s virtual tree explicitly takes actual DOM tops for materialized rows while using a fixed pitch only for absent spacer bands.

The clean contract is therefore:
- add a **required typed enum** `TreeWindowRowExtent` to `TreeWindow`;
- define it as the closed base-row extent; both leading/trailing spacers use it, and every materialized item reserves that same base extent;
- let an open materialized item append its rendered descendants after that reservation;
- keep finite `TreePresentation::Compact` trees variable, as Window Options already does because it has `window: None`.

If a future caller truly needs constant *total* visual rows, it needs a separate explicit window mode that rejects expandable/nested rows and controls larger than the chosen extent. It cannot replace generic `TreeWindow` without breaking its current documented nesting contract.

Use semantic enum cases, not raw pixels or `SpaceToken`: `TreeWindow` is `Copy + Eq`, and the existing spacing token multipliers cannot express compact row dimensions. Map the enum to current theme metrics in each renderer. Make the field required across the contract and producers; do not add a default that hides an unclassified virtual list.

The smallest neutral test is a three-row window with one materialized, collapsed middle row and nonzero leading/trailing counts. It must assert:
- `leading + materialized base + trailing = total × declared extent`;
- materializing a different offset preserves the same total scroll extent; and
- opening a nested materialized row adds only its descendant content after its own base reservation, without changing either spacer calculation.

A second admission law should refuse a hypothetical total-uniform-only window when any materialized row is expandable. This prevents an implicit renderer-only rule.

| Contract anchor | Evidence |
|---|---|
| Current shape is only `total, offset` | [UI contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:465) |
| Nested windowed producers are supported | [Plugin builder](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6042) and [nested test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs:559) |
| WGPU open descendant accumulation | [mounted layout](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:151) |
| Existing WGPU spacer-only pitch | [legacy UI tree window](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2390) |

## Correction — Native144 contradicts the earlier height conclusion

Native144 failed the Window Measures accepted-content-height assertion. The earlier source-only conclusion that found no initial-paint or retained-height defect is therefore retracted **for the height path**. Recomputing the outer rectangle after a document paint does not establish that the UI engine has produced an intrinsic content height rather than the viewport-height fallback. The separate findings about candidate hit staging and folded-rail omission remain source-supported. This report is being amended after tracing the intrinsic-height producer and its first accepted layout boundary.
## Native144 Height Failure — Narrowed Causal Boundary

The Native144 receipt at 🗑️generated/astra-runtime/renderer-native144-compact-ax-locale-red/run.log fails window_options_hug_their_tree_below_the_chip_and_retire_folded_children at Shell test line 51. Its source boundary is now concrete:

- The failing production document is a Component::Tree root with TreePresentation::Compact, built in Shell/🎯️targets/🧊️wgpu/🦀️.rs:4372–4396.
- The generic UI law that otherwise establishes content hugging drives only a legacy Stack root in 🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs:2489–2503. It does not exercise this Tree branch.
- A mounted Tree's claimed intrinsic extent comes from retained_tree_height in 📌️mounted_layout/🦀️.rs:177–186, is emitted as LayoutNodeKind::Tree at line 594, and is written to UiWindow.intrinsic_content_height only at layout publication completion in ⚙️engine/🦀️.rs:1900–1910.
- The outer Measures rectangle reads that accepted extent at Shell/🎯️targets/🧊️wgpu/🦀️.rs:4739–4744. Before any accepted layout it intentionally uses the available viewport height.

This establishes a Tree-specific missing regression. It does **not** yet establish whether the Native144 observation is an absent/NaN intrinsic witness or an overlarge Tree calculation: the current API exposes neither result to the law.

### Smallest Fail-First Split

1. In the existing UI engine unit module, build the same wire/reconcile Tree shape: Compact root; Grid section open with Toggle, Select, Slider; Sun section closed. Drive the real retained layout at 800 × 720, assert the exposed content height is positive and below 270 px; drive at 800 × 1,200 and assert the same height. Its expected base is the authored row sum (14.4 + 22.4 + 16 + 22.4 + 14.4 = 89.6 logical px), subject only to the shared row-metric tolerance.
2. Keep the Shell law as the host integration assertion: after a completed document layout, the Measures rect must leave its 450 px minimum unused body height and its accepted frame must contain only the visible Grid controls.
3. If (1) is green while (2) is red, the proven repair is a **single bounded host reflow**: paint_window_measures_step computes a fallback rect before layout at line 4699; the interpreter accepts the intrinsic later in UiDocumentFramePhase::Layout at Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2769–2775 and currently enters Paint without re-reading host bounds. Reuse UiDocumentFrameCursor::restart_viewport_after_host_reflow (lines 2575–2581) once for that document revision, recompute the rect, and lay out/paint against it. Never ACK or register candidate hits from the viewport-sized predecessor.
4. If (1) is red, repair the Tree intrinsic producer rather than Shell geometry. The Stack test is insufficient evidence for that path.

The ordinary folded-rail result remains separate: paint_window_measures_step returns before hit registration when folded (lines 4690–4695), while per-frame staging is cleared before the document walk. That behavior does not account for Native144 line 51.

## UI25 Follow-up — Tree Selection Is Correct; the Intrinsic Witness Is Not Yet Presented-Owned

Root reports that UI25 is green (669/669), including the Compact Tree 89.6 logical-pixel height before and after an ACK. This audit did not run that suite. The current direct path supports that result:

- `surface_content_height` selects `tree` while a candidate is ready, and `presented_tree` once an acknowledged presentation leaves `candidate_ready == false` and `presented_ready == true` ([UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2757)).
- `paint_window_measures_step` re-reads the host rectangle on each opportunity. If an accepted document's rectangle changes, it restarts only that cursor's viewport before background paint and retained-hit registration ([Shell WGPU](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4693)).
- ACK moves the tree and router together and removes the candidate-ready authority ([UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1203)).

There remains one distinct ownership gap. `UiWindow` has only one `intrinsic_content_height`. A candidate layout completion writes it ([UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1897)), but ACK neither moves nor snapshots that witness. Discarding a sealed candidate clears only the sealed witness ([UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1230)); publishing a successor starts a new candidate but also does not clear the scalar ([UI engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1626)). Consequently, after presented A, a ready/sealed B, B's discarded presentation, and a successor C published before C has an accepted layout, the accessor selects A's `presented_tree` but can read B's scalar witness.

The stale value is only observable if the selected presented root still has an accepted layout; `surface_content_height` otherwise returns `None` before reading it. That makes a user-visible wrong Measures rectangle an unproven runtime consequence, not a claim from this audit. The state authority mismatch itself is source-proven.

The clean bounded repair is two scalar witnesses: `candidate_intrinsic_content_height` written only by candidate layout publication, and `presented_intrinsic_content_height` moved at matching ACK. Candidate supersession clears the candidate witness; it never clears the accepted witness. `surface_content_height` must select the tree and its matching witness as one pair. A presented interaction that starts a new candidate must leave the presented height usable until the replacement ACK. This needs no tree clone, scan, or synchronous retirement.

Add a fail-first engine-plus-Shell law: acknowledge a short A; build and layout a taller B; seal then discard B; publish C before C completes layout; run the Measures rect opportunity. It must return A's accepted extent (or the documented no-layout fallback), never B's. After C completes and ACKs, it must return C's extent and only C's controls may enter the retained candidate registry. This covers the state transfer missing from the current before/after-ACK law.
