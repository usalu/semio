# Native Export Class Coverage R92

Read-only source-wide dispatch audit. The native exchange BODIES table omits all IfcFooting and IfcRailing occurrences. step_excerpt raises ValueError before geometry for these classes. This is an end-to-end export gap despite occupied geometry being available through other APIs. IfcOpeningElement is also omitted; openings require their own void-relation export acceptance rather than a fabricated occupied solid.

Source snapshot f9fc017334468debb90f4e57797010fe0e4984991c24e0bb3e4c8d2a9c736248 (normalized JSON hash; raw hash recorded by pending executable manifest).

## IfcFooting

- `FT-CELLAR`
- `FT-STAIR-PA`
- `FT-CELLAR-C14`
- `FT-BLINDING-1`
- `FT-PAD-1`
- `FT-BLINDING-2`
- `FT-PAD-2`
- `FT-BLINDING-3`
- `FT-PAD-3`
- `FT-BLINDING-4`
- `FT-PAD-4`
- `FT-BLINDING-5`
- `FT-PAD-5`
- `FT-BLINDING-6`
- `FT-PAD-6`
- `FT-BLINDING-7-12`
- `FT-PAD-7-12`
- `FT-BLINDING-11`
- `FT-PAD-11`
- `FT-BLINDING-16`
- `FT-PAD-16`
- `FT-BLINDING-17`
- `FT-PAD-17`
- `FT-BLINDING-21`
- `FT-PAD-21`
- `FT-BLINDING-22`
- `FT-PAD-22`
- `FT-BLINDING-23`
- `FT-PAD-23`
- `FT-BLINDING-24`
- `FT-PAD-24`
- `FT-BLINDING-27`
- `FT-PAD-27`
- `FT-BLINDING-28`
- `FT-PAD-28`
- `FT-BLINDING-29`
- `FT-PAD-29`
- `FT-BLINDING-30`
- `FT-PAD-30`
- `FT-BLINDING-31`
- `FT-PAD-31`
- `FT-BLINDING-32`
- `FT-PAD-32`
- `FT-BLINDING-33`
- `FT-PAD-33`

## IfcRailing

- `GUARD-RM-1`
- `GUARD-RM-2`
- `GUARD-ST-GF`
- `TUBE-ST-GF`
- `GUARD-ST-FR`
- `TUBE-ST-FR`
- `TUBE-RM-2`

## IfcOpeningElement

- `OP-S`
- `OP-E`
- `OP-N`
- `OP-W`
- `OP-G-WIN-1`
- `OP-G-WIN-2`
- `OP-G-WIN-3`
- `OP-G-N-WIN`
- `OP-D-SERVANT-1`
- `OP-D-SERVANT-2`
- `OP-D-LAUNDRY`
- `OP-D-CHAUFFEUR`
- `OP-D-ENTRY`
- `OP-D-KITCHEN`
- `OP-D-PANTRY`
- `OP-D-SALON`
- `OP-GL-TERRACE`
- `OP-D-GUEST`
- `OP-D-SON`
- `OP-D-PARENTS`
- `OP-D-BOUDOIR`
- `OP-GL-RAMP-GARDEN`
- `OP-D-CHAUFFEUR-BEDROOM`
- `OP-D-CHAUFFEUR-BATH`
- `OP-SERVING-HATCH`
- `OP-D-SERVICE`
- `OP-D-KITCHEN-PATIO`
- `OP-KITCHEN-PATIO-FRAME`
- `OP-W-UPPER`
- `OP-CELLAR-STAIR-ACCESS`
- `OP-D-SHARED-BATH-GUEST`
- `OP-D-SHARED-BATH-SON`
- `OP-GARAGE`
- `OP-SC-VIEW`
- `OP-HOIST-CELLAR`
- `OP-HOIST-HATCH`
- `OP-COAL-CELLAR`
- `OP-COAL-HATCH`
- `OP-DP-1-FLOOR`
- `OP-DP-1-ROOF`
- `OP-DP-2-FLOOR`
- `OP-DP-2-ROOF`
- `OP-FLUE-FLOOR`
- `OP-SOIL-FLOOR`
- `OP-GARDEN-E`
- `OP-KIOSK-E`
- `OP-STAIR-ROOF`
- `OP-FLUE-ROOF`
- `OP-SALON-FLUE-ROOF`
- `OP-KITCHEN-FLUE-ROOF`
- `OP-BOUDOIR-LOW-ROOF`
- `OP-LINGERIE-LOW-ROOF`
- `OP-N-2`
- `OP-W-UPPER-2`
- `OP-W-UPPER-3`
- `OP-D-WC-FIRST-WEST`
- `OP-D-WC-FIRST-EAST`
- `OP-D-WC-GROUND-SERVICE`
- `OP-D-WC-CHAUFFEUR`

The executable native-export-manifest-r92.py imports the actual current BODIES table without building geometry, preserving the production interface. No production edits.

Executable manifest completed against raw source SHA256 `78d203273fd5662ac32ee0d3b59411b20da2fd6f344e36e4cc6f21d87b26bfde`: 111 unsupported source occurrences. This is a sparse dispatch audit, not proof that every supported class serializes or decodes correctly.


Physical class-dispatch gap repaired and independently decoded: all45footings and7railings×ENDE PASS, including complete mixed materials after SDK caught GUARD-RM-2 loss. See footing-railing-native-review-r92.md.
