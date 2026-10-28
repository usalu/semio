# Continuous Virtual Teaching UI

Replace the institute intro with a virtual application: persistent navbar, canvas of five lesson windows, and footer. The curriculum order is physical understanding, ventilation, heating, cooling and energy balance. Each full lesson zooms from the same canonical overview into its window and returns to that exact overview. Overview thumbnails, navigation and footer remain independent of active lesson/progress so decoded end/start frames match.

The new presentation contract is schema-first, localized explicitly in English/German, and bound to canonical semio colors, typography, spacing, square WindowSilhouette outlines and fused title chips. Primary remains the generic accent. Native Manim draws chrome and zoom transitions; the existing tool interface composes authored content and audio with FFmpeg. This avoids altering scientific content geometry or introducing dependencies.

The source body uses its full authored frame: cropping the upper17.5% would discard section context and DIN citations. Shared chapter/beat headings become compact left-aligned tabs instead of the current large centered title placement. The outer window cap identifies the lesson. Remove the old intro from curriculum/schema/plans and remove its source entry points, rather than supporting both presentation systems.

Root owns runtime, manifest/schema, title helpers and wiring. High execution agents own presentation implementation and portable/native tests; low audit agents inspect canonical chrome and independently review continuity. All four slots are used while work can run independently. Tests include pure layout/schema cases, independent native geometry checks, real short synthetic audio/video composition and decoded boundary equality across all five lesson windows. A focused review film will demonstrate the physical-to-ventilation handoff without rerendering the full curriculum.

## Concurrent Work

The separate audience implementation added speaker/spectator variants while this work was in progress. Its required audience arguments, per-variant directories, subtitle policy and fixture changes are preserved. The virtual UI envelope accepts the extra setting and fingerprints it. The review runner explicitly selects spectator when capturing authored captions. Scientific body rendering also encountered a transient person-outline NameError in concurrent geometry work; the preview uses current physical power and ventilation volume scenes and leaves that separate implementation untouched.
