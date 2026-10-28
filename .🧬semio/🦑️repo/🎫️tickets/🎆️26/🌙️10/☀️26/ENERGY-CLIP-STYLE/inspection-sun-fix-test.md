# Canonical Sun Regression

The demand regulations stage attached an updater written for an obsolete multipart sun to the current one-contour filled silhouette. Its first native FadeIn resumed that updater and raised IndexError when it accessed nonexistent children 2 and 3. This occurred before any consuming morph; clearing the live updater would have hidden the intended solar-level behavior.

The language-neutral fixture declares one drawable and levels 0, 0.5 and 1 with fill opacities 0.10, 0.25 and 0.40. The permanent driver constructs the actual authored _stage, exercises native sun updates and the full FadeIn lifecycle, and compares geometry and native Cairo pixels with independent direct-glyph references. Both tests failed before the source edit, reproducing the exact IndexError at all six subcases (0.260 seconds).

The authored fix removes only the obsolete ring/ray stroke-index updates. The live fill law remains 0.10 + 0.30 × solar level. Both tests then passed in 0.267 seconds, retaining the updater and exact geometry/raster output. Shared glyph helpers were not changed.

The fixture and driver are regulations/🧪️tests/☀️sun/🔣️.json and 🐍️.py, already registered in the authoring test entry. The retained invocation wrapper is inspection-sun-test.py. Evidence is in 🗑️generated/is/sun-red.log and sun-green.log. All generated evidence remains beneath this ticket.
