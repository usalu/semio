# Wave 2 fan-out — shooting/shooting (standards/1/subsets/any) mutations facet report

Facet: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-shooting`

## Source shape (before)

`ShootingSnapshot`: three id-keyed collections (`assets: Vec<ShootingAsset>` — id/name/url/format/
origin/orientation/scale; `shots: Vec<ShootingShot>` — id/label/width/height/format/shape/background/
camera_id; `saved_cameras: Vec<ShootingSavedCamera>` — id/label/camera), one document-root facet
(`scene: ShootingSceneLighting` — sun/ambient/shadow/material, patchable per-field via
`ShootingScenePatch`'s 7 optional fields), and two document-root selection pointers
(`active_shot_id`, `active_asset_id`, both plain non-`Option<>` `String`, empty = none).

The old `ShootingMutation` enum had 11 variants: three generic `CollectionMutation<String, T, TPatch>`
wraps (`Assets`, `Shots`, `SavedCameras`), `SetActiveShot`/`SetActiveAsset` (struct-variant setters,
already taxonomy-shaped), `SetShotCamera` (indirect: patches the *saved camera* a shot references),
`PatchScene` (a raw `ShootingScenePatch` option-bag used directly as the mutation payload — the
"forbidden option-bag Patch struct" pattern), `TranslateAssets`/`RotateAssets`/`ScaleAssets` (bulk
spatial transforms over `Vec<String> asset_ids`, already correctly plural), and the banned
`SetSnapshot { snapshot }`. 11 triad-leaf directories already existed as **empty scaffolding stubs**
(3-line `//#region.../ //#endregion` placeholders, or wrapper-struct `*Diff { diff: ShootingDiff }`
stubs never referenced by the dispatch enum) — none held a real `MutationKind` payload; all apply/
diff/inverse logic lived hand-inlined in the dispatch `🦀️component.rs` itself.

## Constraint that shaped this fan-out: `📦️glue.rs` is immutable and already fully wired

