# DSL Record Model Source Boundary Audit

The recorded Hub run refused `semio-framework-dsl-record-model` at file 25 of 32: 263,240 bytes against 262,144. This is a partial traversal, not the full source size.

The live tree shows a recent concurrent split: the model root no longer includes `🧾️json-schema/🦀️.rs` (6,511 bytes), which is now declared by the schema facade. The changed model and facade files have matching recent modification times. The worker identity is unknown; do not overwrite the concurrent work. This separation is appropriate to the schema-emission domain, but removing 6,511 bytes does not prove that the remaining files fit the full admission traversal.

A read-only recursive `#[path]` scan found 30 Rust files totaling 272,667 bytes, including approximately 17 KB of `cfg(test)` files. The count excludes admission-specific resolution and should not be substituted for the repository gate. If the current split remains over budget, `🛫️encode` and its expression and receiving children form a separate approximately 52 KB emission unit. The existing `semio-framework-dsl-record-emission` depends on model; moving the encoding surface there requires updating the facade and consumers without adding a model-to-emission dependency cycle. Verify current ownership and public consumers before changing it.

From `🌎️hub`, rerun `repo cargo --store=shared --configuration=prototype --lang=en -- test -p semio-hub-puzzle --lib surface`. A successful source-admission stage verifies the live boundary, and the surface tests verify Hub behavior. A direct native Cargo run is useful as a separate compile/test check but does not validate the repo source-admission limit. No build was run for this audit.
