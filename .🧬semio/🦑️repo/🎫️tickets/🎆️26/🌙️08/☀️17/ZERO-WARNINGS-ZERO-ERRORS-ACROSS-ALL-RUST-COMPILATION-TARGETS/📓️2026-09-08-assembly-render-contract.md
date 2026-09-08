# Assembly Render Contract

During the handler inventory, Assembly's authored editor/viewer were found to use the old infallible `UiNode`/`ComponentTree` render signatures, while `TreeWindowKit::render` and the artifact surface traits return `UiAssemblyResult`.

Both structure-window render functions now return `UiAssemblyResult<BuiltNode>`. Both surface functions map successful built nodes to component trees and propagate errors, including unknown-body fallback assembly failures. Their existing collection-branch tests now inspect the current retained tree keys and children rather than the retired `UiNode::Tree` shape. Collection construction and labels are unchanged.

The artifact root and procedural registration explicitly leave these Assembly surfaces unmounted. Their source alignment is therefore separate from the currently compiled WFC artifact. These two tests are not added to the current Cargo runtime selections, because they would not be discovered; no runtime or compiler type-check success is claimed for these unmounted surfaces. Existing WFC runtime selections remain included. Parser validation is pending.

Changed files are the editor/viewer roots, their structure-window render modules and those modules' existing unit tests under `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/` (six Rust files).

Pass 615 parsed all six changed Assembly Rust files successfully with Rust 2021; all source snapshots remained unchanged during parsing. Their unmounted runtime status is unchanged.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🧪️tests/🔬️unit/🦀️.rs
