# Corrected Render Execution

The stable corrected controller exited successfully. Native process inventory found zero remaining owned render or assembly processes after completion.

The parent's independent final verifier subsequently exited successfully for all five movies: 113,285 total decoded frames, exact ordered raw-segment/chapter/movie totals and sample checks, and both full decoding implementations passed. Final corrected-glyph visual and nested assembly audits also passed. Ticket closure, final staging, and generated-output cleanup remain parent-owned coordination steps.

- Successful full segment render attempts: 117/117; native exit status 0 for every attempt.
- Peak recorded concurrent Manim processes: 14 (configured global bound: 14).
- Raw render wall time: 63.67 minutes, measured from first child start to last child finish.
- Frozen font-helper SHA256: 00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2.
- Every segment has successful source/settings/class provenance. No partial-animation shortcuts, old movie reuse, or visual provenance rebasing was used.

Header frame counts below are execution metrics; the independent final verifier decodes every frame and checks exact segment-to-chapter-to-movie totals.

| Video | Header Frames | Duration (Seconds) | Declared Format |
| --- | ---: | ---: | --- |
| physical_fundamentals | 15,070 | 1004.648 | 854×480, 15/1 fps |
| ventilation | 24,333 | 1622.187 | 854×480, 15/1 fps |
| heating | 25,918 | 1727.856 | 854×480, 15/1 fps |
| cooling | 16,142 | 1076.124 | 854×480, 15/1 fps |
| energy_balance | 31,822 | 2121.460 | 854×480, 15/1 fps |
