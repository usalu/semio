# 🎯️ `♾️infinite` host-deps split for puzzle/flow — one real driver closed, one large driver traced and correctly left alone

## Headline — before/after

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before (ticket brief) | after (this pass) |
|---|---|---|
| `semio-s-plugin-puzzle` | **104** | **104** (unchanged — see "What was investigated and NOT changed") |
| `semio-s-plugin-flow` | **117** | **104** |

Flow's tree is now byte-identical to puzzle's (`diff` of the sorted crate-name lists is empty). The
13 crates removed from flow (`allocator-api2`, `displaydoc`, `fontique`, `grid`, `icu_locid`,
`litemap`, `parley`, `swash`, `taffy`, `tinystr`, `writeable`, `yazi`, `zeno`) were **not** the
drivers the ticket brief named (`rustybuzz`/`resvg`/`usvg`/`image`/`taffy` via `ui-render`) — they
were a *different*, previously-unnoticed bug: an unconditional Cargo.toml dependency edge, the exact
"one-line win" class the brief flagged for `animate`'s `semio-framework-os`. `taffy` itself is now
confirmed **absent** from both plugins' wasip2 trees (`cargo tree -i taffy` → "nothing to print" on
both) — the brief's suspicion about `ui-render`'s `taffy` was correct in kind, just reached flow via
a crate the brief didn't name.

## Fix 1 (real, verified): `semio-framework-os-flow` linked the browser WebGPU backend unconditionally

### The bug

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` declared:

```toml
ui_webgpu = { path = "...🖼️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust", package = "semio-framework-ui-backend-webgpu" }
```

in the **unconditional** `[dependencies]` table. `semio-framework-ui-backend-webgpu`'s own
docstring says exactly what it is: *"Browser WebGPU backend (wgpu) — the ONLY place wgpu is
permitted in this repo… This crate is never embedded inside a shipped plugin component."* Its own
Cargo.toml correctly gates the real `wgpu` crate to
`[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]` — but that
crate-internal gate can't undo the fact that `os-flow` pulled the *crate itself* unconditionally,
and `semio-framework-ui-backend-webgpu`'s own unconditional dependency on
`semio-framework-ui-render` drags `taffy`/`parley`/`swash`/`fontique` (and their transitive tail:
`allocator-api2`, `displaydoc`, `grid`, `icu_locid`, `litemap`, `tinystr`, `writeable`, `yazi`,
`zeno`) onto **every** target `os-flow` builds for, wasip2 included.

Traced with `cargo tree -i`:
```
taffy → semio-framework-ui-render → semio-framework-ui-backend-webgpu → semio-framework-os-flow → semio-s-plugin-flow
```

### Where it's actually used — confirmed dead-for-wasip2, not dead-for-real

`ui_webgpu::` is referenced in exactly one file repo-wide:
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` (mounted as `pub mod
wasm_session;` in `os-flow`'s `📦️glue.rs`, **unconditional**, no `#[cfg]` at all). That file's own
docstring calls itself *"Flow editor owned byte/message bridge and primitive linear-memory
exports"* — the browser wasm-pack SDK entry point (`flow_bridge_allocate`/`_release`/`_send`/
`_poll`/`_begin_close`/`_terminal_is_empty`, all `#[no_mangle] extern "C"`). Confirmed by grep that:

- `semio-s-plugin-flow`'s own guest entry point is `semio_framework_plugin::plugin_exports!(...)` in
  its own `📦️glue.rs` — completely separate from `wasm_session`.
- Nothing anywhere in the repo (`grep -rn "wasm_session::\|flow_bridge_allocate\|FlowBridge"`)
  references this module except the module itself. (Two unrelated same-named `mod wasm_session {
  ... }` blocks exist elsewhere — `🗺️surface/🕸️node-graph` and `♾️infinite/…/🕸️dag` — both already
  correctly `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`-gated; false-positive
  matches on the name, not the same code.)
