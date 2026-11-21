# Async Graph Pick Audit

- `observeFlowTask` in `🕸️NodeGraph/🔄️flow-session-sync/🟦️.ts` cancels replaced tasks but invokes `consume` on any resolved task without checking whether it is still current. A late resolution after cancellation can deliver obsolete work. Guard delivery against replacement, explicit cancellation, and session retirement.
- `dag_pick_targets_at_screen_json` in `🕸️graph/🖱️hover-rect-and-pick-targets/🦀️.rs` uses `hit_test_pick_targets`, which picks handle discs only (`⚙️engine/🔗️engine-connection-and-hit-test/🦀️.rs`). Existing native detail/micro hover instead uses `port_row_handle_hit` over the entire channel row (`🕸️graph/🧮️world-snap-engine-sync-and-hit-tests/🦀️.rs`). Consequently a pointer over a channel row away from its anchor resolves to the node and loses channel/world hover. Include row hits at these LODs with connector precedence, and exercise a point outside the anchor disc.

This is a static audit; no builds or runtime tests were run.
