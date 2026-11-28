# H28: m04 107 GB under tickets/26/11

## Root cause (evidence: S/origins/g1-m04/partitions.json 01:56, depth 7)
- `.🧬semio/…/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/🗑️generated` = 100.3 GB, 189,294 files; `☀️18/DEV-PUZZLE-5D-…/🗑️generated` 2.9 GB. Nothing else.
- Not a replica bug: manifest digest (e070e68a5e5b, 522,310 files) is identical on m01/m02/m04/m07; incoming queue 0. `🗑️generated` is ignored by the root .gitignore (lines 13/20/21), so it is never replicated and its deletion is never propagated. The Macs deleted their copy at ticket end; m04 is also a dev workstation (/home/deck, agent sessions of 11-25/26, see the ticket's disk-2026-11-25.md) whose lane output stayed behind. Earlier the same pattern held 137 GB (REMODEL) and 57 GB.
- The census 107 -> 116 GB growth was the partial-walk budget, not writes (free bytes flat at 16 GB).

## Fix
1. `🚦️ready/🧹️ticket-generated` (schema-first policy, bilingual texts): every worker's ready sidecar removes ticket `🗑️generated` folders (also the FE0F-less name) in which no entry changed for 24 h, 2 h while free < max(32 GiB, 3 %); judged by newest mtime, early exit when active, links never followed, entry budget, cancel, first run 2 min after start then every 10 min. m04 (16 GB of 475 free) therefore sweeps with the 2 h limit as soon as it runs the new code; no manual deletion.
2. `🔁️replica/🛑️apply-guard`: store files are not written when free after the write < 2 GiB + 3 % of the volume (statfs, 2 s cache, unknown volume never blocks); deletions, own code and git stage unaffected; event `blocked`/`disk-short` bilingual; retried every turn.

## Tests
ticket-generated 5/0 (Python oracle walk, root .gitignore proof via replica matcher), apply-guard unit 4/0 (df oracle), replica integration 1/0 (short volume: delete applied, file held, arrives after room). Typecheck 6 files 0 errors.

## Unverified
- m04 has not run the new code yet; the folder goes only if it is idle for 2 h (a lane still writing there keeps it; then that lane is the writer). Check partitions.json after the next ready restart.
- Mac coordinator now also sweeps 24 h-idle generated folders (warm caches idle > 24 h are lost).
- Mixed `🎆26` vs `🎆️26` ticket dirs on Mac (FE0F-less) noticed, out of scope.
