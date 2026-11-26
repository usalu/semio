# Diagram controlled publication test

The full React suite reached the first controlled 20,000-node publication and received all 20,000 nodes, but the handoff did not record a valid generation. The original test measured real elapsed time while the callback traversed the proposal, so a build host under contention could cross the same 8 ms threshold that the test meant to exercise only on the second call. The focused case passed in isolation.

The test now uses a controlled handoff clock alongside its existing cooperative frame clock. The first callback still reads all 20,000 nodes. The second callback explicitly consumes 10 ms on the handoff clock, and the test checks the exact violation, the last valid generation and quarantine. Production behavior is unchanged.

All 55 Diagram component tests passed with the repository's React Vitest config and installed Chrome. The isolated runner is check-diagram-levels.py.

Changed test at 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🧲️force-tests/🔸️cursorizes-live-controlled-20/🟦️.tsx.
