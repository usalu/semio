# 🛑️ The generic migration: built, dry-run over all 54, attempted, and stopped — with cause

After the step family landed by hand + a family script, the obvious next move was a GENERIC migrator so
the remaining 54 aggregates could follow. It was written, dry-run across every candidate, and then run
for real on one aggregate. It is now stopped, and the reason is not caution — it is a timestamp.

## The generic approach, and why it is sound

Most aggregates keep their per-variant logic in one big `match self` inside `impl Mutation`. The
migrator does not redistribute that. It lifts the two method bodies VERBATIM into free functions over
the aggregate and has each leaf delegate back:

```rust
pub(crate) fn agg_diff(this: &Agg, base: &Snap) -> MutationOutcome<Diff> { /* original body */ }
// leaf: fn diff(&self, base) { agg_diff(&Agg::Variant(self.clone()), base) }
```

Semantics preserved by construction. Leaves take `use super::*;` so they inherit the aggregate's
imports without any per-file import analysis.

**Dry run over all 54 candidates: 43 clean, 9 skipped**, each skip naming its reason —
3 have no hand-written `impl Mutation`, 1 is already newtype, and 5 need a VERB DECISION because their
first token is not approved: `stamp` (`StampBaseProfile`), `strip` (`StripNonTiny`), `splice`,
`truncate` (`TruncateAt`), `declare` (`DeclareDoctype`/`DeclareEntity`), `upsert` (`UpsertInstance`).
Those five are the genuine judgment calls; the other 43 are mechanical.

## Two bugs it hit, both caught before any build

* **`POOL="🔧🔩…".split()` returned the whole string as ONE element**, so all 14 las leaves were created
  in directories prefixed with the entire emoji pool. Caught by looking at `ls`, not by compiling.
  Fixed to an explicit list, and a guard added that refuses any directory name that is not
  `<one-grapheme-emoji><kind>`.
* **`set-snapshot` already exists as a leaf in most aggregates**, so the migrator created a DUPLICATE
  (`🔧set-snapshot` beside the existing `📄set-snapshot`). A generic migrator must reuse existing leaf
  directories by kind, never author a second one.

las was restored to its exact pre-edit state from the git index (`git show :<path>` — a read, not a
checkout), and all 14 generated directories removed. `git diff` on the artifact is clean of my work.

## Why it is stopped: another session is doing this migration, right now, in a different shape

The existing `set-snapshot` leaves are not stubs. They carry `🔺️diff/`, `↩️inverse/` and `🧪️tests/`
subdirectories — a PER-FACET leaf layout, not the single-file shape this migrator emits. And they are
moving under us:

```
Aug 28 23:16:16  📄set-snapshot/🦀️.rs          ← 19 lines removed
Aug 28 23:16:16  📄set-snapshot/🔺️diff/🦀️.rs
Aug 28 23:16:16  📄set-snapshot/↩️inverse/🦀️.rs
```

All three written in the same second, by an automated split that moves code out of `🦀️.rs` into the two
facet files. The same signature appears in `✳️drawing` (`🔺️diff/🦀️.rs` created 22:42, after my own
22:39 edit) and is the source of the 6 `E0425` + 1 `E0599` currently in the build.

Running a mechanical migration across 43 aggregates against that would duplicate their leaves, emit a
second incompatible leaf shape, and race their edits file by file. The step family was safe precisely
because `📐️step/🔖️ap214` is not where they are working.

## What is left, stated for whoever continues

The migrator is preserved at `📜️one-shot/migrate-aggregate-to-leaves-generic.py`. Before it can be run
at scale it needs one change — **reuse an existing leaf directory when one already exists for a kind** —
and one decision: whether leaves are single-file (what it emits) or per-facet (what the other session is
authoring). That is a coordination question, not a code question, and it belongs to whoever owns the
per-facet split.


## Correction: the existing `set-snapshot` directories are not leaves at all

The section above says the existing `set-snapshot` leaves "are not stubs" and carry per-facet
subdirectories. The second half is true; the first half is wrong in a way that matters, and reading one
of them settles it:

```rust
// ☁️las/…/🧬️mutations/📄set-snapshot/🦀️.rs — the WHOLE file
use crate::artifacts::las::LasSnapshot;
use crate::artifacts::las::schema::diff::{LasDiff, diff_set_snapshot};
use crate::artifacts::las::schema::mutations::{LasMutation, apply_las_mutation};

pub fn apply(projection: &mut LasSnapshot, mutation: &LasMutation) -> protocol::MutationOutcome<LasDiff> {
    apply_las_mutation(projection, mutation)
}
```

No payload struct, no `#[derive(dsl::MutationLeaf)]`, no `impl MutationKind`. It is an APPLY-HELPER
directory that happens to be named after a kind — not a migrated leaf. `dsl::Mutations` would not
accept it, and my migrator creating `🔧set-snapshot` beside it would have produced two directories
claiming the same `semanticKind`, which the descriptor roster rejects outright.

## Which makes the collision exact, not merely likely

To migrate any of these aggregates, the `📄set-snapshot` directory must BECOME a leaf — its `🦀️.rs`
gaining a payload struct and a `MutationKind` impl. That file is precisely what the other session's
sweep is rewriting: it moved 19 lines out of las's copy into `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`,
all three written at 23:16:16.

So the conflict is not "they are working nearby". It is that the single file each aggregate's migration
must edit first is the file their sweep is currently rewriting, in every artifact — 113 such files, 101
of them in `🧿️semio` where their newest edits are.

The `📐️step/🔖️ap214` family was migratable for a reason that is now precise: those six subsets have NO
`📄set-snapshot` helper directory, so nothing of theirs had to be touched.

## What this changes for whoever continues

The migrator needs a third change beyond the two named above: when a directory already exists for a
kind, do not create a sibling and do not assume it is a leaf — CONVERT it, preserving whatever helper
API it already exposes (`apply`, and the `🔺️diff`/`↩️inverse` facets if the split has reached it). That
conversion is the coordination point with the other session's work, and doing it blind in 43 artifacts
while their sweep runs would corrupt both.
