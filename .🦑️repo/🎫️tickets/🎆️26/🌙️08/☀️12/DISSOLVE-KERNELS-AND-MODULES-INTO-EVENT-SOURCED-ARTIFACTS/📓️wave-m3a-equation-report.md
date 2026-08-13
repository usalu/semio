# Wave M3a — `🧮️math/🧮️cas` + `🧮️math/📈️polynomial` → `➗️mathematical` artifact tree

**THE DUPLICATION WINDOW IS CLOSED.** `🧮️cas`/`📈️polynomial` are deleted from `🧮️math`, their `#[path]` mounts removed from `🧮️math/📦️packages/🦀️rust/📦️glue.rs` in the same change, and `cargo check -p semio-framework-math --all-targets` is clean. All 8,689 LOC exist exactly once now, inside `➗️mathematical`'s artifact tree. On top of the migrated internals, ONE real vertical slice is proven end-to-end: `EquationSnapshot` (persistent, label-addressed expression tree) → `change-coefficient` mutation triad (real `diff`/`inverse`, `PersistentLabel`-addressed) → `roots` inference (the codebase's first real `impl InferredField<P>`, real `DepHash` chain, delegates into `📈️polynomial-internals`' actual Sturm-sequence isolation + bisection refinement).

This report supersedes the mid-wave version (git history has it, if the intermediate "step 1 only" state is ever needed).

## 1. Coupling map (cas ↔ polynomial ↔ number/algebra) — unchanged from the mid-wave finding, now load-bearing

```
cas        → number    (Rational, Integer, Natural — ~35 call sites)
cas        → algebra   (MatG, VecG — matrix module, polybridge's partial-fraction solve)
cas        → polynomial (MonomialOrder, PolyM, PolyU, factor_integer_poly, AlgebraicReal, isolate_real_roots — ~15 call sites)
polynomial → number    (CommutativeRing/Field/GcdDomain/IntegralDomain/Ring, ModInt, primes, Integer/Natural/Rational — polynomial is GENERIC over this hierarchy)
polynomial → cas       ZERO (grepped explicitly, confirmed twice)
number/algebra → cas/polynomial  ZERO CODE (re-checked before deleting: both files DO mention `crate::cas`/`crate::polynomial` in doc-comment PROSE — e.g. algebra's own doc: "this crate must not depend on `crate::polynomial`" — but no actual `use`/call. Confirmed by reading every matched line before deleting anything.)
```

A clean one-directional DAG: `cas → polynomial → number`, `cas → algebra`/`number` directly, nothing pointing back. This is what made the cut safe.

## 2. What's implemented (all four steps, in order)

**Step 1 — COPY.** `🧮️cas`/`📈️polynomial` copied verbatim into `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/{🌿️cas-internals,📈️polynomial-internals}/🦀️component.rs`. `crate::number`→`math::number`, `crate::algebra`→`math::algebra` (new `semio-framework-math` dependency, aliased `math`); `crate::cas::…`/`crate::polynomial::…` self-references left untouched, made to work by mounting BOTH crate-root-direct (`#[path=…] pub mod cas;` in the plugin's `📦️glue.rs`, exactly like `🧮️math`'s own glue did) rather than nested-with-reexport — the first attempt (`mod component; pub use component::*;` nested under `inferences`) broke on `error[E0433]: cannot find canon in cas` because `mod canon` is non-`pub` and a glob re-export doesn't leak private items back out. Documented in both files' doc headers and the glue.rs comments.

