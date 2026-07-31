# Fix introductionStep crash on Paneele complete

## Cause
Shell mid-migrated from `advance` to `interactions` while brand/types still used `advance`. Completing Paneele (and other by-doing steps) hit undefined `introductionStep` / missing `interactions`.

## Fix
- Regenerated/aligned `IntroductionStepDefinition` on `interactions` + `ordered` (dropped `IntroductionAdvance`).
- Brand Paneele: `interactions: [{ on: { kind: "panel", id: catalogue } }]` — no Next.
- Shell completes panel/expand/action/utility/tool via `completeIntroductionInteraction`.
- UIIntroduction already checklist-based; hardened `interactions ?? []`.

## Verify
- vitest brand introduction: pass (`vitest-intro-interactions.txt`)
- vitest UIIntroduction: see `vitest-ui-introduction.txt`
