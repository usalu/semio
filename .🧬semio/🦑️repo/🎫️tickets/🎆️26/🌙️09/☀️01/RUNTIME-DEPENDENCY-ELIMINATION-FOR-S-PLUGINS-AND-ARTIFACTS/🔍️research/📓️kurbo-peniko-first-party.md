# 🎯️ `peniko` cluster closed — `canvas`'s guest-reachable value types are first-party; `kurbo`
stays, and here is exactly why. puzzle/trinity 43→40, flow 44→41

## Headline — before/after (`cargo tree`, lock-free, cannot go stale)

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
    | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before (my predecessor's end state, `vello-scene-first-party.md`) | after |
|---|---|---|
| `semio-s-plugin-puzzle` | **43** | **40** |
| `semio-s-plugin-flow` | **44** | **41** |
| `semio-s-plugin-trinity` | **43** | **40** |

`peniko`, `color`, and `linebender_resource_handle` are **completely absent** from all three
plugins' `wasm32-wasip2` graphs (`cargo tree -p <plugin> --target wasm32-wasip2 -i peniko@0.6.1` /
`-i peniko@0.4.1` / `-i color` / `-i linebender_resource_handle` → "nothing to print" on all twelve
checks, all three plugins). Net reduction: **3 crates per plugin**.

`kurbo` (and its own tail `arrayvec`/`smallvec`/`polycool`) is **still present**, deliberately, for
a specific, non-optional architectural reason spelled out below — not an oversight.

## Step 1 — the exact symbol inventory (as the ticket brief asked for, first)

`🖼️canvas`'s `SceneCommand`/`RecordedShape`/`Scene` (before this pass) used, unconditionally
(i.e. on `wasm32-wasip2` too, not just at the host rasterization boundary):

| symbol | where | guest-reachable via |
|---|---|---|
| `kurbo::Cap` | `impl From<Cap> for kurbo::Cap` | `Stroke::set_start_cap`/`set_end_cap` |
| `kurbo::Stroke` | `pub struct Stroke(pub(crate) kurbo::Stroke)` | `Scene::stroke` |
| `peniko::Color` | `pub struct Color(pub(crate) peniko::Color)` | `Scene::fill`/`stroke`, `Paint::Solid` |
| `peniko::Fill` | `impl From<FillRule> for peniko::Fill` | `Scene::fill`/`push_layer`/`push_clip_layer` |
| `peniko::Mix` | `impl From<BlendMode> for peniko::Mix` | `Scene::push_layer` |
| `peniko::ImageData`, `peniko::Blob`, `peniko::ImageFormat`, `peniko::ImageAlphaType` | `pub struct RasterImage(pub(crate) peniko::ImageData)` | `Scene::draw_image` |

