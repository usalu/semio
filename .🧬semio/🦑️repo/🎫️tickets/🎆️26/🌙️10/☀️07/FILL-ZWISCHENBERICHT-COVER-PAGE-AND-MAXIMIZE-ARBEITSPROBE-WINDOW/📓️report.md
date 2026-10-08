# Fill Zwischenbericht Cover Page and Maximize Arbeitsprobe Window

Manual ticket bookkeeping: `ticket_open` returned a malformed result twice.

## Findings

1. **The cover column never reached its declared height.** `WindowColumn` subtracted an
   estimated per-item chrome of `titlebar + 2·bodypad + 2·hairline + 3pt` although a
   window's `height=` already contains its padding and rules. Six items lost ≈ 12 pt each;
   together with the hard-coded `\textheight-1.2cm` the page ended ≈ 104 pt above the
   bottom margin (ink 44 pt → 695 pt on an 842 pt page).
2. **Tab heights were content-dependent.** A title with descenders (`Arbeitsprobe`,
   `Antragstellende Institution`) made its tab 2.25 pt taller than one without, so no
   constant could describe the chrome.
3. **The Arbeitsprobe was a hand-sized `\includegraphics` in a padded minipage**, bound by
   height, leaving ≈ 60 pt of empty window left and right and padding above and below.

## Changes

- `semio-window.sty`
  - `WindowColumn` floors every tab in the column to one descender-safe height and
    measures its real chrome from reference windows rendered through its own item loop
    (one item → leading chrome, one → two items → pitch). Bodies now sum to the declared
    height exactly. The column is one `\vbox to <height>`, so it cannot shed an item onto
    the next page.
  - Generic `Window` honours `image-fit=cover` (previously only kind windows such as
    `Photo` received the padless clipped body).
  - Cover-fit target height is the real body (`height − bottom rule`); it was one hairline
    short and left a canvas sliver.
- `semio-components.sty` — cover arrangement uses `\textheight`; the Arbeitsprobe is
  `\begin{Window}[image-fit=cover] \SemioImage{…}`.
- `semio-phd.sty` — cover arrangement uses `\textheight` (same removed fudge).

## Verification

`cover-probe.ts` compiles a cover from the live sources into `🗑️generated`.

| Cover | Ink top → bottom (pt) | Page / margins |
| --- | --- | --- |
| Zwischenbericht (before) | 44.0 → 695.0 | 842 / 42.5 |
| Zwischenbericht | 42.5 → 799.0 | 842 / 42.5 |
| Forschungsbericht | 42.5 → 799.0 | 842 / 42.5 |
| Kompaktbericht | 42.5 → 799.0 | 842 / 42.5 |
| PhD template | 42.5 → 799.0 | 842 / 42.5 |

Arbeitsprobe window on the Zwischenbericht, sampled at 1200 dpi through the window:
image height 194 pt → 255 pt, full window width, no canvas at the bottom or sides, 0.18 pt
at the top (the pre-existing tab/frame seam shared by every window). At weight 10.3 the
window has the capture's 2:1 proportion, so nothing is cropped.

The report template's `Photo` cover demos and the in-item cover checkpoint still render
once each.

## Open

- The Forschungsbericht Arbeitsprobe window is flatter than its capture, so cover-fit crops
  it top and bottom there.

Full report rebuilt (193 pages); page 1 of the published PDF measures 42.5 → 799.0 pt. Before/after renders: `cover-before.png`, `cover-after.png`.
