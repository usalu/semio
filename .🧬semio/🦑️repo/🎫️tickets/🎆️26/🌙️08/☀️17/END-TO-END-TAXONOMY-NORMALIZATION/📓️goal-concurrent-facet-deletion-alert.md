# ⚠️ Concurrent session is deleting per-mutation semantic facets

Observed 2026-08-28 ~11:55 CEST, baseline `bb06c41f73`.

## What is happening

The git **index** holds 6 764 staged paths, of which **5 703 are deletions** of per-mutation
facet leaves — `↩️inverse/`, `🔺️diff/`, `🦠️mutation/` — replaced by a single direct
`🧬️mutations/<verb-noun>/🦀️.rs`:

```
D  …/🧬️mutations/✂️disconnect-nodes/↩️inverse/🦀️component.rs
D  …/🧬️mutations/✂️disconnect-nodes/🔺️diff/🦀️component.rs
D  …/🧬️mutations/✂️disconnect-nodes/🦠️mutation/🦀️component.rs
A  …/🧬️mutations/✂️disconnect-nodes/🦀️.rs
```

Across 19 plugins:

| plugin | deleted facet leaves | plugin | deleted facet leaves |
|---|---:|---|---:|
| 📕️norm | 1 692 | 🗄️stdio | 300 |
| 🏛️architect | 1 596 | 🏗️fem | 225 |
| 🧱️block | 624 | 🗒️note | 198 |
| 🎥️shooting | 186 | 📏️layout | 150 |
| 🧩️puzzle | 104 | 💠️lowpoly | 96 |
| ➗️mathematical | 90 | 📐️cad | 90 |
| 🌍️gis | 72 | 💡️reasoning | 60 |
| 🌀️procedural | 54 | 🎞️animate | 54 |
| 🖨️raster | 48 | 📖️playbook | 36 |
| 🌊️flow | 28 | | |

## Attribution — this is NOT this session's fleet

- Every worker in this session is under a standing instruction never to run a mutating git
  command, so none of them can stage anything. These paths are **staged**, by another session plus
  the repository's auto-commit.
- This session's footprint is the **unstaged** worktree set, ≈61 plugin files
  (✒️writer, 🌍️gis, 🌿️vcs, 🎪️demonstrator, 🎬️sequence, 📜️imperative, 🔋️energy, 🔱️trinity,
  🪐️space, 🪵️sourcing) plus `🧹️normalization/🟦️.ts`, the two `✨️derive` crates, and ticket
  documents. The large unstaged blocks (📕️norm 538, 🗄️stdio 283) are the other session mid-write.
- Behaviour differs per plugin: in 🌿️vcs the facets SURVIVE (`pub mod inverse;` still points at
  `↩️inverse/🦀️component.rs`, only the direct leaf was renamed), while in ➗️mathematical the facet
  leaves were deleted outright.

## The SSOT currently sanctions the deletion

`🔣️taxonomy.json`:

```
mutationOptionalFacetDirs = ["🔺️diff", "↩️inverse", "🧩️plan", "📝️text", "💾️binary", "🧬️schema"]
```

`_mutationOwnershipComment`:

> Every concrete `🧬️mutations/<emoji><verb>-<noun>/` directory directly owns one `🦀️.rs`.
> Payload, apply, diff and inverse behavior stay in that leaf; `🔺️diff`, `↩️inverse`, `🧩️plan`,
> `📝️text`, `💾️binary` and `🧬️schema` are **optional organizational facets, never completeness
> requirements**.

So the other session is following the written contract. The dev instruction of 2026-08-28 —
*"Don't remove semantic folders such as inverse and diff per mutation; these are domain-neutral and
you are making it less semantic by removing them"* — **conflicts with that contract**.

## Consequence

Keeping the facets is not something a single session can enforce by restraint: the SSOT text is
what authorises their removal. To make the dev instruction stick, `mutationOptionalFacetDirs` and
`_mutationOwnershipComment` must change from *optional facets* to *required* (or at least
*retained*) per-mutation directories, and the completeness gate must enforce it. Until then any
session reading the SSOT will keep collapsing them.

This session has not deleted any facet directory and its workers carry an explicit guardrail
against doing so.

---

