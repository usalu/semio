# Wave-M — `🪵️sourcing` / `🗂️curate` mutation facet migration

## facet

`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`
Crate: `semio-s-plugin-sourcing`. Schema key: `sourcing.curate`.

## status

**partial** — all facet code is authored and internally consistent on disk; per the coordinator's
mid-run instruction, `cargo check`/`cargo test` were NOT run by this lane (build-lock contention
across ~10 concurrent lanes), so compilation is unverified by me and deferred to the coordinator's
consolidated pass. A separate, pre-existing FOREIGN break in this plugin's `📦️glue.rs` (see
`blocked-churn` below) will fail the crate's build regardless of this lane's work until the owning
session lands its file.

## mutationsCreated

Derived per `📓️derivation-rules.md` rule 2 from `CurateSnapshot.curated: Vec<CuratedItem>`
(`CuratedItem { object_id: String (#[dsl(refs = "object")]), count: u32 }`).

| slug (triad dir) | verb | variant | payload | superseded old variant |
|---|---|---|---|---|
| `🌱create-curated-item` | `create` | `CreateCuratedItem(CreateCuratedItem)` | `{ item: CuratedItem }` (`#[dsl(block)]`) | `SetSnapshot` |
| `🗑️delete-curated-item` | `delete` | `DeleteCuratedItem(DeleteCuratedItem)` | `{ object_id: String }` | `SetSnapshot` |
| `🔢change-curated-item-count` | `change` | `ChangeCuratedItemCount(ChangeCuratedItemCount)` | `{ object_id: String, new_count: u32 }` | `SetSnapshot` |

