# EN 1993 Field Align

Source: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`.

## Added

| Struct | Field | Rust type | Schema unit / label |
| --- | --- | --- | --- |
| `SteelJoint` | `weld_force_longitudinal` | `f64`, `#[dsl(unit = "N")]` | N — weld force along the weld axis |
| `SteelJoint` | `weld_force_transverse` | `f64`, `#[dsl(unit = "N")]` | N — weld force perpendicular to the weld axis |
| `FatigueDetail` | `consequence` | `FatigueConsequence` | enum `Low` / `High` |
| `PlatedPanel` | `steel_grade` | `String` | steel grade |
| `PlatedPanel` | `steel_subgrade` | `String` | sub-grade |
| `PlatedPanel` | `design_temperature` | `f64` | °C (UI unit; no `x-semio-dsl` unit) |
| `BridgeFatigue` | `consequence` | `FatigueConsequence` | enum `Low` / `High` |
| `JointPlate` | `group_method` | `BoltGroupMethod` | enum `Elastic` / `InstantaneousCentre` |
| `FormingGain` | `used_yield` | `f64`, `#[dsl(unit = "Pa")]` | Pa — yield strength used |
| `En1993Snapshot`, `En1993Artifact` | `fatigue_stresses` | `Vec<FatigueStress>` | required `fatigueStresses` |
| `En1993Snapshot`, `En1993Artifact` | `buckling_lengths` | `Vec<BucklingLength>` | required `bucklingLengths` |
| `En1993Snapshot`, `En1993Artifact` | `forming_gains` | `Vec<FormingGain>` | required `formingGains` |
| `En1993Snapshot`, `En1993Artifact` | `material_ductility` | `Vec<MaterialDuctility>` | required `materialDuctility` |
| `En1993Snapshot`, `En1993Artifact` | `fastened_joints` | `Vec<FastenedJoint>` | required `fastenedJoints` |

The five collection item structs already existed. They were not copied under a second name. `to_snapshot` / `from_snapshot` now move those vectors; they had been hardcoded to empty.

Showcase and wiring literals already named the scalar fields and the five collections. The SQLite reader literals that omitted the new fields now use `0.0`, `String::new()`, or `FatigueConsequence::Low`.

Twins updated where they already described the same struct: schema snapshot and artifact GraphQL, proto, and TypeScript, plus the compact text GraphQL and binary TypeScript snapshot twins. Collection item types and their enums were added there so the new parent fields have a declared item type.

## Refused

- No second copy of `fatigueStresses`, `bucklingLengths`, `formingGains`, `materialDuctility`, or `fastenedJoints`. None of those collections already lived on `En1993Snapshot` or `En1993Artifact` under another name.
- Diff GraphQL, proto, and TypeScript (`En1993*Patch`) were left alone. They describe patch types. The Rust row patches already listed these fields.
- Compact IO twins were already missing `beamWebThickness` / `beamWebFy` on `SteelJoint` and `reliabilityClass`, `frameActions`, `frameMemberBases` on `En1993Snapshot`. Those older gaps were not backfilled.
- The SQLite `SteelJoint` reader omitted `beam_web_thickness` and `beam_web_fy` and stored the weld forces as `0.0`. The insert already wrote columns 65–68. The reader now uses `v.real(64)` through `v.real(67)` for beam web thickness, beam web yield, and the two weld forces.
- The JSON schema was not edited for snapshot fields. It was the type and label source.

## Diff cardinality

`absorb`, `apply`, `is_empty`, and `inverse` already call delta methods on `joint_plates`, `foundation_blocks`, `material_ductility`, `forming_gains`, `buckling_lengths`, `fatigue_stresses`, `frame_actions`, `frame_member_bases`, and `fastened_joints`. Those nine `En1993Diff` fields were `Option<…Delta>`. They are now the delta types, matching `placements`. JSON `required`, GraphQL, and proto now require the same nine. The TypeScript parent and `parseEn1993Diff` require them too. The five collection delta interfaces that the parser names are now declared in `diff/🟦️.ts`.

## Diff TypeScript twins

`diff/🟦️.ts` now declares the five collection families named by `En1993Diff` and `parseEn1993Diff`. Each family is removal, insertion, relocation, modification, and patch, with the matching parse functions. Insertion rows use the snapshot interfaces `MaterialDuctility`, `FormingGain`, `BucklingLength`, `FatigueStress`, and `FastenedJoint`.

- `En1993MaterialDuctilityDelta`
- `En1993MaterialDuctilityRemoval`
- `En1993MaterialDuctilityInsertion`
- `En1993MaterialDuctilityRelocation`
- `En1993MaterialDuctilityModification`
- `En1993MaterialDuctilityPatch`
- `En1993FormingGainDelta`
- `En1993FormingGainRemoval`
- `En1993FormingGainInsertion`
- `En1993FormingGainRelocation`
- `En1993FormingGainModification`
- `En1993FormingGainPatch`
- `En1993BucklingLengthDelta`
- `En1993BucklingLengthRemoval`
- `En1993BucklingLengthInsertion`
- `En1993BucklingLengthRelocation`
- `En1993BucklingLengthModification`
- `En1993BucklingLengthPatch`
- `En1993FatigueStressDelta`
- `En1993FatigueStressRemoval`
- `En1993FatigueStressInsertion`
- `En1993FatigueStressRelocation`
- `En1993FatigueStressModification`
- `En1993FatigueStressPatch`
- `En1993FastenedJointDelta`
- `En1993FastenedJointRemoval`
- `En1993FastenedJointInsertion`
- `En1993FastenedJointRelocation`
- `En1993FastenedJointModification`
- `En1993FastenedJointPatch`
