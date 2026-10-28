# Normative Authored BIM Rotation

The new Cartesian admission correctly rejected the spatial exporter's previously authored one-sided orientation. bimSpatialToIfc4x3 now authors explicit Z together with RefDirection for every nonzero horizontal building rotation. Zero rotation retains both unset. The importer continues to enforce the normative provision; it does not special-case its own exporter's invalid output.

Five neutral negative/zero/positive/quarter/half-turn cases, a closed JSON schema, feature and actual exporter/importer tests preceded the change: **2 pass / 4 fail** reproduced the defect. The new six-test suite is explicitly registered. The new and existing BIM suites pass **19 tests / 95 assertions**, zero failures, with one pre-existing genuine-output-gated skip.

Independent IfcOpenShell 0.9.0.post1 fully validates all five actual corrected outputs with zero EXPRESS errors. Its placement evaluator independently reproduces all rotations and exact local origin/elevation. Four explicit counterexamples reconstructing the previous one-sided provision each produce exactly one AxisAndRefDirProvision error. Five independent physical STEP serializations reimport with the original supported rotation/origin/elevation. Retained inputs: bim-axis-export.ts, bim-axis-oracle.py and bim-axis-verify.ts. Native execution is false.

The High details worker owns equivalent native Builder/Reader and controlled FEM identity admission fixes and native regressions. Their report and genuine native execution remain separate from these TypeScript/IfcOpenShell results.
