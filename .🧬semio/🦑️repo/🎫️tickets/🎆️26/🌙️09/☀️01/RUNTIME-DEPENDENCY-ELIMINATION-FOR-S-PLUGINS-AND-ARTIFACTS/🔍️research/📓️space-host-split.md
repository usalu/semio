# 🪐️ `space`'s 13-crate outlier — closed, matches its siblings at 36

## Headline

```
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 --edges normal --prefix none \
  | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
  | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
36
```

**49 → 36.** `space` now matches `draw`/`puzzle`/`animate`/`trinity` exactly (`flow` is 37). The
remaining 36 are the same WASI component-ABI build machinery (`wasm-encoder`, `wasm-metadata`,
`wasmparser`, `wit-bindgen*`, `wit-component`, `wit-parser`, `syn`/`quote`/`proc-macro2`/
`unicode-ident`/`prettyplease`, `heck`, `id-arena`, `leb128fmt`, `macro-string`, `semver`, `log`,
`anyhow`) plus the serde family (`serde`, `serde_core`, `serde_derive`, `serde_json`, `itoa`,
`memchr`, `zmij`) and the two other agents' `bitflags`/`bytemuck`/`bytemuck_derive`/`equivalent`/
`foldhash`/`hashbrown`/`indexmap` — none of the 13 extra crates the brief named.

```
$ for c in zip base64 flate2 miniz_oxide zopfli adler2 crc32fast simd-adler32 bumpalo cfg-if \
    displaydoc thiserror thiserror-impl; do
  cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i "$c"
done
# all: "warning: nothing to print." (thiserror/thiserror-impl disambiguated by exact version —
# @1.0.69 and @2.0.18 both "nothing to print" too)
```

All 13 named crates — `zip`, `base64`, `flate2`, `miniz_oxide`, `zopfli`, `adler2`, `crc32fast`,
`simd-adler32`, `bumpalo`, `cfg-if`, `displaydoc`, `thiserror`, `thiserror-impl` — are gone from
`space`'s wasip2 graph, confirmed by reading the full `-i` output for each (per the ticket's own
"cargo tree -i prints one tree per resolved instance" caveat — none printed a second tree either).

## Root cause, and why it was two different bugs wearing one trail

