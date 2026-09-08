# Framework Test Import Warnings

Native604's test-target checks reported unused Flow VCS imports of ArtifactId, Edit, SchemaId, create_document_envelope and ArtifactCommand. Removed the three unused test-only import statements with their cfg attributes. The compiler diagnostics establish that these imports are unused in the test build; source inspection found no remaining use in that module.

The standalone framework Playbook test unnecessarily qualified an already imported os_store module through its store alias. Applied the compiler's suggested shorter path. The third-party JSON oracle, block-order checks and DSL/pack equivalence assertions are unchanged.

Changed files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧪️tests/📖️playbook/🦀️.rs`

Fresh compiler verification is pending.
