# Held Model Borrowed Slice Iteration Repair

The independent static audit found an extra borrow of the shared RowIndex indices slice. Exactly one held source region now iterates relations.indices() directly. The original production Before still matches byte for byte. The registered aggregate phase reads the same mutable owner held-pairs input, so no duplicated script payload is needed. The correction changes no row order, field reconstruction, paid reserve, mutation behavior, or owner path count.

The full current held source and exact one-region guard are retained in semio-model-complete-semantic/physical-borrowed-slice-iteration-held-guard.json. No compiler/runtime credit is claimed for held source.

A follow-up read-only census of all six lane-owned held SQL main files found no remaining directly borrowed RowIndex indices iteration matching the same error pattern. This syntax-pattern census does not establish typechecking or runtime behavior.