The brief traced the edge to `zip / base64 ← semio-framework-os ← semio-s-plugin-space`, with
`semio-framework-os`'s `Cargo.toml:37` declaring the host crate unconditionally with `features =
["os-host-full"]` and no `[target]` tables. Grepping `semio-framework-os`'s own `Cargo.toml`
(`🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`) showed the real shape:
`serde`/`serde_json`/`base64` were unconditional `[dependencies]` (not gated on the
`os-host-full` feature at all), and `zip` was `optional = true`, wired into the graph only via
`os-host-full = ["dep:zip", ...]` — which `space` unconditionally enables. `png`/`resvg`/
`tiny-skia`/`usvg` were **already** correctly narrowed to the native-host target table by an
earlier pass (`os-host-tier-split.md`) — not this pass's concern.

`base64` and `zip` needed two different fixes because they are two different shapes:

## (b) `base64` — guest-reachable, re-pointed at the first-party codec, not gated

Grepped every `base64::` use in `semio-framework-os`'s own source
(`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`, the file mounted unconditionally as `host_core` —
**not** behind the `os-host-full` feature, so it compiles for every consumer of this crate on
every target). 13 call sites across `backbone`, `instance`, `media_export_raster`,
`media_export_simple`, and `workflow` — all the same shape:
`use base64::Engine;` + `base64::engine::general_purpose::STANDARD.encode(...)`/`.decode(...)`.

These back `OsMediaExportResult`'s binary payload encoding — reached from `space`'s own
guest-dispatched command handlers (`🎮️commands/🖼️export-media`, `🎮️commands/🖼️import-media`,
`🎮️commands/🏙️bind-space-file`, …), the same `Editor::handle` command-table dispatch
`raster-tier-split.md` traced for animate's video export. Base64 encode/decode is target-neutral
pure computation — it needs no OS API on any target — so unlike a genuine host capability this is
not a tier-split candidate at all. The correct fix is case (b)'s other form: route through the
**existing, already-proven first-party interface** instead of gating anything.

`🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust` (`semio-framework-io-base64`) already
ships `base64_standard_encode`/`base64_standard_decode` — strict RFC 4648 standard-alphabet,
padded, byte-exact (`verified-outcomes.md`'s "base64 (RFC 4648) | 4/4 incl. RFC vectors + oracle
differential"), and is already `space`'s own dependency (aliased `base64_codec`, used today in
`🎮️commands/🖼️export-media/🦀️.rs:79`). `os-kernel`'s own `🏪️store/🦀️component.rs` had the
identical `pack_value_to_base64`/`pack_value_from_base64` bug and was fixed the identical way
(`os-kernel-host-crates-split.md`) — this is that same fix applied to the host crate `space`
actually links.

**Applied**: all 13 `base64::engine::general_purpose::STANDARD.encode(...)` →
`base64_codec::base64_standard_encode(...)`, all 5 `.decode(...)` →
`base64_codec::base64_standard_decode(...)`, all 9 now-redundant `use base64::Engine;` lines
deleted. `base64 = "0.22.1"` removed from `[dependencies]`; `base64_codec = { path = "...",
package = "semio-framework-io-base64" }` added, unconditional (it is pure Rust with zero
platform dependency, so it needs no target gate at all — correct on every target by construction).
No caller changed: every site already did `.map_err(|error| ...error.to_string()...)` or
`.expect(...)`, both of which work identically against `Base64Error` (implements `Display`) as
they did against `base64::DecodeError`.

**Disclosed behavioural difference: none.** `base64_standard_encode`/`_decode` implement the same
RFC 4648 standard, padded alphabet the `base64` crate's `STANDARD` engine did — this is a drop-in
byte-identical replacement, not a fallback.

## (a) `zip` — host-only, PROVEN zero production callers anywhere, gated

Grepped `zip::` across `semio-framework-os`'s full mounted input set. Exactly one production
site: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs`'s `//#region 🔖️Zip` (mounted into
`semio-framework-os` as `pub mod space`, gated `#[cfg(feature = "os-host-full")]` but — before
this pass — with no target gate at all). This is a **different** `🪐️space` from the plugin under
audit: it is the OS-kernel-adjacent framework module that defines `SpaceZipError`,
`export_collection_zip`/`import_collection_zip`, and their `ZipStoreBridge`
(`real_artifact_reader`/`real_blob_reader`/`import_document_artifact`/`import_blob`) — a portable
`.zip` byte-stream codec for a whole `CollectionSnapshot` (multi-artifact folder bundle), built
entirely from caller-injected, already-in-memory byte closures. Its own docstring says so
explicitly: *"IO-free: `read_artifact`/`read_blob` are injected so this crate never touches a
live store/filesystem itself."*

That IO-free-ness might suggest guest-reachability (no ambient filesystem needed), so this was
traced by caller, not assumed:

```
$ grep -rn "export_collection_zip\|import_collection_zip\|real_artifact_reader\|real_blob_reader\|\
    import_document_artifact\|import_blob" --include='*.rs' . | grep -v "🪐️space/🦀️.rs"
(no output)
```

**Zero callers anywhere in the repo outside the defining file itself** — not in `space` the
plugin, not in any other plugin, not in `semio-framework-os`'s own `host_core`
(`export_os_space_pack`/`import_os_space_from_pack`, the functions `space`'s guest-reachable
`💾️export-studio-pack`/`💾️import-space-pack-payload` commands actually call, delegate to
`store::parse_document_pack`/`store::ArtifactPackFiles` — os-kernel's own `.spk` pack container,
**not** the `zip` crate at all; confirmed by reading both functions' bodies). The only callers of
`export_collection_zip`/`import_collection_zip`/the `ZipStoreBridge` fns, anywhere, are this same
file's own `#[cfg(test)] mod tests` (`zip_export_import_round_trips_structure_and_bytes`,
`zip_export_import_export_is_byte_stable`, `zip_export_import_round_trips_real_store_documents_
and_blob`). The region's own docstring names its intended real caller as **"a live
`store::SpaceHost`'s registered members, W4's storage wave"** — future, native-host work, not
shipped, and not a WASI guest by that same docstring's own framing (a `SpaceHost` bundling
registered member documents into a portable archive is host/backup tooling, not something a
guest component does to itself).

This is the exact same proof shape `os-kernel-host-crates-split.md` used for `os_extension`'s
`zip` usage (".sxt package format", 3 call sites, all in the native host crate, zero guest
dispatch path) — PROVEN zero-guest-reachability, not assumed from "folder plugins probably don't
need archives in a guest."

