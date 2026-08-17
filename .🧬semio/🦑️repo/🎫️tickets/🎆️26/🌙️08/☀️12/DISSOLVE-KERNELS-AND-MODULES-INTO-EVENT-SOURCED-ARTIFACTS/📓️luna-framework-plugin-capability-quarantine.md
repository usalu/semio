# Framework Plugin Capability Quarantine

## Verdict

BLOCKED. Framework typed-capability validation is not a stable downstream gate for stdio yet.

## Hot Coupled Owner

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` is actively modified and its mtime advanced during this audit. Its paired builder is staged and semantically coupled. Both paths remain one quarantined owner lease.

## Current Coherence Failures

- Two `MutationDiff::apply` call sites lack the visible trait import.
- Four capability-claim sites have `ArtifactDialect: From<semio_framework::Dialect>` mismatches caused by distinct I/O mounts and nominal types.
- The formerly reported registration-plan and runtime-registry symbols matched in a later snapshot, but this is not release-stable while the owner is editing.

The builder directly calls `ArtifactRegistrationPlan::from_declarations`, `into_runtime`, `Plugin::with_runtime_registry`, and `commit_artifact_registration_plan`; the pair must release atomically.

## Required Release Gate

The minimum framework scope is plugin core plus paired builder, frozen together and passing:

```text
cargo check -p semio-framework-plugin --lib
```

Only then may the stdio registry/runtime lease rerun its Cargo validation. Standalone I/O, store, and OS kernel-I/O mount paths are stable/protected and must not be edited merely to unblock this gate.

The audit was read-only and changed no source, generated output, configuration, or ticket state.
