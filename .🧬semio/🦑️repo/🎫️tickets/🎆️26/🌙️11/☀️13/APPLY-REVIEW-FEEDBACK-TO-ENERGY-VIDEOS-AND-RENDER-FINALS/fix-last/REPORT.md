# fix-last: leftovers before the final renders

Paths under `🎓️teaching/🏛️architecture/⚡️energy/`. Scripts in `TK/fix-last/`, generated output in `TK/🗑️generated/fix-last/` (to delete when the ticket closes).

## A. Ventilation
- `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` Beat2: the Q_V,lat winter/summer chip moved from y -1.46 to -1.27, its own slot between the eta gauge (above) and the formula box (below). Seen clean in the chapter preview render (frame at 1:18 of the scene, `🗑️generated/fix-last/mech4/m78.png`).
- `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` Beat4_Auftrieb: the root panel `panel_v` was taller than the others (stacked kg/m3 and m/s under and after the root, root radical below the row) and reached up into the room drawing and down into the caption box. Now: units inside the root are inline (`kg/m^3`), the result unit is inline `m/s`, the panel is set at 0.85 x formula size with edge buffer 1.86, and the scaffold band bottom is -0.98 (was the formula safe bottom -1.66, which did not lift it, the room sat on the panel top). All panels of the scene now clear the room by at least 0.2 units; the root panel clears room and caption (`🗑️generated/fix-last/nat6/a.png`, chapter preview render).
- Preview renders of Ventilation_04 and Ventilation_05 (`--profile preview --audience speaker`, private root `🗑️generated/fix-last/out`) completed; held frames looked at.

## B. Physics
- The new fraction readout "20 m2 . 0,133 kWh/m2 = 2,7 kWh" fits (right column, clear of the vacuum clock and the formula box), seen in a direct scene render of `Beat6_ThermischeMasse` (`🗑️generated/fix-last/phys1/s_01.png`).
- The reference chip "DIN EN ISO 13786 ... Wärmespeicherung" sat 0.14 under the room slab: moved from y -1.75 to -1.98 in `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (clear of the room, still above the caption band). Seen at 1:20 (`phys1/z80.png`).

## C. Shared tests
- `🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py` `test_physics_scene_constructor_bounds`: expected height is now `CASES["physical"]["radiator_height_m"] * metre` (0.65 m real); the bottom check became "not below the room floor and less than 0.1 m above it" because the elevation cast has a skirt gap of 0.057 units to the floor.
- `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`: the cooling sizing `load` caption pinned in en and de now reads "From part 4, for all windows of the house: ...". `regen_captions.py` (copy of the tests report script) found no other stale pinned caption, heating included.

## D. Posters
`script.ts posters --language de --profile preview --audience speaker` (script `TK/fix-last/posters.sh`, repo.exe, private root `🗑️generated/fix-last/posters-out`) run for ventilation, physical_fundamentals, cooling, energy_balance, heating. Result: ventilation 6 current, cooling 6 current, heating 6 current (already recaptured by the chapter renders and earlier runs, nothing stale); physical_fundamentals 1 recaptured (one Manim frame); energy_balance all 7 recaptured (incl. EnergyBalance_05_Primaerenergie). Stored assets: `<lesson clip folder>/🖼️posters/`, not committed. BIM/Grasshopper/Excel/Optimization posters left alone as instructed.

## Tests (preview profile, language en, SEMIO_TEST_MANIM=1)
- physics fundamentals: 13 OK
- ventilation architecture, numbers, transitions, volume: 21 OK
- visuals integration (9 tests incl. physics bounds and captions): OK (30 tests in the joint final run, all OK)

## Left
- Nothing in scope. The final renders can start. The sources of the touched scenes changed, so poster staleness was checked after the last edit.
