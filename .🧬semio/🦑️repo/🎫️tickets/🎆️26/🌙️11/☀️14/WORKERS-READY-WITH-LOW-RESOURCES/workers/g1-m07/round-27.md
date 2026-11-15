# g1-m07 round 27 report (series named round-24*, tries 1-4, 08:03Z-09:14Z; plus round 21 tries 1-3 04:33Z-08:03Z)

Machine: Mac mini, 16 GiB, macOS 27.2.1, worktree /Users/ueli/Documents/semio. Ready: **no** (`.cache/f/native-bootstrap.done` absent).

## Disclosures (my own errors)
- Round 21 try 1 (06:18Z-06:57Z, native_rc=130) was stopped **wrongly by my monitor**: it read `graph:` lines only from the bootstrap log, but "waiting for the fleet" lines live in the service log. The try was healthy (server last progressed ~340 s earlier). The monitor now reads the service log; pressure is no longer a stop (round 23).
- Step 0 was **not rerun before round 21 tries 2 and 3** (my wake-ups were too slow to catch the driver's 3-minute gap). A watcher script now does it.
- Round 21 try 2 (07:02Z-07:26Z): native_rc=1, EPERM on .cache/b/.reset.json on route launchd (pid 21367) and direct (pid 22385), access-proof version=3 sample=ok. Lines: round-21-try-2.eperm.txt, last 40 lines round-21-try-2.last40.txt. Round 26 says this is fixed at its cause.
- Round 21 try 3 (07:29Z-08:03Z): native_rc=75, plan stage inference 9837 MB > 6553 MB, 3002 MB available.

## Step 0 (repo tools setup all)
- Attempt 1 (round 17 note, 03:39Z-03:54Z): killed with the old session, no exit code.
- Attempt 2 (04:33:44Z-06:17:58Z): exit_code=1, 6254 s. Last line: "The dependency store returned no complete retirement plan (malformed, 227675 bytes, first violation: /value/roles/names must contain at most 16 item(s)); nothing was retired."
- Attempt 3 (08:03:44Z-08:31:30Z, after round 25 said fixed): exit_code=1, 1666 s, **same error**. Log ~/tools-setup-round-16.attempt3.log. No per-scope restored/installed lines seen in it (not measured beyond the last 20 lines).
- Not rerun since; will rerun when a round says the 16 vs 18 roles fix has landed for this machine.

## Tries (round-24* files)
| try | ended (UTC) | native_rc | plan stage inference grew to | available | run peak resident | pressure/hard stop |
|---|---|---|---|---|---|---|
| 1 | 08:42:15 | 75 | 9932 MB | 3260 MB | 5.78 GB | none |
| 2 | 08:59:37 | 75 | 10003 MB | 2125 MB | 5.64 GB | none |
| 3 | 09:07:18 | 75 | 9882 MB | 2780 MB | 2.94 GB | none |
| 4 | 09:14:29 | 75 | 9859 MB | 2782 MB | 4.77 GB | none |
Every refusal: "more than the 6553 MB this machine gives a plan ... (at least 2048 MB are kept in reserve) after it gave back what it could". Round 20 says refusal above 9830 MB: all four are 29-173 MB above that.

## Plan line (first, verbatim, service log of try 4 window)
graph: 676 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave)
-> the 0 MB bug is fixed on this machine (3512 MB, 192 per wave, 5 too heavy).
- Fleet: "graph: 0 of 676 projects arrived from the fleet, 0 inferred here, 676 left" for the whole wait; the fleet delivered nothing in these tries.
- Plan stage lines seen: "plan stage inference starts (managed 1120 MB, resident 1790 MB)"; "project @semio-tech/value-rs (language:cargo) grew by 1873 MB, more than the 1756 MB this machine gives one project ... marked too heavy here"; "@semio-tech/value-sort-rs grew by 2817 MB (same)"; "plan stage inference (during inference) grew the plan by 5022 MB, more than the 4915 MB this machine allows for it"; "plan stage inference settled from 6583 to 5539 MB" (a window of the service log; per-try attribution not separated). Restore line (restored ... from a state A min old) not found by my grep: not measured.
- access-proof version=3 appears at every service start (route=launchd or direct); no EPERM seen in round-24* tries (checked final client lines: all "native_rc=75" refusals, none EPERM).

## Launcher
Unchanged: ~/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo (see round-8/round-21 launcher.txt). Access receipts after round 21 try 2: client ok, service ok, launcher.json absent.

## Ask
Plan stage inference lands ~9.9 GB on a 16 GiB machine with ~2-3 GB available; limit 9830 MB. Needs the inference to read less, or fleet delivery of the 676 projects (0 arrived).
Final client line every try: "The project graph could not be read on this machine yet: ... Nothing is wrong with your tree; run the command again when a machine with free memory ..." then native_rc=75 (full text in round-27.tail.txt for try 4).
