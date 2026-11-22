# Result E43 (tests-caching-3)

All runs with `repo test-file --json`, 4-5 files in parallel, load 27-51 (the runner waits while load > 40), 420 s cap per file (the plain budget is 300 s). Table builder: `T/🧰️scripts/E43/table.py`, runner `run-list.py`, lock helper `lock.sh` (taken in alphabetical order, released after the edit), error-head helper `head.sh`.

## What was measured
- Population: every `.ts/.mjs` file below `C` (⚡️caching) inside a `🧪️tests` folder that calls a test registrar: 524 files. One of them is a declared helper that `test-file` refuses (`🏭️producer`, exit 2) and is excluded from the counts.
- Before: one full pass (11:20-12:15), taken partly while I edited, so a few files are already counted at their fixed state. After: the red files were re-run once (pass 2, 69 of 122 reached before its budget ended) and the files I changed were re-run after the last edit; files red before and not re-run are carried over unchanged (they are marked "not re-run" below). No full second pass was possible in the time (a full pass is about 70 minutes at this load).