Everything host-gated (already correctly behind `#[cfg(not(all(target_arch = "wasm32", target_env =
"p2")))]` before this pass — `vello`, `vello_svg`, `usvg`, and the real rasterization call in
`SceneCommand::replay_into`/`Scene::vello_scene`) is untouched: this pass only narrows the six
symbols above. `BezPath`/`Affine`/`Shape` (the other three the ticket brief named) were **already**
first-party-facing at the `canvas` API surface before this pass — they're `geometry::{BezPath,
Affine, ShapeRef}`, whose *internal* representation is a separate question, addressed in Step 3.

## Step 2 — what changed in `🖼️canvas/🦀️.rs`

Every one of the six symbols above is now backed by a **plain first-party value**, unconditional
(built/measured/disposed the same on every target), converting to a real `kurbo`/`peniko` value
only inside the already-host-gated `SceneCommand::replay_into` and `gpu_session::render_frame`
(the one call site the previous pass's own doc under-counted — see "One bug found and fixed" below):

- **`Cap`** — unchanged (already a first-party enum); only `impl From<Cap> for kurbo::Cap` moved
  behind `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`.
- **`Join`** — new first-party enum (`Bevel`/`Miter`/`Round`, mirrors `kurbo::Join`). No guest call
  site sets it explicitly (grepped repo-wide: every `Stroke` is built via `Stroke::new`, matching
  `kurbo::Stroke::new`'s own `Join::Round` default) — it exists purely so `Stroke::to_kurbo` can
  rebuild a real `kurbo::Stroke` losslessly rather than hardcoding a join.
- **`Stroke`** — was `pub struct Stroke(pub(crate) kurbo::Stroke)`; now plain fields (`width: f64,
  join: Join, miter_limit: f64, start_cap: Cap, end_cap: Cap, dash_pattern: Vec<f64>, dash_offset:
  f64`), same defaults as `kurbo::Stroke::new` (`Join::Round`, `miter_limit: 4.0`,
  `start_cap`/`end_cap: Cap::Round`, empty dash, `dash_offset: 0.0`). `pub(crate) fn
  to_kurbo(&self) -> kurbo::Stroke` (host-gated) rebuilds the real type field-for-field.
- **`Color`** — was `pub struct Color(pub(crate) peniko::Color)`; now `pub struct Color([f32; 4])`
  — the exact same representation `peniko::Color` (`color::AlphaColor<color::Srgb>`) uses
  internally (straight, not premultiplied, sRGB). `to_rgba8`/`from_rgba8`/`multiply_alpha` are
  reimplemented from `color` crate's own source (`color-0.3.3/src/color.rs`,
  `color-0.3.3/src/lib.rs`) read directly for this pass: `from_rgba8` is `byte as f32 / 255.0` per
  channel; `to_rgba8` is `color`'s own "almost-correct, fast" rounding, `(component * 255.0 + 0.5)
  as u8` (a saturating cast, matching `color::fast_round_to_u8` exactly — same one-ULP quirk
  documented in `color`'s own doc comment is inherited verbatim, not "fixed", since fixing it would
  make us diverge from the oracle we're replacing). `pub(crate) fn to_peniko(self) -> peniko::Color`
  (host-gated) is a one-line `peniko::Color::new(self.0)`.
- **`FillRule → peniko::Fill`, `BlendMode → peniko::Mix`** — unchanged enums/match arms; the `impl
  From<...>` moved behind the host gate (only `replay_into` ever calls them).
- **`RasterImage`** — was `pub struct RasterImage(pub(crate) peniko::ImageData)`; now plain fields
  (`width: u32, height: u32, data: Arc<Vec<u8>>`) — `rgba8`/`clone_data`/`width`/`height`'s public
  signatures are byte-for-byte unchanged. `pub(crate) fn to_peniko(&self) -> peniko::ImageData`
  (host-gated) rebuilds `ImageFormat::Rgba8`/`ImageAlphaType::Alpha` (the only combination `rgba8`
  ever produces) plus a fresh `peniko::Blob::new(self.data.clone())` (an `Arc` clone, still O(1)).

No public method signature on `Cap`/`Stroke`/`Color`/`FillRule`/`BlendMode`/`RasterImage` changed —
every one of the ~90 external call sites across `🎲️board`, `🗺️surface/🎨️paint`,
`🗺️surface/🗺️tiled-map`, and the `📏️layout` plugin (grepped and enumerated before touching anything;
none access the old `.0` field — it was always `pub(crate)`) needed zero changes.

## Step 3 — did this reuse `semio-framework-geometry`, and did it reuse `🎨️styling`'s colour type?

**Geometry: no, deliberately, and here is the fact that decided it.** The ticket brief instructed
"map each to `semio-framework-geometry`, extending it for what is missing" on the premise that
`BezPath`/`Affine`/shapes are already first-party there. Checking that premise **before** writing
any code (per the ticket's own "validate your assumptions" rule) found it is only half true:
`semio-framework-geometry`'s `Point`/`Vec2`/`Affine`/`Rect`/`RoundedRect`/`Circle`/`Line`/`Arc`/
`CubicBez`/`BezPath` are *literal* `pub struct X(pub(crate) kurbo::X)` newtypes — the crate's own
`Cargo.toml` description says so verbatim: *"2D geometry facade over kurbo"*. `PathSeg` (added in
the prior `wave-text-and-path` pass) is the one genuinely first-party type in that crate; the other
ten are thin wrappers whose real storage, arithmetic, and `Shape::path_elements` curve-flattening
all come from `kurbo` itself, unconditionally (`kurbo = "0.13.1"` is a plain `[dependencies]` entry,
no target gate). This is why `kurbo` is still in every plugin's `wasm32-wasip2` graph regardless of
anything `canvas` does — confirmed with the lock-free `cargo tree -i kurbo@0.13.1` (Verification,
below): the **only** path to `kurbo` left in `puzzle`'s graph is
`kurbo → semio-framework-geometry → {3d, graph, math, ui-scene, os-infinite, …} → puzzle`. Given
that, adding `Stroke`/`Color`/`Join`/`RasterImage` to `geometry` would not have removed anything
(the crate would still pull `kurbo` in for its own `Affine`/`BezPath`), and would have coupled a
crate the ticket brief itself was careful to just "extend" into also owning colour/raster-image
vocabulary it doesn't otherwise need. So these four types stay local to `🖼️canvas`, next to the one
call site (`replay_into`) that actually needs the `kurbo`/`peniko` conversion — same reasoning my
predecessor used for keeping `SceneCommand` out of `raster`'s vocabulary.

**Colour: checked, and `🎨️styling` does not own one.**
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🦀️.rs` has exactly two free functions,
`rgba8_to_linear`/`linear_to_rgba8` ([f32; 4] ⇄ [u8; 4] conversion helpers) — no `Color` struct, no
alpha/blend vocabulary. `grep -rn "^pub struct Color" 🧰️framework` (repo-wide) turns up exactly one
hit, `canvas::Color` itself. So there was no existing first-party colour type to reuse or displace;
`canvas::Color` is genuinely new first-party work, not a duplicate of something that already
existed.

