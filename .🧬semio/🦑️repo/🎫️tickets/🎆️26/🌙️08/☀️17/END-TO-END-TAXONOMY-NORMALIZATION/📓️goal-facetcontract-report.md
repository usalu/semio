# Facet Contract Reversal — Diff/Inverse/Mutation Are Placement-Bound, Never Inlinable

## Background

Earlier the same day, a sibling session deleted 5,703 per-mutation facet leaves (1,885 `↩️inverse/`,
1,885 `🔺️diff/`, 1,933 `🦠️mutation/`) across 19 plugins, inlining diff/inverse/apply behavior into
each mutation's direct `🦀️.rs` under `//#region 🔖️Diff` / `//#region 🔖️Inverse` / `//#region 🔖️Payload`
sections. That followed the letter of the then-current `🔣️taxonomy.json` contract:
`mutationOptionalFacetDirs` listed `🔺️diff`, `↩️inverse`, `🧩️plan`, `📝️text`, `💾️binary`, `🧬️schema` as
**all equally optional, never a completeness requirement** — nothing in the vocabulary said diff/inverse
behavior was forbidden from living inline.

The dev overruled that outcome: folders carry semantics, files are kind-only leaves. `↩️inverse` and
`🔺️diff` must never be re-collapsed into the direct leaf. This ticket changes the *contract*, not the
plugin trees (sibling workers are splitting those back out concurrently across the 19 plugins).

## Old rule vs new rule

**Old** (`mutationOptionalFacetDirs`, one flat list of 6): every facet dir — `🔺️diff`, `↩️inverse`,
`🧩️plan`, `📝️text`, `💾️binary`, `🧬️schema` — was "optional organizational, never a completeness
requirement." No distinction between behavior and organization; nothing forbade inlining.

**New**: the six-item list is split by what the dir actually carries, and a PLACEMENT rule is added for
the behavior half:

- `mutationBehaviorFacetDirs: ["🦠️mutation", "🔺️diff", "↩️inverse"]` — carries the mutation's own
  apply/payload-definition, diff, and inverse *behavior*. **Required-when-present, never inlinable**:
  whenever that behavior exists for a mutation it must live in its own facet directory, never inside the
  direct leaf. `🦠️mutation` is legitimized here as a genuine sibling facet for the "apply" behavior
  (already the real shape of e.g. `stdio/🧿️semio/…/📸️set-snapshot/🦠️mutation/`), coexisting with — never
  substituting for — the mandatory direct `🦀️.rs`; this does not touch the separate, pre-existing
  wave‑2b "legacy central-only nesting" migration engine in `📜️script.ts` (`inventoryMutationTaxonomy` /
  `planMutationTaxonomy`), which only fires when a mutation has **no** direct leaf at all — an orthogonal,
  still-valid concern.
- `mutationDirectLeafForbiddenRegionMarkers: {"🔺️diff": "🔖️Diff", "↩️inverse": "🔖️Inverse"}` — the exact
  region marker whose presence in a mutation's direct leaf proves that facet's behavior was inlined
  instead of split out. (No entry for `🦠️mutation`: its legitimate inline marker, `🔖️Mutation`, is also
  the marker used for the payload+dispatch wrapper that legitimately stays inline — e.g. `🌿️vcs/🏷️add-tag`
  — so presence of that marker alone can't be treated as a violation signal the way `🔖️Diff`/`🔖️Inverse`
  can.)
- `mutationOrganizationalFacetDirs: ["🧩️plan", "📝️text", "💾️binary", "🧬️schema"]` — unchanged in
  membership from the old list's remainder; genuinely optional, never a completeness requirement.

`_mutationOwnershipComment` was rewritten to state the placement rule instead of "optional... never
completeness requirements" (the wording that licensed today's collapse).

`taxonomyLeafParentDirs` gained `🦠️mutation` (alongside the already-present `🔺️diff`/`↩️inverse`), since
all three behavior facets must be able to parent their own component leaf.

