# Native Provenance Source Errors R92

The main standard contract is BimModelStructureContract.test_every_quantity_is_sourced_or_assumed_with_a_unit. Sourced values must carry canonical source identifiers, not prose. Descriptions may remain in value/evidence narrative fields. These current source fields failed the unchanged SOURCE_ID regex:

- `LIGHT-HALL-ZEISS-1` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original November1930 photo244; Zeiss upward bowl"
- `LIGHT-HALL-ZEISS-2` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original November1930 photo244; Zeiss upward bowl"
- `LIGHT-HALL-BASIN-19` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original November1930 photo244 and text242 column19"
- `LIGHT-HALL-CEILING` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original November1930 FLC L2(17)061, Albalite stepped ceiling body"
- `LIGHT-FIRST-HALL-CEILING` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original November1930 FLC L2(17)059, Albalite ceiling body"
- `LIGHT-KITCHEN-1` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original ArchitectureVivante1931 plate34 upright295/375 occluded fixture at sink-window side"
- `LIGHT-KITCHEN-2` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original ArchitectureVivante1931 plate34 upright656/375 clear short arm and shallow bell"
- `LIGHT-SALON-LINEAR` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original November1930 FLC L2(17)098; absentAugust1930, invertedVchromed trough"
- `LIGHT-EXTERIOR-REAR-W` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Text242 rear porch spots; selected actual outer porch soffit region, exact historicXYunregistered"
- `LIGHT-EXTERIOR-REAR-E` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Text242 rear porch spots; selected actual outer porch soffit region, exact historicXYunregistered"
- `LIGHT-EXTERIOR-11` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Original photo244 visible lowXriser11 spotlight, text242; upward/outward morphology"
- `LIGHT-EXTERIOR-6` → `psets.Pset_LightingEvidence.ExecutionWitness.source`: "Text242 opposite falsepiloti spot; exact location/direction inferred"
- `FIX-CHAISE` → `attributes.mosaic_module.source`: "SAB2024-p167-168 original5cm mosaic"
- `FIX-BATH` → `attributes.mosaic_module.source`: "SAB2024-p167-168 original5cm mosaic"
- `FIX-BATH` → `attributes.proposed_length.source`: "January1930 proposal only, NOT completed measurement"
- `FIX-PARENTS-WET-PLATFORM` → `attributes.mosaic_module.source`: "SAB2024-p167-168 original5cm mosaic"

Four current source elements fail the required-core-field test BimModelStructureContract.test_every_element_is_complete_and_references_exist because `psets` is missing: FIX-CHAISE, FIX-BATH, FIX-PARENTS-WET-PLATFORM, FIX-DRESSING-STORAGE. No other required key is missing in these four records.

Root owns canonical source repair. The old native cohort opening roster mismatches are already superseded by the current master fixture edit; no additional roster change is requested.