Semantic descriptors (all verbs in `APPROVED_VERBS`; `kind` == kebab of variant name == triad-dir
stem with emoji stripped, as the derive's `const _: () = assert!(...)` enforces):

- `{ verb: "create", entity: "curated-item", kind: "create-curated-item", record: "CreatedCuratedItem" }`
- `{ verb: "delete", entity: "curated-item", kind: "delete-curated-item", record: "DeletedCuratedItem" }`
- `{ verb: "change", entity: "curated-item", kind: "change-curated-item-count", record: "ChangedCuratedItemCount" }`

Diff/inverse are real and handcrafted, never apply-then-capture, never snapshot clones:

- `create` diff → `CurateDiff { curated: Some(CurateCuratedDelta { added: vec![item] }) }`;
  inverse → `delete-curated-item(item.object_id)` (payload carries the id, no BASE lookup needed).
- `delete` diff → `removed: vec![object_id]`, early-returns `CurateDiff::default()` when the target
  is absent from `base`; inverse reconstructs the FULL removed `CuratedItem` from `base` as a
  `create-curated-item`, `Vec::new()` on missing target. No cascade exists to sever — nothing in
  `CurateSnapshot` references `curated` entries.
- `change` diff → `patched: vec![CurateCuratedPatchEntry { object_id, count: Some(new_count) }]`,
  early-returns default when absent; inverse reads the OLD `count` off `base` (never inverts the
  diff structurally), `Vec::new()` on missing target.

## (a) curated vs stock scoping — REQUESTED EXPLICITLY

**Only `curated` got real vocabulary. `stock` deliberately got NONE.** This was investigated, not
assumed; the coordinator's hypothesis is **confirmed**.

Evidence from grepping every `stock`-touching call site in the plugin:

- The ONLY writer of `stock` in the whole plugin is
  `🎛️apps/🗂️curate/🎮️commands/📄️artifact/🦀️component.rs::stock_from_catalogue`, which bulk-merges
  every not-yet-present kind from `engine::available_modules()` in one gesture. There is no
  command, panel, window or engine helper anywhere that creates, deletes, renames, or edits a single
  `ObjectKind` — no per-field editor, no add-custom-kind affordance.
- `stock` is otherwise seeded wholesale: `engine::default_document()`/`empty_document()` parse it
  from the bundled `.curate` fixtures, and `engine::sync_sourcing_module_contributions()` refreshes
  it from host-pushed `"sourcing.module"` topic contributions (hot-installed modules), i.e. it is a
  reference catalogue owned by the module system, not user-authored content.
- Every other `stock` reader is read-only: `filtered_stock`, `curated_count`, `curate_set`/
  `curate_delta` (availability clamping), `sourcing_catalog_fragment`, `kind_mesh_json`, the pool/
  grid/preview windows.

Therefore, per rule 6 and the `ArtifactStore::reset` rule, whole-catalogue population goes through
the non-history reset path, NOT the mutation enum. No `create-object-kind` / `delete-object-kind` /
`rename-object-kind` / `change-object-kind-*` was minted; consequently rule 2's `Vec`-field clause
(`typology_path` → `add-/remove-object-kind-typology`) and large-structured-field clause
(`geometry: Box<GeometryRecipe>` → `replace-object-kind-geometry`) were also **not** triggered,
since they only apply to a collection that is itself user-editable. If a future ticket adds a real
"author a custom object kind" gesture, that is when `stock` earns its own vocabulary.

`CuratedItem` likewise gets no `rename-` (its only identity field IS the address `object_id`, and a
"rename" would be a different object entirely — that is `delete` + `create`), no `reorder-` (the
curation set has no user-meaningful display order; both windows sort by their own table sort), and
no `delete-curated-items` plural (sourcing has single-select only — see `world_select`'s "keeps
only the LAST id" comment).

## genericVariantsRemoved

- `SourcingMutation::SetSnapshot { snapshot: CurateSnapshot }` — the entire pre-migration enum. No
  replacement mutation, per `📓️taxonomy.md`.
- The hand-written `impl protocol::Mutation<CurateSnapshot> for SourcingMutation` (apply/diff/
  inverse match dispatch) — now generated by `#[derive(dsl::Mutations)]`.
- Free functions `apply_sourcing_mutation` / `inverse_sourcing_mutation` — deleted; the only
  callers were the deleted `📸️set-snapshot/🦠️mutation` leaf and a re-export in
  `🧬️mutations/📝️text/🦀️component.rs`. `ArtifactBuilder::mutate` in `🧬️schema/🦀️component.rs`
  already used the correct `<Diff as protocol::MutationDiff<Snapshot>>::apply(...)` form and needed
  no change.
- `whole_document_operation` override on `SourcingCurateApp` — removed; the app now stays at the
  trait's `None` default and overrides `import_media` instead.
- Triad dir `📸️set-snapshot/` (mutation + diff + inverse + 3 `.ts` mirrors) — deleted outright.

Whole-document replace was re-routed to the sanctioned non-history path, modelled on fem3d's
`reset_document_effect`: a new `crate::apps::curate::reset_document_effect(&CurateSnapshot) ->
HostEffect::LoadDocument { pack, spr }`, used by `import_media("document:in")`,
`set_artifact_json`, `set_active_example` and `stock_from_catalogue`.

To keep the curation commands emitting REAL targeted mutations (rather than mutating a document
clone), the engine grew a `CurationDecision` enum (`NoOp | Create | ChangeCount | Delete`) plus
`curation_decision_for_delta` / `curation_decision_for_set`, resolved against the live base
snapshot. The pre-existing mutating helpers `curate_delta`/`curate_set` now fold through the same
decision so engine-level fixtures and tests keep their exact old behavior. A no-op adjustment
(e.g. removing an already-uncurated object) now emits **nothing** — there is no `NoMutation`
sentinel to fall back on.

## filesTouched

**created (18)** — 3 triads × (`🦠️mutation` + `🔺️diff` + `↩️inverse`) × (`🦀️component.rs` +
`🟦️component.ts`), all under
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`:

```
🌱create-curated-item/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}
🗑️delete-curated-item/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}
🔢change-curated-item-count/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}
```

**updated (14)**

```
✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️component.grammar.semio
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️component.graphql
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️component.json
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️component.proto
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎮️commands/🧺️curation/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎮️commands/📄️artifact/🦀️component.rs
```

**removed (6)** — the whole `📸️set-snapshot/` triad dir:

```
…/🧬️mutations/📸️set-snapshot/🦠️mutation/{🦀️component.rs,🟦️component.ts}
…/🧬️mutations/📸️set-snapshot/🔺️diff/{🦀️component.rs,🟦️component.ts}
…/🧬️mutations/📸️set-snapshot/↩️inverse/{🦀️component.rs,🟦️component.ts}
```

Description files were rewritten honestly to the real 3-rule/3-record vocabulary (grammar keywords
= slugs without emoji, args address-first then `new-*`; GraphQL/JSON-Schema/proto now describe a
tagged union of the three payloads instead of the old snapshot-shaped record). The facet-level
`🟦️component.ts` now exports real `CreateCuratedItem`/`DeleteCuratedItem`/`ChangeCuratedItemCount`/
`SourcingMutation` types instead of `export {};`; the 9 per-triad-leaf `.ts` mirrors are short
stubs matching the existing repo-wide convention (policy flags those as non-blocking `"low"`).
The `📝️text`/`💾️binary` subdir description files were left alone (generic stdio envelope format,
out of scope).

## gates

**NOT RUN BY THIS LANE — deferred to the coordinator's consolidated pass**, per the coordinator's
mid-run instruction to stop cargo work because ~10 lanes were contending for one shared cargo build
lock.

- `cargo check -p semio-s-plugin-sourcing` — **not completed, no clean result observed.** One
  attempt was started before the stop instruction; it did not reach a verdict on this lane's code
  because it aborted on the foreign `📦️glue.rs` mount error below, and a retry attempt sat in
  `Blocking waiting for file lock on package cache` / `on build directory`. The single error
  observed was:
  ```
  error: couldn't read `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/./././../../🎛️apps/🗂️curate/🎮️commands/📄️document/🦀️component.rs`: No such file or directory (os error 2)
     --> ✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs:367:13
      |
  367 |             pub mod document;
      |             ^^^^^^^^^^^^^^^^^
  error: could not compile `semio-s-plugin-sourcing` (lib) due to 1 previous error
  ```
  This is the pre-flagged foreign break, not attributable to any file this lane touched. Because
  the module tree never resolved, the compiler never type-checked this lane's own code — **I make
  no claim that my code compiles.**
- `cargo test -p semio-s-plugin-sourcing --lib` — **not run.** The crate does not currently build
  for the foreign reason above.
- `bun ./📜️script.ts policy` — **not run** (same stop instruction).

What WAS verified without the toolchain:

- `grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/🪵️sourcing --include="*.rs" --include="*.ts"` → **zero hits** (exit 1), comments and doc-comments included; every prose mention was reworded to "the former whole-snapshot-replace variant".
- `grep -rn "apply_sourcing_mutation\|inverse_sourcing_mutation" ✏️s/🔌️plugins/🪵️sourcing` → zero code hits.
- Brace-balance check over all 32 created/updated Rust files → all balanced.
- Trait/derive contracts cross-checked against source rather than assumed: `MutationKind<P, Op>`
  (`📡️spr/🎮️command/🦀️component.rs:298`) — `SEMANTICS: SemanticDescriptor`, `diff(&self, &P) ->
  Op::Diff`, `inverse(&self, &P) -> Vec<Op>`, `label`, defaulted `target`; `SemanticDescriptor`
  field order `verb, entity, kind, record`, all `&'static str` (line 284); the `Mutations` derive
  (`🗣️dsl/✨️derive/🦀️component.rs:977-1094`) asserts `kind == kebab(variant)` and
  `is_approved_verb(verb)` and requires exactly one unnamed tuple field per variant; testkit
  signatures `assert_mutation_inverse_law(&P, &Op)` (line 523) and
  `assert_mutation_diff_absorb_law(&P, D, D)` by value (line 507). All match what was authored.

