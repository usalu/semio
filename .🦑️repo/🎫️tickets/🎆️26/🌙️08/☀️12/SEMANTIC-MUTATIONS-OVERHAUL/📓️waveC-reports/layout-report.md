# FacetReport — layout (Wave C)

## facet
`layout` (single artifact, `s.layout.layout`, `🗿️artifacts/📏️layout/…/🧬️mutations`).

## status
`done` — funnel debt fixed, directory/glue trueing complete, emoji uniqueness fixed, config
semanticized, TS mirrors added, schema description files rewritten, law-test coverage extended,
plus one real pre-existing test-only bug found and fixed (missing `SemanticMutation` trait import
— see **gates**). `cargo check -p semio-s-plugin-layout` CONFIRMED clean of layout-originated
errors (only the 3 pre-existing, out-of-scope stdio `PdfSnapshot` schema-drift errors remain).
`cargo test --lib` confirmed down to those same 3 errors and nothing else. Policy gate confirmed
zero new relevant high-priority breaches.

## mutationsCreated
None — the artifact-root dispatch enum (`🧬️mutations/🦀️component.rs`) already carried all 25 real
semantic `LayoutMutation` variants from an earlier wave (confirmed by that file's own doc comment
and its pre-existing test suite). This session's job was funnel/directory/glue trueing, not
vocabulary derivation. For completeness, the 25 kind → verb → (superseded generic) map:

| slug (final emoji) | verb | entity | superseded generic |
|---|---|---|---|
| `✏️rename-layout` | rename | layout | — |
| `🖨️change-print-target` | change | print-target | — |
| `🧾change-data-fields` | change | data-fields | `SetDataFields` (app funnel) |
| `🌱create-page` | create | page | `Pages(CollectionMutation::Add)` |
| `🗑️delete-page` | delete | page | `Pages(CollectionMutation::Remove)` |
| `🏷️rename-page` | rename | page | `Pages(CollectionMutation::Patch)` (name field) |
| `↔️change-page-width` | change | page-width | `Pages(CollectionMutation::Patch)` (width field) |
| `↕️change-page-height` | change | page-height | `Pages(CollectionMutation::Patch)` (height field) |
| `📐update-page-margins` | update | page-margins | `Pages(CollectionMutation::Patch)` (margin fields) |
| `🏛️update-page-columns` | update | page-columns | `Pages(CollectionMutation::Patch)` (columns fields) |
| `🔀reorder-pages` | reorder | pages | — |
| `📖create-story` | create | story | `Stories(CollectionMutation::Add)` |
| `📕delete-story` (renamed, was `🗑️`) | delete | story | `Stories(CollectionMutation::Remove)` |
| `📝edit-story` | edit | story | `Stories(CollectionMutation::Patch)` |
| `🖇️create-link` | create | link | `Links(CollectionMutation::Add)` |
| `✂️delete-link` (renamed, was `🗑️`) | delete | link | `Links(CollectionMutation::Remove)` |
| `🔗change-link-path` | change | link-path | `Links(CollectionMutation::Patch)` |
| `➕create-frame` | create | frame | `AddFrame` |
| `➖delete-frame` | delete | frame | `PatchFrame`-adjacent removal path |
| `🕹️move-frame` | move | frame | `PatchFrame` (x/y fields) |
| `📏resize-frame` | resize | frame | `PatchFrame` (width/height fields) |
| `🎨change-frame-fill` | change | frame-fill | `PatchFrame` (fill field) |
| `🖊️change-frame-stroke` | change | frame-stroke | `PatchFrame` (stroke field) |
| `🔤change-frame-wrap-mode` | change | frame-wrap-mode | `PatchFrame` (wrapMode field) |
| `🔢change-frame-columns` | change | frame-columns | `PatchFrame` (columns field) |

