# Lower Scene1 House Layout

Ticket: `26/08/09/LOWER-SCENE1-HOUSE`

## Change

In `Cooling/1_heating_vs_cooling/scene_1.py`, shifted the default house center from `ORIGIN + DOWN * 0.15` to `ORIGIN + DOWN * 0.45` in `_build_cross_section_house`.

Sun, solar rays/labels, internal gains, watt anchors, heat block, thermometer, and exhaust streams are all positioned from house corners/`center`, so they move together.

## Validation

Layout-only constant change; no animation logic edited.
