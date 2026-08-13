# Wave M3a — `🧮️math/🧮️cas` + `🧮️math/📈️polynomial` → `➗️mathematical` artifact tree

**Slice**: `🧮️cas` (6,323 LOC) + `📈️polynomial` (2,366 LOC) = 8,689 LOC.
**Status: STEP 1 (COPY) COMPLETE AND VERIFIED. STEPS 2–4 NOT DONE — stopped here deliberately, see "What's not done" below.**

Nothing was deleted. Every line of both files exists, verbatim, in its new home, mounted, compiling, and test-verified to produce byte-identical pass/fail results to the original. `🧮️math` itself was **not touched** — `🧮️cas`/`📈️polynomial` still live there too, unmodified. This is the "code exists twice, bounded duplication window" state the ticket describes as the expected mid-wave condition.

## 1. Coupling map (cas ↔ polynomial ↔ number/algebra)

Grepped every `crate::` prefix in both files (`grep -oE "crate::[a-z_]+"`), then cross-checked each call site:

```
cas        → crate::number   (Rational, Integer, Natural — assume/expr/simplify/matrix/integrate/sums/ode/limits/rootof, ~35 call sites)
cas        → crate::algebra  (MatG, VecG — matrix module's numeric solve path, polybridge's partial-fraction linear system)
cas        → crate::polynomial (MonomialOrder, PolyM, PolyU, factor_integer_poly, AlgebraicReal, isolate_real_roots —
                                  polybridge/rootof/limits/matrix::charpoly/integrate/sums/ode, ~15 call sites)
cas        → crate::cas      (self-reference only — every submodule reaches sibling submodules via the full crate-root path,
                                  e.g. `crate::cas::canon::make_symbol` called from `cas::expr`)
polynomial → crate::number   (CommutativeRing/Field/GcdDomain/IntegralDomain/Ring traits, ModInt, primes, Integer/Natural/Rational —
                                  polynomial is generic OVER this trait hierarchy, ~10 call sites)
polynomial → crate::polynomial (self-reference only, same pattern as cas)
polynomial → crate::cas      ZERO — grepped explicitly (`grep -n "super::cas\|crate::cas\|::cas::"`), no hits.
```

**Verdict: a clean one-directional DAG.** `cas → polynomial → number`, `cas → number`, `cas → algebra` directly; polynomial never reaches back into cas. This is exactly the shape that makes "copy cas+polynomial together, leave number+algebra behind" a clean cut — no back-edge to sever, no mutual recursion between the two files being migrated.

`polynomial` also uses `geometry::random::Rng` (twice, in `univariate`/`finite` test modules) via the plain crate-name path — this works unchanged because the `➗️mathematical` plugin already depends on `semio-framework-geometry` aliased as `geometry` in its own `Cargo.toml` (same alias math's `extern crate semio_framework_geometry as geometry;` provided). Neither file uses `thiserror`, `wasm_bindgen`, `serde`, or any of math's other `extern crate` aliases (`dsl_core`, `dsl_schema`, `dsl`, `graph_core`) — grepped and confirmed zero hits, so no other new dependency was needed.

## 2. Destination architecture — what I found before writing any code

Read `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/` in full before touching it, per the brief. Two findings changed my plan:

**(a) The snapshot was JUST refactored away from inline domain fields.** `MathematicalSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs`) no longer holds `graph`/`geometry` directly — ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`mathematical→C:text,table,value`) replaced them with three fixed composed CHILD slots (`notation: ArtifactChild<SemioTextSnapshot>`, `results: ArtifactChild<SemioTableSnapshot>`, `computed: ArtifactChild<SemioValueSnapshot>`), reconstructing `MathematicalGraph`/`MathematicalGeometry` from those children at the boundary (`mathematical_graph_geometry_from_children`). I checked whether the `value` child (`SemioValueSnapshot` — `Null/Bool/Int/Float/Str/Bytes/List/Map/Ref`, an untyped JSON-like graph) could host the equation `Expr` AST instead of adding new plugin fields, per the brief's "only add new artifact dirs if the existing one genuinely cannot host equations." It genuinely cannot: it has no operator/variable/assumption vocabulary, and structural mutations like "change coefficient of term 3" need a typed `Expr` enum to identify, not an untyped `Map`/`List` walk. **Conclusion (design, not yet implemented): a 4th field belongs directly on `MathematicalSnapshot` — `equation: EquationSnapshot` — parallel to, not routed through, the text/table/value composition contract**, which governs this plugin's competing content models for exactly those three kinds, not new domain content.

