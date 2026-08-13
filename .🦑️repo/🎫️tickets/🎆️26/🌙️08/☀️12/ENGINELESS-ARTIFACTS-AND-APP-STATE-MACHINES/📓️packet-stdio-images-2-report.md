# Packet: stdio images wave 2 — gif (87a+89a), bmp, tiff, svg, jpg

Ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES. Targets assigned: `🎞️gif` (87a + 89a
standards), `🖼️bmp`, `🖼️tiff`, `🎨️svg`, `📷️jpg`. All 6 `⚙️engine` directories dissolved and deleted.

## Baseline (before editing, captured live — not `git show HEAD:` per the ticket's own warning)

```
find ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎞️gif,🖼️bmp,🖼️tiff,🎨️svg,📷️jpg} -type d -name "⚙️engine"
```
6 hits:
- `📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/⚙️engine` — 1492 lines
- `🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/⚙️engine` — 339 lines
- `🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/⚙️engine` — 789 lines
- `🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/⚙️engine` — 1056 lines
- `🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/⚙️engine` — 909 lines
- `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/⚙️engine` — 986 lines

Total: 5571 lines across 6 files.

## Pre-flight: which of the 5 formats are "do-not-touch protected" (stdio's 10 imperative `engine::register()` plugin-root calls)

Read `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`'s `plugin()` fn directly (never inferred): the 10
protected calls are `binary`, `txt`, `ifc`, **`gif`**, **`bmp`**, `semio.v1`, `wav.riff_pcm`,
`epw.energyplus`, `tsv`, `html.v5`. **`gif` and `bmp` ARE protected** — their bare
`crate::artifacts::{gif,bmp}::engine::register()` calls at the plugin root are untouched, byte-for-
byte, per the ticket's "do not touch" rule. `jpg`/`svg`/`tiff` are **not** in this list — their root
`component.rs` files already used the `declaration()` pattern (confirmed by reading each file), so
their engine's own `register`/`register_pilot_languages`/`register_artifact_inferences`/
`register_schema_specs` cluster was genuinely dead code (verified: 0 real call sites repo-wide,
only doc-comment mentions) and was deleted outright rather than relocated.

## Destination per region per artifact

### 🎨️svg (`standards::v1_1`, engine mounted **beside** `subsets` — confirmed via glue.rs, not inferred)
- `empty_svg_snapshot`/`demo_svg_snapshot` → `🧬️schema/component.rs` (rule 5, pure helpers over `SvgSnapshot`/`XmlDocument`).
- `io_registry` (bare `&'static [ComposerEntry]`) → `🚪️io/component.rs` (rule 3).
- `register`/`register_pilot_languages`/`register_artifact_schema`/`register_artifact_inferences` — **deleted outright** (dead: superseded by `declaration()`, 0 real callers besides the doc comment already noting the fact).
- `SvgEngine` struct — **deleted outright** (0 construction sites, confirmed by grep before AND after).
- Tests (`empty_snapshot_matches_schema`, `codec_round_trip`, `conformance_laws` mod) → `🚪️io/component.rs`'s own `mod tests` (mirrors `las`'s already-settled precedent in this same ticket).

