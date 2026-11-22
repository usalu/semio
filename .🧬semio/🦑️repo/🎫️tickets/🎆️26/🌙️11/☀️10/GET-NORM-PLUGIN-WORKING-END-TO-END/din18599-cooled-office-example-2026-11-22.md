# DIN V 18599 cooled-office example

Root cause: the authored `cooled-office` asset was wrong. It modelled a well-insulated office with a 26 C cooling setpoint (equal to the DIN V 18599-10 default) and Q_C,nd = 1877 kWh/a, below the 0.5 * Q_H,nd limit (DIN V 18599-7), so no check failed. The report was only non-compliant through unverified GEG items, so the claimed "at least 2 failing checks" never held. The engine and the assertion are correct.

Fix (asset only, `...any/assets/cooled-office/dsl.semio`): cooling setpoint 26 -> 24 C (overcooled office), south facade changed from 42 m2 wall to 12 m2 wall plus 30 m2 unshaded glazing (g 0.6, fc 1). Result: `din18599.2.cooling-need.zone-living` fails (24 < 26 C) and `din18599.7.cooling` fails (Q_C,nd 8754 kWh/a over 0.5 * Q_H,nd). No engine code changed; no other family affected.

Result: `test result: ok. 15 passed; 0 failed` for `semio-s-artifact-norm-din18599-examples`.
Note: `schema/tests/compliance` also references `cooled-office`; not run (only the examples crate was permitted).
