# 📝️ haiku-consumer-audit: Viewer i18n, Accessibility and Tests (relayed by the coordinator)

This is a static read taken while sonnet-viewer was editing. `R` = `🎯️targets/⚛️react/🔨️modules`.

| Severity | Where | Finding |
|---|---|---|
| High | `R/🔬️detail/📤️export-panel:91` | `{representation}` and `{viewpoint}` placeholders are not passed, so the user sees them raw |
| High | `R/🔬️detail:75` | The compare panes have no `viewpoint`, which `ComparePane` requires; this is probably a typecheck error |
| High | `R/🔭️viewpoints/🪟️sheet:59` and `🔭️viewpoints:113-115` | The focus order (elevation, front, plan) does not match the visual grid (plan top-left, elevation bottom-left, front bottom-right); WCAG 2.4.3 |
| Medium | `🔠️english/🎛️viewer:7-9`, `🔭️viewpoints:22-24`, `🗣️labels:48` via `viewpointCaption` | English uses three vocabularies: "Side elevation", "Side view" and "Side" |
| Medium | `🔭️viewpoints:36-46,64-68,122,143` | The new strings live outside the i18n bundles, and `FAILED` concatenates the raw `error.message` |
| Medium | `🎚️selector:34-35` | An unavailable option shows no reason, neither inline nor in its tooltip |
| Medium | viewer | There is no live region that announces a change of viewpoint or layout |
| Medium | `🌐️i18n/*/🎛️viewer:6-8` | `viewer.viewpoint` is unused |
| Low | `🔭️viewpoints:39` | The accessible name is lowercase while the visible label is capitalised |
| Low | sheet | The focus button has no named pressed state; `aria-description` support varies across browsers; several `role="status"` cells announce at once |
| Open | `📤️export-panel:86`, `🧪️playground` and its export, `🎛️viewer:110,135`, `📤️downloads:102-105` | The snippet has no viewpoint; the playground has no viewpoint; the scene goes stale; filenames have no viewpoint |

**Tests.** No React test covers viewpoint. The tests at `🧪️tests/📤️downloads:32,43,85,97` pin filenames without a viewpoint. The radio-group tests in `📏️range-span` and `📖️detail-sections` can serve as patterns.

**Already correct.** The selector uses a radiogroup with roving tabindex and arrow keys. Invalid `vp` and `vl` values in routing are reported, not silently replaced. The German grammar is correct, and the German bundles are typed against the English keys.
