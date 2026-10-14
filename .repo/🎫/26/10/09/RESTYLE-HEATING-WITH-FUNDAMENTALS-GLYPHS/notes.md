# Restyle Heating Video with Physical Fundamentals Glyphs

Goal: `R26-02/UPDATED-DOCS/UPDATED-USER-DOCS/UPDATED-TUTORIALS`. Repo MCP unavailable (`CONNECTION_CLOSED`) — ticket kept by hand.

Dev request (2026-10-09): same as the Cooling restyle (ticket `26/10/09/RESTYLE-COOLING-WITH-FUNDAMENTALS-GLYPHS`) for the Heating video — Physical Fundamentals objects and animations (energy flow, house, person, lamp, sun, sun radiation) replace the Heating ones; only visualisation and animation change, not the content; every scene self-reviewed for positions, overlaps and subtitle sync; deliver the full video.

Approach: one agent per part (`1_introduction`, `2_conduction`, `3_convection`, `4_internal_heat_gain`, `5_solar_heat_gain`, `final_calculation`), each editing only its own scene file, verifying with `LAYOUT_CHECK=1` + contact sheets in `checks/sheets/<part>/`; the shared `manim_visuals.py` stays untouched by agents. Then a lead review of every sheet, the compose staleness fix, and the 1080p60 render.

## Progress

## All six parts restyled and reviewed (2026-10-10)

Six agents in parallel, one per part, each editing only its own scene file. Shared `manim_visuals.py` untouched. All parts went through: `LAYOUT_CHECK=1` guard silent across every beat/scene, dense contact sheets (every 3s) read by the agent and then independently re-reviewed by the lead.

- **Modul 1** (`1_introduction/scene_1.py`, 9 beats): house/room rebuilt from PF filled slabs, `person_glyph`, `moon_glyph`, PF `radiator()`. Heat/air motion converted to `flow_animation`/`ripples`/`pulse_flashes` driven through `hold_for(..., during=...)`. Abstract diagrams (funnel/layer-stack in Beat6, gauge panels in Beat7/9) correctly left alone. Lead review: all 9 beats clean.
- **Modul 2** (`2_conduction/scene_2.py`, 6 beats): `thermometer_glyph` columns, warm→cold particle flow through walls/slabs/tiles, `person_glyph`, PF-style thin device icons, `house_section` building envelope with `radiator`. Fixed mid-session: tile-flow particles crossing the "U" label (moved off the diagonal) — found and fixed by lead during review, then re-verified clean.
- **Modul 3** (`3_convection/scene_3.py`, 8 beats): `house_section`/`room_section` rooms, `open_window`/`window_glyph` sashes, `radiator`, `clock_glyph`, spinning `_fan` glyphs, continuous particle streams through every subtitle hold (replacing one-shot `animate_flows`). Lead review: all 8 beats clean.
- **Modul 4** (`4_internal_heat_gain/scene_4.py`, 5 beats): `house_section`/`room_section`, `seated_person_glyph`, `lamp_glyph`, `thermometer_glyph`, `moon_glyph`+stars, electricity/heat particles and ripples. Lead review: all 5 beats clean.
- **Modul 5** (`5_solar_heat_gain/scene_5.py`, 8 beats): `sun_glyph`/`moon_glyph`, straight parallel `sun_rays`/`shine`/`pulse_flashes` replacing wedge/band sunlight, `ripples` for secondary glass heat and thermal-mass charge/discharge, PF-style filled room slabs. Lead review: all 8 beats clean.
- **Final calculation** (`final_calculation/merged_scenes.py`, 6 scenes): `house_section`, `window_glyph`, `room_section`, `sun_glyph`/`sun_rays`/`shine`, `person_glyph`, `lamp_glyph`, `radiator`, `energy_tank`, `thermometer_glyph`; sun/moon crossfade over the 24h chart; pan-balance replaced by a PF house whose loss/gain tags morph into the master equation. One pre-existing (not-restyle-caused) overlap in `AnlagenVerluste`'s readout column fixed (font sizes 17/21 → 15/19). Lead review: all 6 scenes clean.

`full_heating_video.py` staleness check added (mirrors the Cooling fix): a section clip is only reused when newer than its beat modules, their `vo_timing.json` and the shared `manim_visuals.py`/`manim_fonts.py`/`full_heating_video.py` itself — so a stale clip from before the restyle can never sneak into the final render.

Full video rendering at 1080p60 now (`full_heating_video.py -q h --no-play`, log `checks/full_heating_qh.log`) → `rendered/Full_Heating_Demand_1080p60.mp4`.

## Closed 2026-10-10

Full video: `full_heating_video.py -q h --no-play` (log `checks/full_heating_qh.log`) -> `rendered/Full_Heating_Demand_1080p60.mp4`, 1920x1080 @ 60 fps, 35:12, silent; frames every 88s checked across the whole length in `checks/sheets/hq_full_heating_*.png` -- clean throughout.

Status: closed 2026-10-10 (manually -- repo MCP unavailable).

## Reopened 2026-10-10 — g-value scene too cluttered

Dev report (screenshot at 26:21 of the full video): Modul 5 Beat3_GWert ("Gesamtenergiedurchlassgrad g") was cluttered — ten parallel rays (one per 10% share) crossing a reflected-ray fan made the split hard to read. Asked to match the cleaner single-ray glazing graphic already used in the Cooling video (Teil 4 Beat4_GlassTransmittance).

- Replaced the ten-ray fan with one ray through a glass cross-section, same approach as `Cooling/4_solar_radiation/scene_4.py` `Beat4_GlassTransmittance`: a single beam from the sun hits the outer pane, splits into a reflected ray and a ray that continues through the glass and out the far side, with `pulse_flashes` running along both paths during every hold.
- Kept all of Heating's own content: the same percentages (reflektiert 20%, im Glas 30%, durchgelassen 50%, Wärme nach innen 10%/außen 20%), the same `g ≈ 0,6` brace spanning only the "innen" contributions (durchgelassen + Wärme nach innen), the same formula fly-in into `Φ = G·A·F_f·g`, the same pastel colours and narration.
- Removed the now-dead `RAY_SHARE` constant and the ray-fan math it drove; `_soft_ray`/`_pulses` stay (still used by other beats in this file).
- One guard hit found and fixed: the shallow incoming ray passed behind the "reflektiert 20%" label — moved the label below the reflected ray instead of above it.
- Verified: `LAYOUT_CHECK=1` over all 8 beats of `5_solar_heat_gain/scene_5.py` is silent. Dense contact sheet (`checks/sheets/5_solar_heat_gain/lead_gwert_v2_*.png`) reviewed frame by frame — clean.
- Full video re-rendered at 1080p60 (`checks/full_heating_qh_v2.log`); the staleness check reuses the other five sections and only re-renders Modul 5.
