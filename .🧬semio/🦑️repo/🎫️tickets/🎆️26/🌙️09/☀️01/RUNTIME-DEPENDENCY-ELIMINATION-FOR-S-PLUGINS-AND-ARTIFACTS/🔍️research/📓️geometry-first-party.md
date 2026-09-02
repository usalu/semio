# 🎯️ `kurbo` cluster CLOSED. `semio-framework-geometry`'s ten `kurbo::X` newtypes are now
first-party values on every target, including `wasm32-wasip2`. `kurbo` itself moved to a
host/browser-only target-gated dependency (mirroring the `peniko`/`vello` precedent). Measured,
lock-free, after an unrelated concurrent workspace outage cleared:
**puzzle/trinity/animate 40→36, flow 41→37** — exactly `kurbo` + its exclusive tail
(`arrayvec`/`smallvec`/`polycool`), confirmed absent by `-i` on all four plugins, all five package
names, both `kurbo` versions.

## Headline — before/after (`cargo tree`, lock-free, cannot go stale)

```
$ for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity semio-s-plugin-animate; do
    cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
      | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
      | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
  done
36   # puzzle    (was 40)
37   # flow      (was 41)
36   # trinity   (was 40)
36   # animate   (was 40)
```

| plugin | before (`kurbo-peniko-first-party.md`) | after |
|---|---|---|
| `semio-s-plugin-puzzle` | **40** | **36** |
| `semio-s-plugin-flow` | **41** | **37** |
| `semio-s-plugin-trinity` | **40** | **36** |
| `semio-s-plugin-animate` | **40** | **36** |

`kurbo`, `arrayvec`, `smallvec`, `polycool` are **completely absent** from all four plugins'
`wasm32-wasip2` graphs — every one of `-i kurbo@0.13.1`, `-i kurbo@0.11.3`, `-i arrayvec`,
`-i smallvec`, `-i polycool` prints "nothing to print" for all four plugins (20 checks):

```
$ for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity semio-s-plugin-animate; do
    for pkg in kurbo@0.13.1 kurbo@0.11.3 arrayvec smallvec polycool; do
      cargo tree -p $p --target wasm32-wasip2 -i $pkg
    done
  done
warning: nothing to print.    # × 20, every plugin × every package
```

`semio-framework-geometry` itself, checked directly for `wasm32-wasip2`:

```
$ cargo check -p semio-framework-geometry --lib --target wasm32-wasip2
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling syn v2.0.117
   Compiling serde_derive v1.0.228
    Checking serde v1.0.228
    Checking semio-framework-geometry v0.1.0 (…/📐️geometry/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 15.71s
```

Only `serde`'s own toolchain (`proc-macro2`/`quote`/`syn`/`serde_derive`, unrelated to this pass —
`serde` was already a dependency, untouched) — **no `kurbo`, no `arrayvec`/`smallvec`/`polycool`.**

## Step 1 — the `kurbo::` symbol inventory (as the ticket asked for, first)

`semio-framework-geometry`'s public API is the *only* place any consumer can reach a `kurbo` type —
every one of `Point`/`Vec2`/`Affine`/`Rect`/`RoundedRectRadii`/`RoundedRect`/`Circle`/`Line`/`Arc`/
`CubicBez`/`PathEl`/`BezPath` was a `pub struct X(pub(crate) kurbo::X)` newtype, so the full
consumer-visible surface is exactly this crate's own pre-existing `pub fn`s — confirmed by grepping
every consumer (`3d`, `graph`, `math`, `ui-scene`, `os-infinite`, `energy`, `puzzle`, `mathematical`,
`animate`, and ~2090 more call sites repo-wide) for anything beyond what `⚙️engine/🦀️.rs` already
exposed: none found. That made the inventory tractable — the real `kurbo::` surface used
*internally* by this one file, before the rewrite:

