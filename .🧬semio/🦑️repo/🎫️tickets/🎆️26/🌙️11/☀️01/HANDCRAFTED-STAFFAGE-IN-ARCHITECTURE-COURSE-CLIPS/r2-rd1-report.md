# r2-rd1: demand `🧪️practice` and `🏭️primary-energy` staffage render check

D = `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`.

## Rendered and viewed (quality l, 15 fps, one scene at a time, frame sheets plus zoom crops)

practice Beat1, Beat2, Beat3; primary-energy Beat1, Beat2, Beat3, Beat4. Beat1 and Beat2 of primary-energy have no changed staffage beyond occupant/radiator/tree icon and read fine.

## Findings and fixes

1. `V/🧍️staffage/🔣️.json` `demand-practice-radiator` had no `viewpoint`, so it was drawn side-on (thin slab, 0.07 wide) in the practice house section (Beat1/Beat3) and in the Beat2 room row, while primary-energy already used `front`. Added `"viewpoint": "front"`. Re-render: radiators are frontal with ribs, sit on the floor line, supply/return pipes still meet them (x_in/x_out come from the cast bounds).
2. `D/🏭️primary-energy/🐍️.py` `_boiler`: the boiler was 0.62 high (about as tall as, or taller than, the standing person: 1.0x in Beat1, 1.3x in Beat3) and floated above the ground. Now `_boiler(base, h)`: foot on the ground (`G + 0.01`), `h` = 0.36 (Beat1 annex) and 0.30 (Beat3 plant), about half a person height. Beat1 annex box still contains it.
3. `D/🧪️practice/🐍️.py` `_boiler`: same issue (0.64 fixed box, taller than the person). Now `_boiler(floor_y, room_h, x)` at 40 % of the clear room height, foot on the floor line, at true scale to the 70 % occupant.
4. `V/🧪️tests/🔣️staffage_only.json`: the rule A guard flagged `primary-energy::_meter_glyph`, `practice::_gas_meter` and `practice::_thermo` (diagram instruments, not staffage). Added them to `allowed` with reasons.

## Checked OK (no change)

- Pump (practice Beat2): fitted 0.6 x 0.6 on the supply pipe, rotor spins (frames differ), base on the pipe, label "Pumpe" clear of the pipes.
- Thermostatic valves (practice Beat2): four valves under the radiators on the risers, knob closes in the close-down clip (opening 1.0 to 0.3), pipes meet them, no label overlap.
- Pumpjack (primary-energy Beat1): on the ground line, walking beam nods across frames, thermal tint reads; "Gewinnung" label clear above it.
- Heat pump (primary-energy Beat3): on the ground, still until the rate is raised, thermal tint consistent with the boiler.
- Occupants (practice talker/buyer/tenant, primary-energy `_stand`): feet on the floor, 70 % of room height, thermal breathing clip.
- Look: all use the same handcrafted thermal palette, no mixed line glyphs.

## Tests

- `staffage_only` guard (18 tests): my three entries pass. 3 failures remain, all outside my files: `❄️cooling/📐️system-sizing::_vent_unit`, `📊️demand/♨️system-losses::_meter` (rd2's area; concurrent edits to the allowlist removed its entry), `🧲️physics/🧲️fundamentals::_thermometer`.
- `V/🧪️tests/🐍️integration` (9 tests, includes demand emitters/occupants and system-losses/regulation stages): 8 OK, 1 failure `test_physics_scene_constructor_bounds` (radiator height 0.3408 vs 0.408, physics clip, not demand).
- regulations `☀️sun` clip test: 2 OK.
- No catalog change, so no catalog check needed.

## Open

- The three guard failures and the physics radiator height belong to other agents.
- Beat3 heat pump/boiler are small by design (half a person); readable in the 480p render but near the lower limit.

Scripts: `T/🧰️rd1/chain.sh`, `chain2.sh`. Generated frames and mp4 were deleted.
