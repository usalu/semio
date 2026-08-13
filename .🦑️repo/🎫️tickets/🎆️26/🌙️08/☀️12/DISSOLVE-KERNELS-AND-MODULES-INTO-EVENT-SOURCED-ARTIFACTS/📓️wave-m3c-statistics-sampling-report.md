# Wave M3c — statistics family + sampling

**No open duplication window.** The one slice I executed (tabular/probability/statistics/causal)
was carried through the full COPY → verify → DELETE+unmount → verify cycle in this session; nothing
is left living in two places at once. Four subdirs (entropy, sampling, fuzzy, number) are
researched and placement-decided but **not yet moved** — see Remainders. `🧮️math` is smaller
(55,522 → 50,900 LOC) but not yet empty of my slice.

## Coupling map — measured, not assumed

Grepped every one of the 8 subdirs for `crate::`, `graph::`/`graph_core::`, `geometry::`, and
cross-checked against math's own `📦️glue.rs` extern-crate aliases (`geometry` = `semio_framework_geometry`,
`graph_core` = `semio_framework_graph`, both already carved out of math in wave M2). Real dependency
edges found:

```
📋️tabular      (621 LOC)   — zero deps (leaf)
🎲️probability (1,205 LOC) — geometry::random only (external, already its own crate)
📊️statistics   (762 LOC)   — crate::algebra, crate::probability, crate::tabular
🔗️causal      (2,022 LOC) — crate::probability, crate::statistics, crate::tabular, crate::algebra,
                              geometry::random, AND graph_core::algorithms::{adjacency, topo_sort,
                              connected_components} (PC-stable's skeleton/cycle checks)
🎲️entropy    (9,881 LOC) — ZERO cross-refs to any sibling or to geometry::random. Its own nested
                              `pub mod graph { … }`/`pub mod spatial { … }` are entropy's OWN measures
                              (degree-distribution entropy, spatial entropy), not references to
                              math's sibling `graph`/`spatial` modules — confirmed by reading the
                              module doc comment ("no graph-library dependency") and by the fact that
                              `pub use graph::…`/`pub use spatial::…` at the file's own export region
                              resolve to entropy's own children, not crate-root siblings (edition 2021
                              has no implicit crate-root fallback for bare paths).
🎯️sampling   (9,809 LOC) — geometry::random only (SplitMix64/Rng/AliasTable), nothing else
🌫️fuzzy      (2,449 LOC) — crate::algebra, geometry::random
🔢️number     (3,456 LOC) — zero internal deps; but LIVE external cross-crate consumers (see below)
```

**Correction to the ticket brief's hypothesis:** "entropy↔graph" coupling does not exist — it was a
plausible-sounding guess from the directory listing that measurement disproved. entropy is the
single largest (9,881 LOC) and most cleanly self-contained subdir of the eight; that makes it the
next cheapest real win, not a coupling risk.

**graph_core coupling in causal was real and I initially missed it** — my first grep for bare
`graph::` caught zero hits; the actual reference is `graph_core::algorithms::…` (math's own
extern-crate alias for `semio_framework_graph`), only surfaced by the compiler once I moved the
file (`E0433: cannot find graph_core`). Rule I'm re-confirming for future waves: a grep miss is not
proof of no-coupling — a real `cargo check` after the copy is the only trustworthy signal, which is
exactly why the ORDERING mandates copy-then-verify before delete.

## Consumer census — who outside math uses these 8 today

```
causal / entropy / probability / sampling / statistics / tabular:  ZERO external consumers
                                                                     (repo-wide grep for both
                                                                     `semio_framework_math::<mod>`
                                                                     and `math::<mod>::`)
fuzzy:                                                              ZERO external consumers
number:                                                             TWO live external consumers
```

`number`'s consumers:
1. `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚖️predicates/🦀️component.rs` — imports
   `semio_framework_math::number::Rational` directly for exact-arithmetic escalation when f64
   orientation predicates fall inside roundoff tolerance. Live-mounted in
   `semio-framework-3d`'s `📦️glue.rs:16`.
2. The `➗️mathematical` plugin's Equation artifact (wave M3a) — `📸️snapshot`,
   `💡️inferences/🌱roots`, `💡️inferences/📈️polynomial-internals`, `💡️inferences/🌿️cas-internals`
   all import `math::number::{Integer, Rational, Natural, ModInt, primes, CommutativeRing, Field,
   GcdDomain, IntegralDomain, Ring}` — not just `Rational`, the whole arbitrary-precision + ring/field
   trait surface.

## Placement decisions