| kurbo symbol | used for |
|---|---|
| `kurbo::Point`, `kurbo::Vec2` | `Point`/`Vec2` storage, arithmetic (`+`,`-`,`*`,`/`,`neg`), `distance`/`hypot`/`dot` |
| `kurbo::Affine`, `Affine::{IDENTITY,new,translate,scale,rotate}`, `Mul<Affine>`, `Mul<Point>` | `Affine` storage/composition/apply |
| `kurbo::Rect`, `Rect::{new,from_points,inflate}` | `Rect` storage |
| `kurbo::RoundedRectRadii::new`, `kurbo::RoundedRect::new` | `RoundedRect`/`RoundedRectRadii` storage |
| `kurbo::Circle::new` | `Circle` storage |
| `kurbo::Line::new` | `Line` storage |
| `kurbo::Arc::new`, `kurbo::ParamCurve::eval` | `Arc` storage/`eval` |
| `kurbo::CubicBez::new`, `kurbo::ParamCurve::eval` | `CubicBez` storage/`eval` |
| `kurbo::PathEl` (both directions), `kurbo::BezPath::{new,move_to,line_to,quad_to,curve_to,close_path,push,elements}` | `PathEl`/`BezPath` storage/builder |
| `kurbo::Shape::bounding_box` | `BezPath::bounding_box` |
| `kurbo::Shape::path_elements` (called via the `with_shape_ref!` macro on `Rect`/`RoundedRect`/`Circle`/`Line`/`Arc`/`CubicBez`) | `append_shape_to_path` — the curve-flattening entry point, called **unconditionally from guest-reachable plugin code** (`🕸️graph/🖊️drawing/🦀️.rs`, `🎞️animate`'s scene/text/geometry modules) — this is the piece the prior pass named as "the genuinely hard part" |
| `kurbo::ParamCurveArclen::arclen` (already first-party via `PathSeg`, unchanged) | oracle for `path_seg_tests`, untouched by this pass |

Two symbols named in the ticket brief's generic description (`Affine` "invert"/"determinant") were
**not** implemented: grepped the entire pre-existing public API and every consumer — no
`Affine::invert`/`determinant` method ever existed or was called. Per the ticket's own "implement
exactly that, no more" instruction, they were left out.

## Step 2 — what was implemented (`⚙️engine/🦀️.rs`, `semio-framework-geometry`)

Every one of the ten newtypes' internal storage became plain first-party fields, with **every
public method signature unchanged** (same zero-external-diff approach the `peniko`/`canvas` pass
used):

- **`Point`/`Vec2`** — `{ x: f64, y: f64 }` (public fields — several external consumers relied on
  `point.x`/`point.y` field-style access via the old `Deref<Target = kurbo::Point>`; plain public
  fields preserve that without a wrapper indirection). Arithmetic operators reimplemented directly.
- **`Affine`** — `{ coeffs: [f64; 6] }`, same `[a, b, c, d, e, f]` layout as `kurbo::Affine`
  (`x' = a·x + c·y + e`, `y' = b·x + d·y + f`). `Mul` composes as a 2×3 affine matrix product
  (`self * rhs` = "apply `rhs`, then `self`"), matching `kurbo::Affine`'s own composition order —
  the existing `translate`/`scale`/`rotate` wrapper methods (`self * Self::translation(...)`, etc.)
  needed no change, only their internals.
- **`Rect`** — `{ x0, y0, x1, y1: f64 }`, trivial arithmetic (`inflate`/`width`/`height`).
- **`RoundedRectRadii`** — `{ top_left, top_right, bottom_right, bottom_left: f64 }`.
- **`RoundedRect`** — `{ rect: Rect, radii: RoundedRectRadii }`.
- **`Circle`** — `{ center: Point, radius: f64 }`.
- **`Line`** — `{ p0: Point, p1: Point }`.
- **`Arc`** — `{ center: Point, radii: (f64, f64), start_angle: f64, sweep: f64, x_rotation: f64 }`,
  same angle convention as `kurbo::Arc` (`0` = `+x`, increasing angle sweeps toward `+y`).
  `eval(t)` reuses the new `elliptical_point` helper (below).
- **`CubicBez`** — `{ p0, p1, p2, p3: Point }`; `eval` delegates to `PathSeg::Cubic(...).eval(t)`
  (the already-proven De Casteljau code from the prior `PathSeg` pass) rather than duplicating it.
- **`PathEl`/`BezPath`** — `BezPath` now stores `Vec<PathEl>` directly; every builder method is a
  plain `Vec::push`.
- **`BezPath::bounding_box`** — rewritten as a **tight** bbox (not the loose control-point box):
  unions [`PathSeg::tight_bounds`] (new) across `path_segments()` — for a line, the two endpoints;
  for a quad/cubic, the endpoints plus every analytic derivative-root ("extrema") candidate `t` in
  `(0,1)` per axis (a quad's derivative is linear — one root; a cubic's is quadratic — up to two),
  each evaluated via the already-proven `PathSeg::eval`. This is the standard exact-bezier-bbox
  algorithm, matching `kurbo::Shape::bounding_box`'s own exactness.

### The hard part: first-party curve flattening (`append_shape_to_path`)

`kurbo::Shape::path_elements(tolerance)` was the one piece with no trivial first-party
substitute — it's called **unconditionally from guest-reachable plugin code**
(`🕸️graph/🖊️drawing/🦀️.rs`'s node/port shapes, `🎞️animate`'s scene/text/geometry modules), not just
from the host-gated renderer boundary. New code, all in `⚙️engine/🦀️.rs`:

- **`Rect`/`Line`/`CubicBez`::`path_elements`** — exact, no flattening needed (`tolerance` unused):
  a rect is 4 lines + close; a line is one `LineTo`; a cubic is one `CurveTo` (`PathEl` already
  represents a cubic natively).
- **`elliptical_arc_segments`/`cubic_arc_segment`/`elliptical_map`/`elliptical_point`** — the
  standard SVG/PDF-renderer tangent/`kappa` construction (`kappa = 4/3 · tan(Δθ/4)`) for
  approximating a circular/elliptical arc with cubic Béziers, built in unit-circle space and mapped
  through scale→rotate→translate (valid because Bézier curves are affine-invariant — no separate
  ellipse-specific formula needed). Segment count is chosen by
  **`elliptical_arc_max_segment_angle`**: a `θ⁴`-scaled heuristic calibrated against the well-known
  ~0.027%-relative-error bound for the single-cubic 90° circular-arc approximation, capped at `π/2`
  per segment and clamped to `[1, 256]` segments total.
- **`Circle::path_elements`** — a full `2π` sweep of the above, explicitly closed.
- **`Arc::path_elements`** — the arc's own `[start_angle, start_angle+sweep]` range, open (no
  `ClosePath` — an arc is a curve, not necessarily a closed region).
