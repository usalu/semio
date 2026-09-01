# Puzzle + Forms Leaf Mutation Mirrors (44 of 107 in `empty-leaf-any.txt`)

Scope: `🧩️puzzle/🗿️artifacts/🧊️3d` (35) and `📋️forms/🗿️artifacts/📋️forms` (9) `🦠️mutation/🟦️component.ts`
leaves listed in `/private/tmp/.../scratchpad/empty-leaf-any.txt`. All 63 `🗄️stdio` `set-snapshot`
triads in that list were **not touched** — a concurrent session is actively deleting those modules
(375 modified `.rs` under stdio with `D` deletions of `📄set-snapshot/*`); filling them would be
wasted work in direct conflict, per the assignment's explicit instruction.

Both facets have only `🦠️mutation` leaves in this stub set — no `🔺️diff`/`↩️inverse` TS leaves exist
on disk for any of these 44 verbs (puzzle's diff/inverse subdirs hold only `🦀️.rs`; forms' hold
`🦀️component.rs` but no TS component at all), so nothing there needed mirroring.

## Puzzle 3D (35/35) — fixture-confirmed casing

Every verb's Rust struct (`<verb>/🦀️.rs`) carries `#[serde(rename_all = "camelCase")]`, matching
the pre-existing `🧬️mutations/🟦️component.ts` union file's inline field spellings, and I
spot-confirmed several (`add-object-vortex`, `connect-vortices`, `change-object-anchor`,
`replace-kind-catalogs`, `connect-kind-compatibility`, `replace-reference-source`) against their
committed `🧪️tests/*/🦠️mutation/🔣️component.json` fixtures — all camelCase, all matched exactly.
Wrote 35 leaf files as standalone `export interface <Name> { ... }`, importing shared entity types
(`Puzzle3dObject`, `Puzzle3dVortex`, `Puzzle3dObjectAnchor`, `Puzzle3dTargetVolume`,
`Puzzle3dReference`, `Puzzle3dCompatSpecificity`) from `../../../📸️snapshot/🟦️component.ts`, and
mutation-only shared types (`Puzzle3dScale`, `Puzzle3dReferenceSource`, `Puzzle3dKindCatalogs`)
from the sibling top-level `../../🟦️component.ts`. `Option<T>` fields with no
`skip_serializing_if` (e.g. `index`, `new_orientation`, `new_scale`, `new_mesh_url`, `new_label`,
`new_object_kind`, `new_catalogs`) are typed as required keys with a `| null` value, per rule 5 —
confirmed against fixtures showing explicit `"index": null` etc.

| Verb dir | Interface | Rust :line |
|---|---|---|
| 🌱create-object | CreateObject | 🦀️.rs:14 |
| 🗑delete-object | DeleteObject | 🦀️.rs:15 |
| 📍move-object | MoveObject | 🦀️.rs:13 |
| 🔃rotate-object | RotateObject | 🦀️.rs:13 |
| 📏scale-object | ScaleObject | 🦀️.rs:13 |
| 🧱change-object-mesh | ChangeObjectMesh | 🦀️.rs:13 |
| 🖋️edit-object-label | EditObjectLabel | 🦀️.rs:13 |
| 🏗change-object-kind | ChangeObjectKind | 🦀️.rs:13 |
| ⚓change-object-anchor | ChangeObjectAnchor | 🦀️.rs:13 |
| 👁change-object-hidden | ChangeObjectHidden | 🦀️.rs:13 |
| 🔒change-object-locked | ChangeObjectLocked | 🦀️.rs:13 |
| ➕add-object-vortex | AddObjectVortex | 🦀️.rs:14 |
| ➖remove-object-vortex | RemoveObjectVortex | 🦀️.rs:14 |
| 🔌replace-object-vortex | ReplaceObjectVortex | 🦀️.rs:14 |
| 🔗connect-vortices | ConnectVortices | 🦀️.rs:15 |
| ✂️disconnect-vortices | DisconnectVortices | 🦀️.rs:13 |
| 🧮replace-attraction-geometry | ReplaceAttractionGeometry | 🦀️.rs:13 |
| 🌍create-target-volume | CreateTargetVolume | 🦀️.rs:14 |
| 🪦delete-target-volume | DeleteTargetVolume | 🦀️.rs:13 |
| 🚀move-target-volume | MoveTargetVolume | 🦀️.rs:13 |
| 🌀rotate-target-volume | RotateTargetVolume | 🦀️.rs:13 |
| 📐scale-target-volume | ScaleTargetVolume | 🦀️.rs:13 |
| 🙈change-target-volume-hidden | ChangeTargetVolumeHidden | 🦀️.rs:13 |
| 🔐change-target-volume-locked | ChangeTargetVolumeLocked | 🦀️.rs:13 |
| 🖼create-reference | CreateReference | 🦀️.rs:14 |
| 🚮delete-reference | DeleteReference | 🦀️.rs:13 |
| 🎯move-reference | MoveReference | 🦀️.rs:13 |
| 📎resize-reference | ResizeReference | 🦀️.rs:13 |
| 🖇replace-reference-source | ReplaceReferenceSource | 🦀️.rs:13 |
| 👀change-reference-hidden | ChangeReferenceHidden | 🦀️.rs:13 |
| 🗝change-reference-locked | ChangeReferenceLocked | 🦀️.rs:13 |
| 🌐change-domain | ChangeDomain | 🦀️.rs:13 |
| 🤝connect-kind-compatibility | ConnectKindCompatibility | 🦀️.rs:13 |
| 💔disconnect-kind-compatibility | DisconnectKindCompatibility | 🦀️.rs:14 |
| 📚replace-kind-catalogs | ReplaceKindCatalogs | 🦀️.rs:15 |

