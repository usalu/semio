# Forms React Parity — Verification Log

## Automated tests

| Suite          | Command                                       | Result                                                                                                                                                                                                                                    |
| -------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| forms domain   | `cargo test -p forms`                         | 5 passed (incl. new `update_form_op_sets_and_reverts_title`)                                                                                                                                                                              |
| forms plugin   | `cargo test -p forms-plugin`                  | 25 passed (builder cards, selection, wizard gating + inline errors, slider unit + number bounds, url image node, patchStep/removeStep/moveStep/updateForm, dropQuestionKind, drop command on document tree, always-editable kind editors) |
| react renderer | `bun x vitest run` (framework/renderer/react) | 64 passed (new: field description/required/error, slider unit readout, input min/max/accept, disabled buttons, selectable card ring, image node, tree drop dispatch)                                                                      |
| workspace      | `cargo check --workspace`                     | forms/framework/react-relevant crates green; remaining errors are another dev's in-flight `ui/wgpu` module extraction (unrelated)                                                                                                         |

## WASM + dev host

- Rebuilt plugins via `framework/product/os/dev`: `bun ./📜️script.ts plugin forms` (full catalog, wasm32-wasip2, no errors).
- React dev host launched via new launch.json entry `forms-react-dev` (`bun run dev:forms`, FORMS_PLAY_PORT=6558, SEMIO_RENDERER=react).

## Browser verification (http://localhost:6558)

- **Try wizard**: form title + step title emphasized, `Step 1 / 2`, step description; required markers on Component Name/Material; multi renders as `#` chip row; boolean check-icon Yes toggle; date/color inputs; Back present and `disabled=true` on step 1; Next enabled with defaults.
- **Validation**: clearing required Component Name → Next `disabled=true` + inline `field-error` "Component Name is required" attached to the field (verified via DOM query).
- **Step 2**: Geometry step renders note, number, slider, vector steppers, and the buildingComponent extension question resolved through external slots (HEIGHT/RADIUS/SIDES module sliders visible). A later hang on step 2 is the known pre-existing procedural-3D preview hang (open ticket FIX-PROCEDURAL-3D-SPHERE-CUT-TORUS-INFINITE-HANG), not forms.
- **Blueprint builder**: form-title editor ("Building Component"), 14 question cards; clicking a card selects it (ring + inspector + document tree sync); editing the card Label input propagated live to the Try window ("Component Title\*") and inspector; `mod+z` undo reverted it.
- **Inspector completeness**: selected slider question exposes Label, Kind select, Id, Required, Description, Min, Max, Step, Default, Unit — all editable even when previously unset.
- **Drag-and-drop**: synthetic HTML5 drop of `application/x-semio-forms-question-kind` `{"kind":"slider"}` onto the Identity step dropzone inserted a new question (cards 14 → 15) that came back selected (`forms-blueprint.card.q-1`); document tree and inspector reflect it. Tree-level drop (`dropCommand` on the document tree) is unit-tested since catalogue/document share one tabbed panel.
- Console: fixed a React controlled/uncontrolled warning by keying stack/section children by node id instead of index (React was reusing a text input DOM node for the file input across step changes).

## Known out-of-scope leftovers

- `exportFixture` remains a no-op: premigration was itself only `console.log("[DEBUG] forms export", …)` and the plugin host has no logging/effect channel yet.
- wgpu renderer support for the new schema fields (field description/error, selectable/droppable stacks, image node) is fallback-level only — separate wgpu parity ticket.
- Pre-existing wgpu RasterScene paint code still targets the pre-overhaul raster scene (owned by the raster migration).
