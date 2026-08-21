# 🎥️ Shooting mutation fixtures — 31/31 handcrafted

Tree: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Wiring: `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs`

`fixtures lint --by-tree` → this tree no longer appears in the uncovered list (was `31/31`, now `0/31`)
and raises no `❌️` finding of its own. Derived-encoding gaps (`.op/.spr/.patch/.dsl/.pack`) are the
expected `⚠️` warnings pending `fixtures generate`.

## 🧱️ The base snapshot

One shared, hand-authored `⬅️before` shape is used by every case (each case carries its own copy;
cases that need a different starting point would diverge, none did). It is the smallest snapshot that
contains every entity family the 31 mutations address, with enough plurality to make ordering,
cascade and cursor claims meaningful:

| field | content |
| --- | --- |
| `schema` | `"shooting.shooting"` |
| `assets` | `asset-hero` (origin `[1,2,3]`, orientation `[0,0,0,1]`, scale `[2,2,2]`), `asset-prop` (origin `[0,0,0]`, orientation `null`, scale `null`) |
| `savedCameras` | `cam-wide` ("Wide"), `cam-close` ("Close") |
| `scene` | background `#101014`; sun `{enabled:true, az 45, el 35, int 2.4, #ffffff}`; ambient `{1.15, #ffffff}`; shadow `{enabled:true, 0.35, 1.0}`; material `{#9aa0ab, metal 0.0, rough 1.0, #000000, 0.0}` |
| `shots` | `shot-wide` (512×512, png, rectangle, background `#ffffff`, `cameraId: cam-wide`), `shot-close` (256×256, svg, ellipse, no background, **no** `cameraId`) |
| `activeShotId` / `activeAssetId` | `shot-wide` / `asset-hero` |
| `emblem` | absent (`skip_serializing_if = "Option::is_none"`, so it is omitted, not `null`) |

