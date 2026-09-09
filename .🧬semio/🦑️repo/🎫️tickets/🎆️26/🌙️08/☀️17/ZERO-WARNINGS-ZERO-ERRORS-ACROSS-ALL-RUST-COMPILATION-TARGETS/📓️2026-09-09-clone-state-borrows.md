# Clone Cursor State Borrow Repair

The post-change host test build exposed three fresh Rust borrow errors in the shared value clone cursor after its state was boxed. Borrowed the concrete state once inside advance_item so Rust can split its frame, completed-value, source, and accounting fields. This changes neither ownership transfer nor allocation. Rust syntax parsed before the guarded write; compiler and runtime checks remain pending.

- 🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs


## Wrapper Correction

Fresh native922 diagnostics showed the current state wrapper is ManuallyDrop. The current shared source now uses an explicit dereference to borrow the concrete state, which avoids assuming an as_mut method. The earlier report description of a boxed state was inaccurate; the retained wrapper is ManuallyDrop. Verified the correction in current source before the next compiler dispatch.
