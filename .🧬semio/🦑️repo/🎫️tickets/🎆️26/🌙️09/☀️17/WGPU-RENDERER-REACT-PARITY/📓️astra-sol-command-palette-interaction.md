# Astra Sol Command Palette Interaction

## Scope

This packet closes the six interaction and accessibility gaps from `📓️terra-command-palette-final-audit.md` without changing the browser probe or adding a runtime dependency.

## Owned contract

The neutral `ShellSearch` schema and fixture now define:

- an editable command-palette combobox that owns a listbox and a stable active descendant;
- a non-editable retained `Select` combobox, which remains a button;
- activation on an uncancelled primary release over the same row;
- dismissal that preserves the query and valid selection that clears it;
- a dialog, accessible Close button, listbox, group, option, and polite empty-status hierarchy;
- NFKD normalization, mark removal, lowercase folding, a 96-scalar fuzzy limit, and EN/DE/fullwidth/ligature cases.

The React component oracle checks the same owned relationships and runs JavaScript's native `normalize("NFKD")` as the independent oracle.

## Production changes

### Browser keyboard and accessibility mirror

The mirror projects an editable palette combobox as an `input[role=combobox]`, while retained `Select` remains a `button[role=combobox]`. `aria-controls` and `aria-activedescendant` resolve through stable, per-window DOM IDs. Palette option elements publish their canonical command item key.

The root keyboard bridge forwards Escape, navigation keys, and Enter from the editable combobox to the retained Shell. Printable text, IME composition, and Tab stay native. Ordinary text fields keep their existing native editing and undo behavior.

### Shell interaction

The retained palette plan now includes the full dialog background, physical Close button, canonical item ID for every row, and list geometry. The background is registered before children, so child hits win while group, status, and padding areas remain modal dialog hits.

A primary row press only arms the exact physical control. Activation occurs on release over that same row. Moving away, an outside dismissal, Escape, and Close cancel the release owner and preserve the query. A successful valid selection uses the existing Search/Find action funnel and then clears the query.

The accessibility projection is modal while a palette is open. It publishes one hierarchy:

- dialog;
- Close button;
- editable combobox with list ownership and active descendant;
- listbox;
- groups;
- stable canonical options;
- polite empty-result status.

Accessibility option activation resolves the canonical item ID back to the current filtered result, rather than depending on a transient physical row ordinal.

### Shared normalization

`normalize_nfkd_text` is owned by the existing UI Select implementation and re-exported by the WGPU UI target. It covers the shipped neutral repertoire: Latin-1 canonical decompositions, combining-mark ranges, fullwidth ASCII, and Unicode presentation ligatures. Shell fuzzy matching and Select typeahead use this one implementation. The contract is deliberately bounded to the schema's shipped repertoire; it is not presented as a general-purpose Unicode normalization library.

## Tests and receipts

Test-first laws cover:

- query persistence across chord dismissal and reopen;
- the neutral NFKD rows and precomposed/decomposed German equivalence;
- editable combobox/listbox/active-descendant projection;
- physical Find activation on release;
- cancelled release preserving query and selection;
- dialog interior absorption and outside dismissal;
- accessible Close activation;
- dialog/list/group/option/status hierarchy and stable canonical IDs.

Executed locally through Bun and Nx:

- `ShellSearch` React oracle: **6/6 passed**.
- browser keyboard and accessibility mirror suite: **34/34 passed**.
- `rustfmt --check` parsed the changed Rust boundary successfully; it reported repository formatting differences, so this is a parse receipt rather than a native compile receipt.

Root owns the native and browser runtime gates. The focused native filter is `shell_shortcuts_palette_tests`. No fresh WGPU browser runtime result is claimed in this report.

## Files

- `ShellSearch/🧬️schema/🔣️.json`
- `ShellSearch/🧪️fixtures/🔣️.json`
- `ShellSearch/🧪️tests/🧩️component/🟦️.tsx`
- React test config registration
- `Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs`
- UI accessibility contract in Rust and TypeScript
- WGPU accessibility mirror and browser input wire
- WGPU Select normalization owner and WGPU target re-export
- browser keyboard-scope fixture, schema, and tests
