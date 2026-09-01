# 🎯️ os-kernel host-runtime crates — tokio/zip/base64 closed

## Headline

`semio-framework-os-kernel` is depended on by every plugin, so this slice benefits all of them.
Classified all three named crates by grepping their actual usage (not just which file they live
in), same method as `wgpu-tier-split.md`/`raster-tier-split.md`. All three closed:

| plugin | third-party crates in wasip2 graph, before | after |
|---|---|---|
| `semio-s-plugin-draw-fsm` | **31** | **11** |
| `semio-s-plugin-flow` | **282** | **117** |

The 11 remaining for `draw-fsm` are 100% serde-tail (`serde`, `serde_core`, `serde_derive`,
`serde_json`, their proc-macro deps `proc-macro2`/`quote`/`syn`/`unicode-ident`, plus `itoa`/
`memchr`/`zmij` pulled in by `serde_json`) — explicitly out of scope per this ticket's fence
("serde removal is a separate later wave"). `zip`'s whole transitive tail (`flate2`, `miniz_oxide`,
`zopfli`, `crc32fast`, `adler2`, `thiserror`, `indexmap`, `hashbrown`) is gone too, since removing
`zip` itself removed everything downstream of it.

## Per-crate classification

### `tokio` — host-only, PROVEN

Searched every file `os-kernel`'s own `📦️glue.rs` mounts (`🗣️dsl`, `🎒️pack`, `📡️spr`, `🌿️vcs`,
`🪪️identity`, `📇️directory`, `🚪️io`, `🏪️store`, `⚙️engine`, `💡️inference`, `🧬️semio`,
`🧩️extension`) for literal `tokio::` usage. Exactly two hits, both already inside gates that
exclude `wasm32-wasip2`:

1. `📇️directory/🔌️client/🦀️component.rs`'s `pub mod browser` (`tokio::sync::mpsc::unbounded_channel`
   for a `web_sys::WebSocket` event bridge) — already
   `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`-gated at the module level. The
   file's own top docstring states outright: "tokio stays confined to
   `semio-framework-os-services`... this crate names no tokio type anywhere" (outside that one
   browser bridge).
2. `🏪️store/🔄️sync/🦀️component.rs` (`use tokio::sync::{broadcast, mpsc};` plus `tokio-tungstenite`,
   `#[tokio::test]`, `tokio::time::*`, `tokio::spawn`, `tokio::select!` throughout) — the WHOLE FILE
   is mounted only behind `#[cfg(feature = "sync")]` in `📦️glue.rs`, and its own top docstring
   states: **"WASI-P2 plugins never link this crate — inside the sandbox a store attaches vcs's
   pure `PortBackbone`... This actor is a host-side concern only."** No plugin manifest in the repo
   requests os-kernel's `sync`/`worker` feature (confirmed: `draw-fsm`'s and `flow`'s manifests
   depend on `semio-framework-os-kernel` with no `features = [...]`, so only `default = ["deflate"]`
   applies).

The `tokio` crate ITSELF, however, was declared in an **unconditional** `[dependencies]` entry
(`features = ["sync", "macros"], default-features = false`) — not gated by the `sync` cargo feature
at all, just given a smaller default feature set. Cargo links a dependency for every target it is
declared for regardless of whether the code paths that use it are compiled in, so this alone put
`tokio` (and none of its heavier features, since `rt`/`net` were feature-gated) into every wasip2
plugin's link graph. This is the actual bug: the crate-level dependency edge was unconditional even
though every real usage of it was already correctly code-gated.

**Fix**: moved the base `tokio = { features = ["sync", "macros"] }` entry out of `[dependencies]`
into a new `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table
— present on native and browser wasm32 (where the browser bridge above still needs it), absent only
from `wasm32-wasip2`. The pre-existing `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`
table (adds the `net` feature, kept off ALL wasm32 per its own docstring about tokio's build script
hard-erroring on `net` for wasm32) is untouched and still unions correctly for native.

### `zip` — host-only, PROVEN

Searched the same file set for `zip::` usage. Exactly one hit: `🧩️extension/🦀️component.rs`, whose
own top docstring says what it is: **"Runtime-installable `.sxt` extension package format — semio
binary envelope over a deterministic deflate zip."** Its `pub async fn pack`/`unpack`/`verify` are
the only exported surface, and a repo-wide grep for every caller
(`grep -rn "os_extension::\|extension::pack(\|extension::unpack(\|extension::verify(\|extension::content_hash"`)
found exactly three call sites, ALL in `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` —
`semio-framework-os`, the **native host crate** (never built for `wasm32-wasip2`; it is the host
that loads guest components, not a guest itself). No plugin, no guest-reachable command dispatch
path, and no other framework crate names this module. Installing/verifying a runtime extension
package is unambiguously host tooling: a WASI guest component has no reason to unpack its own
installer format.

**Fix**: gated the module mount itself in `📦️glue.rs` —
`#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] pub mod os_extension;` and its
`pub use crate::os_extension as extension;` re-export, same gate shape — and moved the `zip`
dependency into the same new host-only target table as `tokio` above. Whole-module gate, not a
per-symbol split, because the entire file (errors, manifest types, zip pack/unpack, `content_hash`)
exists only to serve those three host-only call sites; nothing in it is reachable from guest code.

