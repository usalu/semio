# WASI 1027 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":1,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T11:01:34.012Z","finishedAt":"2026-09-09T11:01:56.258Z","packages":160}

## E0432 — unresolved import `super::window_transient::bounded_window_transient_store_owners`

error[E0432]: unresolved import `super::window_transient::bounded_window_transient_store_owners`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:282:39
    |
282 | ...ient::{bounded_window_transient_store_owners, WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowT...
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `bounded_window_transient_store_owners` in `component::window_transient`


