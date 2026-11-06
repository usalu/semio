# Interpreter Runtime Receipt Consumers

The synchronous owned interpreter paths predate Source-owned escrow snapshots. They cannot consume the new pointer/length merely by extracting the first tuple element: their ordinary owned-operation driver copies a fixed header into a `Vec` and physically hands it back before the host can bind and copy the Source capsule.

- `🏃️runtime/📣️host-replies-wasi-and-channel-faults/🦀️.rs::decode_owned_result` detects an actor fixed receipt from an already copied byte slice and returns `TurnFault::ActorCall`. It has no guest instance or Source callback from which to borrow the capsule.
- `🏃️runtime/🔄️owned-runtime/🦀️.rs::actor_context_call` returns only `OriginalActorFixedCall` after `resume_owned_operation_observed`. Its `admit_context` and `close_context` wrappers inherit that missing custody.
- The same file's `drive_job`/`job_call` and job output acknowledgement path decode SJH after returning an owned byte buffer. They cannot prove the actual capsule or settle cross-loan credit.
- Production calls to the actor-context and `*_actor_job` methods are in `🖥️host/🎭️guest-runtime/🧠️owned-runtime-adapter/🦀️.rs`, which implements the broad GuestRuntime adapter; interpreter tests also exercise the direct methods. Launch's Source-owned cold/relay replacement is staged but not mounted. Root explicitly directed removal of the old methods and usages only after those real paths take over; the existing synchronous path is not an acceptable compatibility fallback.

The new `GuestRuntime` frozen unit/scalar views operate only on the actual native Source borrow and held interpreter frontier, and can lend fixed header/capsule words to the staged HistoryHome before physical handback. Their native compilation, issuer-ledger settlement, guest Source acceptance evidence and adapter replacement remain outstanding.
