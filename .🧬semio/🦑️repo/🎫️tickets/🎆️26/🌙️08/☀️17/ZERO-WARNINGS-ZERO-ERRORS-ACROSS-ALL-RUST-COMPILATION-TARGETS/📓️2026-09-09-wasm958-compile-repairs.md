# WASI 958 Compile Repairs

Two Jack mutation descriptors omitted the required owner field. Added the exact normalized source-directory owner to each fourteen-field descriptor, preserving payload and mutation semantics. The resulting derive should restore MutationLeaf implementations for SetQuery and ReplaceQueryResult; compiler verification is pending. Removed the unused Jack wasm alias module and its declaration after checking Rust and TypeScript consumers: its only content referenced a retired, unavailable framework_editor dependency. Updated the space seed's draw and writer demo includes to their current artifact asset directories after verifying both files exist. All source anchors were guarded, and remaining Rust sources parsed before writes.

- Updated: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🔣️.json
- Updated: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🔣️.json
- Updated: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs
- Removed: ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs
- Updated: ✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs


## Wires Value Imports and Camera Mutation

Matched the existing Trinity rewriting dependency convention: Wires imports the first-party replication crate as replication and reads the shared value clone contract from that module. The existing kernel alias remains the mutation/DSL contract. Corrected the two node-preview calls to the verified mutations module. Handcrafted the missing camera mutation descriptor and payload schema, using binary tag zero from the single-variant aggregate and warning/applied outcomes from the implementation. All original files were guarded and Rust sources parsed before writes. Strict compilation and the existing Wires regressions are still pending.

- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🧬️schema/🔣️.json
