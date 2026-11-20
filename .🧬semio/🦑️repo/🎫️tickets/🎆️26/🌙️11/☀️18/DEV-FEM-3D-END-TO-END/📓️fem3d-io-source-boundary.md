# fem3d source boundary blocker

## Finding
The refused crate is `semio-s-artifact-fem-3d-model` (schema/📦️packages/🦀️rust), not the io crate. Its closure pulled the io sqlite/binary/text snapshot files in through `#[path]`. The sqlite snapshot impls (`impl ... for Fem3dSnapshot`) cannot leave the model crate (orphan rule), so the split follows the diff and guards instead.

## Closure before/after (limits 262144 bytes / 32 files / 4000 active lines)
| crate | bytes | files | lines |
|---|---|---|---|
| fem-3d-model before | 262407 | 27 | 3590 |
| fem-3d-model after | 172274 | 21 | 2363 |
| fem-3d-diff (new) | 75047 | 6 | 1035 |
| fem-3d-guards (new) | 17424 | 2 | 261 |
| fem-3d-io | 74322 | 19 | 1032 |
| fem-2d-model (untouched, at risk) | 253810 | 28 | 3669 |
| fem-2d-editor | 179571 | 21 | 2733 |
| fem-2d-editor-session | 177890 | 9 | 3145 |
| fem-3d-editor-visual-job | 170526 | 13 | 3449 |

## Split
- `semio-s-artifact-fem-3d-diff`: `🧬️schema/🔺️diff/📦️packages/🦀️rust` (schema::diff, io::text::diff, io::binary::diff; module paths preserved).
- `semio-s-artifact-fem-3d-guards`: `🧬️schema/🛡️guards/📦️packages/🦀️rust` (schema::guards, depends on diff and model).
- Orphan fixes: `Keyed` impls for FemElement, FemLoad and the five `key: id` rows moved into model `🏗️structure`; the `list_delta!` rows use `key: String = keyed`.
- All 227 dependents rewritten to the new crate paths; Cargo deps and the 3d workspace members and dependencies registered.

## Verification
- wasm32-wasip2 `cargo check` of model, diff, guards, io and `semio-hub-fem-component`: clean.
- Native `cargo check --workspace --tests` of the 3d workspace: every lib target compiles. Remaining test-target errors (SealedShared, NativeDecodeWorkRecipient, RetireOwned, Arc<ViewModel>) are stale tests unrelated to diff or guards.
- `cargo test` mutations and load: all pass. Model lib: 3 failures ("sealed source requires original granted handback" in framework retirement), in code this split did not touch.
- The sqlite/binary snapshot stays in the model crate (orphan rule), so there is no snapshot crate and no snapshot test to run separately.
- Disk was at 0 bytes free once during the run (other builds in C:/b); builds ran with CARGO_INCREMENTAL=0. Private dirs deleted.

## Pending
`fem-2d-model` (253810 bytes, 28 files, 3669 lines) is next: same diff and guards split, held until the component build publishes.