# 📐️ Measured extent and the restoration in flight

## The correct detection predicate

The taxonomy contract landed with a marker-based detector
(`mutationDirectLeafForbiddenRegionMarkers = {"🔺️diff":"🔖️Diff","↩️inverse":"🔖️Inverse"}`).
That **undercounts**: several plugins were inlined with no `//#region` marker at all — every one of
`🏛️architect`'s 266 mutations is like that, the `diff`/`inverse` bodies sitting as plain free
functions. Marker grep finds 815; the behavioural predicate finds 1 167.

A mutation direct leaf is INLINED when

```
(matches ^pub (async )?fn diff\(    AND has no sibling 🔺️diff/ directory)
OR
(matches ^pub (async )?fn inverse\( AND has no sibling ↩️inverse/ directory)
```

Repo-wide over 1 757 Rust mutation direct leaves: **1 167 inlined**, 49 still correctly faceted,
541 with neither.

| plugin | inlined | plugin | inlined |
|---|---:|---|---:|
| 📕️norm | 371 | 🏛️architect | 266 |
| 🗄️stdio | 113 | 🧱️block | 104 |
| 🏗️fem | 50 | 📸️remodel | 35 |
| 🗒️note | 33 | 🎥️shooting | 31 |
| 🧩️puzzle | 26 | 📐️cad | 20 |
| 💠️lowpoly | 17 | 🏭️process | 16 |
| ➗️mathematical | 15 | 🌍️gis | 12 |
| 🖨️raster | 12 | 💡️reasoning | 10 |
| 🌀️procedural | 9 | 🌊️flow | 9 |
| 🎞️animate | 9 | 📖️playbook | 9 |

Still correctly faceted: 🔱️trinity 15, 🎬️sequence 8, 🌿️vcs 6, 🪐️space 5, ✒️writer 4,
📜️imperative 4, 🪵️sourcing 3, 🌍️gis 2.

## A latent bug the inlining introduced

Moved bodies kept `super::`-qualifiers that were correct one module deeper. In `🏛️architect`:

```rust
pub async fn inverse(payload: &super::DeleteProgramElement, base: &ProgramSnapshot) -> …
    … super::super::create_program_element::CreateProgramElement …
```

`super::DeleteProgramElement` now names a type declared in the SAME file. Restoring each body to
its facet file makes the original qualifiers correct again — which is why the restoration reads
from the pinned commit rather than moving the inlined text.

## Restoration method (7 workers in flight)

Authoritative source: commit `bb06c41f73f0122fbed315b7487428b976f99921` still holds every deleted
facet file at its original path. Per mutation directory `D`:

1. `D/🔺️diff/🦀️.rs` and `D/↩️inverse/🦀️.rs` written from the pinned originals — restored as
   **kind-only** leaves, not the original `🦀️component.rs`.
2. The inlined functions (and any `//#region` wrapper) removed from `D/🦀️.rs`.
3. The `MutationKind` impl delegates, exactly as `🌿️vcs/🏷️add-tag` still does:
   `super::diff::diff(self, base)` / `super::inverse::inverse(self, base)`.
4. The plugin crate root `📦️packages/🦀️rust/📦️glue.rs` mounts each facet with `#[path]` beside
   the existing `mod component;`.

## Separate, larger finding: the TypeScript surface was deleted outright

Of the deleted facet leaves, ~3 255 are `🟦️component.ts`, and these were **not** inlined anywhere
— over 516 TypeScript mutation direct leaves the inlining predicate finds **0**. The deleted files
were type-level mirrors:

```ts
/** 🔺️ Mirrors `diff(payload, base)` → ProgramDiff (see sibling 🦀️component.rs for the real
 *  handcrafted logic — this is a type-level mirror only). */
export type DiffDeleteProgramElement = (payload: DeleteProgramElement, base: ProgramSnapshot) => ProgramDiff;
```

`🏛️architect`'s mutation directories now contain no TypeScript file at all. Mutation descriptors
declare `requiredLanguageSurfaces` including `typescript`, so this is a multi-implementation
regression on a second axis. **Not attempted here** — the Rust restoration is 1 167 leaves already,
and this is a further ~3 000-file decision that needs its own call.
