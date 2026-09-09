# Flow Retained Window Configuration

The retained Flow tick now uses its admitted FlowMainWindowConfig directly through a shared evaluation helper, while the public command handler resolves the current window configuration. Cache resolution receives the NoConfig type its handler actually accepts. Remove error propagation from the Drawing preflight close caller after confirming the helper returns retirement progress directly.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