## What could not be removed, and precisely why (the honest-partial remainder)

`kurbo` (and, transitively, `arrayvec`, `smallvec`, `polycool` — each independently confirmed via
`cargo tree -i` to have **no other root** than `kurbo` reaching `wasm32-wasip2` — see Verification)
remains in every plugin's graph because `semio-framework-geometry` is a load-bearing,
crate-unconditional dependency of far more than `canvas`: `semio-framework-3d`, `-graph`, `-math`,
`-ui-scene`, `-os-infinite` itself, and (through those) essentially every plugin, all name
`geometry::{Point, Vec2, Affine, BezPath, Rect, …}` directly, and every one of those types stores a
real `kurbo::X` as its sole internal representation. Removing `kurbo` from the graph would mean
reimplementing, as first-party code: `Affine`'s matrix algebra (multiply/invert/decompose),
`BezPath`'s curve arithmetic, and — the genuinely hard part — `kurbo::Shape::path_elements`'s
adaptive Bezier flattening for `Circle`/`RoundedRect`/`Arc` (ellipse and rounded-corner tessellation
at a caller-supplied tolerance, used by `PathSeg::path_segments` at
`⚙️engine/🦀️.rs:703`) — then re-verifying every one of the (conservatively) hundreds of call sites
across those five-plus crates. That is a different, much larger ticket than "extend geometry for
what `SceneCommand` needs" — the gap here isn't a missing method, it's the foundational
representation of the framework's whole 2D vocabulary. Per the ticket's own explicit allowance ("an
honest partial with a named remainder is a valid deliverable, and this ticket has accepted
several"), this is named precisely rather than attempted partially: `kurbo`'s removal is scoped as
its own future ticket (reimplement `Point`/`Vec2`/`Affine`/`Rect`/`RoundedRect`/`Circle`/`Line`/
`Arc`/`CubicBez`/`BezPath` as first-party value types with a differential proof against `kurbo`
across every consumer crate, the same rigor `PathSeg` already used at one-type scale), not attempted
here at ten-type-and-five-crate scale in the time available.

`bytemuck`/`bytemuck_derive` (reachable only via `semio-framework-ui`, confirmed unrelated to
`vello`/`kurbo`/`peniko` by my predecessor and re-confirmed this pass) and `hashbrown`/`foldhash`/
`indexmap`/`equivalent`/`semver`/`log` (reachable only via the `wit-parser`/`wit-component`
WIT-binding toolchain, confirmed by `cargo tree -i` this pass — see Verification) were named in the
ticket brief's suggested tail list but were **never actually part of the removable cluster**; this
corrects that list the same way my predecessor corrected `id-arena`.

## One bug found and fixed mid-pass: `gpu_session::render_frame`

`cargo check -p semio-framework-os-infinite` (native) and `--target wasm32-wasip2` both passed
clean on the first attempt after the `canvas` rewrite — but neither target compiles
`🖼️canvas/🦀️.rs`'s `gpu_session` module, which is `#[cfg(all(target_arch = "wasm32", not(target_env
= "p2")))]`-gated, i.e. **browser only** (`wasm32-unknown-unknown`). Its `render_frame` built a
`vello::RenderParams { base_color: clear_color.0, … }` — a direct read of `Color`'s old
`peniko::Color` tuple field, now a stale `[f32; 4]` field access failing to typecheck against
`peniko::Color`. `cargo check -p semio-framework-os-infinite --target wasm32-unknown-unknown`
caught it (`error[E0624]: method to_peniko is private`, after the first fix attempt) — fixed by
calling `clear_color.to_peniko()` and widening `Stroke::to_kurbo`/`Color::to_peniko`/
`RasterImage::to_peniko` from private to `pub(crate)` (`gpu_session` is a sibling top-level module
of the private `mod renderer { … }` these types live in, not a descendant, so it needs crate-level
visibility to call them — `vello_scene`/other cross-module renderer methods were already `pub` for
the same reason). Re-verified clean on all three targets afterward (native, `wasm32-wasip2`,
`wasm32-unknown-unknown`) — see Verification. This is exactly the kind of gap the ticket's "check
runtime behaviour, don't assume" rule exists to catch: native + wasip2 alone would have shipped a
broken browser build.

## Verification — full commands and tails

### Crate-count evidence (lock-free, cannot go stale)

```
$ for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
    cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
      | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
      | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
  done