`📦️glue.rs` (plugin package root, outside this facet's boundary, explicitly on the DO-NOT-TOUCH list)
`#[path]`-wires **exactly** the 11 pre-migration triad directories above — no more, no fewer — each as
`pub mod <name> { pub mod mutation; pub mod diff; pub mod inverse; }`. Unlike some sibling facets'
wave-2 passes (e.g. `writer/writer`), I could not create brand-new triad directories: an unwired new
directory produces unresolved-module errors, and editing `glue.rs` is forbidden. So every semantic
mutation kind derived below lives inside **one of the 11 already-wired directories**, grouped by the
collection/facet it belongs to (`📦assets` hosts `CreateAsset`/`DeleteAsset`/`RenameAsset`/
`ChangeAssetUrl`/`ReorderAssets`; `📸shots` hosts all 8 shot-collection kinds; `🎥saved-cameras` hosts
all 5 saved-camera-collection kinds; `☀️patch-scene` hosts all 7 scene-field kinds) — never invented
structure, always the taxonomy verb, just multiple payload structs/diff-fns/inverse-fns per file
instead of one. This is a deliberate, documented deviation from the "one triad dir ⇒ one verb"
convention, called out in a doc-comment at the top of `🧬️mutations/🦀️component.rs` and tracked below
as a `sharedFileRequests` item (rename directories + re-wire `glue.rs`, once that file can be touched).

## Derivation applied (per `derivation-rules.md`)

Derived from the snapshot's own patchable-field shape and cross-checked against every real
`🎛️apps/🎥️shooting/🎮️commands/*` call site (so no invented vocabulary: e.g. `ShootingAssetPatch` has
no `format` field, so no `change-asset-format` was minted; the app's `☀️scene` commands set each sun/
ambient/shadow/material field independently, never as one bundled facet, so `scene` got 7 separate
`change-scene-<field>` kinds instead of one bundled `update-scene-sun`).

| Old | New (verb — kind) |
|---|---|
| `Assets(CollectionMutation::Add)` | `create` — `create-asset` |
| `Assets(CollectionMutation::Remove)` | `delete` — `delete-asset` |
| `Assets(CollectionMutation::Patch{name})` | `rename` — `rename-asset` |
| `Assets(CollectionMutation::Patch{url})` | `change` — `change-asset-url` |
| `Assets(CollectionMutation::Move)` | `reorder` — `reorder-assets` |
| `Shots(CollectionMutation::Add/Remove)` | `create`/`delete` — `create-shot`/`delete-shot` |
| `Shots(CollectionMutation::Patch{label/width/height/format/shape})` | `rename`/`change`×4 — `rename-shot`, `change-shot-width`, `change-shot-height`, `change-shot-format`, `change-shot-shape` |
| `Shots(CollectionMutation::Move)` | `reorder` — `reorder-shots` |
| `SavedCameras(CollectionMutation::Add/Remove)` | `create`/`delete` — `create-saved-camera`/`delete-saved-camera` |
| `SavedCameras(CollectionMutation::Patch{label})` | `rename` — `rename-saved-camera` |
| `SavedCameras(CollectionMutation::Patch{camera})` | `replace` (whole-value overwrite, not a merge) — `replace-saved-camera-view` |
| `SavedCameras(CollectionMutation::Move)` | `reorder` — `reorder-saved-cameras` |
| `SetShotCamera` | `replace` (indirect target via `shot_id`) — `replace-shot-camera` |
| `SetActiveShot`/`SetActiveAsset` | unchanged shape, `set` — `set-active-shot`/`set-active-asset` (taxonomy: `set` stays approved for narrow addressed single-field setters) |
| `PatchScene{patch: ShootingScenePatch}` | `change`×6 + kept sun as 4 separate — `change-scene-sun-enabled`, `change-scene-sun-azimuth`, `change-scene-sun-elevation`, `change-scene-sun-intensity`, `change-scene-ambient-intensity`, `change-scene-shadow-enabled`, `change-scene-material-roughness` |
| `TranslateAssets` | `drag` (relative offset, taxonomy's exact verb for this gesture) — `drag-assets` |
| `RotateAssets`/`ScaleAssets` | unchanged shape/math, `rotate`/`scale` — `rotate-assets`/`scale-assets` |
| `SetSnapshot` | **deleted, no replacement** (banned; `ArtifactStore::reset` is the sanctioned whole-doc path, outside `Mutation`) |

31 mutations total, all closed-taxonomy verbs (`create`, `delete`, `rename`, `change`, `reorder`,
`drag`, `rotate`, `scale`, `replace`, `set`).

## Triad leaves (real `MutationKind` payloads, not stubs)

Every `<dir>/🦠️mutation/🦀️component.rs` now holds one or more payload structs (`Clone, Debug,
PartialEq, Serialize, Deserialize`) each implementing `protocol::MutationKind<ShootingSnapshot,
ShootingMutation>` with a real `SEMANTICS: SemanticDescriptor` const and `diff`/`inverse` that
delegate to a same-named function in the sibling `🔺️diff`/`↩️inverse` leaf (never inline logic in the
`🦠️mutation` leaf itself, matching the mandated shape). `🔺️diff` functions build `ShootingDiff`
sparsely and directly from the payload (never apply-then-capture) — id-keyed collection kinds
construct `ShootingAssetsDelta`/`ShootingShotsDelta`/`ShootingSavedCamerasDelta` literals directly;
`reorder-*` recomputes the id order from `base`; the 7 scene kinds clone `base.scene` and set one
field (the `ShootingDiff::scene` field is whole-struct-when-present, not itself sparse — this is
schema-shape, not a design choice). `↩️inverse` functions reconstruct the undo mutation from `base`
only: id-keyed `delete-*`/`rename-*`/`change-*-*`/`reorder-*` look up the target by id and return
`Vec::new()` when absent (replacing `NoMutation`); `create-*`'s inverse is unconditionally
`delete-*`; the two selection setters and the 7 scene setters are always-applicable (single required
document-root fields, no missing-target case); `replace-shot-camera`'s inverse walks
`shot → camera_id → saved_cameras` from `base` and is `Vec::new()` when either link is absent
(preserves the pre-migration "no saved camera referenced ⇒ no-op" behavior exactly).

`drag-assets`/`rotate-assets`/`scale-assets` keep the exact pre-migration math (`quat_mul`/
`quat_from_axis_angle`/`shooting_asset_scale`, all still exported from the plugin-root
`🦀️component.rs`, untouched) — only the mutation *shape* changed (bulk `Vec<String> asset_ids` payload
struct instead of a struct-variant), not the transform semantics or their inverses (negated offset/
angle, reciprocal scale-factor).

`📄set-snapshot`'s three leaf files are reduced to a one-paragraph doc comment explaining the
retirement (no payload, no `SetSnapshot`/`ScaleSnapshot`/etc. variant remains anywhere) — kept only
because `glue.rs` still `#[path]`-wires the three file paths and I cannot delete the directory without
touching that file.

## Dispatch rewrite (`🧬️mutations/🦀️component.rs`)

`ShootingMutation` is now 31 single-field tuple variants, `#[derive(..., dsl::Mutations)]` with
`#[mutations(snapshot = ShootingSnapshot, diff = ShootingDiff, schema = "shooting.shooting")]`
(**note**: the ticket's worked example spells this `dsl_derive::Mutations`, which is the derive
crate's own internal name; from a *consuming* plugin crate the re-exported form already used
throughout this file for `DslRecord`/`DslEnum` is `dsl::Mutations` — confirmed by grep against
`os_dsl`'s `pub use dsl_derive::{.., Mutations};` and this crate's own `extern crate
semio_framework_os_kernel as dsl;` alias). All hand-written `apply_shooting_mutation`/
`inverse_shooting_mutation`/`impl Mutation<ShootingSnapshot> for ShootingMutation` deleted — the
derive generates `impl Mutation`/`impl SemanticMutation` now; nothing outside this file called the
deleted free functions (verified: `grep -rn "apply_shooting_mutation|inverse_shooting_mutation"` the
whole plugin directory before deleting — zero hits outside this file).

## OpText/OpBinary simplified, not hand-rolled DSL-mirrored

The pre-migration `📝️text/🦀️component.rs` carried a large `ShootingMutationDsl` mirror enum + two
newtype-node wrappers (`ShootingAssetNode`/`ShootingShotNode`/`ShootingSavedCameraNode`) + a
`ShootingSnapshotDsl` — all existing *solely* to route around the orphan rule for the old
`CollectionMutation<..>`-wrapped variants (documented in the file's own top comment) and to give
`SetSnapshot` a `#[dsl(block)]` payload. Neither problem exists anymore: every variant now wraps a
plain local struct. Replaced the whole mirror with a direct `impl OpText`/`impl OpBinary for
ShootingMutation` over `serde_json`'s compact encoding (`serde_json.workspace = true` was already a
direct dependency — no new Cargo dependency added), which satisfies both traits' laws exactly:
`print_op` is single-line by construction (JSON escapes embedded control chars, never emits a literal
`\n`), `parse_op(op.print_op()) == op` (serde round-trip), `encode_op` is deterministic (struct field
order is declaration order). Kept `COMPONENT_GRAMMAR_SEMIO`/`COMPONENT_GRAMMAR_PATH` (`include_str!`
of the pre-existing `.grammar.semio` file, untouched — recipe step (f) is non-blocking and I ran out
of scope to also rewrite the handcrafted grammar/protocol `.semio` files honestly; noted below).

`💾️binary/🦀️component.rs`'s two tests updated to the new tuple-variant construction
(`SetActiveShot(set_active_shot::mutation::SetActiveShot{..})`, `CreateAsset(assets::mutation::
CreateAsset{..})` replacing the removed `CollectionMutation::Add`).

## Schema-level `🔺️diff/📝️text/🦀️component.rs` cleanup (sibling of `🧬️mutations`, same artifact
directory, in-boundary)

Removed `assets_delta_from_collection_mutation`/`shots_delta_from_collection_mutation`/
`saved_cameras_delta_from_collection_mutation`/`diff_set_snapshot` — dead code once nothing in
`🧬️mutations` builds a `CollectionMutation` or constructs a whole-artifact-replace diff anymore (every
triad leaf now builds its delta literal directly, per the mandated shape). Removed the now-unused
`CollectionMutation`/`ShootingAssetPatch`/`ShootingShotPatch`/`ShootingSavedCameraPatch` imports that
only those four functions needed. `apply_assets_delta`/`apply_shots_delta`/
`apply_saved_cameras_delta`/`ShootingDiff::apply`/`::absorb`/`::apply_to_artifact` (the actual
apply-side machinery, still exercised by every triad leaf's diff output) are untouched.

## Tests

Extended `🧬️mutations/🦀️component.rs`'s existing `#[cfg(test)] mod tests` (no new test files):

- Per-collection create/rename/change/delete + reorder round-trip tests for assets, shots, saved
  cameras (`round_trip` helper unchanged, still uses `vcs::apply_mutation` + `operation.inverse`).
- `delete_asset_of_a_missing_id_has_an_empty_inverse` — the `Vec::new()`/missing-target law, pinned
  directly (not just via the generic law helper).
- `drag_rotate_scale_assets_round_trip`, `replace_shot_camera_is_a_no_op_when_shot_has_no_saved_camera`
  / `..._patches_the_saved_camera_it_references`, `set_active_shot_and_asset_round_trip`,
  `scene_field_mutations_round_trip` (all 7 scene kinds) — carried over from the pre-migration test
  bodies, adapted to the new tuple-variant construction.
- `shooting_op_text_round_trips_every_variant` — `assert_op_line_round_trip` against all 31 variants.
- `⚖️SemanticLaws` region: `protocol::os_spr::testkit::assert_mutation_inverse_law`/
  `assert_mutation_diff_absorb_law` (added by the Wave 0 mechanism pass, reachable at
  `protocol::os_spr::testkit::*` — confirmed via `semio-framework-os-kernel`'s own package `glue.rs`,
  `pub mod os_spr { .. pub mod testkit; }`, already a direct Cargo dependency of this crate under 4
  aliases; no new dependency) against the three most structurally distinct new kinds:
  `create-asset` (inverse law + diff-absorb law composed with a follow-up `rename-asset`),
  `drag-assets` (bulk relative-offset inverse law), `set-active-shot` (document-root setter inverse
  law).

## Verify

`cargo check -p semio-s-plugin-shooting`: **zero errors and zero warnings anywhere inside
`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting`** (my full package boundary — checked with `grep`,
not just this facet's subdirectory). All 106 real errors (37 remain after fixing the `dsl_derive` →
`dsl::Mutations` path name, see below) originate exclusively from `🎛️apps/🎥️shooting/**` — every
single error's own `-->` location line (not just a `:::` cross-reference back to my dispatch enum)
points into one of `🦀️component.rs` (root), `🎮️commands/🧭️gumball`, `🎮️commands/☀️scene`,
`🎮️commands/📷️shot`, `🎮️commands/🗃️fixture`, `🎮️commands/📦️asset`, `🎮️commands/🎥️camera` — verified by
`awk` extracting the line immediately after every `^error` and confirming all 37 are `🎛️apps` paths,
zero are `🗿️artifacts`. This is the expected, cataloged fallout from deleting the generic vocabulary
(`Assets`/`Shots`/`SavedCameras`/`PatchScene`/`TranslateAssets`/`SetShotCamera`/`SetSnapshot` no
longer exist as those shapes) — every call site and its exact replacement is listed below.

I hit one real mechanism-level issue while getting here: the ticket's worked example literally writes
`#[derive(.., dsl_derive::Mutations)]`, but `dsl_derive` (the proc-macro crate itself) is not a
dependency of `semio-s-plugin-shooting`'s `Cargo.toml` — only `semio-framework-os-kernel` is,
aliased 4× (`dsl`/`protocol`/`store`/`vcs`). `dsl_derive::Mutations` failed with `cannot find module or
crate dsl_derive`. Fixed by using `dsl::Mutations` instead (the re-exported form — `os_dsl`'s own
component file does `pub use dsl_derive::{.., Mutations};`, and this crate's `dsl` alias points at
`os_kernel`'s crate root, which globs `os_dsl::*` up to its own root) — matching the exact pattern
this file already used for `dsl::DslRecord`/`dsl::DslEnum` elsewhere in the plugin. Once fixed, the
derive's generated compile-time `assert!`s (`SEMANTICS.kind == kebab(variant)`, `SEMANTICS.verb ∈
APPROVED_VERBS`) all passed silently for all 31 variants — no assertion-failure errors appeared in
the log for any of them.

**What I could not verify**: `cargo test` requires the whole crate (a single `rlib`/`cdylib` target
covering both `🗿️artifacts` and `🎛️apps`) to compile first; with the 37 expected `🎛️apps` errors still
present (by this task's own design — I was told not to fix them), no test binary can be produced, so
none of the tests above (including the new law-based ones) have actually been *executed*. I traced
every diff/inverse function by hand against the pre-migration logic they were extracted from (the
math and patch-application shapes are copied verbatim, just re-homed per mutation instead of inlined
in one big match), and `cargo check`'s full type-check (which does expand and type-check the
`serde`/`dsl::Mutations` derives, not just check trait bounds) gives good but not complete confidence.
Flagging honestly rather than claiming green tests I didn't run.

## Shared-file reconciliation needed (NOT edited — outside my artifact directory)

### `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs`

Cosmetic-only follow-up, not required for correctness: rename the 11 `#[path]`-wired directories to
match their now-plural contents 1:1 per mutation kind (e.g. split `📦assets`'s block into 5 directories
`✏️rename-asset`, `🌱create-asset`, `🗑delete-asset`, `🔗change-asset-url`, `🔀reorder-assets`, update the
`pub mod assets { .. }` block into 5 separate `pub mod <name> { .. }` blocks each pointing at its own
directory) — and correspondingly move the files. Until then the directory-name-vs-`SEMANTICS.kind`
mismatch (e.g. `📦assets/` physically hosting `create-asset`/`delete-asset`/`rename-asset`/
`change-asset-url`/`reorder-assets`) is a known, documented deviation (see top-of-file doc comment in
`🧬️mutations/🦀️component.rs`), not a functional defect — `dsl::Mutations`'s own compile-time assert
only checks `SEMANTICS.kind == kebab(variant name)`, never the directory name.

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🦀️component.rs`

- ~Line 170: `Some(ShootingMutation::SetSnapshot { snapshot })` (a `whole_document_operation`-style
  override) — delete; `SetSnapshot` has no replacement mutation. If this port needs to keep working,
  route it through `ArtifactStore::reset` at whatever layer above `ArtifactApp::handle` owns that
  (wave 0 confirmed `store.reset(..)` is the sanctioned non-history path but did not add new `Emit`
  plumbing to reach it from inside a command handler — that's a mechanism gap, not something this
  fan-out pass can close).

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/🧭️gumball/🦀️component.rs`

- Line 37: `ShootingMutation::TranslateAssets { asset_ids: ids, dx: payload.dx, dy: payload.dy, dz:
  payload.dz }` → `ShootingMutation::DragAssets(crate::artifacts::shooting::mutations::translate_assets::mutation::DragAssets
  { asset_ids: ids, dx: payload.dx, dy: payload.dy, dz: payload.dz })`.
- Line 62: `ShootingMutation::RotateAssets { asset_ids: ids, ax, ay, az, angle }` → same field names,
  tuple-wrap in `crate::artifacts::shooting::mutations::rotate_assets::mutation::RotateAssets { .. }`.
- Line 86: `ShootingMutation::ScaleAssets { asset_ids: ids, sx, sy, sz }` → tuple-wrap in
  `crate::artifacts::shooting::mutations::scale_assets::mutation::ScaleAssets { .. }`.

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/☀️scene/🦀️component.rs`

All 7 handlers build `ShootingMutation::PatchScene { patch: ShootingScenePatch { <one field>: Some(v),
..Default::default() } }` — each becomes a direct tuple-wrapped `Change*` construction from
`crate::artifacts::shooting::mutations::patch_scene::mutation::*` (no `ShootingScenePatch` needed at
all anymore):
- Line 21 (`sun_azimuth`) → `ChangeSceneSunAzimuth { new_azimuth: payload.value }`
- Line 37 (`sun_elevation`) → `ChangeSceneSunElevation { new_elevation: payload.value }`
- Line 53 (`sun_intensity`) → `ChangeSceneSunIntensity { new_intensity: payload.value }`
- Line 69 (`ambient_intensity`) → `ChangeSceneAmbientIntensity { new_intensity: payload.value }`
- Line 85 (`material_roughness`) → `ChangeSceneMaterialRoughness { new_roughness: payload.value }`
- Line 101 (`shadow_enabled`) → `ChangeSceneShadowEnabled { new_enabled: payload.value }`
- Line 117 (`sun_enabled`) → `ChangeSceneSunEnabled { new_enabled: payload.value }`

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/📷️shot/🦀️component.rs`

- Line 40: `ShootingMutation::SetActiveShot { shot_id: Some(id.into()) }` → tuple-wrap in
  `crate::artifacts::shooting::mutations::set_active_shot::mutation::SetActiveShot { .. }`.
- Lines 59/78/97/118: `ShootingMutation::Shots(CollectionMutation::Patch { id, patch:
  ShootingShotPatch { label/format/shape/generic-field: Some(v), .. } })` → the matching
  `RenameShot{id, new_label}` / `ChangeShotFormat{id, new_format}` / `ChangeShotShape{id, new_shape}`
  from `crate::artifacts::shooting::mutations::shots::mutation::*` (line 118's `shot_patch_for_field`
  dispatch becomes a small match over `payload.field` returning the right `ShootingMutation` variant
  instead of a `ShootingShotPatch`; `width`/`height` map to `ChangeShotWidth`/`ChangeShotHeight`).
- Line 142: `ShootingMutation::Shots(CollectionMutation::Add { index, item: shot })` →
  `ShootingMutation::CreateShot(shots::mutation::CreateShot { shot, index: Some(index) })`.

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/🗃️fixture/🦀️component.rs`

- Lines 22, 50, 66: all three build a full replacement `ShootingSnapshot` and emit
  `ShootingMutation::SetSnapshot { snapshot }`. No semantic replacement exists (whole-document
  replace has no in-history mutation by design). Two honest options for the reconciliation pass:
  (a) decompose into the actual field-level mutations that differ from the current snapshot
  (mechanical for the two fixed-target cases at lines 50/66 — `load_document_text`/`default_snapshot`
  — diff every collection/scalar field, emit `create-*`/`delete-*`/`change-*` only for what changed);
  (b) route through `ArtifactStore::reset` once a reset-capable `Emit` variant exists (same mechanism
  gap noted above for the app root file). Line 22 (`SetSnapshotJson`, a raw dev/debug JSON setter)
  is the strongest candidate for (b) — it accepts arbitrary untrusted JSON, which is exactly
  `reset`'s designed use case (file-open/import/dev-fixture-load), not a real editing gesture.

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/📦️asset/🦀️component.rs`

- Line 33: `ShootingMutation::SetActiveAsset { asset_id: Some(id.into()) }` → tuple-wrap in
  `crate::artifacts::shooting::mutations::set_active_asset::mutation::SetActiveAsset { .. }`.
- Line 54: `ShootingMutation::Assets(CollectionMutation::Patch { id, patch })` (`asset_patch_for_field`
  dispatch over `"name"`/`"url"`) → match returning `RenameAsset{id, new_name}` (for `"name"`) or
  `ChangeAssetUrl{id, new_url}` (for `"url"`) from `crate::artifacts::shooting::mutations::assets::mutation::*`.
- Lines 77, 102: `ShootingMutation::Assets(CollectionMutation::Add { index, item: asset })` →
  `ShootingMutation::CreateAsset(assets::mutation::CreateAsset { asset, index: Some(index) })`.

### `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/🎥️camera/🦀️component.rs`

- Line 30: `ShootingMutation::SetShotCamera { shot_id, camera }` → `ShootingMutation::ReplaceShotCamera(
  crate::artifacts::shooting::mutations::set_shot_camera::mutation::ReplaceShotCamera { shot_id, new_camera: camera })`.
- Line 51: `ShootingMutation::SavedCameras(CollectionMutation::Add { index, item: saved_camera })` →
  `ShootingMutation::CreateSavedCamera(saved_cameras::mutation::CreateSavedCamera { saved_camera, index: Some(index) })`.

(`🎮️commands/🗂️selection`, `🎮️commands/🖨️export`, `🎮️commands/🗣️locale`, `🌉️wasm/🦀️component.rs` only
reference the `ShootingMutation` type generically — as a generic-param/return-type, never construct a
variant — so none of those need edits.)

## Skipped / non-blocking (recipe step f)

Did not touch `📖️component.grammar.semio` / `📡️component.protocol.semio` / the sibling `.json`/
`.proto`/`.graphql`/`.g4`/`.abnf`/`.ksy`/`.spicy`/`.ts` schema-description files in `🧬️mutations/` —
they already describe the old vocabulary generically and updating them honestly for 31 real mutation
kinds (vs. the old 11) is a substantial independent pass I did not have time for in this facet slot;
explicitly non-blocking per the recipe.

## Files touched

Rewritten (real `MutationKind` payloads + diff/inverse, replacing 3-line stubs or wrapper-struct
stubs) — all 11 pre-existing triad directories, `🦠️mutation`/`🔺️diff`/`↩️inverse` in each:
- `🧬️mutations/📦assets/**` (5 kinds: `CreateAsset`, `DeleteAsset`, `RenameAsset`, `ChangeAssetUrl`, `ReorderAssets`)
- `🧬️mutations/↔️translate-assets/**` (`DragAssets`)
- `🧬️mutations/🔄rotate-assets/**` (`RotateAssets`)
- `🧬️mutations/↕️scale-assets/**` (`ScaleAssets`)
- `🧬️mutations/📸shots/**` (8 kinds: `CreateShot`, `DeleteShot`, `RenameShot`, `ChangeShotWidth`, `ChangeShotHeight`, `ChangeShotFormat`, `ChangeShotShape`, `ReorderShots`)
- `🧬️mutations/📷set-shot-camera/**` (`ReplaceShotCamera`)
- `🧬️mutations/🎥saved-cameras/**` (5 kinds: `CreateSavedCamera`, `DeleteSavedCamera`, `RenameSavedCamera`, `ReplaceSavedCameraView`, `ReorderSavedCameras`)
- `🧬️mutations/🎯set-active-shot/**` (`SetActiveShot`)
- `🧬️mutations/📌set-active-asset/**` (`SetActiveAsset`)
- `🧬️mutations/☀️patch-scene/**` (7 kinds: `ChangeSceneSunEnabled`, `ChangeSceneSunAzimuth`, `ChangeSceneSunElevation`, `ChangeSceneSunIntensity`, `ChangeSceneAmbientIntensity`, `ChangeSceneShadowEnabled`, `ChangeSceneMaterialRoughness`)
- `🧬️mutations/📄set-snapshot/**` (retired to doc-only stubs — no payload)

Modified:
- `🧬️mutations/🦀️component.rs` (dispatch enum rewrite — 31 tuple variants + `dsl::Mutations` — and
  its `#[cfg(test)] mod tests` extended, not replaced with a new file)
- `🧬️mutations/📝️text/🦀️component.rs` (dropped the `CollectionMutation`-flattening DSL mirror;
  direct `serde_json`-backed `OpText`/`OpBinary` impls)
- `🧬️mutations/💾️binary/🦀️component.rs` (2 tests updated to tuple-variant construction)
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` (removed 4 now-dead `CollectionMutation`/`SetSnapshot`
  helper fns + their now-unused imports; apply-side machinery untouched)

Not modified (outside boundary — see "Shared-file reconciliation needed" above):
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/{🧭️gumball,☀️scene,📷️shot,🗃️fixture,📦️asset,🎥️camera}/🦀️component.rs`