## lawTests

Written, **not executed** (crate does not build for the foreign reason above). All extend existing
`#[cfg(test)]` regions — no new test files.

In `🧬️mutations/🦀️component.rs` (region `🧪️MutationLaws`), using
`protocol::os_spr::testkit`, one test per mutation kind (3 kinds × both laws = 6 law assertions):

| test | laws | kind |
|---|---|---|
| `create_curated_item_satisfies_the_inverse_and_absorb_laws` | `assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law` | `create-curated-item` |
| `delete_curated_item_satisfies_the_inverse_and_absorb_laws` | both | `delete-curated-item` |
| `change_curated_item_count_satisfies_the_inverse_and_absorb_laws` | both | `change-curated-item-count` |

Plus the din16798-style pair over the closed set: `every_variant_registers_an_approved_semantic_descriptor`
(asserts `is_approved_verb` per variant and `kinds().len() == every_mutation().len()`) and
`every_variant_round_trips_via_inverse` (apply-then-inverse restores `base` for all 3). The
`sample_snapshot()` fixture pre-curates `beam-glulam-gl24h` so `delete`/`change` have real targets
and leaves `beam-kvh-c24` uncurated so `create` has a genuinely-absent target.

In `🧬️mutations/📝️text/🦀️component.rs`: 3 per-kind `assert_op_line_round_trip` tests plus
`every_variant_op_text_round_trips` over the closed set. In `🧬️mutations/💾️binary/🦀️component.rs`:
`op_binary_round_trips_and_agrees_with_text` (via `assert_op_text_binary_equivalence`) and the
store text/pack round trip, both re-pointed off the deleted variant onto `create_curated_item`.
In `🎮️commands/📄️artifact`: 4 command tests re-expressed against `HostEffect::LoadDocument` pack
bytes instead of `app.snapshot()`. In `🎮️commands/🧺️curation`: the 3 pre-existing tests kept
as-is plus a new `curate_remove_on_an_uncurated_object_emits_no_mutation` pinning the
no-sentinel-needed behavior.

