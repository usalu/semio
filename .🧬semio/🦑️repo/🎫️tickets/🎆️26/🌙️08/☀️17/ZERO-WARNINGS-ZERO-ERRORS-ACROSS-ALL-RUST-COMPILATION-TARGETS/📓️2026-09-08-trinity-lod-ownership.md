# Trinity LOD Ownership

Pass571 fixes Jack's compile-time reference to the separately packaged Rewriting editor. Moved the existing six-tier detail scale and JSON serializer into Jack's editor, which is already a dependency of Rewriting. Both editors now consume this shared implementation. Rewriting's component-app-assembly feature already enables Jack's component-app-assembly feature. All tier IDs, labels, descriptions, zoom boundaries, and serialized fields remain identical. The existing Rewriting six-tier runtime test now imports the shared serializer directly.

Also corrected the same world's two stale framework paths: force_graph and the canvas module exported by semio-framework-os-infinite. No dependency was added.

Files:

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️lod/🦀️.rs` (created)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🧪️tests/🔬️unit/🦀️.rs`

Syntax, native/WASI compilation and the existing LOD runtime law remain pending.
