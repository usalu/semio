# P6i Revoked-Family Independent Re-Audit

Date: 2026-08-25

Auditor: Codex `/root/p6h_audit_remediation/p6i_revoked_family_reaudit`

Scope: Current-tree, read-only P6i/P6h source/static audit. No Cargo, Nx, Wasm, browser, or broad build command was run.

## Verdict

**RED — concrete ordinary-Drop recovery counterexample remains.**

The earlier P6i source gate is not sufficient for the revoked-family requirement. A mounted job may be ordinarily dropped while its shell still owns a non-terminal `MountedState`: `MountedJob::drop` merely cancels (`session/component.rs:3279-3283`), while `MountedState::drop` merely cancels and asserts terminal emptiness (`:2986-2993`). It transfers neither the state nor its snapshot/numerical/solver/candidate/current/displaced owners into a fixed recovery authority and performs no one-owner recovery grant.

This is a direct violation of the required ordinary-Drop shallow handback for both `MountedJob` and `MountedState`. The release assertion is not recovery: in the counterexample it panics in non-test builds, and without an explicit transfer it cannot provide the required one real owner/page/control drain per recovery grant.

## Live Trace

`MountedState::close_step` is correctly incremental only when a caller explicitly drives it: it retires candidate, displaced, current, numerical, solver, snapshot-return, and backing lanes (`session/component.rs:2874-2978`). An ordinary `Drop` bypasses that state machine. `MountedJob::terminal_drop_is_shallow()` returns `true` at `:3274-3276`, but its actual `Drop` does not hand the `Rc<RefCell<Option<MountedState>>>` into any recovery queue; it invokes only `cancel()`.

The other three requested family members have distinct recovery paths: solver pages are placed into `FEM3D_BACKING_RECOVERY`; leases mark the World3d snapshot orphaned; and candidate pages/tokens/orders are transferred to their recovery stores. Those paths do not repair the missing state/job handback, because the contained owners cannot reach their own drops while the live mounted state is retained in its shell.

The prepared World3d route does claim its typed snapshot draw permit before `begin_world3d_draw_rebuild` (`world/component.rs:8922-8934`), and the current source continues to reserve the process draw budget at snapshot begin. This audit found no separate concrete counterexample in the permit-before-rebuild ordering.

## Verifier False Green

The P6i verifier positively requires the forbidden shape: it accepts `MountedState::drop` when it contains `self.cancel.cancel_now()` and the text `external shell recovery owner reached terminal empty`, and accepts `MountedJob::drop` when it contains only `self.cancel()` (`📜️script.ts:5721-5727`). Its corresponding hostile mutations merely rename `impl Drop` to `impl Reclaim`; they do not mutate the ordinary-drop body to remove or replace a real recovery transfer. Thus all 93 mutations can pass while the live ordinary-drop counterexample remains.

## Required Repair

Install a fixed-capacity mounted-state/job abandonment authority. Ordinary `MountedJob::drop` must atomically transfer or register its exact shell/identity recovery owner, and ordinary `MountedState::drop` must shallow-transfer every still-owned top-level owner to it without cloning or deep dropping. Its public recovery close must advance exactly one real page, backing, snapshot/control owner, or shell owner per grant and release the matching process credit only after terminal-empty. Strengthen the verifier with live-body mutations that replace these transfers with cancellation/assertion-only bodies and require rejection.

## Preserved Findings

The accepted fixed pre-allocation sequence remains visible for solver page/fixed-order backing admission; the numerical child is still mounted and generation-qualified; the physical reaction/modal paths, pre-work numerical fuel gates, editor/viewer/renderer lease graph, and P6g/P6h source boundaries remain present. These observations do not cure the ordinary-drop failure.

## Reproduced Isolated Gates

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test` | PASS — `hostile-mutations=93` (false-green for this finding) |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test` | PASS — `hostile-mutations=70` |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` across P6i/P6h source census | PASS |
| Scoped `git diff --check HEAD -- <P6i/P6h census>` | PASS |

No production source was modified.
