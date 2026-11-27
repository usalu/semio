FINAL

# Goat horn case rewritten to the current volumes-feature rule (2026-11-26 17:02)

Change: the case "retains the measured horn stations at print scales and detects deformed substitutes" (written against the removed 1:50-counterpart allowance) is now "judges the goat horns at 1:500 on their own and finds each one thinned below the bone limit" in the model-silhouette suite.
- Authored horns horn-n and horn-f at 1:500: smallest extent above the limit min(75 mm, thinnest drawing of bone head) = 26.42 mm, 0 findings per horn and 0 findings for the whole 1:500 volume.
- Each horn thinned (all rx, ry halved, smallest extent 13.8 mm): exactly 1 finding on its path with the message "Solid ... of body is 13.8 mm thin, below 26.4 mm" naming the thinnest drawing of bone "head"; restored, 0 again.
- Translation and material change no longer end anything: asserted 0 findings.
- Removed: sections-equal-to-1:50 and station-count expectations and the translation/material finding expectations (allowance only).
- Feature scenario "Retained physical sections of a goat horn" replaced by "A goat horn is judged at its own scale".
- Goat asset and rule untouched. No other reference to physicalSources, physicalMatch or retainedStations remains in the staffage tree (grep); volumes-rules mentions only unrelated gauge/breath "physical" wording.

Files: 🧪️tests/🧊️model-silhouette/🟦️.ts and 🥒️.feature (under the staffage folder).

Tests (memory queue, 1600 MB): model-silhouette plus volumes-rules, 2 files, 152 of 152 passed (first run 23 of 24 failed only on a wrong helper of my own, fixed).
