# BREP Semantic Referential Validation

Cross-collection BREP identity validation belongs to canonical schema. The unchanged typed validator now lives there, while physical payload decoding and validator registration remain IO and invoke it privately. Semantic validation-report inference imports the canonical helper directly. Existing composition tests use the canonical private import from IO. No new IO facade export was added. Native verification is pending.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🦀️.rs
