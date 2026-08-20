# 📓 terra-dispatch-tree-seam

Packet `dispatch-tree-seam`, wave W3. **Done.**

## OWNS — all three edited

- `🖼️render/📦️packages/🦀️rust/🦀️frame.rs`
- `…/🦀️dispatch.rs`
- `…/🦀️element.rs`

No file outside this list touched, created, or removed.

## What changed, in one sentence

`crate::DispatchTree: From<Vec<Hitbox>>` (a bounds-containment-stack heuristic over flat, listener-less,
overlay-blind geometry — `render-dispatch`'s own report flagged it as known-lossy) is **deleted**, and
replaced with a real construction path: `PrepaintCx` now owns a live `&mut DispatchTree` and two new
methods — `register` and `with_children` — so every `Element::prepaint` call appends its own node
(parent, flags, listeners) as it walks, instead of a later pass trying to reconstruct that information
from a flat rect list that never carried it.

## Decisions

- **`Hitbox`/`HitboxId` still earn their place — as spatial-index material, not tree structure.**
  `Hitbox` (`element.rs`) stays exactly what it was: `element`/`bounds`/`clips_children`/
  `hit_transparent`, nothing more. `DispatchTree` already stored a flat `Vec<Hitbox>` internally
  (indexed by `HitboxId`, used by `DispatchTree::bounds`); I added a `DispatchTree::hitboxes(&self) ->
  &[Hitbox]` accessor so that flat list is reachable for a future spatial index (quadtree/BVH) without
  it pretending to be the tree's parent/child source any more. `FrameSnapshot`'s own separate
  `hitboxes: Vec<Hitbox>` field is **removed** — it was a second, redundant copy of exactly what
  `DispatchTree` already owns; a caller now reads `snapshot.dispatch.hitboxes()`.

- **How prepaint registration threads the parent link, exactly.** `PrepaintCx` gained a private
  `parent: Option<FrameNodeId>` field (not `pub` — `register`/`with_children` are the only sanctioned
  way to read or advance it) plus a `new(shared, dispatch, text)` constructor:
  - `PrepaintCx::register(element, flags, listeners, hitbox) -> FrameNodeId` calls
    `self.dispatch.insert(self.parent, element, flags, listeners, hitbox)` — the *same* `insert`
    primitive `dispatch.rs`'s own hand-built-tree tests already use via their `leaf()` helper. No new
    tree-mutation entry point was added; registration and hand-built test construction now share one
    code path.
  - `PrepaintCx::with_children(node, body: impl FnOnce(&mut Self))` swaps `self.parent` to `node`, runs
    `body`, restores the previous parent. A container `Element` wraps its take→recurse→put_back calls
    into each child's own `prepaint` inside this — since prepaint already visits parent-before-children
    in order (the walk itself, not a separate registration pass), the parent id is simply *whatever
    node this call's own `register` just returned*, free to capture here and structurally impossible to
    recover from a flat post-hoc list (this is exactly why the deleted `From` impl could only ever
    guess via geometry).
  - `frame.rs::build_frame` constructs one `DispatchTree` per frame (`DispatchTree::new(UiRevision(0))`
    — see the revision note below), hands `&mut dispatch` into `PrepaintCx::new`, and moves the same
    `dispatch` value into `FrameSnapshot` once the whole prepaint walk (and paint) has finished. No
    `From` conversion step exists any more between "the tree the walk built" and "the tree the snapshot
    carries" — they are the same value.

- **`UiRevision(0)` is a placeholder, not a regression.** The deleted `From` impl also hard-coded
  `UiRevision(0)` (no revision existed to plumb through a `Vec<Hitbox>` either). `build_frame` takes a
  bare `root: E: Element`, not a `UiSnapshot`, so there is still no real per-frame revision source at
  this layer — that only exists once a reconciliation packet (`runtime-reconcile`) turns a `UiSnapshot`
  into an `Element` tree and threads its `UiRevision` through `FrameInputs`. I deliberately did **not**
  speculatively add a `revision: UiRevision` field to `FrameInputs` now, to keep this packet's diff
  scoped to the dispatch-tree seam itself rather than guessing at an API a different, not-yet-landed
  packet owns. Flagged below as a registrar-request-adjacent note for whoever lands
  `runtime-reconcile`/wires a real `UiSnapshot` into `build_frame`.

- **Test elements skip `FrameArena`/`AnyElement` erasure on purpose.** The four new `frame.rs` tests
  (below) need nested `Element`s to exercise `register`/`with_children` through `build_frame`, but their
  shapes are fixed at compile time (`Wrap<C: Element>`, a single-child wrapper; `Pair<A: Element, B:
  Element>`, a two-child one). Erasure (`element.rs`'s fn-pointer-vtable + `Box<dyn Any>` mechanism) only
  earns its cost for genuinely heterogeneous/dynamic children, which these tests don't need — using
  concrete generics instead is simpler, no less "real" a use of the `Element` trait/`PrepaintCx` API,
  and doesn't touch `element.rs`'s own arena tests, which already separately cover
  `FrameArena`/`AnyElement` take→put_back and phase-ordering.

- **Test geometry is authored, not predicted.** `Wrap`/`Pair`'s own container `LayoutSpec` uses
  `Sizing::Fill` (`taffy::style::Dimension::percent(1.0)`) uniformly, so `build_frame`'s one taffy read
  (`layout_cx.resolved(root_node)`) is a deterministic 100%-of-viewport rect rather than resting on this
  packet's own understanding of taffy's *implicit* root-sizing behaviour. Every level below the root
  computes its child's absolute bounds by simple offset arithmetic from the bounds its own parent handed
  it (`offset_bounds`), never by a second `LayoutCx::resolved` call on a non-root node — `PrepaintCx` was
  not extended with read access to the taffy tree, so no test depends on that working. Every assertion
  reads geometry back from the *built* tree (`crate::node_bounds`) rather than pre-computing expected
  numbers, so the suite's correctness does not depend on any taffy arithmetic assumption surviving
  contact with a real `cargo test` run — see "risk area" below for the one assumption that does remain.

## TESTS — the four the ticket asked for, all in `frame.rs`

1. `build_frame_populates_parent_links_overlay_flags_and_listeners_on_the_real_tree` — asserts directly
   on `DispatchNode.parent`/`.flags`/`.listeners.binding_for(...)` for nodes reached via
   `tree.element_node(ElementId)`, never on geometry.
2. `a_click_three_levels_deep_bubbles_to_the_root_through_the_built_tree` — `hit_test` resolves the
   deepest of three real nested `Wrap<Wrap<TestLeaf>>` levels; a real `Dispatcher::dispatch(&tree,
   &DispatchEvent::PointerDown{..})` call then shows capture landed on the leaf and the hover chain
   (which bubbles via `DispatchNode.parent`) reached both ancestors up to the root.
3. `an_overlay_registered_last_is_hit_before_the_content_beneath_it_through_the_real_pipeline` — two
   fully-overlapping real `Pair` children, content registered first, `DispatchFlags::OVERLAY` child
   registered last; `hit_test` returns the overlay.
4. `a_layout_container_with_no_bindings_passes_the_hit_through_to_what_is_under_it` — a real
   `LAYOUT_CONTAINER` `Wrap` root whose child covers only part of it; a point inside the root but outside
   the child misses entirely (pass-through, not a match), a point on the child still hits it.

Test count: `frame.rs` 3 → 7 (+4, exactly the four above); `element.rs` and `dispatch.rs` unchanged at 7
and 33 respectively (confirmed via `git diff` — no existing test body was touched, only two doc-comment
regions and one now-obsolete `impl From` block were removed from `dispatch.rs`). Crate-wide `#[test]`
count across all 12 files in this package: **121 → 125**.

## Existing tests whose expectations changed

**None.** No existing test's assertions, inputs, or expected outputs were modified. `dispatch.rs`'s 33
tests are untouched byte-for-byte except for two doc comments above the `mod tests` block (updated to
stop describing the deleted `From` impl); confirmed via `git diff` that no `fn`/`#[test]` line was
removed or altered. The deleted `From<Vec<Hitbox>>` impl had no dedicated test of its own (its own doc
comment said as much: "exercised directly against hand-built `DispatchTree`s ... never through this
adapter"), so nothing lost coverage by its removal.

## acceptance: UNRUN (U4 — sol runs every gate)

```
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target cargo check -p semio-framework-ui-render --lib --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target cargo check -p semio-framework-ui-render --all-targets --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target cargo test -p semio-framework-ui-render --lib --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target cargo test -p semio-framework-ui-render --lib frame:: --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target cargo test -p semio-framework-ui-render --lib dispatch:: --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/cargo-target cargo tree -p semio-framework-ui-render --invert wgpu --timeout 600000   # must stay empty
```

**Cheap non-cargo checks performed, all three edited files, both before and after every edit:**
- Python brace-balance scan: depth 0 at EOF for all three files, no unbalanced `{`/`}`.
- Python paren-balance scan: depth 0 at EOF for all three files.
- `rustfmt --edition 2021 --check` against each file individually: exits non-zero (wants reformatting)
  but **zero `error:` lines** in any of the three — only cosmetic line-wrap diffs (this codebase's
  existing single-line struct-literal style vs rustfmt's default wrapping), meaning rustfmt's own parser
  accepted all three files as syntactically valid Rust.
- Manual cross-reference of every new call site (`cx.register`, `cx.with_children`, `crate::Hitbox{..}`,
  `crate::DispatchFlags::*`, `crate::ListenerSet{..}`, `crate::node_bounds`, `crate::hit_test`,
  `crate::Dispatcher::new`/`dispatch`/`capture_of`/`is_hovered`, `crate::DispatchEvent::PointerDown`,
  `crate::PointerInfo`/`PointerId`/`PointerKind`/`PointerButton`) against the actual `pub` signatures
  read directly from `dispatch.rs`/`element.rs` — no name, arity, or field mismatch found.
- `git diff` review confirming no test function was altered or removed, only added.

**One real risk area, flagged explicitly since I cannot run cargo (U4) to confirm it:** the four new
`build_frame`-level tests rely on `Sizing::Fill` (`Dimension::percent(1.0)`) resolving the *root* taffy
node to exactly the `Definite` available space passed to `compute_layout_with_measure` — standard,
well-established taffy/flexbox root-sizing behaviour, and the one taffy assumption these tests cannot
avoid (everything below the root is authored directly, not read from taffy — see Decisions). If this
assumption is wrong, the failure mode is a **wrong absolute rect for the root**, which every downstream
assertion in all four tests reads back from (`crate::node_bounds`), so a wrong assumption here would show
up as a clear geometry-based test failure, not a silent false pass — the `a_layout_container_with_no_
bindings_passes_the_hit_through_to_what_is_under_it` test additionally carries an explicit sanity
`assert!` on the query point's position that would fail loudly first if root sizing came out
unexpectedly small. If `sol`'s `cargo test` run shows a failure in this file, check this first.

## registrar-requests

None. Nothing touched here is on the U7 registrar-only list.

## deviations

None from the ticket's GOAL/RULES. The one thing worth naming as a **non-deviation**: I did not add a
`revision: UiRevision` field to `FrameInputs`, and did not extend `PrepaintCx` with any read access to
`LayoutCx`/taffy — both were considered (see Decisions) and both are genuinely out of this packet's
scope (real revision plumbing belongs to whichever packet wires a `UiSnapshot` into `build_frame`; a
`LayoutCx` read channel belongs to whichever packet builds the first real multi-child production
container, since none exist in this crate yet). Flagging both here rather than silently deciding them.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️element.rs` — `PrepaintCx` gained `dispatch:
  &mut DispatchTree`, a private `parent` field, `new`/`register`/`with_children`; `hitboxes: &mut
  Vec<Hitbox>` field removed; docstrings updated.
- `…/🦀️dispatch.rs` — `impl From<Vec<Hitbox>> for DispatchTree` deleted; `DispatchTree::hitboxes(&self)
  -> &[Hitbox]` accessor added; two doc comments updated to stop describing the deleted impl. No test
  body changed.
- `…/🦀️frame.rs` — `build_frame` builds `DispatchTree` directly via `PrepaintCx::new`/registration
  instead of `DispatchTree::from(hitboxes)`; `FrameSnapshot.hitboxes` field removed; module docstring
  updated; four new tests plus their `TestLeaf`/`Wrap`/`Pair` test-only `Element` harness added.

No other file created, edited, or removed.
