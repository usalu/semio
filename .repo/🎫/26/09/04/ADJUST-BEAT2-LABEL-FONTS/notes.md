# Adjust Beat2 Label Fonts

## Fonts
- Beat2 `Solare Gewinne (Exzessiv)` + `Raumtemperatur` → `LABEL_FONT_SIZE`
- Beat3 `Raumtemperatur` → `LABEL_FONT_SIZE`

## Shared layout
- `HOUSE_CENTER = ORIGIN + DOWN * 0.75` (was `DOWN * 0.45`) — applies to all beats via `_build_cross_section_house`
- `THERM_OFFSET = RIGHT * 3.55 + UP * 0.1` (was `RIGHT * 2.9`) — Beat2 + Beat3 thermometer slot clear of side exhaust
- Beat3 upward exhaust shortened (`UP * 0.85`, slightly less lateral) so cyan plumes clear the topic title