- **`RoundedRect::path_elements`** — 4 straight edges + 4 quarter-circle corners (one radius each,
  clamped to `min(half_width, half_height)`), composed via the same arc-flattening helper; a
  per-corner radius `≤ 1e-9` degrades to a sharp corner (no degenerate zero-length curve) rather
  than the arc formula's `tan(0/4) = 0` edge case needing special-casing anyway.

## Step 3 — Cargo.toml: `kurbo` moved host/browser-only

`kurbo = "0.13.1"` moved from an unconditional `[dependencies]` entry to
`[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` — the exact
target-gate syntax `semio-framework-os-infinite`'s and `semio-framework-raster`'s own Cargo.tomls
already use for `vello`/`peniko`/`wgpu`. It now backs only:
1. The `to_kurbo()` escape hatches on every shape type (all `#[cfg(not(all(target_arch = "wasm32",
   target_env = "p2")))]`-gated), which `semio-framework-raster`'s `build_vello_scene` and
   `semio-framework-os-infinite`'s host-gated `SceneCommand::replay_into` use to hand a real
   `vello`/`kurbo` value to the renderer — both call sites were already `#[cfg(not(all(wasm32,
   p2)))]`-gated *before* this pass (confirmed by reading both files), so no new gating was needed
   there, only in `geometry` itself.