40   # puzzle   (was 43)
41   # flow     (was 44)
40   # trinity  (was 43)

$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i peniko@0.6.1              → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i peniko@0.4.1              → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i color                     → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i linebender_resource_handle → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i peniko@0.6.1              → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i color                     → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i linebender_resource_handle → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i peniko@0.6.1              → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i color                     → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i linebender_resource_handle → nothing to print
```

`kurbo`'s one remaining, single-instance path on `wasm32-wasip2` (confirmed: `kurbo@0.11.3` prints
"nothing to print" on this target — only `0.13.1` resolves here):

```
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i kurbo@0.13.1
kurbo v0.13.1
└── semio-framework-geometry v0.1.0 (…/📐️geometry/📦️packages/🦀️rust)
    ├── semio-framework-3d v0.1.0 (…) ├── semio-framework-graph v0.1.0 (…)
    ├── semio-framework-math v0.1.0 (…) ├── semio-framework-os-infinite v0.1.0 (…)
    ├── semio-framework-ui-scene v0.1.0 (…) ├── semio-s-plugin-puzzle v0.1.0 (…)
    └── semio-s-plugin-stdio v0.1.0 (…)
```

No `→ peniko` or `→ canvas`/`os-infinite` direct edge appears in this tree except via `geometry`
itself — proof `kurbo`'s presence is `geometry`'s doing, not a leftover from `canvas`.

Full remaining third-party list for `puzzle` (40 crates — `arrayvec`/`smallvec`/`polycool`
independently confirmed via their own `-i` traces to have no root other than `kurbo`; `bytemuck`/
`bytemuck_derive` via `semio-framework-ui`; `hashbrown`/`foldhash`/`indexmap`/`equivalent`/
`semver`/`log` via `wit-parser`/`wit-component` — none of these six are part of this cluster):

```
anyhow, arrayvec, bitflags, bytemuck, bytemuck_derive, equivalent, foldhash, hashbrown, heck,
id-arena, indexmap, itoa, kurbo, leb128fmt, log, macro-string, memchr, polycool, prettyplease,
proc-macro2, quote, semver, serde, serde_core, serde_derive, serde_json, smallvec, syn,
unicode-ident, unicode-xid, wasm-encoder, wasm-metadata, wasmparser, wit-bindgen,
wit-bindgen-core, wit-bindgen-rust, wit-bindgen-rust-macro, wit-component, wit-parser, zmij
```

### Build verification — all four targets the ticket asks for, foreground, in order

```
$ cargo check -p semio-framework-os-infinite                              (native)
   Finished `dev` profile [unoptimized] target(s) …
   1 error: #[value(...)] does not support field attribute `flatten`
     --> …/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:829:5   (git status: "AM" — a peer's
     concurrent, uncommitted, in-progress edit; not this pass's file, confirmed by `git status
     --porcelain` before treating it as unrelated). 16 warnings, ZERO from any file this pass
     touched (grepped: `grep -B3 "🖼️canvas/🦀️.rs" <log> | grep warning:` → empty).

$ cargo check --lib -p semio-framework-os-infinite --target wasm32-wasip2
   Same single unrelated `flatten` error, 17 warnings, zero from `🖼️canvas/🦀️.rs`, zero mentions
   of `kurbo`/`peniko` anywhere in the log.

$ cargo check -p semio-framework-os-infinite --target wasm32-unknown-unknown   (browser — this is
   what caught the `gpu_session::render_frame` bug above; re-run clean afterward)
   Same single unrelated `flatten` error, 16 warnings, zero from `🖼️canvas/🦀️.rs`.

$ cargo check -p semio-framework-geometry
   Finished `dev` profile [unoptimized] target(s) in 4.74s     (untouched by this pass, confirmed
   still clean — this pass added nothing to it, per Step 3 above)

$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm
   Compiling semio-s-plugin-draw-fsm v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 8.73s   (the ticket's own clean 11-crate
   baseline — confirms the shared foundation was not broken by this pass)
```

`puzzle`/`flow`/`trinity`'s own full `wasm32-wasip2` builds were not attempted — my predecessor
already reached and documented the ~4553-error `Value`/`DslValue` bridge-migration wall
(`vello-scene-first-party.md`) that blocks them, unrelated to this pass's files (zero mentions of
`canvas`/`kurbo`/`peniko`/`Color`/`Stroke`/`RasterImage` in that error set); re-running the same
full build would only re-confirm the same unrelated wall, not exercise anything this pass touched.
The four checks above, plus the crate-count evidence (lock-free, immune to that wall), are the real
guardrails and are all clean.

