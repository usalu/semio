# Phone usability and performance

## Staffage

The phone catalog uses a native scrolling list with search and direct links. List entries come from the manifest, and opening an asset loads its detail on demand. Returning from a detail restores the previous list position. The asset viewer opens paused on phones and keeps page scrolling available around the artwork. The language gate scrolls on short screens, and primary controls use larger touch targets. A failed load offers one Retry action that fetches only the selected asset. Geometry validation findings are still shown for invalid records.

The Staffage React typecheck passed. The final registered React suite passed 294 of 295 tests, including all phone and viewer cases. The all-assets readiness test reached its 120-second limit on the final run. An earlier full run completed the assertion and found the authored failures described below. Both Chromium and WebKit verified navigation, one selected-asset request, playback, Back restoration and no horizontal overflow at 320 × 640. They also verified the language gate at 844 × 260 and 200% text scaling. The 11-test catalog navigation and selected-asset retry suite passed. A separate component test proved that a cancelled touch restores the viewer and a completed drag commits. In Chromium, a dispatched touch that started on the drawing cancelled the stage pointer, moved the native detail scroll position from 465 px to 577 px, and preserved the drawing position. The six geometry oracle cases passed using a ticket-local uv cache.

An independent run of the current production archive validator checked all 152 authored records. It found 136 ready and 16 invalid. The invalid records comprise five vegetation and eleven animal models with record-specific coverage, printability or scale defects. The active asset-authoring ticket owns those records. A fresh full validation of the current shared checkout returned the same 136 ready and 16 invalid records. Both registered phone test files also passed again (18 tests), and TypeScript passed again.

## Architecture course

The course phone overview retains swipe navigation and adds a direct picker for destinations. It mounts only the active pane and loads only the visible clip poster on a phone. Quiz crowd requests follow the revealed quiz, and assignment pages provide direct open and download links on portable layouts. A short touch landscape viewport receives the same navigation. The overview respects reduced motion.

Holding a quiz item at the top or bottom of its scroll area now scrolls the nearest available vertical scrollport, updates the highlighted drop target and preserves the drag position. The live quiz uses a nested scrollport, which the browser test confirmed must scroll independently of the outer course shell. The initiating pointer owns the gesture. A cancelled or lost gesture clears the ghost, highlight and animation. Scroll suppression applies to touches that begin on the enabled grip. Coarse-pointer quiz controls use 44 px targets and answer fields render at 16 px or larger. The swipe overview keeps its settled-card callback stable and ignores duplicate updates to prevent a rendering loop on phones.

The registered course suite passed all 1,163 tests across 49 files after the final changes. Coverage includes home layout, navigation and on-demand crowd requests, touch targets and nested drag scroll, local language recovery, and keyboard interactions. Playwright discovery found all four canonical phone specs, including the completed classification touch drop. A later full pass again passed all 1,163 tests with two workers. An earlier high-contention run exposed fragile synchronization in the language failure test. It now waits for the alert element before checking its localized message, retry and dismissal.

The production course UI completed its offline journey in Playwright Chromium and WebKit with iPhone 13 emulation. A 320 × 640 portrait and 812 × 375 short landscape viewport exercised navigation, an actually decoded clip poster, direct assignment PDF links without inline downloads, and zero idle quiz crowd requests. At 375 × 560, the English Physics quiz completed and scored 100 through the on-device authority, with zero uncaught browser errors. Chromium also verified that a real touch drag scrolls the quiz and changes the selected answer by dropping it in an offscreen category, using the same helper as the canonical phone spec.

Further Chromium checks completed the German Cooling quiz at 100% after real swipes through both overview axes and the sparse nested row. The English hard Physics run earned full credit with all 11 guess fields at least 44 px tall, a 16 px font and no horizontal overflow. Neither context reported an uncaught browser error.

The course browser runner’s split development waves now use two-character report directories. A regression fixture reproduced the 257-character reserved path with the old name and passes at 256 with the new name. All 59 local-stack tests passed, including the independent Ajv path-length check. The complete architecture-course site suite passed all 875 tests across 20 files.

A standalone repo launcher workspace inside the ticket recovered test execution while the shared images were stalled. Later the live launcher recovered, so the React test counts above are from the repository’s registered configurations. The browser harness uses actual Vite HTML, JavaScript and CSS without a listening socket, suppressing only the dev-server HMR connection when serving requests in-process. After loopback listen recovered, the same Chromium phone journey also passed against a live local Vite server with HMR running. It verified narrow portrait, touch landscape, idle request counts, the offscreen classification drop, and the 100% Physics result.

The canonical course React typecheck passed along with its six declared build prerequisites, including the framework, actor and UI generators. Direct TypeScript verification also passed after those products were generated. Direct verification of the production site TypeScript configuration passed. The wider teaching configuration found an invalid Playwright timeout option in the chaos diagnostic. That diagnostic now races evaluation against the existing five-second timer. Playwright still discovers all four phone tests and the chaos test. The wider teaching configuration still reports 27 diagnostics in shared repository tooling and reports zero in course code. The canonical connected development gate completed successfully after building its prerequisites. The browser boot test and all four phone specs passed against the real Vite server and native Proctor. The specs exercise the German swipe journey through scored results, the English hard-mode quiz, portrait and landscape navigation with on-demand crowd traffic, and touch drop onto an offscreen category. All six generated assignment URLs independently returned HTTP 200 and application/pdf. The response bytes matched the corresponding published artifacts by SHA-256. The browser matrix used phone emulation in Chromium and WebKit.

Desktop Chromium verified the Staffage strip, keyboard entry into Humans and all 28 manifest cards at 1280 × 800. It also verified the course’s desktop strips, keyboard focus restoration and inline assignment viewer at 1440 × 900.

## Verification records

- [Staffage implementation and browser checks](staffage-phone.md)
- [Staffage retry and catalog follow-up](staffage-phone-follow-up.md)
- [Staffage audit](STAFFAGE-PHONE-AUDIT.md)
- [Staffage desktop verification](staffage-desktop-check.md)
- [Staffage geometry audit](STAFFAGE-GEOMETRY-AUDIT.md)
- [Architecture course implementation](architecture-course-phone.md)
- [Architecture course audit](PHONE-AUDIT.md)
- [German swipe and English hard-mode browser verification](course-phone-interactions.md)
- [Connected assignment print prerequisites](course-assignment-receipts.md)
- [Independent connected phone verification](course-live-phone.md)
