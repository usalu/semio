# H4 result: remote execution offload

## Finding 1: the fleet-status offload numbers are cumulative since 11-14, not live
`offload.reasons` is the byte size of one file per reason in `.cache/f/offload-counters/reason/` of the coordinator checkout (append-only, never reset). The file mtimes show when each class last occurred:
| reason | count | last bump |
|---|---|---|
| unmapped-path | 3273 | 11-26 12:36 (log: last detailed 11-24 19:46) |
| infrastructure | 1658 | 11-26 05:18 |
| pin-failed | 676 | 11-24 14:38 |
| unmeasured | 1076 | 11-26 04:57 |
| worker-policy | 235 | 11-25 17:17 |
| incomplete-inputs | 165 | 11-25 18:07 |
Per-day from `.cache/b/r/m/u/x/offload.jsonl` (7331 events): unmapped-path 601/0/535/1214/313/2/10/0 (11-14..11-26 days with data), infrastructure 791+736 on 11-20/21 then 22, 5, 18, 28, 14; offload-run 1013 on 11-22, 447, 240, 221, 4. So unmapped-path (remap-path-prefix, source outside CARGO_MANIFEST_DIR, NATIVE_PLAY_OUT env) and infrastructure ("cache endpoint unreachable", "Content store unreachable", memo) were fixed by earlier waves; they dominate the totals only historically. Since 11-25 the log shows only: not-portable build-script spawns-process 63, CARGO_MANIFEST_LINKS/DEP_* env 6, host-probing 3 (policy, correct) and Content store 507 (insufficient storage, 13; cache-capacity lane H2/H3).
Live scheduler (`repo remote-execution status`, started 11-26 22:41): completed 104, failed 1, cached 0, retried 2, refused 0, queue wait p50 0 ms / p90 1 ms; 15 workers registered, Windows m05 correctly takes no macOS unit.
Monitoring consequence (H5 lane): show windowed rates (last hour/day) next to cumulative counters, else a fixed problem keeps looking open.

## Finding 2 (fixed): local-slot ignored memory pressure (12.8k declines)
`offloadGateV1` returned local-slot as soon as a compile slot was free and never asked the host verdict, so a 16 GB Mac under memory/load pressure kept compiling worthy units itself. Now: a free slot keeps the unit only when the host verdict is not `wait`; under `wait` the unit goes through cheap (stays local, cheaper than a round trip), skips the unmeasured-wait shortcut, and is offered and queued as `host-wait`. The host verdict is memoised per machine (`sharedUnitAdmissionV1`), so one extra sample per memo window.
Files: admission/🟦️.ts, tests/🚪️gate/🐍️.py (independent model) and 🔣️.json (3 new vectors, 3 updated `asked`).
Proof: `repo test-file …/🚪️gate/🟦️.ts` 4 pass / 0 fail, including the exhaustive vector-versus-python model over all signal combinations.

## Not verified
- No live offload run after the change (load 60+, no live experiments, supervisors restart on landing); expected counters after restart: local-slot down, host-wait out up on pressured machines. Check `repo fleet status --json` .offload.reasons deltas.
- `repo typecheck` of the touched file had not returned at release (very slow under load); the edit is a typed local variable only.
- unmeasured (1076), pin-failed (676), worker-policy (CARGO_PKG_DESCRIPTION env, 235) are historical; not re-derived live. worker-policy CARGO_PKG_DESCRIPTION last seen 11-25 17:17, likely already fixed.
- Per-origin offloaded in/out now (fleet): in 2017, out 1467, widest takers m12 (961), m11 (501), m07 (310).
