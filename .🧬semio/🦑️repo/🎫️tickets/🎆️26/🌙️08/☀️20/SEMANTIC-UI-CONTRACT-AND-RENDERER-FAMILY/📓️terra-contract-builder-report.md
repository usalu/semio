# 📓️ terra-contract-builder-report

Packet `contract-builder`, wave W1 remainder.

## Done

Replaced the `🦀️builder.rs` scaffold wholesale (only file owned), keeping the `//! @emoji` header and
`//#region 🔖️Builder` / `//#endregion 🔖️Builder` wrapper. 1252 lines, 12 tests. Structure, innermost
regions first:

- **`🧱️BuiltNode`** — the contract-local terminal shape every builder converts into:
  `{ key, component, layout, style, activity, disabled, accessibility, bindings, menu,
  children: Vec<BuiltNode> }`. Every field but `key`/`component` carries `#[serde(default,
  skip_serializing_if = ...)]` against its own type's `Default`, so a default-styled, default-laid-out,
  binding-free, childless node costs nothing beyond `key`+`component` on the wire — the "defaults must
  be invisible" requirement.
- **`🧩️Base`** — private `NodeBase` (id/layout/style/activity/disabled/accessibility/bindings/menu/
  children) plus `assemble()` (finishes a `NodeBase`+`Component` into a `BuiltNode`, leaving `key`
  empty when no `.id(..)` was set) and `positional_key(index)` (`"#{index}"`).
- **`🔧️Traits`** — three small chainable traits instead of one macro-generated pile, so each concrete
  builder opts into exactly the vocabulary that makes sense for its component:
  - `HasBase` (every builder): `id`, `disabled`, `activity`, `style`, `tone`, `variant`, `size`,
    `emphasis`, `density`, `menu`, `on`, `on_with`, `label`, `describe`.
  - `HasChildren` (container-shaped builders only — enforced at compile time, not by convention):
    `child(impl Into<BuiltNode>)`, `children(impl IntoIterator<Item: Into<BuiltNode>>)`.
  - `HasStackLayout` (same set as `HasChildren` today): `gap`, `padding`, `align`, `justify`, `grow`,
    `wrap`, mutating the builder's own `LayoutSpec::Stack`.
  - `Buildable` — blanket-implemented for every `T: Into<BuiltNode>`, giving every builder a free
    `.build()` that fills an empty key with `"#0"`. `ImageBuilder` shadows it with an inherent `build`
    (see Decisions).
- **Sixteen constructors, thirteen builder types**: `stack`/`column`/`row` → `StackBuilder`
  (`Container(role: Plain)`); `section`/`field` → `ContainerBuilder` (`Container(role: Section|Field)`,
  plus `.description`/`.required`/`.error`/`.default_open`/`.drop_overlay`); `text` → `TextBuilder`
  (`+.emphasize`); `button` → `ButtonBuilder` (`+.icon`, auto-derives accessibility label from the
  visible label); `input` → `InputBuilder` (`+.value/.placeholder/.commit/.min/.max/.step/.accept`);
  `toggle` → `ToggleBuilder` (`+.icon/.text`, `.text` back-fills accessibility label if unset);
  `select` → `SelectBuilder` (`+.item/.items/.placeholder`); `slider` → `SliderBuilder`
  (`+.min/.max/.step/.unit`, defaults `0.0..=1.0` step `0.1`); `tree` → `TreeBuilder`
  (`+.interaction_domain`); `tree_section` → `TreeSectionBuilder` (`+.default_open`); `tree_item` →
  `TreeItemBuilder` (`+.description/.icon/.default_open/.draggable/.drag_data/.dimmed/.row_action(s)`,
  same auto-accessibility as `button`); `image` → `ImageBuilder` (`+.alt/.decorative`, see Decisions);
  `surface` → `SurfaceBuilder` (wraps a caller-built `SurfaceProps` verbatim); `extension` →
  `ExtensionBuilder` (`+.props`).
- **Tests** (12, region `🧪️Tests`, four subregions): wire-cost (`button("Save")` json omits
  `layout`/`style`/`disabled`/`bindings`/`menu`/`children`); nested shape (`column().children([...])`
  and mixed-type `.child().child()` chaining); bindings (`.on`/`.on_with` land in `bindings`, `args`
  correctly `None`/`Some`); accessibility (button auto-derivation, explicit `.label` override, image
  panics with no alt/decorative, image decorative sets `hidden`+omits `alt`, image alt populates both
  the component and accessibility); keys (positional keys stable across two builds and distinct among
  three siblings, explicit `.id` overrides the positional fallback for one sibling while its neighbor
  still gets its position).

