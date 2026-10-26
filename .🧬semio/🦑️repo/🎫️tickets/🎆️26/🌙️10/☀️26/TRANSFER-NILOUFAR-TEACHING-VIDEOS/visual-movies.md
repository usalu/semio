# Completed Movie Visual Samples

Result: visual defects found in two samples. Independent contact-sheet inspection covers four completed 480p15 movies at 10%, 50%, and 90%; it does not certify the fifth Heating movie or continuous playback. Commands completed; no source, index, or render process was changed.

| Movie | Target times (seconds) | Observation |
| --- | --- | --- |
| Physical Fundamentals | 100.451, 502.257, 904.063 | Diagrams and narration captions are visible. At 904.063 s, multiple solid white replacement rectangles appear beside temperature/humidity labels and bottom diagram annotations. These are visible missing-glyph artifacts, not a black-frame transition. |
| Ventilation | 162.212, 811.060, 1459.909 | Roadmap, natural-ventilation diagrams, and captions are visible without obvious clipping or missing glyphs. The 90% sample shows seven empty outlined cards beneath the title while its caption introduces the board; this is consistent with an intentional build-in transition, rather than a missing movie frame. |
| Cooling Demand | 107.592, 537.962, 968.331 | Diagrams and captions are intact. At 537.962 s, the blue ventilation formula label starts with a small boxed unresolved glyph. No large clipping or black frame was seen. |
| Energy Balance | 212.219, 1061.097, 1909.974 | Diagrams, axes, and narration captions are visible without obvious clipping or missing glyphs. Partially drawn equation at 10% is consistent with an animation build at the sampled instant. |

All twelve sampled movie frames contain scene content; none is entirely black or missing. Unused black contact-sheet cells are layout padding, not movie samples. At this sampling scale, diagrams and lower narration captions generally fit within the frame. Continuous transitions, small text at native resolution, and unsampled frames remain outside this spot check. Frame targets were computed from container durations; the script chooses the first decoded frame at or after each target.

Derived sheets remain under generated: visual-movies-10.jpg, visual-movies-50.jpg, visual-movies-90.jpg. The first 10% command saved its sheet before its terminal print encountered Windows cp1252 Unicode encoding; the JPEG was successfully opened and inspected. Subsequent commands used UTF-8 mode and completed normally.