### `base64` — guest-reachable, re-pointed at first-party codec, PROVEN

Searched the same file set for `base64::` usage. Exactly one hit, in `🏪️store/🦀️component.rs`
(unconditionally mounted, no cfg gate anywhere near it):
`pack_value_to_base64`/`pack_value_from_base64`, which wrap a `pk:`-prefixed base64 string around
pack-encoded bytes for "component-scene `*Json` string slots." This is core DSL/value-encoding
logic reachable by any plugin working with scene JSON fields through the store — not host-only.

The ticket's brief named the exact fix: the framework already owns
`🧰️framework/🔨️modules/🚪️io/🔤️base64/` (`semio-framework-io-base64`), whose own top docstring says
it is **"the sole runtime encoding any s-plugin needs, replacing the third-party `base64` crate"**
and is already consumed by `semio-framework-replication`, `semio-framework-mesh-engine`, and five
s-plugins (`remodel`, `raster`, `process`, `cad`, `space`, `draw`, `lowpoly`). Its public API
(`base64_standard_encode(impl AsRef<[u8]>) -> String`,
`base64_standard_decode(impl AsRef<[u8]>) -> Result<Vec<u8>, Base64Error>`) is a drop-in match for
`base64::engine::general_purpose::STANDARD.encode`/`.decode`.

**Fix**: added `semio-framework-io-base64` as an unconditional dependency, replaced both call sites
in `🏪️store/🦀️component.rs`, and removed the third-party `base64 = "0.22.1"` entry from
`Cargo.toml` entirely (not gated — deleted, since nothing else in the crate used it).

## Before / after `cargo tree` evidence

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i tokio
warning: nothing to print.
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i zip
warning: nothing to print.
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i base64
error: package ID specification `base64` did not match any packages
```
(the third result is the strongest possible proof for `base64` — it is not merely absent from the
wasip2 tree, it no longer resolves for ANY target in the workspace at all, since the third-party
crate was deleted from the manifest rather than gated.)

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 --edges normal --prefix none \
    | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
31   → 11

$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 --edges normal --prefix none \
    | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
282  → 117
```

Remaining 11 for `draw-fsm` (verbatim): `itoa`, `memchr`, `proc-macro2`, `quote`, `serde`,
`serde_core`, `serde_derive`, `serde_json`, `syn`, `unicode-ident`, `zmij` — all serde/serde_json
and its proc-macro/transitive tail (`zmij` traced via `cargo tree -i zmij` to a transitive dep of
`serde_json` itself, through `semio-framework-os-kernel-dsl-derive` and `semio-framework-pack`).
Zero relation to this pass's changes. Confirms the scope fence held: this pass touched zero serde
surface.

## Build results

