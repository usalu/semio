# Wave FIXALG — Report

## What moved and where

`MatG<T>`/`VecG<T>` (plus the inherent impl blocks that must travel with them under Rust's
"inherent impls only in the defining crate" rule — `ExactElimination`, `Charpoly`, `Smith`) were
relocated **out of** `📸️remodel`'s
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️component.rs`
**into** a new `pub mod algebra { … }` region added to
`🧰️framework/🔨️modules/🔢️number/🦀️component.rs` (crate `semio-framework-number`), right after the
existing `Interval` region, with a `pub use algebra::{MatG, VecG};` added to that file's existing
crate-root flatten block (alongside `Rational`, `Integer`, `ModInt`, etc.), so consumers write
`number::MatG` / `number::VecG` — consistent with how the crate already exposes everything else.

`lll_reduce` (LLL lattice basis reduction) was **not** moved — it's a free function, not an
inherent impl, has no second consumer anywhere in the repo, and is remodel-specific. It stays in
`➕️algebra-internals/🦀️component.rs`, rewritten to build on `number::VecG<number::Rational>`
instead of the old (broken) `math::number::…` path.

Both real consumers were repointed:
- `📸️remodel`'s own `➕️algebra-internals` (its `lll_reduce` + its one remaining test).
- `➗️mathematical`'s `🌿️cas-internals` (`apart`'s exact partial-fraction linear solve, and
  `matrix::SymMatrix`'s numeric `rank`/`nullspace`/`rref`/`solve_numeric` paths) — its 6 real
  `math::algebra::MatG`/`math::algebra::VecG` call sites now read `number::MatG`/`number::VecG`.

### Cargo/glue changes
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml`: removed the `math` (`semio-framework-math`)
  dependency, added `number` (`semio-framework-number`) — `math::number::…` is what used to resolve
  through `math`, and nothing else in remodel ever read `math::`.
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml` and `📦️glue.rs`: removed the
  `semio-framework-math` dependency and its `extern crate … as math;` line — it existed solely for
  the now-fixed `math::algebra::` call sites; nothing else in the crate ever read `math::`.
- `RUSTC_WRAPPER="" cargo metadata --no-deps --format-version 1` run after each Cargo.toml edit:
  `WORKSPACE_OK` both times.

Doc-comment accuracy passes (no code change): top-of-file docstrings in
`🧰️framework/🔨️modules/🔢️number/🦀️component.rs`, `➕️algebra-internals/🦀️component.rs`,
`🌿️cas-internals/🦀️component.rs`, mathematical's `📦️glue.rs`, and (one stray mention)
`📈️polynomial-internals/🦀️component.rs` updated to describe the FIXALG relocation and stop
pointing at the dissolved `math::algebra`/`math::number` paths.

## Proof: MatG/VecG exist in exactly one place

```
$ grep -rn "pub struct MatG\|pub struct VecG" --include="*.rs" /Users/ueli/Documents/semio
🧰️framework/🔨️modules/🔢️number/🦀️component.rs:3489:    pub struct VecG<T> {
🧰️framework/🔨️modules/🔢️number/🦀️component.rs:3540:    pub struct MatG<T> {
```

Zero hits anywhere else (`📸️remodel`'s `➕️algebra-internals` no longer defines them; only
references `number::VecG` inside `lll_reduce`).

## Verification (mandatory exact form, run once each)

`TD` = ticket target dir. `RUSTC_WRAPPER=""`, `--all-targets`, `touch`-first, all applied.

### `semio-framework-number` (gate: 79/0 baseline)
```
touch 🧰️framework/🔨️modules/🔢️number/🦀️component.rs
cargo check -p semio-framework-number --all-targets   → Finished, 0 errors
cargo test  -p semio-framework-number --lib            → test result: ok. 97 passed; 0 failed
```
97 = 79 baseline + 18 tests moved in from `➕️algebra-internals`'s old `exact_tests` module.

### `semio-s-plugin-mathematical` (gate: was 15 errors per ticket text; re-derived baseline)
Did **not** re-derive a clean baseline separately (touching the file to get a pre-fix count would
have meant reverting my own fix); the ticket's own description of the algebra errors was corroborated
directly: `math::algebra::MatG`/`math::algebra::VecG` at exactly 6 real code call sites (lines 3298,
3303 in `apart`; 5099, 5116, 5119, 5158 in `matrix::SymMatrix`) — not 10 as the ticket's summary
estimated; the other 4 mentions were doc-comment prose, not call sites.
```
touch 🌿️cas-internals/🦀️component.rs
cargo check -p semio-s-plugin-mathematical --all-targets
  → 9 errors remain, 0 of them algebra-related (confirmed by grep for "algebra" across the error log)
cargo test  -p semio-s-plugin-mathematical --lib
  → same 9 pre-existing errors block compilation; cannot run tests
```

### `semio-s-plugin-remodel` (gate: "measure before your first edit" — done)
Baseline measured before any edit: **47 errors**, 6 of them
(`E0432 unresolved import math::number` ×3, `E0433 cannot find number in math` ×3) matching the same
cross-wave hazard as `cas-internals` — `➕️algebra-internals` had never been repointed after wave
MATHEND moved `number` out of `semio_framework_math`, despite its own `Cargo.toml` comment claiming
otherwise (a second, independent staleness the ticket didn't call out but the "re-derive, don't
trust" instruction caught).
```
touch ➕️algebra-internals/🦀️component.rs
cargo check -p semio-s-plugin-remodel --all-targets
  → 41 errors remain (47 − 6), 0 mentioning number/math
cargo test  -p semio-s-plugin-remodel --lib
  → same 41 pre-existing errors block compilation; cannot run tests
```

## Test arithmetic (nothing lost)

`➕️algebra-internals/🦀️component.rs` test count: **84 → 66** (65 in the untouched `Tests` module +
19 in the old `ExactTests` module → 65 + 1 remaining `lll_recovers_a_short_relation`).
`84 − 66 = 18`, and `semio-framework-number` gained exactly 18 (79 → 97, confirmed by the passing
test run above). Every moved test (`rref_and_rank_hand_case`, `det_matches_cofactor_hand_case`,
`inverse_round_trips_to_identity`, `solve_matches_hand_solved_system`,
`nullspace_vectors_are_in_kernel`, `cayley_hamilton_holds_for_3x3`, `bareiss_det_matches_field_det`,
`bareiss_det_over_integer_matches_hand_case`, `berkowitz_over_modint_matches_brute_force_2x2`,
`smith_normal_form_divisibility_chain`, `vecg_basic_ops_match_hand_computation`,
`det_of_singular_matrix_is_zero`, `inverse_of_singular_matrix_is_none`,
`solve_of_singular_system_is_none`, `rank_bareiss_hand_cases_full_and_deficient_rank`, and the 3
`quick::*` random-matrix tests) now lives and passes in `semio-framework-number`.

## Attribution of the other errors (not mine, not fixed)

`semio-s-plugin-mathematical`'s remaining **9** errors and `semio-s-plugin-remodel`'s remaining
**41** errors are a different, unrelated shape entirely: `E0432`/`E0433`/`E0422` "cannot find module
X in commands" / "cannot find struct X in this scope" for a long list of command names
(`document`, `SetArtifact`, `set_directed`, `node_graph_edit`, `import_video`, `set_frame_cursor`,
`clear_dense`, `add_gcp`, …) — none of them mention `algebra`, `number`, `math`, `MatG`, or `VecG`.

Concretely, for mathematical: `🎛️apps/➗️mathematical/🦀️component.rs` line 16 expects
`commands::document::set_artifact`, but `📦️glue.rs`'s `commands` module mounts `set_artifact` etc.
flat (no `document` submodule) — a mismatch between an app-shell file and the module-mount wiring
that didn't originate from anything in this wave.

```
$ stat -f '%Sm %N' ✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🦀️component.rs
Aug 13 21:19:02 2026 …/🎛️apps/➗️mathematical/🦀️component.rs

$ git log -1 --date=iso -- ✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🦀️component.rs
dda7ceead1 2026-08-13 18:52:17 +0200 …
```

The file's on-disk mtime (21:19:02) postdates its last commit (18:52:17) — an uncommitted,
in-progress edit from a live concurrent session, matching this ticket's own warning about "the same
shape that hit cad earlier tonight." `semio-s-plugin-remodel`'s 41 errors are the identical pattern
(`crate::apps::remodel::commands::ingest`/`shell`/`calibration`/… unresolved, plus ~30
`cannot find module or crate X in this scope` for individual command names) — same foreign,
in-progress command-module rename, a different file but the same hazard class. Neither crate's
command-wiring files were touched by this wave; both are left exactly as found, per the ticket's
explicit instruction to fix only the `algebra` errors.

## Honest remainders

- Neither `semio-s-plugin-mathematical` nor `semio-s-plugin-remodel` can currently run their test
  suites end-to-end (`cargo test -p … --lib`) — both are blocked by the foreign command-wiring
  breakage described above, unrelated to and pre-existing this wave's edits. The algebra-specific
  fix is verified in isolation via `semio-framework-number`'s own passing test suite (97/0) and via
  `cargo check`'s error-count delta (mathematical: no algebra errors in the remaining 9; remodel: 47
  → 41, exactly the 6 algebra errors gone).
- `🏗️fem`'s own dense-basics `➕️algebra` module (a separate, pre-existing duplicate per wave M3d's
  own docstring) was left untouched — out of this wave's scope, not a consumer of either relocated
  symbol.
- `📈️polynomial-internals`'s local `det_bareiss` (operates on raw `Vec<Vec<C>>`, not `MatG<C>`) was
  left as its own deliberate, pre-existing duplication — not `MatG`/`VecG`, not introduced or removed
  by this wave, only its doc comment's stale `math::algebra` reference was corrected.

## Files touched

- `🧰️framework/🔨️modules/🔢️number/🦀️component.rs` — added `algebra` module (`MatG`/`VecG`/
  `ExactElimination`/`Charpoly`/`Smith` + 18 moved tests), crate-root re-export, docstring update.
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️component.rs`
  — removed `VecG`/`MatG`/`ExactElimination`/`Charpoly`/`Smith`; `lll_reduce` and its one remaining
  test repointed at `number::`; docstrings updated.
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml` — `math` dependency replaced with `number`.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️component.rs`
  — 6 `math::algebra::…` call sites repointed at `number::…`; docstring updated.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️component.rs`
  — doc-comment-only accuracy fix (no code change).
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml` — removed unused `semio-framework-math`
  dependency.
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` — removed `extern crate
  semio_framework_math as math;`; comment updated.