## Consumers updated

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `_mutationOwnershipComment`
  rewritten; `mutationOptionalFacetDirs` replaced by `mutationBehaviorFacetDirs` +
  `mutationDirectLeafForbiddenRegionMarkers` + `mutationOrganizationalFacetDirs`; `🦠️mutation` added to
  `taxonomyLeafParentDirs`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` — `Taxonomy` type
  (new `mutationBehaviorFacetDirs`/`mutationDirectLeafForbiddenRegionMarkers`/
  `mutationOrganizationalFacetDirs` fields replacing the old one); `artifactFacetChildLevel`'s mutation
  branch now unions both facet lists; `validateTaxonomy` checks canonical order for both new lists, that
  every entry registers in `taxonomyLeafParentDirs`, and that the region-marker map is exactly
  `{"🔺️diff":"🔖️Diff","↩️inverse":"🔖️Inverse"}` (replacing the old single-list check and the now-obsolete
  "must not admit legacy nested 🦠️mutation" guard); `directoryValues` aggregation spreads both new lists;
  added new exported gate `mutationDirectLeafInlinedBehaviorFacets(source, taxonomy)`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` — updated
  the `loadTaxonomy` shape assertions, the "registers every ... facet as a taxonomy leaf parent" test, the
  "declares direct mutation ownership ..." test, flipped `taxonomyLeafParentDirs` to `toContain("🦠️mutation")`
  (was `not.toContain`), flipped the `artifactFacetPathIsDeclared(".../add-node/🦠️mutation")` assertion
  from `false` to `true` and folded it into the shared facet loop, and added the new test described below.
  Left the separate wave-2b `policyMutationStructuralBreaches`/`inventoryMutationTaxonomy` fixtures
  (central-only-nesting migration, `describe("direct mutation ownership", …)`) untouched — different,
  still-valid invariant (no direct leaf at all), not this ticket's placement rule.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — `TAXONOMY_MUTATION_OPTIONAL_FACET_DIRS`
  renamed to `TAXONOMY_MUTATION_FACET_DIRS`, now the union of both new taxonomy arrays (3 call sites
  updated: two "if facet dir exists it must not be empty" walks, one `taxonomyLeafParents` aggregation).
- `📜️script.ts` (root) — line ~27865's `policyStructuralMutationChildren` classification now checks
  membership in `mutationBehaviorFacetDirs` OR `mutationOrganizationalFacetDirs` (was the single old list).

No other repo file referenced `mutationOptionalFacetDirs` (verified via a repo-wide grep excluding
`node_modules`/`target`).

## Test added

New test in `🧪️index.test.ts`, `describe("direct mutation taxonomy", …)`:

```
test("flags a direct leaf that inlines diff/inverse behavior and accepts the split add-tag shape", () => {
  expect(mutationDirectLeafInlinedBehaviorFacets(INLINED_MUTATION_DIRECT_LEAF_FIXTURE).sort()).toEqual(["↩️inverse", "🔺️diff"]);
  expect(mutationDirectLeafInlinedBehaviorFacets(SPLIT_MUTATION_DIRECT_LEAF_FIXTURE)).toEqual([]);
});
```

`INLINED_MUTATION_DIRECT_LEAF_FIXTURE` mirrors the real (pre-fix) `➗️mathematical/…/❌️delete-node/🦀️.rs`
shape (`//#region 🔖️Payload` + inlined `//#region 🔖️Diff` + `//#region 🔖️Inverse` bodies) — literal
strings, not a live read of plugin files, since sibling sessions are actively rewriting those trees.
`SPLIT_MUTATION_DIRECT_LEAF_FIXTURE` mirrors the real `🌿️vcs/…/🏷️add-tag/🦀️.rs` shape (only
`//#region 🔖️Mutation` wrapping payload + a dispatch impl that delegates to `super::diff::diff` /
`super::inverse::inverse`). This is the language-agnostic gate: it operates on the leaf's text, not a
Rust-specific AST, so the same check applies regardless of which language backs a given mutation.

The gate function itself, added to `discovery/🟦️component.ts`:

```ts
export function mutationDirectLeafInlinedBehaviorFacets(directLeafSource: string, taxonomy: Taxonomy = loadTaxonomy()): readonly string[] {
  const markers = taxonomy.mutationDirectLeafForbiddenRegionMarkers ?? {};
  return Object.entries(markers)
    .filter(([, marker]) => new RegExp(`//#region\\s+${marker.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}\\b`, "u").test(directLeafSource))
    .map(([facet]) => facet);
}
```

No `.vscode/launch.json` change: no new nx target/CLI subcommand was added, only a plain exported function
covered by the existing `test` target.

## Real test output

Scoped run (everything I touched — `loadTaxonomy`, `validateTaxonomy`, `direct mutation taxonomy`):

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript
$ bun test 🧪️index.test.ts --test-name-pattern "loadTaxonomy|validateTaxonomy|direct mutation taxonomy"

bun test v1.3.14 (0d9b296a)

 52 pass
 477 filtered out
 0 fail
 338 expect() calls
Ran 52 tests across 1 file. [2.97s]
```