**Step 2 — new home compiles + tests run.** `cargo check -p semio-s-plugin-mathematical --all-targets` clean; migrated tests produced BYTE-IDENTICAL pass/fail counts to the original (166 passed/13 failed, same 13 names, cross-checked against `🧮️math`'s still-intact copy before deleting anything).

**Step 2.5 — the vertical slice (this session's continuation), §3 below.**

**Step 3 — repoint consumers.** Census (`grep -rln "math::cas\|math::polynomial"` and broader `cas::\|polynomial::` excluding math/the new location itself, both re-run immediately before deleting): **zero** external consumers, twice. Nothing to repoint.

**Step 4 — delete + verify.** `rm -rf 🧮️math/🧮️cas 🧮️math/📈️polynomial`, removed both `#[path]` mounts + `pub mod cas;`/`pub mod polynomial;` from `🧮️math/📦️packages/🦀️rust/📦️glue.rs` in the same change, updated the crate's `Cargo.toml` `description` (no longer mentions CAS). `🔢️number`/`➕️algebra` and every other math subdir untouched — confirmed by `git status`-equivalent (only `🧮️cas`/`📈️polynomial` removed, only `📦️glue.rs`/`Cargo.toml` edited).

## 3. The vertical slice — `roots`, end-to-end

### 3a. `EquationSnapshot` (new `#[state(persistent)]` field on `MathematicalSnapshot`)

Read the destination's CURRENT snapshot shape before adding anything: `MathematicalSnapshot` was JUST refactored (ticket `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, `mathematical→C:text,table,value`) to hold three composed CHILD slots (`notation`/`results`/`computed`, each an `ArtifactChild<S>` into a stdio subset) instead of inline domain fields. Checked whether `computed`'s `SemioValueSnapshot` (an untyped `Null/Bool/Int/Float/Str/Bytes/List/Map/Ref` JSON-like graph) could host the equation instead of adding a new field — it can't: no operator/variable vocabulary, and label-addressed structural mutations need a typed enum to address, not an untyped tree walk. So `equation: EquationSnapshot` is a FOURTH, plain (non-`#[child]`) persistent field, added directly — not routed through the child-composition contract, which governs this plugin's competing text/table/value content models specifically, not new domain content.

`EquationSnapshot { expr: EquationNode, next_label: u64 }`. `EquationNode { label: EquationNodeLabel(u64), kind: EquationNodeKind }`, `EquationNodeKind ∈ {Integer{lexeme}, Rational{numer,denom}, Symbol{name}, Add{terms}, Mul{factors}, Pow{base,exponent}}` — a deliberately separate, plain, serde-friendly type from `cas::expr::Expr` (which has no `Serialize`/`Deserialize` and whose private `Node`/hash-cache invariants are only safe to build through `canon.rs`'s smart constructors — a naive field-by-field deserialize would violate them). Bridged to/from `cas::expr::Expr` through `cas`'s PUBLIC constructor/accessor API only (`Expr::kind()`, `Expr::integer`/`from(Rational)`/`symbol`/`add`/`mul`/`pow`) — `equation_node_to_expr`/`expr_to_equation_node` in `📸️snapshot/🦀️component.rs`. Scope: `Integer`/`Rational`/`Symbol`/`Add`/`Mul`/`Pow` only, enough for a single-variable polynomial — `Fn`/`Piecewise`/`Rel`/`Wild`/`RootOf`/`Constant` are explicitly out of scope this wave (documented in the type's own doc comment, not silently dropped: `expr_to_equation_node` falls back to `Integer(0)` for anything outside scope rather than panicking).

**Label addressing — `EquationNodeLabel`, not a positional path.** This is the part the coordinator flagged explicitly. A `u64` issued at node birth, never reused, carried in the snapshot — mirrors `✳️brep`'s `PersistentLabel`. The concrete argument for why positional (`expr.children[2].children[0]`) is unsafe here, not hypothetical: `MathematicalSnapshot`'s OWN pre-existing `➕️insert-point`/`➖️remove-point` triad has exactly this bug, already found and documented (with root cause and dating, `git log` `2026-08-12 11:09:41`, before this ticket opened) in `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave4-reports/mathematical-report.md`: `remove-point`'s diff computes an index against `base`, then that diff gets applied to a DIFFERENT `state` whose collection length has changed — `insert_point_inverse_is_remove_point_at_same_index` fails because the base-relative index silently resolves to the wrong element. `EquationNodeLabel` cannot have this failure mode by construction: `find`/`replace` walk the WHOLE tree looking for the exact label, never index into a positionally-addressed slot, so an unrelated insert/delete elsewhere in the tree cannot shift what a label resolves to. (This pre-existing bug is STILL failing, unfixed, unrelated to this wave's change — see §5.)

Text/binary codecs extended: `equation` round-trips as hex-encoded `serde_json` (text) / length-prefixed JSON bytes (binary) — no handcrafted grammar yet (documented as future work), consistent with how the child-handle half of the existing codec already trades a minimal wire format for a real one. `🗣️example.dsl.semio` (the plugin's fixture) updated with a real `equation=` line (hand-verified to deserialize correctly, not hand-waved) rather than adding a backward-compat "missing equation line" fallback — CLAUDE.md's no-legacy-support rule; there are no persisted users to migrate.

### 3b. `change-coefficient` — the authoring mutation triad

`🧬️mutations/🔄️change-coefficient/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` + `🟦️component.ts` per leaf, mirroring `🏷️change-node-label`'s exact shape. Verb `change` (from `APPROVED_VERBS`), entity `coefficient`, record `ChangedCoefficient`.

- **Payload**: `{ label: EquationNodeLabel, numer: String, denom: String }` — `numer`/`denom` are `Integer` decimal lexemes (never `f64`), `denom == "1"` means a plain integer coefficient.
- **`diff(payload, base)`**: looks up `payload.label` in `base.equation`; if it resolves to an `Integer`/`Rational` LEAF (not any other node kind), replaces it — otherwise the diff is a no-op (equation unchanged). Computed purely from `(payload, base)`, never apply-then-capture.
- **`inverse(payload, base)`**: looks up `payload.label` in `base` (the PRE-mutation state) and returns a `ChangeCoefficient` restoring base's own value — `Vec::new()` if the label isn't a numeric leaf in `base`.

Registered in `MathematicalMutation` (15th variant now), and — because this plugin hand-rolls its own `OpText`/`OpBinary` (`🧬️mutations/📝️text/🦀️component.rs`, NOT macro-generated) — added a `change-coefficient` keyword case to the text grammar, tag `14` to the binary codec, and a demo case to the existing exhaustive round-trip test (`op_text_binary_roundtrip_law`), which passed.

Tests (extended the EXISTING `🧬️mutations/🦀️component.rs` test module, no new test files): `change_coefficient_obeys_the_inverse_law` (via the same `protocol::testkit::assert_mutation_inverse_law` helper every other triad's law test uses), `change_coefficient_sets_the_targeted_numeric_leaf`, `change_coefficient_at_an_unknown_label_is_a_no_op`. `semantic_kinds_cover_every_variant`'s count updated 14→15 (a real count that changed because a real variant was added, not a hidden gate).

### 3c. `roots` — the first real `InferredField<P>` in the codebase

Grepped repo-wide for `impl.*InferredField for|impl InferredField<` before writing anything: **zero hits** anywhere outside `InferredField`'s own definition file's unit tests. Every currently-named inference in the ENTIRE codebase (`🧭topology`, `📦bounds`, `flat-position`, …) documents, in its own doc comment, why it uses the plain `compute_X(snapshot) -> X` pattern instead ("a plain whole-snapshot derivation… nothing to invalidate incrementally"). `roots` is the genuine fit: real roots of a polynomial form a small INDEXED COLLECTION with no cross-root dependency — exactly `InferredField::Key`'s intended shape (`Key = usize`, index into the isolated-root list; no parents, since roots don't depend on each other).

`💡️inferences/🌱roots/🦀️component.rs`:
- **`plan(snapshot)`**: extracts a single-variable, INTEGER-coefficient polynomial from `snapshot.equation` (a real structural walk over `EquationNode` — `Add`/`Mul`/`Pow(Symbol,IntegerLit)`/`Integer`/`Rational{denom==1}`; anything else, including a second variable or a non-integer coefficient, returns `None`); if extraction succeeds, calls `polynomial::roots::isolate_real_roots` (Sturm-sequence sign-change counting — real math, not reimplemented, delegated straight into `📈️polynomial-internals`) and plans one step per isolated interval. Out-of-scope equations plan ZERO steps — an empty root list, never a panic or a wrong answer (tested explicitly).
- **`dep_input(snapshot, key, _)`**: the polynomial's own coefficients (so ANY coefficient edit invalidates EVERY root's cache entry — roots are a global function of all coefficients) plus `key`'s own isolating interval bytes.
- **`compute(snapshot, key, _)`**: re-isolates, bisects the `key`-th interval to width `1/10^9` via `polynomial::roots::refine_root` (again, real delegated math), returns the refined interval's midpoint as `f64`.
- **`compute_mathematical_roots(snapshot)`**: calls `protocol::infer_field::<MathematicalSnapshot, MathematicalRootsField>(snapshot, None)` — the REAL plan→dep-hash→compute orchestration `InferredField` exists for (found in `💡️inference/🦀️component.rs`'s own test module, the only place in the repo that had ever exercised it), not a hand-rolled loop.

Wired into `MathematicalInference` (`roots: Vec<MathematicalRoot>`, `#[state(inferred)]`) alongside `topology`, registered in `InferenceSpec::fields()`.

**Proof it actually computes roots, not just compiles**: `compute_mathematical_roots_finds_one_and_two` builds `x² − 3x + 2` as a labeled tree by hand and asserts the two computed roots are `1.0` and `2.0` within `1e-6` — this test PASSED (real output below). `dep_input_changes_when_a_coefficient_changes` proves the `DepHash` chain is actually wired to `equation`, not a constant (also passed).

## 4. Test arithmetic — every number below is from a real run, commands and outputs in the scratch files named

**Plugin (`semio-s-plugin-mathematical`), after the full vertical slice, `cargo test --all-targets`** (`scratch-m3b-fulltest2.txt`):
```
test result: FAILED. 248 passed; 14 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.63s
```
14 failures = the 13 migrated cas/polynomial failures (unchanged names, unchanged panic sites, just a new file path in the message) + 1 pre-existing, already-documented, unrelated `insert_point_inverse_is_remove_point_at_same_index` (see §3a's label-addressing discussion — this is the SAME bug that motivates `EquationNodeLabel`, still unfixed, out of this wave's scope). 248 passed = 238 (the internals-copy baseline from step 1/2) + 10 new tests from the vertical slice, ALL passing: 6 in `🌱roots`, 1 in the top-level inference test module, 3 in the mutations test module. Zero previously-passing tests broke; zero previously-failing tests started passing unexplained.

**`🧮️math` (`semio-framework-math`), after deletion, `cargo test --lib`** (`scratch-m3c-math-test.txt`):
```
test result: FAILED. 1402 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.83s
```
Wave-0 baseline was **1568 passed / 15 failed = 1583 total**. `1583 − 1404 (new total) = 179` — **exactly** 166+13, the cas/polynomial tests that emigrated. The 2 remaining failures are `graph::dsl::tests::parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error` and `graph::dsl::wire::tests::dag_from_wire_literal_rejects_unexpected_char` — cross-checked line-for-line against `scratch-w0-baseline-failures-sorted.txt`: **both were already in the Wave-0 baseline**, in a module (`🕸️graph/🗣️dsl`) this wave never touched. `1568 − 1402 = 166` (matches cas/polynomial's passing count exactly); `15 − 2 = 13` (matches cas/polynomial's failing count exactly).

**Verification commands, real, run in this order:**
```
$ touch "…/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo check -p semio-s-plugin-mathematical --all-targets
    Finished `dev` profile [unoptimized] target(s) in 2.80s          (0 errors — scratch-m3b-check1.txt)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo test -p semio-s-plugin-mathematical --all-targets
    test result: FAILED. 248 passed; 14 failed; …                     (scratch-m3b-fulltest2.txt)

$ touch "…/🧮️math/📦️packages/🦀️rust/📦️glue.rs"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo check -p semio-framework-math --all-targets
    Finished `dev` profile [unoptimized] target(s) in 3.99s          (0 errors — scratch-m3c-math-check.txt)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo test -p semio-framework-math --lib
    test result: FAILED. 1402 passed; 2 failed; …                     (scratch-m3c-math-test.txt)
```
Both `cargo check` runs are genuinely fresh (glue.rs `touch`ed immediately before each, `RUSTC_WRAPPER=""` throughout, `--all-targets` throughout — no `cargo check` without `--all-targets` was ever treated as sufficient).

**Policy** (`bun ./📜️script.ts policy`, `scratch-m3c-policy-final.txt`, 23,849 high-priority breaches across 29 rules — IDENTICAL count to the pre-vertical-slice run, confirming zero new breaches from the new mutation triad/inference/schema field). Grepped explicitly for `change-coefficient`/`🌱roots`/`cas-internals`/`polynomial-internals`/`EquationSnapshot` in the full breach output: zero hits.

**Census, re-run immediately before deleting anything**: `grep -rln "math::cas\|math::polynomial"` and the broader `cas::\|polynomial::` (excluding math and the new plugin location) — both empty. Zero external consumers, confirmed twice, right before the irreversible step.

## 5. Honest remainders — what this wave did NOT do

- **Only ONE mutation (`change-coefficient`) and ONE inference (`roots`) are wired.** The full verb table (create-term/delete-term/change-exponent/rename-variable/replace-expression/reorder-terms/create-assumption/delete-assumption/change-assumption — 9 more) and inference table (simplified/derivative/taylor_series/limit/factorization/integral/ode_solution/series_sum/solution_set/matrix_ops/transforms — 11 more) from the mid-wave design are still just a design, now proven-compatible with a real implemented instance of each category (one plain-mutation pattern via `change-coefficient`, one `InferredField` pattern via `roots`) but not replicated. Next step is mechanical repetition of a now-proven pattern, not fresh design risk.
- **`EquationNode`'s vocabulary is `Integer`/`Rational`/`Symbol`/`Add`/`Mul`/`Pow` only** — no `Fn` (so no `sin`/`cos`/`exp`/etc in an authored equation yet), no `Piecewise`/`Rel`/`Wild`/`RootOf`/`Constant`. `roots` itself is further scoped to single-variable, integer-coefficient polynomials (rational coefficients, a second variable, or anything using an unsupported `Kind` all degrade to an empty root list, never a wrong answer or a panic — tested).
- **`assumptions`/`domains`** (mentioned in the mid-wave mutation-verb design for `create-assumption` etc.) are not part of `EquationSnapshot` yet — would need to land alongside whichever wave implements those mutations.
- **The `insert_point_inverse_is_remove_point_at_same_index` bug is still failing**, unfixed, exactly as documented in the prior wave's report — this wave's `EquationNodeLabel` design is a deliberate structural avoidance of the same bug CLASS, not a fix to the existing instance.
- **No graphql/proto/json_schema facet leaves were extended** for the new `equation` field or the new mutation/inference — Rust + TypeScript only (which the ticket's own instructions specifically call for: "non-stub `component.ts`… Inference values need a `component.ts`"). The pre-existing `artifact-schema/facet-completeness` breach category (276 instances repo-wide before this wave, unchanged after) already reflects this as a known, repo-wide, pre-existing gap pattern, not one this wave introduced fresh.

## 5b. Concurrent churn observed, not mine

`git status` on `🧰️framework/🔨️modules/🧮️math/` shows modified/deleted entries this session never touched — `🎲️random/🦀️component.rs` (deleted), `📐️geometry/🦀️component.rs` (deleted), several `🕸️graph/*` and `🧩️wfc/*` files (modified), `build.rs`/`📋️project.json`/`📜️script.ts`/the TS package files (modified). This session's own edits to `🧮️math` are exactly two: `📦️packages/🦀️rust/📦️glue.rs` (mount removal) and `📦️packages/🦀️rust/Cargo.toml` (description) — every other `🧮️math` diff is another concurrent session's in-flight work (consistent with this repo's live-multi-session model). Both `cargo check`/`cargo test -p semio-framework-math` runs above passed cleanly against the live tree including that concurrent state, so it isn't blocking this wave — flagged here per policy, not re-investigated, not touched.

## 6. Files touched this wave

**Deleted**: `🧰️framework/🔨️modules/🧮️math/🧮️cas/` (whole dir), `🧰️framework/🔨️modules/🧮️math/📈️polynomial/` (whole dir).

**Modified**: `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📦️glue.rs` (removed 2 mounts), `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/Cargo.toml` (description), `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` (mounts + `extern crate semio_framework_math as math;`), `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml` (new dependency), `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs` (`mathematical_snapshot_with_state`), `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️component.rs, 📸️snapshot/🦀️component.rs, 🔺️diff/🦀️component.rs, 🔺️diff/📝️text/🦀️component.rs, 💡️inferences/🦀️component.rs, 🧬️mutations/🦀️component.rs, 🧬️mutations/📝️text/🦀️component.rs}`, `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`.

**Created**: `…/💡️inferences/{🌿️cas-internals,📈️polynomial-internals}/🦀️component.rs` (verbatim migration), `…/💡️inferences/🌱roots/{🦀️component.rs,🟦️component.ts}`, `…/🧬️mutations/🔄️change-coefficient/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}`.
