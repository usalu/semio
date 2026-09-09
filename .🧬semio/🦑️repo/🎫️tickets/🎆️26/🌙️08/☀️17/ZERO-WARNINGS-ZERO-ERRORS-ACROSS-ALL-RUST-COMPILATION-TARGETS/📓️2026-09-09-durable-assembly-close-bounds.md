# Durable Assembly Close Bounds

wasm1012 reached durable-group composition and found that its private close helper omitted ArtifactStoreBatchPublication::close_step's Send + Sync + static snapshot contract. Declare that requirement on the helper. Its parent/drawing/value assembly implementation already has those bounds for every snapshot type.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs
