# result-cbrebase (lane `cbrebase`, wave 1027b/g49) — clock, shard, witpub restaged on live

C = this ticket folder; R = `/Users/ueli/Documents/semio`. Nothing live was edited (mirror `cbrebase`, guard clean on every run, dropped at the end).

## FOR coordinator
- Apply order: `clock` (nothing to apply, already live) -> `shard` -> ... `schemafirst/bazelscript` -> `edit30/witpub`.
- `shard`: `python3 -I C/cbudget-tools/staged/shard/stage.py apply`; laws `VERIFY_ROOT=R sh C/cbudget-tools/staged/shard/verify.sh`; product proof `bun repo run @teaching/architecture-course:test`; revert `stage.py revert`.
- `witpub` (only after `schemafirst/bazelscript` is applied; before that `check` reads `drifted` on `📜️script.ts`, by design): `python3 -I C/edit30-tools/staged/witpub/stage.py apply`; laws `sh C/edit30-tools/staged/witpub/verify.sh R`; revert `stage.py revert`.

## State today (`stage.py check` on live)
| batch | rows | check | note |
|---|---|---|---|
| cbudget/clock | 9 | 9 applied | fully landed (not 7+2): the two "drifted" rows already contain the clock edit, they only differ by later live edits |
| cbudget/shard | 4 | 4 base | rebuild.py reproduces the manifest byte for byte; no path shared with clock |
| edit30/witpub | 7 | 6 base, 1 drifted | script.ts base = live + bazelscript new, so `drifted` until bazelscript lands |

## 1. clock
- Live vs staged at 23:50: `⏲️contention/🟦️.ts` and `🧪️tests/🟦️.ts` carry later edits (compliance/lang: `refuseProcessV1("invalid","testing.…")` instead of literal `Error`, one docstring emoji), the schema index and the build guide carry the clock edit plus unrelated later edits (`semio:build-test-envelope:v1` line, graph-farm guide paragraph). I verified that every line the clock added is present on live (index: the contention id; guide: the EN and DE bullet).
- Re-recorded without touching live: rows 00.ts and 03.ts: `files/`, `src/` and manifest `new` = live bytes (`C/cbrebase-tools/relive.py`, `*.pre-relive` copies). Rows 07.json, 08.md: 3-way merged (`rebase_entry.py`, `*.pre-rebase`), then `base` re-derived as live minus the clock's own added lines (`C/cbrebase-tools/unland.py`), so `revert` removes only the clock edit and `indexcheck.py` (added == exactly the contention id) still holds.
- Verify (mirror, `verify.sh`): stage check 9 applied, index ok, `bun test` 5 files 31 pass / 0 fail (2704 expects). Note `rebuild.py` of clock now refuses (`already edited`): the batch is landed; do not run it.
- Mirror caveat: the default mirror lacks `✨️derive/🗂️schema-document-index.json`; copy it from live into the mirror before `verify.sh` (reports `drifted`/missing otherwise).

## 2. shard
- Base rows equal live (no row shared with clock), `rebuild.py` re-run on live: manifest unchanged. Order after clock holds trivially.
- Verify (mirror, shard applied, needs the course dir in the mirror): stage check 4 applied, 10 pass / 1 skip / 0 fail (128 expects; the skip is the real-Bazel test, which needs `SEMIO_TEST_BAZEL_BINARY`, as before).

## 3. witpub
- `schemafirst/bazelscript` was restaged at 23:32 by sfrebuild (`check` = 3 base, its script.ts base = live). I built a mirror of live + bazelscript's staged files, ran `witpub/edit.py <mirror> apply` (anchors all matched, schema edit idempotent), copied witpub's other 5 whole-file rows into it (their bases still equal live) and re-cut with `edit30-tools/make-stage.py` (stage.py/docstring identical). Then row `00.ts` base := bazelscript's new script.ts (hash `6ab9ae04…`, new `cba943f0…`; was base `73e86e06`, new `4d1a340b`). Other rows unchanged (same base/new hashes). Old batch kept in `C/🗑️generated/cbrebase/witpub.before/`.
- Verify (mirror = live + bazelscript + witpub): stage check 7 applied; witness laws 14 pass / 0 fail (publisher digest = reference = Python hashlib, real Bazel); the 79-suite list: 409 pass, 36 distinct failing names vs `base.set` 25: 11 new names, but all 11 also fail with witpub reverted on the same mirror (today's live + bazelscript; pre-existing, not caused by witpub), no failing name appears only with witpub. Stage-mode law OK.
- Dependency: if bazelscript is rebuilt again (its script.ts new changes), re-run `edit.py` on a fresh mirror of live + bazelscript, copy the 5 whole rows, `make-stage.py`, then set row 00 base to that new bazelscript script.ts (script above in this section).

## Files
`C/cbudget-tools/staged/clock/{manifest.json,files,base,src}` (+ `.pre-rebase` / `.pre-relive`), `C/edit30-tools/staged/witpub/{manifest.json,files,base}`, helpers `C/cbrebase-tools/{relive.py,unland.py,wp-chunk.sh}`. Debug outputs `C/🗑️generated/cbrebase/[DEBUG]-*`.
