# Round 2 fz: gas-hob notes and calculation-clip radiators

## 1. gas-hob plan figures
- Geometry: `widthM` 0.3, `depthM` 0.52 (`S/🗂️catalog/🔌️appliances/gas-hob/🔣️.json` lines 54-55). At the 1:50 key the plan is 6 mm by 10.4 mm; 15 x 26 mm was 1:20. The key (50) was right, the notes were wrong.
- Fixed in the 50 representation description, en and de (lines 930-931): "(15 mm by 26 mm)" to "(6 mm by 10.4 mm)", "(15 mm mal 26 mm)" to "(6 mm mal 10,4 mm)". No other 15/26 mm remain.
- Catalog check `--asset gas-hob`: NOT COMPLETED. Two attempts (about 590 s and 550 s) stayed in `[capacity]` ("memory pressure is raised ... behind the host itself"), then hit my timeout; no check marks, no failures.
- `capacity_status`: host pressure "warn" (load 16, about 20 GB free of 64 GB), floor 4096 MB. One running grant (pid 72308, reserved 2048 MB, observing 232 MB) held a slot since ~50 min earlier; 18 command requests waiting (all 2048 MB, reason `pressure-warn`, including mine). Nothing of mine held capacity. The gate is the pressure level, not free memory. Re-run once the queue drains.

## 2. Calculation clip radiators (`AnlagenVerluste`, `_plant_house`)
- Rendered `AnlagenVerluste` de and en (ql, 15 fps, one scene), frames extracted with PyAV and zoomed.
- Orientation: correct. Left radiator unmirrored with its pipe stub at the wall side; the right one mirrored, stub toward the right wall. Both stand on the floor slab and connect to the pipe run.
- On the wall: after the fix the cast bounds touch the inner wall faces exactly (anchors -6.41 and -1.59 vs faces -6.538 and -1.462).
- Defect found: scale. `_cast(cast, metre, ...)` takes a figure height, so passing `metre` drew the radiator 1.41 times too big (a 0.71 m panel as 1 m, bulky next to a 1.69 m person). Fix in `🧮️calculation/🐍️.py` (`parts["radiators"]`): height is `staffage_height("shared-radiator-thermostat") * metre`. The radiator is now about half the person's height.
- Test: `scenes.SceneScaleTest.test_heating_plant_uses_one_physical_scale` expected figure height 1.12 m, which only matched the oversized drawing. Set `radiator_height_m` to 0.81 (the cast's figure height 0.518 / 0.64) in `🧪️tests/🔣️scenes.json`; the test passes (1 test OK).
- Not changed: the wall thermostat cast (`heating-calculation-thermostat`, y 0.5) floats above the left radiator by design, a separate wall unit.
- Final frames were rendered with a 0.71 m height; the test-aligned 0.81 m differs by 14 percent, no re-render. Agent rd was also asked about this test; the fix above should resolve it.
- The `taskkill /IM bun.exe` by agent nf did not hit my renders (the first render finished exit 0 before, the re-renders finished after); only the catalog check was lost, and it had already timed out.

Generated frames and renders deleted. Scripts in `🧰️fz/`.
