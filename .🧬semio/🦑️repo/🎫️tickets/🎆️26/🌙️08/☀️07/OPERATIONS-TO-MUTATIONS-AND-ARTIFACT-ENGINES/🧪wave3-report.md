# Wave 3 Report — Lowpoly Pilot (Operations → Mutations)

## Gate

| Command | Result | Log |
|---------|--------|-----|
| `cargo check -p semio-s-plugin-lowpoly` | **PASS** | `🧪wave3-lowpoly-check.txt` |
| `cargo test -p semio-s-plugin-lowpoly --lib` | **PASS** (138 tests) | `🧪wave3-lowpoly-test.txt` |

Use `DEVELOPER_DIR=/Library/Developer/CommandLineTools` on this machine when `xcode-select` points at a full Xcode that is unavailable.

## Reference layout (copy for Wave 4)

```
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/
  🧬️mutations/                         # REQUIRED facet
    🦀️component.rs                     # LowpolyMutation dispatch enum + Mutation impl
    🟦️component.ts                     # TS facade stub
    <emoji><kebab-name>/               # one dir per mutation
      🦠️mutation/{🦀️component.rs,🟦️component.ts}   # struct + builder + apply
      🔺️diff/{…}                                      # per-mutation Diff fragment
      ↩️inverse/{…}                                    # inverse(&base, …) -> Vec<LowpolyMutation>
  🔧️op/                                # KEPT: OpText/OpBinary + grammar only
  🔺️diff/                              # aggregate LowpolyDiff { mutations }
  ⚙️engine/                             # LowpolyEngine: ArtifactEngine
  🗣️dsl/ 🎒️pack/ 📡️spr/ 📚️examples/
```

### Mutation dirs (lowpoly)

| Dir | Payload |
|-----|---------|
| `➕️objects-add` | ObjectsAdd |
| `➖️objects-remove` | ObjectsRemove |
| `↔️objects-move` | ObjectsMove |
| `🩹objects-patch` | ObjectsPatch |
| `➕️add-paint-layer` | AddPaintLayer |
| `➖️remove-paint-layer` | RemovePaintLayer |
| `🩹patch-paint-layer` | PatchPaintLayer |
| `🖌️paint-stroke` | PaintStroke (+ PixelRun) |
| `🖼️set-projection` | SetProjection |

Shared paint helpers (`LowpolyPaintLayerPatch`, `PixelRun`, `apply_paint_layer_patch`, `apply_pixel_runs`) live in `🧬️mutations/🦀️component.rs`.

### Glue wiring

`📦️packages/🦀️rust/📦️glue.rs` declares `artifacts::lowpoly::mutations` with nested `objects_add` / … modules (`mutation` / `diff` / `inverse` leaves). Root `pub use component::*` re-exports builders.

### Rename table applied in this plugin

| Old | New |
|-----|-----|
| `LowpolyOperation` | `LowpolyMutation` |
| `LowpolyConfigOperation` | `LowpolyConfigMutation` |
| `NoDraftOperation` | `NoDraftMutation` |
| `apply_lowpoly_operation` / `invert_*` | `apply_lowpoly_mutation` / `inverse_lowpoly_mutation` |
| `document_operations` / `Emit::operations` | `document_mutations` / `Emit::mutations` |
| `DocumentApp::Operation` / `ConfigOperation` | `Mutation` / `ConfigMutation` |
| `OperationDiff` / `CollectionOperation` | `MutationDiff` / `CollectionMutation` |
| serde tag `"operation"` | `"mutation"` |
| grammar `start operation` / `operation =` | `start mutation` / `mutation =` |
| spr `schema lowpoly.document.operation` | `schema lowpoly.document.mutation` |
| Diff field `operations` | `mutations` |

**Kept:** `🔧️op`, `*.op.semio`, `OpText`, `OpBinary`, `print_op`/`parse_op`/`encode_op`/`decode_op`, `LanguageRole::Ops`.

### Engine

```rust
pub struct LowpolyEngine { projection: LowpolyProjection }
impl ArtifactEngine for LowpolyEngine {
    type Projection = LowpolyProjection;
    type Mutation = LowpolyMutation;
    type Diff = LowpolyDiff;
    // apply/inverse delegate to mutation modules
}
```

`LowpolyDocument` remains an engine-internal compute session.

### App

- `DocumentApp` associated types: `Mutation` / `ConfigMutation` / `DraftMutation`
- `app_commands!` for `LowpolyProjection, LowpolyMutation, LowpolyConfig, LowpolyConfigMutation, ctx = LowpolyScratch`
- Mid-gesture scratch persists via TLS (`LOWPOLY_SCRATCH`) because `handle` is an associated fn with no `&mut self` (host session slot lands later)

### TS package

`📦️index.ts` keeps `lowpoly_op` and adds `lowpoly_mutations`.

## Wave 4 checklist

1. Create `🧬️mutations/` with one emoji-prefixed dir per mutation (`🦠️mutation` / `🔺️diff` / `↩️inverse`).
2. Move apply/inverse out of monolithic `🔧️op`; keep OpText/OpBinary + grammar there (re-export the mutation enum).
3. Implement `<Artifact>Engine: ArtifactEngine` in `⚙️engine`.
4. Rename `*Operation` → `*Mutation`, Emit fields, DocumentApp associated types, Collection helpers.
5. Grammar: `start mutation` / `mutation =`. Protocol: `schema <x>.mutation` if present.
6. Wire modules in `📦️glue.rs`; extend TS index with `*_mutations`.
7. `cargo check -p <crate>` (and crate tests if any).
8. No legacy `Operation` names for the document-mutation concept; keep Op* brand.

## Collateral fixes (required for green gate)

These are outside the plugin folder but blocked `cargo test -p semio-s-plugin-lowpoly`:

1. **`🧬️semio` preamble_line** — emit `semio plugin.artifact.component vN` (dotted component) to match `parse_preamble_line`.
2. **`🏪️store` `DocumentStore::dispatch` receipt** — do not slice `applied_edit_ids[before..]` when Undo/Redo shrinks the list.
3. **Envelope ids** — `lowpoly.lowpoly` / `lowpoly.lowpolycfg` (`plugin.artifact` form).
4. **CAD optional** — `cad-fixtures` feature; default lib tests no longer compile broken CAD mid-Wave-4.

## Files of note

- Mutations root: `🗿️artifacts/💠️lowpoly/🧬️mutations/🦀️component.rs`
- Op codecs: `…/🔧️op/🦀️component.rs`
- Engine: `…/⚙️engine/🦀️component.rs` (`LowpolyEngine`)
- Glue: `📦️packages/🦀️rust/📦️glue.rs`
- Setup restore: `🔌️plugin/🔧️setup/🦀️component.rs` (`register_lowpoly_exports`)
