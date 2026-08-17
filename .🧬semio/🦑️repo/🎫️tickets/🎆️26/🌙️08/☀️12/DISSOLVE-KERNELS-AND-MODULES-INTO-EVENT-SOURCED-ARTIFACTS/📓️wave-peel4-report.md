# Wave PEEL4 Report — last brep batch (foundations)

## Cycle re-verification (grep, real code only — doc-comment `[links]` excluded)

Both cycles claimed in the brief were **re-derived and found to be illusory in real code** — the
only real edges are doc-comment cross-references (`[\`crate::brep::X\`]`), which are not compiled
dependencies. Grepping `crate::brep::` restricted to non-comment lines gave:

- `{vec, tolerance, predicates}`: **no cycle**. Real edges: `matrix -> vec`, `predicates -> vec`.
  `tolerance` has zero real deps; `vec` has zero real deps. `tolerance -> predicates` only exists
  as a doc link.
- `{polynomial, bspline, curve, surface}`: **no cycle**. Real edges: `bezier -> vec`,
  `bspline -> bezier, vec, poly` (test-only), `curve -> bspline, mat, vec`,
  `curve_ops -> bspline, curve, mat, vec`, `surface -> bspline, mat, vec`,
  `surface_ops -> surface, vec, mat` (test-only `mat`). `polynomial` has zero real deps on bezier/
  bspline (docstring only). `bspline -> curve`/`bspline -> surface` are doc-comment-only.

Since the destination is a **single target crate** (stdio) for all twelve modules, intra-batch
ordering never mattered anyway — the only binding constraint was the crate-direction law (nothing
left behind in `semio-framework-3d` may reference something moved to stdio). Confirmed by
repo-wide grep: **zero non-stdio, non-`📐️brep` production consumers** of
`semio_framework_3d::brep::{vec,mat,tolerance,predicates,poly,bezier,bspline,curve,curve_ops,
surface,surface_ops,error}` exist anywhere in the repo. All external consumers of `brep::engine`/
`brep::kernel` (flow extension, cad plugin, `os`/`brep-geometry`, `os` host) touch **only**
`engine`/`kernel`, never the twelve foundation modules — so the whole batch moved in one shot.

## Batch: 12 foundation modules → stdio `✳️brep/🧬️schema/📸️snapshot`

| Source (`📐️brep/…`) | New home | Mount |
|---|---|---|
| `➡️vector` | `📸️snapshot/➡️vector/🦀️component.rs` | pre-mounted stub, replaced |
| `🔢️matrix` | `📸️snapshot/➡️vector/🔢️matrix/🦀️component.rs` | **new local mount** under vector (`#[path]`, no glue.rs edit) |
| `📏️tolerance` | `📸️snapshot/📏️tolerance/🦀️component.rs` | pre-mounted stub, replaced |
| `⚖️predicates` | `📸️snapshot/➡️vector/⚖️predicates/🦀️component.rs` | **new local mount** under vector |
| `〰️polynomial` | `📸️snapshot/〰️polynomial/🦀️component.rs` | pre-mounted stub, replaced |
| `🎢️bezier` | `📸️snapshot/➰️curve/🎢️bezier/🦀️component.rs` | **new local mount** under curve |
| `🪢️bspline` | `📸️snapshot/➰️curve/🪢️bspline/🦀️component.rs` | **new local mount** under curve |
| `➰️curve` | `📸️snapshot/➰️curve/🦀️component.rs` | pre-mounted stub, replaced |
| `✂️curve-ops` | `📸️snapshot/➰️curve/✂️curve-ops/🦀️component.rs` | **new local mount** under curve |
| `🏄️surface` | `📸️snapshot/🏄️surface/🦀️component.rs` | pre-mounted stub, replaced |
| `🪡️surface-ops` | `📸️snapshot/🏄️surface/🪡️surface-ops/🦀️component.rs` | **new local mount** under surface |
| `🚨️error` | `📸️snapshot/🚨️error/🦀️component.rs` | pre-mounted stub, replaced |

