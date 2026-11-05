# Semantic Record Names and Runtime Impact

Read-only review, 2026-11-04.

The owned SemanticDescriptor type lives in framework OS spr/command. Its documentation defines record as the past-tense operation-log/GraphQL name (examples RenamedPiece, FlattenedDesign), while native payload types remain imperative. MutationDescriptor::new currently checks only that record and entity are nonblank; kind must equal leaf.semantic_kind and verb must be approved. Therefore the twelve strings below do not fail this validator or Rust compilation simply because they contain spaces.

Record is not inert display-only text: descriptor_fingerprint serializes the entire SemanticDescriptor into the canonical descriptor identity before hashing. Plugin registration copies record into ContributedMutationSemantics and exposed operation ownership metadata; wasmtime-node-host forwards it, and MCP capability text displays it. The inspected routing identity uses semantic kind/id, not record. No evidence was found that prose currently changes mutation dispatch or payload persistence encoding. Correcting record changes descriptor fingerprint and advertised metadata, so corresponding descriptor fixtures/generated descriptions should be reconciled; no compatibility layer is appropriate.

Expected names below are inferred directly from the exact native payload symbol with imperative Change converted to past-tense Changed, matching the documented naming rule. They are not existing separate native event struct types.

| Native payload | Current record | Expected record | Source label |
| --- | --- | --- | --- |
| ChangeFatigueStressDetailId | ChangedFatigueStressFatigue detailId | ChangedFatigueStressDetailId | Change stress range: Fatigue detail |
| ChangeFormingGainMemberId | ChangedFormingGainFatigue memberId | ChangedFormingGainMemberId | Change forming gain: Cold-formed section |
| ChangeFatigueStressMaxCaseId | ChangedFatigueStressLoad case of the maximumId | ChangedFatigueStressMaxCaseId | Change stress range: Load case of the maximum |
| ChangeBucklingLengthMemberId | ChangedBucklingLengthFatigue memberId | ChangedBucklingLengthMemberId | Change buckling length: Column |
| ChangeMaterialDuctilityMaterialId | ChangedMaterialDuctilityFatigue materialId | ChangedMaterialDuctilityMaterialId | Change material ductility: Material |
| ChangeFastenedJointMemberId | ChangedFastenedJointsFatigue memberId | ChangedFastenedJointMemberId | Change fastened joint: Cold-formed section |
| ChangeFastenedJointKind | ChangedFastenerFastened joint | ChangedFastenedJointKind | Change fastened joint: Fastener fastened joint |
| ChangePanelSteelSubgrade | ChangedPlatedPanelsFatigue steelSubgrade | ChangedPanelSteelSubgrade | Change Plated panel: Sub-grade (empty = not given) |
| ChangePanelSteelGrade | ChangedPlatedPanelsFatigue steelGrade | ChangedPanelSteelGrade | Change Plated panel: Steel grade (empty = not given) |
| ChangeFatigueStressMinCaseId | ChangedFatigueStressLoad case of the minimumId | ChangedFatigueStressMinCaseId | Change stress range: Load case of the minimum |
| ChangeTestPairs | ChangedTestTest pairs r_e:r_t (D.7.2) | ChangedTestPairs | Change Test series: Test pairs r_e:r_t (D.7.2) |
| ChangeAssessmentMidSteelAreaM2 | ChangeAssessed elementIntermediateSteelAreaM2 | ChangedAssessmentMidSteelAreaM2 | Change Assessed element: Intermediate longitudinal steel area A_s,int (0 = none) |

## Exact Sources

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-detail-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-forming-gain-member-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-max-case-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-buckling-length-member-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-material-ductility-material-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fastened-joint-member-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-fastened-joint-kind/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-panel-steel-subgrade/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-panel-steel-grade/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-min-case-id/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️change-test-pairs/🦀️.rs`; adjacent descriptor has no top-level record value
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-assessment-mid-steel-area-m2/🦀️.rs`; adjacent descriptor has no top-level record value

This audit did not execute descriptor registration or persistence tests.

## Metadata Repair Verification

Normalized 12 of the exact twelve audited native record strings to ChangeX -> ChangedX. Each file was reread before editing; a reversible text comparison established the record value was the only changed text, preserving localized labels and verb/entity/kind values. All twelve expected values were verified afterward. Full census: 2676 simple literals, 0 malformed field sets, 0 whitespace-bearing record names. No new tests or native build launched for this metadata-only correction.

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-detail-id/🦀️.rs`: `ChangedFatigueStressFatigue detailId` -> `ChangedFatigueStressDetailId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-forming-gain-member-id/🦀️.rs`: `ChangedFormingGainFatigue memberId` -> `ChangedFormingGainMemberId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-max-case-id/🦀️.rs`: `ChangedFatigueStressLoad case of the maximumId` -> `ChangedFatigueStressMaxCaseId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-buckling-length-member-id/🦀️.rs`: `ChangedBucklingLengthFatigue memberId` -> `ChangedBucklingLengthMemberId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-material-ductility-material-id/🦀️.rs`: `ChangedMaterialDuctilityFatigue materialId` -> `ChangedMaterialDuctilityMaterialId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fastened-joint-member-id/🦀️.rs`: `ChangedFastenedJointsFatigue memberId` -> `ChangedFastenedJointMemberId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-fastened-joint-kind/🦀️.rs`: `ChangedFastenerFastened joint` -> `ChangedFastenedJointKind`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-panel-steel-subgrade/🦀️.rs`: `ChangedPlatedPanelsFatigue steelSubgrade` -> `ChangedPanelSteelSubgrade`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-panel-steel-grade/🦀️.rs`: `ChangedPlatedPanelsFatigue steelGrade` -> `ChangedPanelSteelGrade`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-min-case-id/🦀️.rs`: `ChangedFatigueStressLoad case of the minimumId` -> `ChangedFatigueStressMinCaseId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️change-test-pairs/🦀️.rs`: `ChangedTestTest pairs r_e:r_t (D.7.2)` -> `ChangedTestPairs`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-assessment-mid-steel-area-m2/🦀️.rs`: `ChangeAssessed elementIntermediateSteelAreaM2` -> `ChangedAssessmentMidSteelAreaM2`.
