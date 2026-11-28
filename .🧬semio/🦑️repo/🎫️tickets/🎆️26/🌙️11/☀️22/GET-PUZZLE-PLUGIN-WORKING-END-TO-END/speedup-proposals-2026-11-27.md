# 🚀️ Clean Long-Term Speed-Up Proposals (from two days of the puzzle effort)

Measured basis: on a 15.8 GB host, each heavy cargo run waited 30–120 min in the capacity queue and compiled 30–120 min; roughly half of all runs died on causes unrelated to the code under test (foreign half-finished edits, stale artifacts, gate errors, memory kills). Real defects found per hour of wall time were low because feedback latency, not reasoning, dominated.

## 1. A shared tree that always compiles

- **Pre-publish gate in the replica daemon:** an edit replicates only after the crates it touches pass `cargo check` (and the type check for TS), evaluated on the editor's machine. Half-finished multi-file edits stay private until complete.
- **Atomic change sets:** multi-file refactors (crate splits, re-export lists, dependency moves) publish as one unit, so no machine ever sees the facade without its new step crate.
- **Effect:** removes the largest loss of the session (job-context split, store-engine witness, LocalizedLabel, test-support dependencies: each cost every lane one or more 1–2 h runs).

## 2. Content-addressed build cache shared by all lanes

- A rustc wrapper keyed by input hashes (source contents, flags, dependency artifact hashes) with a CAS store, instead of per-lane `target` directories with timestamp freshness.
- No locks shared between lanes, no stale-but-fresh artifacts, no 80 GB per lane, no hardlink seeding, automatic GC of old configurations.
- The existing remote-execution CAS (15 idle workers) can be the backing store, which also makes item 3 cheap.

## 3. Fleet execution of compiles and tests

- Ship crate compilations and test-binary runs to idle fleet workers through the remote-execution API, with the plain cargo workflow as the client interface (`repo cargo test` routed remotely).
- A 16 GB laptop then coordinates; the fleet compiles. Expected: 2 h local builds become minutes.

## 4. Capacity gate admission by measurement, with backfill

- Admit by the learned per-crate peak (the warden already measures it), not by caller guesses (8,000 MB reserved vs 1,300 MB observed was common).
- EASY backfill: smaller requests run past a waiting large head when they cannot delay its earliest start.
- Per-agent sub-budgets instead of one session for all lanes of a coordinator.
- Count foreign memory (other agents' graph workers held 7.5 GB) explicitly and reserve headroom for them, so 8 GB requests do not fail with `memoryUnreachable` after a 3-minute probe.
- Version-skew tolerance as a rule for every long-running reader (the `unrecognizedGrant` crash killed four hour-long waits at once; fixed only for the waiter path).

## 5. Test crates sized for parallelism

- Carve the 2D/5D lib-test crates (7–8 GB rustc peak) along the domain taxonomy so several compile in parallel within 3 GB each; the warden already flags them.
- Keep `mirror` test crates small and per-domain, so a fix reruns one small crate, not a workspace.

## 6. Size and turn laws as first-class, cheap checks

- Every bounded state type gets a `size_of` law next to its definition (the 4,096 B cursor limits were discovered only by full test runs). A const assertion or a fast unit test turns a 2 h discovery into seconds.
- Latency laws (turns per mutation) should emit their census by default in a structured artifact, so no trace prints are added to shared framework files.

## 7. Stable test-grant fixtures

- Fixture grants derived from priced demands (`binding_copy_bytes()`, `next_*_demand`) instead of magic numbers; layout changes then do not break dozens of unrelated tests.
- Document each test's law (single-kind progress vs bounded per-step demand) in its docstring, so agents do not have to reverse-engineer the intent.

## 8. Built-in lane tooling for agents

- `repo lane open <name>` / `repo lane close`: isolated scratch, build cache view, capacity sub-session, log directory and cleanup — replacing the hand-written `current-native-tests.sh`, `builder.sh`, `lreg-*.sh`, `lguest-*.sh` scripts each coordinator reinvents.
- `repo lane wait <run>`: one supported way to wait on a queued run that survives agent hand-backs, instead of dozens of ad-hoc `tail --pid` waiters.

## 9. Agent orchestration hygiene

- Agents should hand back only on FINAL or a concrete external blocker; a queued run is not a blocker (a large share of hand-backs were "still queued", each needing a resume).
- Resume messages race with agent termination; the coordinator needs a reliable "deliver or restart with this message" primitive.
- Long-context agents degrade: rotate to a fresh agent with a compact brief at ~300k tokens instead of resuming.
- Debug prints: env-guarded, allocation-free, in the lane's own files only (one print broke a heap-audited test; several lanes left prints in shared files).
- Never seed cargo directories with hardlinks; never filter deletions by relative time with bfs (both cost hours here).

## Priority

1, 2 and 3 remove most of the wall-clock loss; 4 and 5 make a single machine usable meanwhile; 6–9 cut wasted iterations per defect.
