# Result H43: the "graph state" phase of a narrow read (`repo inputs repo-graph-maintenance:test`)

## Root cause (proved)
`openBuildGraphOptionalLedgerV1` (`📥️inputs/💾️storage/📒️ledger/🟨️.mjs`) ran `clean(measure())` on EVERY open: a synchronous lstat/readdir walk of the whole `inference` tree of the graph image (387,026 entries, 4.5 GB; 61 s standalone via `🧰️scripts/H43/walk.mjs`, up to 2x per writer because the inline eviction reopens the ledger), inside `begin exclusive` of both ledger databases (other writers were declined as ledger-contention) and with the worker event loop blocked (progress frozen at "0/5 of 600"; sampling showed idle later because the walk had ended). Not a lease wait, not journal replay, not image attach.

## Fixes (landed 06:01 and 07:13, see landings.md)
1. Ledger re-measures at open only if dirty (crash), no stamp, or stamp older than 24 h (`Protocol.remeasureMilliseconds`, table `semio_graph_optional_measure`). Publish/retire stay incremental.
2. Writer opens the lease image once for the whole writer (`withRepositoryCommandCustodyV1`) instead of ~5 times.
3. Step label (en/de) now: "opening the graph store and its quota ledger and taking the graph state lease" / "Öffne den Graphspeicher samt Kontingentbuch und nehme die Graphstatus-Leihe".
4. Tests: ledger 7/0 (reopen with fresh stamp scans 0 dirs; stale stamp heals external deletion; crash test commits intent like the real protocol; wrapper race fixed, it failed identically at baseline), storage main 22/0 (fixture hashes of OptionalQuota (stale since H38) and OptionalLedger fixed), read-steps 9/0.
- storage `🧪️tests/🟦️.ts` "6 failures Original parent Script process invocation required": only under plain bun; under `repo test-file` the real failure was the stale OptionalQuota hash (fixed).

## Measured (same command, graph-slice phase)
- Before: graph-state 90 s (run 05:15), 134 s (semio tool 05:28); H38 saw ~170 s. Slice answered ~188 s.
- After fix 1 (first run after landing paid the one-time measure, 55 s): 37-43 s.
- After fix 2 (07:28, load ~45): graph-state ~25 s (21 s -> 46 s), inventory starts at 46 s.
Remaining: ~6 s worker boot, ~25 s open of images (per-open 1.7-3.8 s measured in tests: hdiutil roster), inventory scan ~40 s, capacity wait. Not seconds yet; next lever: skip `maintain(false)` open/close of the graph image and the 5 s inline eviction for readers (unmeasured, not done).

## Coordinator note (H50: `repo run repo:test-<suite>` 43 min in graph read)
Not profiled live. `buildInvocationGraphSelectionV1` already gives `run` a `tasks` selection, but without `--exclude-dependencies` the selection carries the whole dependency closure of the test target, which for repo suites is most of the workspace, so it is effectively a whole read and queues behind the whole read's custody (overlay only helps stale owners of small closures). Use `repo run --exclude-dependencies` or `repo test-file` for test lanes. Making closure reads share the sealed generation is open.

## Unverified
Debug timing of the writer stages never ran live (old service generation pid 88214 held by another agent's read blocked the restart for 25+ min); fix 2 effect is inferred from the 25 s vs 37-43 s runs, not from stage timings.
