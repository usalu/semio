# crate budget I (wave 2): over-gate splits

No cargo/rustc, no git writes. Scripts: `🧰️scripts/crate-budget/i_*.py`. `freeze.txt` was absent for the whole run (en1994 included).

## Splits (public paths kept by re-exports)
- `en1994-scene` (238 KB / 4009 lines, 15 files) -> new `en1994-scene-base` (root `🎬️scene/🧱️base`: statics, facts, layout, kit, handles; `pub(crate)`/`pub(super)` -> `pub`). Decoupling edits: `LAYER_MODE_BUCKLING/LTB` moved from `tracks` into `kit`; `kit::finish` no longer calls `tracks::finish`, `En1994Kit::finish` calls it after. The scene root re-exports `facts`, `handles`, `layout` and imports `kit`, `statics`.
- `en1992-mut-geometry-tendons-studs` (37 files) -> `-more` (6 mounts); `en1994-mut-beam-fields`, `-beam-steel`, `-column-material` (31 files) -> `-more` (5 mounts each), via `i_mut.py` (split_mounts).
- `mechanics` (101 %: 4052 lines) -> new `semio-s-plugin-norm-mechanics-applied` (root `🧮️mechanics/🧱️applied`: fastening, geotechnics); kernel re-exports both modules and depends on the new crate (kernel Cargo.toml gained a `[dependencies]` section). Registered in norm `Cargo.toml` (member + workspace dependency) and all 14 locks that know mechanics.
- Registered: workspace members/deps, Cargo.lock blocks (artifact locks, hub, fixture-sweep, bridge, norm), en1994/en1992 `script.ts`.

## Gates
- Source-budget: 4 over (was 6 + mechanics + 3 en1994 near-limit). Remaining OVER are generated twins only: `en1998-diff`, `en1998-model-json-schema`, `en1991-schema`, `en1991-model-json-schema` (pretty-printed regenerated JSON twins counted via `include_str!`); recorded in `📌️important/requests.md` under the codegen request, not hand-edited (coordinator line 1938 forbids the H reflow).
- Still above 90 %: `en1993-model` 92 % (needs the schema-codegen decision from H's report). 85-90 %: structural-scene-foundation, en1990/1992/1996 scene, en1991-inferences, vdi3805-inferences, en1995-kernel, en1998-mutations, several `mut-*` at 28 files.
- `check-cargo-authority.py` 31 crates 0 errors; `repo ./📜️script.ts verify build-owners sync` passed (added=1); tomllib over 254 Cargo.lock files and all Cargo.toml in the touched trees OK; `#[path]`/`include_str!` targets exist in mechanics, en1994, en1992 (0 missing).

## Unverified by compiler
`pub(crate)` -> `pub` widening in the base/applied crates, `use super::*` in `kit` (now sees only the base root's externs), glob/re-export ambiguity of `pub use base::{facts, handles, layout}` next to `pub use handles::handles`, features/dev-dependencies (none needed: base and applied have no features; tests stay in the hosts).

## Ready
`- ready for rebuild: crate budget I` appended to every `build-en19*/latest.md`.
