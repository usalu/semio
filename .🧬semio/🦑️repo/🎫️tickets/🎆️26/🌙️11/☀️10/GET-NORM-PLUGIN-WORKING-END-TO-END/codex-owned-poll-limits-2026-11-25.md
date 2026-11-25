# Empty poll scalar limits

The preceding EN 1999 native build reported Rust E0599 in `semio-plugin-actor-input` when calling `OwnedCorePollLimits::from_scalars`. Its Cargo log shows kernel identity compiling before the libc wait. The working identity source was updated at 20:53:38 CET, after that compile and after the sampled libc wait began. The current plugin ABI reexports the same kernel identity type, which now exposes the scalar constructor.

The method checks every u32 slot and preserves the three u64 slots in the canonical ABI order. It creates a value containing requested limits. The poll later narrows those limits against the authenticated job budget. The parser still requires fuel, depth and a retained-copy grant before publishing an actor input.