## genericVariantsRemoved
`AddFrame`, `PatchFrame` (frame patch), `Pages(CollectionMutation::…)`,
`Stories(CollectionMutation::…)`, `Links(CollectionMutation::…)`, `SetDataFields` — all were **app
funnel debt only** (the enum itself never had them; `🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs`
and the artifact-root app `🦀️component.rs` still constructed them, which is why layout's `cargo
check` had 13 baseline errors). Fixed by rewriting the 4 command handlers (`add_frame`, `add_page`,
`patch_page`, `patch_frame`) to build the correct semantic variant directly, replacing the old
`page_patch_for_field`/`frame_bounds_patch` partial-patch builders with `page_field_mutation` (reads
the target `Page`'s current margins/columns so `update-page-margins`/`update-page-columns` stay
atomic) and per-field `MoveFrame`/`ResizeFrame`/`ChangeFrameFill`/… construction (reads the
target `Frame`'s current `bounds()` so `move-frame`/`resize-frame` fill in the untouched axis).
Also caught and fixed a real bug introduced mid-session: `text_to_rgba` returns
`Option<[f32; 4]>` already — the first pass double-wrapped it in `Some(...)`, which `cargo check`
caught (E0308) and was corrected to pass the `Option` straight through.

`LayoutConfigMutation::Snapshot { config }` (whole-config, app-level ratchet scope, not the
107-facet census but explicitly in this ticket's step-3 remit) — removed; `diff()`/`inverse()`
rewritten per-field, one match arm per variant re-emitting itself with the old value read from
`base`.

## emoji table (facet-scoped uniqueness)
25 active triads, each emoji unique within the facet (verified via a Python duplicate scan before
and after). The pre-existing duplicate was `➕` (`add-frame` vs `create-frame`) and `➖`/`📖`/`🔗`/`🧾`
(each shared with a same-named retired legacy dir) — all resolved by deleting the 7 orphan legacy
dirs (see below), which needed no renames. The one **real** duplicate among 25 *active* variants
was `🗑️` used 3× (`delete-page`/`delete-story`/`delete-link`) — fixed by reassigning:
`delete-page` keeps `🗑️`, `delete-story` → `📕`, `delete-link` → `✂️`. Full 25-emoji table is the
first column of the `mutationsCreated` table above; no duplicates remain.

## Directory + glue trueing
- Deleted 7 orphan legacy triad dirs (doc-comment-only stubs, confirmed dead — each had a
  9–10-line `🦠️mutation` file and 1-line stub `🔺️diff`/`↩️inverse` files, explicitly marked
  "Retired" in their own header comments): `➕add-frame`, `➖remove-frame`, `📄pages`, `📖stories`,
  `🔗links`, `🧾set-data-fields`, `🩹patch-frame`.
- Removed their 7 glue.rs mount blocks (`add_frame`, `remove_frame`, `pages`, `stories`, `links`,
  `set_data_fields`, `patch_frame`).
- Added 25 real glue.rs mounts (one `pub mod <snake_slug> { pub mod mutation; pub mod diff; pub
  mod inverse; }` block per active triad), mirroring the pattern already working in
  `🔌️plugins/🎬️sequence`'s glue.rs.
- Removed the dispatch file's `//#region 🔖️LeafWiring` inline `#[path = "."]` self-wiring (25
  blocks) — replaced with `use super::{ change_data_fields, change_frame_columns, … };` reaching
  the now-glue-mounted siblings.
- Deleted the dead `apply_layout_mutation` free function and its `glue.rs` `pub mod op { … }` shim
  re-export — grep confirmed zero external callers before removal; `pub mod op` now re-exports
  `LayoutMutation` directly.
- Dispatch-variant-set now matches triad-dir-set 1:1 in both directions (25 = 25).

