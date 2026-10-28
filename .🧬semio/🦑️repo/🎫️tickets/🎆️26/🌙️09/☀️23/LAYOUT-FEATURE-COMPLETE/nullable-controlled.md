# Controlled Nullable Diff Ownership

Repaired all 13 sparse nullable fields on LayoutDiff, CharacterStylePatch, PagePatch and FramePatch. Present values decode through the inner Option owner under NativeDecodeControl, absent fields use the outer optional owner's admitted default, and error cleanup explicitly retires the full optional owner. Existing ordinary and serde presence semantics remain unchanged.

Language-neutral corpus: C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🫙️nullable/🔣️.json (11 cases: absent, clear and concrete values). Tests were authored before hooks: C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🫙️nullable/🦀️.rs. They compare ordinary and controlled JSON construction, controlled re-encoding, independently derived serde row patches, cancellation and tiny budget refusal.

Actual verification: independent web-tree-sitter Rust parser accepted all four changed native sources with no syntax errors. Native compilation and the authored runtime tests have not run because the repository-owned executable currently provides no evidence-producing native runner. This is a syntax pass, not a native execution pass.

Changed source files:
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️row-patches/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧩️patchable-frames/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🫙️nullable/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🫙️nullable/🔣️.json
