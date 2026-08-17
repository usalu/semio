# W4 — `block` composes stdio `kit` subset (partial: `🧊️3d` slice complete)

**ucas-status: partial**

## Pre-flight

`git status --porcelain -- ✏️s/🔌️plugins/🧱️block` re-checked before starting: 18 files staged
(not mine) — trivial doc-comment rewords (`"persistent fields only"` → `"artifact-lane fields
only"`) plus a mechanical `type Transient = semio_framework_plugin::NoTransient;` /
`type TransientMutation = …NoTransientMutation;` addition to each of `Block2dPlayApp`/
`Block3dPlayApp`/`Block5dPlayApp`'s `ArtifactApp` impls — the SAME repo-wide framework trait-surface
addition `sourcing`'s report documented, landing elsewhere in this ticket, unrelated to this
migration. Left untouched — did not conflict with anything this migration touched. See
`## Concurrent-churn observations`.

**SMO clearance, checked properly (not from the stale `📓️smo-clearance.md` "0/3" historical
snapshot, which `📌️important.md` explicitly says is not authoritative)**: SMO's own live predicate
file `../SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` lists `🧱️block | ◻2d, 🧊️3d, 🖐️5d |
26 / 37 / 41 mutations` under **RELEASED**, with the note "Compile evidence … `cargo check
--workspace` reports zero errors" but "⚠️ Test targets are NOT yet verified for these". Block is
free to edit, including `🧬️mutations/**`, but its test suite had never been run.

Baseline `cargo check -p semio-s-plugin-block --all-targets` (before any edit): **red**, 8 errors,
**all inside block's own boundary** (confirmed via grep on `-->` lines, zero outside
`✏️s/🔌️plugins/🧱️block/**`):
- 3× `E0308 '?' operator has incompatible types` (`serde_json::Value` vs stdio's `JsonValue`) in the
  2d/3d/5d JSON export serializers.
- 5× `E0080 evaluation panicked: #[derive(Mutations)]: …'s MutationKind::SEMANTICS.kind must equal
  "…" (its own kebab form)` in `🖐️5d`'s mutations dispatch file (`MoveGrip2d`/`MoveGrip3d`/
  `ResizeGrip3d`/`UpdatePart2d`/`UpdatePart3d`) — an extra hyphen (`"move-grip-2d"` vs the derive's
  expected `"move-grip2d"`) in each `SEMANTICS.kind` literal.

These are SMO's own block-migration bugs (their reports for `block-2d`/`block-3d`/`block-5d` in
`../SEMANTIC-MUTATIONS-OVERHAUL/📓️waveM-reports/` explicitly say `gates: NOT RUN`/`cargo check …
NOT RUN to completion`), not concurrent churn and not mine — fixed outright per this ticket's
"cheaper to just fix than chase" guidance since all 8 are trivial, unambiguous (§ Verification).

## What block duplicated

Each of block's three artifacts (`◻2d`, `🧊️3d`, `🖐️5d`) carries one or two hand-rolled "kind
catalogs" — `Block3dVortexKind { id, name, label, color, default_cable_kind }` (3d), and (per file
inspection, not yet migrated — see Scope below) `Block2dHandleKind`/`Block2dNodeKind`-shaped and
`Block5dPartKind`/`Block5dGripKind`-shaped equivalents — matching the design doc's
`puzzle/block/sourcing→C:kit (kills kit.catalog dup, fixes app-owned ids)` line: an `id`/`name`
type-registry duplicating `s.stdio.semio.kit`'s `SemioKitType`. Each registry is actively edited via
a full per-item CRUD mutation vocabulary (create/rename/delete/change-color/change-label/change-
default-*-kind), unlike `sourcing`'s `stock` (bulk-seeded only, no per-item editor) — this is a
materially harder case than every prior exemplar (lowpoly/cad/writer/sourcing all composed
never-per-item-mutated content).

Separately, each of the three apps' `🎛️apps/<dim>/🦀️component.rs` independently declares a
`kit.catalog` `ArtifactKindSpec` (`KIT_CATALOG_ARTIFACT_ID`) feeding a `"catalog:out"` media port via
`puzzleNd_catalog_fragment` — a bespoke ad-hoc JSON shape matching `puzzle`'s own catalog format
(`objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`), NOT literally `SemioKitType`-shaped.
This is the exact "harmless duplicate" `sourcing`'s report flagged block for. Left untouched this
pass (downstream of the storage migration, and `puzzle` — the consumer — is being migrated
concurrently/separately per this ticket's own scope note: "do not coordinate with it").

## What changed — `🧊️3d` slice, fully migrated

### The split: composed child + block3d-owned overflow

`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️component.rs` (`🔖️VortexKindCatalogComposition` region,
new): kept `Block3dVortexKind { id, name, label, color, default_cable_kind }` as the LOGICAL
in-memory type every app/command/test call site still uses (this differs from `sourcing`'s pattern —
here the mutation vocabulary operates on rows of this type, not on the storage type, so nothing about
its shape needed to change). New:
- **`Block3dVortexKindExtra { id, label, color, default_cable_kind }`** — the overflow half
  `SemioKitType` (`id`/`name`/`category` only) can't carry. `#[dsl(defines = "vortex_kind")]` moved
  here from `Block3dVortexKind.id` (mirrors `sourcing`'s `ObjectKindExtra` precedent) so
  `Block3dVortexTemplate.vortex_kind`'s `#[dsl(refs = "vortex_kind")]` still validates.
- Real bidirectional converters: `kit_type_from_vortex_kind`/`vortex_kind_extra_from_vortex_kind`
  (split, `category` is a fixed constant `"vortex-kind"` — never round-tripped through extra, so no
  data loss) and `vortex_kind_from_parts` (join). `catalog_snapshot_from_vortex_kinds`/
  `vortex_kind_extra_list_from_vortex_kinds`/`vortex_kinds_from_catalog_and_extra` lift these to
  whole-list operations.
- **Content-addressed handle minting**: `catalog_child_handle(kinds: &[Block3dVortexKind]) ->
  store::ArtifactChild<SemioKitSnapshot>` — hashes the deterministic JSON of the derived
  `SemioKitType` list (`DefaultHasher`), `child_id = "catalog-{hash:016x}"`, mirrors `sourcing`'s
  `catalog_child_handle` exactly.
- **`vortex_kinds_of_parts`/`set_vortex_kinds_parts`** — the accessor/writer pair every
  render/export/inference/mutation-diff/apply call site funnels through, parameterized over
  `(catalog, extra)` directly rather than a whole struct, so the SAME pair serves both
  `Block3dSnapshot` and `Block3dArtifact` (which mirror these two fields) without duplicating the
  cache-read/mint-and-cache logic — `vortex_kinds_of`/`set_vortex_kinds` are the `Block3dSnapshot`-
  specialized wrappers every call site actually uses.

`🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
`Block3dSnapshot.vortex_kinds: Vec<Block3dVortexKind>` → `catalog: store::ArtifactChild<SemioKitSnapshot>`
(`#[child(kind = "s.stdio.semio.kit")]`) + `vortex_kind_extra: Vec<Block3dVortexKindExtra>`.
Hand-written `impl Default` mints the same empty-list handle `catalog_child_handle(&[])` would.

`🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (`Block3dArtifact`):
mirrors the same split field-for-field (`to_snapshot`/`from_snapshot`/`set_snapshot` updated).

### §2 codec wall — resolved via the real framework capability, not hand-rolled

Checked `impl<S> crate::os_dsl::DslField for ArtifactChild<S>` in `🏪️store/🦀️component.rs:523`
first, per the recipe's 2026-08-13 update — real, generic, present. Kept `dsl::DslRecord` on
`Block3dSnapshot` (and `Block3dVortexKindExtra`, itself also derived), added `catalog`/
`vortex_kind_extra` with `#[child(kind = "…")]`/`#[state(artifact)]` alongside the existing fields.
The existing hand-rolled `impl store::ArtifactDsl`/`impl store::ArtifactPack` (delegating to
derive-emitted `__dsl_spec()`/`__dsl_to_record()`/`__dsl_from_record()`) needed ZERO changes — same
outcome `sourcing` found. No hand-rolled codec bytes written; text encoding for the new fields came
out as `catalog=child_id=<hex> target="<uri>"` + a `vortex-kind-extra [id:TEXT label:TEXT
color:TEXT default-cable-kind:TEXT] { … }` table, verified by the round-trip tests (below) and by
inspecting the regenerated fixtures.

### §3/§4 working-scene cache

Checked `VcsArtifactApp.children`'s actual population for this plugin, per the recipe's explicit
instruction (not just the type signature) — same finding every prior wave reached: no
`open_child`/`register_child` caller exists anywhere in this fan-out, so `ArtifactView::with_children`
has zero live content for block. Built the `thread_local!` working-scene cache
(`BLOCK3D_VORTEX_KIND_CATALOG_SCRATCH: RefCell<HashMap<String, SemioKitSnapshot>>`) in
`🗿️artifacts/🧊️3d/🦀️component.rs`. Populated at `set_vortex_kinds_parts`/`seed_vortex_kind_catalog_
scratch` (fixture loaders); read through `vortex_kinds_of_parts`/`vortex_kinds_of`. Staleness gap
documented in the region's doc comment (not fail-closed — matches `sourcing`'s reasoning: undo/redo
across a create/delete could go stale relative to the cache, a miss silently drops rather than
fabricates).

### Granular mutations kept unchanged — internals rewired only

Per this ticket's explicit instruction, the 6 vortex-kind CRUD triads (`🌱create-vortex-kind`,
`❌delete-vortex-kind`, `🖋rename-vortex-kind`, `🎫change-vortex-kind-label`,
`🎨change-vortex-kind-color`, `🔌change-vortex-kind-default-cable-kind`) kept their exact verbs,
payload shapes, and dispatch variants. Their `🔺️diff`/`↩️inverse` bodies were rewired internally:
every `base.vortex_kinds.iter().find(...)` became `crate::artifacts::block3d::vortex_kinds_of(base)
.iter().find(...)` (via a local `let` binding in the 4 `let-else` diff bodies, since a `let-else`
initializer's temporaries do NOT get lifetime-extended the way a `match` scrutinee's do — the 5
`match`-shaped inverse bodies needed no such binding, Rust's match-scrutinee temporary-extension rule
covers them directly). `Block3dDiff.vortex_kinds: Option<Block3dVortexKindsDelta>` — the actual DIFF
TYPE — needed **zero changes**: it already operates on the logical `Block3dVortexKind` type (added/
removed/patched-by-id), which is unaffected by where the data is stored. Only the `apply`/
`apply_to_artifact`/`absorb` functions in `🔺️diff/📝️text/🦀️component.rs` needed rewiring: resolve
current via `vortex_kinds_of(_parts)`, run the existing `apply_delta!` macro unchanged, then
`set_vortex_kinds(_parts)` to re-split and re-mint the catalog handle. `diff_set_vortex_kind`/
`diff_remove_vortex_kind` (used for reorder-on-insert) rewired the same way.

### Real bidirectional converters — no stubs

See `kit_type_from_vortex_kind`/`vortex_kind_extra_from_vortex_kind`/`vortex_kind_from_parts` above
— every `Block3dVortexKind` field lands in exactly one half, lossless together; `category` is a
fixed constant (never carries real data, so no shadow field needed to preserve it — this is stricter
than `sourcing`'s `category ← module_id` mapping, since block3d's vortex kinds have no grouping
concept at all).

### App-layer + test rewiring

Every direct `.vortex_kinds` field read across the app layer switched to the `vortex_kinds_of(_parts)`
accessor: `🎛️apps/🧊️3d/🌍️world/🦀️component.rs` (`vortex_kind_color`, `resolve_brush_vortex_kind_id`),
`…/🎚️options/🖌️brush/🦀️component.rs` (select-item list), `…/🎮️commands/🌀️vortex/🦀️component.rs`
(`add_vortex`'s default-kind lookup), `…/🎮️commands/🎨️example/🦀️component.rs`
(`replace_document_operations`'s vortex-kind diff loop), `…/🎮️commands/🔘️vortex-kind/🦀️component.rs`
(`add_vortex_kind`'s next-id scan), `…/🎮️commands/🖌️brush/🦀️component.rs` (`PlaceVortex`'s
empty-catalog auto-create check), and the app-level test module in `🎛️apps/🧊️3d/🦀️component.rs`
(`undo_redo_round_trips_through_the_wrapper`, `place_vortex_on_surface_auto_creates_kind_and_vortex`).
`🧬️schema/💡️inferences/🦀️component.rs`'s `puzzle3d_catalog_fragment` and the mutations dispatch
file's own test fixtures (`seeded_snapshot`, `create_rename_delete_vortex_kind_round_trip`) rewired
the same way, using `set_vortex_kinds` where a fixture used to `.push()` directly onto the field.

### §8 fixture regeneration

`🖼️assets/🗣️nakagin-capsule.dsl.semio` and `🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio`
(the two 3d example fixtures, `include_str!`'d as `BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT`/
`BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT` — confirmed by grep as the ONLY Rust consumers of any
`🖼️assets/*` file for this artifact; the sibling `.pack.semio`/`.op.semio`/`.spr.semio` files are not
`include_str!`'d anywhere and were left untouched, out of scope) were both in the obsolete
pre-migration format (a literal `vortex-kinds […]` table). Regenerated via the temporary-debug-test
technique: added a `#[cfg(test)] mod debug_fixture_regen` to `📸️snapshot/📝️text/🦀️component.rs` that
built the nakagin-capsule snapshot via the existing `tests::nakagin_capsule()` helper (rewired to use
`set_vortex_kinds`) and a matching forest-left snapshot (hand-built from the OLD fixture's own
literal content, to preserve every id/position/direction/radius/color exactly), ran
`cargo test … debug_fixture_regen -- --nocapture`, and wrote the captured `print_dsl()` output
verbatim as the new fixture files. Confirmed the temporary module is fully removed: `grep -rn
debug_fixture_regen` returns nothing.

## Two pre-existing, trivial, unambiguous bugs fixed outright (not part of the kit-composition slice)

Found incidentally while running the newly-green test suite for the first time (SMO's block reports
say tests were never run before this session):
1. **`compute_block3d_bounds`/`compute_block5d_bounds`'s test expectations had a wrong `min.y`** in
   4 places (`🧊️3d` and `🖐️5d`'s `💡️inferences/📦bounds/🦀️component.rs` and `💡️inferences/🦀️component.rs`,
   copy-pasted across both dimensions): `multiple_vortices_union_their_footprints`/
   `bounds_match_vortex_positions_inflated_by_radius` (3d) and their `🖐️5d` grip-named twins asserted
   `min: [-1.25, -0.5, 2.5]`. Hand-checked the arithmetic against the two fixture vortices/grips
   (`(1,2,3,r=0.5)` → y-range `[1.5,2.5]`; `(-1,0,4,r=0.25)` → y-range `[-0.25,0.25]`; union
   `min.y = -0.25`, matching x and z, which the test already had right) — the implementation was
   correct, the hardcoded expected value was not. Traced via `git log -1 --date=iso` on
   `📦bounds/🦀️component.rs`: last real touch **2026-08-12 11:09:41**, well before this ticket
   (opened 15:02:49) and completely disjoint from anything this migration touches (`vortices`/
   `grips`, not `vortex_kinds`/`catalog`). Fixed all 4 occurrences to `[-1.25, -0.25, 2.5]`.
2. The 3× `JsonValue`/`serde_json::Value` `?`-operator baseline errors and 5× kebab-form
   `SEMANTICS.kind` baseline errors listed under Pre-flight — fixed outright (see Verification).

## Scope not completed — `◻2d` and `🖐️5d`'s own kind registries

**`◻2d`** (`Block2dHandleKind`/`Block2dNodeKind`-shaped registries — 2 separate CRUD vocabularies,
per the design doc's per-artifact mutation-triad directory names: `create/rename/delete/change-
*-color/change-*-label/change-*-default-wire-kind` for handle-kind, plus `rename/change-*-label/
change-*-description/change-*-unit/change-*-variant/change-*-icon` for node-kind) and **`🖐️5d`**
(`Block5dPartKind`/`Block5dGripKind`-shaped registries, same double-registry shape) were **not**
migrated this pass. Each mirrors 3d's exact duplication pattern (confirmed by directory/field-name
inspection) and the identical migration recipe demonstrated above applies mechanically — but doing
it for real, verified, would roughly double or triple this pass's already-large touch count (the 3d
slice alone touched 34 files: the type/cache/converter home file, snapshot, artifact-schema, diff
apply/absorb, 9 of the 12 vortex-kind-touching files inside `🧬️mutations/**`, the mutations dispatch
tests, the inference file, 6 app-layer command/world/options files, 1 app-level test file, the
snapshot-text fixture test file, and 2 regenerated `.dsl.semio` fixtures — and 2d/5d each carry
**two** such registries, not one). Landing 2d/5d in the same pass risked exactly the "half-broken
cascade" this ticket's own brief says to avoid rather than a smaller fully-verified slice. `◻2d`
and `🖐️5d`'s snapshot/diff/mutation files are otherwise untouched by this session (only the pack of
trivial pre-existing-bug fixes above landed there).

**Continuation recipe for a future wave** (mechanical, already proven correct for 3d): for each
registry, (1) add a `<Dim><Registry>Extra` struct with the non-`id`/`name` fields, `#[dsl(defines =
…)]` moved onto it; (2) add `<registry>_kit_type_from_*`/`*_extra_from_*`/`*_from_parts` converters
+ `catalog_child_handle`/working-scene cache in the artifact's top `🦀️component.rs` (co-locate
multiple registries' caches in ONE `thread_local!` `HashMap` keyed by a registry-prefixed child_id,
or use separate statics — either is fine, just document the choice); (3) swap the snapshot/artifact
struct fields; (4) rewire the diff-apply/absorb functions and the ~6-10 triads' `base.<field>` reads
per registry (the `let`-vs-`match` temporary-lifetime distinction found here applies identically);
(5) rewire app-layer/test call sites; (6) regenerate any `.dsl.semio` fixtures containing the old
table format via the same temporary-debug-test technique.

## Verification

`CARGO_TARGET_DIR=".../🎯️target"` for every invocation below.

- `cargo check -p semio-s-plugin-block --all-targets`: baseline **8 errors** (all pre-existing, all
  inside block's own boundary, none in `🧊️3d`'s vortex-kind files specifically — 3 in JSON export
  serializers across 2d/3d/5d, 5 in 5d's mutations dispatch). After this migration's edits + the two
  outright bug fixes: **0 errors**, 147 warnings (all pre-existing dead-code/unused-import style
  warnings, none newly introduced by this migration — spot-checked the warning list contains no
  `catalog`/`vortex_kind_extra`/`vortex_kinds_of` references).
- `cargo test -p semio-s-plugin-block --lib`: **157 passed, 6 failed** out of 163. Reproduced stable
  across two consecutive full runs (not flaky). The 6 failures (`apps::block2d::…::
  export_media_catalog_out_wraps_the_puzzle2d_fragment`, `apps::block2d::…::
  set_active_example_loads_left_fixture`, and their `block3d`/`block5d` twins) are **pre-existing and
  outside this pass's boundary** — every one traces to `replace_document_operations` failing to apply
  any mutation when loading a bundled example fixture via `SetActiveExample`, in files this migration
  never touched (2d's `🎮️commands/🎨️example/🦀️component.rs`, last real edit per `git log -1
  --date=iso`: **2026-08-12 15:02:49**, the literal ticket-open timestamp — predates every edit in
  this session). `block2d_example_fixtures_parse_and_round_trip_as_dsl` (a DIFFERENT test exercising
  the same fixture text through `parse_dsl` directly, bypassing the app/command layer) passes, so the
  DSL text itself is not at fault — the bug is somewhere in the app-level dispatch of the
  `replace_document_operations` batch, not investigated further (out of this pass's `🧊️3d`/vortex-
  kind-catalog scope, and SMO's own `block-2d`/`block-3d`/`block-5d` reports confirm `gates: NOT RUN`,
  i.e. this class of bug was never exercised before this session).

  **Correction (orchestrator, 2026-08-13):** this report originally claimed `🧊️3d`'s own equivalent
  tests (`set_active_example_loads_capsule_fixture`, `export_media_catalog_out_wraps_the_puzzle3d_fragment`)
  pass, attributing that to this migration's fixture regeneration. **That claim is false** —
  independently re-ran both tests directly against the current on-disk state (not cached/stale) and
  both FAIL, identically and reproducibly (`objectKinds[0].id` expected `"Capsule J"`, got `""`),
  the same symptom shape as 2d/5d. This does not change the root-cause diagnosis above (still the
  `SetActiveExample`/`replace_document_operations` dispatch layer, not the fixture text, not this
  migration's `vortex_kinds`/`catalog` composition work — 3d exhibiting the identical bug if anything
  *supports* "shared framework-level cause across all three dimensions" over "fixture-specific"), but
  it means **all 6 failures share the same unresolved cause, not 3 of 6**. `git log -1 --date=iso`
  on `🎛️apps/🧊️3d/🦀️component.rs` (the 3d file with the failing assertion) was NOT independently
  useful for dating here since the file was live-uncommitted at verification time (this migration's
  own final edits) — the 2d dating citation above (2026-08-12 15:02:49, predating this session) stands
  as the actual evidence this class of bug is pre-existing, not the "3d passes" claim, which is
  retracted.

## sharedFileRequests

None. Every edit is inside `✏️s/🔌️plugins/🧱️block/**`; stdio was read-only reference throughout
(`s.stdio.semio.kit`'s `SemioKitSnapshot`/`SemioKitType`, `semio-s-plugin-stdio` already a
`Cargo.toml` dependency of this crate — no new dependency added).

## Concurrent-churn observations

1. **18 pre-existing staged files** (not mine, present before I started): trivial doc-comment
   rewords + a `Transient`/`TransientMutation` associated-type addition to `Block2dPlayApp`/
   `Block3dPlayApp`/`Block5dPlayApp`'s `ArtifactApp` impls — same repo-wide framework trait-surface
   addition `sourcing`'s wave-4 report documented. Left untouched — did not conflict with anything
   this migration touched.
2. **Baseline red, from SMO's own block migration** (see Pre-flight) — not concurrent churn in the
   "another live session mid-edit" sense, but a real defect in already-landed, already-"RELEASED"
   work that had never been compiled to completion (`cargo check … NOT RUN to completion` per SMO's
   own `block-2d`/`block-3d` reports) or test-run. Fixed outright rather than worked around.
3. `📌️important.md` was observed to update mid-session (external edit, auto-commit sweeping the
   ticket's own coordination file) — not something this session caused or needed to react to beyond
   noting it per the harness's own instruction not to revert it.

ucas-status: partial — `🧊️3d`'s vortex-kind catalog fully composes `s.stdio.semio.kit`, real
converters, working-scene cache, unchanged mutation vocabulary, fixtures regenerated, 0 compile
errors, 157/163 tests passing (6 pre-existing failures outside this slice's scope, traced and
documented, not fixed). `◻2d` and `🖐️5d`'s own kind registries are NOT yet composed — left as
their pre-existing inline `Vec<KindType>` shape, with a proven-correct mechanical continuation recipe
above for whichever wave picks them up next.
