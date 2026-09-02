# 🎯️ `vello` cluster closed — `canvas::Scene` is now a first-party command list; `vello` moved
host/browser-only; puzzle/trinity 63→43, flow 64→44

## Headline — before/after (`cargo tree`, lock-free, cannot go stale)

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
    | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before (this session, = end of prior session) | after |
|---|---|---|
| `semio-s-plugin-puzzle` | **63** | **43** |
| `semio-s-plugin-flow` | **64** | **44** |
| `semio-s-plugin-trinity` | **63** | **43** |

`vello` and `vello_encoding` are **completely absent** from all three plugins' `wasm32-wasip2`
graphs (`cargo tree -p <plugin> --target wasm32-wasip2 -i vello` / `-i vello_encoding` → "nothing to
print" on all six checks). Net reduction: **20 crates per plugin** — one more than the predecessor's
"~19" estimate. Every crate on the predecessor's predicted-removal list (`vello`, `vello_encoding`,
`skrifa`, `read-fonts`, `font-types`, `png@0.17.16`, `flate2`, `miniz_oxide`, `fdeflate`,
`crc32fast`, `adler2`, `simd-adler32`, `guillotiere`, `moxcms`, `pxfm`, `svg_fmt`, `bytemuck_derive`,
`static_assertions`, `core_maths`, `euclid`, `num-traits`) is gone from the `wasm32-wasip2` name list;
the one predicted name still present, `id-arena`, was re-traced this session and confirmed to come
from an unrelated path — `wit-parser`/`wit-bindgen-core` (the WIT-binding codegen toolchain used by
every plugin), not from `vello` — so it was never actually part of the removable cluster; the
predecessor's estimate over-counted it. `bytemuck` (bare, not `_derive`) similarly persists via
`semio-framework-ui`, unrelated to `vello`.

`kurbo`/`peniko` (the value-type family `Affine`/`Stroke`/`Color`/`Paint`/`RasterImage`/shapes are
built on) remain in the graph — by design, per the ticket brief's option (a) — now as **direct**
dependencies of `semio-framework-os-infinite` rather than reached through `vello`, at the exact same
locked versions (`kurbo@0.13.1`, `peniko@0.6.1`) `vello@0.7.0` itself already resolved to, confirmed
via `cargo tree -p peniko@0.6.1` / `-p kurbo@0.13.1` before this pass — adding these two dependency
lines introduced **zero new crate versions**, only new edges onto already-locked packages, so the
native/browser build graph is bit-for-bit unaffected by this pass (confirmed: `cargo check -p
semio-framework-os-infinite`, native, `Finished`, 0 errors; `vello@0.7.0` still resolves there
unchanged).

## Verifying the predecessor's four claims

All four were verified independently this session before writing any code, per the ticket's
instruction not to build on unverified prior-session claims:

1. **"`🎲️board` never references `vello::`/`peniko::`/`kurbo::` directly."** Confirmed — `grep -rn
   "vello::\|peniko::\|kurbo::" 🎲️board/` returns zero matches. Every board/port file reaches drawing
   types exclusively through `canvas::{Scene, Affine, Color, Paint, Stroke, RasterImage, FillRule,
   BlendMode, ShapeRef, …}`.
2. **"Real rasterization is isolated to one already-host-gated call site."** Re-traced and found
   this was slightly imprecise — there are **two** real `render_to_texture` call sites in the whole
   repo, both already host/browser-gated, neither a new discovery but worth stating precisely since
   the predecessor's own sizing section separately says "two already-host-gated render call sites":
   - `🖼️canvas/🦀️.rs`'s `gpu_session::render_frame` (line ~1554 pre-edit), gated
     `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` — browser WebGPU only.
   - `📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1229` (the predecessor's
     doc mis-cited this as `EngineCanvas/🧊️component.rs` — the real path has an intermediate
     `🎯️targets/🧊️wgpu/` segment), unconditional in a crate that is itself never part of any
     plugin's `wasm32-wasip2` graph (confirmed again this session: `cargo tree -p
     semio-s-plugin-puzzle --target wasm32-wasip2 -i semio-framework-os-renderer-wgpu` → "did not
     match any packages").
   No third call site exists anywhere in the repo (`grep -rn "render_to_texture" 🧰️framework
   ✏️s` → exactly these two, confirmed both before and after this pass's edits).
3. **"Glyph-run buffers are never populated on this target."** Confirmed by construction rather than
   just by trace this time: the new first-party `Scene` has no glyph-run buffer at all — real glyph
   painting goes exclusively through `svg_icon::render_svg_tree_literal`/`render_svg_tree_themed`
   (first-party `fill`/`stroke` calls, unconditional, unaffected by this pass) or through
   `SvgDocument::append_to_scene` (the one `vello_svg::append_tree` caller, host/browser-gated,
   reachable only from `IconPaintCache::get_or_build`'s `preserve_original_style` arm — itself
   `#[cfg(not(all(wasm32, p2)))]`). Neither path exists on `wasm32-wasip2`, so the question of a
   guest-side glyph-run representation never arises.
