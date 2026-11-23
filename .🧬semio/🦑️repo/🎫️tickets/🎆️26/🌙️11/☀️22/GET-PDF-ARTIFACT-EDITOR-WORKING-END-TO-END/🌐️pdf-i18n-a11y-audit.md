# PDF 1.7 base editor and viewer: i18n and accessibility audit

Read-only audit. No source was edited. Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/` (`✏️editor` and `👁️viewer`, Rust files excluding tests). Baseline: the XLSX base editor's `xlsx-i18n-fixes.md` (ticket 26/11/20) fixes G5 (layout titles), G6 (unknown-body label), G7 (grid axis labels), G8 (locale-aware numbers), G11 (hidden argument labels).

Paths below are relative to `.../🪆️subsets/🧱️base/`. Short names: `editor/` = `✏️editor/`, `viewer/` = `👁️viewer/`.

## Summary of severity

- High: 5 items (numbers, viewer English output, English layout titles, Fault messages, missing canvas name).
- Medium: 6 items (unknown-body fallback, bool-based locale, `if De else En` fallbacks, heading semantics, cancellation text, German help text parity).
- Low: 4 items (casing, duplicated helpers, raw key labels, PDF technical keys).

## Findings

### High

H1. Locale-unaware decimal numbers (German users cannot enter decimals the German way).
- `editor/🖼️page/🔎️inspector/🦀️.rs:380-387` `shown_number` formats with `{value:.4}` (always `.`). Used for every number input value (lines 21, 25-70, 156-164, 231-234).
- `editor/🖼️page/🔎️inspector/🦀️.rs:247` box quad values use `f64::to_string()` (`.` decimal, joined by spaces).
- `editor/🖼️page/🧰️arguments/🦀️.rs:48,52` and `editor/🖼️page/✍️authoring/🦀️.rs:48,219`, `editor/🖼️page/🎨️resource-edits/🦀️.rs:43,283`, `editor/🖼️page/🔖️annotation-edits/🦀️.rs:17`, `editor/🖼️page/📄️document-edits/🦀️.rs:42` parse with `parse::<f64>()` / `parse::<u32>()`, so a German `1,5` fails as a Fault.
- Fix: follow XLSX G8. Add a `pdf_decimal_separator(locale)` / `format_pdf_number(value, locale)` / `parse_pdf_number(text, locale)` trio in `editor/🖼️page/` and pass the session `view_state.locale` through `field_input`, `shown_number`, `text_arg`/`number_arg`. Keep the canonical stored value (dot) unchanged, and keep the invariant `None` path for the agent/tool path. Add a German decimal-comma test to `editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`.

H2. Viewer page summary is English-only and not localized.
- `viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs:33` ", CropBox [...]"
- `viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs:34` "(no extracted or authored text)"
- `viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs:35` "Page {} -- MediaBox [...]"
- Cause: `page_summary` (line 31) and `render` (line 39) take no locale. `viewer/🦀️.rs:87` `render` has `_view_state` unused.
- Fix: pass `view_state.locale` into `main::render` and `page_summary`. Use a catalog (see H3) with native en/de for "Page", "MediaBox", "CropBox", "no extracted or authored text". Keep the `page.text()` payload as-is (document data).

H3. English-only window and layout titles (the XLSX G5 class).
- `editor/🖼️page/🪟️windows/🦀️.rs:180` `stack(0.12, THUMBNAILS, "Pages")`, `"Document"`, `"Text layer"`, `"Object"`, `"Details"` (hardcoded `title: Some(...)`).
- `editor/🎭️modes/✏️edit/🦀️.rs:26` `title: Some("Pages".into())`.
- `viewer/🎭️modes/👁️view/🦀️.rs:26` `title: Some("Pages".into())`.
- Fix: as XLSX G5, set `title: None` so the dock falls back to the localized window-kind label (`LocalizedLabel::native` on each window definition, already present for thumbnails, text layer, inspector, main). For `snapshot_details` (`"Details"` at line 180) note the same residual as XLSX: it is a shared contract, so track it there rather than in this ticket.

H4. Fault messages shown to users are English-only and not localized (systemic).
- Examples: `editor/🖼️page/✍️authoring/🦀️.rs:37` "the {phase} step was cancelled"; `editor/🖼️page/🧰️arguments/🦀️.rs:48` "the action requires argument '{key}'"; `editor/🖼️page/🔖️annotation-edits/🦀️.rs:8,17,22,50,78,88,96,100,131,142,185,212,367-398,405,447`; `editor/🖼️page/📄️document-edits/🦀️.rs:7,42,44,48,58,67,110`; `editor/🖼️page/🎨️resource-edits/🦀️.rs:8-291` (about 40 messages); `editor/🖼️page/📝️page-text/🦀️.rs:35,46,54,103`; `editor/🖼️page/🖱️object-edits/🦀️.rs:29,39,130,143,152,153`; `editor/📄️document/🦀️.rs:13,22,52,126,138,154,165`; `editor/📄️document/🖼️image-authoring/📥️admission.rs:22` "Invalid image placement coordinates"; `editor/📄️document/🖼️image-authoring/📂️picker-work.rs:42,57`; `editor/🦀️.rs:255` "PNG import requires its retained interactive job".
- Fix: give each user-facing Fault a stable `FaultCode` (already there: `stdio.pdf.page-edit`, `stdio.pdf.unhandled-action`) and resolve the message from a bilingual catalog keyed by that code, or pass locale into `fault()` and use `LocalizedLabel`-style pairs. Internal invariants (`ValueError::invariant` in `editor/💾️native-save/`) are developer-facing; leave them English but confirm they never reach the UI.
- Verify: how the host renders `Fault.message` (not checked in this audit).

H5. The canvas surface has no accessible name.
- `editor/🖼️page/🪟️windows/🦀️.rs:141` and `:145`, `editor/🖼️page/🪟️windows/🦀️.rs:285` `scene_surface(BODY_KEY, SurfaceKind::Canvas2d, ...)` pass only an id. The framework `scene_surface` (`🧰️framework/.../🔌️plugin/🪟️window/🦀️.rs:223`) takes no label.
- Fix: check whether the framework surface can carry a localized label/description. If yes, pass a native en/de "Page canvas" / "Seitenfläche" from `window_definition`. If not, add it to the framework and keep the text-layer window (`editor/🖼️page/♿️text-layer/`) as the documented accessible alternative and say so in its docs. Not confirmed by running the UI.

### Medium

M1. Silent English default via `if locale == De { de } else { en }`.
- `editor/🖼️page/🔎️inspector/🦀️.rs:97` (`label` closure), `:200` (add-entry label), `:207` (heading closure).
- `editor/🖼️page/🪟️windows/🦀️.rs:221` (`label` closure).
- Fix: replace with exhaustive `match locale` or a single catalog resolver (`app_labels!`, as XLSX does at `📕️xlsx/.../✏️editor/🦀️.rs:54-69`). A future third locale would otherwise fall back to English silently. This also removes the repeated closure pattern across files (see L2).

M2. Bool-based locale in a shared control.
- `editor/📄️document/🖼️image-authoring/🎛️replace-button.rs:9` `image_replacement_button(..., german: bool)`, label at `:17`; call site `editor/🖼️page/🪟️windows/🦀️.rs:267` `locale == Locale::De`.
- Fix: take `Locale` and `match` it. Remove the `bool`.

M3. Viewer and editor "Unknown body" fallback is English-only (XLSX G6).
- `viewer/🦀️.rs:90`, `editor/🦀️.rs:272`, and the pdf14/pdf17 kits `editor/🧰️kit/🧾️pdf14/🦀️.rs:210`, `editor/🧰️kit/🧾️pdf17/🦀️.rs:203`.
- All use `semio_ui_locale::Label::data(format!("Unknown body: {body_key}"))`.
- Fix: exactly XLSX G6. Declare `app_labels!` with `unknown_body: native_en "Unknown body: {body}", native_de "Unbekannter Bereich: {body}"` in the viewer and editor, and pass `view_state.locale`.

M4. Headings are plain text, so screen readers get no heading role.
- `editor/🖼️page/🪟️windows/🦀️.rs:265` (object kind, e.g. "Image"), `:205` (page heading "Page 1").
- `editor/🖼️page/🔎️inspector/🦀️.rs:190` (entry group heading).
- `editor/👥️presence/...` not affected.
- Fix: use the framework heading primitive if one exists (`ui::heading` or equivalent; not found in this audit, so verify). Otherwise keep the text but group it with an accessible group name.

M5. German help text is not at parity with English.
- `editor/🖼️page/🪟️windows/🦀️.rs:211` DE: "Seitenrahmen als links unten rechts oben eingeben. Leere optionale Rahmen verwenden den Standardwert."
- `editor/🖼️page/🪟️windows/🦀️.rs:212` EN: "Enter box coordinates in left, bottom, right, top order, separated by spaces. Clear an optional box to use its default."
- The German omits "separated by spaces" (the input format) and reads ungrammatically.
- Fix: DE: "Rahmen als Koordinaten in der Reihenfolge links, unten, rechts, oben eingeben, getrennt durch Leerzeichen. Ein leeres optionales Feld verwendet den Standardwert." Same check for `editor/🖼️page/🔎️inspector/🦀️.rs:153-154` (DE lists keys in English, consistent with EN; acceptable but note).

M6. Cancellation message is English-only and user-visible.
- `editor/🖼️page/✍️authoring/🦀️.rs:37` "the {phase} step was cancelled".
- Fix: localized message via catalog; the phase names need en/de as well (`Progress { phase }` values are internal ids, so map them).

### Low

L1. Casing mismatch in a German/English pair.
- `editor/🖼️page/♿️text-layer/🦀️.rs:47` `pick(locale, "untitled", "ohne Titel")`: English lowercase, German capitalized sentence. Fix: "Untitled" / "Ohne Titel" (or keep both as labels, not sentences).

L2. Duplicated localization helpers.
- `pick` (`text-layer/🦀️.rs:33`), `label` (`view/🖼️thumbnails/🦀️.rs:59`), `heading` (`inspector/🦀️.rs:207`), `label` (`inspector/🦀️.rs:97`, `windows/🦀️.rs:221`). Also the duplicated `match (ObjectKind, Locale)` table at `text-layer/🦀️.rs:58-71`.
- Fix: one app-level catalog (`app_labels!`) for the editor and one for the viewer, placed close to each other per the multi-implementation rule.

L3. Raw PDF keys shown as labels.
- `editor/🖼️page/🔎️inspector/🦀️.rs:194` (`entry_input(..., &key, &key, ...)`) uses the raw dictionary key as the accessible label; line 196 shows "{key}: {value}".
- Impact: keys such as `/Lang` are technical names; acceptable, but the label should say what the key is. Fix: keep the key as the name and add a localized description (`describe`) when a key is known.

L4. Technical names are not localized in labels.
- `editor/🖼️page/🔎️inspector/🦀️.rs:153-154` and `:161`, `:162`, `:164`: the German labels keep English key names such as `singlePage` and `useOutlines`. This is consistent with the PDF spec; mention only so a reviewer does not flag it as a missing translation.

## Parity check (labels that are present in both languages)

Verified bilingual, no action needed: all `ActionArgDef` and `ActionDefinition` labels in `editor/🖼️page/🪟️windows/🦀️.rs:9-118` (`LocalizedLabel::native`), the inspector field pairs (`label_en`/`label_de`) in `editor/🖼️page/🔎️inspector/🦀️.rs`, the thumbnail "earlier/later" rows, the text-layer rows, the view action table (`editor/🖼️page/🔭️view/🎬️actions/🦀️.rs:35`), the presence captions and describe text (`editor/👥️presence/🧮️board/🦀️.rs:177-181, 277-300`), and `editor/💾️native-save/🏭️factory.rs:25,46`. Button labels for add/merge/replace/extract are bilingual (`inspector/🦀️.rs:102-119`).

## Accessibility notes (controls)

- Buttons, inputs and tree items all receive a label (`try_label`, `ui::button(Label)`). No unlabeled icon-only control was found in the base editor or viewer.
- Keyboard alternatives exist for pointer-only actions: the text layer (`♿️text-layer`) lists every object as a button, and the inspector exposes X/Y/width/height as numeric inputs (move and resize without the canvas). Thumbnails are draggable (`thumbnails/🦀️.rs:103`) but also offer "move earlier/later" child rows. Those children are hidden until the row is expanded (`default_open(false)`, line 103). Verify that the tree item can be expanded with the keyboard.
- Canvas pointer actions (`windows/🦀️.rs:33-35`) are input events without a keyboard equivalent; covered by the inspector fields, so not a blocker.

## Comparison with XLSX base editor

XLSX already has: an `app_labels!` catalog (`XlsxEditorLabels`, `📕️xlsx/.../✏️editor/🦀️.rs:54-69`), a `None` title on layouts, a localized unknown-body fallback, and locale-aware numbers (G8). The PDF base editor has none of these. Its label handling is inline `match locale` / `if De` pairs (see M1, L2), which is the pattern XLSX moved away from.

## Not verified (could not be checked in this read-only pass)

- How the host renders `Fault.message` and whether it is localized anywhere downstream (H4).
- Whether the framework surface accepts a label for the canvas (H5) and whether a heading primitive exists (M4).
- Whether `ViewModel.locale` can default to English when the user has no preference (framework, not inspected). The base code itself has no `"en"` default: `Locale` is an exhaustive two-variant enum everywhere it is matched.
- Runtime behaviour: keyboard expansion of the thumbnail tree, screen-reader output, and the German rendering of long labels in a real window.
- Tests: no existing test asserts German parity for layout titles or the viewer page summary.

## Test-coverage gap

`editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs:491-497` covers the text layer in both languages. There is no German decimal test, no viewer German test for the page summary, and no unknown-body test in either language. Add these with the fixes above.
