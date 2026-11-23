# Gap Bridge Notes From Formula Panel

## Problem
In Beat6 Wärmebrücken the red method notes (detailliert / pauschal …) sat almost on the red formula panel (≈0.09 gap).

## Fix
Raise the comparison columns (`base_y` −0.72 → −0.32), slightly lower the bar scale (`k_h` 0.018 → 0.0155), and keep notes clear of both the white heads and the formula band (≥0.25).

## Tests
`bridgeMethodLayout` case plus `test_bridge_method_notes_clear_formula_band`.
