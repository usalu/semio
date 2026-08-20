# 📓️ Packet `render-text` — report

Anchor commit `5e7b8046be`. Source read (not modified, lives in the externally-red `wgpu-engine`
feature): `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️text.rs` (604 lines).
File owned and rewritten wholesale:

- `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️text.rs`

## Done

Before writing anything I verified the actual `parley 0.5.0` / `swash 0.2.9–0.2.10` / `fontique 0.5.0`
/ `peniko 0.4.1` API surface against the vendored crate sources under `~/.cargo/registry/src/…` (two
research passes, cross-checked myself with direct `grep`/`Read` on the vendored files for the load-
bearing calls — `pub use fontique;`/`pub use layout::*`/`pub use style::*` in `parley`'s `lib.rs`,
`Blob::id()`, `FontInfoOverride`'s field list, `RangedBuilder`/`Layout`/`Cluster`/`Cursor`/`Selection`
signatures) rather than trust the corrupted source file's imports blindly. Finding: the pre-corruption
file's imports and call sequence (`RangedBuilder` → `Layout` → `PositionedLayoutItem::GlyphRun` →
swash `Render`) were already correct against these exact APIs — the asyncify pass had only sprinkled
`async fn`/`.await` onto plain synchronous CPU code with no suspension point, exactly the defect class
`📋️master.md` already documents for the WGSL shaders. Stripping that is the mechanical part of this
port; everything else is a genuine architectural rewrite, not a strip:

**`🔖️Font`** — `FontFamilyChoice`/`TextStyle` (no `parley`/`fontique` type in either), `FontHandle`
(font identity interned by `(fontique Blob::id(), font_index)` — `Blob` already carries a monotonic
unique id per allocation, so no content hashing or pointer games needed), `FontDependencyId`/
`FontStatus`/`request_font`/`provide_font_bytes`/`font_status`/`take_ready_dependencies` (the host-font
loading lifecycle), and the `FontSource` seam trait (defined, deliberately unused inside this crate —
see Decisions). `TextSystem::new()` eagerly registers the four embedded families (Anta/Kelly Slab/
Share Tech Mono/Noto Emoji, ported verbatim byte-for-byte from the source) with no network/system-font
scan, same deterministic-and-dependency-free guarantee the old `FontAtlas::builtin()`/`shaped()` merge
gave — there is no separate "bitmap fallback" mode anymore (see Deviations).

**`🔖️Shape`** — `layout_for` (the one private `RangedBuilder` builder every other method funnels
through), `TextStyle`/`ShapedGlyph`/`ShapedText`, and `shape`/`shape_uncached` — real whole-string
`parley::Layout` shaping (bidi + font fallback resolved by parley/fontique automatically) replacing the
old per-character single-glyph `Layout` build. `shape` caches by `(text, family, size)`; `measure`
(see `🔖️Measure`) calls `shape` internally, so there is exactly one cache, not two.

**`🔖️Atlas`** — `GlyphEntry` (UV coordinates deliberately not stored — see Decisions),
`GlyphCacheKey`/`RasterizedGlyph`, `AtlasPage` (row-cursor bin packer, ported from
`FontAtlas::pack_glyph`, height-only-doubling growth), and `TextSystem::ensure_glyph`/`rasterize` —
rasterization ported near-verbatim from `render_resolved` (color-bitmap/outline-first swash `Render`
pipeline), now returning `GlyphEntry`/queuing through `ResourceRegistry::request_atlas_upload` instead
of raw `pixels: Vec<u8>` + `take_dirty` bools.

**`🔖️Measure`** — `measure`/`measure_wrapped` (return `crate::layout::Measurement`, this crate's own
type, not a new one), `wrap` (byte-range lines via parley's own UAX#14 line breaking, replacing the old
whitespace-split hack), `next_grapheme`/`previous_grapheme`/`selection_geometry` (new — built on
`parley::Cursor`/`Selection`, grapheme-cluster-safe by construction, no new dependency), and
`utf8_to_utf16`/`utf16_to_utf8` (new, free functions).

