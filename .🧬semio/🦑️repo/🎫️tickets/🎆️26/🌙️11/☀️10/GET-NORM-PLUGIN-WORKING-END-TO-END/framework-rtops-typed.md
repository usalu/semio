# runtime-ops typed-operations migration

## Changes (🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops)
- `🦀️.rs`: local grant-style `retired` wrapper removed; re-exports `original_steps::{begun,retired,retired_poll}` (effect style).
- `🧾️typed-operations/intent.rs`, `response.rs`: dropped removed imports, local `construction`/`receipt` replaced by `original_steps`, decoder identity `(NativeAllocationIdentity,u64)`, close via `retired_poll(step_granted(body,effect))` / `begun(begin_granted(slot,effect))`, shared-view allocation close via `retired_poll(close_step(effect))`.
- `response.rs`: the cached `Arc<ViewModel>` is retired through a resident `lease: Option<ViewLease>` (Arc has no `RetireOwned`); `SharedUtf8::admit` takes the `actual` out-param; `terminal_is_empty`/`close_demands` include the lease.
- `response-tests.rs`: closes use `retired`/`begun`/`retired_poll`; `progress` -> `receipt`.
- Plugin-program `admit(cx,&mut admission,slot)` callers in response.rs already matched.

## Status
`check -p semio-plugin-runtime-ops --lib` (rtops-typed-2.log): 0 errors in my files; sole remaining error is ingress/🦀️.rs:23 (`work` u64 vs u128, ingress agent). Borrowck/tests not reached; response-tests unverified.
