# P6i Ordinary-Drop Narrow Independent Re-Audit

Date: 2026-08-25

Auditor: Codex `/root/p6h_audit_remediation/p6i_drop_narrow_reaudit`

Scope: Current-tree, strictly read-only source/static P6i ordinary-Drop re-audit, including the live FEM3D mounted session, P6i verifier, and P6h preservation gate. No Cargo, Nx, Wasm, browser, or broad build was run.

## Verdict

**GREEN — no concrete ordinary-Drop recovery counterexample found.**

## Re-audited Ownership Trace

- `reconcile` allocates the shell and its process credit, reserves that shell's `MountedRecoverySlot` for the exact `Identity`, and only then constructs `MountedState`. The factory requires both shell-state identity and reserved-recovery identity, so an old job cannot capture a reused shell.
- A queued or running `MountedJob::drop` first publishes the exact identity as `Recover`, cancels, then either transfers the exact shell state to the fixed slot or, if its `RefCell` is contended, leaves that exact state in the shell. `recover_abandoned_one` later rediscovers the shell state and installs it in the same slot. A failed transfer restores it to the shell.
- A direct nonterminal `MountedState::drop` cancels, terminal-replaces itself, and transfers the complete old state to its pre-reserved recovery slot. The replacement is terminal-empty, so this Drop does not recursively deep-close the moved owner.
- Recovery takes one exact owner, executes one `MountedState::close_step` opportunity, restores the same exact nonterminal owner, and releases recovery authority, process credit, and shell only after terminal identity-zero. Candidate, displaced/current lease, numerical child, solver, snapshot/return witness, and backing terminal checks remain in that close graph.
- A completed `MountedJob::drop` publishes `Retained`, leaving the completed shell state/current lease in place; recovery acknowledges that retained state instead of closing it. Thus the valid visual remains renderable until normal successor retirement or application close.
- `maintenance_step` and `close_step` admit at most one recovery/World3d-page/recovered-backing/retirement opportunity per call. The recovery law exercises queued, contended, running, and direct-state Drop, then verifies one-item/page-bounded draining and terminal credit zero.

## Counterexample Search and Verifier Assessment

I specifically traced old-generation Drop after shell reuse, publication-before-borrow, contended shell discovery, slot collision restoration, terminal placeholder Drop, completed retained visuals, and false-terminal credit/shell release. The identity checks on reserve/publish/take/restore/release plus factory validation prevent stale handoff capture. No live-source counterexample was found.

The P6i verifier is no longer the prior marker-only false-green for this family: its 101 mutations directly replace the state transfer, job publication/transfer/restoration, recovery reservation, incremental close/restore/credit release, maintenance invocation, and body-level laws. This remains a source/static assurance, not a runtime proof.

Fixed-owner, fuel, numerical, and render paths remain represented by the P6i exact census; the independent P6h exact gate also remains clean.

## Reproduced Checks

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test` | PASS — `hostile-mutations=101` |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test` | PASS — `hostile-mutations=70` |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` over the P6i/P6h Rust census | PASS |
| Scoped `git diff --check HEAD --` over the P6i/P6h verifier/source census | PASS |

No production source was modified.
