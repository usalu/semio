# Wave-R / r2a — gis + shooting `CollectionMutation` leaf rewrites

Scope: drop the throwaway `protocol::CollectionMutation` manufacture from 13 triad leaves (gis: 12
`🔺️diff` leaves + 3 `↩️inverse` leaves that also hit the banned vocabulary in doc comments;
shooting: 1 `🦠️mutation` leaf whose doc comment named the type), per
`📓️remaining-work-map.md`'s "CollectionMutation debt in migrated leaves" and `📓️taxonomy.md`'s
forbidden-vocabulary rule.

## gis (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/🧬️mutations/`)

All 12 `🔺️diff/🦀️component.rs` inlined the matching arm of
`features_delta_from_collection_mutation` (read from
`…/🔺️diff/📝️text/🦀️component.rs:169`) directly from the payload, dropped the `protocol::
CollectionMutation` import, and reworded doc comments to avoid the banned tokens.

| leaf | before | after |
|---|---|---|
| `🆕create-position/🔺️diff` | built `CollectionMutation::Add{index,item}` then called the helper | `GisMapFeaturesDelta{ added: vec![payload.item.clone()] }` (index was already ignored by the `Add` arm) |
| `🆕create-route/🔺️diff` | same, `routes` | `GisMapFeaturesDelta{ added: vec![payload.item.clone()] }` on `routes` |
| `🆕create-region/🔺️diff` | same, `regions` | `GisMapFeaturesDelta{ added: vec![payload.item.clone()] }` on `regions` |
| `🗑delete-position/🔺️diff` | `CollectionMutation::Remove{id}` + helper | `GisMapFeaturesDelta{ removed: vec![payload.id.clone()] }` |
| `🗑delete-route/🔺️diff` | same, `routes` | `GisMapFeaturesDelta{ removed: vec![payload.id.clone()] }` on `routes` |
| `🗑delete-region/🔺️diff` | same, `regions` | `GisMapFeaturesDelta{ removed: vec![payload.id.clone()] }` on `regions` |
| `🔁replace-position-data/🔺️diff` | `CollectionMutation::Patch{id,patch}` + helper | `GisMapFeaturesDelta{ patched: vec![GisMapFeaturePatchEntry{id, patch: MapFeaturePatch{data: Some(new_data)}}] }` |
| `🔁replace-route-data/🔺️diff` | same, `routes` | same shape on `routes` |
| `🔁replace-region-data/🔺️diff` | same, `regions` | same shape on `regions` |
| `🔀reorder-positions/🔺️diff` | `CollectionMutation::Move{id,to_index}` + helper | inlined the `Move` arm's id-order recompute from `base.positions` directly into the leaf, `GisMapFeaturesDelta{ reordered: Some(ids) }` |
| `🔀reorder-regions/🔺️diff` | same, `regions` | same recompute against `base.regions` |
| `🔀reorder-routes/🔺️diff` | same, `routes` | same recompute against `base.routes` |
| `🗑delete-position/↩️inverse` | doc comment said "the taxonomy's replacement for the banned `NoMutation` sentinel" | reworded to "an empty inverse rather than a no-op sentinel mutation" (logic unchanged — no `CollectionMutation` was ever in the code, only the banned word in prose) |
| `🗑delete-route/↩️inverse` | same | same reword |
| `🗑delete-region/↩️inverse` | same | same reword |

For all "add"/"remove"/"patch" leaves, `base` is now unused and the parameter was renamed to
`_base`. The three `reorder-*` leaves keep `base` (they genuinely need it to recompute the id
order) — unchanged signature there.

### Helper deletion