- The file has its own `#[cfg(test)]` suite (9 tests) that must keep compiling on **native**, so
  the fix could not use the browser-only `all(wasm32, not(p2))` gate (that would drop it from
  native too and break those tests) — it needed the **host-only** `not(all(wasm32, p2))` shape
  instead, present on native + browser wasm, absent only from `wasm32-wasip2`. Exactly the target
  table `os-kernel-host-crates-split.md` already established for `tokio`/`zip`.

### The fix

1. `📦️glue.rs`: gated the `wasm_session` module mount itself —
   `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` above `pub mod wasm_session;`.
2. `Cargo.toml`: moved `ui_webgpu` out of unconditional `[dependencies]` into the pre-existing
   `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table (the
   same table that already narrows `ui_wgpu`'s `wgpu-engine` feature for this crate) — present on
   native + browser wasm, absent only from `wasm32-wasip2`.

### Evidence

```
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i taffy    → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i parley   → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i swash    → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i fontique → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i semio-framework-ui-backend-webgpu
    → nothing to print
$ cargo tree -p semio-framework-os-flow -i semio-framework-ui-backend-webgpu   (native, no --target)
    → resolves fine, ui_webgpu still linked for native/browser as intended
```

Count: `semio-s-plugin-flow` wasip2 third-party crates **117 → 104**, now equal to puzzle's 104.
`cargo metadata --no-deps --manifest-path .../flow/📦️packages/🦀️rust/Cargo.toml` parses cleanly
(manifest syntax valid).

### Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`

## Fix 2 (real, verified, zero count impact): one confirmed-dead `image::` call site deleted-in-place

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`'s
`pub fn apply_reference_image_bytes(state, url, bytes)` decodes arbitrary image bytes via
`image::ImageReader`. Repo-wide grep (`grep -rn "apply_reference_image_bytes"`) found **zero**
callers anywhere — not a plugin, not a test, not even inside its own file. Its sole callee,
`fn publish_world_pixels(...)` (private, one call site), becomes unreferenced too once this is
gated, which is an acceptable `warn`-level `dead_code` lint under this workspace's own
`[workspace.lints.rust]` policy ("kept at warn, never deny… zero-warning enforced at verification
gates via `cargo clippy -D warnings`", not by `cargo check`/`cargo build`) — not a build break.

Gated `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` rather than deleted, to keep the
diff minimal and reversible. **This does not remove `image` from either plugin's wasip2 link
graph** — see Fix-attempted-and-reverted below for why `image` (and the entire
`usvg`/`rustybuzz`/`vello_svg`/`fontdb` tail) has to stay. Filed here anyway because it is a real,
independently-verified dead-code finding matching the ticket's classification #1, at zero risk (the
function is 100% unreferenced on every target, confirmed by grep, and the change is a pure
`#[cfg]` addition with no logic change).

## Fix attempted and reverted: the real reason `usvg`/`rustybuzz`/`image`/`vello_svg` (~60 crates) cannot be gated away

This is the important negative result and the reason puzzle stayed at 104. Documented in full
because the ticket's own brief predicted this exact driver group and expected it to be a "strong
host-only candidate" — that prediction turned out to be **wrong** for this codebase, and only
tracing the actual call graph (not the crate names) revealed why.

### The hypothesis (matching the ticket brief and `raster-tier-split.md`'s precedent)

`♾️infinite`'s `🖼️canvas`/`🎲️board` modules contain a large icon/label rendering pipeline:
`IconPaintCache::get_or_build` (in `🎲️board/🔌️ports/➡️directed/🦀️component.rs`) turns an encoded
icon string into a `vello::Scene` via `SvgDocument::parse_icons` (`usvg::Tree::from_str`, pulling
`rustybuzz`/`fontdb`/`roxmltree`/`svgtypes`/`tiny-skia-path`/…) or a raster decode
(`image::load_from_memory`). `build_vector_scene`/`paint_scene` (the `BoardHost`/`DagHost` public
entry points) are the only callers of `get_or_build`. A repo-wide grep for their only real callers
found:

