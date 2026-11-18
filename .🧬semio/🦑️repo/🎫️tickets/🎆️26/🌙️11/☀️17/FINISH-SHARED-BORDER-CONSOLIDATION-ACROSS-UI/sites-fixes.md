# Sites fixes (audit-sites.md)

## Language choice (H8, S1, M1, M2)
No existing ui element fit: the course `Segments` is course-specific (course icons, `overviewCardChipClass`), 🎛️ToggleGroup needs an icon and has no language contract, 📚️I18n is a translation provider. Added one domain-neutral element:
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🈯️LanguageChoice/🟦️.tsx`: `LanguageChoice` (+ `LanguageChoiceOption`, `LanguageChoiceProps`) on ToggleGroup `semantics="radio"` (radiogroup, arrow keys, one tab stop, no press-off, one framed group with shared cell dividers, per-option `lang`, nothing selected until `value` names a language). Exported via package.json `./elements/LanguageChoice`, `📚️component-exports`, the `ui-react/source` barrel.
- Used at: hub `📚️I18n` gate (`AdminLocaleProvider`), semio-tech `🎡️play` `PlayBoot`, mit-bestand `⚛️locale-picker.tsx` (both the landing picker and the navbar switcher; hand-rolled bordered buttons and `aria-pressed` buttons removed, `cn` import dropped; change kept minimal, static build files otherwise untouched).

## Hub admin
- H1 Connections: outer `rounded border p-single` cards removed; one vertical framed `SharedBorderRow` (list) with nested vertical rows per space/document, heading cells, and per connection a nested horizontal row (`data-row-id` kept) with a text cell and the kick `Button` as a direct cell.
- H2/H3 Overview/Observability: new `🌎️hub/.../🧱️elements/📊️StatTiles/🟦️.tsx` (`StatTiles`, rows of `columns` cells, padded with filler cells, one frame); `StatTile` and `Figure` helpers deleted, both pages use it (Overview 3 columns, Observability 2 for the pair and 4 for residency). Trade-off: column counts are fixed (no responsive 2/3/4 grid) because flex dividers cannot cross a wrapping grid.
- H4 Spaces: `border-t` removed from `MembersPanel` (the table's last row border is the single divider).
- H5 Spaces add-member row: `SharedBorderRow` with `Input`, `SelectTrigger`, `Button` as direct children, no gap; the wrapping `<label>` replaced by `aria-label` + placeholder on the Input and `aria-label` on the role trigger.
- H6 space row actions and H7 rebuild/cancel: `SharedBorderRow className="w-fit"` instead of `gap-single` flex rows (used `SharedBorderRow` rather than `ActionGroup`).
- Not changed: invite-create row in the members panel (single ghost button plus token text, not a seam).

## Tests
- New `🈯️LanguageChoice/language-choice.test.tsx` + language-agnostic `language-choice.cases.json` (3 cases + choose / no-repeat behaviour): 14 pass together with SharedBorderRow; react-export-surface, react-package-export, ToggleGroup: 27 pass (`vitest-ui.config.ts`).
- `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🛡️admin/🟦️.tsx`: chooser now asserts radios/radiogroup frame/none preselected; new Connections single-owner assertions; Spaces members panel has no border, add-row is an owner with exactly `input-root, select-trigger, button-group` children and no label/gap, space actions are an owner; new "Stat tiles" test (Overview) and Observability tile groups assertions (no framed descendant that is not a cell). Ticket-local config `vitest-hub-admin.config.ts`: 22 of 23 pass; the one failure (`🧪️command-routing`, Nx test-level env) is unrelated and environmental.
- Not run: mit-bestand and semio-tech builds/tests (type-checked only by reading; both only swap an element for the exported `LanguageChoice`); no browser visual check.
