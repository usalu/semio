# PDF editor accessibility, round 6

Closes open-items rows 21 (`a11y-heading`) and 22 (`a11y-runtime`, static half), the cheap part of row 52 and the verification half of row 49. Sources: `🌐️pdf-i18n-a11y-audit.md`, `🌐️pdf-i18n-a11y-fixes.md`, `📋️open-items-round-6.md`.

## 1. Framework heading primitive (row 21)

Decision: a heading is `Component::Text` with an optional `heading_level` (wire `headingLevel`, `1..=6`), not a new component. Every text pipeline (packed text leaves, text pages, census, selection menus, retirement, compare, copy) keeps working for headings, and an exhaustive new variant would have touched about twenty more match sites for no gain. `None` is a paragraph.

Path followed (guest to host), each layer changed once:

| Layer | Change |
| --- | --- |
| Contract props (Rust) | `TextProps.heading_level: Option<u16>`, ordinal 3 in `ui_component_field_catalog`; `HEADING_LEVEL_MIN/MAX`; `TextProps::heading()` clamps (`🧬️contract/🧩️component/🎨️basic-widget-props`, `🧾️fields`, `🦀️.rs`) |
| Kernel patch wire (Rust) | kernel JSON `TextProps.heading_level` (`🎠️kernel/📤️return/🔤️json/🖼️ui`) |
| Guest builder (Rust) | `ui::heading(label, level)`: emphasized text, level clamped to 1..6 (`🏗️builder/📚️stack-container-text-and-button-builders`, re-exported by `🏗️builder/🦀️.rs`, reaches `plugin_app_close_prelude` through the existing glob) |
| Role projection (Rust) | `accessibility_role` answers `heading`; `accessibility_projection_node.level` carries the clamped level (`🧬️contract/♿️accessibility`) |
| Schema text and generated TS | `TextProps` TypeScript text gains `headingLevel?: number \| null` (`🧬️schema/🟦️projection/...schema-rows-slider-props-to-trigger`, and the ignored generated copy `🛂️manifest/🤖️generated/📜️ui-contract`) |
| Typed wire decoder (TS) | `text` reads `headingLevel` (integer 1..6, anything else throws) and omits it when absent (`🧵️retained/📦️wire/🧾️typed`) |
| TS role projection | `uiAccessibilityRoleV1` answers `heading`; `uiHeadingLevelV1`; projection `level` (`🖼️presentation/♿️accessibility`) |
| React host | `TextView` renders `<h1>` to `<h6>` (real heading element, `font-semibold`); plain text stays `<p>` (`🗣️Interpreter/🔤️display-views`) |
| wgpu accessibility mirror | `heading` role becomes `<hN>` with its label as text (`🧊️wgpu/♿️accessibility-mirror`) |

Language-neutral fixtures (one oracle, Rust and TS):
- `♿️accessibility-projection.json` gains `headings` (levels 1 and 3, 0 clamps to 1, 9 clamps to 6, a levelless text stays a paragraph). Answered by Rust `heading_text_projects_the_heading_role_and_its_clamped_level` and the TS twin `accessibilityProjectionSelfTests` (399 checks).
- Conformance corpus case `🧩️component/🔠️heading` (snapshot and expectation, catalog entry); the corpus counts moved 78 to 79 (all cases) and 56 to 57 (wgpu accept cases) in the Rust harness, the wgpu corpus law, the TS corpus self-test and the React `unknown-component-placeholder` law.
- Ajv/typed decoder case: `carries a text heading level exactly and refuses a level outside 1 to 6` (levels 0, 7, 1.5, "2" throw; no level stays absent).
- React: `🔠️ heading text` renders h2 and h6, clamps 9 to h6, keeps `<p>` for plain text (`🪪️container-node-ids`).
- Rust builder law: `heading_builder_clamps_the_level_and_emphasizes` (JSON carries `headingLevel` only for headings).

All 20-odd `TextProps { .. }` literals in Rust (tests, fixtures, staged-form control, panel records) gained `heading_level: None`.