4. **"Verdict: the guest only builds, measures and incrementally disposes scene data — it never
   rasterizes. Replaceable in principle."** Confirmed and now *acted on* — see below.

## The first-party scene representation

`canvas::Scene` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs`) is no longer
`pub struct Scene(pub(crate) backend::Scene)` (a `vello::Scene`/`vello_encoding::Encoding` wrapper).
It is now:

```rust
pub struct Scene(pub(crate) Vec<SceneCommand>);

enum SceneCommand {
    Fill { rule: FillRule, transform: Affine, paint: Paint, brush_transform: Option<Affine>, shape: RecordedShape },
    Stroke { stroke: Stroke, transform: Affine, paint: Paint, brush_transform: Option<Affine>, shape: RecordedShape },
    DrawImage { image: RasterImage, transform: Affine },
    PushLayer { rule: FillRule, blend: BlendMode, alpha: f32, transform: Affine, clip: RecordedShape },
    PushClipLayer { rule: FillRule, transform: Affine, clip: RecordedShape },
    PopLayer,
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    VelloFragment { scene: Arc<vello::Scene>, transform: Affine },
}

enum RecordedShape { Rect(Rect), RoundedRect(RoundedRect), Circle(Circle), Line(Line), Arc(Arc), CubicBez(CubicBez), BezPath(BezPath) }
```

`RecordedShape` is an **owned copy of a `ShapeRef` variant**, kept as the exact typed shape (never
flattened to a polyline `BezPath` at record time). This was a deliberate correctness call: flattening
early with a fixed tolerance, then applying a later transform (e.g. `scale_scene_for_device_pixel_ratio`,
or a camera zoom via `Scene::append`), would scale the flattening error along with the transform —
visibly faceting a circle when zoomed in, a real rendering-fidelity regression the original
`vello::Scene`-backed code never had (`vello` always flattened the *final*, fully-transformed shape at
its own device-appropriate tolerance). Storing the typed shape and replaying it into a real
`vello::Scene` only at rasterization time preserves this exactly.

### Did it reuse `semio-framework-raster`'s `VectorScene`/`DrawOp`/`FillOp`/`StrokeOp`? No — and why

Per the ticket's instruction, `semio-framework-raster`'s types were evaluated first. They do not fit:
`DrawOp` has exactly two variants (`Fill`/`Stroke`), each over a single `BezPath` with a plain `[f32;
4]` solid color, no fill rule, no brush transform, and no image/layer-push/layer-pop primitives.
`canvas::Scene`'s guest-reachable API needs `draw_image`, `push_layer`/`push_clip_layer`/`pop_layer`
(compositing groups with blend modes and alpha), `FillRule::NonZero/EvenOdd`, and exact (not
flattened) shape variants — none of which `raster`'s vocabulary carries, and retrofitting them would
have meant either forking `raster`'s types (defeating the point of reuse) or bolting unrelated fields
onto a crate whose actual job (GPU-rasterizing a *closed* vector scene to pixels) is unrelated to
`canvas::Scene`'s job (recording a mutable, appendable, layered scene graph fragment). What *was*
reused, per the ticket's actual instruction ("reuse those types if they fit"), is the underlying
**geometry vocabulary** — `semio-framework-geometry`'s `BezPath`/`Affine`/`Rect`/`RoundedRect`/
`Circle`/`Line`/`Arc`/`CubicBez` — the same primitives `raster`'s own `DrawOp` is built on. No second
geometry vocabulary was invented; only a canvas-local *command* vocabulary (`SceneCommand`) was
added, matching the shape of what `raster`'s `DrawOp` already established one layer up.
`semio-framework-os-infinite` does not depend on `semio-framework-raster` at all (before or after
this pass) — adding that dependency just to get two non-fitting structs was not worth the coupling.

## `retirement_step` — how it was resolved

The predecessor's stated blocker: *"`retirement_step`'s granularity is coupled to `vello_encoding`'s
internal buffer layout"* (11 separate `Vec`s — `glyph_runs`, `glyphs`, `normalized_coords`,
`color_stops`, `patches`, `path_tags`, `path_data`, `draw_tags`, `draw_data`, `transforms`, `styles`
— popped one at a time, short-circuiting on the first non-empty one).

Resolution: with `Scene` now `Vec<SceneCommand>`, `retirement_step`/`retirement_is_empty` are:

```rust
pub fn retirement_step(&mut self) -> bool { self.0.pop().is_none() }
pub fn retirement_is_empty(&self) -> bool { self.0.is_empty() }
```

Same return-value contract as before (`true` = fully retired, `false` = one more unit of work was
just retired), same O(1)-per-call cost bound (CLAUDE.md's "support progress and cancellation for all
expensive operations"), just at first-party command granularity — one `SceneCommand` per call —
instead of sub-command buffer-entry granularity. This is coarser per the *count* of steps but not per
the *cost* of a step: since glyph-run buffers are never populated on this target (verified claim #3
above), the 11-buffer version was, in practice, already only ever popping from `path_tags`/
`path_data`/`draw_tags`/`draw_data`/`transforms`/`styles` per drawing op on `wasm32-wasip2` — i.e.
already multiple pops per logical drawing command. Retiring one whole `SceneCommand` per call is
*fewer, still-O(1)* steps for the exact same logical content, not a larger unit of work smuggled in.

The one non-mechanical design decision this required was `Scene::append`: the original
`vello::Scene::append(&other, transform)` mutated a *stateful* internal buffer at the encoding level.
A `Vec<SceneCommand>`-based `append` could have nested `other`'s commands behind one opaque
`Append { commands: Vec<SceneCommand>, transform }` entry — mechanically simpler, but it would
reintroduce exactly the "one call synchronously drops/appends an unbounded amount of nested work"
problem `retirement_step` exists to avoid (dropping one `Append` entry would recursively drop its
whole nested `Vec`, an O(n) not O(1) cost hidden behind one `pop()`). Instead, `append` **flattens**:
every command in `other.0` is cloned and transform-composed (`outer * existing`, matching kurbo's own
`(a * b) * p == a * (b * p)` convention, confirmed against every real call site in `🎲️board` — camera
transforms applied to world-space child scenes on append) directly into `self.0`, so every appended
command remains independently, individually poppable. `SceneCommand::transformed(self, outer:
Affine)` does this per-variant, including for the one host-only `VelloFragment` variant (its own
`transform` field composes the same way).

## The one host-only escape hatch: `VelloFragment`

`SvgDocument::append_to_scene` (the sole real caller of `vello_svg::append_tree`, itself already
`#[cfg(not(all(wasm32, p2)))]`-gated, reachable only from `IconPaintCache::get_or_build`'s
`preserve_original_style` icon-painting arm — confirmed by grep, one call site repo-wide via the
public `append_svg_document` re-export) now builds a real `vello::Scene` fragment via
`vello_svg::append_tree`, wraps it `Arc<vello::Scene>`, and records it as one
`SceneCommand::VelloFragment { scene, transform: Affine::IDENTITY }` — the only `SceneCommand`
variant that is itself `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`-gated, since its
field type (`vello::Scene`) does not exist on `wasm32-wasip2`. `RecordedShape` on `wasm32-wasip2`
is consequently write-only (its fields are populated by every guest `fill`/`stroke`/`push_layer`
call but never read back, since the only reader, `SceneCommand::replay_into`, is host-only) — this
is the expected shape of "the guest builds and disposes but never rasterizes," made explicit with an
`#[allow(dead_code, reason = "...")]` on that target rather than left as an unexplained warning.

