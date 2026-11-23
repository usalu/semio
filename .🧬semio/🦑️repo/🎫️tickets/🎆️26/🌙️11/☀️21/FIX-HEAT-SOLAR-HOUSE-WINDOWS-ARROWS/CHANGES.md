# Solar House Windows And Hide Flow Arrows

## Problem
1. House casements on light section walls used white joinery, so only a thin cyan sash stroke read — looking like mid-wall sticks.
2. Solar Beat1 drew cyan `flow_guides` path arrows under the heat-loss particles.

## Fix
1. `emphasize_casement` / `house_section`: joinery follows sash cyan (readable on light walls); outer-leaf flush unchanged.
2. Solar loss guides: `visible=False`; no FadeIn/FadeOut of the arrows (particles keep using the paths).

## Tests
House joinery color matches sash; solar source requires `visible=False` and forbids `FadeIn(loss_guides)`.