Local mounts for `matrix`/`predicates`/`bezier`/`bspline`/`curve-ops`/`surface-ops` (no pre-mounted
stub existed) were added via `#[path]` inside the already-glue-mounted parent files (`vector`,
`curve`, `surface`), mirroring the `⚙️engine → 📄️step/📦️mesh-io` local-mount pattern from wave
PEEL3 — **no edit to stdio's `📦️glue.rs` was needed** for the twelve modules.

### Parity — file line counts (pre/post move, identical, `cp`-based move)
`vector` 398, `matrix` 348, `tolerance` 235, `predicates` 344, `polynomial` 469, `bezier` 325,
`bspline` 414, `curve` 400, `curve-ops` 464, `surface` 314, `surface-ops` 214, `error` 183 —
**4108 LOC total, byte-identical bodies**, only `use`/path rewrites and prepended "moved from"
docstring lines changed.

### Parity — `#[test]` count, per file, before vs. after
All twelve files: `vector` 10, `matrix` 12, `tolerance` 9, `predicates` 11, `polynomial` 18,
`bezier` 8, `bspline` 13, `curve` 10, `curve-ops` 10, `surface` 9, `surface-ops` 6, `error` 4.
**Total 120 before → 120 after, exact per-file match.**

### Distinctive-symbol spot check (before delete / after move)
`Vec3`/points in `vector`, `Mat3`/`Trsf` in `matrix`, `Tol` in `tolerance`, predicate fns in
`predicates`, `Bernstein`/`Poly` in `polynomial`, `RationalBezier2/3` in `bezier`, `KnotVector` in
`bspline`, `Curve3` in `curve`, `arc_length`/`split` in `curve-ops`, `Surface` enum in `surface`,
`closest_point` in `surface-ops`, error enum in `error` — **all present in the new location**
before the old `📐️brep/{these 12 dirs}` were `rm -rf`'d.

### Import rewrites
- Internal cross-module refs rewritten to `super::…` (parent/child) or the full
  `crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::…` path (cross-tree),
  matching each new nesting depth. Caught and fixed two depth-counting mistakes during
  verification: `curve-ops`'s test module needed `super::super::bspline` (not `super::bspline`,
  since `super` inside `tests` is `curve_ops` itself); `bspline`'s test module's inline
  `super::bezier::…` needed the same `super::super::` fix.
- 14 stdio consumer files (`⚙️engine`, `📄️step`, `📦️mesh-io`, `💡️inferences/*`, `📸️snapshot/🕸️topology`,
  `🔺️diff/*`) had their `use semio_framework_3d::brep::{vec,mat,tolerance,predicates,poly,bezier,
  bspline,curve,curve_ops,surface,surface_ops,error}::…` rewritten to the internal
  `crate::artifacts::…::snapshot::…` paths (including two bare `use …::curve_ops;`/`…::surface_ops;`
  module imports, and one stray doc-comment link).
- Added `semio-framework-number` as a new dependency in stdio's `Cargo.toml` (`✏️s/🔌️plugins/
  🗄️stdio/📦️packages/🦀️rust/Cargo.toml`) — `⚖️predicates` needs `semio_framework_number::Rational`
  for its exact-arithmetic escalation path, and stdio didn't previously depend on that crate
  (only `semio-framework-math`, which does not export `Rational`; rustc's own "similar name"
  suggestion was wrong and would not have compiled).
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs`: removed the 12 `#[path]` mounts for
  `error/vec/mat/tolerance/predicates/poly/bezier/bspline/curve/curve_ops/surface/surface_ops`
  inside `pub mod brep { … }`, keeping only `engine`. Doc comment on the `brep` module updated to
  explain the split.
- Deleted the 12 now-empty source directories under `🧰️framework/🔨️modules/🧊️3d/📐️brep/`.