Two deliberate asymmetries carry most of the discriminating power:
`asset-prop` has `orientation: null` + `scale: null` (exercises `unwrap_or` defaults and proves the
unaddressed asset is untouched), and `shot-close` has **no** `cameraId` (the only way to reach
`replace-shot-camera`'s no-op branch).

Deletes always target the **trailing** element (`asset-prop`, `shot-close`, `cam-close`) because
`apply_identified_delta` applies `added` with a plain `push` — the inverse `create-*` therefore only
restores the original order when the removed item was last.

### 📐️ Payload serde shape (verified, not assumed)

`ShootingMutation` is `#[serde(tag = "mutation", rename_all = "camelCase")]`, so the variant tag is
camelCase (`createAsset`, `changeSceneSunEnabled`). **None of the 31 payload structs carries any
`#[serde(...)]` attribute** (`grep -rn "serde(" */🦠️mutation/🦀️component.rs` → empty), so their
fields stay snake_case on the wire: `new_name`, `new_url`, `to_index`, `asset_ids`, `shot_id`,
`new_camera`, `saved_camera`, `new_enabled`, `new_roughness`, …
`index: Option<usize>` has no `skip_serializing_if`, so it is always emitted (`null` when absent).

## 🧪️ The 31 cases

| mutation leaf | case name | what the `after` encodes |
| --- | --- | --- |
| `🌱️create-asset` | `appends-asset-detail` | pushed LAST despite the payload's `index: 0` |
| `🗑️delete-asset` | `removes-trailing-asset-prop` | asset gone, shots/cursors untouched |
| `✏️rename-asset` | `renames-asset-hero-to-lead` | only `name` |
| `🔗️change-asset-url` | `points-asset-prop-at-v2-mesh` | only `url` (`format` NOT re-derived) |
| `🔀️reorder-assets` | `moves-asset-hero-behind-asset-prop` | order `[prop, hero]` |
| `↔️drag-assets` | `offsets-both-assets-and-skips-a-ghost` | per-asset relative origins + `mutation.partial` |
| `🔄️rotate-assets` | `spins-asset-hero-about-z` | orientation `[0, 0, sin .75, cos .75]` |
| `↕️scale-assets` | `doubles-asset-hero-scale` | `[2,2,2] × 2 → [4,4,4]` (multiplicative) |
| `📸️create-shot` | `appends-shot-macro` | new shot with no `background`/`cameraId` |
| `🚮️delete-shot` | `removes-trailing-shot-close` | saved cameras NOT garbage-collected |
| `🏷️rename-shot` | `relabels-shot-close-to-detail` | only `label` |
| `📏️change-shot-width` | `widens-shot-close-to-1024` | width only, no aspect coupling |
| `📐️change-shot-height` | `heightens-shot-close-to-768` | height only, no aspect coupling |
| `🖼️change-shot-format` | `switches-shot-wide-to-svg` | only `format` |
| `✂️change-shot-shape` | `rounds-shot-wide-to-ellipse` | only `shape` |
| `🔃️reorder-shots` | `moves-shot-close-to-front` | order `[close, wide]`, cursor stays on `shot-wide` |
| `📷️replace-shot-camera` | `rewrites-cam-wide-through-shot-wide` | addressed by SHOT, writes the referenced saved camera |
| `🎥️create-saved-camera` | `appends-saved-camera-top` | new pose, no shot rebound |
| `🧹️delete-saved-camera` | `removes-trailing-cam-close` | no cascade into `shots.cameraId` |
| `🪪️rename-saved-camera` | `relabels-cam-close-to-tight` | patch `label: Some, camera: None` |
| `🎞️replace-saved-camera-view` | `repositions-cam-close-view` | whole pose replaced, `label: None` |
| `🔁️reorder-saved-cameras` | `moves-cam-close-to-front` | order `[close, wide]`, id binding survives |
| `🎯️set-active-shot` | `activates-shot-close` | root scalar only |
| `📌️set-active-asset` | `activates-asset-prop` | root scalar only |
| `☀️change-scene-sun-enabled` | `switches-scene-sun-off` | sun off, its az/el/int preserved |
| `🧭️change-scene-sun-azimuth` | `turns-scene-sun-to-315-degrees` | stored unwrapped (no ±180 normalization) |
| `🌅️change-scene-sun-elevation` | `raises-scene-sun-to-60-degrees` | within the closed ±90 band |
| `💡️change-scene-sun-intensity` | `dims-scene-sun-to-half` | `2.4 → 1.2`, ambient untouched |
| `🔅️change-scene-ambient-intensity` | `dims-scene-ambient-to-quarter` | `1.15 → 0.25`, sun untouched |
| `🌑️change-scene-shadow-enabled` | `switches-scene-shadows-off` | shadows off, opacity/softness preserved, sun stays on |
| `🪨️change-scene-material-roughness` | `polishes-scene-material-to-quarter` | `1.0 → 0.25`, metalness/albedo untouched |

Every case declares `{"status": "applied"}`; `drag-assets` additionally declares
`{"level":"warn","code":"mutation.partial"}`. No case is `rejected`, so no
`🔺️diff/🚫️component.absent` marker was needed anywhere in this tree.

## 🚦️ Rejection / no-op codes found in this tree

Read off the 31 `🔺️diff/🦀️component.rs` files. Only the four frozen `mutation.*` codes appear.

| code | severity | raised by |
| --- | --- | --- |
| `mutation.duplicate-id` | Fatal | `create-asset`, `create-shot`, `create-saved-camera` — id already present |
| `mutation.invariant` | Fatal | `scale-assets` (non-finite **or** non-positive factor); `rotate-assets` (non-finite axis/angle); `change-shot-width` / `change-shot-height` (zero); `change-scene-sun-azimuth` (non-finite); `change-scene-sun-elevation` (non-finite or outside `-90..=90`); `change-scene-sun-intensity` / `change-scene-ambient-intensity` (non-finite or `< 0`); `change-scene-material-roughness` (non-finite or outside `0..=1`) |
| `mutation.target-missing` | Error | every id-addressed delete/rename/change/reorder/replace; `set-active-shot` / `set-active-asset` when the id is `Some(unknown)`; `drag-assets` / `rotate-assets` / `scale-assets` when **none** of `asset_ids` resolve |
| `mutation.partial` | Warning | `drag-assets`, `rotate-assets`, `scale-assets` — some but not all of `asset_ids` resolve; the diff still applies for the ones that did |
| `mutation.no-op` | Warning | `rename-asset`, `change-asset-url`, `reorder-assets`, `rename-shot`, `change-shot-width`, `change-shot-height`, `change-shot-format`, `change-shot-shape`, `reorder-shots`, `replace-shot-camera` (shot has **no** `cameraId`), `rename-saved-camera`, `replace-saved-camera-view`, `reorder-saved-cameras`, `set-active-shot`, `set-active-asset`, and all seven scene leaves |

Guard **order** matters and is asserted where it differs from the obvious one:
`change-shot-width`/`change-shot-height` check `target-missing` FIRST, then the zero invariant, then
the equality no-op — a zero width on a nonexistent shot reports `target-missing`, not `invariant`.
`create-*` have no no-op guard at all; `drag/rotate/scale` have no no-op guard either (re-applying
accumulates). `set-active-*` accept `null` as the legal "nothing active" value, stored as `""`.

## 🧬️ Test shape

Each `🦀️component.rs` is standalone: four `#[semio_framework_async_macros::async_test] async fn`s in
the de-async style (no `.await`), local `before()/expected_after()/mutation()/apply()` helpers, and
assertions phrased for that one mutation. There is no shared harness, macro or loop.

The shooting glue declares no `vcs` alias (unlike puzzle's), and this artifact exposes no
`apply_shooting_mutation`/`inverse_shooting_mutation` free functions, so the tests drive the traits
directly: `protocol::Mutation::diff` → `MutationOutcome::into_parts().0` →
`protocol::MutationDiff::apply`, and `protocol::Mutation::inverse` for the round trip.

The fourth test in every file is the mutation's own guard probe: it re-applies the committed payload
to the `after` state (or decodes a small inline probe payload) and asserts the exact
`worst_level()` + `messages()[0].code.0` + `target` that this leaf's diff builder produces. That is
what makes the files non-interchangeable.

## ⚠️ Open / could not determine

1. **`rotate-assets` depends on libm's `sin`/`cos`.** The committed `after` stores
   `[0.0, 0.0, 0.6816387600233341, 0.7316888688738209]` — `sin(0.75)`/`cos(0.75)` as f64. Angle 1.5
   rad about `+z` was picked because `cos(h)² + sin(h)² == 1.0` **exactly** for these doubles (so the
   inverse's `w` lands on exact `1.0` and `inverse_restores_before` is bit-exact), and because the
   bit patterns agree between macOS libm and V8. If some target platform's `f64::sin`/`cos` differs
   by 1 ULP, this one case's `after` and its inverse round trip would need regenerating. There is no
   exact non-identity alternative: any non-zero angle produces transcendental components.
2. **Non-finite invariants are unreachable from a committed payload.** JSON cannot express `NaN`/`inf`,
   so `change-scene-sun-azimuth`'s only invariant (`!is_finite()`) can never be a committed fixture.
   That leaf's probe therefore asserts the reachable behaviour instead: azimuth is unbounded (720°
   applies cleanly, unlike elevation's ±90 clamp) and the equality no-op fires.
3. **`cargo` was not run** (peer de-async sweep has the workspace broken). Nothing here is claimed to
   pass. Validation was structural only: every `include_str!` target exists, every glue `#[path]`
   resolves, `rustfmt --edition 2021` parses all 31 test files and `📦️glue.rs`, and `fixtures lint`
   reports the tree clean.
4. **`ShootingDiff::emblem`** (the composed `s.stdio.semio.image` child slot) is written by no
   mutation triad, so no fixture exercises it — matching its own doc comment.

## 🧰️ Authoring aid

`🧪️scratch/shooting-fixture-json.py` in this ticket holds the per-case `after` edits (one explicit,
hand-derived edit per mutation) and writes the JSON quartets with stable formatting. It is an
authoring tool, not a harness — nothing in the test tree depends on it.

---

# 🔺️ Follow-up — `🔺️diff/🔣️component.json` for all 31 cases

Added after the dev ruled the serialized diff the most important file in a case and the lint promoted
it into `CORE_CASE_FILES`. All 31 shooting cases are `applied`, so all 31 carry the JSON; none needed
`🔺️diff/🚫️component.absent`.

## 🧬️ `ShootingDiff`'s serde shape (read, not assumed)

```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingDiff { … }
```

Three findings that shape every committed file:

1. **`rename_all = "camelCase"` here, unlike the payload structs.** The mutation payloads carry NO
   serde attrs at all (so `new_name`, `to_index`, `asset_ids` stay snake_case on the wire), but the
   diff type and all three collection deltas and all three `*Patch` records DO carry
   `rename_all = "camelCase"`. A single fixture directory therefore mixes both conventions:
   `🦠️mutation/🔣️component.json` is snake_case and `🔺️diff/🔣️component.json` beside it is camelCase.
2. **Nothing carries `skip_serializing_if`.** So every committed diff is **19 keys wide** —
   `artifact, schema, assets, savedCameras, scene, shots, activeShotId, activeAssetId, emblem,
   selectedShotIds, activeUtilityId, defaultShotFormat, defaultShotShape, defaultAssetFormat,
   centerModel, fitRevision, cameraDraftLabel, camera, locale` — with `null` in the 18 untouched
   slots. Same rule one level down: a delta always shows all four of
   `added` / `removed` / `patched` / `reordered` (`reordered: null` when unset), and a patch always
   shows all of its slots (`ShootingAssetPatch` 5, `ShootingShotPatch` 5,
   `ShootingSavedCameraPatch` 2) with `null` for the ones the mutation did not fill.
3. **The diff spans all three state lanes.** `ShootingDiff` carries presence (`selectedShotIds`,
   `activeUtilityId`) and config (`camera`, `locale`, `centerModel`, `fitRevision`, the three
   `default*` fields) slots alongside the artifact lane. No mutation triad in this tree writes any of
   them, so all 31 committed diffs leave those 10 slots null — which is itself worth pinning.

## 🩹️ What each mutation family's diff actually sets

| family | diff slot(s) set | shape |
| --- | --- | --- |
| `create-*` | `assets` / `shots` / `savedCameras` | `added: [<whole record>]` |
| `delete-*` | same | `removed: ["<id>"]` — a bare id, never a record |
| `rename-*`, `change-asset-url`, `change-shot-{width,height,format,shape}` | same | `patched: [{id, patch:{one slot filled, rest null}}]` |
| `reorder-*` | same | `reordered: ["<complete id sequence>"]`, nothing patched |
| `drag/rotate/scale-assets` | `assets` | one `patched` entry per **resolved** asset, carrying the already-resolved absolute value |
| `replace-shot-camera` | `savedCameras` | patched, keyed by the **dereferenced camera id** — `shots` stays null |
| `replace-saved-camera-view` | `savedCameras` | `patch: {label: null, camera: <whole pose>}` |
| `set-active-{shot,asset}` | `activeShotId` / `activeAssetId` | a bare scalar, **no collection delta at all** |
| the seven `change-scene-*` | `scene` | the **whole cloned `ShootingSceneLighting`** |

The last row is the notable one: shooting's scene leaves are the coarsest diffs in the tree. There is
no `ShootingScenePatch` slot on `ShootingDiff` (that record exists on the artifact root but is only
used by the DSL/OpText mirror), so `change-scene-sun-azimuth` — a one-float edit — ships sun, ambient,
shadow, material and background wholesale. The seven scene fixtures therefore assert the *carried*
values explicitly (e.g. sun-intensity's diff pins `ambient.intensity == 1.15` and sun-elevation's pins
`material.roughness == 1.0`), because "unchanged" is the only guarantee left once the block is coarse.

Three diffs are also the only place a claim is checkable at all:
`replace-shot-camera` (payload names a shot, delta patches a camera, `shots` null),
`drag-assets` (three ids in, **two** patch entries out — the ghost contributes nothing), and
`change-shot-width`/`-height` (the sibling dimension is explicitly `null` *in the patch*, which is the
only real proof there is no aspect coupling).

## ✅️ Three assertions added per file (31 × 3 = 93)

Matching puzzle5d's `📍move-part2d` form, each worded for its own mutation:

- `produces_committed_diff` — `mutation().diff(&before()).diff()` serialized equals the committed JSON,
  plus **three case-specific structural assertions** naming that mutation's own filled slot and the
  sibling slots that must stay null (e.g. rotate pins `patch.orientation[3] == cos(angle/2)` and
  `patch.origin.is_null()`; create-shot pins that `cameraId` is *absent*, not null, because
  `ShootingShot.camera_id` does skip-serialize).
- `committed_diff_is_canonical` — `ShootingDiff` decode→encode is a fixed point.
- `committed_diff_applies_to_after` — `decoded.apply(&before())` equals `expected_after()`.

The seam noted earlier composed cleanly: `use crate::artifacts::shooting::{ShootingDiff, ShootingSnapshot};`
plus the already-imported `protocol::{Mutation, MutationDiff}` covers all three, with no new helper.

## 🔍️ Verification (cargo still unusable)

- `fixtures lint --by-tree` — shooting appears in neither the uncovered list nor the error list; a
  local replay of `lintCase`'s error rules (including the new `🔺️diff/🔣️component.json` core entry)
  over the 31 shooting cases returns **no findings**, so the tree is clean even though the CLI
  truncates its error printout at 40 rows.
- `rustfmt --edition 2021 --emit stdout` parses all 31 test files and `📦️glue.rs`. 0 failures.
- `include_str!` — 155 targets across 31 files, all resolve; every file references
  `🔺️diff/🔣️component.json`.
- **New:** `🧪️scratch/shooting-fixture-verify.py` re-implements `ShootingDiff::apply` /
  `apply_identified_delta` from `🔺️diff/📝️text/🦀️component.rs` in Python and confirms for all 31 cases
  that `apply(committed_diff, before) == after`, **and** that each committed file has the exact 19-key
  diff field set, the exact 4-key delta field set and the exact patch-slot sets. This is a
  transcription check on the JSON, not a claim that the Rust tests pass.

## 🧰️ Authoring aids added

`🧪️scratch/shooting-fixture-diff.py` (the 31 hand-transcribed diffs) and
`🧪️scratch/shooting-fixture-diff-tests.py` (the per-case doc comments, extra assertions and failure
messages, spliced into the existing files). Both are authoring tools; the test tree depends on
neither.
