# Overview drag → slide jump

Reveal.js overview registers capture `click` on each slide (`overview.js` `onSlideClicked`). Pointer drag on dispositions still synthesizes a `click` on release, which exits overview and navigates.

Fix: after move/resize drag or marquee, register a one-shot capture listener on `.reveal` that swallows the next slide `click` before reveal’s slide handler runs.
