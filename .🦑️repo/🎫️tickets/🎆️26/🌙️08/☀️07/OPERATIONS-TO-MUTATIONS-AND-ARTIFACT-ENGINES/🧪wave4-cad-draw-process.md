# Wave 4 Report — CAD / Draw / Process3d (Operations → Mutations)

## Gate

| Crate | Command | Result | Log |
|-------|---------|--------|-----|
| `semio-s-plugin-cad` | `cargo check -p semio-s-plugin-cad --lib` | **PASS** | `🧪wave4-cad-check.txt` |
| `semio-s-plugin-draw` | `cargo check -p semio-s-plugin-draw --lib` | **PASS** | `🧪wave4-draw-check.txt` |
| `semio-s-plugin-process` | `cargo check -p semio-s-plugin-process --lib` | **PASS** | `🧪wave4-process-check.txt` |

Use `DEVELOPER_DIR=/Library/Developer/CommandLineTools` on this machine when `xcode-select` points at a full Xcode that is unavailable.

## Layout (Wave 3 checklist)

Each artifact now has:

```
🗿️artifacts/<artifact>/
  🧬️mutations/                         # REQUIRED facet
    🦀️component.rs                     # <Artifact>Mutation dispatch enum + Mutation impl
    🟦️component.ts                     # TS facade stub
    <emoji><kebab-name>/               # one dir per mutation
      🦠️mutation/{🦀️component.rs,🟦️component.ts}
      🔺️diff/{…}
      ↩️inverse/{…}
  🔧️op/                                # KEPT: OpText/OpBinary + grammar only (re-exports mutation enum)
  🔺️diff/                              # aggregate Diff (MutationDiff)
  ⚙️engine/                             # ArtifactEngine impl
```

### CAD (`📐️cad`) — 14 mutations

| Dir | Variant |
|-----|---------|
| `➕️add-object` | AddObject |
| `➖️remove-object` | RemoveObject |
| `🩹patch-object` | PatchObject |
| `↕️translate-objects` | TranslateObjects |
| `🔄rotate-objects` | RotateObjects |
| `↔️scale-objects` | ScaleObjects |
| `🖼️set-pane-objects` | SetPaneObjects |
| `➕️add-node` | AddNode |
| `➖️remove-node` | RemoveNode |
| `🏷️rename-node` | RenameNode |
| `🩹patch-reference` | PatchReference |
| `📎set-references` | SetReferences |
| `🎯set-active-model-definition` | SetActiveModelDefinition |
| `🎬️set-scene` | SetScene |

Shared patches (`CadObjectPatch`, `CadNodePatch`, `CadReferencePatch`) live in `🧬️mutations/🦀️component.rs`. Aggregate `CadDiff` stays field-level (not mutation-list).

### Draw (`🖍️draw`) — 15 mutations

| Dir | Variant |
|-----|---------|
| `👁️set-layer-visible` | SetLayerVisible |
| `🔒️set-layer-locked` | SetLayerLocked |
| `🌫️set-layer-opacity` | SetLayerOpacity |
| `🖌️set-layer-blend-mode` | SetLayerBlendMode |
| `🏷️set-layer-name` | SetLayerName |
| `↔️set-layer-transform` | SetLayerTransform |
| `🎨set-fill` | SetFill |
| `✏️set-stroke` | SetStroke |
| `🔀set-boolean-operation` | SetBooleanOperation *(field `boolean_operation` kept — different concept)* |
| `🖼️set-trace-params` | SetTraceParams |
| `➕️add-layer` | AddLayer |
| `🧬️duplicate-layer` | DuplicateLayer |
| `➖️remove-layer` | RemoveLayer |
| `🔃reorder-layer` | ReorderLayer |
| `📄set-document` | SetDocument |

Inverse for draw is document snapshot (`SetDocument`). Apply helpers: `apply_draw_edit_mutation`.

### Process3d (`🏭️process` / `🧊️process3d`) — 5 mutations

| Dir | Variant |
|-----|---------|
| `📋steps` | Steps { collection: CollectionMutation } |
| `🛠️machines` | Machines { collection: CollectionMutation } |
| `🧱set-stock` | SetStock |
| `⏱️set-cursor` | SetCursor |
| `📄set-document` | SetDocument |

DSL twin (`Process3dMutationDsl`) stays in `🔧️op` for OpText/OpBinary (foreign `CollectionMutation` orphan rule).

## Rename table applied

| Old | New |
|-----|-----|
| `CadOperation` / `DrawOperation` / `Process3dOperation` | `*Mutation` |
| `CadConfigOperation` / `DrawConfigOperation` / `Process3dConfigOperation` | `*ConfigMutation` |
| `Operation` / `OperationDiff` | `Mutation` / `MutationDiff` |
| `CollectionOperation` / `apply_collection_operation` / `invert_*` | `CollectionMutation` / `apply_collection_mutation` / `inverse_collection_mutation` |
| `document_operations` / `config_operations` / `Emit::operations` | `document_mutations` / `config_mutations` / `Emit::mutations` |
| `DocumentApp::Operation` / `ConfigOperation` / `DraftOperation` | `Mutation` / `ConfigMutation` / `DraftMutation` |
| serde tag `"operation"` | `"mutation"` |
| grammar `start operation` / `operation =` | `start mutation` / `mutation =` |
| spr `schema *.operation` | `schema *.mutation` |

**Kept (Op brand):** `🔧️op`, `*.op.semio`, `OpText`, `OpBinary`, `print_op`/`parse_op`/`encode_op`/`decode_op`, `LanguageRole::Ops`.

**Untouched (different concepts):** CAD scripting `commit.operation.*`, draw `boolean_operation`, query/binop `operation` fields, “no-operation” comments.

## Engines

```rust
impl protocol::ArtifactEngine for CadEngine { /* Projection=CadProjection, Mutation=CadMutation, Diff=CadDiff */ }
impl protocol::ArtifactEngine for DrawEngine { /* … DrawDocument / DrawMutation / DrawDiff */ }
impl protocol::ArtifactEngine for Process3dEngine { /* … Process3dDocument / Process3dMutation / Process3dDiff */ }
```

## Glue / TS

- `📦️packages/🦀️rust/📦️glue.rs` — `artifacts::<name>::mutations` with nested per-mutation `mutation` / `diff` / `inverse` modules.
- TS index adds `cad_mutations` / `draw_mutations` / `process3d_mutations` beside existing `*_op` exports.

## Collateral fixes (CAD green)

Pre-existing CAD mid-refactor breakage unblocked `cargo check` (same class as Wave 3 lowpoly collateral):

1. Restored broken `CadPlayApp` / `DocumentApp` associated types + TLS `CAD_PREVIEW_SEQ` / `gesture_preview`.
2. `cad_brep_host` OnceLock; `**kernel` → `*kernel` (Brep guard is single-indirect).
3. `CadEngagementContext` `Deref`/`DerefMut`; interaction catalog `OnceLock`.
4. `next_cad_id` without `blake3`; `CAD_DEFAULT_TYPOLOGY_EXTENT`; contribution handler accepts `CadDispatchCtx`.

CAD scripting/kernel `operation` fields intentionally unchanged.

## Files of note

- CAD mutations: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧬️mutations/`
- Draw mutations: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🧬️mutations/`
- Process3d mutations: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🧬️mutations/`
- Engines: `…/⚙️engine/🦀️component.rs` (`CadEngine` / `DrawEngine` / `Process3dEngine`)
- Glue: each plugin `📦️packages/🦀️rust/📦️glue.rs`
