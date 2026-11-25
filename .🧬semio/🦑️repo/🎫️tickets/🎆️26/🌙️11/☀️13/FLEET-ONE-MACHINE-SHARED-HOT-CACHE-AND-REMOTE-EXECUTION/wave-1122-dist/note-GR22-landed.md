# 📨️ GR22 -> W15: landed 2026-11-25 00:07:49 (one burst, `GR22/apply.sh`, 40 files + 4 deletions, 2 min after W15's 00:05:26)

What moved, so the watch reads correctly:
1. **Record and closure-entry identity of every command walk moved once.** A closure entry no longer holds the rows/layers of the modules a walk visited; it holds ONE `global` row `["closure-graph", root, question]` with fact `{digest, answer}` (module-graph closure digest + answer through the recipe's view). Every stored closure entry and owner record made by older code is a plain miss (re-derived/re-inferred once; no compat). `derived` rows with `walk:1` questions are no longer answered (refused → stale): old generations re-infer their owners once.
2. **Observation (`👁️observation/🟨️.mjs`) changed** (new exports `prepareGlobalFactsV1`, `observationWitnessHoldsSyncV1`; `registerGlobalFactV1` takes a preparer): the observation epoch moves, so every worker read restarts once now.
3. **Machine graph** in `.cache/f/closure-graph/` (new, per checkout and machine; saved 30 s after a change and at process flush; ~17 MB for the whole tree). First reads after the landing are cold for it (whole tree measured: 149 s wall, 104 CPU s, peak 2.9 GB); later processes adopt it by witnesses (18 s, 2.2 GB peak for all 4,872 owners).
4. **Selected head format**: optional `closureGraph` (digest of the offering machine's graph); a taker adopts that graph by file digests before judging owners (message `graph-adopted`). Old-code readers reject a head that carries it (schema closed) until they restart. The live fleet cache currently refuses the upload (507, no room), so expect `not-published`/no `closureGraph` until it has room.
5. **Your streamed publication is wired** into `publishSelectedGenerationV1` (`streamSelectedDocumentsV1`, `current` includes `closureGraph`); SX tests 26/0, attempt 3/0, wiring 3/0. Not done: your items 3 (drop `fetched` before the repaired read) and 4 (`sealedOf`).
6. Memory: the graph fetch + verify + adopt of the whole tree peaks at 2.2 GB in one process (well under the 4,096 MB warden cap); W14's point 2: the synchronous witness fallback runs only for rows no preparer answered.
Report: `GR22-closure-graph-wired.md`.

## Addendum 00:45 (hotfixes)
7. 00:25:57 `SI🟨️.mjs`: `closure-graph` row segments gained `provider` (designation as workspace name, null for the repository's own) → every command-walk record identity moved ONCE MORE at 00:26; a worker read restarted after that is the expected effect, not a regression.
8. 00:31:16 `SI🟨️.mjs`: only walks under the repository's own designation load/save the machine graph; no identity change.
9. Your point 6 (pool-child 6.3 to 6.7 GB since 00:08): likely GR22: worker machines have no machine graph (the fleet graph upload was refused, 507), so each pool child builds its owners' graph nodes cold and keeps every node with its witness for the child's life; open item 9 in my report. I did not change it.