`DiffAlgebra` for `CurateDiff` was NOT implemented (absent before this lane, and
`assert_diff_algebra_*_law` were not in scope for this facet's assignment).

## allowlistKeysToRemove

All five existing `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` entries for this plugin
(`📜️script.ts:5869-5873`) can be removed — four are now free of the banned tokens in raw content
including comments, and the fifth no longer exists on disk. **`📜️script.ts` was not edited by this
lane.**

```
✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎮️commands/📄️artifact/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎮️commands/🧺️curation/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦠️mutation/🦀️component.rs   ← file DELETED, key is now dangling
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
```

## sharedFileRequests

None outside this plugin. Nothing in `🧰️framework/**`, `🛢️db`, or `📜️script.ts` was touched.

One in-plugin note for whoever owns the `📄️document` → `📄️artifact` rename (see below): once that
session lands, `📦️glue.rs:366-367`'s `#[path]` and the mounted module name must agree with whatever
the final directory name is. This lane owns `📦️glue.rs` and edited ONLY the `mutations` block
inside it; the `commands` block was left exactly as found.

## (b) blocked-churn — FOREIGN, NOT TOUCHED — REQUESTED EXPLICITLY

**Confirmed: I did NOT create, fix, move, or otherwise modify
`🎛️apps/🗂️curate/🎮️commands/📄️document/🦀️component.rs`. It belongs to another session.**

State as found and left: `📦️glue.rs:366-367` mounts

```rust
#[path = "../../🎛️apps/🗂️curate/🎮️commands/📄️document/🦀️component.rs"]
pub mod document;
```

but no `📄️document/` directory exists — the on-disk directory is `📄️artifact/` (mtime Aug 12
15:00, i.e. another session's in-flight rename), and the `commands` block has no mount for it.
This is the exact break the fan-out brief pre-flagged, and it fails `cargo check` for the whole
crate on its own, independent of this lane's work. Verbatim error is quoted in `gates` above.

Consequence for this lane, stated plainly: the file whose CONTENT corresponds to `commands::document`
is `📄️artifact/🦀️component.rs`, and that is the file this lane edited (its three whole-document
commands now emit `reset_document_effect`). Those edits are content-correct and consistent with
`🎛️apps/🗂️curate/🦀️component.rs`'s `use crate::apps::curate::commands::document::{...}` import,
but they cannot be compiled until the owning session reconciles the directory name with the glue
mount. I deliberately did not "fix" this by renaming either side, per the brief's do-not-fix rule.

The second pre-flagged foreign break — `UndoGroup::member_edits` missing at construction sites in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — appears to have been resolved by
another session during this run: both construction sites (lines 5415 and 5476) now pass
`member_edits: Vec::new()`, and that error did not appear in the observed `cargo check` output.
No action taken; framework files were never touched by this lane.

## deviations

1. **`stock` intentionally has no vocabulary** — full reasoning in section (a) above. This is the
   one substantive scoping judgement call in this facet and it was made from call-site evidence,
   not from the hypothesis alone.
2. **`whole_document_operation` removed rather than repointed.** The trait default is `None` and
   its only consumer is the default `import_media`, which returns `MediaError::NotImplemented` when
   it is `None`. Rather than lose `document:in` import entirely, `import_media` is overridden to
   decode the pack and emit `HostEffect::LoadDocument` — matching fem3d's post-migration shape
   (`🏗️fem/🎛️apps/🧊️3d/🦀️component.rs:189-198`), which was used as the reference.
3. **`stock_from_catalogue` routed to `reset_document_effect`, not to per-item mutations.** It is a
   bulk catalogue population over a non-user-authored collection; minting `create-object-kind` just
   to express it would contradict decision (1) and rule 6's "whole-document load is not a mutation".
4. **New `CurationDecision` enum in `⚙️engine`.** Needed because the curate commands previously
   computed a mutated document clone and shipped it wholesale; with that path gone they must decide
   *which* semantic mutation applies (create vs change vs delete vs nothing) against the live base,
   including the availability clamp. Putting it in the engine keeps one source of truth for both
   the command handlers and the pre-existing `curate_delta`/`curate_set` helpers (2+ consumers,
   which is exactly that file's stated rule for what lives there).
5. **`diff_set_snapshot` / `CurateDiff.artifact` retained.** They are diff-internal whole-artifact
   replacement machinery used by `apply_to_artifact`, not a mutation payload, so they are outside
   the forbidden-vocabulary rule (which bans option-bag/whole-object shapes *as a mutation's own
   payload*). Its unit test was re-expressed to call the function directly rather than through a
   now-deleted mutation variant.
6. **Per-triad-leaf `.ts` mirrors are stubs.** The fan-out brief asks for non-stub mirrors, but the
   task brief explicitly permits short stubs at the leaves (matching repo-wide convention, policy
   priority `"low"`); the facet-level `🟦️component.ts` is a real, non-stub export.
7. **Gates unrun.** Recorded honestly per the coordinator's instruction; see `gates`. No pass is
   claimed anywhere in this report.
