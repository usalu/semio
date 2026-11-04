# Eurocode field mismatches

One obvious compile mismatch per code, from struct literals and JSON schema `required` fields under the norm artifact folders. EN 1990 and EN 1991 were not checked.

| code | struct | field | definition file | use file |
| --- | --- | --- | --- | --- |
| EN 1992 | consistent | consistent | consistent | consistent |
| EN 1993 | SteelJoint | weld_force_longitudinal | /home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | /home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs |
| EN 1994 | consistent | consistent | consistent | consistent |
| EN 1995 | consistent | consistent | consistent | consistent |
| EN 1996 | consistent | consistent | consistent | consistent |
| EN 1997 | consistent | consistent | consistent | consistent |
| EN 1998 | ChangeAssessmentAnalysisMethod | new_analysis_method | /home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-assessment-analysis-method/🦀️.rs | /home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-assessment-analysis-method/↩️inverse/🦀️.rs |
| EN 1999 | consistent | consistent | consistent | consistent |

EN 1993: `SteelJoint` is declared in the snapshot Rust file and does not have `weld_force_longitudinal` or `weld_force_transverse`. The `SteelJoint` literal in that same file sets both. The snapshot JSON schema lists `weldForceLongitudinal` and `weldForceTransverse` as required.

A glob for `**/en1993/**/sqlite/**/snapshot/🦀️.rs` printed `none` because the directory is `🔩️en1993`. The SQLite snapshot is `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` with `🗄️.sql` beside it. That SQL already names `weld_force_longitudinal`, `weld_force_transverse`, `steel_grade`, `steel_subgrade`, `design_temperature`, `group_method`, `used_yield`, `consequence`, `fatigue_stress`, `buckling_length`, `forming_gain`, `material_ductility`, and `fastened_joint`. The gap is the Rust snapshot struct, not a missing table.

EN 1998: `ChangeAssessmentAnalysisMethod` declares `new_analysis_method_choice`. The inverse, diff, demo case, and op-text members used `new_analysis_method`. Those four sites now read and write `new_analysis_method_choice`. The op-text token stays `new-analysis-method`. JSON, GraphQL, proto, and TypeScript already used `newAnalysisMethodChoice`.

EN 1992, EN 1994, EN 1995, EN 1996, EN 1997, and EN 1999: paired JSON schema `required` fields are present on the Rust struct (the JSON `mutation` const tag is not a struct field), and the sampled struct literals do not pass a field the struct omits.