Broader run (`--test-name-pattern "mutation"`, 54 tests) surfaced 26 pre-existing failures, all inside
the *separate* `describe("direct mutation ownership", …)` / `describe("mutation metadata facts", …)`
blocks (the wave-2b `policyMutationStructuralBreaches`/`inventoryMutationTaxonomy` engine, and an
unrelated Rust-alias-parsing fixture) — none in code this ticket touched. Every failure traces to one of:
`Source admission root has unsafe ancestry: /var` (macOS `tmpdir()` resolves through a `/var` → `/private/var`
symlink that `sourceAdmissionPrepareOptions` rejects), `fatal: not a git repository` (those tests need
`SEMIO_TEST_ARTIFACT_DIR` pointed inside the real repo tree; absent that env var they run `git ls-files`
against a bare tmp dir), and one unrelated `restricted: true` field mismatch in an alias-parsing fixture.
None reference `mutationBehaviorFacetDirs`, `mutationOrganizationalFacetDirs`,
`mutationDirectLeafForbiddenRegionMarkers`, or `🦠️mutation`, and they are pre-existing environmental
issues, not regressions from this change.

## For the sibling plugin-splitting workers

Match these exact vocabulary names when splitting the 19 plugins back out:

- Facet dirs to restore: `🦠️mutation`, `🔺️diff`, `↩️inverse` (`mutationBehaviorFacetDirs`).
- The direct `🦀️.rs` must never contain `//#region 🔖️Diff` or `//#region 🔖️Inverse` again — that's the
  literal signal `mutationDirectLeafInlinedBehaviorFacets` checks for.
- The direct leaf keeps its payload struct + a dispatch impl that *delegates* (`super::diff::diff(...)`,
  `super::inverse::inverse(...)`), matching `🌿️vcs/🏷️add-tag`'s shape exactly.

## Addendum (2026-08-28, post-review): detector defect fixed — marker was not the contract

The coordinator found a real defect after a live plan run: `mutationDirectLeafInlinedBehaviorFacets`
detected inlining by `//#region` comment marker only. The other session's collapse inlined many mutations
with **no marker at all** — plain free `pub (async)? fn diff(...)`/`fn inverse(...)` sitting in the direct
leaf. Marker-based detection was blind to every one of those (all 266 `🏛️architect` mutations, confirmed:
`✂️🧲disconnect-adjacency` etc. inline with zero `//#region`).

### Fix

`mutationDirectLeafInlinedBehaviorFacets` in `discovery/🟦️component.ts` now takes a `siblingFacetDirs:
ReadonlySet<string>` parameter and unions two independent signals:

1. **Structural (the actual contract now)**: `MUTATION_DIRECT_LEAF_STRUCTURAL_PATTERNS` —
   `/^pub (?:async )?fn diff\(/mu` for `🔺️diff`, `/^pub (?:async )?fn inverse\(/mu` for `↩️inverse` —
   matched against the direct leaf source AND the corresponding sibling directory absent. Column-zero
   anchor deliberately excludes the indented trait-impl delegate (`async fn diff(&self, ...) { diff(self,
   base) }`), which is required to stay inline.
