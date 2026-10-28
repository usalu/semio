# Right Chip Browser Verification

The portable bilingual layered fixture declares a bottom right View/Ansehen chip containing maximize-2 and one top right Restore/Wiederherstellen chip containing minimize-2. The independent shared component Chromium oracle explicitly configures the teaching variant and separately preserves the shared default footerLeft/chevron-right behavior.

## RED

Before production changes, `LayeredBrowserOracleTest.test_production_two_levels_match_shared_glass_and_exact_hover_placement` failed in both English and German: actual chip x was 6.390625; required x was greater than the card midpoint 266.6640625. One test ran in 44.832 seconds and reported two assertion failures.

## Assertions

- Actual topic and video chips occupy the right half of their cards, retain the shared glass styles, and contain exactly maximize-2.
- Actual focused right caps contain one control with the localized restore label and exactly minimize-2. The full control fills the cap and aligns with the window right edge.
- Generic square icon-only geometry tests remain unchanged; focused teaching browser parity verifies the wider fused label control.
- Existing actual pointer hover, press, release, cancellation, and independent shared pan/reveal/open/Escape-return checks remain active.
- Added actual native restore press/release/click assertions in English and German.

## GREEN

The first focused browser batch passed four tests in 128.201 seconds: native hover/press/release/cancellation at both levels; production bilingual placement, icon, fused-cap and style parity; independent shared teaching pan/reveal/open/return; and focused browser geometry plus pixel parity.

Two generic geometry regressions passed in 0.010 seconds: canonical square cap/control geometry and small-gap/title-cap silhouette normalization.

The final batch passed two tests in 59.198 seconds: actual fused restore hover/press/release/click returns to the video overview in English and German, and the independent shared oracle preserves the default footerLeft/chevron-right variant while validating the teaching footerRight/maximize-2 variant.

All eight focused test invocations passed after the bilingual RED baseline. No full encoded clip regression was repeated for this narrow placement and icon change.

A source audit found no stale left-position teaching assertions; the remaining left-position assertion exclusively protects the shared default variant.

## Owned Files

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🥞️layered/🟨️.mjs`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🥞️layered/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🐍️.py`
- This ticket report.
