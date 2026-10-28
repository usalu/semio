# result-warmfs (lane warmfs, wave 1027b, 10-28 01:0x): warmderive + futurestamp made landable on live after G50

C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`, ROOT = `/Users/ueli/Documents/semio`. No live file touched. Mirror `wf` (`/private/tmp/m/wf`, one create at 01:00, clone of live with G50) is dropped. Backups of the staged dirs before my edits: `C/🗑️generated/warmfs/{warmderive.orig,warmderive.rb1,futurestamp.orig}`.

FOR coordinator:
- state (checked 01:02 on live): `warmderive` 10 rows `base`, `futurestamp` 5 rows `base`.
- apply (any order; they share no file; warmderive is the one-line-edit batch, futurestamp the disk batch):
  - `python3 -I C/audit11-tools/staged/warmderive/stage.py apply --root /Users/ueli/Documents/semio`
  - `python3 -I C/diskproof-tools/staged/futurestamp/stage.py apply --root /Users/ueli/Documents/semio`
  - do NOT apply `C/edit30-tools/staged/warmderive`.
- verify: `sh C/audit11-tools/staged/warmderive/verify.sh /Users/ueli/Documents/semio` (warm laws 7/0, tsc 0 new, policy literal law 63/0, suites 70 pass / 2 known fail -> `VERIFY OK`); `sh C/diskproof-tools/staged/futurestamp/verify.sh /Users/ueli/Documents/semio` (after: 89 pass 0 fail -> `VERIFY OK futurestamp`; `... before` = RED AS EXPECTED, 2 future-dated tests).
- revert: `python3 -I <batch>/stage.py revert --root /Users/ueli/Documents/semio` (proven on the mirror: both back to all `base`).
- product proof for warmderive (unchanged from audit11): course edit probes `edit-probe.py wdA …` / `wdB …` (2nd edit without a `build:graph-derive` child); `ps -A | grep '♨️warm/⌨️main'` at most one deriver.

## 1. warmderive (audit11 copy, rebuilt on live after schemafirst memo)

- `python3 -I C/audit11-tools/staged/warmderive/rebuild.py /private/tmp/m/wf` on the fresh mirror: lane anchors and audit11 anchors all held; 10 rows, all `base` on live (3 had drifted: script-inputs `🟨️.mjs`, memo `🟦️.ts`, memo schema `🔣️.json`).
- One real defect found on the mirror, fixed in `rebuild.py` (EDITS for `04.json`): schemafirst `memo` made the memo schema carry a `PolicyDescriptions` table (EN+DE per policy key) and the law `🏗️build/🧪️tests/🧬️policy-literals` demands every `Policy` key there. The new `warm` policy key had none, so the law went 63/0 -> 62/1 (`Policy en ... warm`). Now `warm` has an EN and a DE description. `rebuild.py` is idempotent and reproduced `00.mjs`, `03.ts`, `05.ts` byte-identically from base (a re-run on a base-reset mirror).
- `verify.sh` now also runs the policy literal law (step 3), so this cannot regress unnoticed: law 63 pass / 0 fail with warmderive and futurestamp applied.
- Mirror results: warm laws 7/0, tsc outside the pre-existing error 0, suites `pass 70 fail 2` (= `known-failures.txt`), `VERIFY OK`.

## 2. futurestamp

- `cache-gc/🏃️runner/🟦️.ts`: already 3-way rebased, `base` (kept).
- `cache-gc/👯️dedupe/🟦️.ts`: the 3-way merge conflicted only on the adjacent import lines (schemafirst added `import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts"` right above futurestamp's `cacheRecencyMsV1` import change). Resolved by hand from the LIVE file: keep all schemafirst changes (`CACHE_GC_LIMITS_V1.dedupeProgressMilliseconds`, `dedupeCountProgressStride`) and apply futurestamp's two edits (import `cacheRecencyMsV1`, `recencyMs:cacheRecencyMsV1(info,ports.now())`). New base = live bytes, row `base`. Hand-merge copies: `files/f003.pre-rebase`, `manifest.json.pre-merge` (lane) and `C/🗑️generated/warmfs/futurestamp.orig`.
- Law RED -> GREEN on the mirror: before 87 pass / 2 fail (`a tree whose files are dated ten years ahead ...`, `a retired root holding future-dated files ...`), after 89 pass / 0 fail, `VERIFY OK futurestamp`. Policy literal law 63/0.

## Checks
- Parse check (`bun C/gate-g-tools/scan.ts`): 12 TS/MJS files of both batches, `scanned 12 bad 0`; JSON rows parse.
- `python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>`: OK for both.
- Logs: `C/🗑️generated/warmfs/[DEBUG] *.log` (temporary, delete when done).