**(b) `InferredField<P>`/`DepHash` is real, but this plugin's own precedent argues against it for scalar derivations.** Read `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs` per the brief. But the *existing* `🧭topology` inference in this exact plugin uses the simpler `protocol::Inference<P>`/`InferenceSpec<P>` top-level trait with a plain `compute_mathematical_topology(&graph) -> MathematicalTopology` function — no `InferredField`, no `DepHash`. I found the reasoning documented verbatim elsewhere in the SAME codebase: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/…/💡️inferences/📦bounds/🦀️component.rs`'s own doc comment: *"Block3d has no parent/child object graph … so this is a plain whole-snapshot derivation, not a per-entity `InferredField` chain: every vortex contributes independently to one aggregate box, there is nothing to invalidate incrementally."* An `Expr` AST is exactly this shape for `simplify`/`diff`/`integrate`/`limits`/etc — one whole-snapshot value in, one whole-snapshot value out, nothing to invalidate per-entity. **Design decision: use the plain `compute_X(snapshot) -> X` pattern (matching `📦bounds`/`🧭topology` precedent) for whole-Expr derivations, and reserve `InferredField<P>` for `roots` specifically** (`Key = usize` index into the root list — a genuine small indexed collection, and the brief's own named example `💡️inferences/🌱roots/`), where per-root caching is a defensible real use of the DAG mechanism. This is a documented deviation from a blanket reading of the brief's "each is an `impl InferredField<P>`," backed by the codebase's own precedent for exactly this shape of problem.

## 3. What's implemented (Step 1 — COPY, done and verified)

Both files copied **verbatim** (`cp`, not retyped) into:

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️component.rs` (6,323 → 6,335 lines, +12 doc-header lines)
- `…/💡️inferences/📈️polynomial-internals/🦀️component.rs` (2,366 → 2,375 lines, +9 doc-header lines)

Physically placed under the facet's `💡️inferences/` dir — mirrors stdio's `📐️step` io facet's `🪜️ladder`/`📐️part21`/`🧱️brep` precedent for deep Rust-only helper dirs a facet's real leaves delegate into. Neither file is itself a `MutationKind` or `InferredField` leaf.

**Edits made to the copies** (mechanical, sed + 2 doc headers, nothing else):
- `crate::number` → `math::number`, `crate::algebra` → `math::algebra` (all ~35+~10 occurrences, code AND doc-comment prose) — these two stay in `🧮️math` for now, reached through a new `extern crate semio_framework_math as math;` dependency.
- `crate::cas::…` / `crate::polynomial::…` self-references: **left untouched, zero edits.**

**Mounting** (`✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs`): first attempt mounted the two files as `mod component; pub use component::*;` nested under `…schema::inferences` — this **broke the build** (`error[E0433]: cannot find canon in cas`, 13 occurrences) because `mod canon { … }` inside `cas`'s component.rs is non-`pub`, and Rust privacy is structural: a `pub use X::*` re-export does not leak `X`'s private items back out through the alias, no matter how many wrapper modules point at it. Fixed by mounting **directly at crate root** — `#[path = "…/🌿️cas-internals/🦀️component.rs"] pub mod cas;` / same for `polynomial` — exactly mirroring how `🧮️math`'s own `glue.rs` mounted them, which is the only way every `crate::cas::…` (including references to non-`pub` inner modules) keeps resolving unedited.

`Cargo.toml`: added `semio-framework-math = { path = "…/🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust", package = "semio-framework-math" }`. Verified no dependency cycle (math is a `role = "framework"` crate, never depends on plugins).

## 4. Verification (real commands, real output)

```
$ touch "✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo check -p semio-s-plugin-mathematical --all-targets
    Finished `dev` profile [unoptimized] target(s) in 1.79s
```
0 errors (`grep -c "^error\["` → `0`). 737 warnings, all pre-existing and unrelated (unnecessary-qualification lints in untouched files, one `ambiguous_glob_imports` `testkit` warning in the pre-existing `🧬️mutations/🦀️component.rs` from `os_spr::*`/`os_pack::*` glob overlap — not in a file I touched, not caused by my mount). Full output: `scratch-m3a-cargo-check2.txt`.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo test -p semio-s-plugin-mathematical --lib
test result: FAILED. 238 passed; 14 failed; 0 ignored; 0 measured; finished in 1.56s
```
Full output: `scratch-m3a-plugin-test1.txt`. Broken down by prefix:

| group | ok | failed | total |
|---|---|---|---|
| `cas::*` | 131 | 7 | 138 |
| `polynomial::*` | 35 | 6 | 41 |
| plugin-own (pre-existing, untouched) | 72 | 1 | 73 |
| **total** | **238** | **14** | **252** |

Cross-checked against `🧮️math`'s own (still-intact, unremoved) copy:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="…/🎯️target" cargo test -p semio-framework-math --lib -- cas:: polynomial::
test result: FAILED. 166 passed; 13 failed; 1404 filtered out; finished in 1.77s
```
131+35 = 166 ok, 7+6 = 13 failed — **exact match**, same 13 test names, in both locations. This is the "test-count arithmetic" the brief asks for: the migrated copy produces byte-identical pass/fail results to the original. Full output: `scratch-m3a-math-crossref.txt`.

**The 13 pre-existing failures** (12 named in the brief, actual count is 13 — the brief's own list, hand-counted, has 13 bullet items: `integrate`(1) + `limits`(1) + `ode`(2) + `sums`(3) + `polynomial::algebraic`(4) + `polynomial::finite`(1) + `polynomial::univariate`(1) = 13; "12" in the brief text appears to be a miscount against its own list, not a discrepancy I introduced):

```
cas::integrate::tests::integrate_simple_partial_fraction
cas::limits::tests::limit_at_infinity_of_rational_function
cas::ode::tests::bernoulli_ode
cas::ode::tests::linear_first_order_ode
cas::sums::tests::fourier_coefficients_of_a_polynomial_smoke_test
cas::sums::tests::sum_of_k_from_1_to_n_is_gauss_formula
cas::sums::tests::sum_of_k_squared_matches_known_hand_values
polynomial::algebraic::tests::cbrt2_times_cbrt4_equals_2
polynomial::algebraic::tests::neg_and_inv_hand_cases
polynomial::algebraic::tests::root_of_selects_correct_irreducible_factor
polynomial::algebraic::tests::sqrt2_plus_sqrt3_has_minimal_poly_degree_4
polynomial::finite::tests::is_irreducible_hand_cases
polynomial::univariate::tests::interpolate_reconstructs_quadratic
```
All 13 migrated with their code, still failing, same names, same assertion sites (just a new file path in the panic message). None fixed, none deleted. **No test that was passing before is failing now, and none that was failing is now passing** — verified by exact name-for-name diff against `scratch-w0-baseline-failures-sorted.txt`.

**One pre-existing, unrelated failure surfaced in the same run**: `…mutations::component::tests::insert_point_inverse_is_remove_point_at_same_index`. Not in a file I touched (I never edited `🧬️mutations/🦀️component.rs`, `➕️insert-point/`, or `➖️remove-point/`), deterministic (no RNG), and already fully documented with root cause and dating in `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave4-reports/mathematical-report.md` (a whole-collection-slot diff computed from `base` instead of live `state`, dated `2026-08-12 11:09:41` — before this ticket even opened, explicitly deferred to a future DiffKit rework). Not touched here, consistent with the prior wave's own decision.

**Policy check** (`bun ./📜️script.ts policy`, full output `scratch-m3a-policy.txt`, 23,878 lines / 23,849 pre-existing high-priority breaches across 29 rules, all repo-wide and pre-existing — `handcrafted-grammar/spec-distinctness`, `artifact-io/sniff-reality`, `taxonomy/dead-example-leaf`, etc., affecting dozens of unrelated plugins uniformly). Grepped explicitly for `cas-internals`/`polynomial-internals` in the full breach output: **zero hits** — the new dirs introduce no new breach of any kind, confirming the brief's claim that the taxonomy walker doesn't descend to subset depth for new-shape artifacts.

**Census for consumers** (step 3 precondition): `grep -rln "math::cas\|math::polynomial" --include="*.rs" .` (excl. `target`) → **empty**. Broader `grep -rln "cas::\|polynomial::"` excluding `🧮️math` itself → **empty**. Zero external consumers, confirmed twice with different patterns. Step 3 (repoint) is therefore trivially satisfied by construction — there was nothing to repoint.

## 5. Mutation verb table (design — NOT implemented this wave)

`EquationSnapshot` (design): `{ expr: Expr, assumptions: BTreeMap<String, AssumeSet>, domains: BTreeMap<String, VariableDomain> }`, using `cas`'s own `Expr`/`Kind`/`AssumeSet` types as the persistent field's value type (these move under `🧬️schema/📸️snapshot/` as support types once implemented — they are NOT derived, they ARE the authoritative content, so they don't belong under `💡️inferences/` the way the calculus/polynomial operations do).

| verb | payload | diff (from base) | inverse (from base) | why mutation not inference |
|---|---|---|---|---|
| `create-term` | `{ path: NodePath, term: Expr }` | insert `term` into the `Add`/`Mul` node at `path` | `delete-term` at the same resulting index | user typed a new addend/factor; nothing to compute |
| `delete-term` | `{ path: NodePath }` | remove the node at `path` | re-insert the removed subexpr (captured from `base`) at the same index | user removed a term they authored |
| `change-coefficient` | `{ path: NodePath, coefficient: Rational }` | replace the leading numeric factor at `path` | restore `base`'s original coefficient at `path` | direct numeric edit to authored content |
| `change-exponent` | `{ path: NodePath, exponent: Expr }` | replace a `Pow` node's exponent at `path` | restore `base`'s original exponent | direct edit to an authored `Pow` |
| `rename-variable` | `{ from: String, to: String }` | rewrite every `Symbol(from)` to `Symbol(to)` | `rename-variable { from: to, to: from }` | user relabels a symbol; simplify/diff/etc. are all downstream of this, not this itself |
| `replace-expression` | `{ path: NodePath, expr: Expr }` | swap the whole subtree at `path` | restore `base`'s original subtree at `path` | wholesale authored substitution (e.g. swap `sin(x)` for `cos(x)`) — payload is user-authored, not computed |
| `reorder-terms` | `{ path: NodePath, order: Vec<usize> }` | permute an `Add`/`Mul`'s children per `order` | the inverse permutation | display/authoring order is user intent, canonical order is `simplify`'s job (an inference) |
| `create-assumption` | `{ symbol: String, operator: RelationalOperator, bound: Rational }` | insert into `assumptions[symbol]` | `delete-assumption { symbol, … }` | user asserts a fact (`x > 0`), never derived |
| `delete-assumption` | `{ symbol: String, index: usize }` | remove one fact from `assumptions[symbol]` | re-insert the removed fact (captured from `base`) | inverse of `create-assumption` |
| `change-assumption` | `{ symbol: String, index: usize, operator: …, bound: … }` | replace one fact in place | restore `base`'s original fact at that index | editing an already-authored assumption's bound/operator |

All ten verbs are drawn from the closed `APPROVED_VERBS` set (`create`/`delete`/`change`/`rename`/`replace`/`reorder`) — no `set-*`/`update-*`/option-bag payload anywhere in this table. None of these were implemented this wave (see §7).

## 6. Inference table (design — cross-referenced against what physically exists now)

| inference | mechanism | compute() delegates to | status |
|---|---|---|---|
| `simplified: Expr` | plain `compute_X(snapshot)`, precedent §2(b) | `cas::simplify::simplify` (+ `cas::trig::trig_canon` as a candidate) | internals present, not wired |
| `derivative: Expr` | plain `compute_X` | `cas::diff::diff` | internals present, not wired |
| `taylor_series: Vec<Expr>` | plain `compute_X` | `cas::series::taylor_series` | internals present, not wired |
| `limit: Option<Expr>` | plain `compute_X` | `cas::limits::limit` | internals present, not wired |
| `roots: Vec<Root>` | **`InferredField<MathematicalSnapshot>`**, `Key = usize` | `cas::rootof` + `cas::solve` + `polynomial::roots` + `polynomial::algebraic` | internals present, not wired — the brief's own named example (`💡️inferences/🌱roots/`) |
| `factorization: Vec<Expr>` | plain `compute_X` | `polynomial::factor` (via `cas::polybridge` Expr↔Poly bridge) | internals present, not wired |
| `integral: Option<Expr>` | plain `compute_X` | `cas::integrate::integrate`/`integrate_definite` | internals present, not wired |
| `ode_solution: Option<Expr>` | plain `compute_X` | `cas::ode` | internals present, not wired |
| `series_sum: Option<Expr>` | plain `compute_X` | `cas::sums` | internals present, not wired |
| `solution_set: SolutionSet` | plain `compute_X` | `cas::solve` | internals present, not wired |
| `matrix_ops: MatrixResult` | plain `compute_X` | `cas::matrix` (+ `math::algebra::{MatG,VecG}`) | internals present, not wired |
| `transforms: Expr` | plain `compute_X` | `cas::transforms` | internals present, not wired |

"Internals present, not wired" means: the Rust code that would BE each `compute()` body already exists, compiles, and its own tests pass/fail exactly as before — it just isn't yet called from an `InferredField`/`Inference` impl, because that requires `EquationSnapshot` (§5) to exist first as something to compute FROM, and I did not build that this wave.

## 7. What's NOT done, and why I stopped here

- **`EquationSnapshot` schema addition** (§5's prerequisite) — not implemented. This needs a real `#[derive(ArtifactSchema)]` struct, DSL text/pack codecs (mirroring `MathematicalSnapshot`'s own handcrafted hex/bracket + LEB128 codecs), and wiring into `MathematicalBuilder` — a genuine new-feature surface, not a code-relocation task, and I judged it unsafe to rush under this session's remaining budget.
- **All 10 mutation triads and 12 inferences from §5/§6** — designed, none implemented. Implementing them for real (not stubs) means: `MutationKind` impls with genuine `diff()`-from-`(payload,base)` and `inverse()`-from-`base` for tree-structured `Expr` edits (needs a `NodePath` addressing scheme I have not designed in code), plus `.ts` mirrors for each, plus wiring `roots` as a real `InferredField` with a real `DepHash` chain. This is the actual point of "turn everything into artifacts" and I did not want to deliver 22 rushed, likely-subtly-wrong CQRS leaves over a computer algebra system's edit semantics just to claim completion.
- **Step 3 (repoint)** — trivially satisfied (§4 census: zero consumers), nothing to do.
- **Step 4 (delete from `🧮️math`, remove `#[path]` mounts from `🧮️math/📦️packages/🦀️rust/📦️glue.rs`)** — deliberately NOT done. Per the brief's own ordering, deletion is safe only once the new home's tests run — they do — but the deeper intent of this migration ("artifacts, not library dumps") isn't met by internals-only relocation; deleting from `🧮️math` now would leave the plugin with a second copy of a raw library and no artifact surface on top of it, which is not an improvement over the status quo, just a relocation. I judged it more honest to leave the duplication window open, fully documented, than to close it prematurely.

**Files touched this wave** (all under `✏️s/🔌️plugins/➗️mathematical/`, `🧮️math` completely untouched):
- **Created**: `🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️component.rs`, `…/📈️polynomial-internals/🦀️component.rs`
- **Modified**: `📦️packages/🦀️rust/📦️glue.rs` (mounts + `extern crate semio_framework_math as math;` + doc), `📦️packages/🦀️rust/Cargo.toml` (new dependency)

Next wave should start from §5/§6 directly — the coupling map, verb table, and inference table are the actual design deliverable here, not just documentation of what was skipped.