No other file touched. `📦️glue.rs` (already mounts `mod builder;` / `pub use builder::*;`),
`Cargo.toml`, and all nine sibling region files were read only, for API surface.

## Acceptance: UNRUN

Per U4: executors write code and reasoning and run only cheap non-cargo checks; `sol` runs every gate.
I did not invoke cargo. Checks performed instead, on the one owned file:

- `rustfmt --edition 2021 --check 🦀️builder.rs` (rustfmt is not cargo) — **parses clean**: 3 diffs, all
  pure line-wrapping on long `assemble(..., Component::X(XProps { ... }))` calls, matching this crate's
  own established style of long single-line struct literals elsewhere (e.g. `🦀️limits.rs`'s test
  fixtures). No parse errors — a real syntax error would fail differently, not as a wrap-only diff.
- Python brace/paren/bracket balance over the full file, comment/string-aware: 0 remaining open, 0
  unmatched close.
- `grep -n "async fn"` — zero hits (only doc-comment mentions of the U1 rule).
- `grep -nw dyn` — zero hits.
- Every one of the 126 `fn` sites that has a body carries the `// 🚫️async: U1 …` tag immediately above
  it (verified with a script comparing each `fn` line's preceding two lines against the tag string);
  the one exception, `HasBase::base_mut`, is a required trait method with no body and is tagged too for
  completeness. `#[test] fn` bodies are intentionally untagged, matching every sibling file's own
  convention (checked `🦀️document.rs`/`🦀️style.rs`/`🦀️action.rs` test modules — none tag test fns).
- Manual field-by-field cross-check of every `crate::XxxProps { ... }` construction in every `From<...>
  for BuiltNode` impl against the actual struct definitions in `🦀️component.rs`/`🦀️layout.rs`/
  `🦀️style.rs`/`🦀️action.rs`/`🦀️accessibility.rs`/`🦀️surface.rs` (field names, types, `Option`-ness) —
  no mismatches found.
- Hand-traced the positional-key algorithm against both key-related tests line by line (see the trait
  doc comment on `HasChildren::child` for the algorithm) — traces match the asserted output.

The coordinator should run `cargo check -p semio-framework-ui-contract --lib` and `--all-targets`
(`CARGO_TARGET_DIR` in the scratchpad, 600000 ms timeout), then `cargo test -p semio-framework-ui-contract
--lib` for the 12 new tests plus the crate's existing 61. Expected: green — every symbol this file
references (`crate::Component`, `crate::LayoutSpec`, `crate::StyleSpec`, `crate::Tone`/`Variant`/
`SizeToken`/`Emphasis`/`Density`, `crate::Trigger`/`ActionId`/`ActionBinding`/`UiValue`,
`crate::AccessibilitySpec`, `crate::Label`, `crate::MenuRef`, `crate::SelectItem`, `crate::RowAction`,
`crate::DropOverlaySpec`, `crate::SurfaceProps`, every `*Props` struct) is defined in an already-landed
sibling file, and no new external dependency was added.

## Decisions

**`BuiltNode` vs the runtime's `ComponentTree` — flagging the duplication, coordinator call.** The
brief is explicit that this crate must not depend on `semio-framework-ui-runtime` (dependency runs the
other way), so `BuiltNode` necessarily duplicates shape the runtime also needs: id-less node,
inline-nested children, same field set as `UiNodeRecord` minus `id` plus recursion instead of
`UiNodeId` addressing. This is the same "one recursive escape hatch, deliberately" pattern the crate
already uses for `UiValue` (see `🦀️action.rs`'s own doc comment) and for the exact reason
`🦀️document.rs`'s header calls out: an authored tree has no ids yet to address children by, so it
cannot be flat like `UiSnapshot`. I judge this is the right shape to keep — not one to merge with
`ComponentTree` — because the two types answer different questions (authoring-time tree literal vs.
reconciler-ready structure) and merging them would either leak runtime-only fields backward into this
dependency-free crate, or leak `BuiltNode`'s builder-specific empty-key sentinel forward into the
runtime. The one place duplication could actually bite is if a `*Props` field shape changes later and
someone updates `component.rs` without updating `builder.rs`'s matching `From` impl — that risk exists
for any of the nine landed sibling files' consumers, not specifically because of this packet.

**Accessibility defaults — literal ask plus one consistent extension.** The brief names two examples
explicitly (`button` auto-derives, `image` must supply `alt` or opt out) as the pattern to apply
"because the builder is the cheapest place to enforce it." I implemented both literally, and extended
the same auto-derivation to `tree_item` (its `label` is exactly as much a primary visible name as a
button's) and to `ToggleBuilder::text` (back-fills accessibility label only if `.label(..)` was not
already called explicitly, via `get_or_insert_with`, so an explicit override always wins). I did not
extend it to `select`/`slider`/`toggle`'s own construction (no natural single visible-label argument at
construction time) — those still get an accessible name only via the universal `.label(..)`, which I
judged an acceptable gap rather than over-reaching into a guess (e.g. "value" is not a good accessible
name for a slider). `image` enforces its rule at runtime — `ImageBuilder::build` panics if neither
`.alt(..)` nor `.decorative()` was called — rather than at compile time, because the brief pins the
constructor's exact signature (`image(src: impl Into<String>) -> ImageBuilder`, no alt parameter), which
rules out a typestate/phantom-generic compile-time encoding without breaking that literal API. This
mirrors how the crate already accepts a runtime check over a type-level one elsewhere
(`validate_snapshot`/`apply_patch` in `🦀️limits.rs` reject bad documents at a function boundary, not
via the type system).

**Positional key scheme.** An unset `.id(..)` resolves to `"#{position}"` where position is the index
in the parent's `children` Vec at the moment `.child(..)` pushes it (0-based; `.build()` on a builder
with no parent falls back to `"#0"`, giving it a harmless default rather than an empty string). This is
stable for a fixed structure (same children, same order, same call sequence) across separate `.build()`
calls, and distinct among siblings by construction (monotonic Vec length at push time) — both covered
by tests. It is **not** stable across a reorder, insert, or removal among siblings, which is exactly why
every doc comment on `id`/`child`/`children` says explicitly: give an explicit `.id(..)` to any node
whose position can change (a reorderable list row, a filtered list, anything keyed by domain identity)
— that is the single most common authoring mistake this API can prevent, per the brief's own framing.

