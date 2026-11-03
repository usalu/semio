# WP-06 Typed inspector: status report (A6)

STATUS: implemented, NOT compiled or tested. Blocked by the build environment.

## Blocker
`repo run @semio-s/plugin-norm-app-surface:test` waited about 1 h in the capacity queue, then failed in `build:dependencies`:
`Bun installation owner is a symlink: CLAUDE.md`. An untracked symlink `/home/deck/semio/CLAUDE.md -> AGENTS.md` (created Nov 2 00:23, not by me)
breaks the Bun workspace scan at the repo root, so every `repo` build fails until it is removed. I did not delete it (shared state, not my file).
Coordinator action: remove or git-ignore that symlink, then rerun `@semio-s/plugin-norm-app-surface:test` (check first), then each artifact `-rs:check`.

## Delivered (SURFACE = app-surface)
- `🪟️visual-windows/🧾️properties/🦀️.rs`: `render_inspector`, `render_inspector_value`, `render_inspector_for::<Family>`, `field_view`, `InspectorOptions`. Collections go through the outliner; scalars become typed field rows. Windowed, so the old 32-child cap workaround is unnecessary.
- `🎛️controls/🦀️.rs`: unit table and presets (`UnitDisplay`, `UnitPreset`, `unit_display`), locale number format/parse, `parse_quantity`/`format_quantity_text`, `FieldView` with refusal text; controls: number input (display_factor, precision, limits, step), slider with snaps (when `soft_range`), stepper, checkbox toggle, select (choices and entity refs), text; NA badge plus reset-to-default action; all commit via `setField`.
- `✅️field-feedback/🦀️.rs`: per-field governing check (exact subject path or remedy target), utilisation bar 0-120 % with glyph, one-click `applyRemedy` chip, entity roll-up.
- `🌲️outliner/🦀️.rs`: `render_collection` (rows `id - name . glyph u%`, selection, filter, add/remove via `insertItem`/`removeItem`), `OutlinerState`.
- `📝️document-editors/🦀️.rs`: removed `render_document_editor`, `render_object_editor`, `render_array_editor`, `render_value_editor`, `render_document_json` (kept `inputs_section_id`, `render_summary`). SURFACE unit tests rewritten onto `inspect(...)`.
- All 15 artifacts' `📥️inputs` windows now call `render_inspector_for::<Family>` (en1991 keeps its grouped value via `render_inspector_value`); window test renamed `renders_the_typed_inspector`.
- Mounts in `SURFACE/🦀️.rs`: `outliner`, `properties` added; A7 lines marked done in mounts.md (applied by their owners); one non-actionable line remains.
- Tests: unit tests in `properties/🧪️tests/🔬️unit`, `outliner/🧪️tests/🔬️unit`; `properties/🧪️tests/🥒️.feature`; oracle `🔮️oracles/🐍️.py` (stdlib exact rationals; pint/babel not installed) with golden `🔣️.json`.

## Known gaps / open
- Selection and filter are parameters only (no render-time selection exists in norm apps); requests posted in requests.md to A7/A10/A12. Filter input is drawn only when `filter_action` is set.
- Uses `NormFieldMeta`, own unit table; awaiting A2's `FieldSpec`/`UnitId` (`wp02-contracts.md` never appeared); then replace `field_view` and the table (no dual types). Group/tier/visible_if/handle not yet supported.
- Slider, hard ranges and NDP badges need FieldSpec data to appear in real artifacts; covered by direct `FieldView` tests only partly.
- Likely first-compile fixes: builder API details (`ui::slider` display_unit, `try_label`, `Progress.tone`), `parse_number` simplification, asserted JSON substrings in tests.
- Not done: other tests in artifacts that may assert old JSON text; remaining `render_text_chunks` use only in tests.
