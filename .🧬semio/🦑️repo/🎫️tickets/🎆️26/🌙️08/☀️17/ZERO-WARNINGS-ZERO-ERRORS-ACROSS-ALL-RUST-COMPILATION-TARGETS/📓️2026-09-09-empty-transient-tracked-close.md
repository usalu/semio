# Empty Transient Tracked Closure

Keep the exact zero-size and no-drop guarantees on NoTransient and NoTransientMutation. Remove the obsolete assumption about the containing store layout. Its constructor now installs the shared tracked-read disposer with a concrete zero-payload owned retirement factory, so read leases remain owned until release and closure advances with bounded grants. Remove the unused bespoke whole-store disposer type instead of preserving the previous whole-store drop behavior. The preceding neutral regression verifies zero grant, held-read blocking, release, and terminal emptiness once the plugin compilation blockers are resolved.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/♻️retirement/🧪️tests/🔬️unit/🦀️.rs
