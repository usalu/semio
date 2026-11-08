# 1107 panede: pane smoke law runs the whole cycle in en and de

## Locale finding
The locale is not fixed at fixture construction. It rides on each dispatch as `ActionMeta.view_state` (a `ViewModel`), and on each render as the `ViewModel` argument. The law used `meta("local")` (no view state), so apply, undo and redo ran with no language at all. A closed app cannot be reused, so each language still needs its own fresh fixture; the panes loop over it.

## Law change (artifact-app-laws, EDITED, not yet compiled to completion)
- `pane-smoke-law/🦀️.rs`: `assert_pane_smoke_by_example` takes an explicit `&PaneSmokeLanguage { view, body_key, catalog }`. Every dispatch (load-example, apply, undo, redo, second undo) carries `language.view`. After load and after redo the pane body is rendered under that view and must contain every catalog string of that language (comparison against the pane's own label table, never English literals). `PANE_SMOKE_LOCALES = [En, De]`, no default.
- Removed the unused `assert_pane_smoke` (text-load variant, a second copy of the cycle, zero callers).
- `paired-apps-history-and-convergence-laws`: new `settle_history_verb_as(.., &ActionMeta, ..)`; `settle_history_verb` delegates with `meta("local")`.
- Re-exports in `artifact-app-laws/🦀️.rs` updated.

## Callers (all loop `PANE_SMOKE_LOCALES`, fresh fixture per language)
generator (2 files: procedural pane-smoke, composition copy; body catalogue, `labels.widgets`), koordinator (cad `cad.play.artifact`, `pane_shape`+`nodes`), verfolgen (gis `gis2d.play.artifact`, `layer_water`+`layer_roads`), statik (fem3d artifact, `nodes`+`elements`), bearbeiten (process3d artifact, `stock`+`steps`), aussuchen (sourcing pool, `restock`+`col_name`; removed the `[DEBUG]` probe test), aggregator (own cycle: now `aggregator_cycle(locale)` per language, artifact tree must carry `objects`+`references` labels), energie (own action cycle replaced by the law with typed `SetActiveExample` + `SetStructureField`; panel gained pub `site_label(locale)` as its catalog; mounted-bestest Cargo.toml gained the editor-panels dep; `command_from_action` set-node also accepts the typed `field` key).

## Unverified (guesses to check on first compile/run)
- Chosen catalog labels must appear in the rendered body; a wrong pick fails as a catalog miss, not a law bug.
- Import paths: `gis2d::terminology`, `gis2d::panels::artifact`, `fem3d::terminology`, `fem3d::panels::artifact::BODY_KEY`, `puzzle3d_labels` in the aggregator module scope.
- Energie typed `SetStructureField` round trip through `command_from_action`.
- Render under `for_panel`/plain view for gis, cad, sourcing, generator.

## Panecat fixes (11/07)
- statik: terminology and body key now come from `semio_s_artifact_fem_3d_editor_terminology` and `semio_s_artifact_fem_3d_editor_panels` directly. Both are already regular deps of the fem3d editor package (Cargo.toml lines 60, 63); no Cargo edit needed.
- aggregator: catalog check extracted to `assert_document_tree_in_catalog`, called by the body render check and again after redo.
- Confirmed by reading: generator composition copy has `semio-plugin`, `-terminology`, `-editor-panels`, `semio-ui-locale` in its package (Cargo lines 56, 63, 82, 138), and set_active_example/add_generation/context come through the same include chain as before; sourcing `schema::curated_count` is the exact path HEAD's test used (2 uses); aggregator dispatch builds its meta from `app.window_view(..)`, which clones `self.view`, so `set_label_axes` locale is carried.
- Energie: the Cargo.lock entry for editor-panels is the coordinator's.

## Type-check verdict (panede-2, rc 1)
`check -p semio-s-artifact-sourcing-curation --test aussuchen_pane_smoke --keep-going`: 3 errors, all in `semio-os-store-snapshot-clone` (store4's crate): unresolved `semio_framework_value::retirement::original_vector` and `semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient`. The artifact-app-laws crate and the aussuchen test were not reached, so my edits are neither confirmed nor refuted. Blocked on snapshot-clone FIXED.