`Scene::vello_scene(&self) -> backend::Scene` (host/browser-only, `#[cfg(not(all(wasm32, p2)))]`) is
the single place a real `vello::Scene` is built from the first-party command list, by replaying every
`SceneCommand` in order (`geometry::with_shape_ref!` dispatches each `RecordedShape` back to the
exact typed `vello::Scene::fill`/`stroke`/`push_layer`/`push_clip_layer` call). Its signature changed
from `&backend::Scene` (a reference into an eagerly-built field) to an **owned** `backend::Scene`
(built fresh per call) — both of its two call sites were updated to bind the owned value to a local
before passing `&` to `render_to_texture`, no other change needed since both were already inside the
`not(all(wasm32, p2))`-reachable region.

## Cargo.toml — `semio-framework-os-infinite`

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`:

- Added, unconditional `[dependencies]`: `kurbo = "0.13.1"`, `peniko = "0.6.1"` — the exact versions
  already locked via `vello`'s own transitive tree (verified before adding, see Headline).
- Removed entirely from the `[target.'cfg(all(target_arch = "wasm32", target_env = "p2"))'.dependencies]`
  table: `vello = { version = "0.7.0", default-features = false }`.
- `vello = { version = "0.7.0", features = ["wgpu", "wgpu_default"] }` in the
  `not(all(wasm32, p2))` table (native + browser) is unchanged.
- `semio-framework-os-infinite`'s own `🖼️canvas/🦀️.rs`: `vello_backend`'s `pub use vello::kurbo;`/
  `pub use vello::peniko;` re-exports were removed (no longer needed — `kurbo`/`peniko` are now
  direct crate dependencies, reachable by bare name from anywhere in the crate via the 2018+ extern
  prelude); every `backend::kurbo::`/`backend::peniko::` reference in the file was mechanically
  rewritten to `kurbo::`/`peniko::` (18 call sites), which also cleared 18 `unused_qualifications`
  lint warnings `cargo check` raised once the direct dependency made the old `backend::` indirection
  genuinely redundant.

## Verification — full commands and tails

```
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i vello            → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i vello            → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i vello            → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i vello_encoding   → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i vello_encoding   → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i vello_encoding   → nothing to print
# native — capability preserved where it can actually run
$ cargo tree -p semio-s-plugin-puzzle -i vello
vello v0.7.0
├── semio-framework-os-infinite v0.1.0 (…/♾️infinite/📦️packages/🦀️rust)
│   └── semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
└── vello_svg v0.9.0
    └── semio-framework-os-infinite v0.1.0 (…/♾️infinite/📦️packages/🦀️rust) (*)