- `cargo metadata --no-deps` — parses cleanly after all edits (manifest syntax valid).
- **`cargo check -p semio-framework-os-kernel` (native, PROVEN)** — first attempt hit a phantom
  `E0433: cannot find crate zip` in `🧩️extension/🦀️component.rs` after ~19 minutes queued on the
  shared, heavily-contended target-dir lock (many concurrent `cargo check`/`build` processes from
  other live sessions observed via `ps aux` throughout this pass — os-kernel is the most-contended
  crate in the repo per this ticket's own brief). Re-reading the on-disk `Cargo.toml` immediately
  after confirmed it was byte-correct (target table present, `zip` correctly placed) and a live
  `cargo tree -p semio-framework-os-kernel -i zip -e normal` at that same moment showed the edge
  resolving fine — textbook match for `verified-outcomes.md`'s documented "phantom blockers" class
  (a lock-blocked check can observe a transient state written by a concurrent peer edit mid-run,
  not this pass's own content). Re-ran clean: **`Finished dev profile [unoptimized] target(s) in
  6m 59s`, 0 errors**, 33 warnings, every one pre-existing shape (`unnecessary_qualification`,
  `non_shorthand_field_patterns`) at `🏪️store/🦀️component.rs` lines this pass did not touch.
- **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm` (PROVEN)** —
  `Finished dev profile [unoptimized] target(s) in 4m 40s`, **0 errors**. The plugin most exposed
  to this pass's `Cargo.toml` changes (it takes no extra os-kernel features, so it is the cleanest
  proof the base tokio/zip removal didn't need any feature it silently relied on) builds and links
  clean for the actual shipped target.
- **`cargo check -p semio-framework-os` (native host, PROVEN not-a-regression)** — the guardrail
  that exercises the now target-gated `extension::pack`/`unpack`/`verify`/`content_hash` (its only
  three call sites repo-wide, per the classification above). Exit non-zero: **781 errors, all 781
  in `semio-s-plugin-stdio`** — confirmed by extracting every unique `--> file:line` location in
  the full log (`grep -oE '^\s*-->\s*[^ ]+' | sort -u`): 16 unique locations, **100% under
  `✏️s/🔌️plugins/🗄️stdio/`**, zero under `🧩️extension`, `🏪️store`, this crate's `Cargo.toml`, or
  `📦️glue.rs`. This is `verified-outcomes.md`'s and `raster-tier-split.md`'s already-documented,
  actively in-progress, unrelated peer wave ("`🗄️stdio` ~563 real call-site files... last seen 2217
  errors mid-conversion" / "1404 modified files, an uncommitted peer session mid-refactor") — this
  run saw 781 (fewer than the 2218 the raster pass observed the same day, i.e. that peer session
  has been making forward progress, not regressing). `semio-framework-os` is a direct, unconditional
  dependency of `stdio`-dependent plugins on every target, so `cargo check` never reaches a clean
  exit while that unrelated wave is mid-flight — this is not something this pass introduced or can
  fix, and the file-location grep is the objective proof.

## Files touched

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — removed unconditional `base64` and
  `tokio` (`sync`,`macros` features) `[dependencies]` entries; removed unconditional `zip` entry;
  added `semio-framework-io-base64` to `[dependencies]`; added new
  `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table holding
  `tokio` (`sync`,`macros`) and `zip`; docstrings rewritten to explain each.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` — `os_extension` module mount and its
  `extension` re-export both gated `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `pack_value_to_base64`/
  `pack_value_from_base64` re-pointed from `base64::engine::general_purpose::STANDARD` to
  `semio_framework_io_base64::base64_standard_encode`/`base64_standard_decode`.

## Deliberately left alone

- `🎒️pack`, `📡️spr`, `🌿️vcs`, `🪪️identity`, `⚙️engine`, `💡️inference`, `🧬️semio` — grepped and
  confirmed zero `tokio::`/`zip::`/`base64::` usage; no changes needed.
- `semio-framework-os-services`/`semio-framework-actor`/`tokio-tungstenite`/`ureq` — already
  correctly `optional = true` behind the `sync`/`ureq`/`worker` features and/or the pre-existing
  `not(target_arch = "wasm32")` target table; untouched, out of this slice's scope.
- Serde/serde_json removal from os-kernel itself — explicit scope fence in this ticket's brief
  ("~150 references, hand-written `impl Serialize` blocks that must stay... a separate later
  wave"). Not attempted.

## What is proven vs. not proven — stated plainly

**PROVEN**: the classification of all three crates (grep evidence above, cross-checked against each
file's own docstrings which independently corroborate the same host/guest split); the `cargo tree
-i` results for `tokio`/`zip`/`base64` on `draw-fsm`'s wasip2 target (lock-free, cannot go stale);
the before/after third-party crate COUNTS for `draw-fsm` (31→11) and `flow` (282→117); `cargo
metadata --no-deps` parses the edited manifest without error.

**UNVERIFIED / not attempted**: an end-to-end `cargo build`/`cargo check` for `flow` or any
larger plugin at 0 errors — the `flow` 282→117 count comes from the lock-free, cannot-go-stale
`cargo tree -i`/count method only (the same method `wgpu-tier-split.md`/`raster-tier-split.md` treat
as their primary evidence), not a completed compile; a real compile of `flow` or `puzzle` for
`wasm32-wasip2` would additionally hit the concurrent `os-kernel`/`ToValue` cascade and `stdio`
waves this ticket's other research docs already document, unrelated to this pass. All three named
builds above (`os-kernel` native, `draw-fsm` wasip2, `semio-framework-os` native) DID complete and
are PROVEN, including the one (`semio-framework-os`) that does not exit 0 — its non-zero exit is
itself proven attributable to the unrelated `stdio` wave, not to this pass, by the file-location
grep.
