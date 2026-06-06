# Fill Target Volumes

Extended puzzle 3d fill with persisted oriented-box **target volumes** that constrain fill placement to AABB-strict containment inside the union of volumes.

## Delivered

- **Shared primitive** (`infinite/world/r3f`): `WorldVolumeProps`, pose/transform helpers, `WorldVolumeLayer` / `WorldVolumeBoxItem` with gumball, `worldVolumesContainAabb`.
- **Fixture** (`puzzle/3d/react`): `FixtureV1.targetVolumes[]`, parse/encode, add/update/remove/relocate ops, selection pick kind `targetVolume`.
- **Draw tool**: 3-point + height CAD-box session with live ghost preview; commits via `onTargetVolumeDraw`.
- **Fill sub-mode**: `fillEditTargetVolumes` controller state, engagement toggle "Edit target volumes" + "Delete volume".
- **Fill constraint**: TS stepper + Rust `fill_step_one` gate placements when volumes are non-empty; invalidation on volume change.
- **Playground host**: wires props, relocate/draw handlers, fill session prep with `targetVolumes`.
- **Hierarchy**: Target Volumes group in play tree panel.

## Tests

| Package | Result |
|---------|--------|
| `@infinite/world/r3f` | 52 passed |
| `@puzzle/3d/react` | 338 passed |
| `@puzzle/3d/play` | 71 passed |
| `@puzzle/3d/rs` | `world_volumes_contain_aabb_respects_oriented_box` ok |

## Runtime `[DEBUG]` hooks

- `puzzle3d addTargetVolume`, `patchTargetVolumeRelocate`, `target volume committed`
- `puzzle3d fill skip outside target volume` when TS stepper rejects a candidate
