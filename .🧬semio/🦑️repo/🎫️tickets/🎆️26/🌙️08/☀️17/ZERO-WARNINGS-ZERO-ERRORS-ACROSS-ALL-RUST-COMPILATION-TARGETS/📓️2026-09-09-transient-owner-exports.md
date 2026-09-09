# Transient Owner Exports

Export the concrete WindowTransientOwnerBundle and the owned-factory transient_store_disposer constructor supplied by the current modules. Remove the six reexports whose implementations no longer exist. Existing app/window implementations still reference the removed factory API and must be completed against their concrete owners; no compatibility factories are restored. This resolves the plugin's invalid imports and exposes the actual ownership contract for those consumers.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
