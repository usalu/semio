# Cursor Preview Test Exploration

Read-only investigation of teaching presentation authoring. Root AGENTS.md was read; no additional AGENTS.md appeared in the teaching source inventory.

## Existing Contracts and Fixtures

All paths below are relative to 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation.

- 🔣️.json and 🧬️schema.json own the topic/video hierarchy and focus/restore timing. The hierarchy loader reads this parent-level contract, not a separate hierarchy contract file.
- 🥞️hierarchy/🧪️tests/🔣️.cases.json is language-neutral semio-teaching-hierarchy-cases/v1. It owns stages, controls, click effects, topic membership, and en/de Ansehen/View labels. Existing controls are synthetic and do not encode a hover-tracking motion case.
- 🖱️interaction/🧪️tests/🔣️.cases.json covers hit paths, event commits, cursor intervals, transitions, and native frame samples. 🖱️interaction/🧬️schema.json owns half-open hit geometry, enabled ancestry and source-order precedence.
- 🌐️browser/🧪️tests/🥞️layered/🔣️.cases.json and its JavaScript runner are existing real shared-UI oracles.

## Render Entry Points and Gap

- 🐍️.py:99 render_interaction_frame delegates to the hierarchy renderer; render_overview and render_shell share that pipeline.
- 🥞️hierarchy/🖌️render/🐍️.py:54 _probes measures native DOM chip rectangles at fixed initial hierarchy states; lines 68–70 retain only their centers.
- The same file:76 choreography uses fixed controls throughout topicReveal/topicPress/topicHold and videoReveal/videoPress. _camera(point) drives panning from that same point. Fixed screen-space centers can become stale while the DOM moves in response to the pointer.
- The same file:171 frame_batch probes once per video, derives all timeline points, and sends the finite batch to the actual browser renderer. It returns measured nodes, browser cursor CSS, pointer events, timeline, hover ancestry and styles; these are strong regression assertions without image guessing.

## Existing Tests to Extend

- 🥞️hierarchy/🧪️tests/🐍️.py:52 validates exact release/click boundaries using independent NumPy cumulative sums. It currently validates effects and pressed state, not hit membership during travel/reveal.
- 🖱️interaction/🧪️tests/🐍️.py:542 validates layered focus/reveal/open state and canonical boundary pixel equality.
- The same file:576 verifies production focus and restore at 4 fps, including two native pointerdown/up/click sequences; it uses physical_fundamentals only.
- 🌐️browser/🧪️tests/🐍️.py:216 already measures actual chip rectangles, then verifies native hover/press/release for both levels. This is ideal precedent for checking pointer membership in measured controls.

## Minimal Regression and Oracle Proposal

Add a language-neutral motion section to hierarchy cases before implementation. Cover distinct topic/video centers and near-coincident centers (existing waypoint branch), en/de, multiple viewport sizes, and optimization topic/video rather than energy alone. Specify sampled normalized times, expected active target, release effects, and a tolerance in canonical pixels.

For pure movement calculations, use NumPy array interpolation or numpy.polynomial independently of the production scalar interpolation implementation. Preserve endpoint and finite-input assertions. For actual moving-chip behavior, use existing Playwright/Chromium DOM measurements as the third-party oracle: at each reveal/press/release sample the cursor hotspot must lie within the intended measured chip rect and the browser hit target must be the intended control. Assert both clicks still occur at 4 fps, no extra click fires during travel, the target remains hovered for its intended dwell, and restore ends at the canonical park with matching boundary pixels.

A fixed-point chip target, if chosen, should be tested with synthetic measured transforms as portable input/output cases and independently solved using NumPy linear algebra for affine pan geometry; retain actual Chromium verification for the real UI transforms. Do not merely compare the implementation with a copied formula.

No tests were run and no implementation files were changed by this exploration.