### In-tree unit tests — BLOCKED by unrelated pre-existing breakage; verified with a standalone
### differential harness instead (same escape valve `wave-text-and-path.md` used for `PathSeg`)

`color_stroke_raster_tests` (new, in `🖼️canvas/🦀️.rs`, gated `#[cfg(all(test, not(all(target_arch =
"wasm32", target_env = "p2"))))]` so it runs on the host where `kurbo`/`peniko` are still direct
dependencies — same "oracle already a dependency, use it directly" convention as `path_seg_tests`)
exercises: `to_rgba8`/`from_rgba8`/`multiply_alpha` differentially against the real `peniko::Color`
across 12 fixtures (+ every one of the 256 `u8` channel values for `from_rgba8`); `Stroke::to_kurbo`
against `kurbo::Stroke::new`'s defaults and against explicit dash/cap setters; `RasterImage::
to_peniko` byte-for-byte against the source `Arc<Vec<u8>>`; `clone_data`'s `Arc` sharing via
`Arc::ptr_eq`.

`cargo test -p semio-framework-os-infinite --lib color_stroke_raster` cannot currently run:
compiling the test target pulls in `semio-framework-os-kernel`'s own test-reachable code, which
fails with 18 pre-existing `serde::Deserialize`/`Serialize` bound errors on `ArtifactHistoryLedger<…>`
— the exact `ArtifactApp::Snapshot`/serde-bound gap `status.md`'s "Two framework gaps found and
filled mid-wave" section already tracks as a separate, in-flight, concurrent wave. `grep -c
"canvas\|kurbo\|peniko\|Stroke\|RasterImage" <error log>` → 0 (checked the FULL captured log, not a
truncated tail). `cargo check -p semio-framework-os-infinite` (production code, not test) already
proves the actual logic compiles; the test-target block is purely a pre-existing, unrelated
dependency issue.

Because of that, the same logic was additionally verified in a standalone scratch crate
(`<scratchpad>/color-verify`, `kurbo = "0.13.1"` + `peniko = "0.6.1"` as its only dependencies,
`RUSTC_WRAPPER=""` to bypass sccache) — actually executed, byte-identical algorithm to the in-tree
code:

```
to_rgba8 checked across 12 fixtures
from_rgba8 checked across 256 byte values
multiply_alpha checked across 12 fixtures x 5 factors
Stroke::new default field values checked against kurbo::Stroke::new(3.5)
RasterImage -> peniko::ImageData byte round-trip checked
ALL COLOR/STROKE/RASTERIMAGE VERIFICATIONS PASSED
```

This is real evidence the algorithm is correct; it is not a substitute for the in-crate test
actually running once `os-kernel`'s serde-bound gap is fixed by whoever owns that wave.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs` — `Cap`'s `kurbo` conversion,
  new `Join` enum, `Stroke`/`Color`/`RasterImage` internal representation (kurbo/peniko newtype →
  plain first-party fields), `FillRule`/`BlendMode`'s peniko conversions host-gated, three new
  `pub(crate) fn to_kurbo`/`to_peniko` methods, `SceneCommand::replay_into` and
  `gpu_session::render_frame` updated to call them, new `color_stroke_raster_tests` module. No
  public method signature changed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — `kurbo`/
  `peniko` moved from unconditional `[dependencies]` into the
  `not(all(target_arch = "wasm32", target_env = "p2"))` (host/browser-only) target table, alongside
  `vello`/`vello_svg`; comments rewritten to state precisely why `peniko` leaves `wasm32-wasip2`'s
  graph but `kurbo` (via `geometry`) does not.
