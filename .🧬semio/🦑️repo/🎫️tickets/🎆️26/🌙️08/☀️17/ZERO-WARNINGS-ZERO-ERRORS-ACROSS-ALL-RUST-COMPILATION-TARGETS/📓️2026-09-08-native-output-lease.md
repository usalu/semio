# Native Output Lease

Pass592 adds the repository's existing process-bound Cargo output lease format to this ticket's native compiler worker. The lease refreshes every ten seconds, is recognized by the repository cleanup guard, and is removed on process exit, terminal receipt or owner disappearance. A detached native-protect command can attach the same protection to an already-running worker without restarting Cargo.

This addresses the missing cleanup protection observed after native560's generated directory disappeared. It does not identify which external process removed that directory. No cleanup exclusions, lint settings or compiler scope changed.

Changed file: `📜️script.ts` in this ticket. The detached protector for native582 started as PID 4288 and printed `[DEBUG] native582 cleanup lease recognized: true`. Cargo itself remains owned by worker PID 69494. Compiler completion and terminal lease cleanup remain pending.
