# Page rotation missing in de-DE journey (ops-de)

## Root cause
Harness, not product logic. In de-DE the German label "Ausführen" widens the Execute button of the staged action form in the Thumbnails window. Its centre (67,945) lies under the docked "Hilfsmittel" chip row (`DIV.flex min-h-chip-row ...`), whereas in en-US the button sits at (76,934) and is uncovered. `clickUncovered` only retires covering panels (`data-slot="panel"`), so the chip row stayed on top; the click hit the chip row and still returned `ok`. The action was never dispatched (no notice, no fault, console identical in both locales).

Evidence: `🗑️generated/rotation-de/d/journey.json` (`rotated.hit`: de `top=DIV#.flex min-h-chip-row...`, en `top=self`). Form readback was identical in both (page=2, x=90, Execute enabled). The guest is locale-independent (`✏️editor/📄️document/🦀️.rs:40 set_page_rotation`, `🖼️page/🎬️actions/🦀️.rs:36`), so no wasm rebuild is needed.

## Fix (`pdf-pane-journey.ts`, `act`)
- Before submitting, record `elementFromPoint` at the Execute centre (`hit`), the form readback (`form`, `executeState`) and post-state (`after`, `notices`, `alerts`) in the step evidence.
- If Execute is covered, submit with a DOM click (`submitted: "dom-click"`), otherwise `clickUncovered`.
- The verdict still relies on the third-party oracle (pymupdf + pypdf: `/Rotate 90` on page 2), so a swallowed click fails loudly.

## Evidence
`sh run-round4-journey.sh rotation-de/e ops-de ops-en`: all 6 verdicts pass; `de-ops.pdf` and `en-ops.pdf` both "2 pages, rotation 90, A4, text, red rectangle, title" (`🗑️generated/rotation-de/e/`).

## Remaining product note
The Execute button of the staged form being coverable by the bottom chip row at this window size in de-DE is a real layout defect (a user could hit it too). Not fixed here; worth a separate ticket against the window-pane layout (`ShellHelpers/🪟️window-panes/🟦️.tsx` action footer vs chip row).
