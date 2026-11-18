# Round 4 audit fixes

Verified each finding of `round4-audit-regressions.md` against the code and the CSS mechanism. Logs: `🗑️generated/r4-audit-*.log`.

## Mechanism added
`🎨️.css` (vertical owner dividers): registered `@property --shared-border-flow` (integer, not inherited, default 0). A cell of a vertical frame/group draws `border-inline-end` = hairline x flow and `border-block-end` = hairline x (1 - flow); the last cell draws neither. A product that lays the cells side by side at a width only sets `--shared-border-flow: 1` on the cells (`.owner > *`), so the one divider per seam moves from block end to inline end and no product re-declares a divider, colour or last-cell rule.

## Findings
1. Overview button raw `border-b border-l`: FALSE POSITIVE. It is a lone corner chip anchored at `top-0 right-0` of the overlay; its top and right sides coincide with the overlay edge (no border there), the two inner sides are its only outline, and the cell rules only touch `border-inline-end` (zero, last cell). Round 3 F6 probe: 0. No change. Live re-probe not needed.
2. ShellHost context-menu trigger `border-l border-t`: FALSE POSITIVE. The trigger sits ABOVE the footer band (`bottom: var(--footer-band-height)`); its bottom edge touches the footer's top line and it draws no `border-b`, so the footer owns that seam; `border-t`/`border-l` are its own outline against page content (floating overlay). No change.
3. Dashed read-only guess: REAL (in intent). The selector `.course-guess > input:read-only` also never matched (input is nested in `.course-guess-entry`), so it was dead dashed CSS. Replaced with `.course-guess-entry > input.course-input:read-only { background: transparent }` (+ transparent placeholder); the read-only state is a fill, never a dashed border. File: `📚️course/🎯️targets/⚛️react/🎨️styles/🎚️challenge-keys/🎨️.css`. Test: course `🧱️shared-borders` (no dashed in any `.course-guess` rule, new selector present). Live re-probe: optional (now the read-only fill actually applies).
4. Hand-rolled vertical dividers: REAL. Deleted the overrides in `🧩️task-layouts/🎨️.css` (`.course-steps`, `.course-classify` pool/categories) and `🏁️results-and-preferences/🎨️.css` (`.course-result[data-crowd]`); each now sets `--shared-border-flow: 1` on its children. Same pattern found and fixed for `.course-match > .course-cards-cell` (not in the audit). Test: course `🧱️shared-borders` (flow variable per owner, no hand-rolled `border-inline-*` hairline on a cell, old stylesheet-order test replaced) and framework `round3-framework.test.tsx` (CSS mechanism). Live re-probe needed: course task view at >=40rem steps, >=84rem classify/results, >=64rem match (a result that was `cell + owner` was why the variable is set on children, not on the owner).
5. Control tree row `gap-x-tiny`: FALSE POSITIVE. The gap separates label TEXT from the value control (not two bordered siblings); the control stretches the full row height and is flush right (`[data-slot="tree-item-control"]` rule), the row is a bottom-edge frame, so the seam is one line. No change.
6. Staged action dialog `SharedBorderRow framed={false} ... border-t`: REAL (minor, raw seam bypassing the edges mechanism). Now `<SharedBorderRow edges={["top"]} className="w-full">`. The dialog's own `border` is the single outer line (the row sits inside it, no coincident edge), left as is. Files: `💻️os/.../🏛️ShellHost/📝️staging/🟦️.tsx`, test `📝️staging/dialog-borders.test.ts` (updated) plus `round3-framework.test.tsx`. Live re-probe: staged action dialog open (footer top line, one hairline).
7. tbody `tr:last-child { border-bottom-width: 0 }`: FALSE POSITIVE. Table rows are not cells so the cell mechanism cannot cover it; the frame (or popover/dialog) owns the bottom line, so dropping the last row's bottom is the one-line rule, not a lost seam. Kept (as round 3 decided).
8. Tree.Files raw `border` on an all-edges frame: FALSE POSITIVE. Since round 3 `data-shared-border-edges` drives native sides: the listed sides keep their native border, and the compositor overlay is positioned at `-borderWidth` on the same line (and ownership makes an attached native side transparent). One line. No change.

## Tests
- `vitest-course.config.ts shared-borders adaptive`: 32 passed.
- `vitest-r4-audit.config.ts` (new, ticket-local: staging dialog, Tree, SharedBorderRow suites): 42 passed.
- `vitest-ui-all.config.ts shared-border border`: 108 passed; failures are environment only (two suite-less files, Playwright browser missing).

## Notes
- Course `.course-tiles` still hand-declares its grid dividers (`> *` borders and nth-child rules); a 2D grid is outside the flow variable and was not part of the findings.
- Live re-probe (browser owned by another agent): items 3, 4, 6 above.
