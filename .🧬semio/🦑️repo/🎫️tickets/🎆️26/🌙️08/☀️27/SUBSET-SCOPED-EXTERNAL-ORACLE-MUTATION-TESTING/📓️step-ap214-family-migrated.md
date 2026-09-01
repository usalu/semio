# 📐️ The whole `step@ap214` conformance-class family migrated — cc1…cc6

`✳️cc1` was done by hand to prove the pattern. The other five were done by a migrator built from it,
because they are the same aggregate five times over.

## What was migrated

| subset | variants before | leaves after | notes |
|---|---|---|---|
| `✳️cc1` | 5 | 4 | hand-migrated; `remove-shape-representation` is its only ladder verb |
| `✳️cc2`…`✳️cc5` | 6 | 5 | adds `set-shape-representation` + `demote-shape-representation` |
| `✳️cc6` | 5 | 4 | no demotion verb — its ceiling admits every rung |

**28 leaf directories**, each with `🦀️.rs` (payload + `MutationKind` impl) and `🔣️.json` (descriptor).
Every one validated against the derive's own rules BEFORE compiling — owner path, directory name,
hyphenated kind, variant exists in the enum, `kind == kebab(variant)`, verb in `APPROVED_VERBS`:
**28 valid, 0 failing.**

## `demote` is where correction 2 paid off a second time

`DemoteShapeRepresentation` looked unmigratable: `demote` is not one of the 41 approved verbs. It
migrated unchanged anyway, because the KIND and the VERB are different fields and only the verb is
checked:

```rust
SemanticDescriptor { verb: "change", entity: "shape-representation", kind: "demote-shape-representation", record: "DemotedShapeRepresentation" }
```

The vocabulary keeps the name the class documents; the verb comes from the table. Under my earlier
(wrong) reading this variant would have been deleted from four subsets.

## What the migrator does, and what it deliberately does not

It rewrites `Agg::Variant { .. }` to `Agg::Variant(module::Type { .. })` with brace matching — the SAME
transformation for constructions and for patterns, since Rust spells them identically. That is why no
`diff`/`inverse` logic had to be rewritten: the match arms keep their bodies and only their patterns
change shape. The class-neutral edit still happens in one place (`ladder::apply_class_edit`), reached
through `class_diff`/`class_inverse` helpers left `pub(crate)` in each aggregate.

Three things it does NOT do, each found by checking rather than by assumption:

* **External test harnesses** (`🧪️tests/mutate-step-ap214-ccN/🦀️.rs`) are separate crates, so their
  leaf paths must be fully qualified — handled by a second pass.
* **`✳️cc6` has a `🏭️bridge/🦀️component.rs`** that none of its five siblings has. Found by sweeping for
  stale `NoMutation` references after the migration, not by reading the migrator's output.
* **`✳️cc1`'s declaration-gate test** was only partly rewritten by my earlier hand edit, because an
  intermediate replacement had already changed one line and the block no longer matched verbatim.
  Same sweep caught it. Stale references now: **0**.

## Peer activity, attributed rather than chased

Mid-run the build reported 55 `E0433` and 1 `E0599` that vanished on the next build, plus 6 `E0425`
naming `parent_and_index`, `is_contiguous_ascending` and `collect_flattened_leaves` in
`✳️drawing/…/🧷group-nodes/🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`.

Those are not mine. Those subdirectories do not exist in my work — `🔺️diff/🦀️.rs` was created at
**22:42**, after my own last edit to that leaf at **22:39**. Another session is splitting the drawing
leaves into per-facet files right now and its helpers are not yet in scope. Attribution settled by
mtime and by the fact that I never created those paths, not by assuming.

## Confirmed result

```
cargo build -p semio-s-plugin-stdio --offline
  54 error[E0046]     ← was 60; the six migrated aggregates are gone
   6 error[E0425]     ← peer's in-flight ✳️drawing per-facet split
   1 error[E0599]     ← same (`SemioDrawingDiff::absorb` in 🖐️drag-nodes/🔺️diff/🦀️.rs)
errors naming 📐️step: 0
```

## How to scale this to the remaining 54 — the delegation trick

The step family was easy because its per-variant logic already lived behind one shared helper
(`ladder::apply_class_edit`), so each leaf's `diff`/`inverse` is a two-line call. Most aggregates are
not like that: their logic sits in one big `match self` inside `impl Mutation`.

Those do NOT need their logic redistributed by hand. Rename the trait methods to free functions that
take the aggregate, and let every leaf delegate back:

