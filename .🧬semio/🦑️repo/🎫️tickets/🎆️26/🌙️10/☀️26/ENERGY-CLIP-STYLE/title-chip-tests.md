# Title Chip Tests

The portable layered fixture now places the restore action in the window title chip. It specifies an icon-only minimize control, one native button, no right cap control, the actual localized window title, and no restoration from clicking the name.

The Chromium-backed production tests verify English and German title text, one embedded minimize icon and native button, containment of the icon button and SVG in the same visual title chip, preservation of open state while clicking the name or holding the icon, and restoration only when releasing the embedded icon. The opened title checks are complemented by the overview extension below, which places each maximize icon in its name chip.

The production geometry parity test compares the title cap against shaped title width plus the canonical control button and gap. The generic window geometry helper remains unchanged. Body geometry and independent raster pixel checks remain in the production parity test.

## Validation

- RED: The native bilingual test failed before the UI change because the separate control displayed Restore and Wiederherstellen instead of the window title. Both locale subtests failed in 49.528 seconds.
- GREEN: All three focused Chromium-backed methods passed in 152.243 seconds: the native bilingual interaction, both production overview levels, and production geometry plus pixel parity. English and German name-only clicks kept the video open; the embedded minimize SVG restored the overview only on release.

## Files

- presentation/🌐️browser/🧪️tests/🥞️layered/🔣️.cases.json
- presentation/🌐️browser/🧪️tests/🐍️.py
- presentation/📐️geometry/🧪️tests/🐍️.py

## Overview Title Icon Extension

The portable overview contract now places maximize in the actual name chip and leaves the View/Ansehen footer text only. The generic shared default keeps its left footer chevron. Both topic and video native pointer tests now target the actual header maximize SVG rectangle.

RED completed in 8.973 seconds: the generic shared default passed, then the Chromium oracle rejected the former footer maximize icon against the new empty footer icon list. Production changes were authorized after that failure. GREEN passed after the shared and teaching UI updates.

The final focused run includes native topic/video opening at the actual name-chip SVG bounds, bilingual opened-name versus embedded-icon semantics, production overview parity, and independent production geometry/pixel parity. The opened title cap uses the canonical 22.4px control button plus the 3.2px gap. The interaction and production methods passed; the final geometry result is recorded below.

The four-method extension run completed in 215.261 seconds. The portable overview oracle and all native overview/opened interaction plus two-level production assertions passed. The independent geometry test exposed a 28.765625px title cap after the canonical 22.4px button was inserted into a vertically padded title chip; the expected cap is 22.4px. This regression was reported to the UI owner for correction before a focused geometry rerun.

Final geometry GREEN passed in 8.651 seconds against the frozen final source, after the UI owner removed vertical container padding and preserved the canonical 22.4px icon button. Independent title/body geometry, typography, border, and raster pixel checks passed. The earlier py-0 variant also passed in 11.344 seconds; the final run verified inline paddingBlock=0 plus title truncation and button shrink prevention. All focused contracts are now green.
