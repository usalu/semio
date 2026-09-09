# Store Retirement API

The completed WASI 1106 run also found two composition-bound failures in the generic document/configuration disposer. Expose the inherent bounded store close methods and use them directly in that disposer. Configuration records do not need document composition traversal; both typed member opening and plugin retirement now use the same close implementation and terminal witness. SpaceMember continues to delegate to these methods for actual composed documents. This preserves the existing retirement grants and errors without imposing artifact schema metadata on configuration records.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
