# Unused WFC Metadata

Removed twelve internal error variants with no producer or matcher outside their own enum/Display definitions. Retained TooManyNodes because the graph-view conversion does produce it. Removed unused constraint name/exactness metadata: all four implementations returned constant Exact, and no solver consulted either trait method. Constraint initialization and complete-assignment validation remain intact; no algorithm was removed or exported merely to avoid a warning.

Five Rust sources parsed. Full compiler and runtime verification pending.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚠️error/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔢️constraints-card/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔗️constraints-conn/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌊️flow/🦀️.rs
