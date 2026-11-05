# Rust Semantic Descriptor Field Census

Read-only scan of 2676 simple SemanticDescriptor literals in norm Rust sources, stripping string contents before field-name extraction. Expected owned fields: verb, entity, kind, record. Missing/extra field sets: 0. This source census does not compile Rust or inspect macro-expanded literals.

Suspicious record identifiers (outside simple native type-name shape):

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-detail-id/🦀️.rs:18`: record `ChangedFatigueStressFatigue detailId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-forming-gain-member-id/🦀️.rs:18`: record `ChangedFormingGainFatigue memberId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-max-case-id/🦀️.rs:18`: record `ChangedFatigueStressLoad case of the maximumId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-buckling-length-member-id/🦀️.rs:18`: record `ChangedBucklingLengthFatigue memberId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-material-ductility-material-id/🦀️.rs:18`: record `ChangedMaterialDuctilityFatigue materialId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fastened-joint-member-id/🦀️.rs:18`: record `ChangedFastenedJointsFatigue memberId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-fastened-joint-kind/🦀️.rs:18`: record `ChangedFastenerFastened joint`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-panel-steel-subgrade/🦀️.rs:18`: record `ChangedPlatedPanelsFatigue steelSubgrade`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-panel-steel-grade/🦀️.rs:18`: record `ChangedPlatedPanelsFatigue steelGrade`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-fatigue-stress-min-case-id/🦀️.rs:18`: record `ChangedFatigueStressLoad case of the minimumId`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️change-test-pairs/🦀️.rs:17`: record `ChangedTestTest pairs r_e:r_t (D.7.2)`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-assessment-mid-steel-area-m2/🦀️.rs:17`: record `ChangeAssessed elementIntermediateSteelAreaM2`.

The record strings are metadata, not a proven Rust compile error; compare them with the actual record type before modifying.