**Applied**: whole-item `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` on all 15
top-level production items in the `//#region 🔖️Zip` (`SpaceZipError` + its 4 impls,
`ImportedCollection`, `zip_file_options`/`write_zip_file`/`read_zip_entry`,
`export_collection_zip`, `import_collection_zip`, `real_artifact_reader`, `real_blob_reader`,
`import_document_artifact`, `import_blob`) and on the 7 test-side items that exercise them
(`zip_fixture_bytes`, `TestBlobStore` + `test_blob_hash` + its `impl BlobStore`, and the 3
`#[test]` fns) — matching `raster-tier-split.md`'s per-item gating convention exactly (not a
wrapping module, since the file's own convention is flat `//#region` folding, not real nested
`mod`s). The file-top `use std::io::{Cursor, Read as _, Seek, Write as _}` (used only by this
region) got the same gate so it does not warn as unused on wasip2. `zip = { version = "2.4", ...,
optional = true }` moved from unconditional `[dependencies]` into
`[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` in
`semio-framework-os`'s `Cargo.toml`, alongside the already-gated `png`/`resvg`/`tiny-skia`/`usvg`.
The `os-host-full` feature's `"dep:zip"` entry is untouched — Cargo resolves an optional
dependency declared in a target table the same way regardless of which table it lives in.

**Disclosed behavioural difference: none observable today** — there is no shipped caller on
either target. If/when W4's storage wave adds a real host caller, it will compile exactly as
before on native/browser; a future guest caller would need the two-implementations pattern this
ticket used for `VelloRenderer` (an honest `Err` on wasip2), not attempted here since no such
caller exists yet to give it a contract.

## Files touched

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml` — `base64` removed, replaced
  by unconditional `base64_codec` (= `semio-framework-io-base64`); `zip` moved from unconditional
  optional to the native-host target table.
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` — all 13 `base64::engine::general_purpose::STANDARD`
  call sites re-pointed at `base64_codec::base64_standard_encode`/`base64_standard_decode`; 9
  `use base64::Engine;` lines deleted.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs` — 15 production items + 7 test items in
  `//#region 🔖️Zip` gated `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`; the
  `Cursor`/`Read`/`Seek`/`Write` import gated the same way with a docstring explaining why (per
  CLAUDE.md, every narrowed gate gets a WHY comment — `target_arch = "wasm32"` is true for wasip2).

## Deliberately left alone

- `serde`/`serde_json` in `semio-framework-os` — untouched. They are the common cross-plugin
  serde-tail this ticket explicitly fences as a separate later wave (`verified-outcomes.md`), not
  one of the 13 crates named in this brief, and present identically in all 5 sibling plugins'
  36-crate baseline.
- `png`/`resvg`/`tiny-skia`/`usvg`, `tokio`/`semio-framework-actor`/`semio-framework-plugin-host`/
  `semio-framework-async` — already correctly target-gated by an earlier pass
  (`os-host-tier-split.md`); re-verified present and unchanged, not re-touched.
- `export_os_space_pack`/`import_os_space_from_pack`/`export_os_space_dsl` (the functions
  `space`'s own guest-reachable pack/dsl-export commands actually call) — read and confirmed they
  never touch `zip::` at all (they delegate to os-kernel's own `.spk` pack container via
  `store::parse_document_pack`/`store::ArtifactPackFiles`), so nothing there needed any change.

## Verification

```
$ cargo metadata --no-deps            # exit 0, both edited manifests parse
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 --edges normal --prefix none \
    | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
    | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
36
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i {zip,base64,flate2,miniz_oxide,\
    zopfli,adler2,crc32fast,simd-adler32,bumpalo,cfg-if,displaydoc}
warning: nothing to print.        (×11, full output read each time, no truncation)
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i thiserror@1.0.69
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i thiserror@2.0.18
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i thiserror-impl@1.0.69
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i thiserror-impl@2.0.18
warning: nothing to print.        (×4, version-disambiguated since the bare name is ambiguous
                                    workspace-wide)
```

Foreground builds (`cargo check -p semio-s-plugin-space` native, `cargo build --lib --target
wasm32-wasip2 -p semio-s-plugin-space`) run under this ticket's documented heavy shared-target-dir
contention (`ps aux` showed multiple concurrent peer `rustc` invocations mid-run, e.g.
`semio_s_plugin_sourcing`) — results appended below once each completes; not fabricated ahead of
completion per this ticket's own "phantom blockers" warning.

<!-- BUILD_RESULTS_PLACEHOLDER -->
