# R155: frozen slice lab (Part 1 landed, Part 2 not started) (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R155/` (`window.sh`, `land.py` restore tested exact on a scratch mirror, `stage2.py`, `extra.py`, `files.json`). Entries: `landings.md` "R155 landing 1" to "landing 7" (22:22 to 22:35 UTC, each hold about 1 s, all before the 22:40 rule that limits tooling landings to hh:00 to hh:05; nothing landed since).

## Part 1: the frozen view (landed)

Evaluation of the options:
- APFS snapshot as read-only root: `tmutil localsnapshot` works without sudo, but `mount_apfs -s` fails (`Operation not permitted`). Rejected. The test snapshot was deleted again.
- Pinned observation seam: the build and the service import and read files directly, a digest pin would have to sit under every read. Rejected as far too wide.
- Clone of the tree below the lab's work directory (chosen): the set `git ls-files` hands the build (167696 files, 1839 MB logical; `.🧬semio/🦑️repo/🎫️tickets` excluded, 113556 files), cloned with `copyfile` clone flags (no bytes copied on APFS), its own git index and a HEAD, so `ls-files` agrees with the live tree. Only read-only git commands touch the live tree. Measured: 339 s at load 60 (150 s system time: the per-file cost is the metadata, `cp -cR` of the finished root is as slow), so the lab can keep a root: `--frozen=<dir>` builds it once (sidecar `<dir>.frozen.json`) and reuses it; plain `--frozen` freezes into the run's work directory and removes it afterwards.
- Frozen root details found by running (each fixed with a law): `node_modules` is a real directory whose entries link to the live dependency generation, minus links into the tree itself (the launcher only accepts a root `node_modules` link bound to the root it was installed for); 345 ignored `node_modules`/`dist` links recreated; `.cache` stays a real directory with only `.cache/b/d/m/j` linked (the lease opener refuses a linked `.cache`).
- `--edit=<file>` appends one line feed to a file of the frozen root when the last machine starts (needs `--frozen`, 2 machines, stagger other than 0) and restores it; the edited file is excluded from the churn count (step 4 of Part 2).
- Churn counter: in a frozen run it counts files of the whole frozen root changed after the freeze, minus what git ignores; the verdict fails unless it reads 0. The live drift (files of the live tree changed since the run start, invisible to the machines) is reported separately. Measured on the real root in runs 1 to 3: churn 0, drift 3 to 10.
- Law: `slice-lab/🧪️tests/🧊️frozen` (8 tests, red before because the module did not exist; independent check with `diff -r` and `git ls-files` of the live fixture); the 17 existing lab tests still pass.

## Part 2: not done (honest)

No step of the table was reached. The frozen lab now runs the five stages up to `workspace` on one machine, then fails the same way on the frozen and on the live root: `build:implementation-source` ends with `Internal error: Unable to connect. Is the computer able to access the url?` (host log in scratch `R155/run1`, `run1b`). It appeared after another agent landed a private-HOME seeding of the lab (`slice-lab/🏠️home`, `home.json`: seeds launcher, repo, runtimes by clonefile; `downloads` log empty), so the likely cause is that change or the service's source-serving step, not the frozen view; I did not separate it. Until that is fixed no lab reaches `ready`, so walls 33 to 35 and 40 are still as in R143.

| step | seconds | present / fetched / inferred | tasks cache / run | verdict |
| --- | --- | --- | --- | --- |
| 1 A alone (frozen, `--execute`) | not reached | | | wall: implementation-source "Unable to connect" after 25 to 28 s of `workspace` |
| 2 B after A | not run | | | |
| 3 A and B at once | not run | | | |
| 4 B with one edited file | not run (`--edit` exists) | | | |

## Next

1. Find the "Unable to connect" cause (`source.served` phase, `🏗️build/🏛️sealed/📥️execution`); compare a run with the home seeding switched off.
2. Walls 34 and 35, step 2 (52 records stale for B: compare `neutralRecordV1` refusals and replayed rows between two frozen machines; with one shared frozen path the root cannot be the cause), step 3 (claim before look-up; the re-inference of a stale fetched record publishes without claim).
3. Land further lab or library edits only in the hh:00 to hh:05 window (stage 2 files are in scratch `R155/stage`).

## State left

`/tmp/slw-r155probe/r` (+ `r.frozen.json`): a reusable frozen root, 1.8 GB logical, clone, outside the tree (rerun: `--frozen=/tmp/slw-r155probe/r`). Delete it when done; it holds symlinks into the live `.cache`, remove with `rm -rf` only after `realpath` check (rm does not follow links).