`grep -rn "features_delta_from_collection_mutation" ✏️s/🔌️plugins/🌍️gis` → zero remaining callers
after the rewrite (the only hit left was the function's own `pub fn` line). **Deleted** the fn
from `…/🔺️diff/📝️text/🦀️component.rs`'s `//#region 🔹Helpers`, and dropped the now-dead
`protocol::CollectionMutation` import and the now-dead `MapFeaturePatch` import from that same
file (both were only referenced by the deleted fn's signature).

`grep -rn "diff_set_snapshot" ✏️s/🔌️plugins/🌍️gis` → still has a caller (its own unit test
`a_whole_artifact_diff_wins_over_every_collection_diff` at line ~217, plus the sibling
`gisterrain` artifact has its own independent `diff_set_snapshot`). Left untouched per the "leave
`apply_*_delta`/`absorb_*_delta` alone" instruction — this fn builds a real `GisMapDiff{artifact:
Some(...)}` replacement diff, not a `CollectionMutation` bridge, so it was never in scope for
deletion.

`apply_features_delta` / `absorb_features_delta` left untouched as instructed.

## shooting (`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/…/🧬️mutations/📦assets/🦠️mutation/🦀️component.rs`)

This leaf's `CreateAsset` payload docstring referenced `CollectionMutation::Add` only in prose
(describing why `index` is advisory — the sibling `🔺️diff/🦀️component.rs::diff_create_asset`
already does real sparse construction, `ShootingAssetsDelta{ added: vec![payload.asset.clone()] }`,
with **no `CollectionMutation` anywhere in the actual code** of this triad — diff/inverse already
delegate to the sibling leaves per the triad contract). Fix was a doc-comment reword only:

- before: "matching the pre-migration `CollectionMutation::Add` behavior"
- after: "the item onto the end of the list, regardless of `index`"

No behavioral change; the `🔺️diff` and `↩️inverse` leaves for `📦assets` were already clean before
this ticket (verified by reading `🔺️diff/🦀️component.rs`, which builds `ShootingAssetsDelta`
directly for `CreateAsset`/`DeleteAsset`/`RenameAsset`/`ChangeAssetUrl`/`ReorderAssets`).

## Gate results

**`cargo check -p semio-s-plugin-gis`** — clean. 0 errors, ~21 pre-existing warnings unrelated to
this change (unnecessary qualifications, elided lifetimes, dead `artifact` field, etc., all outside
the touched files or pre-existing in files this ticket didn't author).

**`cargo test -p semio-s-plugin-gis --lib`**:
```
test result: ok. 170 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```
Ran twice (before and after the delete-* inverse docstring pass) — both green, 170/170.

**`cargo check -p semio-s-plugin-shooting`** — **NOT clean**: 36 pre-existing errors, all in
`🎛️apps/🎥️shooting/🎮️commands/{☀️scene,🎥️camera,📦️asset,📷️shot,🗃️fixture,🧭️gumball}/🦀️component.rs`
and `🎛️apps/🎥️shooting/🦀️component.rs` — none in `🧬️mutations/📦assets` or any file this ticket
touched. `git status --porcelain` on the shooting plugin tree shows only the one file this ticket
edited (the `🦠️mutation/🦀️component.rs` doc-comment fix); the 36 errors reference `ShootingMutation`
variants this ticket never touched (`PatchScene`, `Shots`, `SetSnapshot`, `Assets`,
`SetActiveAsset`/`SetActiveShot` field names, `ScaleAssets`/`RotateAssets` field names,
`SavedCameras`, `TranslateAssets`, `SetShotCamera`). This is the documented "App / glue funnel
debt" for shooting (`📓️remaining-work-map.md`: "shooting: `setSnapshotJson` route + `🗃️fixture`
handlers, 11 collection sites in `📦️asset`/`📷️shot`/`🎥️camera`") plus other unmigrated
`🧬️mutations` variants outside the `📦assets` facet — out of scope for this task per the "don't
chase other sessions' breakage" rule. **Did not attempt `cargo test -p semio-s-plugin-shooting`**
since the crate does not compile for reasons unrelated to this change; not run, not claimed to
pass.

## allowlistKeysToRemove

Repo-relative paths now free of `CollectionMutation`/`SetSnapshot`/`NoMutation` (raw content
including comments):

```
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-route/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-region/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-route/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-region/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-position/↩️inverse/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-route/↩️inverse/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-region/↩️inverse/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-route-data/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-region-data/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-regions/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-routes/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs
✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦assets/🦠️mutation/🦀️component.rs
```

(16 files, not 15 — the extra one is `…/gismap/…/🔺️diff/📝️text/🦀️component.rs`, the helper file
itself, which lost its `features_delta_from_collection_mutation` definition and is now also free of
the banned tokens.)

Still hitting the banned tokens (out of scope, untouched, left for other waves per
`📓️remaining-work-map.md`):
- `…/gismap/…/🧬️mutations/💾️binary/🦀️component.rs` (facet-level dispatch doc comment)
- `…/gismap/…/🧬️mutations/📝️text/🦀️component.rs` (facet-level dispatch doc comment)
- shooting's `🎛️apps/🎥️shooting/🎮️commands/*` and top-level `🧬️mutations/🦀️component.rs`
  (unmigrated variants / app-glue-funnel debt — pre-existing compile breakage, not touched)

## Files touched (16)

Same list as `allowlistKeysToRemove` above (12 gis `🔺️diff` leaves + 3 gis `↩️inverse` leaves +
1 gis facet-level `🔺️diff/📝️text` helper file + 1 shooting `🦠️mutation` leaf = 17 total edits,
16 distinct files free of the banned tokens as listed; the 17th touched line was the
`use protocol::{CollectionMutation, MutationDiff, Patchable}` → `use protocol::{MutationDiff,
Patchable}` import trim, in the same `📝️text/🦀️component.rs` file already counted above).
