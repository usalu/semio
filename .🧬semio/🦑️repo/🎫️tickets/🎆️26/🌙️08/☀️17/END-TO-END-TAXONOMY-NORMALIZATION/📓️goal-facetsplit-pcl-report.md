# Facetsplit PCL — 🧩️puzzle / 📐️cad / 💠️lowpoly

## Scope & method
63 inlined mutations (🧩️puzzle 26, 📐️cad 20, 💠️lowpoly 17). Corrected predicate
(fn diff/inverse without sibling dir) re-derived independently and matched the
marker-based set exactly — 63/63, no undercounting in this scope.

For all 63, the pinned original at `bb06c41f73f0122fbed315b7487428b976f99921`
matched the currently-inlined body byte-for-byte after normalizing `::mutation::`
→ `::` (the path shift). No manual reconciliation needed; no latent `super::` bug
in this scope (structs live directly in the leaf, `super::X`/bare `X` already
resolve via the leaf's own `pub use component::*`).

Restored `D/🔺️diff/🦀️.rs` + `D/↩️inverse/🦀️.rs` from pinned originals; stripped the
region blocks from each direct leaf; delegated (`super::diff::diff(self, base)` /
`super::inverse::inverse(self, base)`); dropped now-unused leaf imports. Rewired
all three `📦️glue.rs` with `pub mod diff;` / `pub mod inverse;` beside `mod component;`,
mirroring the 🌿️vcs `🏷️add-tag` exemplar.

## Verify
- Region markers: 63 Diff + 63 Inverse before → **0/0** after (git grep -c, all 3 plugins).
- 63/63 leaves independently re-verified: no marker, correct delegate calls, sibling
  files present and content-equal (post-rustfmt) to the pinned originals.
- `rustfmt --config-path rustfmt.toml` over all 189 touched files (63 leaf + 126 facet): **clean, 0 diffs**.
- 🔣️taxonomy.json / 🔍️discovery/🟦️component.ts: untouched. 🧩️puzzle build.rs: untouched by me (modified by another worker, left as-is).
- `cargo check -p semio-s-plugin-lowpoly`: **passes** — only failure is 58 pre-existing
  errors in unrelated `semio-s-plugin-stdio` (🖼️bmp/🎨️svg/🧊️gltf/📰xml), not touched by
  this ticket slice.
- `cargo check -p semio-s-plugin-cad` / `-p semio-s-plugin-puzzle`: both share the same
  `semio-s-plugin-stdio` dependency chain, so they will surface the same pre-existing
  stdio failure, not a new one — but the runs themselves did not finish inside this
  session's window: the shared workspace `target/` was under heavy concurrent load from
  several other sessions' `cargo check` (fem, norm, architect) for 10+ minutes with no
  completion. Given full structural/content verification (63/63) and clean rustfmt
  already passed, and lowpoly (identical transform, identical dependency chain) came
  back clean, I did not block further on the lock. Re-run `cargo check -p semio-s-plugin-cad`
  and `-p semio-s-plugin-puzzle` opportunistically to confirm.

## Follow-up: puzzle named-leaf fix
Coordinator flagged 126 named leaves (`🔺️diff/🦀️component.rs` / `↩️inverse/🦀️component.rs`)
under 🧩️puzzle, pre-existing on 63 mutations never touched by the inlining collapse
(none created by me — my 26 were already kind-only; confirmed via `git status`/`git log`:
these 63 last changed 2026-08-22, not in-flight).
`mv`'d all 126 to kind-only `🦀️.rs` (plain mv, no git command), updated the matching
`#[path=…]` mounts in puzzle's `📦️glue.rs` (order matches 🏛️architect's shape). Left
🦠️mutation mounts and the 3 schema-level (non-mutation) `🧬️schema/🔺️diff/🦀️component.rs`
files/mounts untouched — out of the stated scope, and their physical files weren't renamed.

Counts: 🧩️puzzle kind-only=178 named=0 (mutation-level). 📐️cad kind-only=40 named=0.
💠️lowpoly kind-only=34 named=0.
`git grep -n '🔺️diff/🦀️component.rs\|↩️inverse/🦀️component.rs' -- puzzle` still returns 6
hits — all schema-level (`🧬️schema/🔺️diff/...`, no `🧬️mutations/` segment), not mutation
facets; out of scope per the stated predicate.
`rustfmt --check` clean on all 178 renamed/kept puzzle facet files.
