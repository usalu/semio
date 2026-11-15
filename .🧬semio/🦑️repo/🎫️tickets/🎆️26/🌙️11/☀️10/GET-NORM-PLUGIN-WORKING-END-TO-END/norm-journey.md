# Norm browser journey (s app)

## What was added

Owner: `✏️s/🔌️plugins/📕️norm/🧪️tests/🌐️browser/` (same layout as the remodel journey).

- `🧬️schema/🔣️.json` and `🧫️fixtures/🔣️.json`: language-agnostic journey definition (schema `s.norm.browser-journey.v1`): example id, five regions with en and de names (equal to the Rust focus-order catalogue), viewer regions, member row suffix, edit (field names en/de, step key, commit key), utilisation pattern, locale chrome ids, six checks with phase.
- `🧭️journey/🟦️.ts`: `inspectNormEditorV1` (side A) and `inspectNormViewerV1` (side B), written against the shared two-human `OpenedUiV1` port.
- `📋️arguments/🟦️.ts`: strict argument validation (hub, serve, kinds, locale, out under `🗑️generated`).
- `🟦️.ts`: `runNormBrowserAcceptanceV1` calling `runTwoHumanCli` with `{ required, run, viewer }`.
- `🧪️tests/🟦️.ts`: bun test witness (fixture against schema through `semioSchemaAjvV1`, i.e. Ajv; names equal the Rust catalogue text; 5 editor + 1 viewer checks; argument validation).
- Repo targets in `📦️packages/🟦️typescript/📋️project.json` and routes in `📜️script.ts`: `@semio-tech/norm-js:test-browser-journey` (the witness) and `@semio-tech/norm-js:browser-acceptance` (the live journey, `forwardAllArgs`, cache off, like remodel).

## Steps and checks

Editor phase, side A: `editor-windows` (five regions present, each with its accessible name in the document locale), `select-member` (loads example `heb240-compliant` from the picker if the outline lacks the member, clicks the member tree item, expects `aria-selected=true`), `edit-value` (focuses the Length field in Properties by `aria-label`, ArrowUp x5, Enter, value changes), `checks-utilisation-updates` (percent strings of the checks window differ after the edit, within 30 s), `language-de` (en to de through the settings combobox, all five region names relabelled and `html lang` de, then back to en and the run locale).
Viewer phase, side B: `viewer-windows` (exactly 3D model, checks and diagrams visible; outline and properties absent).

Selection uses roles and ARIA (`treeitem`, `aria-selected`, `combobox`, `option`, `aria-label`) and the framework surface attribute `data-surface-window-kind-id` (suffix match on the region role); no CSS class selectors.

## Usage

```
repo run @semio-tech/norm-js:browser-acceptance -- --hub <hub-url> --serve http://127.0.0.1:6070/ --kinds <kind-id> --locale en --out <ticket>/🗑️generated/norm-journey
```

`--kinds` is the creatable kind id of the EN 1993 editor in the hub catalogue; it was not discoverable statically (the catalog excludes norm today, see chain-audit.md), so it must be taken from the Space app picker once norm is published.

## Run status

- Witness `test-browser-journey`: started via `repo run` but never executed; the capacity gate held it for over 3 minutes (free memory below the floor, 5 jobs ahead), so I stopped it. Not run, not verified. Type check and execution of the new files are therefore still open.
- Live journey: not run. `http://127.0.0.1:6070/` was not serving (curl exit 7) and, per the audit, the s catalog does not yet publish norm, so there is no creatable EN 1993 kind to open.

## Unverified assumptions to confirm on first live run

1. Window kind ids end in `-outliner|viewport|properties|checks|diagrams` or `.outliner...` (focus-order roles; viewer ids `norm.en1993.view.*` fit the dot form).
2. Outline rows are `role=treeitem` with `data-ui-node-key` ending in `member-b1` (from `norm-outliner.<path>` ids).
3. The Properties length field has an `aria-label` containing "Length"/"Länge" and exposes `value` or `aria-valuenow`.
4. Regions expose their accessible name as `aria-label` equal to the Rust names. If the app names them differently, fix the app label (the catalogue is authoritative) rather than the fixture.
5. The README says the viewer has three windows, while `single_window_layout` documentation in `🏷️ids-chrome-and-labels` says every norm viewer has exactly one window (report table). The journey asserts three; if the live viewer shows one, that is an app/doc inconsistency to fix.
