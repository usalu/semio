# Wave 2 — norm / din4108 / standard 1 / subset any — fan-out report

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Derivation

Read `📸️snapshot/🦀️component.rs`'s `Din4108Snapshot`: seventeen flat document-root scalars
(hygrothermal/DIN 4108-3 assessment inputs — category, climate zone, airtightness, catalog/material
ids, interior boundary conditions, moisture resistance factors, envelope area, BB2/application
compliance flags) plus one index-keyed, id-less ordered collection, `layers: Vec<LayerDocument>`
(`{thickness_m, lambda_w_mk}`, no stable id — a construction build-up).

None of the seventeen scalars is documented or grouped as an inseparable, jointly-validated facet
(derivation-rules rule 1's `update-<facet>` exception), so each got its own `change-<field>` leaf —
the rule's default. `layers` (rule 3, index-keyed ordered collection) got `insert-layer`/
`remove-layer` (FINAL/BASE-state index per the addressing convention), `reorder-layers`, and
`change-layer-thickness`/`change-layer-lambda` for its two fields, addressed by BASE-state index.

Total: **22 semantic mutations**, replacing the single generic `SetSnapshot` variant.

| Verb | Kind | Record |
|---|---|---|
| change | change-category | ChangedCategory |
| change | change-climate | ChangedClimate |
| change | change-airtightness-n50 | ChangedAirtightnessN50 |
| change | change-psi-times-l-sum | ChangedPsiTimesLSum |
| change | change-rh-int | ChangedRhInt |
| change | change-catalog-id | ChangedCatalogId |
| change | change-material-id | ChangedMaterialId |
| change | change-airtightness-class | ChangedAirtightnessClass |
| change | change-t-int-c | ChangedTIntC |
| change | change-solar-absorptance | ChangedSolarAbsorptance |
| change | change-irradiance-wm2 | ChangedIrradianceWM2 |
| change | change-moisture-mu-exterior | ChangedMoistureMuExterior |
| change | change-moisture-mu-interior | ChangedMoistureMuInterior |
| change | change-envelope-area-m2 | ChangedEnvelopeAreaM2 |
| change | change-bb2-details-conform | ChangedBb2DetailsConform |
| change | change-application-type | ChangedApplicationType |
| change | change-declared-application-class | ChangedDeclaredApplicationClass |
| insert | insert-layer | InsertedLayer |
| remove | remove-layer | RemovedLayer |
| reorder | reorder-layers | ReorderedLayers |
| change | change-layer-thickness | ChangedLayerThickness |
| change | change-layer-lambda | ChangedLayerLambda |

Note: `change-irradiance-wm2`'s kebab form has no hyphen between `w` and `m2` — `#[derive(dsl::Mutations)]`'s
`to_kebab` groups a trailing acronym+digit run (`WM2`) as one word; this is a compile-time-enforced
constraint (`assert!(...)` in the derive), not a style choice, and the triad directory/keyword string
were named to match.

## What changed inside the facet directory

- `🧬️mutations/🦀️component.rs` — rewritten: doc comment, 22 `#[path = "."]` leaf-wiring blocks
  (self-wired here rather than in `📦️glue.rs`, which stays out of scope — same pattern the
  already-migrated `iso16757` sibling facet in this same plugin uses), the
  `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]` dispatch enum
  (`#[mutations(snapshot = Din4108Snapshot, diff = Din4108Diff, schema = "s.norm.din4108")]`), and a
  `🧪️Tests` region (round-trip test for every scalar change, insert/remove-layer round-trip +
  captured-inverse assertion, out-of-range no-op/empty-inverse test, reorder-layers round-trip,
  change-layer-thickness/lambda round-trip + out-of-range test, and a `semantic_kinds_cover_every_variant`
  test asserting `kinds().len() == 22` plus spot-checking `semantics()`/`target()`).
- 22 new triad leaf dirs under `🧬️mutations/`, each `<emoji><verb-kebab-entity>/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
  — payload struct + `impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation>` (delegating to
  sibling `diff`/`inverse` fns), real handcrafted sparse `diff` (never apply-then-capture), real
  handcrafted `inverse` computed from `base` (missing/out-of-range target ⇒ `Vec::new()`).
- `🧬️mutations/📝️text/🦀️component.rs` — rewritten: hand-rolled `impl protocol::OpText for Din4108Mutation`
  and `impl protocol::OpBinary for Din4108Mutation` (the derive only generates `Mutation`/
  `SemanticMutation`, per the derive's own doc comment) — one keyword per verb, every field
  JSON-atom-encoded (all field types here already derive `Serialize`/`Deserialize`) for both the
  text grammar and the binary form; `demo_mutation_cases()` (one value per variant) + a round-trip
  law test, mirroring `iso16757`'s sibling file's structure/rationale.
- `🧬️mutations/💾️binary/🦀️component.rs` — untouched (its `encode_op`/`decode_op` wrappers already
  just forward to `Din4108Mutation`'s `OpBinary` impl, now defined in `📝️text`); its own two tests
  referenced the deleted `SetSnapshot` variant, so they were updated in place to exercise
  `ChangeAirtightnessN50` instead (same law being tested, real variant instead of the banned one).
- `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` — orphaned to a doc-comment-only
  stub (mirrors `iso16757`'s identical orphaning): `📦️glue.rs` (plugin-shared, out of this facet's
  scope) still `#[path]`-wires this leaf into `Din4108Mutation::mutations::set_snapshot`, so the
  files must stay present; `SetSnapshot` itself is deleted from the enum per taxonomy (banned
  outright, no replacement mutation — whole-document replace goes through `store::ArtifactStore::reset`).

No `.ts` mirror files were written for the 22 new leaves — confirmed against `iso16757` (already
migrated in this same plugin), which also left new-leaf `.ts` mirrors unwritten; only pre-existing
stub `.ts` files (`export {};`) remain at the top level and in the orphaned `set-snapshot` leaf.
No grammar/protocol `.semio` spec files were touched (step f, explicitly "not blocking"; `iso16757`
left its equally stale).

## Testkit law helpers (step e)

Grepped: `semio-s-plugin-norm`'s `Cargo.toml` has no dependency on the testkit crate. Per the task's
instructions, skipped rather than adding a new Cargo dependency.

## sharedFileRequests

Not edited (out of this facet's package boundary — `🎛️apps/**` and `📦️glue.rs` are plugin-shared).
A later plugin-wide reconciliation pass needs:

1. **`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`** — delete the `set_snapshot` module block
   inside `din4108`'s `mutations` module (currently `#[path="."] pub mod set_snapshot { pub mod
   mutation; pub mod diff; pub mod inverse; }`, wiring the three now-orphaned stub files) once every
   norm facet has migrated off `SetSnapshot`.
2. **`✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🦀️component.rs:107`** — `import_media`'s `"model:in"`
   branch does `crate::app_surface::import_media(port, media, |snapshot| Din4108Mutation::SetSnapshot
   { snapshot })`; needs a real replacement now that whole-document replace has no mutation-enum
   form (likely: route through `store::ArtifactStore::reset` directly, bypassing `Mutation` entirely,
   per the taxonomy's ruling — a plugin-wide app-surface decision, not per-facet).
3. **`✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🎮️commands/📤️set-snapshot/🦀️component.rs:20`** —
   `commit_snapshot(Din4108Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")`;
   the whole `set-snapshot` app command needs redesigning (batch of semantic `Change*`/layer
   mutations diffed against the current doc, or a non-`Mutation` `store.reset` path) — same
   plugin-wide decision as #2.
4. **`✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🎮️commands/🧮️evaluate/🦀️component.rs:23`** —
   `commit_snapshot(Din4108Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }, "evaluate")`
   re-commits the *unchanged* snapshot (already a no-op today); once `SetSnapshot` is gone this
   should most likely just stop emitting a mutation at all (or emit real `Change*` mutations if
   `evaluate` is ever extended to actually mutate derived fields).

`🎛️apps/📕️din4108/🎮️commands/☑️selected-check/🦀️component.rs` was checked and does **not** need a
request — `commit_selected_check_index::<Din4108Mutation>` only constructs a `NormConfigMutation`
(config-lane), never a `Din4108Mutation`, so it is unaffected by this facet's vocabulary change.

## Verification

`cargo check -p semio-s-plugin-norm`, run 7 times across this session (before/after each fix, then
3 explicit retries per the workspace-churn policy plus one bonus attempt):

- Two real bugs were found and fixed in **my own** files along the way:
  1. `dsl_derive::Mutations` doesn't resolve in a plugin crate (only the framework kernel crate
     depends on `dsl_derive` directly) — fixed to `dsl::Mutations`, the extern-crate alias
     `📦️glue.rs` already declares (`extern crate semio_framework_os_kernel as dsl;`), confirmed
     against the already-compiling `✏️s/🔌️plugins/📐️cad` facet which uses this exact spelling.
  2. `ChangeIrradianceWM2`'s declared `SEMANTICS.kind` (`"change-irradiance-w-m2"`) didn't match the
     derive's compile-time-asserted kebab form of the variant name (`"change-irradiance-wm2"` — the
     derive's `to_kebab` groups a trailing acronym+digit run as one word). Fixed the `kind`/`entity`
     constants, renamed the triad directory, and updated the two OpText keyword strings to match.
- After both fixes, **zero errors originate inside `🗿️artifacts/📕️din4108/`** (excluding
  `🎛️apps/📕️din4108/`) across all four subsequent full-workspace `cargo check` runs. Confirmed by
  grepping every run's error list for `🗿️artifacts/📕️din4108` lines not under `🎛️apps`: none.
- Every remaining error is in a different artifact or plugin entirely, and the *set* of failing
  facets changed between every retry (`Vdi3805` → `+En1994` → `+En1990/1991/1992/1993` →
  `semio-s-plugin-stdio` itself failing to compile with an unrelated `cannot find 'inferences' in
  'schema'` error) — unambiguous concurrent-session churn across the live tree (other wave-2
  sessions migrating `vdi3805`/`en199x`/`din16798` mid-edit, plus at least one unrelated session
  touching `semio-s-plugin-stdio`), not anything caused by this facet's changes. The three expected
  `Din4108Mutation::SetSnapshot` errors (all in `🎛️apps/📕️din4108/**`, listed above as
  `sharedFileRequests`) were present and stable in every run.
- Per the ticket's WORKSPACE CHURN policy (retry up to 3 times, then `blocked-churn` if failures stay
  purely outside the facet directory): retried 3 times after the last in-boundary fix, still
  churning outside `📕️din4108` every time (a 4th bonus retry also churned, now in a different crate
  entirely) — reporting `blocked-churn` / `churn-retry-exhausted` rather than continuing to chase
  other sessions' in-flight work. `cargo test` for this facet's own new law tests could not be run
  (the crate never reached a fully green compile during this session), so `lawTestsPass` is reported
  `false` (not "known failing" — genuinely unexercised); the diff/inverse logic was manually
  re-derived and cross-checked against the schema and the `iso16757` sibling facet's already-proven
  pattern, but has not been runtime-verified.

## Files touched

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (rewritten)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs` (rewritten)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (tests updated)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs` (orphaned)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (orphaned)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs` (orphaned)
- 22 new triad leaf directories under `.../🧬️mutations/`, 66 new `🦀️component.rs` files (listed above by kind/record table)

New/temporary files in this ticket folder: this report only (no scratch files were needed inside
the ticket folder; code-generation helper scripts used during this session live in the session
scratchpad, not the ticket folder, and are not part of the deliverable).
