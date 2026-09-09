# Host Retained Layout Review

Boxed the large mock turn result so unrelated scripted outcomes no longer reserve its full payload size. Kept admitted shard frames, fixed-ring events, live-instance rejection, and preallocated relay slots inline with scoped ownership expectations. The native runtime already has shared Arc owners. Replaced three option-check/unwrap replay branches with direct matches. Documented the fallible cross-platform pipe interface at each infallible platform implementation. Four sources parsed before guarded writes. Fresh native diagnostics and runtime lifecycle assertions are still required.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🔁️lifecycle/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs
