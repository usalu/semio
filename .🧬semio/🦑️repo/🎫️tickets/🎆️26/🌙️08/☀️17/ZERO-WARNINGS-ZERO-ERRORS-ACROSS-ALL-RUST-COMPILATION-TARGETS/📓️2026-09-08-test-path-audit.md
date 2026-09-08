# Renamed Numeric Test Directories

Pass559 re-inspected the two Procedural files skipped by pass557 because their source hashes changed. Their three old numeric paths remained unchanged. Those three path attributes now use the already verified microscopy-prefixed directories: generation2d t005, generation3d t006 and t007. No intervening source changes or test contents were replaced.

Pass554 audits explicit Rust path attributes that still name t-number directories. Native532 reported missing numeric test directories in Norm and Procedural. A candidate is accepted only when the old target is absent and the corresponding existing directory with the microscopy emoji and unchanged numeric identity resolves from the source file. No paths have been edited by this audit.

Candidate files: 13. Candidate paths: 23. Unresolved references: 0.

Pass557 applied 20 paths in 11 files with whole-file hash guards and target-existence checks; 2 changed files were skipped. All test bodies and numeric identities remain unchanged.

- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs (4 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (1 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (1 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (2 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (3 paths)
- ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs (2 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (1 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🦀️.rs (1 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (2 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (2 paths)
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs (1 paths)

Pass557 syntax validation, including passes555 and556: 14/14 files parsed with unchanged source hashes. Actual compiler and runtime validation remain required.
