# Physical Workspace Readiness Receipts

Observed 2026-11-21 15:12 local time from the origin receipts, without remote actions.

- m03: ready.json reports running, ready=false, workspace attempt 1, supervisor PID 89403, reason adopted (already running, not restarted). Its workspace log at 15:12 reports a shared fact-store queue: the leader moved from inventory with 181345 source observations, 48 owner checks and 310272 verified memo rows to taken pass 9, 0 of 66; the waiting load has not begun. This is progress in the leader of the existing adopted attempt, not a verified planner reply or terminal gate.
- m07: after the recorded 13:52 UTC SIGTERM, the latest workspace stage is running and adopted, attempt 4, supervisor PID 46018, ready=false. At 15:12 local its log shows inventory at 296651 observations and 457 owner checks, beyond the expected 600 seconds. The old termination is not the current terminal state; no successful readiness receipt is present.