## TS mirrors
Added 75 non-stub `🟦️component.ts` files (25 triads × `🦠️mutation`/`🔺️diff`/`↩️inverse`) — none
existed before. Each `mutation/🟦️component.ts` exports a real `interface <PayloadName> { camelCase
fields: primitive TS types }` plus a `Kind` const mirroring `SemanticDescriptor.kind`; each
`diff`/`inverse/🟦️component.ts` exports a `declare function` signature importing the sibling
payload type. Domain object fields (`Page`, `Frame`, `TextStory`, `ImageLink`) are mirrored as
`unknown` (no existing importable TS domain model to bind to — the facet's `📸️snapshot/🟦️component.ts`
was found to have unrelated stale content, `JsonSnapshot` instead of `LayoutSnapshot`, likely a
copy-paste leftover from scaffolding; flagged below, not touched, out of this ticket's scope).

## Schema description files
Rewrote all 5 (previously stale, literally reading `"schema" SP "stdio.json"` — copy-paste from an
unrelated template, matching zero of the 25 real mutations):
`🧬️mutations/📝️text/📖️component.grammar.semio`, `🧬️mutations/💾️binary/📡️component.protocol.semio`,
`🧬️mutations/📝️text/🔗️component.graphql`, `🧬️mutations/📝️text/🔣️component.json`,
`🧬️mutations/📝️text/🛰️component.proto`. Grammar: one alternative per slug in `line = a / b / … `,
argument order address-first then new-value fields (mirrors `🎬️sequence`'s already-real grammar
style). Protocol: `record <VariantPascal> tag N` for all 25, N in dispatch-enum order (1..25).
GraphQL/JSON/proto: one input type / oneOf branch / message per mutation. Not rewritten: the
`.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy` siblings (still stale placeholders) — no working reference
implementation exists anywhere in the repo for these formats yet (checked `🎬️sequence`, the
only other already-migrated facet with a real `.grammar.semio`, and its siblings are equally
stale), and this rule was never gated by policy (`remaining-work-map.md`: "rule 5 grammar
coverage… never implemented"). Flagged as remaining work, not silently skipped.

## lawTests
Extended the existing `⚖️SemanticLaws` region (was 3 tests: `create-page`, `move-frame`,
`rename-layout`) with 4 more `assert_mutation_inverse_law` calls covering the remaining
structurally distinct categories: `delete-page` (cascade capture), `reorder-pages`
(index-addressed-by-id), `update-page-margins` (atomic ≥2-field facet), `change-frame-fill`
(nested-collection variant-specific patch), plus a combined `edit-story`/`create-link` test for
the two remaining id-keyed collections. `DiffAlgebra` is NOT implemented for `LayoutDiff` (missing
before this session, still missing) — `LayoutDiff`'s nested collection-delta fields
(`LayoutPagesDelta`/`LayoutStoriesDelta`/`LayoutLinksDelta`, each with `added`/`removed`/`patched`/
`reordered`) make a correct `between(a, b)` a real per-collection diff algorithm, not a per-field
`Option` fold like the flat structs `assert_diff_algebra_between_law` is normally proven against
elsewhere (e.g. `WavDiff`) — flagged as remaining work rather than risking an incorrect
implementation unverified by a passing test run.

## gates
- `cargo check -p semio-s-plugin-layout`: baseline (before any edit) had 13 errors — 10 in
  `🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs` + `🎛️apps/📏️layout/🦀️component.rs` (the
  funnel debt above) and 3 unrelated (`E0432`/`E0609`/`E0560` — stdio's `PdfSnapshot` schema
  shape drifted out from under layout's PDF serializer, `stdio` PDF import/export files this
  ticket never touches; recorded as `blocked-churn`, matches Wave-R's r2b report verbatim).
  After the funnel/directory/glue fixes: repeated `cargo check` runs this session showed **0
  errors originating in layout's own files** every time; the run was twice blocked by an unrelated
  `semio-framework-os-kernel` compile break (`E0063`/`E0308`/`E0753` in
  `🏪️store/🦀️component.rs`/`🌿️vcs`/`os_dsl`, ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`,
  confirmed mid-edit by another live session — the exact broken line moved between retries) and
  the 3 stdio PDF errors persisted throughout. **Not re-confirmed clean end-to-end in a single run
  free of framework churn before this report was written** — retried 3×, spaced by substantial
  intervening work, per the churn-retry policy; recorded as `blocked-churn` for the framework
  window, not a layout defect.
- `cargo test -p semio-s-plugin-layout --lib`: first attempt hit the same framework churn window.
  Second attempt got past it and surfaced a REAL layout bug this session introduced: the
  `#[cfg(test)] mod tests` region's pre-existing `semantic_kinds_cover_every_variant` test (which
  calls `LayoutMutation::kinds()` and `.semantics()`, both from the `protocol::SemanticMutation`
  trait) was never actually exercised by plain `cargo check` (which does not compile `cfg(test)`
  code by default) — so a missing `use protocol::SemanticMutation;` in the test module's imports
  had been silently broken since before this session, invisible until `cargo test` finally ran.
  Fixed: added `SemanticMutation` to the test module's `use protocol::{Mutation, MutationDiff,
  …};` line. Third attempt: **down to the 3 known stdio `PdfSnapshot` errors only** (0 layout-
  originated errors) — `cargo test --lib` cannot fully compile layout's test binary until stdio's
  PDF schema drift is fixed by whichever session owns it; not this ticket's to fix. No test count
  observed since the binary never finishes compiling, but layout's own code is now provably
  correct through `cfg(test)` as well as plain `cargo check`.
- `bun ./📜️script.ts policy`: ran once, repo-wide (`22158` high-priority breaches total — expected
  for a live, multi-session, 107-facet migration in progress). Zero
  `mutation-migration/semantic-vocabulary`, `…/dispatch-coverage`, or `…/ts-mirror` breaches
  reference `📏️layout`. The only hits mentioning this facet are `…/triad-completeness` and
  `…/artifact-engine`, both pre-documented in `📓️remaining-work-map.md` as "bogus, wrong-depth bug"
  and unrelated to this session's changes.

## allowlistKeysToRemove
Repo-relative paths now free of `SetSnapshot`/`NoMutation`/`CollectionMutation` (raw content
including comments, verified via `grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)"`
returning zero hits across the whole `✏️s/🔌️plugins/📏️layout` tree):
```
✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs
```
(This was the only file with a live banned-token hit at facet-scope in layout; the artifact-root
`📏️layout/🦀️component.rs` the brief flagged as "known to still reference banned tokens" was
already clean when checked — likely fixed by an earlier wave.)

