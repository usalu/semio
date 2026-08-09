# Center Cooling Animation Layout — Summary

Ticket: `26/08/08/CENTER-COOLING-LAYOUT`  
Follow-up to `ORGANIZE-COOLING-TEMPLATE`.

## Done

### Remaining internal-gains beats (was only in `merged_scenes.py`)
Migrated into `scene_2.py` with five-rule template + mid-screen anchors:
- `Beat5_SensibleVsLatent`
- `Beat6_HeatTrap`
- `Beat7_HvacCooling`
- `Beat8_Mitigation`

Updated `generate_audio.py` + `build_full_video.py` for Beat5–8.

### Mid-screen repositioning (animations unchanged)
- `scene_1.py` — house at `ORIGIN + UP*0.15` (horizontally centered)
- `scene_2.py` — Beat1 facade mid; Beat6–8 rooms near ORIGIN
- `scene_3.py` … `scene_6.py` — `CONTENT_CENTER` / mid-band shifts; formula/caption slots untouched

Preference applied: main animation in the **middle band**; title top; formula + German caption reserved bottom.

## Smoke
Stills under `smoke/` and `media_s2_rest/` — Beat1 house centered; Beat6 heat-trap mid-screen.
