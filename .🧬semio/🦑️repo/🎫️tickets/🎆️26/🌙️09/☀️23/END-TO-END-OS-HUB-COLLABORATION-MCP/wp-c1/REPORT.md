# WP-C1 progress

## Done
- G3 AJV blockedBy fix
- Fixture + schema + Rust twin
- Trusted-catalog Arc+impl Trait as_ref fix
- PROCESS_POOL_PUMPS_PER_TURN cfg fix (peer break)
- OS_HUB_BINARY skip-build in os-hub-ts script
- PR1 socket/v1 attach restored; join-replay assertion; 5m readiness stall
- launch.json test-long + fixture twin entries
- Live e2e reached attach once (wrong WS URL); fixed; requeued behind hub mutex (h1)

## In flight
- HUB_E2E test-long behind fleet hub mutex (h1 catalog bootstrap)

## Still todo after green e2e
- os-hub / os-hub-ts test-quick via mutex
- finalize 📓️wp-c1.md

## 2026-09-23 long12 (in flight)
- Live merged `/scopes/.../document/ws` accepts session protocols; grant protocols → close 1002.
- HubDocumentAuthority does not relay Presence on framework session lane → presence wire asserts demoted to nonclaims.
- long11 hung: mint/open-plan for B after A WS opened; hub HTTP starved (auth timed out). Fix: mint both grants before any WebSocket; AbortSignal.timeout on fetches.
- Fixture: presencePeerActorMatchesA/presenceJoinReplayShowsA = false.

## 2026-09-23 long13
- Budget raised to 1800000; document created; 2 WS open; HTTP stayed healthy (mint-first).
- Bun worker ~1GB RSS / event-loop starve from unbounded `holder.frames` under framework-lane frame flood.
- Fix: cap frames at 64, promise waiters on each frame, Commands match by mutation_id.