1. `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🌉️wasm/🦀️component.rs` — already whole-file gated
   `#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` (browser-only, confirmed by its
   own docstring: *"keeps the puzzle 2d app's ⚙️engine slot free of wasm-bindgen/web-sys/wgpu…this
   file never links wasm-bindgen/js-sys/web-sys into the wasm32-wasip2 plugin component"*).
2. `🧰️framework/…/🌊️flow/🌉️wasm/🦀️component.rs` — the same bridge fixed in Fix 1 above, now
   excluded from wasip2 entirely.
3. `#[cfg(test)]` blocks in `✏️s/🔌️plugins/🧩️puzzle/…/⚙️engine/🔗️linking/🦀️component.rs`.

Zero non-test, non-browser-bridge callers of `build_vector_scene`/`paint_scene` — matching the
"genuinely never reached by guest dispatch" shape that made the `raster-tier-split.md`/
`wgpu-tier-split.md` gates safe. This looked like a second, much larger win (potentially removing
~55 of the remaining 104 crates: the whole `usvg`/`vello_svg`/`rustybuzz`/`fontdb`/`image`/font-shaping
tail). A gate was drafted and applied (`SvgDocument` + `append_svg_document` + the crate's
`vello_svg`/`usvg` re-exports, all `#[cfg(not(all(wasm32, p2)))]`).

### Why it was wrong, found before landing it

Before trusting the gate, its blast radius was checked one level further: does `usvg` (not just
`SvgDocument`, the Scene-painting wrapper around it) have **any other** caller? It does:

```
🧰️framework/…/♾️infinite/🎲️board/…/🕸️dag/🦀️component.rs:465
    fn preview_media_natural_size(src: &str) -> (f64, f64) {
        match board_resolve_icon_kind(src, |_| None) {
            BoardResolvedIcon::RasterRgba8 { w, h, .. } => (w, h),                    // needs `image`
            BoardResolvedIcon::SvgPlain(s) | BoardResolvedIcon::SvgThemed(s) => {
                canvas::svg_icon::svg_icon_content_bounds_from_str(&s)                // needs `usvg`,
                    ...                                                               // NOT `vello_svg`/Scene
            }
        }
    }
```

This computes an image/SVG's **natural width/height as plain data** (`usvg::Tree::from_str` +
a pure bounds query — no `Scene`, no `vello_svg::append_tree`) — a layout measurement, not
rendering. Traced its callers up one more hop:

```
preview_media_natural_size → image_widget_size / measure_preview_content
  → widget_node_size (🧰️framework/…/🌊️flow/📄️artifact/🦀️component.rs:599)
  → widget_to_dag_node (same file, pub(crate), :691)
  → 🖥️host/🦀️component.rs:1467 — rebuilds every DAG node (incl. size) from `self.fixture.widgets`
    on every fixture change, e.g. called from `set_image_src` (an ordinary, unconditional,
    guest-reachable mutation handler — not test-only, not wasm-bridge-only).
```

`board_resolve_icon_kind` is one shared function handling all `Icon` variants (`Math`/`Emoji`/
`Text` route through `compiler::compile_*_to_svg`, i.e. `rustybuzz`; `Data` routes through
`image::load_from_memory`; `Svg`/`Catalog`/`Themed` route through `usvg` via
`svg_icon_content_bounds_from_str`) — it cannot be split by target without duplicating real
resolution logic, and every branch is reachable from `widget_to_dag_node`'s production path.

