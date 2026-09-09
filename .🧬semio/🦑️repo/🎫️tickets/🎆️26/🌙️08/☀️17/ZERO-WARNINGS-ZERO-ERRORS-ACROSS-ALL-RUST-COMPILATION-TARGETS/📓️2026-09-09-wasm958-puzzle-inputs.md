# Puzzle 3D Internal Input Repairs

The action prologue and unstaged action handler now read action/arguments from their existing Puzzle3dCommand input. The explicit window address and all runtime/interaction inputs remain unchanged. Named the paired persisted/ephemeral emission type. Removed the single-caller session helper and kept the required fallible reducer callback, whose only session value was None.

Fill measurement takes a named request owning the same eight inputs; both production and fixture call sites preserve the same ownership handback. Quoted preview fields use a named field descriptor at all 14 calls, preserving prefixes, optionality, and cursor advancement. The collision engine typed scene sync is infallible; its JSON parser and shared installation callback retain their existing error and supersession boundaries. All sources were guarded and parsed before writing. Runtime and strict compilation remain pending.

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs
