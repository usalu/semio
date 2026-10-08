# 📋️ Diff-Only Mutations — Target Design And Violation Taxonomy

## Laws

Notation: `m` is a mutation of artifact snapshot type `P` with diff type `D`, `base: &P`.

- **L1 — Declarative concrete diff.** `m.diff(base) -> MutationOutcome<D>` builds the diff *declaratively* from the
  payload plus reads of `base`. The diff names exactly the entities/fields it changes with their new values
  (sparse, artifact-specific, typed). It never carries a whole after-snapshot, never is computed by mutating a copy of
  `base` and differencing (`D::between(base, &mutated)`), and never is a generic value/JSON patch.
- **L2 — Concrete inverse.** `m.inverse(base) -> Vec<Op>` returns concrete mutations of the same artifact, built from
  the payload plus reads of `base` inside the leaf's own `↩️inverse/🦀️.rs`. It is never derived generically from the
  forward diff (`*_inverse(base, outcome)`, `inverse_from_diff`, walking `m.diff(base).diff()`), never a
  whole-snapshot restore mutation, never a generic "apply this diff" mutation, never `Vec::new()` for a change that
  actually alters state.
- **L3 — Inverse diffs sum to the negative diff.** With `after = m.diff(base).apply(base)`, `s₀ = after`,
  `dₖ = invₖ.diff(sₖ₋₁)`, `sₖ = dₖ.apply(sₖ₋₁)`, `Σ = d₁ ⊕ … ⊕ dₙ` (`MutationDiff::absorb`):
  `Σ.apply(after) == base` and, where the artifact implements `DiffAlgebra`, `Σ == m.diff(base).diff().inverse(base)`.
- **L4 — Central apply only.** Only the framework's central applier turns a diff into a snapshot. No mutation leaf,
  `MutationKind` impl, hand-written `impl Mutation<P>`, diff builder or inverse builder calls `MutationDiff::apply`,
  takes `&mut P`, or clones `base` to write fields into it.
- **L5 — Impossible by design.** `MutationDiff::apply` requires an `ApplyCapability` that only the central applier
  (`protocol::apply_diff` and the store/vcs folds built on it) can mint, so leaf code cannot call it. Policy gate rules
  in `verify mutation-outcome-law` reject the remaining textual loopholes (`&mut <Snapshot>` in leaves, base clones
  mutated in leaves, `between(` in leaves, generic inverse helpers).

## Violation classes (audit vocabulary)

| Code | Meaning |
|---|---|
| `V1-SNAPSHOT-DIFF` | diff computed by mutating a clone of base and differencing / full-replacement diff |
| `V1-GENERIC-DIFF` | diff is a generic value/JSON patch or whole-collection replace instead of a sparse typed diff |
| `V2-DIFF-DERIVED-INVERSE` | inverse built by walking the forward diff via a shared generic helper |
| `V2-RESTORE-INVERSE` | inverse restores a whole snapshot/collection or wraps a generic diff |
| `V2-EMPTY-INVERSE` | inverse returns nothing for a state-changing mutation |
| `V3-LEAF-APPLY` | leaf/kind/diff/inverse calls `.apply(` or takes `&mut P` / mutates base clone |
| `V3-HAND-MUTATION` | hand-written `impl Mutation<P>` that bypasses `#[derive(Mutations)]` and applies directly |
| `V4-LAW-UNTESTED` | no L3 law test for the leaf |

## Rulings

- **Minimality.** A diff carries exactly the fields the mutation semantically owns. A `replace-<entity>` kind may set its
  whole entity record; a `rename-`/`set-<field>`/`move-` kind that emits a full entity record, a whole list, or a whole
  sub-document is `V1-GENERIC-DIFF`. Collections are diffed per id/index (`added`/`removed`/`modified` keyed rows), never
  as a whole-`Vec` replacement.
