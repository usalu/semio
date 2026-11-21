# Generator Native Trace Handoff

Procedural preview jobs emit trace pages as part of the original ToolRun tick. The previous mounted receive path refused any decoded tick with trace pages at phase 3. ToolRun owns the trace store on the selected entry. Its dashboard delta, scene and recent-attempt panel all read that store.

The mounted domain callback can take several turns to ingest and fold original operations. When it reports its final operation count, native receiving must retain that count before advancing trace work. This keeps later Pending turns from ingesting the same provisional operations again. The trace receiver borrows the exact decoded page slice until it reports completion. That permits presentation only after the mounted trace owner has consumed the pages. The direct native convenience receiver still requires a trace owner for trace-bearing ticks.

The previous final native cursor release reported success before the mounted producer outcome could always fund its acknowledgement. A denied acknowledgement could replay the same descriptor from phase 0. Native receiving now holds completion until the funded acknowledgement sets the mounted outcome flag and resets native receiving together.

The full native witness exercises one domain completion, unchanged decoded page identity across Pending, exactly one upsert into the real resident store, controlled close of a partially reserved store after cancellation, and denied acknowledgement followed by funded retry. The original-presentation fuel and Source tests and both older full-wire variants also run. All six passed on the pinned native test in 0.27 seconds. The funded store suite shares a language-neutral fixture with an independent SQLite oracle for replay order, evictions, and survivors. The native store laws also check underfunded no-op behavior and two published window cursors.

One original trace apply step also accounts for any bounded recent-attempt bookkeeping. The mounted entry reserves 32 rows when constructed. The same update routine preserves the authored high-water key, recent order, clear and retire semantics of the ledger's direct trace application.

| Production source | SHA-256 |
| --- | --- |
| Native tick receiver | `4a993dc2d42c1da40562174325aac549ba6f56dc7627ecb2327ea52be330cb4e` |
| Mounted tool-run turns | `c0a9ffd24a1684fb7d47927cf0db1f84b459c10be35dc3e550623b6fc0a53a7a` |
| Ledger entry and panel helper | `9fc772d49df75ff0368e7731c48b9abb6e4cd71fff7a7681b30f085fdc434816` |

The earlier corrected-presentation build `w2wlSTL` advanced to 1,193 compiled units, then refused mixed Pack JSON and PDF-family-8 revisions before linking. Its candidate produced no transpiled browser probe input.

The actual mounted `semio-plugin-tool-run-turns` package passed a pinned, offline, locked `wasm32-wasip2` Cargo metadata check in 6m18s. The generated log is `generator-mounted-trace-check.log`. After the check, its private Cargo products and the superseded draft executable were reclaimed, freeing 1,206,984 KiB. The retained linked receiver test then passed all six cases again in 0.16 seconds.