### Use in the PDF editor
`ui::heading` replaces the emphasized text titles (PDF 1.7 base editor, which the 1.7 subsets and the 1.4 editors share through the kits): object kind title in the Object pane (level 2, two sites in `🪟️windows`), key-value group titles (`entry_form`, level 3, `🔎️inspector`). The 1.4 editor has no emphasized title of its own (only `page-dimensions` plain text); no 1.4 change needed.

## 2. Keyboard and screen reader (row 22)

Finding from the code: the framework tree already implements the WAI-ARIA tree pattern (`🌳️Tree/⌨️roving`): roving tabindex, `ArrowRight` expands a collapsed row, then moves to its first child, `ArrowLeft` collapses or moves to the parent, `ArrowUp/Down/Home/End`, `Enter` activates. So the move rows are keyboard operable without new key handling. What was missing was the proof and an unambiguous name.

- Names: move rows were "Move page earlier / later" with no page, so a screen reader reading the child alone could not tell which page moves. They are now `move_label(locale, page, later)`: "Move page 3 earlier" / "Seite 3 nach hinten verschieben" (one-based, both languages, exhaustive match; `🔭️view/🖼️thumbnails`).
- Native law `thumbnail_move_rows_are_collapsed_named_and_keyboard_reachable_children`: every page row is a collapsed (`default_open(false)`), activatable (own target), draggable tree item; the first page holds only "later", the last only "earlier", middle pages both; each move row carries its own activation target and its page number, in en and de.
- Native law `inspector_titles_are_headings_with_levels_and_never_emphasized_text`: every Object pane variant (each object, empty selection; en and de) exposes headings, the object kind at level 2 and every key-value group at level 3, no other heading key.
- `pdf-pane-journey.ts` (type-checked with `tsc --noEmit`, not run): new step `a11y-en` / `a11y-de` (added to the default step list), all keyboard only:
  1. `getByRole("heading")` collects heading elements with level and name; asserts at least one level-2 heading, every heading named and 1..6, `[data-ui-node-key="pdf-object-kind"]` is `H2`, and `page.ariaSnapshot()` contains `heading "..." [level=N]` (the removed `page.accessibility` API is not used).
  2. Page strip: the first page row is a `treeitem` that starts `aria-expanded=false`; focus it, press `ArrowRight`; it becomes expanded, the child `treeitem` "Move page 1 later" / "Seite 1 nach hinten verschieben" appears, the strip `ariaSnapshot()` shows `treeitem "Page 1 / 2 ..." [expanded]` and the named child; the focused row has `:focus-visible` and a visible outline or box-shadow.
  3. `ArrowDown` focuses the move row (name and focus indicator asserted), `Enter` commits it with no pointer; the step waits for the row sizes or the canvas to change.
  4. The second (last) page offers only "Move page 2 earlier" and `ArrowLeft` collapses its row again.
  5. Console verdict as in the other steps.

## 3. Minor rows

- Row 52, `stdio.pdf.1.4.page-edit`: the invalid (digit segment) code is gone. The 1.4 canvas refusal now goes through the coded `refuse(PdfFault::PageEdit, ...)` (localized notice); the 1.4 page text drafts file (included by several crates through `#[path]`, no common dependency on the fault catalog) raises the valid existing code `stdio.pdf.page-edit`, which the kit's `fault_notices` already localizes.
- Row 52 residuals left on purpose: the Details window title (shared contract) and the missing number steppers (design, accessibility name and value remain).
- Row 49: the viewer German and English summary and catalog tests are part of the native run below.
- Row 51 (about 180 generic fault sites) is not cheap or local and was not touched.

## Tests and results

All native runs went through `native-cargo.sh` (gated, 3 jobs); logs `🗑️generated/a11y-*.log`.