```rust
// aggregate — the ORIGINAL body, verbatim, `self` renamed to `this`
pub(crate) fn agg_diff(this: &Agg, base: &Snap) -> MutationOutcome<Diff> { match this { /* unchanged */ } }
pub(crate) fn agg_inverse(this: &Agg, base: &Snap) -> Vec<Agg> { match this { /* unchanged */ } }

// leaf
fn diff(&self, base: &Snap) -> MutationOutcome<Diff> { agg_diff(&Agg::Variant(self.clone()), base) }
fn inverse(&self, base: &Snap) -> Vec<Agg> { agg_inverse(&Agg::Variant(self.clone()), base) }
```

No recursion: the derive's generated `Mutation::diff` dispatches to the leaf, the leaf calls the free
function, and the free function matches on the aggregate and runs the original arm. Semantics are
preserved by construction rather than by re-derivation — which is the whole risk this migration carries.

The only mechanical edit left is the pattern rewrite, and it is the same brace-matched
`Agg::Variant { .. }` → `Agg::Variant(mod::Type { .. })` used here, because Rust spells constructions
and patterns identically.

## Gotchas worth carrying forward

* **Sweep for stale `NoMutation` after every aggregate.** It is what found `✳️cc6`'s `🏭️bridge/` (no
  sibling has one) and `✳️cc1`'s half-rewritten declaration-gate test.
* **External test harnesses are separate crates** — their leaf paths must be fully qualified.
* **Orphaned builds deadlock the target directory.** Three of mine (one with PPID 1) were queued on the
  same lock with no `rustc` running; killing them was needed before anything would compile. Check
  `pgrep -f "cargo build"` before concluding a build is slow.

# ➕️ Eleven more aggregates — 54 → 43, and four gaps in "mechanical"

After the step family, a GENERIC migrator (delegation: lift `diff`/`inverse` verbatim into free
functions, let each leaf reconstruct its aggregate value and delegate back) was run on eleven more.

**Selected by the criterion that made step safe**, not opportunistically: the other session's sweep owns
one file per aggregate — the `📄set-snapshot` helper directory it is splitting into `🔺️diff/` and
`↩️inverse/`. Only aggregates with NO such directory were touched. That yielded 16; the dry run split
them 11 clean / 5 needing a verb decision (`stamp`, `strip`, `declare` are not approved verbs).

Migrated: `📷️jpg@baseline`, `🖼️tiff@baseline`, `🏗️ifc@cv20/sav/cobie`, and the `📜️docx`/`📕️xlsx`/`🎞️pptx`
strict+transitional pairs. **77 leaf descriptors, validated before compiling: 77 valid, 0 failing.**

## The four gaps, measured at ~215 errors across four rounds

The step family contained none of these, which is exactly why one worked example was not enough:

| # | gap | cost |
|---|---|---|
| 1 | **`Self::`** — bodies are lifted OUT of an `impl`, where `Self` is legal, into free functions where it is not; and a rewrite keyed on `<Agg>::` cannot see `Self::Variant { .. }` either | 181 × `E0433` + a tail of `E0532`/`E0559`/`E0308` |
| 2 | **unit variants become newtype over an EMPTY payload** (`pub struct RemoveTileTags {}`) — `(_)` in a PATTERN, `(mod::Ty {})` in a VALUE | 16 × `E0532`, then 9 × `` `_` can only be used on the left-hand side `` |
| 3 | **construction sites outside `🧪️tests`** — `✏️editor/🦀️component.rs` had them | 2 × `E0559` |
| 4 | **a leaf module is in scope only inside its aggregate** — elsewhere it needs the full crate path, best read off that file's own import | folded into 3 |

## What that says about the word "mechanical"

The TRANSFORMATION is safe: the bodies move verbatim and no semantics are re-derived. What varies per
artifact is the SURFACE the rewrite must reach — scope of `Self`, arity of former unit variants, which
directories hold constructions. A migrator is a hypothesis about that surface, and each new artifact
family tests it.

The descriptor pre-check earned its keep here (105 descriptors this session, 0 failures, all caught
before any build). The four gaps above had no equivalent cheap check, and each cost a full ten-minute
build to discover. All four are now written into
`📜️one-shot/migrate-aggregate-to-leaves-generic.py` along with the run order that works.

## Confirmed

```
cargo build -p semio-s-plugin-stdio --offline
  43 error[E0046]     ← was 60 at session start; 17 aggregates migrated
   6 error[E0425]  ┐
   1 error[E0599]  ┘  all seven in ✳️drawing — the other session's per-facet split
errors naming the 11 migrated subsets: 0
```
