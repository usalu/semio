STATUS: B7 APPLIED AND GREEN. All 6 patches + the 2 D edits applied; wasm --lib checks GREEN: cad-editor+cad-modes (W-b7-1), flow-editor (W-b7-2), process3d-modes (W-b7-3), procedural semantic-ui (uii18n-4); bun test 4 pass. Logs `🗑️generated/1102/W-b7-*.txt|rc`. Not run: tsc/vitest/browser, new cad label test not yet added. Marker win-B7-done written.

D = `♻️mit-bestand/🧺️demonstrator`. Tools: `C/1102-tools/uii18n/` (`mkpatch.py` fail-closed replace, `gen-*.py` regenerate the patches from the live tree).

## 1101 edits present and consistent
- Locale: `D/🔨️modules/🌐️locale/🟦️.ts` (`resolveDemonstratorLocale`, no fallback, en first then de) and `⚛️.ts` (`commitDemonstratorLocale` -> `initUiLocaleSync`, which persists the choice, sets `<html lang>`; `ui/🌐️i18n/🟦️.ts:219`). `🌐️.html` has no static `lang`. Picker (`⚛️locale-picker.tsx`) lists En/De in own `lang`, nothing preselected; navbar switcher uses `aria-pressed` and a localized group label. 146 catalog keys, all with non-empty en+de; 8 identical cells are proper nouns/cognates.
- A11y: `<h1>` brand, `<main>`, `text-sm`/`text-xs`, `overflow-auto`, footer credits from the shared table (`⚛️footer.tsx`), icon-only credits keep an `aria-label`. Pane shells get `initialLocale`; their brands are `ephemeral`, so a fresh in-memory store cannot override the landing choice (`ShellHost/🟦️.tsx:136`).
- Guest i18n round 2: `GenerationFormLabels`, `form_no_inputs/field/no_image` in generation2d/3d, fem3d/fem2d result labels all present (grep). Standalone dev shell title is localized (`🐚️shell/🟦️.ts`).
- No hard-coded single-language JSX text, `aria-label`, `title` or `alt` found in D (grep); the only language-literal hits are the `en`/`de` table cells in `🪧️brand.ts:22,26`.

## Findings
| # | Sev | Where | Finding | Patch |
|---|-----|-------|---------|-------|
| 1 | High | `✏️s/🔌️plugins/📐️cad/.../✏️editor/🧰️kit/🦀️.rs:633,637-650,652,699`; shown by `.../🎭️modes/✏️edit/🦀️.rs:550,568` | koordinator (cad) engagement status row `Schritt: Committed 1 object(s)` / `Idle` / `Moved` / `Unsupported: x` / `Unknown: x` / `Nothing to commit`: the step is stored as an English machine key and printed raw next to a localized "Schritt" | `cad-step-labels-1/2.patch` (9 labels en+de, `engagement_step_text` maps at render; stored key and existing tests unchanged) |
| 2 | High | `.../📐️cad/.../⚙️engine/🕹️interaction/🦀️.rs:388,437-451`; 60 spec files `.../📚️examples/🖼️assets/🏗️modelDefinitions/*/🕹️interactions/*.json` (e.g. `🪵️placeBeam.json:5,216,250`) | Interaction `label`, state prompts ("Place Beam: click start point") and `last_response` "OK" are English-only data strings, surfaced in the possible-engagements list and step prompt. Needs a schema change (`label`/`text`/`prompt` as `{en,de}` cells) plus 60 files; too large for a patch here | NOT patched; owner = cad lane, schema-first |
| 3 | Medium | `.../🌊️flow/.../🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:123,156` | flow generate form still has `(no image)` and `No input widgets to generate from.` (the 1101 round-2 fix covered only generation2d/3d) | `flow-form-labels-1/2.patch` (labels `form_no_inputs`, `form_no_image`, passed via `FlowPlayLabels`) |
| 4 | Medium | `.../🏭️process/.../🪟️windows/🪚️workpiece/🦀️.rs:208` | bearbeiten status `2/5 steps · 0.1234 m³`: "steps" English; `labels.steps` already exists | `process-status-steps-1.patch` |
| 5 | Medium | `D/🟦️.tsx:157` (`key={locale}`) | Switching language remounts the whole `LayeredOverview` and every opened pane: edits and undo history are lost. Probably acceptable for a demonstrator, but untested; consider a notice or a deferred switch while a pane is open | none (design call) |
| 6 | Low | `D/⚛️locale-picker.tsx:19` | `<main role="group">` replaces the `main` landmark role with `group` (screen readers lose the landmark on the picker). Remove `role="group"`; keep `aria-labelledby` | one-line edit, below |
| 7 | Low | `D/⚛️footer.tsx:6` | `cn` imported, unused (fails `noUnusedLocals` if enabled) | one-line edit, below |
| 8 | Low | `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/🦀️.rs:206-207` | round-2 inserted `GenerationFormLabels` between `generation_form` and its docstring: two stacked docstrings now sit on the struct, the function lost its own | `procedural-docstring-1.patch` |
| 9 | Low | energy viewer `.../👁️viewer/.../⚡️simulation/🦀️.rs:50,53,55` | viewer window prints "English · Deutsch" side by side and an English-only fallback `Energy results unavailable`. Documented deliberate design (viewer has no locale switch); not default-visible in the pane (editor mode) | none |
| 10 | Low | gis `...🎚️config/🧬️schema/🧬️mutations/*/🦀️.rs:39` `mutation.no-op` warnings; cad `Node N`/`Box N`; port labels "Parameters/Geometry/Results/Features/Scene/Catalog" (`generation*/✏️editor/*.rs`, fem, gis, puzzle `AppIo` ports) | English diagnostics / authored data / media-port labels; surface only in diagnostics or workflow wiring, not in the 8 pane defaults | none |
| 11 | Info | `D/🧪️tests/🧪️demonstratorlocale/🐍️.py` | `python3 -m unittest discover -p 🐍️.py` found 0 tests (emoji module name not importable by unittest); Python twin of `localeResolution` unproven this lane | run the twin by path |