### tabular / probability / statistics / causal → inference compute internals under `✳️table` — EXECUTED

This is the ticket's own headline call and I agree with the reasoning as given: these are
derivations *over* tabular data, not a new persisted content shape, and `✳️table` already exists in
the frozen 18+`✳️any` roster. Concretely: four Rust-only compute subdirs
(`📋️tabular-internals`, `🎲️probability-internals`, `📊️statistics-internals`, `🔗️causal-internals`),
siblings of `📸️snapshot`/`🔺️diff`/`💡️inferences`/`🧬️mutations` under
`✳️table/🧬️schema/`, mirroring `✳️brep`'s `⚙️engine` precedent (a Rust-only sibling under `🧬️schema/`,
no TS twin — algorithm, not boundary vocabulary). Cross-file `crate::` references were rewritten to
absolute `crate::artifacts::semio::standards::v1::subsets::table::schema::<x>_internals::` paths
(not `super::`, which breaks once code moves into a nested `mod tests`); `crate::algebra::` became
`semio_framework_math::algebra::` (algebra stays in math); `geometry::random::` became
`semio_framework_geometry::random::`; `graph_core::algorithms::` became
`semio_framework_graph::algorithms::`. Added `semio-framework-geometry`, `semio-framework-graph`,
`semio-framework-math` as new `Cargo.toml` deps of `semio-s-plugin-stdio` — all framework-role
crates with no reverse dependency on stdio, so no cycle (stdio already depends on the framework-role
`semio-framework-os-kernel`/`3d`/`mesh-engine`/`schema`, same shape).

**One real `InferredField` was authored, not just relocated library code**: `📊moments`
(`ColumnMoments`) under `✳️table/🧬️schema/💡️inferences/📊moments/`, keyed by column name, one step
per numeric (`Int`/`Float`) column, no parents, wrapping `statistics_internals::{mean, variance,
std_dev}`. Full DepHash/plan/dep_input/compute impl mirroring `✳️mesh`'s `📦aabb` pilot exactly (13
existing `InferredField` impls found repo-wide; aabb was the closest per-entity/no-parents shape).
7 tests: honesty (real stats, non-numeric columns absent from the plan not faked-zero, empty-snapshot
plan), cache-transparency law, and 3 incrementality-law tests (unrelated-column edit is a hit,
same-column edit is the only miss). All pass. TS mirror (`SemioColumnMoments`) added; not run through
a JS toolchain (no `bun`/`nx` build was invoked for it — only structurally checked against the
`SemioAabb`/aabb TS precedent).

**Honest scope cut**: `📊moments` is NOT wired into the parent `SemioTableInference` aggregate
struct or its hand-rolled binary/text/json/proto/graphql codecs (5 separate serialization formats
that would each need updating and re-verifying byte-for-byte). That is real, separate, higher-risk
work I did not attempt this pass — flagged rather than silently skipped. The field itself is real,
mounted, tested, and independently callable via `store::infer_field::<SemioTableSnapshot,
ColumnMoments>`.

The remaining ~4,500 LOC across the four compute-internals files (OLS/logistic regression, PC-stable/
GES/LiNGAM/FCI causal discovery, backdoor/frontdoor/ID identification, hypothesis tests, distribution
sampling, etc.) compiled and their existing tests pass, but are not yet wired to any *other* new
`InferredField` — they sit as available, tested compute internals per the "keep every math code"
ruling, exactly like brep's 24 pre-allocated compute subdirs before every one had a completed
inference wrapper (status.md, Job 3).

### entropy → NOT executed this session; recommended as compute internals under `✳️table` (or `✳️value`)

