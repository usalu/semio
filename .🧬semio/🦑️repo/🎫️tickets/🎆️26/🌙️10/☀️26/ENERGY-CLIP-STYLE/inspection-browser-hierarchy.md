# Recursive Browser Hierarchy Review

Read-only review of browser Python, TSX, MJS and protocol schema, plus the successful actual DOM oracle snapshots. No duplicate browser render was started.

## Code Review

The recursive Hierarchy maps path depth0/1/2/3 to courses/topics/videos/chapters. Only depth3 renders VideoPane, so a selected chapter receives its own native virtual window. Open/reveal branches are checked against the current ancestor prefix. Closing a chapter returns to its clip path. The tree projects the same branch keys and path arrays; footer breadcrumbs navigate exact ancestor prefixes, with the root button returning an empty path and the range control removing one level. MJS validates opened/revealed paths against actual catalog entries and rejects obsolete navigation fields. Metadata reports actual opened/revealed paths and footer ancestor labels/rectangles. Python canonical-root recognition uses the new empty paths. No blocking branch-path, footer or metadata mismatch was found in the inspected current code.

## Actual DOM Visual Pre-Audit: PASS

Inspected all ten original1600 ×900 snapshots at `🗑️generated/icb/chapter-dom-b99en64z/{en,de}-{0,1,2,3,4}.png`. In both languages, the sequence displays one architecture course, six topic cards, five Energy clip cards, six Ventilation chapter cards, then the native Air Volume and Flow chapter window. Card titles, view chips, fullscreen controls, fixed top section label and footer breadcrumbs fit without collision. The German longest chapter title and breadcrumb remain readable. Background layers stay blurred behind the active level as expected.

This pre-audit demonstrates the new four-level DOM composition and chapter window. It does not yet accept animation trajectories, content-camera seams, all38 chapter exports, exact adjacent export endpoints or the refreshed17 full movies. The chapter window currently displays its thumbnail because this oracle has no authored body media; that is an oracle condition, not a claim of completed chapter delivery. Final video/chapter visual acceptance remains separately gated.

## Read-Only Performance Check

The physical smoke completed all 26 focus PNGs. Frames 8–24 took approximately 1.6–4.3 seconds each; terminal leaf/content-camera frame 25 took 25.85 seconds. The observed restore packet advances the fake browser clock 264.286 ms per ordinary frame and 2000 ms at its canonical ancestor endpoint.

The regular course HomeOverview uses exactly the same lifecycle policy as this browser: budget equals the number of entries and both warm delays are zero. Shared usePaneLifecycle consequently materializes recursive background previews. Reducing the budget would change those previews and is not justified by these observations. Fake-clock runFor also replays animation-frame ticks; fastForward for canonical empty-media settling is only a candidate requiring independent full-raster equivalence before any change. No lifecycle or timer optimization was applied.

A read-only doctor snapshot showed 43.1 GiB available, no memory pressure and only the root-owned 2 GiB physical preview grant alongside verifier testing. At that snapshot the preview had no Chromium child, consistent with browser rendering having completed and the caller executing Python-side guards/discovery. No owned publisher was started or interrupted.
