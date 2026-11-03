# Render audit R1: heating and cooling clips

Scope: every scene of `🔥️heating/🎬️clip/**` and `❄️cooling/🎬️clip/**` that draws staffage (24 scenes). Each rendered alone (`--jobs 1` equivalent: one manim process at a time), `-ql --fps 15`, 4 frames per scene (contact sheet), plus crops. Sheets are kept in `🖼️r1-sheets/` (`<tag>.png`, 2x2 frames, time stamp top left). Scripts: `r1_chain.sh`, `r1_frames.py`, `r1_list*.txt`. `🗑️generated/r1` is deleted.

Tags: h = heating, c = cooling. Verdict is after fixes; the sheet is of the final render unless noted.

| Scene | Sheet | Verdict | Findings and fixes |
|---|---|---|---|
| heating internal-gains Beat1 | `h-ig1s.png` | fixed | Laptop hovered above the desk (it was anchored on the desk top, which includes the monitor), "Geräte" label touched the wall. Laptop now sits on the desk surface (0.72 m) right of the monitor, label moved left of the desk. |
| internal-gains Beat2 | `h-ig2.png` | ok | Worker grounded, heat plume and ripples at chest, typing plays. |
| internal-gains Beat3 | `h-ig3r.png` | improved | Rack is now the `power-up` clip (cold, dark grey -> warm, `rate` 30 set when "Strom wird Wärme" starts), replaces the held pose; `Indicate` replaced by a `Flash` (recolour would hit the thermal figure). Narration "rack equipment" matches the rack. |
| internal-gains Beat4 | `h-ig4r.png` | improved | New cast `heating-internal-gains-pendant-lamp-warm-up` (`switch-on-warm-up`, `rate` 12): lamp starts grey and glows while Phi_l counts to 60 W. |
| internal-gains Beat5 | `h-ig5r.png` | fixed | Icons were small: person unit 0.65 -> 0.8, laptop 1.8 -> 2.4. |
| fundamentals Beat1-5 | `h-fu1..5.png` | ok | Occupant grounded and breathing, radiator thermal (see A11 note). Labels clear. |
| conduction Beat4 | `h-co4r.png` | fixed | Toaster was cold (white) at `rate` 1 although "1000 W". `TOASTER_RATE = 8`: it heats and pops during the beat. |
| conduction Beat5 | `h-co5.png` | ok (A11 note) | Radiator in the house is tiny, wall stub (A11). |
| calculation ReviewingHeatGains | `h-ca3r.png` | fixed | Desk (width 1.6 m) stuck through the right wall, laptop floated. Desk, laptop and person now share one metre scale, desk right end at the inner wall, laptop on the surface. |
| calculation UltimateEnergyBalance | `h-ca5.png` | ok | Person, lamp, radiator (appears at 8 s) read; scale consistent. |
| calculation AnlagenVerluste | `h-ca6s.png` | fixed (label last tweak not re-rendered) | "beheizt, 20 °C" overlapped the head; "30 m Rohr" overlapped "Wärmeerzeuger" and "Speicher", chimney crossed the label. Labels moved ("Wärme-/erzeuger" right of the boiler, "beheizt" left, "30 m Rohr" size 12 at the left pipe). Last nudge of "30 m Rohr" (it touched the tank cap) is not re-rendered. Boiler, pump, tank, radiators are M9/A11. |
| solar-gains Beat5 | `h-so5.png` | ok | Blind lamellae tilted, rays hit lamellae (F1 result holds). |
| cooling heating-comparison Beat1 | `c-hc1r.png` | fixed | Laptop hovered left of the desk (cast origin is the desk's left end, not centre). Now at `desk x + 1.15 unit`, on the desk. Toaster on table, lamps, fanning occupant fine. |
| heating-comparison Beat2 | `c-hc2.png` | ok | Same house, overheated state, fanning occupant. |
| summer-strategy Beat1 | `c-ss1.png` | ok | Fanning occupant grounded, external blinds beside the windows. |
| cooling internal-gains Beat1 | `c-ig1r.png` | fixed | "Gewerblicher Arbeitsbereich" touched the right wall; moved left. |
| internal-gains Beat2 | `c-ig2r.png` | fixed | Runner and walker heat ripples were 1.3 m and 0.3 m left of the body (cast origin offset); `_HEAT_M` and `_ACTIVITY_CENTER_M` corrected, figures now centred on their labels. 50-person grid fine. |
| internal-gains Beat3 | `c-ig3t.png`, `c-ig3t-usage.png` | fixed | Usage panels used the side-on desktop monitor (unreadable thin L) and a closure bug: the replacement icons used the last panel's `cx`, so lit icons all landed in "Besprechung" and the other panels lost them. Now closed -> open `laptop` casts (cross-fade) with each panel's own centre: 7/10/3 lit. Casts `monitor`/`monitor-off` removed, `laptop-closed` added. Rack and desk with laptop read. |
| internal-gains Beat4 | `c-ig4.png` | ok | Cards with person, laptop, lamp. |
| internal-gains Beat5 | `c-ig5r.png` | fixed | "Gerät" label sat on the worker's head; shifted right. |

## Checks across all scenes

- Grounding: all standing, seated, walking and running figures stand on their floors; desks and racks too.
- Sizes: persons 1.75 m, desks 1.6 m, rack 2.0 m scaled with one metre per scene; icons in cards differ on purpose (they are symbols).
- Thermal colours read on the #001117 background; warm-up and typing/breathing/running clips change between frames (compared frame sequences, e.g. runner).
- No `person_glyph`, `seated_person_glyph`, `object_silhouette`, `lamp_glyph` or `radiator(` call remains in either series (grep); the rectangles left in summer-strategy are block-diagram boxes.
- Narration vs visuals: heating Beat3 rack matches "Serverrack"; cooling Beat3 "Notebook, Serverrack, Leuchte" matches.

## Warm-up transients

Used: server rack `power-up` (heating Beat3, rate 30), pendant lamp `switch-on-warm-up` (heating Beat4, rate 12). Toaster `toast-cycle` rate 8 (conduction Beat4). Not used: laptop and monitor warm-up clips. The seated worker already carries a typing laptop and the monitor asset is side-on only, so they would add nothing to the narrative.

## Notes for M9 and A11 (not fixed here)

- A11: `panel-radiator` is a section with a wall stub: in `fundamentals` Beats 1-5 it stands free in the room (stub reads as a partition), in `conduction` Beat5 and `calculation` it is tiny and sits on the slab. Wanted: front viewpoint.
- M9: boiler, tank, pipes, pump in `AnlagenVerluste` are hand-built (labels around them moved); in `conduction` Beat4 stray heat arcs hover where the tealights (now replaced by someone) were.
- Open: `desktop-monitor` has no front viewpoint, so it cannot serve as a monitor icon.

## Tests

Pass (Python, repo cache venv): staffage bridge, `staffage_only`, `scenes`, `section_consumers` (49 tests), cooling clip tests, cooling internal-gains tests. Heating clips have no own test module. Vitest not run.

## Files touched

`V/🧍️staffage/🔣️.json` (rack clip, lamp warm-up cast, laptop-closed, monitor casts removed), heating `👥️internal-gains`, `🧱️conduction`, `🧮️calculation`, cooling `⚖️heating-comparison`, `👥️internal-gains` (+ its `🧪️tests/🔣️.cases.json`), all `🐍️.py`.
