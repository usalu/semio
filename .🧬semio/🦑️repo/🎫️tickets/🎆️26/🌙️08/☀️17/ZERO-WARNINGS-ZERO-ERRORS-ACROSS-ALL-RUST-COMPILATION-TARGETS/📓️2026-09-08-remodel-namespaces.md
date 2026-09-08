# Remodel Namespaces

Pass573 fixes fresh native560 duplicate-name and unresolved-import errors in the extracted Remodeling artifact. Framework schema derives/descriptors use the distinct framework_schema alias; the artifact's schema is directly reexported from its standard. The document text facade is named document_dsl, leaving dsl for the kernel DSL API. Both grammar registration references were updated. Five compute-internal reexports now resolve from the artifact's crate root instead of the former parent-plugin namespace.

Files:

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`

No data fields, math implementations, codecs or test expectations changed. Syntax, compiler and runtime validation remain pending.


Pass574 syntax validation across passes568,569,571 and573: 27/27 files parsed with unchanged source hashes. Compilation and runtime checks remain pending.
