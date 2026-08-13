# Wave M3e — `🎲️entropy` + `🌫️fuzzy` dissolution

**Duplication window: CLOSED.** Both `🧮️math/🎲️entropy` and `🧮️math/🌫️fuzzy` are deleted from disk (confirmed via `find`/`git status` below), their new stdio homes compile clean and their migrated tests pass. No stale mount remains anywhere. `semio-framework-math --all-targets` compiles clean.

## Re-verification of entropy's self-containment

Grepped the original 9,881-line `🧮️math/🎲️entropy/🦀️component.rs` for `crate::` and `use` before touching it:

- Every `use crate::entropy::...` (160 occurrences) resolves to entropy's **own** submodules (`counts`, `numeric`, `estimators`, `discrete`, `knn`, `continuous`, `divergence`, `mutual`, `pid`, `fisher`, `symbolic`, `regularity`, `ordinal`, `markov`, `multiscale`, `lz`, `fft`, `spectral`, `wavelet`, `matrix`, `inference`, `transfer`, `spatial`, `graph`, `ml`, `streaming`, `features`) — zero references to `crate::algebra`, `crate::optimize`, `crate::lie`, `crate::signal`, `crate::spatial` (math's top-level sibling), `crate::graph` (math's top-level sibling), or `crate::wfc`.
- Confirmed the earlier wave's correction: `pub mod spatial { ... }` (line 8736) and `pub mod graph { ... }` (line 8895) inside entropy are entropy's **own** measures — "🖼️ Image / spatial entropy over plain pixel slices (no image-decoding dependency)" and "🕸️ Graph entropy over plain edge lists (no graph-library dependency)" — both explicitly documented as taking raw `&[(u32,u32)]` / pixel slices, not any math-sibling type. No `crate::spatial::` or `crate::graph::` (top-level) reference exists anywhere in the file.
- Zero external crate dependencies: no `thiserror`, `serde`, `rand` — entropy ships its own `Xorshift64` PRNG, confirmed self-contained.
- Repo-wide grep for consumers (`grep -rn "::entropy::"` outside the file itself): **zero hits**, confirming zero coupling both ways (nothing in entropy reaches out, nothing reaches into entropy).
- False friend found and dismissed: entropy's own `regularity::fuzzy_entropy` (the FuzzyEn/ApEn/SampEn family, Chen et al.) is a same-named-by-coincidence entropy measure unrelated to `🌫️fuzzy`'s fuzzy-logic module — no coupling.

**`🌫️fuzzy`** was NOT self-contained: `use crate::algebra::{MatD, VecD}` and `use geometry::random::Rng` (aliased `semio_framework_geometry`). Both were already stdio dependencies (`Cargo.toml`), so the import rewrite was mechanical — until a concurrent wave (below) invalidated the `algebra` half mid-flight.

Repo-wide consumer grep for both, before touching anything: **zero** real hits for `math::entropy`/`math::fuzzy` outside their own files (a few coincidental word matches — "entropy-backed random seed" in `🌊️flow`'s math extension, JPEG "entropy coding", UI "fuzzy search" — none are the math modules).

## Placement decisions

**`🎲️entropy` → `✳️table/🧬️schema/🎲️entropy-internals/🦀️component.rs`** (Rust-only compute internals, no TS twin for the algorithm body) **+ a genuine inference at `💡️inferences/🎲entropy`.** Reasoning: entropy measures are derivations over data a subset already holds (a table column's value distribution) — not a new persisted content shape. A directory named `entropy` is not a content shape; per the binding placement rule, this is an inference over `✳️table`, not a 19th stdio subset. Mirrors `📊️statistics-internals` + `💡️inferences/📊moments` exactly (read before designing, per instructions).

**`🌫️fuzzy` → `✳️value/🧬️schema/🌫️fuzzy-internals/🦀️component.rs`, parked, no inference authored.** Zero consumers anywhere in the repo (re-verified, see above). Fuzzy sets/rule bases are authored content in principle, but with zero consumers there is no evidence for which plugin should own that content — inventing one would be exactly the "directory name is not a mechanism" error the ticket warns against. Placed under `✳️value` because fuzzy's atomic operation (membership-degree evaluation, defuzzification) takes a single scalar value, the closest existing content shape — but this is a judgment call under low signal, stated as such in the file's own header. If a plugin later owns fuzzy-rule-base content as authored data, that plugin should host the snapshot and call into these internals; stdio should not.

## The genuine inference: `ColumnEntropy`

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/🎲entropy/🦀️component.rs` (+ TS twin `🟦️component.ts`, mirroring `📊moments`'s pattern of an unwired boundary-vocabulary interface).

`impl store::InferredField<SemioTableSnapshot> for ColumnEntropy`:
- `FIELD_ID = "s.stdio.semio.table.inference.entropy"`, `SCHEMA_VERSION = 1`
- `reads() = ["columns", "rows"]`
- `plan()`: one step per **declared column, any kind** (unlike `📊moments`'s numeric-only gate — entropy is defined over any discrete symbol alphabet), key = column name, no parents
- `dep_input()`: this column's own non-null cell values, converted to a discrete symbol (`Bool`→`to_string()`, `Int`/`Float`→their `lexeme`, `Str`→`value`; `Null`/`Bytes`/`List`/`Map`/`Ref` excluded), counted into a `BTreeMap<String,u64>` (not `HashMap`) specifically so the serialized `Vec<u64>` — and therefore the `DepHash` bytes — is **deterministic across processes**, never dependent on hash-iteration order. This satisfies the hard determinism requirement for `InferredField`/`DepHash` caching.
- `compute()`: wraps `entropy_internals::estimators::entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Bits)`, defaulting to `0.0` on empty input (honest-default convention, matching `📊moments`'s `unwrap_or(0.0)`).
- Tests: honesty (fair-coin column = exactly 1 bit; constant column = 0 bits; every declared column present regardless of kind; empty snapshot → empty plan), cache-transparency law, and incrementality law in both directions (editing one tracked column misses only its own entry, proven for each of the fixture's two columns since — unlike `moments` — `entropy` has no untracked column to use as a zero-miss control; documented inline why the test shape differs from `moments`'s).

One bug found and fixed by me during verification: my first draft of the "unrelated column" test wrongly asserted zero new cache misses when editing `always_a`, forgetting that `entropy` (unlike `moments`) tracks *every* column, so editing a tracked column always misses its own entry. Rewrote it to assert exactly 1 miss (on `always_a` only) and that `coin`'s value is untouched — caught by the crate's own `--lib` run, fixed, re-verified green.

## Live foreign churn hit mid-verification — reacted, did not touch forbidden zones

While verifying, a **concurrent wave (M3d)** removed `algebra`/`optimize`/`lie`/`signal`/`spatial` from `semio-framework-math` entirely (moved into `📸️remodel`'s own schema, "verified as `📸️remodel`'s sole consumer" per its own comment in math's `glue.rs`) — a verification invalidated by my own concurrent, unfinished move of `🌫️fuzzy` into stdio, which M3d's session had no way to see. This broke `use semio_framework_math::algebra::{MatD, VecD}` in **three** stdio files: my new `🌫️fuzzy-internals`, and two **pre-existing** wave-M3c files (`📊️statistics-internals`, `🔗️causal-internals`) that had nothing to do with my wave but shared the same now-dangling import — a real, compiler-only-discoverable cross-wave coupling, exactly the kind the ticket's ORDERING section warns about.

I did **not** touch `📸️remodel`/`🏗️fem`/`🧩️wfc`/`💻️os/🖥️host`/`🧊️3d` (all explicitly off-limits). Instead, since `semio_framework_math::algebra` was gone and stdio (domain-neutral) depending on `📸️remodel` (domain-specific) would be the wrong layering direction, I extracted just `VecD`/`MatD` (190 lines, zero external deps, byte-identical to the deleted original, recovered via `git show HEAD:...` since the working tree had already deleted it) into a new **`✳️value/🧬️schema/➕️algebra-internals/🦀️component.rs`**, mirroring the exact same duplication precedent `🏗️fem` had already set for its own `➕️algebra`. Re-pointed all three broken files (`🌫️fuzzy-internals`, `📊️statistics-internals`, `🔗️causal-internals`) at it and documented why in each file's own header. This also separately mid-flight broke a math build I hit once via a **transient disk-full event** (unrelated `zerocopy` build-script `.o` truncated to empty mid-write by concurrent multi-wave cargo activity on the shared ticket `🎯️target`; fixed by removing just that crate's stale `debug/build/zerocopy-*` dirs, not touching anyone's source).

Also mid-flight (unrelated to me, another wave): a transient "no space left on device" during one build attempt self-resolved (disk went from 33Gi→128Gi avail between checks — other sessions' builds churning, not sustained exhaustion).

## Verification — real commands, real output

```
$ cd /Users/ueli/Documents/semio
$ touch "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs"
$ TD=".../🎯️target"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-stdio --all-targets
EXIT:0
error count: 0
(Finished dev profile [unoptimized] target(s) in 7m 21s — first run also surfaced and I fixed 137×E0433
+26×E0432 "cannot find entropy_internals" from an initial wrong self-reference path — entropy_internals
is nested under `crate::artifacts::semio::standards::v1::subsets::table::schema::entropy_internals`,
not `crate::entropy_internals` as in math's crate-root mount; fixed with one sed pass.)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-stdio --lib
EXIT:101 (see below — pre-existing baseline only)
test result: FAILED. 2951 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out; finished in 29.78s
failures:
    artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
    artifacts::dwg::standards::v_ac1018::subsets::any::schema::component::tests::conformance_laws::fixture_honesty_law
    artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
    artifacts::ifc::standards::v2x3::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law
    artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law
```
These 5 are **byte-identical** to the ticket's documented `scratch-w0-baseline-failures-sorted.txt` stdio baseline (5 failures, same names). Zero new failures. (An earlier run of this same command, before I fixed my own `changing_an_unrelated_column_does_not_miss` test bug, showed 6 failures — the 6th was `entropy::component::tests::changing_an_unrelated_column_does_not_miss`, my bug, fixed and re-verified above.)

```
$ touch "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📦️glue.rs"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-math --all-targets
EXIT:0
error count: 0

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-math --lib
EXIT:101
test result: FAILED. 773 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
failures:
    graph::dsl::tests::parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error
    graph::dsl::wire::tests::dag_from_wire_literal_rejects_unexpected_char
```
Both failures are in the ticket's documented 15-item math baseline (unrelated to entropy/fuzzy). Math's total test count (773) is far below the ticket's original 1296-passed baseline because **multiple concurrent waves** (M3c: statistics/tabular/probability/causal; M3d: algebra/optimize/lie/signal/spatial; this wave: entropy/fuzzy) have each removed their own slice since that baseline was taken — not attributable to this wave alone.

## Test-count arithmetic (entropy + fuzzy specifically)

- `🎲️entropy-internals`: 318 `#[test]` functions, moved verbatim, all passing in stdio (not in the failure list above).
- `🌫️fuzzy-internals`: 70 `#[test]` functions, moved verbatim, all passing in stdio.
- New `💡️inferences/🎲entropy`: 8 new tests (honesty ×4, cache-transparency ×1, incrementality ×3), all passing.
- Math side: `find "🧰️framework/🔨️modules/🧮️math/🎲️entropy"` / `.../🌫️fuzzy` → **directories do not exist**, confirmed deleted (both `git status` and `find`); zero dangling `#[path]` mounts remain in math's `glue.rs` (grepped, confirmed absent).

## Files touched

**Created:**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️entropy-internals/🦀️component.rs` (9,881 lines, moved verbatim from math, self-references rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/🎲entropy/🦀️component.rs` (new genuine `InferredField`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/🎲entropy/🟦️component.ts` (TS boundary-vocabulary twin, unwired, matching `📊moments`'s own precedent)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🌫️fuzzy-internals/🦀️component.rs` (2,449 lines, moved verbatim, imports rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/➕️algebra-internals/🦀️component.rs` (190-line `VecD`/`MatD` extract, unblocking three files hit by concurrent M3d churn)

**Edited:**
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (mounted `entropy_internals`, `fuzzy_internals`, `algebra_internals`, the new `entropy` inference module)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📊️statistics-internals/🦀️component.rs` (re-pointed `MatD`/`VecD` import off the now-gone `semio_framework_math::algebra`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs` (same re-point, 26 call sites)

**Deleted (by a concurrent session, verified not by me — see churn note above; both confirmed absent from disk and from math's `glue.rs` before I relied on that fact):**
- `🧰️framework/🔨️modules/🧮️math/🎲️entropy/🦀️component.rs`
- `🧰️framework/🔨️modules/🧮️math/🌫️fuzzy/🦀️component.rs`

## Honest remainders

- Neither `📊moments`-style inference nor `🎲entropy`-style inference is wired into the parent `SemioTableInference` aggregate struct or its five hand-rolled codecs — same explicitly-flagged, pre-existing remainder `📊moments` itself carries, out of scope here.
- `🌫️fuzzy-internals` has no `InferredField` — parked on a judgment call (see Placement decisions), open to revision when a real consumer appears.
- `➕️algebra-internals` (my new file) duplicates `VecD`/`MatD` a third time in the repo (math's original, now deleted; `📸️remodel`'s copy; `🏗️fem`'s own copy; and now stdio's). This is the established repo pattern for this exact situation (no single shared home after math's dissolution), not a mistake, but it is genuine duplication worth a future closer's attention if a real shared-math-primitives home gets designed.
- I did not independently re-verify wave M3d's or other concurrent waves' own work beyond what was necessary to unblock my own build; `semio-framework-math --lib`'s 773/2 test result reflects the cumulative state of ALL waves at time of my last check, not a claim about their correctness.