2. **Marker (kept, not dropped)**: the existing `mutationDirectLeafForbiddenRegionMarkers` scan. It is
   *not redundant* — it is the only signal for a non-Rust direct leaf (a `🟦️.ts` mirror has no "free `pub
   fn diff(`" concept) and for any Rust shape that carries the marker without the bare free-function form.
   It adds coverage the structural check cannot reach on its own; the structural check is what makes the
   contract actually enforceable (an author can't evade a syntax shape by omitting a comment).

Test extended: added `MARKER_FREE_INLINED_MUTATION_DIRECT_LEAF_FIXTURE` (literal, mirrors the real
`🏛️architect/…/🗑️🧱delete-program-element/🦀️.rs` shape exactly — free async fns, zero markers, no
sibling dirs) to `describe("direct mutation taxonomy", …)`; asserts it's flagged with an empty sibling
set and cleared once `🔺️diff`/`↩️inverse` sibling dirs are present.

### Real test output (post-fix)

```
$ bun test 🧪️index.test.ts --test-name-pattern "loadTaxonomy|validateTaxonomy|direct mutation taxonomy"
bun test v1.3.14 (0d9b296a)
 52 pass
 477 filtered out
 0 fail
 340 expect() calls
Ran 52 tests across 1 file. [10.44s]
```

### Repo-wide re-measurement (measured 2026-08-28T20:38Z, restoration actively in flight — expect drift)

Script: `🗑️temp/measure_inlined2.mjs` (same predicate as the fixed function, run directly against
`✏️s/🔌️plugins` via `find … -path '*/🧬️mutations/*/🦀️.rs'`, excluding `📚️examples`/`💾️binary`/`📝️text`
non-mutation leaves picked up by the glob):

| | total | marker-based | structural union | still faceted |
|---|---|---|---|---|
| coordinator's snapshot | 1,757 | 815 | 1,167 | 49 |
| this measurement | 1,672 | 875 | **1,227** | 445 |

Structural detection stayed well above marker-only (1,227 vs 875, +40%), confirming the fix closes the
blind spot; absolute counts moved (total −85, faceted 49→445) because seven sibling sessions are
restoring facets concurrently — both totals are snapshots, not a fixed target.

## Answer to the open question: what does the contract mean for `🦠️mutation`?

Investigated by diffing sampled deleted `🦠️mutation/🦀️component.rs` paths against the pre-collapse
commit `bb06c41f73`:

- `🏛️architect/…/✂️🧲disconnect-adjacency`, `…/✂️🧵disconnect-trace`,
  `…/✏️ℹ️rename-information-requirement` (3/3 sampled): **no direct `🦀️.rs` existed at `bb06c41f73`** —
  the entire payload, dispatch, and diff/inverse-delegation lived inside `🦠️mutation/🦀️component.rs`.
- `🗄️stdio/…/📸️set-snapshot` (the example I originally cited as a "legitimate coexisting sibling facet"
  when designing the contract): **also has no direct `🦀️.rs`**, at `bb06c41f73` or at current HEAD.

Every confirmed real instance — past and present — is the *wave-2b "legacy central-only nesting"* shape
(`inventoryMutationTaxonomy`'s `state === "legacy"`: no direct leaf, entire mutation nested one level
down), never a facet coexisting with a mandatory direct leaf. I have found **zero confirmed instances**
of the "sibling facet beside a direct leaf" shape I assumed justified listing `🦠️mutation` in
`mutationBehaviorFacetDirs` as "required-when-present, never inlinable" on the same footing as
`🔺️diff`/`↩️inverse`.

**Precise, current meaning of the contract for `🦠️mutation`:** it is *declarative vocabulary only*.
`mutationBehaviorFacetDirs` registers `🦠️mutation` as a legal directory name (so `artifactFacetPathIsDeclared`
accepts it, `taxonomyLeafParentDirs` lets it parent a leaf) — but the enforcement gate,
`mutationDirectLeafInlinedBehaviorFacets`, deliberately does **not** check it: no forbidden region marker,
no structural pattern. It does not mandate `🦠️mutation`'s presence anywhere, and it does not flag its
absence as a violation.

**Consequence for the 1,164 deleted `🦠️mutation/🦀️component.rs` files:** for every sampled case, deleting
them was the wave-2b *lossless legacy→direct cutover* (`🦠️mutation/component.rs` content moved to become
the direct leaf), which this ticket's contract does not forbid — it is a different, still-valid transform
from the diff/inverse inlining regression this ticket targets. **No follow-up restoration pass is owed
for `🦠️mutation`.** The seven sibling workers restoring only `🔺️diff`/`↩️inverse` are doing the right
thing already.

**Not acted on unilaterally, flagged for the dev:** since no confirmed legitimate "sibling facet beside a
direct leaf" use of `🦠️mutation` exists in this repo today, `mutationBehaviorFacetDirs` including it may
be aspirational/premature rather than descriptive of a real invariant. Options for a follow-up decision:
(a) leave it as forward-looking vocabulary (harmless — it's inert in the gate), (b) drop it from
`mutationBehaviorFacetDirs` until a real coexisting-instance exists, (c) keep it but rename/re-comment to
make explicit it is not currently gate-enforced. I have not changed `🔣️taxonomy.json` for this point this
turn — awaiting the dev's call.
