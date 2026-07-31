# Findings: tcolorbox height-bound content ghost/overfull bug

## Reproduction (confirmed, stable, pre-existing regression)

`print/template/report/report.content.tex`'s existing demo section (Logo Cover Demo /
Cover Anchor NW / Cover Anchor SE, all `height=4cm, image-fit=cover`) reproduces the
bug standalone via `bun ./📜️script.ts build report` — NOT specific to the mit-bestand
appendix. Confirmed with `pdftoppm` renders (see `repro/report-page-05.png`): each
Photo shows a faint colored sliver bleeding out past the right edge of its frame,
matching the same colors/pattern as the source image, repeating across all three
demos regardless of anchor (center/nw/se). Overfull \hbox warnings at a CONSTANT
414.84218pt-class value (187.33167pt in the `report` template's narrower page) recur
identically across anchors.

## What was ruled out (each tested independently on the live baseline, rebuilding
   and re-inspecting the PDF/log after every change; all reverted, no residue left)

- **Box width auto-sizing**: kind windows (Figure/Photo/Image) don't get
  `width=\linewidth` the way `semio~window~row` does. Adding it changes nothing —
  the 187.33167pt excess is bit-for-bit identical.
- **`breakable`**: forcing `breakable=false` on ALL kind windows (not just
  in-WindowItem ones) — identical excess, unchanged to 5 decimals. Also breaks an
  unrelated demo (a text-only Figure gets a new transient overfull on the first
  compile pass), so this is not a viable knob regardless.
- **TikZ bounding-box leakage**: added `\useasboundingbox` before the `\path[clip]`
  in both WIDER/NARROWER branches of `\semio_image_typeset_cover:n`, forcing the
  tikzpicture's own reported size to exactly (target_w × target_h). No change.
- **Savebox register reuse**: split the "measure at height" and "render at width"
  steps into two separate `\newsavebox` registers instead of reusing one. No change.
- **valign/halign positioning**: forced `valign=top, halign=left` on the tcbox
  itself (bypassing tcolorbox's own `\tcb@dbox@center` pgftext-wrapping entirely,
  found in `tcbskins`/tcolorbox.sty as the valign implementation). No change.
- **Multiple `\includegraphics`/`\sbox` calls per invocation**: collapsed the whole
  function down to a single unconditional `\sbox{...}{\includegraphics[width=...]}`
  (no branching, no measurement pass). No change — same 187.33167pt.
- **The image content itself**: replaced the include entirely with
  `\leavevmode\rule{target_w}{target_h}` (zero graphics, zero tikz, zero sbox).
  **Still reproduces** at a nearby value (102.80222pt vs 187.33167pt for the image
  case) — i.e. a bare `\rule` sized to the box's own target dimensions, placed
  directly in the tcbox body, ALSO overflows. This is the most important negative
  result: **the bug is not in our image/cover-mode code at all.**
- Confirmed macro invocation count via `\immediate\write17` — exactly one call to
  `\semio_image_typeset_cover:n` per `\SemioImage`, no double-invocation at the
  macro level (6 demos → 6 log lines, never more).

## What actually correlates with the excess amount

- A `\rule{20pt}{20pt}` (small, well inside both target_w≈420pt and target_h≈108pt)
  → **zero overfull** for the top-level (non-WindowItem) Photo demos.
- A `\rule{target_w}{20pt}` (full target width, tiny height) → overfull reappears,
  same order of magnitude as the image case.
- Raw PDF content-stream inspection (via PyMuPDF, `page.read_contents()`) of a
  built page shows, per Photo instance, **two separate `Do` (paint) operators**
  for the same image XObject: one at the correct/intended size, and a second
  "ghost" one immediately adjacent (same y, x offset by the first box's width),
  sized to the image's natural aspect ratio scaled to `target_h`. The `q`/`Q`
  (graphics-state save/restore) operators between them are NOT balanced — the
  raw stream shows more `Q` than `q` since the last balance point, i.e. TeX/pgf
  emits a corrupted graphics-state stack around this content, and the second
  "Do" ends up outside the clip region established by the (correctly-sized)
  first one.
- The excess amount depends on the natural aspect ratio of whatever's forced to
  fill the width (160×90 image → 187.33pt; 500×500 square image OR a bare rule
  of the same footprint → ~102.8pt) — i.e. it scales with content shape, but is
  NOT eliminated by any of the structural changes above, including ones that
  should have made the "natural size" irrelevant (explicit non-uniform
  width+height, or a plain `\rule` with no image at all).

## Conclusion

This is not fixable inside `semio-window.sty`'s own macros — every structural
change to how *we* construct/typeset the content (register, tikz, sbox, alignment,
width-mode, breakability) leaves the exact same excess. The trigger is: **a single
atomic (unbreakable) box whose footprint approaches the tcbox's own fixed
`height=` value, placed as the body of an `enhanced + breakable` tcolorbox.** The
q/Q stack corruption points at tcolorbox's own internal machinery for handling
`height=` + `breakable` (likely a natural-height trial/measurement pass that
doesn't fully discard or re-balances the graphics state incorrectly when the
trial content is at or near the requested height) — this would need to be
root-caused inside tcolorbox itself (or reported upstream), not in this package.

## Suggested next steps (not attempted — out of scope for more trial-and-error)

1. Try reproducing with a *minimal* tcolorbox-only test file (no semio machinery
   at all: `\tcbset{enhanced,breakable,height=2cm} \begin{tcolorbox} \rule{3cm}{2cm}
   \end{tcolorbox}`) to confirm this is 100% a tcolorbox bug independent of semio,
   then search/file upstream (github.com/T-F-S/tcolorbox or CTAN bug tracker).
2. If confirmed upstream, the only in-repo workaround remains what
   INTEGRATE-TEMP-ZWISCHENBERICHT-CONTENT-INTO-MIT-BESTAND-REPORT already used:
   bypass Figure/Photo/WindowItem for full-size images and use a plain
   `\includegraphics` + manual caption in a `\begin{center}`.
3. Alternatively, investigate forcing tcolorbox into its "natural size" auto-height
   mode (omit `height=` on the tcbox, let content dictate size) for image-bearing
   kind windows specifically, sidestepping the fixed-height code path entirely —
   not attempted here since it changes the layout contract (content no longer
   fills to an author-specified height).

## Environment note

An ambient auto-commit process periodically commits the live working tree
(observed twice during this investigation, ~15-20 min apart) — this makes
leaving broken diagnostic code (e.g. mid-experiment `\rule` placeholders) in
`print/tex/semio-window.sty` for more than a few minutes actively dangerous, since
it can land in shared history before being reverted. All experimental edits in
this session were reverted immediately upon completion; the one that got caught
by an auto-commit (`🐙️ueli🎆️26🌙️06☀️04🚩️319`) was restored in a follow-up edit
within the same session.
