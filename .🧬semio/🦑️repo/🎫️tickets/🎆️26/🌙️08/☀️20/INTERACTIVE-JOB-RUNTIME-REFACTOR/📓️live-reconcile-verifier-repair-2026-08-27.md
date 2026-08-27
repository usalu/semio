# Live Reconcile Verifier Repair

The root source verifier now recognizes the independently changed production admission envelope: 8 MiB per surface, 32 MiB aggregate via the exact surface-times-four constant. The 32 KiB page/step boundary is unchanged. The mounted render error branch must both cancel its exact grant and route the original encoded fault to the exact instance's Shell endpoint. No production capacities or runtime code were changed by this repair.

Added six hostile mutations cover per-surface/aggregate/default credit drift and cancellation/fault-binding/fault-wire removal. Every live-reconcile hostile mutation now must actually change its input source before it can count as a negative test, preventing stale replacement no-ops from silently passing.

Canonical default `bun x nx run workspace:verify-interactivity --skip-nx-cache` executed with `NX_DAEMON=false`. The previous three live-reconcile failures disappeared and all its hostile tests passed. The full gate remains RED on 19 direct-thread findings in standalone `🧪️component.rs` test files; no scanner suppression or allowlist widening was applied. Full stdout is `🧪️live-reconcile-verifier-repair-r1-2026-08-27.txt`.
