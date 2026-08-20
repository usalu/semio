# 📓️ Packet `render-surface` report

## done

Rewrote `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️surface.rs` wholesale (the file was
the packet's own SCAFFOLD placeholder; nothing else was touched). Three top-level regions, per the
packet brief:

- **`🔖️Placement`** — `ClipId` (opaque clip handle, `ROOT` constant), `Transform2D` (2D affine matrix
  with `apply_point`/`invert`), `SurfacePlacement { id, bounds, clip, transform, z_index }` exactly as
  specified, and `route_pointer_event` (bounds check → surface-local coordinates via the inverse
  transform; deliberately not clip-aware — full transformed-bounds/clip-aware hit testing is packet
  `render-dispatch`'s job, this is the per-surface primitive it composes on top of).
- **`🔖️Surface`** — `SurfaceRenderTarget`, `SurfaceResourceNeeds`, `SurfacePrepare`, `SurfaceRenderCx`
  (scene + resources + placement + time, no device), `PointerButton`, `SurfaceInput`, `SurfaceError`,
  and the `Surface` trait with the exact five-method signature the brief specified (`update_snapshot`,
  `prepare`, `render`, `handle_input`, `next_deadline`). Plus `render_placed_surface` (applies
  `placement.bounds` as a scissor around one `render` call) and `dispatch_pointer_moved`.
- **`🔖️Registry`** — `AnySurface` (fn-pointer vtable + `Box<dyn Any>` storage, mirroring
  `crate::element::AnyElement`'s established mechanism), `PlaceholderSurface` (paints a visible tinted
  rounded rect and always returns `SurfaceError::Unregistered`), and `SurfaceRegistry` (a
  `HashMap<SurfaceKind, fn(SurfaceKind) -> AnySurface>` of registered-kind thunks; `create` degrades an
  unknown kind to the placeholder plus the same error, never a blank).

18 in-file `#[cfg(test)] mod tests` cases cover every TESTS-section case named in the brief: placement/
z-order/clip in the emitted `RenderPacket` (two differently-scissored, differently-clipped surfaces
rendered in ascending z order produce two un-merged `DrawBatch`es in call order, each batch's scissor
matching its own placement's bounds), an animating surface's `next_deadline` vs a still surface's
`None`, input outside vs inside bounds (not routed / arrives in local coordinates), an unregistered
`SurfaceKind` painting a placeholder and erroring rather than going silent, and a snapshot revision
that hasn't changed not re-marking dirty.

## acceptance: UNRUN

Per U4 I do not run cargo. `sol` should run, both with `CARGO_TARGET_DIR` pointed at the session
scratchpad (never the ticket folder) and `timeout 600000`:

```
cargo test  -p semio-framework-ui-render --lib
cargo check -p semio-framework-ui-render --lib
cargo check -p semio-framework-ui-render --all-targets
```

Non-cargo checks I did run: a brace/paren/bracket balance pass over the whole file (`python3`, string/
char/comment aware) — balanced. Manually re-traced `Scene::finish`'s `order()`/`batch()` logic against
the new placement/z-order test to confirm two scissor-distinct layers survive un-merged in push order
before trusting that assertion. Grepped the crate's other five landed files for every symbol I import
or extend (`SceneBuilder::push_scissor/push_rounded/push_solid`, `ScissorRect::from_rect`,
`DeviceCapabilities`, `PhysicalSize`, `Deadline`, `Bounds`, `ui_contract::{SurfaceId, SurfaceKind}`) to
confirm exact signatures/derives before using them, and confirmed `Box<dyn Any>::as_mut()`/
`Rc<dyn Any>::downcast()` against the identical pattern already landed in `🦀️element.rs`'s
`AnyElement`. `📦️glue.rs` already had `mod surface;` / `pub use surface::*;` wired from the scaffold —
not touched, no registrar request needed there.

## decisions

**Erasure mechanism for the heterogeneous surface registry (no `dyn`):** the packet brief offered a
choice between a closed enum over first-party surface kinds and fn-pointer thunks. I picked **fn-pointer
thunks**, for the same structural reason `🦀️backend.rs`'s docstring gives for not aliasing
`ActiveBackend` to a real backend in that crate: every concrete `Surface` implementation will live in a
product crate that *depends on* `semio-framework-ui-render`, so this crate can never name them in a
closed enum without inverting that dependency graph — unlike `GraphicsBackend` (exactly one impl per
build target, resolved by a `cfg` alias one layer up in `ui-host`), multiple *different* surface kinds
coexist in one document at once, so even a per-target alias wouldn't fit. `AnySurface` reuses the exact
"fn-pointer vtable + safe `Box<dyn Any>` storage" technique `🦀️element.rs`'s `AnyElement` already
established in this crate (`dyn Any` is U3-permitted, not being a first-party trait); `handle_input`'s
intents come back as `Vec<Box<dyn Any>>` for the same reason — a caller downcasts using the
`Self::Intent` type it already knows for whichever `SurfaceKind` it queried.

**How a product crate registers an implementation:** implement `Surface` directly (plain generics, zero
erasure needed for a crate that only ever handles its own one kind), then hand
`SurfaceRegistry::register::<MyWorld3dSurface>(SurfaceKind::World3d)` a monomorphized
`fn(SurfaceKind) -> AnySurface` thunk. `register`'s only extra bound beyond `Surface` is `Default` — a
freshly-registered surface starts empty and receives real content through the next frame's
`update_snapshot`, the same lifecycle `crate::element::RetainedStore::get_or_insert_with` already uses
for other per-id retained state in this crate. This is the pattern the 3d/surface product crates
should follow.

**`ClipId` stays opaque.** This crate has no clip-resolution system of its own (`SceneBuilder`'s real
`ClipRegion` stack is private, and any general clip *registry* keyed to arbitrary ids belongs to the
not-yet-landed `render-dispatch` dispatch tree). `SurfacePlacement.clip` is carried faithfully through
into `SurfaceRenderCx.placement.clip` and nothing here interprets it — tested by two placements with
distinct `ClipId`s reaching their own `render` call with the right one, never the other's.

## registrar-requests

None. `📦️glue.rs`'s `mod surface;` / `pub use surface::*;` were already present from the scaffold.

## deviations

None from the packet brief. `SurfaceRenderCx` and `SurfacePrepare`'s exact field sets were left to my
judgment (the brief specified the `Surface` trait's method signatures and `SurfacePlacement`'s fields
verbatim but not these two); both are documented in-file.
