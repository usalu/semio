# React fixes

## 1. Failing react tests

- `shared-borders`, `range-span` and the `detail-sections` window-chrome cases passed on the first run: no chrome regression remained (nothing to fix in the framework).
- The remaining failures were one race, in two tests: the export panel calls `save` before it renders the `[data-result]` notice, and `detail-sections` ("runs with progress, saves the result and reports it") and `model-downloads` ("runs a 3D export ...") read the notice right after `save` was called. Both now `waitFor` the notice. Source is correct; tests were racy.

## 2. Floor plan yaw and mirror

- `🖼️stage/🟦️.tsx`: new `planTurn(yawDeg, mirrored)`; in the plan each figure sits in a wrapper turned about its ground point: `rotate(-yaw deg)` (positive yaw to the left, up the page, as in 3D) after a `scaleY(-1)` lateral flip for mirrored placements. Wrapper carries `data-plan-turn`, `data-yaw-deg`, `data-flipped`.
- `🧪️playground/🟦️.tsx`: `mirrorAllowed` is now true in the plan; English and German mirror notes updated (side elevation and floor plan).
- Test `🧪️playground-tabs` "turns a figure about its ground point ..." plus a scenario in its `.feature`.
- Docs: `playground.md` Known limits, README "Playgrounds" section.

## 3. Results

- react: 270/271 pass; the one failure is `catalog-loading` "loads the authored archive ...": `electric-kick-scooter` (another agent's in-flight asset) is invalid (`fork-crown` lacks `tone`, plus a `lateral-ambiguous` issue). Not in scope; it passed earlier in the session before that asset changed.
- site: 9/9 pass (3 files).
- tsc `--noEmit`: react and site clean.
