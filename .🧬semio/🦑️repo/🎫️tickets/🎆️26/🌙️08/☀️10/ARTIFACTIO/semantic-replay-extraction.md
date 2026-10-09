# Generation2d Semantic Replay Ownership

Moved 22 declarations (direct semantic replay, displaced-owner retirement, retained clone helpers, structural bounds, cold disposal and test replay) into canonical schema mutations. Binary ingress imports its needed helpers privately. Physical JSON/pack parsing and store initialization cursor remain in IO. Tree-sitter validates resulting Rust syntax; no native runtime claim.

## Changed Files

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
