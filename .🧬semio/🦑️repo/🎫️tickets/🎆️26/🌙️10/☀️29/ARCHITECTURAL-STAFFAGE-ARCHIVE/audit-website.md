# Website Accessibility, Usability and Honesty Audit (haiku-reader, read-only)

R = `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`, M = `R/🔨️modules`.

## High
1. Settings panel portal at end of body, focus not moved, `aria-controls` dangling (`app:86-113`).
2. Failure states missing: All assets / collections / category show "Loading…" forever on failure (`M/🗃️browse:202,261`, `M/⭐️collections:151`; `M/📚️catalog:96,174` set `failed`).
3. Counts overstate authored assets ("Assets: N" from manifest, `M/🏠️overview:129,171`).

## Medium
4. Stage keyboard help on non-focusable group (`M/🗺️stage:119-123`). 5. `touch-none` and wheel `preventDefault` trap scrolling
(`M/🗺️stage:80,131`). 6. Playback time never announced; play button label+aria-pressed (`M/⏯️transport:45,51`). 7. Clip timeline
focusable group without slider semantics (`M/🔬️detail/🎞️clips:91`). 8. Heading outline skips h2 on asset page. 9. Number fields
reject typed values (`M/🪟️chrome:121-134` and callers). 10. Copy markup without feedback (`export-panel:121`). 11. Editable fallback
input with no-op onChange (`M/🧪️playground:324`).

## Low
12. EN/DE accessible names. 13. Language gate vs WCAG 3.1.1 (by design: no default language). 14. External links without new-tab
cue. 15. Implementation jargon in user text ("Rust writer", "WebAssembly"). 16. Slugs / `String(error)` shown. 17. Hard-coded frieze
ids. 18. Compare card loading state. 19. URL gaps: export options, rate/loop, playground zoom. 20. Focusable `role="img"`, arrow keys
consumed on static figures. 21. Possible low contrast of helper text on dark stage.

## Verified passing
Native buttons, roving radio groups, keyboard playground, Escape/focus return, skip link, `:focus-visible`; figures with label and
hidden value table; status regions; landmarks; reduced motion; en/de bundles with key parity and no default language; honest DWG /
video capability reporting; empty/unsupported/unavailable states; no inert controls (except 11); responsive breakpoints; rich URL state.