**Three small traits instead of one macro.** `HasBase`/`HasChildren`/`HasStackLayout` are ordinary
traits with default method bodies reaching through a `#[doc(hidden)] fn base_mut(&mut self) -> &mut
NodeBase` — no macro, no `dyn` (U3), and the child/stack-layout vocabulary is only available on builder
types that actually implement those traits (e.g. `ButtonBuilder` has no `.child(..)` — it fails to
compile, not silently drops the call at runtime). `NodeBase` itself is `pub` (required — a `pub trait`
method cannot return a private type, E0446) but every field stays private to this module, so external
callers only ever see it as an opaque handle and can never bypass the chainable methods.

**Style tokens — added beyond the brief's literal chainable list.** The brief's enumerated vocabulary
(`.id/.gap/.padding/.disabled/.activity/.label/.describe/.on/.on_with/.menu/.child/.children`) has no
entry for `crate::StyleSpec`'s five tokens (`tone`/`variant`/`size`/`emphasis`/`density`). Without some
way to reach them, no migrated plugin could express something as basic as a danger-toned delete button.
Added `HasBase::style(StyleSpec)` (bulk override) plus five one-line convenience setters
(`.tone/.variant/.size/.emphasis/.density`). Also added `HasStackLayout::align`/`.justify`/`.grow`/
`.wrap` alongside the two the brief names (`gap`/`padding`) — same rationale, and each is a one-line
pass-through onto a field the type already carries.

## Registrar-requests

None. Every symbol needed already exists in the nine landed sibling files; no `Cargo.toml`/glue/taxonomy
change requested.

## Deviations

- `HasBase::style`/`.tone`/`.variant`/`.size`/`.emphasis`/`.density` and
  `HasStackLayout::align`/`.justify`/`.grow`/`.wrap` are additive beyond the brief's literal chainable
  list — see Decisions above for why each was judged necessary rather than scope creep.
- Auto-derived accessibility labels extended to `tree_item` and `ToggleBuilder::text` beyond the brief's
  two named examples (`button`, `image`) — see Decisions.
- `ImageBuilder::build` panics rather than silently omitting `alt` when neither `.alt(..)` nor
  `.decorative()` was called — a deliberate loud-failure choice given the brief pins the constructor
  signature and rules out a compile-time encoding; flagging in case the coordinator prefers a different
  failure mode (e.g. a `Result`-returning `try_build`).
