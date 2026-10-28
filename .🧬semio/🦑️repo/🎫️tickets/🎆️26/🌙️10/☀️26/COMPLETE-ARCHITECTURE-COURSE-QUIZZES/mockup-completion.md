# Mockup Implementation and Verification

Implemented optional `item.mockup` for classification, sorting and matching with strict, schema-first Excel and Grasshopper variants. `SheetItem` preserves only the given workspace data, while grading categories, numeric results, familiarity and explanations remain absent from sheet items. Wire version is 9 and the generated reference fingerprint is `a517a07210e9b0ec`.

Excel previews show native semantic tables, row/column headers, selected cells/ranges, workbook tabs, a name box and a highlighted formula bar. Grasshopper previews show native parameter capsules, components, numeric sliders with bounds and thumbs, yellow text panels, plugin labels, ports, SVG wires and indexed data trees. Every wire also has an explicit textual output-to-input relation. Figures use localized labels, theme tokens, visible target borders and keyboard-scrollable regions. Layout/paint containment keeps reference content inside its scroll region at 320px.

The same language-neutral fixture is consumed by TypeScript tests, the independent AJV structural oracle, React tests and the existing registered Rust validation unit tree. Rust test `software_mockup_vectors_agree_with_every_item_projection` deserializes the shared fixture, compares acceptance through `quiz_issues`, projects every task kind at every challenge and checks grading fields are absent. The fixture includes all widget types, malformed language/kind/property cases, unknown references, invalid selection/grid shapes and invalid slider bounds.

## Executed Verification

- `repo tools exec bun --light -- "<ticket>/verify-mockups.ts"`: passed 21 shared acceptance vectors with TypeScript and AJV, 168 projected items across three task kinds/four challenges, 14 localized fixture renders and 390 localized renders of all authored Excel/Grasshopper items. Every authored preview has a visible target.
- `repo tools exec bun --light -- "<ticket>/verify-mockup-types.ts"`: passed strict TypeScript API checks with zero diagnostics in 13 selected files: schema, validation, sheet, both new tests, renderer, three existing item views, architecture curriculum, quiz-runs, both-languages and layered-home tests. Diagnostics in unrelated dependency files are excluded; this does not claim a complete repository typecheck.
- `repo tools exec bun --light -- "<ticket>/mockup-preview.ts"`: passed actual Chromium rendering through installed Microsoft Edge and the repository Playwright library. Eight screenshots show six authored previews plus desktop/narrow compositions. Checked no root overflow at 320px. Inspected desktop and graph screenshots visually. The temporary generated screenshots/HTML remain only for parent visual review and must be deleted with the generated folder before ticket closure.
- `repo tools exec python --light -- -X utf8 "🧰️framework/🛍️products/📚️course/🧪️tests/🤝️wire-version/🐍️.py"`: generated the wire reference pair; the final pair was written to the fixture by calling that reference function from the ticket input script.
- Read-only `repo tools exec git --light -- grep` confirmed course item Rust construction sites and no additional course item literals outside the product. Unrelated BIM `ClassificationItem` types were left unchanged.
- Updated the server's valid `course-root-query-envelope` vector and its AJV constant to version 9; intentional rejection vectors were preserved.

## Execution Limits

`repo show project @semio-tech/course --graph-policy=current --json --lang=en` and `repo cargo --store=shared --configuration=prototype -- test -p semio-framework-course --lib software_mockup_vectors -- --nocapture` both failed during source admission, before graph inspection or Rust compilation, with `ENOENT` on `C:\git\semio\.cache\b\l\fixed.dir` from `retainFixedUnpublishedBirthV1`. No infrastructure was modified. The parent architecture test target had the same pre-existing admission failure. No Rust, full Vitest or full-app E2E success is claimed.

## Product Files Created or Updated

All paths below are relative to `🧰️framework/🛍️products/📚️course/` unless explicitly stated otherwise.

- `README.md`
- `🦀️.rs`
- `🧬️schema/🔣️.json`
- `🧬️schema/🟦️.ts`
- `🧬️schema/🦀️.rs`
- `🧬️schema/📚️quiz-documents/🦀️.rs`
- `🧬️schema/🃏️sheets-and-results/🦀️.rs`
- `🧬️schema/🖥️mockups/🦀️.rs` (created)
- `🔨️modules/✅️validation/🟦️.ts`
- `🔨️modules/✅️validation/🦀️.rs`
- `🔨️modules/✅️validation/🖥️mockups/🦀️.rs` (created)
- `🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/🃏️sheet/🟦️.ts`
- `🔨️modules/🃏️sheet/🦀️.rs`
- `🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs` (optional field literals)
- `🔨️modules/📏️scoring/🧪️tests/🔬️unit/🦀️.rs` (optional field literals)
- `🧪️tests/🧷️companion-support/🃏️sheet/🦀️.rs` (optional field literals)
- `🧪️tests/🎚️config/🟦️.ts`
- `🧪️tests/🖥️mockups/🟦️.ts` (created)
- `🧪️tests/🖥️mockups/🟦️.tsx` (created)
- `🧫️fixtures/🖥️mockups/🔣️.json` (created)
- `🧫️fixtures/🤝️wire-version/🔣️.json`
- `🎯️targets/⚛️react/🎨️.css`
- `🎯️targets/⚛️react/🎨️styles/🟦️.ts`
- `🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css` (created)
- `🎯️targets/⚛️react/🔨️modules/🖥️mockup/🟦️.tsx` (created)
- `🎯️targets/⚛️react/🔨️modules/🗂️classification/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/↕️sorting/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/🃏️matching/🟦️.tsx`
- `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/🖥️server/🧪️tests/🧫️fixtures/🔌️wire/🔣️.json`
- `🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️wire/🟦️.ts`

Authored quiz content, curriculum integration, taxonomy registration and scheduled unlock edits belong to the other agents. No `AGENTS.md`, Git state or worktree was changed.

## Ticket Inputs and Generated Review Files

Preserved inputs: `mockup-contract.py`, `mockup-twins.py`, `mockup-wire.py`, `mockup-validation.ts.txt`, `mockup-semantic.ts.txt`, `mockup-finalize.py`, `mockup-integrate.py`, `mockup-polish.py`, `mockup-widgets.py`, `mockup-widget-fixtures.py`, `mockup-wire-fixtures.py`, `verify-mockups.ts`, `verify-mockup-types.ts`, `mockup-preview.ts`, and this report. They are task input scripts, not runtime dependencies or migration scripts.

Temporary review output: `🗑️generated/mockup-preview.html`, `{excel,parameter,slider,panel,plugin,graph}-preview.png`, `mockups-desktop.png`, `mockups-narrow.png`. Parent will remove the generated folder after reviewing images. All authored paths remain below the 256-character limit.