```

```
$ cargo check -p semio-framework-os-infinite                              (native)
   ...
   Finished `dev` profile [unoptimized] target(s) in 9.56s
   (0 errors; 63 warnings, all pre-existing dead-code in 🌍️world/🦀️.rs and one pre-existing
   future-incompat note for the `block` crate — zero warnings from any file touched this pass)

$ cargo check --lib -p semio-framework-os-infinite --target wasm32-wasip2
   ...
   Finished `dev` profile [unoptimized] target(s) in 52.67s
   (0 errors; 0 warnings from 🖼️canvas/🦀️.rs)

$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm
   Compiling semio-s-plugin-draw-fsm v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 1m 55s
   (the ticket's own "currently clean" 11-crate baseline — confirms the shared foundation
   (semio-framework-os-kernel, workspace-wide) was not broken by this pass, independent of
   os-infinite's own extra dependency edges)
```

`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-puzzle` — attempted, foreground, twice,
capturing full uncapped output the second time. Both attempts reached the compiler (not blocked by
the workspace-wide `target/` lock other concurrent peer sessions were holding at the time — confirmed
via `ps aux`: simultaneous builds of `architect`, `lowpoly`, `process`, `space`, `stdio` from other
sessions were running against the same `target/` directory) and both failed, the full run with 4553
errors. `grep -oE '\-\-> [^:]+' | sort -u` over the complete captured log (not just the truncated
tail): **1159 unique file paths total, all** either inside `rustlib/src/rust` (std-internal trace
lines, not real error sites), inside `semio-s-plugin-puzzle`'s own artifact editors/viewers/commands/
panels (`◻2d`/`🖐️5d`/`🧊️3d` standards), or inside `✏️s/🔌️plugins/🗄️stdio`'s `📄️pdf` artifact
editors/viewers across many PDF standard/subset combinations (`1.4`/`1.7` × `a`/`base`/`x`/`e`/`h`/
`ua`/…) — `stdio` is a direct dependency of `puzzle`. **Zero** mentions of `🖼️canvas`, `SceneCommand`,
`RecordedShape`, `kurbo::`, or `peniko::` anywhere in the full log (`grep -c` → 0). A representative
sample of the actual errors makes the root cause unambiguous —
```
error[E0308]: mismatched types
  --> …/🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs:8366:77
    ArtifactReservedToolJob::new(Puzzle5dPasteJob::new(request, args))
                                                                 ^^^^ expected `Option<Value>`, found `Option<DslValue>`
error[E0308]: mismatched types
  --> …/🗿️artifacts/◻2d/…/✏️editor/🦀️.rs:153:53
    let ids = semio_framework_plugin::selection_ids(args);
                                                      ^^^^ expected `Option<&DslValue>`, found `Option<&Value>`
```
— this is exactly the `Value`/`DslValue` **`dsl`-bridge migration** this ticket's own HARD
CONSTRAINTS section names as live, in-progress, repo-wide churn ("a ~41k-path basename rename and a
`dsl`-bridge migration"), not a regression from this pass. Per the ticket's own instruction ("if an
error names a file outside your scope, record it and move on"), this was recorded and not chased
further. `flow`/`trinity`'s equivalent full builds were not reached this session: the shared
`target/` directory stayed under heavy contention from other live sessions for the remainder of the
window available. The `cargo tree -i` evidence above is lock-free, metadata-only, and authoritative
for the crate-count claim regardless of this; `semio-framework-os-infinite`'s own `cargo check`
(native and `--target wasm32-wasip2`, both **0 errors**, confirmed a second time after a
formatting-only `rustfmt` pass with no logic change) is the real guardrail this ticket asked for, and
it is clean.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs` — `Scene` internal
  representation (`backend::Scene` → `Vec<SceneCommand>`), new `RecordedShape`/`SceneCommand` types
  and their `transformed`/`replay_into` impls, `vello_backend` module narrowed (`vello`/`Scene` now
  host-gated re-exports; `kurbo`/`peniko` re-exports removed in favor of direct crate references),
  `SvgDocument::append_to_scene` rewritten around `SceneCommand::VelloFragment`, `gpu_session::
  render_frame`'s and the one other render call site's `vello_scene()` usage updated for its new
  owned return type. No public method signature on `Scene` changed except `vello_scene`'s return
  type (`&backend::Scene` → `backend::Scene`, both call sites updated).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🎯️targets/
  🧊️wgpu/🦀️.rs` — `packet.scene.vello_scene()` call site updated for the owned return type (one
  `let` binding added).
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — `kurbo`/
  `peniko` added as unconditional direct dependencies; `vello` removed from the `wasm32-wasip2`
  target table entirely (stays in the `not(all(wasm32, p2))` table, unchanged).

No file in `✏️s/🔌️plugins/**` was touched — every plugin depends on `semio-framework-os-infinite`
unconditionally already; the crate-count reduction is entirely a consequence of that one crate's
internal narrowing, exactly like the `raster`/`typeset` tier splits before it.

## What is proven vs. not proven, stated plainly

**PROVEN** (lock-free `cargo tree`, cannot go stale): `semio-s-plugin-puzzle`/`trinity` wasip2
third-party count **63 → 43**, `semio-s-plugin-flow` **64 → 44** — a clean 20-crate reduction per
plugin, one better than the predecessor's own estimate; `vello`/`vello_encoding` completely absent
from all three; `kurbo`/`peniko` still present (by design) at the same locked versions as before,
adding no new crate version to the graph; native `vello@0.7.0` resolution for
`semio-framework-os-infinite` unchanged. `cargo check -p semio-framework-os-infinite` (native) and
`cargo check --lib -p semio-framework-os-infinite --target wasm32-wasip2` both `Finished`, 0 errors,
0 warnings from any file this pass touched. `cargo build --lib --target wasm32-wasip2 -p
semio-s-plugin-draw-fsm` (the ticket's own clean baseline) succeeds end-to-end.

**NOT proven end-to-end**: a full `cargo build --lib --target wasm32-wasip2` for
`semio-s-plugin-puzzle` — attempted twice, foreground; both runs reached the compiler and failed
with 4553 errors, all three implicated files inside `puzzle`'s own 5D/3D artifact editors, zero
mentions of any file this pass touched (see Verification above) — consistent with the ticket's own
repeated experience of unrelated live churn blocking full-plugin builds, not a regression from this
pass. `flow`/`trinity`'s equivalent builds were not reached this session (shared `target/` lock
contention from concurrent peer sessions). This does not weaken the crate-count claim (`cargo tree
-i`, lock-free, cannot go stale) or the `semio-framework-os-infinite` guardrail checks (both native
and `wasm32-wasip2`, both 0 errors, both directly exercising every line this pass touched) — but a
reader should not take "63/64/63 → 43/44/43" to mean the three plugins currently build clean
end-to-end; they did not, before this pass either, per `verified-outcomes.md`'s own standing notes
about `stdio`/serde-migration churn.