| Suite | Result |
| --- | --- |
| `semio-s-artifact-stdio-pdf-page-editor-tests --lib` (full) | 71 passed, 0 failed, including the two new laws (`thumbnail_move_rows_are_collapsed_named_and_keyboard_reachable_children`, `inspector_titles_are_headings_with_levels_and_never_emphasized_text`) |
| `semio-ui-contract-conformance --lib` (filtered) | 21 passed: accessibility projection incl. `heading_text_projects_the_heading_role_and_its_clamped_level`, `heading_builder_clamps_the_level_and_emphasizes`, typed descendants |
| `semio-ui-contract-builder --lib` (filtered) | 12 passed |
| `semio-ui-contract-validation --lib` (component_integration, conformance) | 22 passed, including the 79-case corpus pairing law |
| Interpreter vitest (`Interpreter/🟦️.tsx`, in-source, 208 tests) | 208 of 208 passed after the corpus pin moved 78 to 79 (includes `🔠️ heading text` and the corpus `heading` case) |
| Typed wire vitest (`wire/🧾️typed/🟦️.ts`) | 6 passed, including the heading level case |
| `bun accessibility-projection` twin | `checks=399` |
| conformance corpus self-test | `cases 79` |
| `pdf-pane-journey.ts` | `tsc --noEmit` clean (bun types, DOM and DOM.Iterable); not run |

Two stale pins unrelated to this change were fixed because they blocked the validation crate: `🛡️limits/🧪️tests/🧬️conformance` called the removed `Label::as_str` (`label.0.as_str()`), and `🛡️limits/🧪️tests/🧩️components` pinned 28 typed fixture rows where the committed fixture has 29.

Not verified in this pass:
- Row 49, the viewer German and English summary and catalog tests: the cargo run `a11y-viewer3` (`semio-s-artifact-stdio-pdf-inferences --lib`, filters `page_summary_speaks`, `viewer_labels_are_complete`, `render_lists_one_line_per_page`) was still queued behind other agents' gated runs when this report was written; read `🗑️generated/a11y-viewer3.log` and `.done`. (`semio-s-artifact-stdio-pdf` and `...-pdf-snapshot-value-tests` do not contain those tests: 0 matched.)
- `kernel`, `ui runtime`, wgpu shell `panel-records` and plugin reactor test crates received the mechanical `heading_level: None` literal edit but were not compiled (only crates in the page-editor and contract graphs were); a plain `check --tests` on them is the cheap follow-up.

## Files

Framework:
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs`, `🧾️fields/🦀️.rs`, `🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/📚️stack-container-text-and-button-builders/🦀️.rs`, `🏗️builder/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs`, `🖼️presentation/♿️accessibility/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🟦️projection/🎨️schema-rows-slider-props-to-trigger/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts` and its test
- `🧰️framework/🔨️modules/🎠️kernel/📤️return/🔤️json/🖼️ui/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🔤️display-views/🟦️.tsx` and its tests (`🪪️container-node-ids`, `🧪️unknown-component-placeholder`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`
- Fixtures: `🧬️contract/🧪️tests/🧫️fixtures/♿️accessibility-projection.json`, `🧪️conformance/📇️catalog.json`, `🧪️conformance/🧩️component/🔠️heading/*`; tests `🧪️tests/♿️accessibility/🦀️.rs`, `🛡️limits/🧪️tests/🧬️conformance/🦀️.rs`, `🧪️tests/🔬️accessibility-projection/🟦️.ts`, `🧪️tests/🔬️conformance-corpus/🟦️.ts`, `🖱️ui/🧪️tests/🧪️conformance-corpus/🦀️.rs`
- `TextProps` literal updates in Rust tests and fixtures (about 20 files, one added field each).

PDF:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🪟️windows/🦀️.rs`, `🔎️inspector/🦀️.rs`, `🔭️view/🖼️thumbnails/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`
- `.../4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs`, `🔭️canvas/🦀️.rs`

Ticket: `pdf-pane-journey.ts`, this report.

## Needs a wasm rebuild to verify

The browser journey steps `a11y-en` and `a11y-de` (and the new heading elements in every other step) need a rebuilt guest wasm: the heading builder, the thumbnail move labels and the 1.4 refusal code are guest code, and the contract field also changes the retained wire. Rebuild, then run `repo pdf-pane-journey.ts a11y-en a11y-de` (and the full default list for regression). Things only a real browser can confirm: the `:focus-visible` indicator on tree rows (the check requires an outline or box-shadow), that `ArrowRight` on a row with its own activation target both expands and selects without a double toggle, and the screen-reader wording of the nested move rows.
