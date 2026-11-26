# Print page surface levels

The printable sheet uses the shared base color in its physical margins. The page frame contains one opaque window-level surface under ordinary text, heading rows, window and table bodies, and running header and footer bands. Title and page-number chips use pane glass, tinted by the foreground, with neutral shared edges. Heading hierarchy accents remain on the chip's inner vertical edge. The opaque print setting composites the chip against the window interior. Signature blanks retain the bare sheet.

The Tectonic renderer paints the interior at shipout using the active page geometry. Running pages join it to the existing border, including either binding edge. Full-page window arrangements and plain title pages use their cover geometry. The standalone Leutwiler proof and notes reuse the page package with their authored article typography and running heads. The browser presentation renderer applies the same levels in Reveal's PDF view and seats its optional pane page number inside the neutral slide outline.

## Verification

- The repo-dispatched Print quick source verifier passed, including page chrome, logo, consolidated frames, glyph policy and all 1,966 visualization leaves. The assignment, token-contrast and PDF paint-reader suites passed all 55 cases. Colord independently confirmed the text contrast.
- Tectonic 0.16.9 rendered the light and dark reports at 12 pages each, the thesis at 32 pages and the landscape flyer at two pages. All four had zero overfull boxes. Poppler previews showed distinct margins and a continuous interior on the report covers and body pages, the thesis title arrangement, and the flyer. The report still shows image content through the glass chips.
- The independent Pillow and pypdf probe confirms the expected light and dark pixels, blank verso appearance, left and right binding offsets, and that the interior matches the outside of the top and bottom rule to 0.02 PDF points. It reads both actual chip opacity states, 0.76 for pane glass and 0.10 for the foreground veil.
- The Leutwiler proof and notes passed two compiler passes using the exact new material list. The proof has nine pages and the notes have three. Neither produced overfull boxes. The PDF pixels confirm a base margin and a window interior on all twelve pages, and the first pages retain Latin Modern. A three-page dark specimen confirms light ink and both opaque levels. The bundle expanded by 78 observed inputs and 1,996,100 bytes, all checksum-verified ranges of the same pinned upstream archive. The bounded file ceiling is 640 and the byte ceiling remains 32 MiB.
- The Presentation React Vitest suite passed seven cases, including the generated PDF page roles. Chrome's PDF mode confirmed separate base and window paints and pane glass in both appearances on a one-slide deck. The owned dev target waited at the repository graph's host-capacity gate, so the browser probe used a graph-free Vite server with the actual Presentation React renderer and shared theme stylesheet.
- A warm standalone rerun reproduced the paint-cancellation fixture racing a short interpreter operation. The fixture now requests actual paint and queues the abort as a microtask so it runs at the first asynchronous boundary. The 56 material checks and three standalone paint checks passed. Their oracles include Ajv and SQLite for the bundle totals and a child process that preserves the cancellation error.

## Changed paths

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-core.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-page.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-table.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-components.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-thesis.sty`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🥒️.feature`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️frame-junctions/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧪️tests/📋️.feature`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/📤️publication/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🎨️paint/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🔒️dependencies.json`
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🎚️deck-chrome/🟦️.ts`
- `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🎞️deck/🟦️.tsx`
- `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🎨️styles/👻️morph-keyframes-ghosts-and-pdf/🎨️.css`
- `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📄️pdf-and-figure-interaction/🟦️.tsx`
- `👴️leutwiler/💤️realparts-of-powers-z-n/🏆️proof/📐️.tex`
- `👴️leutwiler/💤️realparts-of-powers-z-n/📯️notes/📐️.tex`
- `👴️leutwiler/💤️realparts-of-powers-z-n/📋️project.json`
- Ticket probes `print-deck.html`, `verify-deck-print.mjs`, `verify-print-levels.py` and `fetch-article-tex-ranges.ts`
