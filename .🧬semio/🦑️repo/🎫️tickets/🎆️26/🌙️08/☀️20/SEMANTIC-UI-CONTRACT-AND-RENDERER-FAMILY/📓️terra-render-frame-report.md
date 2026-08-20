# 📓️ terra-render-frame — packet `render-frame`

## taffy-api-correction (follow-up from sol's gate run)

`sol` ran `cargo check -p semio-framework-ui-render --lib`: 15 errors, 13 of them the taffy-API risk
this report already flagged, 2 unrelated and not mine (`DispatchTree`/`TextSystem`, packets
`render-dispatch`/`render-text`, untouched). Confirmed every one against the vendored source at
`~/.cargo/registry/src/index.crates.io-*/taffy-0.9.2/src/` rather than by guessing again, fixed
`🦀️layout.rs` and `🦀️frame.rs` accordingly. No cargo run to verify (U4) — checked with
`rustfmt --edition 2021 --check` (parses cleanly, zero `error:` lines, only expected line-wrap diffs)
plus a brace-balance count, both before and after the fix.

| wrong (what I wrote) | right (taffy 0.9.2) | confirmed at |
|---|---|---|
| `Dimension::Auto` (assoc. const / enum variant) | `Dimension::auto()` (method) | `taffy-0.9.2/src/style/dimension.rs:246` |
| `Dimension::Length(v)` | `Dimension::length(v)` (method) | `taffy-0.9.2/src/style/dimension.rs:231` |
| `Dimension::Percent(v)` | `Dimension::percent(v)` (method) | `taffy-0.9.2/src/style/dimension.rs:239` |
| `LengthPercentage::Length(v)` | `LengthPercentage::length(v)` (method) | `taffy-0.9.2/src/style/dimension.rs:29` |
| `grid_track_to_taffy` returning `Vec<TrackSizingFunction>` for `Style.grid_template_columns`/`_rows` | `Style`'s actual field type is `GridTrackVec<GridTemplateComponent<S>>` = `Vec<GridTemplateComponent<String>>` (`S` defaults to `DefaultCheapStr` = `String`) — `GridTemplateComponent<S>` is the `Single(TrackSizingFunction) \| Repeat(..)` union, not `TrackSizingFunction` itself | field: `taffy-0.9.2/src/style/mod.rs:474,477`; enum: `taffy-0.9.2/src/style/grid.rs:1226`; `GridTrackVec<A> = Vec<A>`: `taffy-0.9.2/src/util/sys.rs:42`; `DefaultCheapStr = String`: `taffy-0.9.2/src/util/sys.rs:33` |

`Dimension`/`LengthPercentage`/`LengthPercentageAuto` all moved from bare enums to structs wrapping a
private `CompactLength` in 0.9, with `length`/`percent`/`auto` as `const fn` constructors instead of
variants — confirmed by reading the whole of `dimension.rs` (327 lines), not just the failing calls.

**The actual fix for the grid mismatch:** rather than converting `TrackSizingFunction` → `GridTemplateComponent`
by hand, `grid_track_to_taffy` now asks `style_helpers::{auto,fr,length,min_content,max_content}` for a
`GridTemplateComponent` directly — confirmed at `taffy-0.9.2/src/style/grid.rs:1250-1284` that
`GridTemplateComponent<S>` itself implements every one of the five marker traits those generic helpers
dispatch on (`TaffyAuto`, `TaffyMinContent`, `TaffyMaxContent`, `FromLength`, `FromFr`), the same way
`TrackSizingFunction` does, so the helper functions' generic return type just needed to be asked for
the right `T` — no new conversion code, no new call sites.

