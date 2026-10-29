# Graph Peer Convergence Snapshot

Read-only snapshot at 2026-10-28 19:42–19:44 UTC. Authoritative repo leases status --all identifies PID211237 holding exclusive resource77ec794ceff4 and shared graph image (~1103s at first observation). Parent chain171027→211110→211237 is MCP-rs build parallel1. A newer separate MCP-rs build parallel6 has chain321579→327415→327675.

Two process samples about8seconds apart show PID211237 runnable, CPUticks+777, RSS2777MiB, no children; PID327675 runnable, CPUticks+782, RSS1904MiB, no children. Both are actively consuming CPU, not a dead process or child-admission wait at that moment. Outer parents sleep in epoll while these children work. No signals, profiling, restarts or graph requests were issued.

Graph structural custody state5706fc7c3ff84dacc66aa07a is held, structural=true, 577 owners, queuedMs1793215448615. current.json remains generation0 with4030 verified owners and publication timestamp16:52UTC. The structural state was modified19:42:53UTC. This proves live state writing but not completed inference throughput; inference project counters are not exposed in the inspected state. Child stdout/stderr are sockets, so audit did not consume them. No reliable ETA or monotonic completed-project count inferred from CPU alone.

Recent generation claim records include repo-workspace-go c190bc55c5628f0f454537c3 statefailed at19:39:20UTC, repo-search-go8e98f4d4bf22a915005ee66b statefailed at19:38:56UTC, and11-owner claim2d37f791df781c98894d910f done at19:38:37UTC. These are attempt records, not proof that the waiting top-level target terminated. Latest native logs continue front-door-only peer waiting and generation0; their peer queue counters reset after a new attempt.

Admission snapshot had1096MB spare above1981MB floor,2granted,0waiting; earlier9.7GB available was transient. The next useful observation is the authenticated owner terminal/progress output of PID211237, while preserving peer custody. This audit cannot prove successful convergence or current Source implementation revision from runnable CPU alone.

Failed-claim follow-up: inspected c190bc55 and8e98 records contain no error/cause/message fields. Both bind implementation6b797e3ba893c3468e87c887d89fb7a277f3e4f03acf354a43eec69989ceb969; workspace source stampb107b1130bc91455, search0452d3412259bc16. Their state alone cannot establish a Source-admission error or explain a retry. Target terminal logs remain the authoritative failure evidence.

## Native Target Re-entry Trace

Actual client generator ancestry is300943→333412→359051→370837. Identical repo run argv is accompanied by graph-yield rounds absent→1→2→3, constant ticket/tmp/semio-graph-yield-300943.json and first requestedAt1793215114134. Oracle303925→333476→361724 has rounds absent→1→2 and constant requestedAt1793216296097. Policy generator312844→333851→363227 has rounds absent→1→2 and requestedAt1793216335494. Each youngest process has no live child at observation.

This is intentional finite front-door reexecution, not task recursion. format-stamps/🔄️reexec/🟨️.mjs launchReplacementProcessV1 spawns a replacement and awaits close; runWithStaleReexecV1 calls it after waitGraphQueueYieldV1. Old sleeping front doors remain parents and retain heartbeat timers (~20–39MiB observed), explaining simultaneous elapsed counters. Policy maximumYields3 and wakeMilliseconds600000 bound these replacements; after round3 a command waits in place. They do not prove a new compiler task or duplicate generator execution.

Graph-free route review: runBuildCliV1 has diagnostic/tool exec routes before graph loading, but no corresponding graph-free authored-target route. execution kind source and source-input role freshness classify demanded observations inside the graph admission pipeline (buildNativeSourceObservationV1), not authority to skip task selection/dependencies. Direct leaf script invocation cannot be claimed equivalent to repo run target or workspace setup without preserving those declared prerequisites and Original ownership. No bypass route or invented Original was used.

## Admission Head and Old MCP Lineage Revalidation

Fresh authoritative status and direct read-only ledger check show no7IszbA/teaching-proctor row in host queue or grants and no matching live command (audit command itself excluded). Historical ownership/age cannot be reconstructed from an absent row; it must not be treated as a current blocker or stale grant to remove. Current status had four grants and one exec waiter PID396016 requesting3584MB.

Old MCP graph PIDs211237,211110 and171027 are gone. Gateway170657 remains alive in epoll with no children, so the old captured builder no longer holds current resources. Existing Source gateway staging passes its AbortSignal through stageMcpBinaryAsync to owned startToolV1; after nonzero staging, WarmMcpGateway remains failed and serves warm list replies/refusals, with no automatic staging retry. A living gateway alone does not prove OS MCP binary readiness. No cancellation action is required for the already absent old builder, and no signal/disconnect/restart/ledger mutation was performed.

## d543 Current Blocker Provenance

Canonical resource graph-custody:/home/deck/semio/.cache/b/g/m/generations:d543ead75f1bed0b7e2db372 hashes by official resourceLeasePathV1 SHA256 rule to8b814103519f053c062026963a77ec711342615a89ee0a7c00fa2c98d24dfcbe.sqlite. /proc FD31 ofPID462345 names that exact file, parent460615; runnable stateR, nochildren, captured command89a4... repo run @teaching/architecture-course:typecheck --parallel1. Claim structuralheld199owners/ticket1. At22:03:45UTC CPU44336ticks,RSS2579MiB; laterCPU45850ticks,RSS3277MiB. This is an alive activecoursepeer, not dead from claimfile presence. No DB was opened, no duplicate doctor/query launched, no signals/claimmutations.