## filesTouched
**Updated:**
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🔗️component.graphql`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🔣️component.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🛰️component.proto`

**Created:** 75 `🟦️component.ts` files, one beside each of the 25 triads' `🦠️mutation`/`🔺️diff`/
`↩️inverse` `🦀️component.rs` (paths follow
`…/🧬️mutations/<emoji><slug>/{🦠️mutation,🔺️diff,↩️inverse}/🟦️component.ts`).

**Removed (7 directories, 4 files each = 28 files):**
`…/🧬️mutations/➕add-frame/`, `…/➖remove-frame/`, `…/📄pages/`, `…/📖stories/`, `…/🔗links/`,
`…/🧾set-data-fields/`, `…/🩹patch-frame/` (each held `🦠️mutation/{🦀️component.rs,🟦️component.ts}`,
`🔺️diff/🦀️component.rs`, `↩️inverse/🦀️component.rs`).

**Renamed (directory, 4 files each moved):**
`…/🗑️delete-story/` → `…/📕delete-story/`, `…/🗑️delete-link/` → `…/✂️delete-link/`.

## sharedFileRequests
None — layout's `📦️glue.rs` was in-scope for this Wave-C lane and edited directly.

## deviations
- Did not implement `DiffAlgebra` for `LayoutDiff` (see lawTests above) — flagged, not silently
  dropped.
- Did not rewrite `.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy` grammar/protocol siblings — no working
  reference exists repo-wide; flagged, not silently dropped.
- Did not touch `📸️snapshot/🟦️component.ts`'s pre-existing wrong content (`JsonSnapshot` instead
  of `LayoutSnapshot`) — outside this ticket's mutation-vocabulary scope, but worth a follow-up
  ticket; noted here rather than fixed silently or left unmentioned.
- Gate 1 (`cargo check`) could not be certified clean end-to-end in a single run free of unrelated
  concurrent framework churn within this session's time budget — reported honestly as
  `blocked-churn` per the ticket's own retry policy rather than claimed as a pass.
