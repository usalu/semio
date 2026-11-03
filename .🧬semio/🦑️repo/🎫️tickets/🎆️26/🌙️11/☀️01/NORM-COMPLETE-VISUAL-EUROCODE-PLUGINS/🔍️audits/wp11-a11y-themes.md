# WP-11 Theme, colour maps, a11y, devices, i18n audit (A11)

Status: code and tests written, NOT yet compiled or run. The only build started
(`repo run @semio-s/plugin-norm-structural-scene:test`) sat in the capacity queue for over an hour behind
other agents' `projects` jobs (6.4 GB each). The app-surface check/test was not started (one run at a time).
Log: `🗑️generated/wp11/scene-test.txt` (running build; delete when done).

## Delivered
- `NORM/🎬️structural-scene/🎨️colour-maps/`: `🦀️.rs` (Variant, ResultField, Cue, `utilisation_rgb`, `utilisation_cue`, `scalar_rgb`, `legend`), `🌈️ramps/🦀️.rs` (cool, warm, diverging, failure x light/dark/high-contrast), `👁️metrics/🦀️.rs` (WCAG contrast, CIE76, Machado CVD), `🔮️oracles/🐍️.py` (stdlib-only CIE/WCAG/Machado; colorspacious cross-check only if installed, not installed here), `🧪️tests/{🔬️unit/🦀️.rs,🥒️.feature}`. Mounted in `NORM/🎬️structural-scene/🦀️.rs` as `pub mod colour_maps`.
- Thresholds (palettes pre-tuned in a Python prototype): contrast >= 3 (light/dark), >= 7 (high contrast), diverging centre exempt; adjacent CVD dE >= 2, ends >= 30 (failure ramp >= 15); failure vs safe end >= 15 under all three CVDs. Cue pairs (glyph, hatch) pairwise distinct incl. unverified (dots, `?` glyph).
- `SURFACE/🪟️visual-windows/♿️accessibility/`: `🦀️.rs` (A11yPrefs -> Variant, reduced motion), `📣️live-region` (evaluation and utilisation-change announcements EN/DE, coalescing, assertive-first, overflow summary), `⌨️focus-order` (outliner, viewport, properties, checks, diagrams; never trapped; 28 shortcuts, unique, bilingual), `🌐️i18n-census` (scanner + whole-plugin gate), `🧪️tests`.
- `📱️device-profiles/🦀️.rs`: desktop/mobile/tablet profile (targets 24/44 px, handle mode, budgets, debounce, layout agreement with `visual_layout`, `classify`).
- `🎨️themes/`: `🧬️schema/{🦀️.rs,🛰️.proto,🟦️.ts,🔗️.graphql}` (ThemeBundle), `🦀️.rs` (built-ins, `validate`, `accessibility_findings`), tests.
- Mounts applied (see `📌️important/mounts.md`): `accessibility`, `device_profiles`, `themes` in `SURFACE/🦀️.rs`.

## Open risks for the first compile
- i18n census gate may report existing strings of other agents (empty en/de, or identical English prose); fix at the source, not by weakening the census.
- Test oracle thresholds may need palette tweaks if `check`/`test` disagree with the prototype.
- Oracle test options: `python3 -I`, no third-party packages needed.
- Accessibility oracle for focus/live-region is `std`/`serde_json` only (no third-party a11y library).