**Conclusion: `usvg`, `rustybuzz` (via `compiler`), `fontdb`, and `image` are genuinely
guest-reachable** — flow's plugin computes real layout data (widget/node pixel sizes) from parsed
SVG/raster content as part of ordinary state mutation, with no GPU or display involved and no
"environment unavailable" fallback to return honestly (unlike `wgpu`'s "no adapter" case in
`raster-tier-split.md`, there is no missing *capability* here — `wasm32-wasip2` can run this code
today, it's pure computation). This is fundamentally different from the wgpu/GPU driver class the
ticket brief listed alongside it: no environmental reason blocks it, so there is no honest
"unavailable" value to substitute — only a first-party reimplementation of an SVG-dimension parser
(and, if `compiler`'s three call sites are to go too, of enough of `rustybuzz`'s text-shaping to
size laid-out glyphs) would let this move, at the same scale of effort as the ticket's blake3/
deflate/parry3d first-party rewrites, but larger — and unlike those, unverifiable in the time
remaining in this pass without risking a silent size regression in flow's live diagram layout.

**The drafted gate was reverted in full before landing** (`git diff` on
`🖼️canvas/🦀️component.rs` is empty as of this doc). Nothing was left half-applied.

`SvgDocument`/`append_svg_document`/`vello_svg::append_tree` themselves (the actual Scene-painting
step, as opposed to the bounds-only query) remain confirmed unreached from guest dispatch — but
since `usvg` and its whole dependency tail must stay linked anyway for the bounds-query path above,
gating just the painting step would touch risk for zero measurable count reduction, so it was left
alone rather than done for its own sake.

## Per-crate classification of the remaining 104 (both plugins, now identical)

Sampled with `cargo tree --target wasm32-wasip2 -i <crate>` per crate name in the shared list.

| class | crates (representative) | disposition |
|---|---|---|
| serde tail (ticket's own scope fence) | `serde`, `serde_core`, `serde_json`, `itoa`, `memchr`, `zmij` | out of scope — "a separate later wave" per `verified-outcomes.md` |
| proc-macro-only, **not actually linked** (`cargo tree` measurement-correction per `verified-outcomes.md`) | `syn`, `quote`, `proc-macro2`, `unicode-ident`, `unicode-xid`, `serde_derive`, `heck`, `prettyplease`, `macro-string`, `static_assertions`, plus the whole `wit-bindgen`/`wasm-tools` chain (`wit-bindgen`, `wit-bindgen-core`, `wit-bindgen-rust`, `wit-bindgen-rust-macro` **(proc-macro)**, `wit-component`, `wit-parser`, `wasm-encoder`, `wasm-metadata`, `wasmparser`, `anyhow`, `log`, `leb128fmt`, `id-arena`, `semver`, `equivalent`, `indexmap`, `hashbrown`, `foldhash`, `bitflags`, `cfg-if`) | host-only: every path from these to `semio-s-plugin-{puzzle,flow}` passes through `wit-bindgen-rust-macro (proc-macro)` or `semio-framework-dispatch-macros (proc-macro)`, confirmed for the sampled crates the same way `os-kernel-host-crates-split.md` confirmed `syn`/`quote`. Compiled for the host, never linked into the `.wasm`. |
| SVG/text-shape/raster pipeline, **genuinely guest-reachable** (see above) | `usvg`, `rustybuzz`, `fontdb`, `roxmltree`, `svgtypes`, `tiny-skia-path`, `simplecss`, `xmlwriter`, `data-url`, `strict-num`, `siphasher`, `slotmap`, `unicode-bidi`, `unicode-bidi-mirroring`, `unicode-ccc`, `unicode-properties`, `unicode-script`, `unicode-vo`, `ttf-parser`, `skrifa`, `read-fonts`, `font-types`, `float-cmp`, `pico-args`, `memmap2`, `image`, `png`, `gif`, `image-webp`, `imagesize`, `weezl`, `zune-core`, `zune-jpeg`, `color_quant`, `byteorder-lite`, `fdeflate`, `simd-adler32`, `crc32fast`, `quick-error`, `vello`, `vello_svg`, `vello_encoding`, `peniko`, `kurbo`, `guillotiere`, `moxcms`, `polycool`, `pxfm`, `core_maths`, `linebender_resource_handle`, `svg_fmt`, `color`, `adler2`, `flate2`, `miniz_oxide`, `base64`, `bytemuck`, `arrayref`, `arrayvec`, `euclid`, `libm`, `num-traits`, `smallvec`, `tinyvec`, `tinyvec_macros`, `thiserror`, `thiserror-impl` | left alone — confirmed reachable from `widget_to_dag_node`'s production layout path (flow) / the equivalent shared `IconPaintCache` surface (puzzle, same `♾️infinite` crate) |

The residual 104 is not "104 unclassified crates" — it decomposes into a known, already-scoped
serde wave, a proc-macro tail that per this ticket's own measurement methodology was never actually
linked, and one large, now-verified-necessary computation (not rendering) pipeline. No further
crate in this list is a Cargo.toml-level "unconditional host-only declaration" bug of the kind Fix 1
was — that class of bug is exhausted for these two plugins as far as this pass could find.

## Build verification

- `cargo metadata --no-deps --manifest-path .../🌊️flow/📦️packages/🦀️rust/Cargo.toml` — parses
  clean.
- `rustfmt --edition 2021 --check` on both edited `.rs` files — exit 0 on both; the only diffs
  shown are pre-existing formatting on lines this pass never touched (same shape as
  `raster-tier-split.md`'s fallback verification).
- **`cargo check -p semio-framework-os-flow --target wasm32-wasip2`** and
  **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm`** (the ticket's own
  "currently 0 errors" baseline) — both **BLOCKED**, identically, by a confirmed unrelated,
  in-flight, uncommitted peer edit: `git diff --stat 🧰️framework/🔨️modules/📡️replication` shows
  1480 insertions across 10 files, live right now. Every error in both captured logs is
  `E0277`/`MutationOrigin: serde::Deserialize` in `📡️replication/🎮️mutation/🦀️.rs` and
  `📡️wire/🦀️.rs` — confirmed by `grep -oE '^\s*-->\s*\S+' | sort -u`: **zero** matches for
  `🌊️flow`, `♾️infinite`, or this pass's two edited files in either log. `semio-framework-
  replication` is a direct, unconditional dependency of `os-flow` (and of `draw-fsm`, the ticket's
  own known-good baseline, which is *also* currently blocked by this same peer edit) on every
  target, so no plugin reaches its own compilation while this is mid-flight — consistent with
  `verified-outcomes.md`'s documented "framework-async has ~8 errors from another agent's in-flight
  edit" caveat, just in `replication` instead. Re-running once that peer session lands is the
  natural way to close this gap; `cargo tree`/`cargo metadata`/`rustfmt --check` are lock-free,
  metadata-only, and unaffected by it, and are the primary evidence for the count claims above.

## What is proven vs. not proven — stated plainly

**PROVEN**: `semio-s-plugin-flow`'s wasip2 third-party crate count is **117 → 104** (lock-free
`cargo tree`, cannot go stale); it is now byte-identical to puzzle's set; `ui_webgpu`/
`semio-framework-ui-backend-webgpu`/`taffy`/`parley`/`swash`/`fontique` are confirmed absent from
flow's wasip2 tree by `cargo tree -i` and confirmed still present on native/browser (not a
regression there); the manifest parses; both edited `.rs` files are syntactically valid. The
`usvg`/`rustybuzz`/`image` reachability analysis (the negative result) is proven by grep-traced call
chains ending in a real, unconditional, non-test production function
(`🖥️host/🦀️component.rs:1467`'s DAG-node rebuild), not by inspection alone.

**NOT proven**: an end-to-end `cargo check`/`cargo build` for `semio-framework-os-flow` or
`semio-s-plugin-flow` at 0 errors on `wasm32-wasip2` — blocked by the unrelated, live,
uncommitted `📡️replication` peer edit described above, on **both** this pass's target and the
ticket's own `draw-fsm` baseline identically, so not a regression this pass introduced and not
something this pass can resolve. `semio-s-plugin-puzzle`'s count (104) is unchanged by this pass —
investigated thoroughly (see "Fix attempted and reverted") but no additional safe reduction was
found for it.
