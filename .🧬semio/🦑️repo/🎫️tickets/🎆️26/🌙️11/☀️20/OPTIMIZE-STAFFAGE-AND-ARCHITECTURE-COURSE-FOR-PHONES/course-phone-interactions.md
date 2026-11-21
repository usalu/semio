# Architecture Course Phone Swipe and Guess Fields

Chromium at 375 × 812 with iPhone 13 touch emulation passed the German overview’s seven-card layout. Real touch gestures wrapped both ways horizontally and vertically. A slow, short swipe stayed on the current card. The German edge label was present and tapping the down hint opened the named card.

Physical swipes reached the nested energy overview and travelled both ways between the cooling quiz and its sparse final row. The German cooling quiz completed at 100%, showed localized results and retained the best result on the quiz card when returning home.

The English hard physics quiz kept all 11 rendered guess fields inside the 375 px viewport. Each field measured at least 44 px high with a 16 px font. The task steps recorded completion after answers were entered, classification descriptions stayed hidden, and the run earned the full hard-mode points. The run and results stayed within the viewport horizontally. Neither session reported an uncaught browser error.

The ticket’s `course-phone-interactions.ts` runs these journeys with the canonical learner helpers and the live course application via the in-process Vite adapter. The local deputy scores the submission. The canonical online gate exercises the separate live Proctor path.

## Desktop course check

At 1440 × 900, Chromium used the pointer-panned strip at home and in the nested Quizzes and Energy grids. The course picker stayed hidden. Opening Physics then pressing Escape returned to Energy and restored keyboard focus to the Physics link. The English Parametrization panel mounted an inline PDF iframe with the fit-width fragment and exposed both open and download links. The document fit the viewport and the browser reported zero uncaught page errors.

The ticket’s `course-desktop-browser.ts` verifies the desktop behavior. Its in-process Vite adapter does not deliver PDF bytes. The [receipt audit](course-assignment-receipts.md) records the missing print inputs.
