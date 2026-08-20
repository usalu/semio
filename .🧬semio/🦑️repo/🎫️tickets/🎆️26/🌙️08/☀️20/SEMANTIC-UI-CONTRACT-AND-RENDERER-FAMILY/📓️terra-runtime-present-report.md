# Packet `runtime-present` — actual-read dependency tracking and the `Present` trait

## Done

Wrote both owned files wholesale, replacing the scaffolds:

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️tracking.rs` — region `🔖️Tracking`:
  `EntityId`, `DependencyTracker` (`begin`/`record_read`/`finish`/`dirty_surfaces_for`/
  `notify_entity`/`drain_dirty`), 7 `#[test]`s.
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️present.rs` — regions `🔖️Present` /
  `🔖️ComponentTree`: `Present` trait + blanket stateless impl, `PresentCx<'a>`, `TreeNode`,
  `ComponentTree`, `position_key`, `assert_unique_sibling_keys`, 5 `#[test]`s.

Nothing outside the OWNS list was touched. `📦️glue.rs` already declares and re-exports both modules
(`mod tracking; mod present; pub use present::*; pub use tracking::*;`) from the earlier scaffold
commit — no edit needed there.

## Acceptance: UNRUN (U4 — I do not run cargo)

```
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8fcf59e9-0317-475e-8aa4-dd949409752d/scratchpad/cargo-target \
  cargo check -p semio-framework-ui-runtime --lib --timeout 600000
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8fcf59e9-0317-475e-8aa4-dd949409752d/scratchpad/cargo-target \
  cargo check -p semio-framework-ui-runtime --all-targets --timeout 600000
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8fcf59e9-0317-475e-8aa4-dd949409752d/scratchpad/cargo-target \
  cargo test -p semio-framework-ui-runtime --lib --timeout 600000
```

Expected to fail until packet `runtime-entity` lands real `Entity<T>`/`Context` types (currently empty
scaffolds in `🦀️entity.rs`/`🦀️context.rs`) — `present.rs` references `crate::Entity<T>` and
`crate::Context` by path per the packet brief, unresolved for now as instructed. The
`tracking.rs`/`present.rs` `#[test]` modules were deliberately written to need nothing from
`runtime-entity` at all (see Deviations), so once `runtime-entity` lands, only `PresentCx::read`'s
body needs those two names to resolve — everything else in both files should compile and its tests
should pass standalone.

## Decisions — exact signatures `runtime-reconcile`/`runtime-transact` must call

```rust
// tracking.rs
pub struct EntityId(pub u64); // Copy, Eq, Hash, Debug, Default

pub struct DependencyTracker { /* private fields */ } // Debug, Default (use DependencyTracker::default())

impl DependencyTracker {
    pub fn begin(&mut self, surface: ui_contract::SurfaceId);
    pub fn record_read(&mut self, entity: EntityId); // no-op with no scope open
    pub fn finish(&mut self, surface: ui_contract::SurfaceId); // panics if mismatched with begin
    pub fn dirty_surfaces_for(&self, entity: EntityId) -> impl Iterator<Item = ui_contract::SurfaceId> + '_;
    pub fn notify_entity(&mut self, entity: EntityId); // coalesces into the dirty set
    pub fn drain_dirty(&mut self) -> impl Iterator<Item = ui_contract::SurfaceId> + '_;
}

// present.rs
pub trait Present: 'static {
    fn present(&self, cx: &mut PresentCx<'_>) -> ComponentTree;
}
// blanket: impl<F: Fn(&mut PresentCx<'_>) -> ComponentTree + 'static> Present for F

pub struct PresentCx<'a> { /* private: &'a mut DependencyTracker, &'a crate::Context */ }
impl<'a> PresentCx<'a> {
    pub fn new(tracker: &'a mut DependencyTracker, context: &'a crate::Context) -> Self;
    pub fn read<T: 'static>(&mut self, entity: &crate::Entity<T>) -> &'a T;
}

pub struct TreeNode { pub key: String, pub component: ui_contract::Component,
    pub layout: ui_contract::LayoutSpec, pub style: ui_contract::StyleSpec,
    pub activity: ui_contract::Activity, pub disabled: bool,
    pub accessibility: ui_contract::AccessibilitySpec, pub bindings: Vec<ui_contract::ActionBinding>,
    pub menu: Option<ui_contract::MenuRef>, pub children: Vec<TreeNode> }
impl TreeNode {
    pub fn new(key: impl Into<String>, component: ui_contract::Component) -> Self;
    pub fn at(position: usize, component: ui_contract::Component) -> Self; // key = position_key(position)
    pub fn with_children(self, children: impl IntoIterator<Item = TreeNode>) -> Self; // asserts unique keys
}
pub fn position_key(position: usize) -> String; // "#{position}"
pub fn assert_unique_sibling_keys(children: &[TreeNode]); // panics naming the first duplicate

pub struct ComponentTree { pub root: TreeNode }
impl ComponentTree {
    pub fn new(root: TreeNode) -> Self; // full-tree duplicate-key sweep, non-recursive stack
}
```

**Driving protocol** (the contract `runtime-reconcile`/`runtime-transact` must follow): call
`tracker.begin(surface)`, construct `PresentCx::new(&mut tracker, &context)`, call
`presenter.present(&mut cx)`, then `tracker.finish(surface)`. For a nested present (one presenter
driving another's `present()` internally), nest another `begin`/`new`/`present`/`finish` inside the
outer one — `DependencyTracker`'s scope stack (tested directly) attributes reads to whichever scope is
innermost at the time, so nesting is correct with no extra API on `PresentCx` itself.

**`EntityId` lives in `tracking.rs`, not `entity.rs`.** `DependencyTracker` is the type that actually
needs a hashable entity identity; `Entity<T>`/`WeakEntity<T>` are handles. Expected shape:
`Entity::<T>::id(&self) -> crate::EntityId` and `Entity::<T>::read<'b>(&self, cx: &'b crate::Context) -> &'b T`
— **`runtime-entity` should reuse `crate::EntityId`, not mint a second identity type**; U2 calls a
duplicate definition worse than an unresolved forward reference, which is why this packet defined it
rather than leaving it for `runtime-entity` to invent independently.

## Registrar-requests

None — no root/registrar-only file needed a change (`📦️glue.rs`'s module wiring for both files
already existed from the scaffold).

## Deviations

- The packet's five listed test scenarios ("presenter reads A but not B…", "stale edge disappears…",
  "N notifications coalesce…", "nested present scopes…", entity-vs-event-handler reads) are all
  implemented in `tracking.rs` directly against raw `EntityId` values — never through a real `Present`
  impl or `PresentCx::read` — because `DependencyTracker` doesn't know about `Entity<T>` at all, and
  because `crate::Entity`/`crate::Context` have no constructible values yet (empty scaffolds). This
  keeps `tracking.rs`'s tests fully self-contained and runnable the moment this crate itself compiles,
  independent of `runtime-entity`'s landing.
- `present.rs`'s own tests (duplicate-key detection at two levels, 3-level `ComponentTree`
  build-and-compare, a plain `fn` item satisfying `Present` generically via the blanket impl) never
  call `Present::present()` or `PresentCx::read()`, for the same reason. `PresentCx::read`'s body is
  written to the exact signature the brief specifies and will only type-check once `runtime-entity`
  lands; verifying its actual runtime behavior is `runtime-entity`'s/the coordinator's job once both
  packets land together.
- No `#[cfg(feature = "typegen")]`/`ts_rs` anywhere in either file — `TreeNode`/`ComponentTree` are
  internal builder-side types that never cross the wire, matching the brief.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️tracking.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️present.rs`
