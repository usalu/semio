# Layered Energy Clip UI

Current fullscreen placement: [Window Name Chips](./title-chip-summary.md).

Current playback behavior and preview: [Transition-Only Virtual UI](./content-camera-summary.md). Older review films retain their capture versions.

Current chip placement and preview: [Right-Side Fullscreen Chip](./right-chip-summary.md). Earlier review films below retain their capture version.

The clips now render the actual shared semio LayeredOverview, OverviewCard, OverviewCardOpenChip, WindowChrome, Navbar and Footer components, using the existing course grid CSS. The first level contains Energy, Parametrik, Building Information Modelling, Grasshopper, Excel and Optimierung in the requested order. Energy opens a second level with the five authored videos. Concurrent BIM curriculum work is preserved, including its authored videos; new topic videos added by other agents remain available to subsequent captures. Topics without authored videos show a localized empty page.

Every card has its fullscreen icon in the window name chip. The bottom-right Ansehen chip (View in English) is text-only. Pointer movement drives the shared background follower through both active and inactive nested levels. Chip hover uses the shared full-viewport reveal, tint, veil and silhouette behavior. Previews stay inert until a native click opens them. The cursor clicks the measured fullscreen control to restore the video overview and the measured footer control to return to topics. Topic artwork is distinct and uses the primary accent.

## Review Media

- [Navigation preview](./layered-navigation.mp4): 13.917 seconds, 1600 × 900, 12 fps; both background levels, body hover, chip hover, native click, fullscreen restore and parent navigation.
- [Physical preview](./layered-physical.mp4): 24 seconds, 1280 × 720, 8 fps; a fresh 18-second excerpt from canonical Beat1_Leistung, wrapped by three seconds of navigation at each end. This is an excerpt for visual review; the retained older full physical draft is separate.
- [Topics](./layered-topics.png), [videos](./layered-videos.png), [settled fullscreen preview](./layered-preview.png), [physical content](./layered-physical-body.png).

## Validation

Current scoped verification is green across 14 hierarchy/browser cases, nine contract/publication cases and two native/media cases. All five energy envelopes have identical encoded boundaries and preserve frame order, body audio and silent navigation padding. The combined 25-case run completed in 950.891 seconds: its two contract failures were an already corrected manifest scope and a concurrent backend-file edit. The separate current nine-case contract rerun passed all cases with unchanged strict hash assertions. No media or interaction assertion was weakened.

The independent browser oracle imports the shared components separately. Bilingual production comparisons verify actual glass colors and filters, chip positions, pan transforms, full-preview bounds and inert/opened behavior. Native pointer traces prove down/up/click on the topic chip, video chip, fullscreen control and parent control; releasing outside cancels opening. Four-fps production transitions exercise the same sequence.

The restore timeline returns to topics at 1.5 seconds, parks the cursor at 1.8 seconds and holds the natural shared follower for 1.2 seconds. Its last natural hold and canonical endpoint have identical measured pane positions. Canonical root boundaries omit lesson-specific media, giving the same handoff between every lesson.

The physical preview uses the canonical Beat1_Leistung excerpt captured during this task. Each finite presentation capture owns a defensive catalog snapshot, so other authors can continue adding videos while its phases remain consistent. The common root handoff is one actual shared-UI screenshot reused under a key covering its catalog, locale, dimensions, state and UI source identity. This guarantees exact joins despite subpixel backdrop-filter paint variation across repeated Chromium captures. Both retained films decode completely. Navigation first/last PNGs and physical encoded first/last frames are pixel-identical. See [media verification](./layered-preview-verification.md), [tests](./layered-ui-tests.md), [implementation](./layered-ui-implementation.md), [visual audit](./layered-visual-audit.md) and [code audit](./layered-final-code-audit.md).

## Production Changes

The presentation catalog and schema now describe nested topics and videos, explicit English/German labels, row-major cells and separate focus/restore timing. A pure hierarchy module resolves video ownership. The Python rendering adapter measures actual DOM controls before authoring pointer routes and drives the real shared browser clock. Existing finite source video/audio composition, cancellation and guarded publication remain in place. Publication certificates include the captured catalog payload, so later edits cannot be mistaken for the data that produced an earlier render. The obsolete flat renderer, its layout API and its unused cursor timeline were removed. Actual browser/CSS/font/course-style dependencies participate in source provenance. No shared framework component or new runtime dependency was added.

Temporary generated evidence was removed after final verification. Input scripts, permanent portable fixtures, reports and the retained preview media remain available.
