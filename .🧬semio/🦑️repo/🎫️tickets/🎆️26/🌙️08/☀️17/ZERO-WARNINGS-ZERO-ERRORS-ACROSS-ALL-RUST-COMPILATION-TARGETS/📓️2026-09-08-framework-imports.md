# Extracted Framework Contract Imports

Pass536 addresses fresh native532 compile errors in the Flow host and bridge and the Norm EN1990 artifact. Flow now imports its store, mutation, replacement operation, document schema and widget helper directly from the extracted Flow artifact. Its IO port specification comes from the existing Infinite DAG artifact. The bridge imports the same widget helper.

Norm's NationalAnnex enum closure now explicitly imports the dispatch macro exported by the Norm contract crate. The dispatch implementation confirms that dyn_enum publishes this macro at the defining crate root; the former lexical-scope comment no longer described the extracted arrangement. No national-annex calculations were changed.

Changed:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️bridge/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`

The extracted Flow snapshot had already gained its own widget-helper import from concurrent work, so that file was not edited. Fresh compiler and runtime validation remain required.