## Verification output (run once per crate, after fixing the 3 real compile errors below)

```
TD=".../🎯️target"
touch 🧰️framework/…/📦️glue.rs
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-3d --all-targets
  → Finished `dev` profile [unoptimized] target(s) in 1.62s   (clean, only unrelated pre-existing warnings elsewhere in the workspace)
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-3d --lib
  → test result: ok. 62 passed; 0 failed          (182 baseline − 120 moved brep tests = 62 mesh tests, exact)

touch ✏️s/🔌️plugins/🗄️stdio/…/📦️glue.rs
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-stdio --all-targets
  → Finished `dev` profile [unoptimized] target(s) in 0.89s   (clean after fixes, below)
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-stdio --lib
  → test result: FAILED. 3379 passed; 5 failed; 4 ignored
     (3259 baseline + 120 moved brep tests = 3379, exact; the 5 failures are byte-identical in name
     to `scratch-w0-baseline-failures-sorted.txt`'s pre-existing stdio failures — binary/extent,
     dwg/ac1018 fixture-honesty, dxf/r12 bounds, ifc/2x3 fixture-honesty, zip/entries — none touch
     brep. Zero regressions, zero new failures.)
```

### Real compile errors hit and fixed during first `cargo check -p semio-s-plugin-stdio`
1. `⚖️predicates` (now under `➡️vector`): `use semio_framework_number::Rational;` unresolved —
   stdio's `Cargo.toml` had no such dependency. Fixed by adding
   `semio-framework-number = { path = "…", package = "semio-framework-number" }` to stdio's
   `Cargo.toml` (matching the existing dependency-declaration style there).
2. `✂️curve-ops`'s test module: `use super::bspline::de_boor;` — wrong depth (`super` inside
   `tests` is `curve_ops`, not `curve`). Fixed to `use super::super::bspline::de_boor;`.
3. `🪢️bspline`'s test module: two inline `super::bezier::RationalBezier2::…` calls — same
   depth mistake. Fixed to `super::super::bezier::…`.

All three were caught by the mandated `cargo check --all-targets` (not just `--lib`) before any
test run, exactly as the depth-sensitive local-mount pattern predicts.

## Is `📐️brep` gone? **No — and it should not be.**

`📐️brep/` now contains **only** `⚙️engine/🦀️component.rs` (67 LOC, unchanged). It could not be
emptied because `⚙️engine` has real, current framework-side production consumers that the
crate-direction law forbids serving from stdio:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs` —
  `use semio_framework_3d::brep::engine::{ParamDomain, PointClassification, Vec3};`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs` — same three types.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/{🚪️io/🗺️geometry-import,
  🧬️schema/💡️inferences}/🦀️component.rs` — `Vec3`, `MeshTransfer`.

`os` (`🧰️framework/🛍️products/💻️os`) is framework/product code — verified it does **not** depend
on `semio-s-plugin-stdio` in its `Cargo.toml` (plugins depend on framework/os, never the reverse),
so `brep-geometry` cannot reach `engine` if it moved to stdio. `engine` therefore has a legitimate
reason to survive in `semio-framework-3d`, exactly as `⚙️engine`'s own docstring already documents
for the `BrepKernel`/`kernel` half (out of scope, untouched — confirmed zero references to
`brep::kernel` inside any of the twelve foundation modules or in `⚙️engine` itself; the `os`
component.rs file's `semio_framework_3d::brep::kernel::…` calls are pre-existing, unrelated to this
wave, and were not touched).

`📐️brep/` itself (the directory) also survives for the same reason — it holds `⚙️engine`, which is
real, live, in-scope code, not dead weight. Deleting the directory was correctly declined per the
brief's own instruction to check actual contents before any `rm -rf`.

## Fate of the `⚙️engine` leaf
Stays exactly where it is (`🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs`, 67 LOC,
untouched). Census of `MeshTransfer`/`Vec3`/`ParamDomain`/`PointClassification` consumers found
three framework-side files (above) plus stdio's own many consumers (already reaching it via the
pre-existing `stdio → semio-framework-3d` forward edge, unaffected by this wave). Framework-side
consumers remain — this is the brief's own named legitimate reason for it to survive, confirmed by
grep, not assumed.

## Honest remainders
- `📐️brep/` is down to 67 LOC (`⚙️engine` only), from the wave-start 4175 LOC / 13 subdirs. The
  headline goal ("`📐️brep` must not exist anymore") is **not fully met** — `⚙️engine` is a real,
  necessary survivor with live framework-side consumers, not an oversight. Fully dissolving it
  would require either duplicating `engine`'s types into stdio (rejected — "NO CODE LOST OR
  DUPLICATED") or rewiring `os`/`flow`/`cad` to depend on stdio (a crate-direction violation, and
  explicitly out of scope: `BrepKernel` and its consumers are "its own wave" per this brief).
- `BrepKernel`/`brep::kernel` — untouched, as instructed. Not investigated further beyond
  confirming it shares zero code with the twelve moved foundation modules.
- `semio-framework-ui`'s pre-existing lib-test failures — not touched, not gated on, per brief.
- Did not re-run `df -h` mid-verification (only at start, 121Gi free) since no error output looked
  disk-related; both crates compiled and ran tests to completion without anomalies.

## Files touched
**New:**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➡️vector/🔢️matrix/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➡️vector/⚖️predicates/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🎢️bezier/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/🪢️bspline/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/➰️curve/✂️curve-ops/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/🏄️surface/🪡️surface-ops/🦀️component.rs`

