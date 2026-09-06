# Worker Maintenance Self-Retirement

## Boundary

`WorkerMaintenanceStep::Retire` is a callback-only terminal disposition. The
registry removes the exact generation in `finish`, after the callback returns;
the callback never calls `remove_maintenance_hook` while its own invocation is
running. External removal remains unchanged: it fences requests and returns
`false` while an invocation is live, leaving the caller responsible for the
later terminal acknowledgement.

Artifact authority Drop retirement now returns `Retire` after its strong runner,
terminal job, handoff maintenance pointer, and `WorkerPoolUse` are released. A
stale callback context also retires its own hook rather than occupying a fixed
slot forever.

## Laws and receipts

- The neutral fixture adds the exact 64-cycle self-retirement/reuse contract,
  including a request arriving while the callback is running and a later stale
  generation result.
- `worker_maintenance_running_callback_retires_requested_generation_exactly_once`
  exercises that interleaving directly in the registry.
- `worker_maintenance_native_self_retire_reuses_all_fixed_slots` performs 64
  native callback retirements and then fills/removes every fixed slot.
- `artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity`
  drops 65 live authorities sequentially and requires each registered close
  owner and maintenance slot to reach terminal before reuse.
- `artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry`
  injects a first close panic, proves the exact cursor, generation and pool use
  remain registered, then explicitly wakes the same callback and reaches one
  terminal acknowledgement. The callback catches unwind before its global owner
  leaves the fixed slot.
- `@semio-tech/framework-async-rs:worker-maintenance-check`: GREEN,
  `AJV=1 lifecycle=20 capacity=64 native=1 cooperative=1`.
- `@semio-tech/framework-os-kernel:document-mount-single-flight-check`: GREEN,
  `AJV=1 cases=10 waiters=32 owner-futures=1`.
- Root native receipt `48iOYL/00`: GREEN 13 exact maintenance laws, executable
  SHA-256 `e84a41f0797a90827bbd0035e5297c7e65c358adc4cb3e4e1416a19b4c524921`,
  including request-during-callback `Retire` and full fixed-slot reuse.

Artifact authority panic/retry and 65-drop retirement remain separately pending
the current mount-group native receipt; the maintenance receipt does not imply
that broader artifact lifecycle qualification.

Terra's post-fix read-only review confirms that the local unwind boundary
restores the same cursor while retaining its generation and pool use, and that
the explicit retry reuses the existing hook. It does not yet prove a hostile
panic after `close_one` has already produced one observable side effect; that
partial-effect/idempotence case remains an explicit native-law gap.

Root mount receipt `rJmMZr` built and passed its first 19 exact laws, then the
panic/retry law observed the injected panic before the callback's local unwind
handler had restored the cursor into the fixed row. The test now waits for and
validates the exact restored generation, handoff and pool identities before it
asserts retained pool use and requests the retry. The document-mount source
oracle remains GREEN (`AJV=1`, `cases=10`); a post-fix mount native rerun is
required.
