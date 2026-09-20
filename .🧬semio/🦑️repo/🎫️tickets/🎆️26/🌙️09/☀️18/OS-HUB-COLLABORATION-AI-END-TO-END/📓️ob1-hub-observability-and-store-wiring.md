# OB1 — Hub observability, `checkpoint-publications` decision, durable-store wiring

Slice OB1 (fleet 5, 2026-09-20). Scope: `📓️g10-goal-gap-reaudit.md` §C N8 + N9 and §D Outcome 2;
`📓️g2-hub-depth-audit.md` §9/§12; `📓️w3b-hub-instance-and-durable-stores.md`;
`📓️w3d-server-results-sagas-gate-ts-twin.md`; `📓️k1-hygiene.md` §5.

Three items:
1. **Observability minimal bar** — structured trace records for the two WS handlers, the directory
   command path, auth, and boot/readiness, on the framework's own trace module (no `tracing` crate:
   AGENTS.md forbids runtime deps on external libraries).
2. **`checkpoint-publications` decision** (N9) — wire a client or delete route + gate.
3. **Durable stores wired to a real path** (W3b) — `HubInstance`'s `SessionStore` behind a real hub
   route, saga draining scheduled in the hub runtime, restart-survival proven.

---

## 1. Inherited state

(filling)

## 2. Observability — design

(filling)

## 3. Observability — implementation

(filling)

## 4. `checkpoint-publications` — the decision

(filling)

## 5. Durable store wiring

(filling)

## 6. Tests run, with real counts

(filling)

## 7. Honest gaps

(filling)

## 8. Files changed

(filling)