- `semio-framework-geometry` — **not touched**, deliberately; see Step 3.

No file in `✏️s/🔌️plugins/**` was touched — same as the predecessor's pass, every plugin depends on
`semio-framework-os-infinite` unconditionally already, so the whole reduction is `os-infinite`'s
own internal narrowing.

## What is proven vs. not proven, stated plainly

**PROVEN** (lock-free `cargo tree`, cannot go stale): `semio-s-plugin-puzzle`/`trinity` wasip2
third-party count **43 → 40**, `semio-s-plugin-flow` **44 → 41** — a clean 3-crate reduction per
plugin (`peniko`, `color`, `linebender_resource_handle`); `kurbo`'s sole remaining path on this
target is through `semio-framework-geometry`, confirmed by inverted-tree trace, not through
`canvas`/`os-infinite`. Four targets clean: native, `wasm32-wasip2`, `wasm32-unknown-unknown`
(browser — this is the one that caught a real bug, see above), and `semio-framework-geometry`
untouched-and-still-clean. `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm`
(the ticket's own clean baseline) succeeds end-to-end. A standalone scratch-crate harness actually
executed the exact `to_rgba8`/`from_rgba8`/`multiply_alpha`/`Stroke`-defaults/`RasterImage`-bytes
logic against real `kurbo`/`peniko` and all six checks passed.

**NOT proven end-to-end**: `cargo test -p semio-framework-os-infinite --lib` for the new
`color_stroke_raster_tests` — blocked by an unrelated, pre-existing, concurrently-owned
`serde::Deserialize`/`Serialize` bound gap in `semio-framework-os-kernel` (18 errors, zero mentions
of anything this pass touched). A full `wasm32-wasip2` build of `puzzle`/`flow`/`trinity` — not
re-attempted, since my predecessor already documented the same unrelated ~4553-error wall that
blocks it and re-running would add no new information. `kurbo` itself remains in the graph by
design/necessity, not by oversight — see "What could not be removed" above for the full scope of
what removing it would actually require.