- **Absorb soundness.** `absorb` must coalesce same-key entries (patch∘patch → one patch, create∘delete → nothing,
  delete∘create → replace), otherwise L3's sum is ill-defined. An unsound `absorb` is a violation of its own (`V5-ABSORB`).
- **Generic seams are deleted, not wrapped.** `diff_from_model`, `graph_edit_diff`, `commit_value_tree_edit`,
  `*_selection_inverse`, `Restore`/`SetSnapshot` inverse variants, `apply_in_place`, whole-snapshot config/window diffs and
  `MutationOutcome::apply_to(&mut P)` are removed; each kind gets its own concrete diff/inverse in its leaf directory.

## Framework API (fixed contract — plugin executors code against this)

In `protocol` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`):

```rust
/// 🔑️ Proof the caller is the central applier. Minted only by `apply_diff`.
#[derive(Clone, Copy, Debug)]
pub struct ApplyCapability { _sealed: () }

pub trait MutationDiff<P>: Clone + Default + PartialEq + ToValue + FromValue + DiffAlgebra<P> {
    fn apply(&self, base: &P, capability: ApplyCapability) -> MutationApplyResult<P>;
    fn absorb(&mut self, other: Self);            // must coalesce same-key entries (V5-ABSORB)
    fn retire_cold(self) where Self: Sized {}
    fn retire_projection(projection: P) { drop(projection); }
}

pub trait DiffAlgebra<P>: Sized {
    fn inverse(&self, base: &P) -> Self;          // the NEGATIVE diff: inverse(d, base).apply(d.apply(base)) == base
    fn between(base: &P, other: &P) -> Self;      // sync/import only — forbidden in mutation leaves (gate)
    fn is_empty(&self) -> bool;
}

/// 🎯️ THE central applier — the only mint point of `ApplyCapability`.
pub fn apply_diff<P, D: MutationDiff<P>>(diff: &D, base: &P) -> MutationApplyResult<P>;
```

- `MutationOutcome::apply_to(&mut P)` is DELETED. Every caller uses `protocol::apply_diff(outcome.diff(), &base)`.
- A composite diff (enum over sub-diffs) forwards the `capability` it received to its sub-diffs' `apply`; it never mints one.
- `DiffAlgebra` is a supertrait: every diff type implements a concrete `inverse` (negative diff).
- Law helper (in `📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`, exported next to `assert_mutation_inverse_law`):
  `assert_mutation_inverse_sum_law<P: Clone + PartialEq + Debug, M: Mutation<P>>(mutation: &M, base: &P)` asserts, for an
  applied outcome: (1) inverse non-empty when `after != base`; (2) sequential inverse replay restores `base`;
  (3) `Σ = absorb(d₁..dₙ)` satisfies `apply_diff(Σ, after) == base`; (4) `canon(Σ) == canon(d.inverse(base))` where
  `canon(x) = { let mut c = D::default(); c.absorb(x); c }`.
- Every mutation leaf's `🧪️tests` calls `assert_mutation_inverse_sum_law` on each of its applied fixtures.

## Composite mutations ruling

`fold_plan_diff`/`fold_plan_inverse` stay (a composite is by definition a sequence of concrete child mutations); they
build intermediate states only through `protocol::apply_diff`, and the composite inverse is the reversed concatenation of
the children's concrete inverses.

## Build rules for every executor

- Wrap EVERY cargo call: `"$T/🚦️gate.sh" <your-label> -- cargo check ...` (T = this ticket folder, absolute path).
- Foreground only; never `run_in_background`/`Monitor`; never set `CARGO_TARGET_DIR`; prefer
  `cargo check -p <crate> --target wasm32-wasip2 --message-format=short` for plugins, `cargo check -p <crate>` for framework.
- Errors only inside `🧰️framework/**/🖥️host/**` or other peers' crates are not yours — note them and move on.
- "WRITTEN BUT UNVERIFIED" with a precise list is an acceptable report.
