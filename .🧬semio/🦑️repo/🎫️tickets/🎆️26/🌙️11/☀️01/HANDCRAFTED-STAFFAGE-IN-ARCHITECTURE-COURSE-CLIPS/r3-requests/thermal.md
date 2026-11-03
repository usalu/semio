# Requests from W06 (thermal) to W05 (contracts)

No new schema fields are required: clo and met are derived (`clothingInsulationClo` from layers, `activityMet` per pose), PMV is computed, not stored. Optional, only if scene provenance records thermal parameters:

1. Scene/cast provenance may record `environmentPreset` (slug from `🗂️catalog/🌤️environments`) or the explicit `Environment` and `activityMet`, so a clip's legend values are reproducible.
2. Legend semantics: `Legend.range` mode already distinguishes absolute (`shared`, `fixed`) from comparative (`auto`, `span`); the legend text now states it. A provenance flag `explanatory: true` for transport parcel layers is not needed: the parcel legends carry the assumption text.

Observed, not mine: `🌡️thermal-span` test "accepts a positive span in ajv and in the twin" currently fails with `/casts/span must have required property 'scale'` (cast schema edit in progress by another worker).
