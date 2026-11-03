# W3-HEAVY-OPS

Status: QUEUED, NOT YET RUN. Nothing below has produced a result yet. The serial heavy-slot queue held about 22 waiting jobs (twins generates of other artifacts, evolve applies, sweeps); my first job had not reached the slot after roughly 40 minutes.

## Queued (two detached scripts, each step through heavy.sh, one after the other)
`🗑️generated/heavy-ops/run.sh` (started first):
1. `repo ./taxonomy_generate.ts` in `🧰️scripts/kind-convention-v4` (serves en1994 and the en1991 stale rows, coordinator request). Before-copy of the fixture: `🗑️generated/heavy-ops/taxonomy-before.json`; the script copies the result to `taxonomy-after.json`. Compare for drift beyond `remove-beam.edit.owns`, `insert-stud.item`, en1991 rows.
2. `schema twins generate --under .../🧩️en1994`, 3. same with `check --strict`.
4. `evolve.py contract en1994 --dry-run` (contract has no nested heavy.sh call, so no deadlock risk). `--apply` is NOT queued: read `4-contract-dry.log` first, then run `--apply` and update the sqlite table counts in the Rust/TS tests.

`🗑️generated/heavy-ops/run2.sh` (waits for `taxonomy-after.json`): `evolve.py repair en1991 --apply` (no nested heavy.sh), then `verify_fixtures.py en1991` and `en1994`.

Logs: `🗑️generated/heavy-ops/{1-taxonomy,2-twins-gen,3-twins-check,4-contract-dry,5-repair-en1991,6-verify-*}.log`; `done` / `done2` markers when finished.

## Not started
Other `[open]` requests: only the en1994 item and the law-sweep-2 "twins check --strict" line are heavy runs; no owner log showed them as run. The strict twins check for the whole repo was not queued (en1994 only).

## Note
These scripts are left running in the background deliberately (the waits exceed what a foreground call can hold). Do not delete `🗑️generated/heavy-ops` until `done2` exists.

## Update (taxonomy generator ran and FAILED, root cause fixed)
- `repo ./taxonomy_generate.ts` exited 1: `mutation leaf metadata is unreadable` for en1991 `🧬️mutations/🏗️change-fatigue-model-lane/🦀️.rs`. Its `SemanticDescriptor` used the field `lane:` instead of `kind:` (framework struct has `verb, entity, kind, record`; this would also have been a rustc error). Fixed that one line (`lane:` -> `kind:`). No fixture was written by the failed run; `taxonomy-after.json` is an unchanged copy (other agents added 37 insert/remove/change rows meanwhile, not generator output).
- Re-queued only the generator as `🗑️generated/heavy-ops/run3.sh` (log `1b-taxonomy.log`, compare `taxonomy-before2.json` vs `taxonomy-after2.json`, marker `done3`).
- `run2.sh` (repair en1991, verify_fixtures en1991/en1994) started its wait when the failed run copied `taxonomy-after.json`, so its steps run against the stale taxonomy: after `done3` shows exit 0 with the expected drift only (en1994 `remove-beam.edit.owns`, `insert-stud.item`, en1991 rows), re-run `verify_fixtures.py en1991` and `en1994`.
- Still pending in the heavy-slot queue: twins generate/check en1994, contract dry-run, repair en1991. `contract --apply` and request marking in requests.md remain open.
