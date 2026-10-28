# Lane sfrebuild (wave 1027b, 10-27 23:5x): all 15 schemafirst batches rebuilt on the live tree after G47-G49

C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`, ROOT = `/Users/ueli/Documents/semio`. No live file touched; only `C/schemafirst-tools/` (batches/, staged/) changed. Mirror `sf1` (root `/private/tmp/m/sf1`, 2 creates: the first went stale when G49 part 2 landed) is dropped. Left in place: `/private/tmp/m/.nm` (shared node_modules clone; other lanes' mirrors now live under `/private/tmp/m` and `drop --nm` refused: cbrebase, isov, lang6, lb1, sp).

FOR coordinator:
- state (checked 23:3x on live): all 15 batches `base` on live, 91 rows (`for b in $(cat C/schemafirst-tools/order.txt); do python3 -I C/schemafirst-tools/staged/$b/stage.py check --root . | awk '{print $1}'; done | sort | uniq -c` -> `91 base`). If the gate lands something in between: `sh C/schemafirst-tools/rebuild_all.sh` (reads live, counted anchors, stops on a moved anchor).
- apply (law first, stops on drift): `for b in $(cat C/schemafirst-tools/order.txt); do python3 -I C/schemafirst-tools/staged/$b/stage.py apply; done`
- verify (per batch, law inside): `VERIFY_ROOT=ROOT sh C/schemafirst-tools/staged/<batch>/verify.sh` (stage check + scan + docheck + policy-literal law; final law 63 pass / 0 fail after storelimits).
- revert (reverse order): `sh C/schemafirst-tools/apply_all.sh ROOT revert` (equivalent: `for b in $(sed '1!G;h;$!d' C/schemafirst-tools/order.txt); do python3 -I C/schemafirst-tools/staged/$b/stage.py revert; done`).
- Land BEFORE audit11 warmderive: memo's schema JSON (`🧠️memo/🧬️schema/🔣️.json`) and `🧠️memo/🟦️.ts` are shared; rebuild warmderive after memo (audit14 section 2 items 8-10).
- Product proof as in result-schemafirst.md "Re-keying": course:build, course:test, print-rs:test, dashboard build, then a second hot course no-op (first run after landing plans the graph once, the second serves from the memo).

## Anchor fixes (the only two code changes of this lane)

1. `batches/observation.py`: the roster fixture `👁️observation/🧪️tests/🧫️fixtures/🔣️.json` `declarationExports` no longer starts with `adoptVerifiedBlockV1` (live added `adoptInferenceAuthorityV1`, `canonicalResolutionV1`, ... , 15 names total, sorted bytewise: upper-case first). The insert of `OBSERVATION_LIMITS_V1` now anchors on `"declarationExports": [\n      "adoptInferenceAuthorityV1"` (still sorts first; all 15 live additions kept).
2. `batches/inference.py` (new, found on the mirror): the test `🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` evaluates `targetScriptSources` of the script-inputs Source as text with hand-listed free identifiers. The batch makes `targetScriptSources` read `REVISION_POLICY_V1` (path ceiling) and `yieldInferenceV1` read `INFERENCE_LIMITS_V1`, so on the mirror the test "freshness inputs and explicit runtime entries preserve actual command import demand" went from pass to `ReferenceError: REVISION_POLICY_V1 is not defined`; (the `.rejects.toThrow()` rows of the same test would have passed for the wrong reason). Fix: the test imports both constants and the three hand `ports` objects (the three that read the script-inputs Source: command roles, generator promotion, import-resource closure) carry them. The batch now has 9 entries (was 8); the test is listed in its verify.sh.

All other 14 batches rebuilt unchanged from the specs: their anchors still held after G47-G49 (drift was only in the live bytes the counted edits sit in; the build re-reads those).

## No live change dropped (diff base -> new per entry)

`C/schemafirst-tools/cmpedit.py <C abs>` compares, per entry, the added/removed line multisets of the OLD staged batch (copy before this rebuild) with the NEW one. Result: identical for every entry of 14 batches (observation included: the fixture hunk is the same one line plus the live names as context); the only entries that differ are `bazelscript` script.ts (long lines: the old->new difference is exactly G47-G49's own edits to that file: `sayBuildV1`/`refuseBuildV1` texts, `createBazelActionWidthV1`, the width gate, emoji docstring swaps - checked token by token against the base->base diff) and `inference` (the added source-demand test entry). Edits are counted substitutions on the live text, not whole-file copies, so live changes ride along; the law-src files and new schemas are new files.

## Per batch (rows / new files; stage check `base` on live and on the fresh mirror; apply order; every verify.sh exit 0)

| batch | rows | new | scan (TS/MJS) | docheck | law after batch (cumulative pass) |
|---|---|---|---|---|---|
| law | 3 | 3 | bad 0 | 0 | 3 |
| revision | 4 | 1 | bad 0 | 0 | 6 |
| observation | 8 | 2 | bad 0 | 0 | 12 |
| generations | 13 | 3 | bad 0 | 0 | 21 |
| graphcore | 6 | 3 | bad 0 | 0 | 27 |
| memo | 6 | 1 | bad 0 | 0 | 30 |
| arguments | 2 | 1 | bad 0 | 0 | 33 |
| cachegc | 7 | 3 | bad 0 | 0 | 36 |
| hostcap | 7 | 1 | bad 0 | 0 | 39 |
| bazelsessions | 4 | 1 | bad 0 | 0 | 42 |
| bazelscript | 3 | 1 | bad 0 | 0 | 45 |
| readoracle | 6 | 1 | bad 0 | 0 | 48 |
| buildmisc | 10 | 6 | bad 0 | 0 | 57 |
| inference | 9 | 4 | bad 0 | 0 | 60 |
| storelimits | 3 | 1 | bad 0 | 0 | 63 |

Parse check of every staged entry: `bun C/gate-g-tools/scan.ts <list> ...` over the 43 TS/MJS entries (`.d.mts` skipped by the tool, 6 files): `scanned 43 bad 0`; 40 JSON entries all valid (`json.load`), 3 Python entries `ast.parse` ok. `python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>`: OK x15. Revert on the mirror returns all 15 batches to `base`.

## Law and tests on the mirror (fresh clone of live at 23:3x, envelope runner `C/red-g-tools/mrun.sh`)

- Law `🏗️build/🧪️tests/🧬️policy-literals/🟦️.ts`: red-free: 3/3 after law, 63 pass / 0 fail after all 15 (each batch's verify.sh ran it; includes the independent Python lexer oracle and Ajv strict).
- Module tests: 84 files (every module test dir named by the 15 verify.sh files + `testsall.txt` + `testsnew.txt`), run on the mirror BEFORE apply and AFTER applying all 15 (`cmp.py`): failure sets equal to the base. The pre-existing red files are unchanged (opening-store required 4 fail, generation-protocol 14, source-demand 3, candidate 3, bazel implementation 4, host-capacity/lease-continuation, inventory DATA 7 ...). Two noise rows seen and explained: (1) `generations/🧪️tests` "a shared file edited every few hundred milliseconds ... within 1.2 times its quiet time" failed once under load 13-20 in the first after-run, 27/0 on two reruns (and in the second full after-run); (2) bazel `implementation` test printed no summary line once in the base run (the parser then attributes its 3 named failures to the previous file); on the reverted mirror it is `1 pass 4 fail` with the same 3 names + (unnamed), identical to after.
- Fixed by this lane: source-demand regression above (base 27/3, after the fix 26/3 -> same 3 names; before the fix 26/4).

## Tools

`C/schemafirst-tools/`: `rebuild_all.sh`, `apply_all.sh`, `batches/*.py`, `lib.py`, `docheck.py`, `cmp.py`, `runlist.sh`, `cmpedit.py` (new, entry-level preservation check; expects the old staged copy in `C/🗑️generated/sfrebuild/staged-before`). Scratch (logs, test outputs, old staged copy) was removed from `🗑️generated/sfrebuild`.