**`ElementId: Default`** — traced to `frame.rs`'s `AccessibilityNode` struct having derived `Default`
directly on a struct whose `element: ElementId` field has no `Default` impl (`Option<ElementId>` in
`FocusSnapshot` and `Vec<AccessibilityNode>` in `AccessibilitySnapshot` were both already fine without
it — `Option`/`Vec`'s own `Default` never requires their contained type to be `Default`; only
`AccessibilityNode`'s *own* struct-level derive actually needed `ElementId: Default`). **Decision: do
not implement `Default` for `ElementId`, at all — restructured the one call site instead** (removed
`Default` from `AccessibilityNode`'s derive list). Reasoning: every legitimate `ElementId` is
`fxhash(parent, key)`; a manufactured default (e.g. an all-zero sentinel) is not reserved out of that
hash's range and could silently collide with a real element's id — exactly the "mysterious element
collision" the follow-up warned against. Nothing legitimately needs an out-of-thin-air
`AccessibilityNode` either: it is only ever produced by a real accessibility walk and only ever consumed
inside `AccessibilitySnapshot::nodes: Vec<AccessibilityNode>`, whose own empty-`Vec` default requires
nothing of its element type. Documented this reasoning directly on `AccessibilityNode` in `frame.rs`.

## Done

Wrote all four owned files in `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/` (scaffolds → full
implementations, region structure preserved):

- **`🦀️element.rs`** — regions `🔖️Identity`, `🔖️Element`, `🔖️Arena`. `ReconciliationKey`/`ElementId`
  (hand-rolled FxHash), `Bounds` (alias to `crate::scene::LayoutRect`), `Hitbox`, `RetainedStore`
  (touch/sweep generational map), `SharedFrameCx`/`PrepaintCx`/`PaintCx`, the `Element` trait
  (`request_layout`/`prepaint`/`paint`), `AnyElement` (fn-pointer vtable + `Box<dyn Any>` erasure),
  `FrameArena`/`FrameArenaIndex` (take/put-back bump arena). 8 in-file tests.
- **`🦀️layout.rs`** — region `🔖️Layout`. `LayoutNodeId` (opaque `taffy::NodeId` wrapper),
  `AvailableSpace`/`Measurement`/`MeasureFn`, the full `ui_contract::LayoutSpec` → `taffy::Style`
  mapping (`Leaf`/`Stack`/`Grid`/`Overlay`/`Scroll`/`Absolute`), `LayoutCx` (fresh-per-frame taffy
  tree: `leaf`/`container`/`set_children`/`compute`/`resolved`/`had_pending_measurement`), logical
  pixel-snap. 6 in-file tests.
- **`🦀️frame.rs`** — region `🔖️Frame`. `FrameGeneration`, `FocusSnapshot`/`ImeSnapshot`/
  `AccessibilityNode`/`AccessibilitySnapshot` (defined here — nothing else in the repo names them),
  `FrameSnapshot`, `FrameInputs`, `FrameEngine` (`presented`/`build_frame`). 3 in-file tests (see
  "Deviations" for why `build_frame` itself is not yet testable).
- **`🦀️schedule.rs`** — region `🔖️Schedule`. Hand-rolled bitflag `InvalidationReason` (10 reasons),
  `Deadline`, `FrameScheduler` (`invalidate`/`request_deadline`/`set_visible`/`next_deadline`/
  `should_render`). 7 in-file tests, including the zero-idle-frame guarantee and hidden-window
  deadline tracking.

Did not touch any file outside the OWNS list (`🦀️scene.rs`, `🦀️resource.rs`, `🦀️backend.rs`,
`🦀️shader_contract.rs`, `📦️glue.rs`, `Cargo.toml`, `🦀️dispatch.rs`, `🦀️text.rs`, `🦀️surface.rs`,
`🦀️tessellate.rs` — read-only, for context).

## Acceptance: UNRUN

Per U4, no cargo command was run. Exact commands for `sol`, `CARGO_TARGET_DIR` in the session
scratchpad, both `--lib` and `--all-targets`, 600000 ms timeout:

```
CARGO_TARGET_DIR=<scratchpad>/cargo-target cargo check -p semio-framework-ui-render --lib --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/cargo-target cargo check -p semio-framework-ui-render --all-targets --timeout 600000
CARGO_TARGET_DIR=<scratchpad>/cargo-target cargo test -p semio-framework-ui-render --lib --timeout 600000
```

This packet's own crate **will not compile yet** even in isolation — `🦀️element.rs`/`🦀️frame.rs`
reference `crate::TextSystem` (packet `render-text`) and `crate::DispatchTree` (packet
`render-dispatch`) by path, per this ticket's explicit instruction to leave those unresolved rather
than stub them. This is `blocked-external` per ruling U2, not a defect in this packet. Once either
lands, re-run the above.

**Cheap non-cargo check actually run:** `rustfmt --edition 2021 --check` against each of the four
files individually (parses and re-emits each file; does not resolve names/types/macros, but does
catch gross syntax errors). All four parsed cleanly — `element.rs`/`layout.rs`/`frame.rs` only
produced pure line-wrapping diffs (rustfmt wanting long struct literals condensed onto one line,
matching this codebase's existing single-line struct-literal style seen in the landed siblings), no
structural changes; `schedule.rs` produced no diff at all. Brace/paren balance was also checked
per-file (all balanced) before that.

## Decisions

- **Element erasure mechanism (U3):** fn-pointer vtable (`ElementVTable`, three plain `fn` items
  monomorphized per concrete `E: Element`, stored by value) pairing with `Box<dyn Any>` storage for
  the erased element and its per-phase state. `dyn Any` is explicitly U3-permitted (not a first-party
  trait); the vtable's generic glue functions are the only place `downcast_mut::<E>()` happens, and it
  always succeeds because the vtable and the boxed value are constructed together in `AnyElement::new`
  and never mixed. No `unsafe` anywhere in the crate's new code.
- **`FrameArena` is a `Vec<Option<AnyElement>>`, not a raw-byte bump allocator.** "Bump-allocated,
  dropped wholesale" is satisfied by append-only `alloc` (O(1), no mid-frame slot reuse) plus a single
  `clear()` at the end of `build_frame`; a literal raw-bytes arena was considered and rejected as
  unnecessary unsafe complexity for a semantics that a growable `Vec` already delivers safely. A
  container element recurses into a child via **take → recurse → put back**
  (`arena.take(index)` → `child.request_layout(id, cx)` → `arena.put_back(index, child)`), which is
  what makes it sound for a child's phase call to receive the *same* `cx` that contains the arena the
  child was just taken out of (documented at length in `element.rs`'s module docstring).
- **`ElementId::new(parent, key)` uses a hand-implemented FxHash** (rotate-left-5 XOR multiply, the
  same primitive `rustc-hash` uses), not a crate dependency — this crate's `Cargo.toml` is
  registrar-only (U7) and already carries none, and the wgpu-old target this packet ports from
  hand-rolled its own generational arena for the identical reason (see `🦀️arena.rs`).
- **`LayoutNodeId` wraps `taffy::NodeId` with a private field**, deliberately deviating from the
  ticket's illustrative `Element::request_layout` signature (which shows `taffy::NodeId` directly) in
  favor of the ticket's own separately-stated hard rule that layout.rs's "taffy types must not appear
  in any public signature outside this file." Treated the pseudocode as illustrative and the prose rule
  as binding; `element.rs`/`frame.rs` never name a taffy type.
- **The taffy tree is rebuilt from scratch every `build_frame` call**, not retained and incrementally
  synced the way the wgpu-old `flex::LayoutEngine` did against its persistent `UiTree`. This follows
  directly from the ticket's own framing ("an element tree that is rebuilt from scratch every frame") —
  there is no persistent element tree left to incrementally sync a persistent taffy tree against, so
  `flex.rs`'s `prune_removed`/dirty-flag-gated `sync` machinery was not ported (see next section).
- **`crate::DispatchTree: From<Vec<Hitbox>>`** is `build_frame`'s chosen hand-off contract to packet
  `render-dispatch`, in place of either (a) guessing `DispatchTree`'s real constructor or (b) a
  `build_dispatch` closure parameter threaded through `build_frame`. A `From` impl is the more
  idiomatic, more discoverable contract for that packet's author to satisfy.
- **`FrameEngine.presented` is `Option<Rc<FrameSnapshot>>`**, not an always-present `Rc<FrameSnapshot>`
  seeded with a synthetic empty snapshot — building a legitimate empty `FrameSnapshot` up front would
  still need a `crate::DispatchTree` value from nowhere. `None` before the first successful
  `build_frame` is simpler and needs no fabricated initial state.
- **`FocusSnapshot`/`ImeSnapshot`/`AccessibilityNode`/`AccessibilitySnapshot` are new types defined in
  `frame.rs`** — grepped the repo first; nothing already names them. `AccessibilityNode` reuses
  `ui_contract::AccessibilitySpec` (the existing per-node contract type) rather than re-deriving an
  accessibility vocabulary.
- **`space_token_px`** (SpaceToken → logical px) is a provisional hand-picked table (`None`→0,
  `Xs`→4 … `Xxl`→32), generalizing the wgpu-old target's own hand-picked `gap_for_token`/
  `padding_for_token` values from two tokens to the full ramp `contract-layout` shipped. See
  registrar-requests below — this is the same open item `contract-layout`'s own docstring already
  flagged, not a new one.

## Registrar-requests

- None of the files this packet touched are registrar-only (U7), so nothing to hand off directly.
- Flagging for whichever packet eventually adds a real spacing ramp to `ui/styling/🔣️tokens.json`:
  once it exists, `layout.rs`'s `space_token_px` table should be replaced with a lookup against it
  (currently a self-contained provisional scale, documented in-file) — same open item
  `contract-layout`'s `🦀️layout.rs` docstring already flagged for `SpaceToken` itself.

## Deviations

- **`Element::request_layout`'s literal return type is `(LayoutNodeId, Self::LayoutState)`, not
  `(taffy::NodeId, Self::LayoutState)`** as shown in the ticket's illustrative code block — see
  "Decisions" above; this is required by the ticket's own "taffy types must not appear in any public
  signature outside `layout.rs`" rule, which the pseudocode itself would otherwise violate.
- **`LayoutSpec::Overlay`'s per-child `Anchor` (9-point placement) is not resolved.** `style_from_spec`
  maps `OverlayLayout` to a `position: Relative` flex container using only `inset` (as padding); the
  `anchor` field is read nowhere. `OverlayLayout`'s anchor conceptually describes how a *child* wants
  to be placed within the overlay, but nothing in `ui_contract::LayoutSpec` carries an anchor on the
  *child* side (only `AbsoluteLayout`'s plain `sizing_width`/`sizing_height`) — full anchor-based
  absolute positioning (CSS `inset: auto` + `margin: auto` centering tricks) needs more schema than
  currently exists. Flagging for whichever packet builds the first concrete overlay/modal/popover
  element, since it will need either an extension to `OverlayLayout`/`AbsoluteLayout` or a
  renderer-side convention for where anchor offsets live.
- **`flex.rs`'s incremental taffy-tree sync (`prune_removed`, dirty-flag-gated `sync`) was
  deliberately not ported** — see "Decisions": the frame model this packet implements rebuilds the
  element tree from scratch every frame, so there is no persistent taffy tree to incrementally sync.
  `tree.rs`'s `NodeFlags`/dirty-bit-propagation and `arena.rs`'s generational arena were read for
  reference but not ported either, for the same reason (no persistent retained tree exists here to
  carry them — `RetainedStore` replaces the *retained-state* half of that design, keyed by `ElementId`
  instead of `tree::NodeId`).
- **`frame.rs`'s own tests cannot exercise `build_frame`.** `FrameInputs::text` needs a real
  `crate::TextSystem` value and the `where crate::DispatchTree: From<Vec<Hitbox>>` bound needs a real
  `crate::DispatchTree` — both packets (`render-text`, `render-dispatch`) are still empty scaffolds, and
  neither type can be substituted by a local test-only stand-in since both are named by concrete path,
  not a generic parameter. `blocked-external` per U2. What the two missing tests should look like is
  written out in `frame.rs`'s `mod tests` doc comment so whoever lands either packet can add them
  directly: (1) build twice with a `Probe` root whose `paint` pushes valid geometry the first time and
  `f32::NAN` geometry the second, asserting the second call `Err`s and `engine.presented()`'s
  generation is unchanged from the first (`SceneError` rollback / monotonic generation), and (2) N
  scheduler invalidations coalescing into one `build_frame` call.
- **`taffy` 0.9's exact API surface (`TaffyTree::new`, `new_leaf_with_context`, `new_with_children`,
  `set_children`, `compute_layout_with_measure`'s 5-argument closure shape, `layout()`, the `Style`
  field names/types, `style_helpers::{auto,fr,length,min_content,max_content}`) was not directly
  verified against the pinned version — mirrored as closely as possible from the wgpu-old target's own
  working `flex.rs` (which calls the same methods in the same shapes, albeit against whatever taffy
  version that older crate pins) plus general taffy 0.7–0.9 API knowledge. This is the single highest-
  risk area for the first `cargo check` once `render-text`/`render-dispatch` unblock compilation — flag
  it first if that check fails.