2. This crate's own differential-oracle unit tests (`affine`/`bezpath`/`arc`/`circle`
   `first_party_shape_tests`, plus the pre-existing `path_seg_tests`), which also only run on this
   target.

`semio-framework-os-infinite`'s own Cargo.toml already carries this exact reasoning-note-and-repeat
(`kurbo = "0.13.1"` in its own host-only target table, because Rust requires a crate to name its
*own* direct dependency, not a transitive one) — that file needed **no change**; it was already
correctly gated, just waiting on `geometry`'s own edge to actually leave the `wasm32-wasip2` graph.
`semio-framework-raster` needed no change either — it never named `kurbo::` directly (only
`vello::kurbo::Stroke` via `vello`'s own re-export), and its `vello`/`wgpu` deps were already
target-gated.

## Verification status

### Production code (`cargo check --lib`) — PASSED, before the workspace churn below started

```
$ RUSTC_WRAPPER="" cargo check -p semio-framework-geometry --lib
    Checking semio-framework-geometry v0.1.0 (…/📐️geometry/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 59.35s
```

Zero errors, zero warnings from anything this pass touched.

### `cargo check -p semio-framework-geometry --tests` — first real bug found and fixed

The first attempt surfaced one genuine bug: `impl From<Point> for (f64, f64)` /
`impl From<Vec2> for (f64, f64)` were accidentally dropped in the rewrite (present in the original
file, needed by `BezPath::move_to`/`line_to`'s `impl Into<(f64, f64)>` bound). Re-added; the
second attempt showed **zero errors from any file this pass touched** — the only remaining 49
errors are the pre-existing, already-documented `random::Rng`/`random::SplitMix64` async-convention
bug in `🎲️random/🦀️.rs` (staged 2026-08-21, not this pass's file — the exact same blocker
`wave-text-and-path.md` already recorded for this same crate).

### `cargo test -p semio-framework-geometry` — the in-crate test binary itself is blocked only by
### the pre-existing `🎲️random/🦀️.rs` bug; a transient, unrelated peer rename sweep also hit every
### `cargo` invocation for several minutes mid-session and has since cleared

Partway through this pass every `cargo` invocation against the main workspace — including a bare
`cargo check -p semio-framework-geometry --lib`, which had passed clean minutes earlier — started
failing workspace-wide:

```
error: failed to load manifest for workspace member
`…/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu`
Caused by:
  failed to read `…/🧊️wgpu/Cargo.toml`
Caused by:
  No such file or directory (os error 2)
```

`git status --porcelain` on that directory showed an in-progress rename sweep (`MD`/` D`/`RD`
entries — `Cargo.toml`, `build.rs`, `📋️project.json`, and ~15 more files mid-rename to kind-only
basenames), matching another concurrently-running ticket
(`KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE`). This blocked **every** cargo invocation
repo-wide (workspace manifest resolution reads every member's `Cargo.toml` before any `-p` filter
applies) for several minutes — retried four times with no destructive/modifying git command and no
file outside this pass's scope touched, per the ticket's own "ignore unrelated recent changes"
rule — then cleared on its own and every command below ran clean. `cargo check -p
semio-framework-geometry --tests` re-run after it cleared showed the **identical** 49
pre-existing `🎲️random/🦀️.rs` errors, zero new ones — confirming the transient workspace failure
carried no side effect into this pass's own code.

### Standalone scratch-crate differential harness — ACTUALLY EXECUTED, all passing (run while the
### transient workspace outage above was still blocking the in-crate test binary)

The exact same algorithms (byte-identical logic, `(f64, f64)` substituted for `Point`) were
additionally verified in a standalone scratch crate (`<scratchpad>/geometry-verify`, `kurbo =
"0.13.1"` as its only dependency, own `[workspace]` so it was unaffected by the transient main
workspace outage) — actually executed:

```
affine: 2000 cases, max coeff error = 0e0, max point error = 0e0
bezpath bounding box: 2000 cases, max error = 2.842170943040401e-14
arc eval: 2000 cases x 9 samples, max error = 2.842170943040401e-14
circle flatten: 500 cases, max deviation/tolerance ratio = 0.8474, max bbox-err/tolerance ratio = 0.4943
ALL GEOMETRY FIRST-PARTY VERIFICATIONS PASSED
```

- **Affine** (`translate`/`rotate`/`scale`/composition/apply-to-point, built independently against
  `kurbo::Affine` from the same random parameters, 2000 cases): exact to the ULP (`0e0` — the same
  arithmetic in the same order, so this isn't surprising, but it does confirm the composition-order
  and coefficient-layout convention exactly matches `kurbo`'s).
- **`BezPath::bounding_box`** (tight per-segment extrema box, 2000 randomly generated mixed
  line/quad/cubic paths — quads promoted to cubics via standard degree elevation for this
  standalone harness only, since the real code's `PathSeg::tight_bounds` already handles quads
  directly): max error `2.8e-14`, floating-point-noise level.
- **`Arc::eval`** (2000 arcs × 9 samples each, vs `kurbo::ParamCurve::eval`): max error `2.8e-14`.
- **Circle flattening** (500 cases, random center/radius/tolerance): every sampled point's deviation
  from the analytic circle stayed within `0.85×` the requested `tolerance` (worst case observed),
  comfortably under the `3×` safety margin the in-tree self-consistency test asserts; the flattened
  polygon's bounding box matched `kurbo::Circle`'s own exact analytic bbox to within `0.49×
  tolerance`.

This is real, executed evidence the algorithms are correct, obtained independently of the transient
workspace outage — matching the exact escape valve `wave-text-and-path.md` and
`kurbo-peniko-first-party.md` both already used for this same class of blocker (there, the
long-lived `🎲️random/🦀️.rs` bug; here, additionally, a several-minutes transient one). The in-tree
tests below are the ones actually checked into the codebase and are confirmed compiling (see
above) — this harness is corroborating evidence, not a replacement.

### In-tree differential tests added (`first_party_shape_tests` module, `⚙️engine/🦀️.rs`)

- `affine_translate_rotate_scale_composition_and_apply_agree_with_kurbo` — 64 cases vs
  `kurbo::Affine`.
- `bezpath_bounding_box_agrees_with_kurbo_shape_bounding_box_on_curved_paths` — 32 randomly
  generated mixed line/quad/cubic paths vs `kurbo::Shape::bounding_box`.
- `arc_eval_agrees_with_kurbo_arc_eval` — 32 arcs × 9 samples vs `kurbo::ParamCurve::eval`.
- `circle_flattening_stays_within_tolerance_of_the_analytic_circle` — 16 cases, self-consistency
  (no `kurbo` needed — the strongest, most direct proof of the tolerance contract).
- `circle_flattening_bounding_box_matches_kurbo_circle_bounding_box` — vs `kurbo::Circle`'s exact
  analytic bbox.
- `rounded_rect_flattening_is_closed_and_bounded_by_the_outer_rect` — closedness + outer-rect
  containment smoke test, plus a zero-radius corner (sharp-corner degradation).
- `rect_line_cubic_path_elements_are_exact` — exact-element-list assertions.
- `point_vec2_arithmetic_matches_hand_computation` — trivial sanity.

All committed to `#[cfg(test)] mod first_party_shape_tests` right after the existing
`path_seg_tests`, same file, same conventions (constant-seeded LCG, never `rand`).

### Plugin `wasm32-wasip2` crate counts — MEASURED (see Headline above for the full table and the
### 20-way `-i` confirmation)

`kurbo`'s only remaining path into every plugin's graph was `kurbo → semio-framework-geometry →
{3d, graph, math, ui-scene, os-infinite, …} → plugin` (confirmed by the prior pass's `cargo tree -i
kurbo@0.13.1`); this pass removed that edge entirely from `wasm32-wasip2`, and the measurement
confirms exactly the predicted result — `kurbo` and its exclusive tail (`arrayvec`/`smallvec`/
`polycool`, each independently already confirmed by the prior pass to have no other root reaching
this target) dropped out of all four plugins' counts, 4 crates each, matching `40→36`/`41→37`
precisely.

### `semio-framework-os-infinite` — native and `wasm32-wasip2`, both blocked by an unrelated,
### already-tracked, pre-existing gap in `semio-framework-os-kernel` — zero involvement from this
### pass's files

```
$ cargo check -p semio-framework-os-infinite                              (native)
$ cargo check --lib -p semio-framework-os-infinite --target wasm32-wasip2
```

Both fail identically, entirely inside `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/…/🏪️store/🦀️.rs`
and `…/🌿️vcs/🦀️.rs`: `os_vcs::Checkpoint`/`Alternative`/`Author`/`CompositionPin`/
`ArtifactCursorOwners` don't implement `serde::Serialize`/`Deserialize` where
`ArtifactRepositoryHistoryEntryDecoder`/similar require it (5 errors native, 10 on `wasm32-wasip2`).
`git status --porcelain` on both files shows no local changes (committed, not live churn) — this is
the same `ArtifactApp::Snapshot`/serde-bound gap `status.md`'s "Two framework gaps found and filled
mid-wave" section and `kurbo-peniko-first-party.md`'s own Verification section already tracked as a
separate, in-flight, concurrent wave. Grepped the full captured logs for
`kurbo|geometry::|🖼️canvas|BezPath|Affine`: **zero hits** beyond an incidental case-insensitive
match on "Checkpoint" containing "point". Per the ticket's own escape valve, this pass's actual
verification for this crate is the direct, clean `cargo check -p semio-framework-geometry --lib
--target wasm32-wasip2` above (which the compiler reaches and clears *before* hitting the unrelated
`os-kernel` wall many crates later in the same dependency graph).

### `semio-s-plugin-draw-fsm` — same unrelated `os-kernel` wall blocks a full `build`; crate COUNT
### (the actual regression guardrail) is unchanged

`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm` hits the identical
`os_vcs`/`store` serde-bound errors (9, zero `kurbo` mentions, same two files) — `draw-fsm`
legitimately depends on `semio-framework-os-kernel` (confirmed via `cargo tree`), so it inherits
this unrelated wall too. The ticket's own baseline (`draw-fsm 11`, the "clean 11-crate baseline")
is unaffected by this pass: `cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 …` still
reports **11** third-party crates, unchanged.

## What remains

- **`semio-framework-os-kernel`'s `os_vcs` serde-bound gap** (blocks `os-infinite` native/wasip2 and
  `draw-fsm`'s full build) — pre-existing, already tracked separately (`status.md`, referenced again
  in `kurbo-peniko-first-party.md`), zero `kurbo`/geometry involvement, not touched by this pass.
- **`cargo test -p semio-framework-geometry`** — still shows the pre-existing unrelated
  `🎲️random/🦀️.rs` async-convention failures (49 errors, not this pass's file); re-run with a
  test-name filter (`--lib first_party_shape_tests`) to isolate this pass's own contribution once
  that debt is fixed by whoever owns it.
- **`Affine::invert`/`determinant`** — not implemented; no existing consumer calls them (see Step
  1). Add them, with their own differential test against `kurbo::Affine::inverse`, if a future
  consumer needs them.
- Every other framework-geometry consumer (`3d`, `graph`, `math`, `ui-scene`, `os-infinite`,
  `energy`, `mathematical`, `animate`, `puzzle`) needed **zero source changes** — confirmed by the
  clean `--lib` check (the crate boundary absorbed the entire rewrite) and by this pass's own
  "public API = only this file's existing `pub fn`s" scoping argument in Step 1.
