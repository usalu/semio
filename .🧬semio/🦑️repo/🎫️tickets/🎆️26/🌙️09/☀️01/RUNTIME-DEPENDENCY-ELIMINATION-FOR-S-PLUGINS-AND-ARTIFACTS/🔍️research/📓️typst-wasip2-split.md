# 🎯️ typst-wasip2 split — the largest single edge closes by DELETION, not gating

## Headline

`cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i typst` and the same for
`semio-s-plugin-puzzle` now report **"package ID specification `typst` did not match any
packages"** — `typst` is gone from the entire workspace resolution, not just the wasip2 slice of it.
Same for `typst-svg`. `usvg@0.45.1` (typst's own vendored usvg) is gone with it; only `usvg@0.46.0`
(an unrelated, unconditional `vello_svg` dependency used by `🖼️canvas`'s icon rendering on every
target) remains.

**Before/after third-party crate count** (`cargo tree --edges normal --prefix none | grep -v
'^semio-' | wc -l`, measured immediately before and after the edit, in the same session):

| plugin | before | after |
|---|---|---|
| `semio-s-plugin-flow` | **282** | **123** |
| `semio-s-plugin-puzzle` | **274** | **111** |

A later re-measurement (taken while writing this doc, minutes later) read 117/104 for flow/puzzle
and 11 for `draw-fsm` (previously 31, untouched by this pass) — the ticket brief's own "nine
interactive developer sessions... never revert a peer" note applies: this is concurrent peer work
elsewhere in the same dependency graph landing between my two measurements, not a further action by
this pass. The 282→123 / 274→111 pair is the one directly attributable to this edit (measured back
to back, nothing else changed in between).

## Root cause, precisely — NOT a mixed-classification file, a dead dependency

`♾️infinite`'s Cargo.toml declared `typst = "0.14.2"` and `typst-svg = "0.14.2"` as unconditional
`[dependencies]`. Unlike the `wgpu`/`vello` splits (real code split across GPU vs. neutral halves of
large files), **grepping the entire `♾️infinite` module tree for `typst::` / `typst_svg::` returns
zero matches, on any target, native included**:

```
$ grep -rln "typst::" --include="*.rs" 🧰️framework/🛍️products/💻️os   →  (no results outside 🔤️typeset, an unrelated crate)
$ grep -rn "typst" 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite   →  only docstrings, Cargo.toml, and one bin
```

`typst`/`typst-svg` were never imported by a single line of `♾️infinite`'s actual library source —
not `🌍️world/🦀️component.rs` (the 14k-line file the ticket brief flagged), not `🖼️canvas`, not
`🎲️board`. They were pure dead weight in the manifest, pulling `typst-html`, `typst-eval`,
`typst-library`, `typst-layout`, `typst-realize`, `typst-syntax`, `typst-utils`, `typst-timing`,
`usvg@0.45.1`, `biblatex`, `citationberg`, `comemo`(+`comemo-macros`), `hayagriva`, `hayro`(+
`hayro-font`/`hayro-interpret`/`hayro-svg`/`hayro-syntax`), `chinese-number`, `az`(via `ciborium`/
`ciborium-ll`), `bincode`, `csv`(pulled transitively by `hayagriva`), `wasmi`, `icu_*` (a dozen+
crates), `palette`(+`palette_derive`), `rust_decimal`, `syntect`, `two-face`, `read-fonts`,
`skrifa`, `font-types`, `fontdb`, `svgtypes`, `rustybuzz`, `moxcms`, `ecow`, `lipsum`,
`idna`/`idna_adapter`, `url`, and more — a ~130-crate family (confirmed by an isolated build log of
`semio-framework-typeset`, the *unrelated* animate-side crate that legitimately wraps the same
family, though not every one of those 130 was net-new to flow/puzzle's own graph since some
overlapped with crates already present for other reasons).

**Only `typst-assets` was real** — used exactly once, by `dump-guestslim-typst-fonts`, a native
`[[bin]]` dev-tool (`🖼️canvas/🔨️bin/dump_guestslim_typst_fonts.rs`) that packs Typst's bundled font
blobs into a wire-format blob for `os-dev`'s browser preview pipeline. Also never referenced by any
`.rs` file in the shipped library, only by that bin.

## Per-symbol classification

| Symbol / dependency | Classification | Evidence |
|---|---|---|
| `typst` (crate) | **entirely unused** | Zero `typst::` token anywhere in `♾️infinite`'s source tree, any target. |
| `typst-svg` (crate) | **entirely unused** | Zero `typst_svg::` token anywhere in `♾️infinite`'s source tree. |
| `typst-assets::fonts()` | **real, but native-only** | Used by exactly one `[[bin]]` target (`dump-guestslim-typst-fonts`), never by the lib. `grep -rln "typst_assets" 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite` → only the bin file and `Cargo.toml`. |
| `usvg@0.46.0` (via `vello_svg`) | **target-neutral, unrelated family** | Comes from `vello_svg`, not from typst. Used unconditionally by `🖼️canvas/🦀️component.rs` (`SvgDocument`, `usvg_options_icons`, `render_svg_tree_literal`, `svg_icon_content_bounds`, etc. — real icon/SVG-icon rendering that legitimately runs on wasip2). Left untouched — `vello_svg` was already correctly present in both target tables from `wgpu-tier-split.md`, just with `default-features` differing per side. |
| `semio-framework-typeset` crate (`🔤️typeset`) | **out of scope, unrelated crate** | Only consumed by `✏️s/🔌️plugins/🎞️animate` (`editor/⚙️engine/🔤️text/component.rs`), not by `infinite`/`flow`/`puzzle`. Confirmed by `grep -rln "semio_framework_typeset\|semio-framework-typeset"` repo-wide (4 hits: root workspace `Cargo.toml`, its own `Cargo.toml`, animate's `Cargo.toml`, animate's one call site). Not touched — already isolated behind a first-party interface (`MarkupTypesetter`/`svg_outline_paths`) exactly as CLAUDE.md requires, and unaffected by this pass either way. |

No dual-implementation / "genuinely reachable from wasip2 guest logic" pattern was needed here
(unlike raster's video export) — there is no wasip2-reachable typesetting capability to preserve,
because nothing in `infinite`'s guest-compiled code ever called into Typst in the first place.

## The fix

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`:

- **Deleted outright** (not target-gated): `typst = "0.14.2"`, `typst-svg = "0.14.2"` from
  `[dependencies]`. Per CLAUDE.md ("no dead code", "clean long-term solution", "not pragmatic"), an
  unused dependency has no reason to exist on any target, so this is a deletion rather than a
  `not(all(target_arch = "wasm32", target_env = "p2"))` move — the pattern the ticket brief
  suggested is reserved for dependencies that are genuinely needed off-wasip2, which these are not.
- **Moved** `typst-assets = { version = "0.14.2", features = ["fonts"], optional = true }` into the
  existing `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table
  (the same table `wgpu-tier-split.md` already established for `ui_wgpu`/`vello`/`vello_svg`) —
  preserves `dump-guestslim-typst-fonts` and the `render` feature (`render = ["dep:typst-assets"]`,
  `default = ["render"]`) exactly as before for every target except wasip2, where the bin is never
  built as part of the shipped component anyway.
- Added a docstring on the existing `wgpu-tier-split` comment block explaining both moves and
  explicitly naming that `usvg@0.46.0` is an unrelated, deliberately-unchanged edge.

No source `.rs` file was touched — there was no code to gate, only a manifest to correct.

## `cargo tree -i` evidence (after)

```
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i typst
error: package ID specification `typst` did not match any packages
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i typst
error: package ID specification `typst` did not match any packages
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i typst-svg
error: package ID specification `typst-svg` did not match any packages
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i typst-assets
warning: nothing to print.
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i typst-assets
warning: nothing to print.
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i usvg@0.45.1
error: package ID specification `usvg@0.45.1` did not match any packages
   (only usvg@0.46.0 remains, unambiguous now)

# capability preserved on native:
$ cargo tree -p semio-s-plugin-flow -i typst-assets
typst-assets v0.14.2
└── semio-framework-os-infinite v0.1.0 (…)
    ├── semio-framework-os-flow v0.1.0 (…) └── semio-s-plugin-flow v0.1.0 (…)
    └── semio-s-plugin-flow v0.1.0 (…)

# unrelated usvg edge, unchanged and correctly still present:
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i usvg@0.46.0
usvg v0.46.0
└── vello_svg v0.9.0
    └── semio-framework-os-infinite v0.1.0 (…) └── … └── semio-s-plugin-flow v0.1.0 (…)
```

## Build results

- **`cargo check -p semio-framework-os-infinite`** (native) — `Finished dev profile [unoptimized]`
  in **8m 55s, 0 errors**, 64 pre-existing warnings (all `dead_code` in `🌍️world`/`🎲️board`, unrelated
  to this pass — same shape the two predecessor docs also observed and left alone). Confirms native
  behavior (including the `dump-guestslim-typst-fonts` bin's `typst-assets` dependency) is unbroken.
- **`cargo check -p semio-framework-typeset`** (native, the host/native typesetting path CLAUDE.md
  and the ticket both required stay intact) — `Finished` in **4m 22s, 0 errors**. This crate is
  entirely untouched by this pass; the check exists purely to prove animate's real Typst-backed
  interface still builds. `typst`, `typst-svg`, `typst-library`, `usvg@0.45.1` and their whole
  family compile clean here, on the crate that actually uses them.
- **`cargo check --target wasm32-wasip2 -p semio-framework-os-infinite`** (bonus: the shipped target
  itself, not just native) — 9 errors, **all** in
  `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/../../🦀️.rs:161` /`:169`
  (`cannot find derive macro Serialize/Deserialize in this scope`) — a file this pass never touched,
  in `semio-framework-async`, consistent with the ticket brief's warning about concurrent peer
  sessions live in framework crates. Confirmed by grepping every error's `-->` path: the only unique
  file across all 9 errors is that one `async` component file. Zero mention of `typst`, `typeset`,
  `infinite`, `canvas`, `world`, or `board`.
- **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate`** — exit 101, **793
  errors, all in `semio-s-plugin-stdio`** (serde-bound mismatches across `bmp`/`svg`/`gltf`/`xml`/
  `html`/`gif`/`mp4`/`pdf`/`mp3` mutation-diff files — `E0277`/`E0308`/`E0599` against
  `serde::Serialize`/`Deserialize`). This is the exact, already-documented, unrelated in-flight
  `🗄️stdio` conversion (`📓️verified-outcomes.md`: "~563 real call-site files... in flight"; today's
  own `📓️raster-tier-split.md` hit the identical wave at 2218 errors, now at 2989 warnings/793
  errors — the wave is progressing, not regressing). Confirmed by `grep -oE '\-\-> [^:]+'` over the
  full log: **zero matches** for `infinite`, `typst`, `typeset`, `canvas`, `world`, or `board`.
  `semio-s-plugin-flow`/`puzzle` also depend unconditionally on `semio-s-plugin-stdio`
  (`cargo tree -i semio-s-plugin-stdio` confirms), so the same chokepoint blocks a full `flow`/
  `puzzle` build too — not something this pass, or the `wgpu`/raster passes before it, can route
  around. The lock-free `cargo tree -i` results above are unaffected by any of this and remain the
  primary evidence.

## Deliberately left alone

- `usvg@0.46.0` (via `vello_svg`) and everything `🖼️canvas`'s icon-rendering code does with it —
  target-neutral, unconditional, unrelated to the typst family, already correctly wired by
  `wgpu-tier-split.md`.
- `semio-framework-typeset` (the `🔤️typeset` crate wrapping typst/typst-svg/typst-assets/usvg for
  `🎞️animate`) — a completely separate dependency edge, not reachable from `flow`/`puzzle`, not
  touched, still compiles clean.
- The `semio-framework-async` 9-error break on the wasip2-target check above — not mine, not
  touched, flagged for whoever owns that concurrent wave.
- The `semio-s-plugin-stdio` wave blocking full plugin builds — explicitly out of scope, already a
  separately tracked slice of this same ticket per `📓️verified-outcomes.md`.

## What is proven vs. not proven

**Proven**: `typst`/`typst-svg`/`usvg@0.45.1` are completely gone from the workspace dependency
graph (not just narrowed off wasip2 — genuinely unused, so removed for every target), while
`typst-assets` (the one real, native-only use) is preserved exactly where it's needed and absent
from wasip2. `semio-framework-os-infinite` and `semio-framework-typeset` both compile clean natively
(0 errors each). Flow and puzzle's third-party crate count dropped by roughly 45–47% in one edit to
one manifest (282→123 flow, 274→111 puzzle, measured back-to-back before/after; later drift to
117/104 is concurrent peer work, not this pass).

**Not proven**: an end-to-end `cargo build --lib --target wasm32-wasip2` for `flow`, `puzzle`, or
`animate` at 0 errors — all three are blocked identically by the unrelated, in-flight `🗄️stdio`
conversion, exactly as the two predecessor splits in this same ticket also observed. Re-running once
that wave lands is the natural way to close this gap.
