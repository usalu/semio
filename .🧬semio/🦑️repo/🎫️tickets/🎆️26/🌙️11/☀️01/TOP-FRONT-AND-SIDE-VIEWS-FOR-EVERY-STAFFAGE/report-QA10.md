# Report QA10: Visual QA of O2 Objects

- **Checks:** check, sweep (27 of 27) and three-views vitest (14 of 14) all pass.
- **Fixes:**
  - waste-bin plan lid stepped to the rim's corner radius.
  - street-lamp plan pole, cover and nuts get their own region, so their outlines read.
  - shopping-trolley: leftover helper bone `gate-axis` removed. All rigs now match HEAD.
- **trolley gate:** a pure sagittal swing, so no spread is needed.
- **tealight:** waiting on FW depth ordering, so that the wax occludes the unlit flame.
- **Inspected:** 1:50 montages of all seven assets. 1:100–1:500 were only sampled.
- **Incident:** `taskkill /IM bun.exe` killed other workers' bun processes; a rule was added to the brief.
- **Open:**
  - stepladder: confirm whether any pose name implies folding while the rig is rigid.
  - Inspect all sheets at 1:100–1:500.
  - tealight, after depth ordering lands.

## Round 2
- **Stepladder:** every pose is "opened and locked" and the clips are load cases, so no hinge is needed.
- **Tealight:** the unlit flame is hidden in plan by depth ordering, because the wax-top disc is bound to the `wick` bone.
- **Waste-bin 1:50:** the pen hierarchy is changed so the lid reads as the heavy edge.
- **Not read:** stepladder, crate stack and tealight at 1:100–1:500, and the street lamp at 1:200 and 1:500.

## Round 3
- **waste-bin:** the lid-corner strips failed the print gap at 1:100 and 1:50, so they are coarsened to printable steps (1:100 min gap 0.151 mm). The check is ok.
- **Street lamp:** the single-entry `regions` on filled shapes stay, because the same-region fix only frees unfilled strokes.
- **Montages read:** stepladder, crate stack, tealight and the lamp 1:100 tiles.
- **Not read:** the lamp at 1:200 and 1:500.
- **Writes:** `T/🧰️O2/lib.py` saves atomically.

## Round 4 (Final)
- **waste-bin:** the lid handle is an unfilled outline at z 5.1 in the 1:50 plan, so it shows when the lid is closed. Check ok at every scale (1:50 min feature 0.410 mm), consistency ok, vitest 2 of 2.
