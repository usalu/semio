# Root cause

Two client-side bugs made fill objects appear in Perspective but go missing in Top:

1. **One-shot orthographic framing** — `WorldProjectionContentFrame` framed each seeded pane once, then unmounted. Fill planning appends instances that expand `worldSceneContentBounds`; Top's orthographic frustum stayed locked to the seed/reference footprint so new pieces fell outside the view. Perspective's wider cone still showed them.

2. **Reveal-cutoff store clobber** — every `fillBuildTick` refresh rewrote `interactionJson` with a new `revealCutoffs` object identity carrying the same committed `fill_count` (often `0`). Both panes' reconciliation effects then reset the shared `worldRevealCutoffStore` mid-slider-drag, hiding reveal-tagged fill previews.

# Fix

- Keep content framing enabled while the viewport is not user-owned; re-frame whenever the content-bounds key changes (soft camera update after the first seed remount).
- Reconcile `worldRevealCutoffStore` only when the *committed* cutoff number changes, not on object-identity churn.
