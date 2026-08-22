# P1m Interactive Compute Closure

Date: 2026-08-21

## Outcome

The remaining Phase 1 opaque-compute blocker is closed. The public
`ComputePool::run_blocking(FnOnce)` API no longer exists, and neither production plugin-host router
path wraps effect work in a renamed one-shot closure.

`ComputePool::run_job` now accepts a repository-owned `InteractiveJob`. Its service-owned drive state
retains the job, operation context, preview sequence, result sender, lane, and admission permit across
steps. Each `WorkerPool` closure calls `drive_step` exactly once with the lane's fuel and wall budget.
`Yield`, `PreviewReady`, and `CheckpointReady` enqueue a fresh closure; `Complete`, `Cancelled`, and
`Fault` are terminal and propagate to the awaiting caller.

## Production Host Migration

`RouterEffectHandler` now has one factory method, `create_job`, whose contract permits only bounded
construction of persistent job state. All effect work belongs in `InteractiveJob::step`.
`run_router_effect_job` rejects an already-cancelled operation before constructing the job, submits
the job to `ComputePool::run_job`, and maps every terminal result explicitly.

Both production consumers use that path:

- `🔌️plugin/🖥️host/⏳️imports.rs`: async component host imports await the router job and
  translate complete/cancel/fault/deadline/worker-loss into the guest fault vocabulary.
- `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`: post-turn effect dispatch runs the
  same job and re-enters the actor mailbox with an explicit completion or fault event.

The focused recording handler deliberately yields once before completing. Its test therefore proves
the host path preserves job state across more than one worker closure rather than merely renaming the
old one-shot closure.

## Cancellation, Deadlines, and Admission

- Cancellation is checked before admission, before job creation in the host, and within every job
  step through `StepContext::is_cancelled`.
- Admission polls cancellation and the absolute deadline while waiting for a permit.
- A deadline during execution cancels the operation token and returns `DeadlineExceeded`; the next
  bounded step observes cancellation and releases the retained permit when the drive state drops.
- The capacity permit spans the full job rather than one step, so a yielding job cannot evade the
  per-pool concurrency bound.
- Result-channel loss maps to `WorkerLost`; no nonterminal outcome is exposed as successful
  completion.

## Platform-I/O Boundary

Opaque blocking closures remain only at explicitly classified platform-I/O boundaries:

- `ComputePool::run_io` for synchronous HTTP/TCP operations;
- `StorageScheduler::submit` for blocking storage backend reads/writes/deletes.

These closures are I/O boundaries, not plugin guest execution or interactive CPU work. The browser's
cooperative platform-WASM boundary is unchanged. The exact retained sites are recorded in
`📝️p1-interactive-compute-static-census.log`.

## Static Ratchet

The production host/services census has zero matches for:

- `ComputePool::run_blocking`;
- `.run_blocking`;
- `RouterEffectHandler::handle`;
- `handler.handle(...)`.

The obsolete services `run_blocking` interactivity allowlist entry was removed. The deny-mode audit
is clean and now reports only its permanent test/process-entry classifications.

## Verification

| Gate | Result | Evidence |
| --- | --- | --- |
| Services native quick tests | PASS, 39/39 | `📝️p1-interactive-compute-services-test-quick.log` |
| Plugin-host effects tests | PASS, 11/11; 145 filtered | `📝️p1-interactive-compute-plugin-host-effects-tests.log` |
| Full OS host native debug check | PASS | `📝️p1-interactive-compute-host-check.log` |
| Plugin-host all-target test compile, native debug | PASS | `📝️p1-interactive-compute-plugin-host-tests-check.log` |
| Plugin-host all-target test compile, native release | PASS | `📝️p1-interactive-compute-plugin-host-release-check.log` |
| Services clippy, native `-D warnings` | PASS | `📝️p1-interactive-compute-services-clippy.log` |
| Services clippy, `wasm32-unknown-unknown -D warnings` | PASS | `📝️p1-interactive-compute-services-wasm32-unknown.log` |
| Services clippy, `wasm32-wasip2 -D warnings` | PASS | `📝️p1-interactive-compute-services-wasm32-wasip2.log` |
| Interactivity audit, deny mode | PASS | `📝️p1-interactive-compute-interactivity-audit.log` |
| Opaque-compute/platform-I/O census | PASS, forbidden set empty | `📝️p1-interactive-compute-static-census.log` |

The native plugin-host `-D warnings` router was also run. It is blocked before reaching the host crate
by unrelated existing clippy errors in `semio-framework-hash`, `semio-framework-mesh-engine`, and
`semio-framework-os-kernel-dsl-derive`; the exact compiler output is in
`📝️p1-interactive-compute-plugin-host-clippy.log`. Native debug/release host checks and the
host test build reach the changed files successfully. The native host crate is intentionally not a
WASM target because it retains Wasmtime during Phase 9; dual-WASM coverage applies to the services
job driver, which is green on both targets.

## Files

- Updated `🛎️services/🦀️component.rs`.
- Updated `🔌️plugin/🖥️host/⏳️imports.rs`.
- Updated `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`.
- Updated plugin-host Cargo dependency and the existing OS-host task router.
- Removed the obsolete Phase 1 `run_blocking` allowlist entry from the root task router.
- Updated `📓️p1b-services.md` and `📓️p1l-process-wide-pool-enforcement.md`.
