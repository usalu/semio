# Compile review C (wave 2): norm app-surface crates, dyn_enum_close, io orphan rule

Read-only analysis plus minimal fixes. No cargo/rustc, no builds, no git writes. Working-tree vs index diff (`git diff`, 53 files, about 2300 lines) was read in full for every `.rs` file, plus full reads of canvas-lowering, checks, editor-wiring (gesture part and reducer), lane-runtime, lane schema, properties, controls, field-meta, outliner, design-case-surface, shortcut-dispatch, shell-hooks, screen-reader, focus-order, manifest layout code, selection-mirror, retained-commands (changed part).

## Fixed (certain)
| file | issue | fix |
|---|---|---|
| `app-surface/🪟️visual-windows/🧾️properties/🧪️tests/🔬️unit/🦀️.rs:6` | `include_str!("../../🔮️oracles/🔣️.json")` resolves to `🧾️properties/🔮️oracles` which does not exist (oracle is in `🧪️tests/🔮️oracles`); breaks `panels:test` | `../🔮️oracles/🔣️.json` |

## Open (routed to requests.md)
1. **io crates implement foreign traits on foreign types (orphan rule, all 15 artifacts, high confidence, not yet seen in a log).**
   - `📝️text/📸️snapshot`: `impl_norm_artifact_record!(@text En1990Snapshot …)` expands to `impl NormArtifactRecord for` and `impl pack::snapshot::ArtifactDsl for` the model's snapshot. `💾️binary/📸️snapshot`: `@binary` expands to `ArtifactPack` and `ArtifactStoreSnapshot`. `🪶️sqlite/📸️snapshot`: inherent `impl En1990Snapshot {` (E0116) plus `ArtifactSqliteSnapshotHost`/`ArtifactSqliteSnapshot`. `📝️text/🧬️mutations` and `💾️binary/🧬️mutations`: `protocol::OpText`/`OpBinary` for `En1990Mutation` from the mutations crate (E0117). The model Cargo has no `pack` dependency and mounts nothing from `🚪️io`.
   - How the framework intends it: `🧊️generation3d` (procedural) mounts the codec files **inside the owning crate** (`#[path = "../🚪️io/📝️text/📸️snapshot/🪞️dsl-mirror/🦀️.rs"]` and `…/🔺️diff/🦀️.rs` in its model crate; `pub use …mutations::text as mutations` in the io text crate). Local newtypes are not an option because the editor/viewer generics require the traits on the snapshot type itself.
   - No build has reached these crates (diff fails first; the en1995 run 14 log stops at `…-diff`). Verification is one build of the smallest artifact (en1990) once diff and shared crates are green. Fix shape and owner request: `requests.md` section "from: compile-review C".
2. **Source budget at the gate.** `semio-s-artifact-norm-en1999-checks` 259934 B / 4007 lines (limit 4000 lines; was 4152 lines minutes earlier). `en1990-diff` was 4065 lines and is back under. Outside the ten artifacts: `vdi3805-model` 4276, `iso16757-model` 4433 lines.

## dyn_enum_close! in en1990 (checked, no change needed)
`NationalAnnex` now lives in `⚖️compliance/📜️annex-and-tables` mounted by `compliance-core`; `#[dyn_enum]` emits `#[macro_export] __semio_dispatch_NationalAnnex` at the core crate root. The facade `semio-s-plugin-norm-compliance` has `pub use compliance_core::*`, which re-exports exported macros, so `use semio_s_plugin_norm_compliance::__semio_dispatch_NationalAnnex;` in `en1990/…/⚖️checks/🦀️.rs:115` resolves (the framework's own `PluginApp` does the same with an explicit root re-export, but a glob re-export is equivalent for `macro_export` macros). The captured signatures name `AnnexChoice`, which en1990 imports. `semio-framework-dispatch-macros` is a dependency of both core and facade. `NationalAnnex` and `NaDe`/`NaEn` impls are local to en1990, so no orphan problem.

## Checks run (all clean unless listed)
- `use` resolution: every new name in properties (`annex_parameter_rows`, `NdpValue`, `NdpStatus`, `AnnexChoice::label`, `inputs_section_id`, `list_element_path`, `resolve_field_meta`, `controller_is_live`, `tree_group`, `tree_item_with_action`, `try_child`), shell-hooks (`PluginShellHooks`, `Shell*` types, `bottom_sheet_node`, `accessible_projection`, `from_dsl_value`/`to_dsl_value`/`parse`/`to_string`, `device_view`, `ui_error`), manifest (`create_named_layout`, `NamedLayout`, `WindowLayoutAxisNode`/`Child`), screen-reader (`pick`, `spoken_utilisation`, `entity_spoken_name`, `number`, `table_twin`, `accessible_description`), design-case-surface (`design_case::*`, `NormFieldChoice`, `FieldKind::Quantity`), outliner (`encode_scoped_identity` via `protocol::wire`, same call as picking/selection-mirror) all exist with matching signatures and visibility. Crate layering (foundation, viewport3d, visual-chrome, live-core, panels, live-editor) respected for every `crate::` path used in the new code; each Cargo has the crates it needs.
- Struct literals: `NdpRef` (gained `id`) only in field-meta tests (fixed there); `StructuralScene.owners` in all 9 literals; `ViewLane` (gained `units`, `item`) only via `..self.view.clone()`/Default; `NormSelection.units` in constructor and test; `InspectorOptions` (gained `unit_picker`, `annex_parameters`) only built with `..Default`; `LaneEvent` has no other exhaustive match.
- Match exhaustiveness: `DeviceClass` (3), `CheckStatus` compliance (6, in `check_row_facts`) to scene (4), `ShortcutAction` consumers use catch-alls, `WindowLayoutChild` (2).
- Trait impls: `PluginShellHooks for NormShellHooks` (all methods have defaults, `Send + Sync` unit struct), `Default for LayoutPrefs`/`ViewLane`.
- `#[path]` and `include!/include_str!` targets under app-surface: 0 missing after the fix above. Fixture `retained-command-dispositions/🔣️.json` matches the oracle (5 routes, 75 identities, 5 publication contracts).
- Evaluate command removal: no `Evaluate`/`"evaluate"` remains in norm sources or fixtures.
- `list_delta!`: no `key: id` left in any norm artifact; the `by <Keyer>` arm used by the local markers exists in the macro.
- Source-boundary scan: see item 2 above.

## Not verified / residual risk
- Framework changes of today (shell-hooks module in `manifest-app-surface`, ui-contract `ContainerRole::Sheet`, wgpu shell) are outside this review.
- Builder method names on the ui-contract builders in controls (`display_factor`, `display_unit`, `draft_target`, `try_snap`, `limits`) exist in the framework; their signatures were not re-derived because the file predates today.
- Test-only code (shell-hooks tests, design-case-surface tests, accessibility tests) was only skimmed for API names.
