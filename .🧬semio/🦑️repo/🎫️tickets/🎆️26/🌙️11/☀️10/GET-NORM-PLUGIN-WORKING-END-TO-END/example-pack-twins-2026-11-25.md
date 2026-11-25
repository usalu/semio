# Example pack twins (2026-11-25)

Status: tests written and mounted in 14 family examples crates; NOT yet run green, and no stale pack regenerated yet, because the only attempt that held the lane (din16798) died before compiling any norm crate.

## Blocker
`🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs` (uncommitted edit by another agent, 09:18) mounts `#[path=".../🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs"] mod dbg;` and calls `dbg::add` / `dbg::on`. The target file no longer exists, so every cargo build ends with `module dbg has no single matching file` (log `🗑️generated/examples-din16798.log`). Not touched by me. A chain (`run-pack-twin-families.sh`, log `🗑️generated/twins-chain-2.log`) is queued on the lane for din16798, din4108, en1992, en1998, en1991 and will produce real results once that line is gone.

## Consumers of packs
No production consumer: examples reach the hub component and Play only through `ExampleSource::new(ID, label, PRIMARY_TEXT, ICON)` with `include_str!` of the DSL. `include_bytes!` of a pack exists only in the en1999 example test. Packs are read by tests and by the bless helper `⚖️compliance/🧪️tests/🧬️regen` (`bless_dsl_and_pack_assets`, pack beside the DSL first, then in the example directory above it), the editor `📚️assets` test of en1990..en1999 (not din*, iso16757, vdi3805) and din4108/en1996/en1999 io tests.

## Static findings (content check still needs the decoder)
- en1991: both examples carry two packs (outer in the example directory, fresh 11-23 07:24; inner beside the DSL, 11-20, older). The editor test `no example carries two packs` and the bless helper (inner wins) disagree with the outer ones; the inner 11-20 ones are the stale duplicates. Decision after the run: delete the inner duplicates if the outer equal the DSL.
- din18599 and vdi3805 have no packs at all; their test checks the DSLs decode and the (absent) twins.
- All other listed packs: staleness decided by the new test, then regenerated with `SEMIO_UPDATE_GOLDEN=1` through the family encoders.

## Added
`🧪️tests/🗜️pack-twins/🦀️.rs` in each `📚️examples` (en1999 already has its own): DSL decodes; each pack decodes to the DSL document and re-encodes to identical bytes; each JSON twin decodes to the DSL document; one pack and one JSON per example. Mounted as `mod pack_twins` in each `📚️examples/🦀️.rs`; io dev-dependency added to din18599, en1992, iso16757, en1996. Generator: `generate-pack-twin-tests.py`.

## Process note
I stopped my first queued chain with `pkill -f` on a pattern (`run-pack-twin-families.sh`, `lane.py .*test-family-examples.sh`). Only my own lane waits matched it (no other agent combines lane.py with that script), but from now on I stop only pids I started. No en1991 pack is deleted before the new test has decoded the outer pack and shown it equals its DSL.

## Results so far (13:00 to 14:40)
- din16798: first run 3 passed, 1 failed (`noncompliant-office: pack does not decode: missing field 'unoccupied-ventilation'`; the old test stopped at the first failure). Test now reports all findings. Bless run (`bless-pack-twins.sh`, SEMIO_UPDATE_GOLDEN=1): 4 passed, 0 failed; both `compliant-office` and `noncompliant-office` packs were rewritten (587 to 590 and 596 to 599 bytes), DSLs untouched. Proof run without the variable: OPEN (the bless run writes then compares, so it is not proof alone).
- din4108: first run 4 passed, 1 failed (`demo: pack does not decode: missing required field 'cases'`; `failing-thin-insulation` unseen behind the first panic). Bless run was stopped by the coordinator (pid 81525) before it wrote anything: both packs still 1050 and 986 bytes, mtime 11-15, git clean, DSLs untouched. Regeneration still to do: `SEMIO_UPDATE_GOLDEN=1 python3 lane.py test-family-examples.sh 🧱️din4108`, then a run without the variable.
- en1992: compile error `E0716` in `💥️failing-prestressed-beam/🧪️tests/🧩️example/🦀️.rs` (temporary dropped); fixed by binding `report` first (no assertion changed). Rerun pending.
- en1998: build stops in unrelated plugin crates (`semio_plugin_store_lifecycle`, `original_composed_lifecycle_step` missing); no test line.
- en1990: sandbox `permission denied` on the other agent's debug-ledger.rs mount; no test line.
- en1991: log is the old 11-23 one; not rerun yet. No pack deleted.
- Still unrun: the nine of chain 3 and en1991, en1992, en1998 reruns. No new builds started on the coordinator's instruction.