### 📷️jpg (`standards::v_jfif_1_01`, engine mounted beside `subsets`)
- `JpgError` enum + `Display`/`Error` impls, `ZigZag`, `Idct`/`Fdct`, `Huffman` table builder, `BitIo` (`BitWriter`/`BitReader`), `BlockCodec`, `QuantTables`, `StdHuffmanTables`, `ColorConvert`, `Encode` region (`encode_jpg` + helpers), `Decode` region (`decode_jpg` + `decode_scan` + helpers) → `🚪️io/component.rs` (rule 2 codec, rule 6 — every pure algorithm stays WITH the codec here since it's JPEG-specific plumbing, not artifact-independent; no promotion to a module engine).
- `io_registry` → `🚪️io/component.rs` (rule 3).
- `empty_jpg_snapshot`/`demo_jpg_snapshot` (kept `pub(crate)`) → `🧬️schema/component.rs` (rule 5).
- `register`/`register_pilot_languages`/`register_artifact_inferences`/`register_schema_specs` — **deleted outright** (dead, same reasoning as svg — confirmed 0 real callers).
- `JpgEngine` struct — **deleted outright** (0 construction sites).
- Tests (incl. `idct_fdct_is_identity`, Huffman round-trips, gradient/checkerboard/solid-color MAE round trips, progressive-SOF2-rejection, `conformance_laws`) → `🚪️io/component.rs`'s own `mod tests`.
- **Consumer**: `📸️remodel/🎛️apps/📸️remodel/⚙️engine/🖼️images/🦀️component.rs` calls `semio_s_plugin_stdio::artifacts::jpg::engine::{decode_jpg,encode_jpg,JpgError::{Unsupported,Malformed}}` — resolved via the standard-level inline `engine` barrel (`pub use super::subsets::any::io::*;`), no consumer edit needed. Verified: no `use … as jpg_engine` alias shape.

### 🖼️tiff (`standards::v6_0::subsets::any::engine` — **nested inside `subsets::any`**, confirmed via glue.rs; a SECOND barrel at `v6_0::engine { pub use super::subsets::any::engine::*; }` already sat above it, unchanged)
- `ByteOrder` (`Endian`, read/write u16/u32/u64), `IfdRead` (`RawEntry`/`RawIfd`, `read_ifd_raw`, `read_ifd_chain`, `read_tag_values`), `TagLookup`, `PackBits` (`packbits_decode`/`packbits_encode`), `Decode` (`decode_pixels_from_ifd`, `decode_tiff`), `Encode` (`rgba_to_rgb`, `value_bytes`, `encode_tiff_with`, `encode_tiff`, `encode_tiff_packbits`) → `🚪️io/component.rs`.
- `io_registry` → `🚪️io/component.rs`.
- `empty_tiff_snapshot`/`demo_tiff_snapshot` → `🧬️schema/component.rs`.
- `register`/`register_pilot_languages`/`register_artifact_inferences` — **deleted outright** (dead, confirmed 0 real callers).
- `TiffEngine` struct — **deleted outright** (0 construction sites).
- Tests → `🚪️io/component.rs`'s own `mod tests`.
- **Consumer**: `🧿️semio` image serializer + tiff's own `📚️examples/🎬️demo` call `crate::artifacts::tiff::engine::{decode_tiff,encode_tiff}` — resolved via the nested-inside-`any` inline barrel + the pre-existing `v6_0::engine`/root shims above it, unchanged.

### 🎞️gif 87a (`standards::v87a`, engine mounted beside `subsets`) — **PROTECTED**
- `BitIO`, `Lzw` (`lzw_encode`/`lzw_decode`, `pub` — reused verbatim by 89a), `SubBlocks` (`pack_sub_blocks`/`unpack_sub_blocks`, `pub`), `ColorTable` (`Rgb` type + `color_table_size_field`/`read_color_table`/`write_color_table`, `pub`), `Quantize` (`min_code_size_for`/`quantize_rgba`/`indices_to_rgba`, `pub`), `Interlace` (`deinterlace_rows`/`interlace_rows`, `pub`), `ColorTableConv`, `Codec87a` (`encode_gif`/`decode_gif`/`validated_color_table_size_field`), `Sniff` (`sniff_magic`) → `🚪️io/component.rs`.
- `register`/`register_artifact_inferences`/`register_pilot_languages`/`register_schema_specs` — **kept together, relocated (not deleted)**: `register()` is reached by stdio's protected imperative `crate::artifacts::gif::engine::register()` plugin-root call (line 11 of `🗄️stdio/🦀️component.rs`), via this standard's own inline `engine` barrel.
- `io_registry` → `🚪️io/component.rs`.
- `empty_gif_snapshot`/`demo_gif_snapshot` → `🧬️schema/component.rs`.
- `GifEngine` struct — **deleted outright** (0 construction sites).
- Tests (LZW round trips incl. the documented growth-boundary regression, encode/decode round trips, `conformance_laws`) → `🚪️io/component.rs`'s own `mod tests`.

### 🎞️gif 89a (`standards::v89a`, engine mounted beside `subsets`) — **PROTECTED**
- `ColorTableConv` (89a's own `GifColorTable`↔`Rgb` bridge, distinct type from 87a's), `Codec89a` (`encode_gif`/`decode_gif`/`validated_color_table_size_field`/`write_gce`/`write_plain_text`) → `🚪️io/component.rs`. Cross-standard reuse import `use crate::artifacts::gif::standards::v87a::engine as codec;` left **unchanged** — resolves through 87a's own new barrel.
- `register`/`register_artifact_inferences`/`register_pilot_languages`/`register_schema_specs` — **kept together, relocated**: the top-level `gif::engine` barrel's own local `register()` override explicitly calls `super::standards::v87a::engine::register()` AND `super::standards::v89a::engine::register()` — both untouched, both now resolve through the new barrels.
- `io_registry` → `🚪️io/component.rs`.
- `empty_gif_snapshot`/`demo_gif_snapshot` (89a's calls the real `dancing.gif` fixture decoder) → `🧬️schema/component.rs`.
- `GifEngine` struct — **deleted outright** (0 construction sites).
- Tests → `🚪️io/component.rs`'s own `mod tests`.
- **Consumer**: `🎞️animate/🎛️apps/🎬️present/⚙️engine/🎥️video/🦀️component.rs` calls `semio_s_plugin_stdio::artifacts::gif::engine::encode_gif` — resolved via the (untouched) top-level `gif::engine` barrel → `standards::v89a::engine::*` → new inline barrel. No consumer edit needed. Verified: no `use … as gif_engine` alias shape.

### 🖼️bmp (`standards::v_v3`, engine mounted beside `subsets`) — **PROTECTED**
- `ByteIo`, `RowGeometry` (`row_bytes`, made `pub(crate)` so `../🧬️schema`'s `demo_bmp_snapshot` can compute a real `image_size`), `Bitfields`, `IndexUnpack`, `Codec` (`decode_bmp`/`encode_bmp`) → `🚪️io/component.rs`.
- `register`/`register_artifact_schema`/`register_artifact_inferences`/`register_pilot_languages`/`register_schema_specs` — **kept together, relocated**: `register()` is reached by stdio's protected imperative `crate::artifacts::bmp::engine::register()` plugin-root call (line 12 of `🗄️stdio/🦀️component.rs`).
- `io_registry` → `🚪️io/component.rs`.
- `empty_bmp_snapshot`/`demo_bmp_snapshot` → `🧬️schema/component.rs`.
- `BmpEngine` struct — **deleted outright** (0 construction sites).
- Tests (row-padding, 24-bit round trip, indexed-palette, 16-bit bitfields, `codec_retention_law`, `conformance_laws`) → `🚪️io/component.rs`'s own `mod tests`.

## Module-path shim mechanics (the ticket's own "hit hard tonight" warning)

Every mount's exact nesting was read from `📦️glue.rs` directly (never inferred from a directory
listing), matching the escalation ladder:
- svg/jpg/gif87a/gif89a/bmp: `pub mod engine;` sat **beside** `subsets` at the `standards::vX`
  level → replaced with an inline `pub mod engine { pub use super::subsets::any::io::*; }`
  (gif's two standards also barrel `register()` alongside `io::*` since it lives in the same file).
- tiff: `pub mod engine;` sat **nested inside** `subsets::any` (sibling of `schema`/`io`/`examples`)
  → replaced with `pub mod engine { pub use super::io::*; }` at that same nesting level, leaving
  the pre-existing outer `v6_0::engine`/root-artifact `engine` barrels completely untouched (they
  already pointed at the right relative path).

No file-wide or global search-and-replace was used on any path; every glue.rs edit targeted the
exact `#[path=…] pub mod engine;` line for that one standard, verified per-symbol against the
compiler-adjacent checks below.

## Bare `io_registry` shadow hazard

Repo-wide bare `io_registry::entries()` count (excluding fully-qualified and doc-comment hits):
**0** — verified after every edit. Every artifact root's own `.composers(...)` call reaches the
freshly-relocated `io_registry` by its fully-qualified `standards::vX::…::io::io_registry::entries()`
path (through the barrel), never the bare shortcut.

## Assertion arithmetic

Every `#[test]` fn and every nested `mod conformance_laws` test from all 6 original engine files
was relocated verbatim (same body, same assertions) into the corresponding `🚪️io/component.rs`'s
own `#[cfg(test)] mod tests`. Zero tests dropped, zero assertions altered. Import lines were
adjusted only for the relocated symbols (`empty_*_snapshot`/`demo_*_snapshot` now imported from
`../🧬️schema` instead of being in-file).

## Consumers (public surface)

```
grep -rn "semio_s_plugin_stdio::artifacts::\(gif\|bmp\|tiff\|svg\|jpg\)::" ✏️s 🧰️framework --include="*.rs" \
  | grep -v "^✏️s/🔌️plugins/🗄️stdio/" | grep -vE ':[0-9]+: *(///|//!|//)'
```
Two real `::engine::` consumers found (both left untouched, no `use … as X_engine` alias shape,
both resolve through barrels):
- `📸️remodel/🎛️apps/📸️remodel/⚙️engine/🖼️images/🦀️component.rs` — `jpg::engine::decode_jpg`/`encode_jpg`/`JpgError`.
- `🎞️animate/🎛️apps/🎬️present/⚙️engine/🎥️video/🦀️component.rs` — `gif::engine::encode_gif`.

## Verification

- `find ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎞️gif,🖼️bmp,🖼️tiff,🎨️svg,📷️jpg} -type d -name "⚙️engine"` → **0 hits** (all 6 dissolved and deleted).
- Dangling `#[path]` check (python, `📦️glue.rs`) → **0 dangling**.
- `JpgEngine`/`SvgEngine`/`TiffEngine`/`GifEngine`/`BmpEngine` struct definitions repo-wide → **0** (all deleted).
- Bare `io_registry::entries()` repo-wide → **0**.
- Compiler: see below.

## Compile status

**UNVERIFIED — build-lock contention, not attempted.** Three consecutive dedicated-target-dir
attempts (`target/stdio_img2` x2 self-contended by my own retries, then `target/stdio_img3` fresh)
were started; the coordinator observed ~18 concurrent cargo processes contending the shared
workspace/registry at the time, and was asked to centrally verify once all wave-2 agents land
their edits, so this agent's own `stdio_img3` build was killed before reaching
`semio-s-plugin-stdio` itself (still resolving/building transitive deps — `tokio`/`futures-util`
were the last lines seen). No `Finished`/error verdict was obtained by this agent. Structural
checks (directory absence, dangling `#[path]`, struct absence, bare-`io_registry` absence) all
passed cleanly and are the only assertions this report makes with confidence.

## Deviations

None from the destination rules. `register`/inference/language clusters for the two protected
formats (gif, bmp) were kept as a unit inside `🚪️io/component.rs` rather than split further — they
aren't codecs, but `register()` itself calls `io_registry::register()`-equivalent registration and
is the exact symbol the protected plugin-root call needs reachable, so this is the natural, single
home (mirrors `las`'s own already-settled precedent of keeping registration-adjacent code beside
its `io_registry`).