9 in-file tests (`#[cfg(test)] mod tests`) covering every behaviour the packet brief listed: stable
ASCII measurement matching summed shaped advances; wrapping breaking at max-width on char boundaries
with no character loss; a missing custom font yielding `Measurement::Pending` + placeholder (then
resolving to `Ready` once bytes are provided, with `take_ready_dependencies` draining exactly once);
the shape cache actually being hit (asserted via hit/miss counters, not just answer equality); atlas
insertion queuing exactly one `UploadAtlas` op with a repeat glyph queuing none; UTF-8/UTF-16 index
round-trip across a non-BMP emoji; and (beyond the brief's minimum) grapheme-safe cursor movement never
landing inside an emoji's surrogate pair, and non-empty selection geometry.

No `async fn`, no `.await`, no `wgpu`/`winit` anywhere in the file (only doc-comment prose mentions the
old file's path). Every non-trivial `fn` carries the `// 🚫️async: U1 …` tag.

## Acceptance

Per ruling **U4**: I ran no cargo command. **UNRUN** — exact commands for `sol`, `timeout: 600000`,
from `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust`:
```
cargo test -p semio-framework-ui-render --lib
cargo test -p semio-framework-ui-render --all-targets
bun ./📜️script.ts boundaries
```
Sibling packets are editing other files in this crate concurrently (`resource.rs`, `scene.rs`,
`element.rs`, `frame.rs`, `layout.rs`, `schedule.rs`, `backend.rs`, `shader_contract.rs`, `surface.rs`,
`dispatch.rs`); unresolved names from those files are not this packet's.

Cheap non-cargo checks I did run:
- `rustfmt --edition 2021 --check 🦀️text.rs` — clean (after letting `rustfmt` itself reformat two
  lines that exceeded the project's evident line-width preference; re-`--check` after is exit 0). This
  also proves the file parses as syntactically valid Rust — `rustfmt` errors on a parse failure rather
  than emitting a diff.
- `//#region`/`//#endregion` balance: 5/5.
- Brace/paren/bracket balance: `{}` 155/155, `()` 472/472, `[]` 83/83.
- `grep -c "async fn"` / `grep -c '\.await'`: both 1, and both hits are inside doc-comment prose
  (`` `async fn`/`.await` `` in this file's own header, describing what the port removed), not real
  code — confirmed by inspecting both matches directly.
- `grep -i "wgpu\|winit"`: 7 hits, all doc-comment prose referencing the old file's path
  (`🎯️targets/🧊️wgpu/🦀️text.rs`) or "wgpu-old target" — no actual `use wgpu::`/`use winit::`.
- Manual signature verification against the vendored crate sources for every non-trivial call
  (`RangedBuilder::push_default`/`build`, `Layout::break_all_lines`/`lines`/`width`/`height`/`is_rtl`,
  `Line::metrics`/`text_range`, `GlyphRun::run`/`positioned_glyphs`, `Cursor::from_byte_index`/
  `next_visual`/`previous_visual`/`index`, `Selection::new`/`geometry`, `Blob::id`/`data`,
  `FontInfoOverride`'s full field list, `Collection::register_fonts`/`set_generic_families`,
  `swash::scale::Render::new`/`format`/`render`, `Content::Color`, `zeno::Format::Alpha`) — not a
  substitute for `cargo check`, but the closest available given U4.

## Decisions

- **`GlyphEntry` does not store UV coordinates, only pixel-space `atlas_x`/`atlas_y`/`width`/`height`.**
  `GlyphEntry::uv_rect(atlas_width, atlas_height)` computes them on demand. This is the answer to the
  packet brief's "a repack that invalidates existing glyph coordinates must invalidate the frames that
  used them" question: `AtlasPage` grows **height-only** (doubling, `Vec::resize` appending zeroed
  rows) — `width` and therefore every existing glyph's `atlas_x`/row stride never change, so no
  previously packed glyph's pixel-space coordinates are ever invalidated by growth. The only thing that
  *would* go stale across a resize is a UV value computed against the *old* width/height, so `GlyphEntry`
  simply never caches one — a caller reads `TextSystem::atlas_dimensions()`/`color_atlas_dimensions()`
  fresh every paint call. Since this whole crate's element/frame model already rebuilds from scratch
  every frame (`element.rs`'s own docstring), there is no "stale frame" that could hold a UV computed
  against a since-grown atlas — paint always recomputes it against the size that frame actually has.
- **Atlas growth still needs a fresh `AtlasId`, even though coordinates don't change.**
  `ResourceRegistry::request_atlas_upload` only re-queues an upload for an id that is not yet
  `Resident` (coalescing rule already fixed at the W1 gate per `📓️status.md`). If a backend already
  marked a page's id `Resident` before it grew, a same-key re-request after growth would silently no-op
  and the grown pixels would never reach the device. `AtlasPage` sidesteps this without touching
  `resource.rs` (outside this packet's OWNS list): the interned key is versioned by `generation`
  (`"{key}:{generation}"`), so a grown page always resolves to a brand-new `AtlasId` the registry has
  never seen, and the upload is unconditionally queued. The stale previous-generation id is simply
  abandoned (nothing retains it; no frame references it past the one that triggered growth).
- **`take_dirty`/`take_color_dirty` are gone, not "still distinct as two bools."** They're replaced by
  two independently-coalesced `ResourceOp::UploadAtlas` streams — one per `AtlasPage` (alpha vs RGBA
  color-emoji), each going through its own `request_atlas_upload` call under its own interned key. The
  *reason* they were two flags in the old code — a backend binds the alpha mask page and the RGBA
  color-emoji page as two separate textures, uploaded independently — is preserved exactly; the
  mechanism moved from two hand-rolled dirty bools to the registry's own per-id upload-queue
  bookkeeping, which is strictly more correct (it already handles coalescing/residency/growth
  correctly, which two bare bools didn't).
- **The `FontSource` seam is defined but genuinely unused inside this crate.** Per the packet brief:
  "define a narrow `FontSource` seam ... and note it in registrar-requests for the future `ui-host`
  packet" — it's a contract for that future crate to implement and drive, not something this crate
  calls. Signature: `trait FontSource { fn poll_font(&mut self, key: &str) -> FontFetch; }` where
  `FontFetch = { Pending, Ready(Vec<u8>), Failed }`. Generic per U3 (never boxed/`dyn`): a host
  monomorphizes its own executor/transport type over it and, once `poll_font` reports `Ready`, calls
  `TextSystem::provide_font_bytes` — a plain sync call, matching this crate's `request_font`/
  `font_status`/`take_ready_dependencies` lifecycle exactly.
- **`TextStyle`/`FontFamilyChoice` carry no weight/style/variable-font-axis knobs.** Same posture
  `layout.rs` takes on `Theme` — that's product policy the caller resolves before reaching this crate.
- **No per-glyph `byte_range` on `ShapedGlyph`.** Cursor/selection/hit-testing go through
  `next_grapheme`/`previous_grapheme`/`selection_geometry` (parley's own `Cursor`/`Cluster`/`Selection`
  API, built fresh from a `Layout`), not through walking `ShapedText::glyphs`. Adding a byte range to
  every glyph would need iterating `Run`'s cluster structure alongside its glyphs — real complexity
  with no test requirement or call site (present or forward-referenced) actually needing it; dropped
  rather than spending scope on speculative API surface, consistent with the greenfield "no unused
  compat/legacy surface" posture.

## Registrar-requests

None to `Cargo.toml`/`project.json`/taxonomy — the crate and `parley`/`swash` dependencies already
existed as scaffolds. One note for whichever session lands the `ui-host` packet: wire a concrete
`FontSource` impl (network fetch on wasm, filesystem/ureq on native — the old `fetch_font_bytes`'s job)
that calls `TextSystem::request_font`/`provide_font_bytes` from its own poll loop, per the `FontSource`
trait doc comment in `🔖️Font`.

## Deviations

- **No "bitmap fallback" mode.** The source kept an 8×16 ASCII bitmap `AtlasMode::Bitmap` as a
  dependency-free deterministic fallback used by `FontAtlas::builtin()` and any codepoint no registered
  font could shape at all. This port always uses the real embedded-font `Shaped` pipeline — it is
  already deterministic and dependency-free (no network, no system-font scan), so the bitmap mode's own
  justification for existing is already met by the normal path; keeping two parallel rasterization code
  paths for no remaining reason would be exactly the kind of inconsistency CLAUDE.md's greenfield rules
  ask to refactor away, not preserve. A glyph no registered font (including the Noto Emoji fallback)
  can shape at all now rasterizes as `RasterizedGlyph::empty()` (a real, cached, zero-size entry) rather
  than a crude ASCII box — never a panic, never a blocked call.
- **`wrap` returns `Vec<Range<usize>>` into the input `text`, not owned `Vec<String>` line copies** —
  the old `wrap_text`'s shape. A caller that wants owned lines slices `text[range]` itself; a caller
  building glyph runs per line (the more likely paint-time consumer) avoids a copy entirely. This is a
  public-API shape change from the source, made deliberately per this packet's "big-bang, no compat
  layer" instruction.
- **`measure`/`measure_wrapped` return `crate::layout::Measurement` instead of the old `(f32, f32)`
  tuple** — required by the packet brief, not incidental.

## Files touched

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️text.rs`
  (rewritten wholesale; only file this packet owns or touched)
