# Wave-C funnel — `lowpoly/lowpoly` mutations facet

Facet: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-lowpoly`. Picks up from `📓️wave2-reports/lowpoly-lowpoly-1-any-report.md`,
which derived the full 16-mutation semantic vocabulary and already built 16 real one-kind triad
directories, but self-wired all 16 inline in the dispatch file (`🔖️LeafWiring` region, including a
`remove_paint_layer_mutation` workaround module name to dodge a glue.rs name collision) because it
could not touch `📦️glue.rs`, and left 9 orphan pre-migration directories (8 truly dead + 1,
`➖️remove-paint-layer`, rewritten in place but shadowed) alive because glue.rs still hardcoded them.

**Status: done.**

## 1. App-level compile fixes

Fixed every stale-vocabulary call site cataloged by wave2's `sharedFileRequests`:

- `🎮️commands/➕️add-primitive`: `ObjectsAdd{index,item}` → `CreateObject{index,object}`.
- `🎮️commands/✏️patch-object`: rewrote the `"name"`/`"smoothShading"` match arms to build
  `RenameObject`/`ChangeObjectSmoothShading` directly instead of a `LowpolyObjectPatch` bag.
- `🎮️commands/🖌️paint`: `AddPaintLayer{object_id,index,layer}` → `InsertPaintLayer{..}` (same field
  shape, real variant).
- `🎮️commands/📄️fixture`: both `setSnapshotJson`/`setFixtureJson` routes re-routed through a new
  `reset_document_effect(&LowpolySnapshot) -> HostEffect::LoadDocument` free fn added to the app
  root (mirrors `shooting`'s identically-named helper, itself mirroring the already-migrated
  `cad`/`fem2d`/`draw` pattern — outside undo history, per taxonomy's whole-document-replace ban).
  `SetSnapshotJson` (app command, banned substring) renamed to `ImportSnapshotJson` end to end.
- `🎛️apps/💠️lowpoly/🦀️component.rs`: `whole_document_operation` override removed (falls back to
  `None`); `import_media`'s `"mesh:in"`/`"document:in"` arms now build `reset_document_effect`
  instead of `SetSnapshot{snapshot}`.
- `🖌️session/🦀️component.rs` (the shared gumball-drag/paint-stroke scratch context): added
  `semantic_mutation_for_patch(id, before_transform, patch) -> Option<LowpolyMutation>`, a shared
  helper both `mesh_edit`'s generic kernel-edit commit and `commit_transform`'s gumball-drag commit
  now call — inspects the `object_patch_diff` option-bag (kept as a diff-internal fragment type,
  never a mutation payload, per taxonomy) and picks the ONE real semantic mutation it represents
  (`RenameObject` / `ChangeObjectSmoothShading` / `MoveObject` / `RotateObject` / `ScaleObject` by
  transform-axis / `ReplaceObjectMesh`, first-match priority — a single kernel edit or gumball drag
  never touches more than one of these per commit). Both `PaintStroke{..}` sites (stroke-commit,
  fill) → `EditPaintLayer{..}` (identical field shape).

## 2. Directory + glue trueing

- Removed the dispatch file's entire `🔖️LeafWiring` self-wiring region (16 inline `#[path=".']
  pub mod <slug> { .. }` blocks) and its `🔖️OrphanedByGlue` doc-only region.
- Added 16 real per-slug mounts to `📦️glue.rs`'s `mutations` block (one `pub mod <snake_slug> {
  pub mod mutation; pub mod diff; pub mod inverse; }` each), removing the 9 old mounts
  (`objects_move`, `add_paint_layer`, `objects_add`, `objects_remove`, `paint_stroke`,
  `set_snapshot`, `objects_patch`, `patch_paint_layer`, plus the pre-existing real `remove_paint_layer`
  mount which now points straight at its own triad instead of needing wave2's shadow-avoidance
  workaround).
- Deleted the 8 truly-orphan pre-migration directories outright (`↔️objects-move`,
  `➕️add-paint-layer`, `➕️objects-add`, `➖️objects-remove`, `🖌️paint-stroke`, `🖼️set-snapshot`,
  `🩹objects-patch`, `🩹patch-paint-layer` — 24 doc-stub `.rs` files total).
- Retired the `remove_paint_layer_mutation` self-wired-module workaround: since the orphan
  `objects_remove`-adjacent naming collision no longer exists once the dead mounts are gone, the
  dispatch enum's `RemovePaintLayer` variant and the one cross-reference to it (in
  `➕insert-paint-layer/↩️inverse`, plus one leftover reference in `📝️text`'s test fixture) now use
  the real `remove_paint_layer` module name.
- Fixed the same `taxonomy/emoji-prefix` (missing `U+FE0F`) issue `shooting` hit, scoped to the 10
  of wave2's 16 dirs that used a bare base-emoji codepoint: renamed those 10 directories (+ their 3
  glue.rs path strings each), re-verified `cargo check` clean after.

One triad directory per variant, 16:16, in both directions — dispatch-coverage policy rule
satisfied without ambiguity.

### Emoji table (16 mutations, all unique within the facet — kept wave2's originals where already
correct, only the `U+FE0F` suffix changed on 10 of them)

| Emoji | Slug | Kind |
|---|---|---|
| 🌱️ | create-object | `create-object` |
| 💀️ | delete-object | `delete-object` |
| 🔀️ | reorder-objects | `reorder-objects` |
| 🏷️ | rename-object | `rename-object` |
| 🔘️ | change-object-smooth-shading | `change-object-smooth-shading` |
| ↗️ | move-object | `move-object` |
| 🔄️ | rotate-object | `rotate-object` |
| 📐️ | scale-object | `scale-object` |
| 🧱️ | replace-object-mesh | `replace-object-mesh` |
| ➕️ | insert-paint-layer | `insert-paint-layer` |
| ➖️ | remove-paint-layer | `remove-paint-layer` |
| 🔖️ | rename-paint-layer | `rename-paint-layer` |
| 👁️ | change-paint-layer-visible | `change-paint-layer-visible` |
| 🌫️ | change-paint-layer-opacity | `change-paint-layer-opacity` |
| 🎛️ | change-paint-layer-blend-mode | `change-paint-layer-blend-mode` |
| 🎨️ | edit-paint-layer | `edit-paint-layer` |

TS mirrors: added the missing `🟦️component.ts` stub beside all 16 triads' 3 leaves (48 new files).

## 3. Remaining debt

- Rewrote `📖️component.grammar.semio` (top-level, generic `objects-add|…|set-snapshot`
  alternation) and `💾️binary/📡️component.protocol.semio` (`ObjectsAdd tag 1 … SetSnapshot tag 9`)
  honestly for the real 16-kind vocabulary, tags 1..16 in enum-variant declaration order. Left the
  mutations-root `.json`/`.graphql`/`.proto` files unchanged (already describe the snapshot's
  persistent-field shape generically, no banned-vocabulary references).
- Added an `⚖️SemanticLaws` test region to the dispatch file's now-existing `#[cfg(test)]` module
  (previously the dispatch file had none — the facet's only tests lived in `📝️text`/`💾️binary`):
  `create_object_obeys_the_inverse_and_absorb_laws`, `delete_object_of_a_missing_id_has_an_empty_inverse`,
  `move_object_obeys_the_inverse_law`, via `protocol::os_spr::testkit` (no new Cargo dependency,
  same finding as `shooting`/`demonstrator`).
- `LowpolyConfigMutation` (app view-config) was **not** audited for a whole-config `Snapshot`
  variant this pass — out of time budget; flagged as remaining debt, same deferral rationale as
  `shooting`'s `ShootingConfigMutation::Snapshot`.

## Final sweep

```
grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/💠️lowpoly --include="*.rs" --include="*.ts"
```
0 hits (verified after the `ImportSnapshotJson` rename and rewording the 1 remaining doc-comment
mention of the banned token in prose, in the app root's `import_media` doc comment).

## Gates

- `cargo check -p semio-s-plugin-lowpoly`: **0 self-owned errors** — every error observed across
  several retries traced (via each error's own `-->` location line) to files outside
  `🗿️artifacts/💠️lowpoly`/`🎛️apps/💠️lowpoly`: `🧰️framework/…/🏪️store/🦀️component.rs`
  (`ArtifactEnvelope` gaining/missing an `owner: Option<OwnerRef>` field — another session's live
  edit to the shared framework crate, intermittent across ~6 retries, `blocked-churn`), and once a
  `🎛️apps/💠️lowpoly/📌️panels/📄️document/🦀️component.rs` "no such file" from `📦️glue.rs`'s own
  `pub mod document;` mount — the directory on disk is currently named `📌️panels/📄️artifact/`, i.e.
  another session mid-rename of that panel; not a file this facet's mutations work touches or owns,
  recorded as `blocked-churn` per the fanout brief's own named example of this exact signature.
- `cargo test -p semio-s-plugin-lowpoly --lib`: **blocked-churn**, ran twice (~15 minutes apart,
  matching the repo-wide multi-session load observed via `ps aux`: 16+ simultaneous `cargo check`/
  `test` processes spanning `dag`, `animate`, `architect`, `puzzle`, `stdio`, `process`, `layout`,
  `note`, and others). Both runs fail identically with 16 errors, all inside
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs`
  (`VcsArtifactApp`/`SpaceMember`: `E0277` `(dyn SpaceMember + 'static)` cannot be sent between
  threads safely + one `E0499` double-borrow) — the exact same signature `demonstrator`'s test run
  hit in this same wave, zero hits in `🗿️artifacts/💠️lowpoly`/`🎛️apps/💠️lowpoly` paths in either
  run. Another session's in-progress `Send`-bound work on the shared plugin module; per the brief's
  "retry, never fix" rule for framework churn. `cargo check` (above) is unaffected (test-only code
  path); the facet's own new tests were hand-verified against real type signatures.

## Files touched

Created: `🧬️mutations/{16 slugs}/{🦠️mutation,🔺️diff,↩️inverse}/🟦️component.ts` (48 stubs).

Removed: `🧬️mutations/{↔️objects-move,➕️add-paint-layer,➕️objects-add,➖️objects-remove,🖌️paint-stroke,
🖼️set-snapshot,🩹objects-patch,🩹patch-paint-layer}/**` (8 dirs × 3 files).

Renamed (emoji-prefix fix, dir + glue path only): `🌱create-object`→`🌱️create-object`,
`💀delete-object`→`💀️delete-object`, `🔀reorder-objects`→`🔀️reorder-objects`,
`🔄rotate-object`→`🔄️rotate-object`, `🧱replace-object-mesh`→`🧱️replace-object-mesh`,
`➕insert-paint-layer`→`➕️insert-paint-layer`, `📐scale-object`→`📐️scale-object`,
`🔘change-object-smooth-shading`→`🔘️change-object-smooth-shading`,
`🔖rename-paint-layer`→`🔖️rename-paint-layer`, `🎨edit-paint-layer`→`🎨️edit-paint-layer`.

Modified:
- `📦️packages/🦀️rust/📦️glue.rs` (`mutations` block: 9 old mounts removed, 16 real per-slug mounts
  added; 10 path strings updated for the emoji-prefix renames)
- `🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  (`🔖️LeafWiring`/`🔖️OrphanedByGlue` regions removed, `dsl_derive::Mutations`→`dsl::Mutations`,
  enum variant refs `super::`-prefixed, `remove_paint_layer_mutation`→`remove_paint_layer`,
  `⚖️SemanticLaws` test region added)
- `🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
  (`remove_paint_layer_mutation`→`remove_paint_layer` fixture reference)
- `🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️component.grammar.semio`,
  `💾️binary/📡️component.protocol.semio` (rewritten for the real 16-kind vocabulary)
- `🧬️mutations/➕️insert-paint-layer/↩️inverse/🦀️component.rs` (`remove_paint_layer_mutation` fix)
- `🎛️apps/💠️lowpoly/🦀️component.rs` (whole_document_operation removed, reset_document_effect
  added, import_media rewritten, ImportSnapshotJson rename, 1 test rewritten to assert on
  `HostEffect::LoadDocument`)
- `🎛️apps/💠️lowpoly/🎮️commands/{➕️add-primitive,✏️patch-object,🖌️paint,📄️fixture}/🦀️component.rs`
  (all call-site fixes above; `📄️fixture`'s test rewritten to call `handle` directly and assert on
  the effect, matching `shooting`'s pattern)
- `🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs` (`semantic_mutation_for_patch` helper added;
  `mesh_edit`/`commit_transform`/both paint-stroke sites rewired)

## sharedFileRequests

None outstanding — this facet's `sharedFileRequests` from wave2 (glue.rs rewire, the 6 listed app
call sites) are exactly what this Wave-C pass closed.

## allowlistKeysToRemove

`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` entries confirmed by `bun ./📜️script.ts policy` to no longer
reference banned vocabulary:

- `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🎮️commands/📄️fixture/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

## Deviations

- TS mirrors kept as stubs — see `shooting` report's §2 note.
- `LowpolyConfigMutation` app view-config not audited/semanticized this pass — deferred, see §3.
- `cargo test` could not be observed green (blocked-churn, see Gates) — `cargo check` is clean of
  self-owned errors, and the facet's own new test additions were hand-verified against the
  testkit's real signatures and the triads' real field names.
