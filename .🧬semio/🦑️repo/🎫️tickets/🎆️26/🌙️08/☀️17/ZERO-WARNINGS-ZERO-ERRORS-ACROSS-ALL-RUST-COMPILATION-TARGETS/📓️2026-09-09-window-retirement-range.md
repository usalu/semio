# Window Retirement Range Inference

The native layout baseline exposed E0283 in window-owner retirement: the borrowed str bound can match more than one Borrow implementation for a static string key. Explicitly select the borrowed str lookup type while retaining the existing exclusive cursor, rotation, and ownership behavior. The runtime baseline stopped at compilation before either layout assertion.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs
