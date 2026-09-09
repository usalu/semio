# Synchronous Graph and Subscription Boundaries

Converted pure dependency validation, bounded IO route enumeration/ranking, inference DAG validation, and enum rank accessors to synchronous functions. This preserves atomic registration under the existing registry lock and removes suspension points that never performed asynchronous work. Event subscription and completion-topic construction are synchronous so concurrent first completions cannot race subscription setup. The actual compose guest call now follows a lexical lock scope. Updated all discovered callers and their existing tests. Ten Rust sources parsed before guarded writes; strict compilation and runtime verification remain pending.

- 🧰️framework/🔨️modules/🛂️manifest/🦀️.rs
- 🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️plugin-dependency/🦀️.rs
- 🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs
