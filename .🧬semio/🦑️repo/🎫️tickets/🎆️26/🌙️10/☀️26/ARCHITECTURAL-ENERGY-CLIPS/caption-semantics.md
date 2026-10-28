# Final Resolution

The English opaque-envelope qualifier was restored in the canonical source and portable case. The original English/German caption renderer and source-contract check passed in 17.116 seconds. The retained records reconcile the count: 43 sweep failures plus one earlier Physics German failure equal 44 previous-language strings across 39 clause keys. No remaining semantics finding is open.

# Caption Semantics Review

Independent read-only comparison of the 39 portable caption cases against their retained previous wording and current canonical source. No external scientific validation or renders were repeated. All 78 current EN/DE contract strings exactly match their scene/key NARRATION values by AST inspection.

## Finding

The English transmission-humidity / Beat3_LueftungslastImSommer / intro drops the application qualifier opaque: previous wording says the opaque envelope delivers about 680 W, while the replacement says Envelope: 68 W/K × 10 K ≈ 680 W. The source computes UA_OPAQUE from roof and wall only. Retain Opaque envelope in that caption so 680 W cannot be read as including glazing. Later narration still calls it the opaque envelope. No other concrete loss of scientific meaning or numerical change was identified in the 39 before/after cases.

## Preserved Evidence

- Physics compare2 retains exactly one at the house for electric resistance heating and the possibility of flue gas elsewhere from electricity. The building boundary and upstream-generation distinction remain explicit in both languages.
- Latent heat remains approximately 2450 kJ/kg; condensation releases that energy and requires removal. Moisture comparison remains 32 °C/50%/15 g/kg versus 26 °C/50%/10.5 g/kg, with Δx = 4.5 g/kg.
- Ventilation retains 220 m³/h, 6 K, approximately 450 W sensible heat, 2.5 m/s, 0.0244 m² and required room flow in m³/s. Winter zero recovery still loses 1500 W; recovery comparison remains up to 90%, and summer 32/26 °C with about 5 of 6 K recovered.
- Airflow sizing retains room-minus-supply temperature difference, density × specific heat × temperature difference × volume flow, and division by those three factors when isolating required flow. The 0.126 versus 0.061 m³/s comparison is unchanged.
- Natural ventilation retains series-opening restriction, winter 20 rather than 8 K, the same shaft and nearly triple pressure, summer 18/26 °C, approximately 100 Wh at 6 K, cool-hour ventilation, thermal mass and brief winter airing.
- Solar caption shortening retains irradiance × gross area × the three dimensionless frame/shading/glass factors. Canonical cooling solar_cooling_example imports heating WIN_A = 1.2 × 1.6 = 1.92 m² and FF_GLASS = 0.70, with G_VALUE = 0.50 + 0.10 = 0.60; cooling shading is 0.15 and irradiance 800 W/m². Its window example remains 96.768 W. The separate system-sizing example retains its pre-existing 30 m² × 0.49 × 0.15 × 0.60 × 800 W/m² = 1058.4 W, rounded to 1060 W. No caption change confuses these examples or replaces 0.70 with 0.49 in the canonical window.
- Sorption, heat-recovery, passive/active strategy and microbial claims retain their original instructional meaning; this comparison does not independently establish those original scientific claims.

## Counts and Close Inventory

The portable contract has 39 source/scene/key cases and 44 retained previous-language strings (five cases contain both previous languages). caption-audit.md reports 43 initial overflowing occurrences and separately notes the earlier Physics repair. The discrepancy remains unresolved pending the scale/caption author’s explanation; no cause is inferred here. Ticket close must not equate 43 with the contract previous-language count.

The Complete Scale-Agent Source Footprint in scale.md has 20 unique repository-relative paths, and all 20 exist. It includes all 10 canonical narration files used by these cases as well as scale changes, shared fonts, integration contract/tests and authoring script. That exact list is suitable for the scale/caption portion of ticket_close, alongside other agents’ files and retained ticket reports. This audit did not claim that those 20 paths cover other agents or globally inspect unrelated concurrent changes.
