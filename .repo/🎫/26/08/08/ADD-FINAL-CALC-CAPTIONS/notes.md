# Add Final Calc Captions

## Goal
Add German subtitle syncing to Heating final_calculation only — same pattern as other heating modules (`caption_bar` / `swap_caption` / `hold_for`).

## Change
`merged_scenes.py`: import caption helpers; add `NARRATION` + caption wiring to all five beats. Animations unchanged.

## Verified
Smoke-rendered all five scenes (`ReviewingHeatLosses`, `Scene2`, `ReviewingHeatGains`, `Scene4`, `UltimateEnergyBalance`). Caption bar present and synced; stage content unchanged.

## Fix
`FullFinalCalculationVideo` calls `scene_cls.construct(self)`, so `self.NARRATION` was missing on the full scene. Set `self.NARRATION = scene_cls.NARRATION` before each construct.
