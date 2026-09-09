# Jack Query Semantic Oracle

The runtime baseline reached the first graph-result case and differed only in the content child handle: retained execution intentionally hashes streamed entities into jack-query-result, while the batch executor hashes the materialized JSON into jack-content. Compare kind, columns, rows, mutations, and the complete materialized graph JSON using serde_json, preserving all graph metadata and entity content checks while excluding the two algorithms' internal handle names. The existing neutral result and mutation vectors remain unchanged. This corrects the semantic oracle introduced before boxing; it does not change production handle generation.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs
