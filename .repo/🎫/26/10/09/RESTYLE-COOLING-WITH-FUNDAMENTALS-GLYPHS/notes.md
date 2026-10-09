# Restyle Cooling Video with Physical Fundamentals Glyphs

Goal: `R26-02/UPDATED-DOCS/UPDATED-USER-DOCS/UPDATED-TUTORIALS`. Repo MCP unavailable (`CONNECTION_CLOSED`) — ticket kept by hand.

Dev request (2026-10-09): revise the Cooling video using the Physical Fundamentals objects and animations — animated energy flow, house shape, person, lamp, sun, sun radiation — instead of the Cooling ones. Only object visualisation and animated effects change; the content explained stays. Everything animated well; every scene self-reviewed (positions, text, no overlaps, animations synced with subtitles); then deliver the full video, like the Physical Fundamentals output.

## Shared vocabulary

`tutorial/manim_visuals.py` `#region Scene glyphs` — PF helpers moved verbatim, public names:
`paced_flow`, `color_ramp`, `beam`, `mirror`, `sun_rays`, `shine`, `ripples`, `beam_pulses`, `window_glyph`, `open_window`, `house_section`, `room_section`, `radiator`, `sun_glyph`, `moon_glyph`, `person_glyph`, `cloud_glyph`, `cloud_trail`, `clock_glyph`, `thermometer_glyph`, `energy_tank`, `lamp_glyph`, `droplets`.

Mapping Cooling → PF:
| Cooling object | PF replacement |
|---|---|
| line-art house (Teil 1, 3) | `house_section` (wall thickness, eaves, sash windows, ground hatch) |
| stick figures / occupant icons | `person_glyph` (seated / activity poses in the same line style) |
| bulb dots, ceiling fixtures | `lamp_glyph` |
| sun variants | `sun_glyph` |
| wavy solar rays, beam wedges | straight parallel `sun_rays` + `shine` + `beam_pulses` |
| sine heat plumes, convection ribbons, red heat blocks | `ripples` (long-wave heat from warm sources), `paced_flow` particles |
| custom thermometers / clocks | `thermometer_glyph` / `clock_glyph` |
| line rooms | `room_section` (slabs, glazed wall) |

## Progress

Shared additions in `manim_visuals.py`: `flow_animation` (particle stream as one animation), `pulse_flashes`, `ripples(facing=…)`, `seated_person_glyph`, `hold_for(during=…)` so heat, light and air keep moving while a subtitle is read, `house_section(scale=…)`. Layout guard extended (lines through labels, objects under text, objects touching formula/caption boxes, captions over two lines).

Per part — every beat rendered with `LAYOUT_CHECK=1` until the guard is silent, then reviewed on contact sheets in `checks/sheets/`:

- **Teil 1** (`t1_*`): `house_section` + `sun_glyph`, straight parallel rays through both windows with pulses, person/kitchen/desk/`lamp_glyph` gains with ripples, `thermometer_glyph`, Lüftungsgerät with spinning fan and particle air paths.
- **Teil 2** (`t2*`): `room_section` offices, `seated_person_glyph`, activity ladder as person poses with ripples, devices on particle cables, `lamp_glyph` lighting with rays, ripples and pulses; summary cards with person/laptop/lamp glyphs.
- **Teil 3** (`t3*`): `house_section` lifted to clear the formula box (`t3v2.png`); parallel rays, envelope ripples facing inward, `clock_glyph` time lag, open sash with warm/cool particle streams and `droplets`, `thermometer_glyph` sensible/latent column.
- **Teil 4** (`t4*`, `t4v2.png`): facade → `house_section(scale=0.78)` with sun lowered so rays reach into both storeys; frame factor with parallel rays from `sun_glyph` and pulses through the glass (F_F readout moved right of the window, off the rays); shading section with pulses on direct / reflected / residual rays; glazing section with `sun_glyph`, ripples for q_i (kept off the build-up note); solar load in `room_section` with rays landing on the floor and rising ripples.
- **Teil 5** (`t5*`, `t5v2.png`): `room_section` with RLT unit (spinning fans, cooling coil), ducts and particle supply/exhaust, heat ripples fading as the room cools; volume-flow room with continuous cool particles (room lowered so the Δθ card clears the subtitle); duct particles spread over separate start points and off the `A` label, faster/orange for small A, slower for large A.
- **Teil 6** (`t6*`, `t6v2.png`): `_room` = PF slabs and walls on the old outline, `_person` = `person_glyph`; Beat1 house → `house_section(scale=0.8)` with insulation outline, slats, ripples and single-sided window flow; ripples replace the sine "stuffy"/warm waves; every hold keeps air moving (window dial, cross flow, stack flow incl. faster flow after the shaft grows, night sweep raised off the mass label, free-ventilation exchange, three fan types with spinning fans, heat-exchanger streams, DEC streams with turning wheel); `sun_glyph`/`moon_glyph`/`droplets`; intro holds moved after the build so drawing happens under the intro subtitle; `ungenutzt` tag pinned between frame edge and wall.

- Full video: `full_cooling_video.py -q h --no-play` (log `checks/full_cooling_qh.log`) → `rendered/Full_Cooling_Demand_1080p60.mp4`, 1920×1080 @ 60 fps, 28:41, silent; frames every 72 s checked in `checks/sheets/hq_full_*.png`.

Status: closed 2026-10-09 (manually — repo MCP unavailable).

## Reopened 2026-10-09 — hall badge text too small

Dev report (screenshot at 4:05): the `Gesamtwärme 5000 W` badge under the lecture hall was unreadable — `watt_anchor` stacked vertically and scaled to 0.5 to fit the gap above the caption.

- `watt_anchor(row=True)` lays title, value, device glyph and comparison in one line; the hall badge uses it at full size (value at formula size, labels at label size) below the hall.
- Verified: `LAYOUT_CHECK=1` render of `Beat2_HumanFactor` silent; frame `checks/sheets/t2_hall_anchor.png` reviewed — clear of the hall and the caption.
- Full video re-rendered at 1080p60 (`checks/full_cooling_qh_v2.log`).
