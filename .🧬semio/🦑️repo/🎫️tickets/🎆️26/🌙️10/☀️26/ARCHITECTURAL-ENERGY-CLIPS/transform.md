# Tank Fill Transform Attachment

The overview preview exposed a live fill detached from its tank: the yellow rectangle appeared near x = 227 px while the transformed frame appeared near x = 340 px.

The updater captured the original numeric bottom anchor, width and inner height. Scaling and moving the frame/cap group therefore changed the visible tank while each redraw recreated the fill at its original coordinates and dimensions.

The existing `energy_tank` interface now derives horizontal and vertical scale from the live frame on every update, then places the fill relative to the frame's current bottom edge. The inset and minimum empty-level height scale with the frame. The returned dictionary and frame/cap group are unchanged.

## Validation

The new language-neutral `🔣️transforms.json` cases cover the reported 0.85 scale and move, independent horizontal/vertical stretching, and full/half/empty tracker levels. Each result is compared with an independently constructed Manim Rectangle oracle and checked for containment inside the transformed frame.

Before the fix, all six transform/level combinations failed. After the fix, the new transform test and all twelve existing shared architectural geometry tests passed: 13 tests in 5.799 seconds.

A real Manim render of the retained `tank-preview.py` input showed the translated/scaled 100%, 50% and 0% fills inside their respective frames. The rendered proof was inspected at `🗑️generated/tank-preview/images/tank-preview/TankTransformPreview_ManimCE_v0.22.0.png`. Generated imagery remains disposable under the ticket generated directory.

## Files

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️transforms.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️transforms.json`
- Ticket input `tank-preview.py`

The canonical test entry should include `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️transforms`. This fix covers translated, scaled and stretched upright tanks, matching the reported building composition.

## Overview Visibility Diagnostic

A later same-path image preview appeared to lose the right house. This was not reproduced in actual renders: dynamic-tank, frozen-tank and disabled-cache controls retained all 42 house family members and the same right-house material raster before/after window animation. The exact original 1600 × 900 overview also rerendered correctly in its existing cache directory. The auditor copied the unchanged PNG to a unique path and confirmed that the apparent disappearance was a stale image-tool display cache; no additional shared-source fix was made.

A default-cache 1600 × 900 raster regression now renders the live plant tank beside a separate house for two seconds, opens its nested window sashes, and checks family retention, opaque roof/wall/deck/floor probes and cyan window pixels. The transforms module's two tests passed in 6.360 seconds. The diagnostic input is retained as `overview-diagnostic.py`; raster/family evidence is generated under `overview-diagnostic`.
