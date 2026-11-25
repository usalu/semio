# Web products handoff (audit2-web lanes C, D, E, G, H plus hub admin)

## Fixed
- C-1..3 `lifted` removed from OverviewCard (it was already voided by the ui CSS rule for stack, grid and layered overview), from play, demonstrator and the course presentation browser; the hover test and its json lost the lift vector (`card_paint_target`). ui CSS rule at `🎨️.css` "A card never lifts" is now dead; shared ui owner may delete it with its test.
- D-1 first-visit language chooser tiles the client (no padded main, no `max-w-xl`); test added in `language-choice`.
- D-3 peer focus label is a glass chip (`data-glass-chip`, translucent peer colour, blur) inside the focus frame at top-left; `LABEL_OFFSET.focus` is 0,0; forced colours keep system colours.
- D-6, D-8 load-fallback buttons and the skip link carry icons; the "Jetzt schon anzeigen" row and leaderboard sort headers already had icons (false rows).
- Overlay scrollbar thumb: new course module `🧷️scroll-thumb` (geometry vectors in `🧫️fixtures/🧷️scroll-thumb`, tests), mounted in `Page` (app) and `PageFrame` (chrome).
- E-1 staffage language gate tiles the client; icons on gate and clear-confirm buttons.
- G-1/G-2 presentation handles: radius 0, outline instead of box-shadow ring; test updated.
- H-1 pets stories: radii of card, button, roster, swatch, note, tab, shelf set to 0 (`.prop` depiction kept).
- Hub admin: navbar is an absolute glass band over the pages, pages carry `--admin-clear-top` padding and scroll beneath; test added.

## Skipped (false or by rule)
- C-4 band wrappers: fine. Play and demonstrator bands already have content scrolling beneath (acceptance test).
- D-2 mockup panel and D-5 slider thumb: depictions of another program.
- D-4 table rules: tables fold into records; shared ui Table uses the same row-line class, no compositor table contract exists.
- D-7 dialog reading width: by design. E-3 ground line: drawn content.
- E-2 body inset 3.2: part of the cross-language presentation contract (`chrome.inset`, Python/Manim viewport), not changeable without Python.

## Open
- Home grid (`.course-home-grid`) still has no overlay thumb: its scroller is inside LayeredOverview (ui).
- Bleed-through seen in screenshots is real, not a transition (identical settled captures): the inert, aria-hidden `[data-layered-pane]` of the opened card (e.g. physics quiz page, its red chosen cell) paints blurred under `[data-layered-veil]` (blur 8px) and the card body glass. Owner is shared ui LayeredOverview (veil/pane), out of scope.
- Hub admin test `hands table action cells` (ownFrames 1 vs 0) and `command-routing` fail independent of this change.
- Not run: presentation hover node test (needs ts runtime), playwright acceptance tests (they still expect `ui-chrome-tint`, which stays), Rust, third-party oracle for thumb geometry (none exists).

## Verified
Course vitest 50 files 1191 tests pass; language-choice 29; OverviewCard 6; presentation pdf-and-figure 6; hub admin new test passes. Static build via `web-build.mjs`, shots via `web-shots.mjs` (Chromium channel "chromium"; the default headless shell does not paint here) in `evidence/web/`.