Items 6 and 7 are edits in `D` (frozen): apply in the window: in `⚛️locale-picker.tsx` change `<main role="group" aria-labelledby=` to `<main aria-labelledby=`; in `⚛️footer.tsx` change `import { cn } from "@semio-tech/ui-react";` to nothing (delete the line; `cn` is otherwise unused: grep shows only the import).

## Keyboard / ARIA review (reading, not a browser run)
- Landing: picker buttons, switcher buttons (`aria-pressed`), card buttons (`OverviewCard as="button"` with localized `openNamed` label), credit button/links are all native, tabbable elements; focus-visible emphasis in `🎨️globals.css:7-17`. Navbar wrapper is `pointer-events-none`, switcher re-enables `pointer-events-auto` (unverified in a browser; the acceptance spec `language switch` covers it once serve runs).
- Panes: actions/trees/spinbuttons rely on framework elements (`role=tree/treeitem/spinbutton/progressbar`, `aria-live` asserted in `🧪️tests/🎭️acceptance/🟦️.ts:585,670,777`); not re-audited here.

## Patch list (apply order independent, each needs a narrow wasm `--lib` check)
`C/1102-tools/uii18n/patches/`: `cad-step-labels-1.patch` then `-2`; `flow-form-labels-1.patch` then `-2`; `process-status-steps-1.patch`; `procedural-docstring-1.patch`. Each is `diff -u` from repo root (`patch -p1`). Suggested checks: cad editor crate, flow editor crate, process3d editor crate, procedural semantic-ui crate (`--target wasm32-wasip2 --lib`). New labels are covered by the existing German-is-a-translation terminology tests; add after applying: a cad test asserting `engagement_step_text` for "Committed 2 object(s)" differs between NATIVE_EN and NATIVE_DE (language-agnostic, no oracle applicable).


## B7 window note
uii18n-1..3 failed on foreign syntax/name errors (policy patch `semio-framework-os-store-owned-schema-fields`: undefined `diagnostic` at the `ok_or_else` closure; fixed there to `self.diagnostic("artifact-envelope.history-factory-absent", 0)`; a retained-command `if` after `?` was fixed by a peer). Re-ran as W-b7-1..3, all rc 0.