**Replaced (stub → real content):**
- `.../📸️snapshot/➡️vector/🦀️component.rs`, `.../📏️tolerance/🦀️component.rs`,
  `.../〰️polynomial/🦀️component.rs`, `.../➰️curve/🦀️component.rs`, `.../🏄️surface/🦀️component.rs`,
  `.../🚨️error/🦀️component.rs` (all under the same `✳️brep/🧬️schema/📸️snapshot` base)

**Edited (import rewrites for the moved-module fallout):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📄️step/🦀️component.rs`
- `.../⚙️engine/📦️mesh-io/🦀️component.rs`, `.../⚙️engine/🦀️component.rs`
- `.../💡️inferences/✅validation-report/🦀️component.rs`, `.../💡️inferences/🌳bounding-volume/🦀️component.rs`,
  `.../💡️inferences/🏷classification/🦀️component.rs`, `.../💡️inferences/📏mass-properties/🦀️component.rs`,
  `.../💡️inferences/🧩tessellation/🦀️component.rs`
- `.../📸️snapshot/🕸️topology/🦀️component.rs`
- `.../🔺️diff/↔️offset/🦀️component.rs`, `.../🔺️diff/✂️intersect/🦀️component.rs`,
  `.../🔺️diff/➡️sweep/🦀️component.rs`, `.../🔺️diff/🎨️blend/🦀️component.rs`,
  `.../🔺️diff/🔀️boolean/🦀️component.rs`, `.../🔺️diff/🔺️euler/🦀️component.rs`,
  `.../🔺️diff/🧱️primitives/🦀️component.rs`, `.../🔺️diff/🧵️sew/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (added `semio-framework-number` dependency)
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` (removed 12 mounts, kept `engine`)

**Deleted:**
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/{➡️vector,🔢️matrix,📏️tolerance,⚖️predicates,〰️polynomial,🎢️bezier,
  🪢️bspline,➰️curve,✂️curve-ops,🏄️surface,🪡️surface-ops,🚨️error}/` (12 directories, all contents moved)

**Untouched, confirmed correctly out of scope:** `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/`,
`BrepKernel`/`brep::kernel` anywhere, `🧊️3d/🥽️mesh`, `🔺️mesh-engine`, `➗️mathematical`, `📸️remodel`.