9,881 LOC, the single largest subdir, and — measured above — the *cleanest* of the eight: zero
coupling to any sibling, zero coupling to geometry/random, zero external consumers. Its ~40 public
functions (Shannon/Rényi/Tsallis entropy, KL/JS/Wasserstein divergence, transfer entropy, permutation/
dispersion/sample entropy, wavelet/spectral entropy, PID, …) all operate on generic `&[f64]` sample
arrays, not on any table-specific structure, so it fits equally well as `✳️table` column-level
inference helpers (mirroring the moments precedent just proven) or as `✳️value` array helpers. Given
the mechanism is now proven (this session's tabular/probability/statistics/causal slice), relocating
entropy is now a **known-shape, lower-risk task**: copy the single self-contained file into a new
`🧮️entropy-internals` compute sibling, no cross-reference rewrites needed at all (zero `crate::`
refs), verify, then author one thin `InferredField` (e.g. per-column Shannon entropy) to prove real
wiring. I did not execute it this session — reporting the concrete next step rather than leaving it
unexamined.

### sampling → NOT executed; placement genuinely unresolved, reporting rather than forcing

9,809 LOC, also fully self-contained (only `geometry::random`, already extracted), zero external
consumers today (`TokenId`/`SequenceId` grep repo-wide outside math: zero hits). The ticket's
proposed home — "a plugin-owned sampler artifact in a neural-flavoured home" — does not have a
concrete target: I checked `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/` (2,789 LOC across
`🦀️component.rs` + `⚙️engine/🦀️component.rs`) and it is itself a **framework**-role module, not
owned by any plugin. I found **no plugin anywhere in `✏️s/🔌️plugins/` for neural/LLM/AI** — the
domain has no plugin owner to hand this to.

This changes the placement question: with an existing framework-tier `🧠️neural` engine already
established as precedent (LLM/diffusion machinery treated as domain-neutral framework functionality,
same tier as this ticket's own `geometry`/`graph` M2 carve-outs), I lean toward `sampling` qualifying
for CLAUDE.md's binding-ruling exception ("unless it is domain-neutral framework functionality") and
belonging as a sibling framework module (`🧰️framework/🔨️modules/🎯️sampling/`) rather than an artifact
— but minting a new plugin/artifact to satisfy the ticket's literal "plugin-owned" framing is a
structural decision above what I judged safe to make unilaterally and execute in this pass. Reporting
the fork in the road with evidence rather than guessing: **(a)** move to a new framework module
alongside geometry/graph (my lean, given the `🧠️neural` precedent), or **(b)** a human/owning-session
decision to stand up a new neural/LLM plugin first, then land sampling as its artifact. Either way the
relocation itself is mechanically cheap (zero internal coupling) once the destination is picked.

### fuzzy → NOT executed; no consumer signal to place it against

2,449 LOC, depends on `crate::algebra` (stays in math) + `geometry::random`. Zero external consumers
found anywhere in the repo. The ticket's instruction — "inference helper under whichever artifact
needs it" — presupposes an identifiable need; I found none. Rather than invent a home with no
consumer to validate the cut against (the same mistake the ticket's own m3a postmortem warns about:
relocating code into a directory doesn't make it the right mechanism), I'm reporting this as
genuinely unplaced. If a future wave surfaces a real fuzzy-logic consumer (interpolation/blending in
some plugin), that consumer should drive the placement.

### number → NOT executed; live coupling, reporting rather than forcing per the ticket's own instruction

Both consumers found above (`brep/⚖️predicates`, the `➗️mathematical` plugin's equation inferences)
are real, live, and load-bearing today. Moving `number` would require repointing both in the same
change — the brep predicates consumer especially can't be casually repointed: it needs a first-class
`Rational` type usable directly in exact-arithmetic escalation, not an artifact-scoped inference value
wrapped in `DepHash`/serde machinery, and rewriting it is squarely another ticket's territory (brep's
own migration is "mid-migration to stdio" per this ticket's own brief). My reasoned recommendation:
`number` (arbitrary-precision `Natural`/`Integer`/`Rational`/`ModInt` + `Ring`/`Field`/`GcdDomain`
trait machinery) is genuinely domain-neutral framework functionality by the same test CLAUDE.md's
binding ruling states — it is consumed by two *unrelated* domains (brep geometry's exact-arithmetic
predicates and the mathematical plugin's CAS/equation solving), which is exactly the "shared content
shape needed by ≥2 independent [consumers]" bar the ticket uses elsewhere to justify NOT minting new
homes. I recommend it either stay in `🧮️math` or, in a future wave, get its own
`🧰️framework/🔨️modules/🔢️number/` sibling to `📐️geometry`/`🕸️graph` (mechanically identical to M2's
carve-out) — never an artifact's inference layer, which would break the brep consumer's need for a
concrete arithmetic type.

## Test arithmetic

**Baseline** (captured this session, matches the ticket brief exactly):
`semio-framework-math --lib` → **1402 passed / 2 failed** (both pre-existing, `graph::dsl` tests,
unrelated to this wave).

Per-file test counts before the move: tabular 17, probability 38, statistics 17, causal 34 = **106**.

**After COPY** (new home compiles, `semio-s-plugin-stdio --lib`):
```
$ TD=".../🎯️target"; touch stdio's 📦️glue.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-stdio --all-targets
   Finished `dev` profile [unoptimized] target(s) in 58.15s      (zero errors)
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 2548 passed; 5 failed; 4 ignored        (all 106 new tests: ok)
```
The 5 failures (`binary::…extent::inference_default_law`, `dwg::…fixture_honesty_law`,
`dxf::…bounds_matches_hand_built_entity_extent`, `ifc::…fixture_honesty_law`,
`zip::…entries::inference_default_law`) are byte-identical to 4 of the 7 failures captured in this
ticket's own `scratch-g5-stdio-failures.txt` from wave G5 — pre-existing and unrelated to
table/tabular/statistics/probability/causal (the other 2 of G5's 7 have since been fixed by other
work). Confirmed via direct comparison, not assumption.

**After adding the `📊moments` InferredField** (+7 tests):
```
$ cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 2555 passed; 5 failed; 4 ignored        (2548 + 7 = 2555; same 5 pre-existing failures)
```

**After DELETE from math + unmount** (same change: removed the 4 `#[path]` mounts from math's
`📦️glue.rs` and `rm -rf`'d the 4 source dirs):
```
$ touch math's 📦️glue.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-math --all-targets
   Finished `dev` profile [unoptimized] target(s) in 6.75s      (zero errors)
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-math --lib
test result: FAILED. 1296 passed; 2 failed        (SAME 2 pre-existing graph::dsl failures)
```
**Arithmetic: 1402 − 106 = 1296. Exact.** Zero tests lost, zero silently dropped — the 106 that left
math are the exact 106 that now pass in stdio (plus 7 genuinely new ones for the real inference).

Also ran `cargo metadata --no-deps` after the delete → exit 0, workspace graph consistent (WORKSPACE_OK).
Re-grepped the whole repo for `semio_framework_math::{causal,probability,statistics,tabular}` and for
any `#[path]` string containing the four deleted files' names → the only hit
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs:152`, `🔗️causal`) resolves to `📡️spr`'s own
unrelated `causal` module, not math's deleted one (verified by reading the actual path, not the
substring — the ticket's own "a mount is not a string" lesson, applied).

## LOC arithmetic

`🧮️math`: 55,522 → 50,900 LOC (wc -l count; the ticket brief's 55,522 baseline and my 50,900
post-delete count may differ slightly in methodology from a `cloc`-style count elsewhere in the
ticket, but the delta — 4,610 lines removed across the 4 files — is exact and verified against the
`rm -rf`'d file sizes). Remaining in my slice: entropy 9,881 + fuzzy 2,449 + number 3,456 +
sampling 9,809 = 25,595 LOC not yet touched.

## Files touched

Created:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📋️tabular-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️probability-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📊️statistics-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/📊moments/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/💡️inferences/📊moments/🟦️component.ts`

Modified:
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (mounted the 4 compute-internals + `📊moments`)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (+`semio-framework-geometry`, +`semio-framework-graph`, +`semio-framework-math`)
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📦️glue.rs` (removed 4 `#[path]` mounts)
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/Cargo.toml` (description updated)

Deleted:
- `🧰️framework/🔨️modules/🧮️math/🔗️causal/🦀️component.rs`
- `🧰️framework/🔨️modules/🧮️math/🎲️probability/🦀️component.rs`
- `🧰️framework/🔨️modules/🧮️math/📊️statistics/🦀️component.rs`
- `🧰️framework/🔨️modules/🧮️math/📋️tabular/🦀️component.rs`

Scratch (ticket folder, `.txt` per policy):
`scratch-m3c-stdio-check1.txt`, `scratch-m3c-stdio-test1.txt`, `scratch-m3c-stdio-test-final.txt`,
`scratch-m3c-math-postdelete-check.txt`, `scratch-m3c-math-postdelete-test.txt` (plus the
pre-existing `scratch-m3c-math-check.txt`/`scratch-m3c-math-test.txt`/`scratch-m3c-policy-final.txt`
baseline captures this session inherited).

## What's left for the next M3c continuation

1. **entropy** (9,881 LOC) — known-shape, lower-risk relocation (zero coupling), concrete next step
   documented above. Highest LOC-per-effort win remaining.
2. **sampling** (9,809 LOC) — mechanically cheap once a destination is picked; destination itself
   needs a call between "new framework module" (my lean) and "new plugin" (the ticket's literal
   framing), which I did not make unilaterally.
3. **fuzzy** (2,449 LOC) — no consumer signal; placement genuinely open.
4. **number** (3,456 LOC) — live dual coupling (brep + mathematical plugin); recommend framework-tier
   carve-out in a future wave, not an artifact.
5. `📊moments` wiring into `SemioTableInference`'s aggregate struct + its 5 codec formats (binary/
   text/json/proto/graphql) — deferred, flagged, not attempted.