None are marker variants with no fields — every one carries at least `id`/target-identity data, so
no empty-interface case applied here.

## Forms (9/9) — fixture-confirmed casing, and a real bug found in the process

**Casing trap sprung for real, not hypothetically.** None of the 9 target Rust structs
(`<verb>/🦠️mutation/🦀️component.rs`) carry `#[serde(rename_all = ...)]` — only the *enum*
(`FormMutation` in `🧬️mutations/🦀️component.rs:26`) has `#[serde(tag = "mutation", rename_all =
"camelCase")]`, which only governs the `"mutation": "createStep"` tag, not the payload fields. Per
rule 1/2, every payload field therefore stays **snake_case**. I confirmed this against all 9
committed `🧪️tests/*/🦠️mutation/🔣️component.json` fixtures (e.g. `🔀reorder-step`'s fixture is
`{"mutation": "reorderStep", "id": "...", "to_index": 1}` — camelCase tag, snake_case field) and
against the one forms leaf that was *not* in the stub list, `📝change-step-description`, whose
fixture likewise reads `"new_description": null`.

The pre-existing, hand-written `📋️forms/…/🧬️mutations/🟦️component.ts` top-level union file
(already in the repo, not part of this ticket's 44) gets this wrong: it declares
`{ mutation: 'reorderStep'; id: string; toIndex: number }`, `newTitle`, `stepId`, `blockId`,
`toStepId`, `newDescription` — all camelCase, contradicting every fixture. That file also carries a
now-stale docstring claiming "none of forms' ten verbs has a real leaf to import from," which
became false once these 9 leaves were filled. **Not fixed** — out of this ticket's explicit 44-file
scope, and touching the widely-imported top-level union risks colliding with the heavy concurrent
churn already happening elsewhere in the repo. Flagging here for a follow-up ticket.

| Verb dir | Interface | Rust :line | Fields (snake_case) |
|---|---|---|---|
| 🌱create-step | CreateStep | 🦠️mutation/🦀️component.rs:14 | step, index |
| 🗑️delete-step | DeleteStep | 🦠️mutation/🦀️component.rs:13 | id |
| 🔀reorder-step | ReorderStep | 🦠️mutation/🦀️component.rs:13 | id, to_index |
| ✏️rename-step | RenameStep | 🦠️mutation/🦀️component.rs:13 | id, new_title |
| ➕create-block | CreateBlock | 🦠️mutation/🦀️component.rs:14 | step_id, block, index |
| ➖delete-block | DeleteBlock | 🦠️mutation/🦀️component.rs:13 | step_id, id |
| 📦move-block-to-step | MoveBlockToStep | 🦠️mutation/🦀️component.rs:16 | step_id, block_id, to_step_id, index |
| 🔁replace-block | ReplaceBlock | 🦠️mutation/🦀️component.rs:14 | step_id, block |
| 🏷️change-form-title | ChangeFormTitle | 🦠️mutation/🦀️component.rs:13 | new_title |

`step`/`block` fields import `FormStep`/`FormQuestion` from the sibling top-level
`../../🟦️component.ts` — those entity types (unlike the mutation-payload field casing) are correct
there: they mirror `flow::playbook::{PlaybookStep, PlaybookBlock}`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs:31,41`), which *does* carry
`#[serde(rename_all = "camelCase")]`, and the existing inline TS fields matched that struct
field-for-field.

`CreateStep.index`/`CreateBlock.index`: `Option<usize>` with no `skip_serializing_if` → required
key, `number | null`, confirmed against fixtures (`"index": null` present). `ChangeFormTitle.
new_title`: same pattern, `string | null`.

## Fixture-confirmed vs attribute-derived

- Fixture-confirmed casing (a committed `🦠️mutation/🔣️component.json` was read and matched): all 9
  forms leaves, and 6 of 35 puzzle leaves spot-checked directly (`add-object-vortex`,
  `connect-vortices`, `change-object-anchor`, `replace-kind-catalogs`,
  `connect-kind-compatibility`, `replace-reference-source`).
- Attribute-derived only (no fixture spot-checked individually, but `#[serde(rename_all =
  "camelCase")]` is present verbatim on the struct, and the field list matches the pre-existing
  top-level puzzle union file which itself does match every fixture format style seen): remaining
  29 puzzle leaves. All 35 puzzle structs use the same `rename_all = "camelCase")]` line, so this is
  low-risk, but only 6 were individually fixture-verified.

## Verification (real output)

```
$ bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
    --esModuleInterop --skipLibCheck --allowImportingTsExtensions \
    "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📦️index.ts"
(no output) — exit 0

$ bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
    --esModuleInterop --skipLibCheck --allowImportingTsExtensions \
    "✏️s/🔌️plugins/📋️forms/📦️packages/🟦️typescript/📦️index.ts"
(no output) — exit 0
```

`git status --porcelain` scoped to the exact 44 target file paths → 44 lines (all modified, none
missing). No file in the 44 still contains a bare `export {};` (grep confirmed empty).

## Not fixed / follow-ups (honest boundaries)

- Forms' top-level `🧬️mutations/🟦️component.ts` union file has wrong (camelCase) field casing for
  every one of the 10 verb payloads it inlines, and a stale docstring — out of this ticket's scope,
  not touched.
- Puzzle's top-level `🧬️mutations/🟦️component.ts` union file already duplicates the same interface
  definitions I wrote into the 35 leaves (verified correct, not a casing bug there) — this
  pre-existing duplication (leaf vs. union inline) was not resolved; doing so would mean editing a
  widely-imported shared file outside this ticket's 44-file scope.
- Only 6/35 puzzle leaves were fixture-spot-checked individually rather than all 35; the remaining
  29 rest on the uniform `#[serde(rename_all = "camelCase")]` attribute plus agreement with the
  existing (independently-authored) top-level union file.
