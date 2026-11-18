# Rounds 22 to 30 on g1-m02, Combined

Written 2026-11-18 from the runner notes (`round-NN.samples.txt`) and logs. Numbers are the runner's `TRY n END` lines; the per-try details stay in the `round-NN.*` files.

## Result

None of rounds 22 to 30 reached `native_rc=0`. Every try ended in a fleet refusal (`native_rc=75`), a stall stop (`native_rc=7` or `130`), or `native_rc=1`. Machine: Mac mini M4, 16 GB.

| Round | Tries | Ends (native_rc) | Highest resident of a try |
|---|---|---|---|
| 22 | 2 | 1, 7 (no progress for 907 s) | 6.4 GB |
| 23 | 3 (try 4 has only a sampler file, no end line) | 1, 75, 1 | 9.1 GB |
| 24 | 2 | 130 (dependency store wait 5 min), 75 | 8.7 GB |
| 25 | none run | runner content was round 26 code | not measured |
| 26 | 4 | 75, 75, 75, 75 | 8.6 GB |
| 27 | 6 | 75, 1, 75, 75, 75, 75 | 9.3 GB |
| 28 | 9 | 75 (tries 1 to 8), 1 (try 9) | 8.9 GB |
| 29 | none | content of the runner was round 30 code | not measured |
| 30 | 5 | 75, 130 (stall 904 s), 75, 130 (stall 901 s), 7 | 9.5 GB |

Rounds 22 and 24 peaks are from the end lines; 23, 27, 28 and 30 from the first end line of each try.

## What was common

- The refusals were the graph plan memory limit (a plan may take about 6553 MB on this Mac); one project's inference needed about 7.3 to 10.6 GB. Step-down in round 30 did not help.
- Rounds 25 and 29 have no tries of their own: the runner files were re-used (see `chain-note.txt`).
- Round 31 stopped my own setup runs; the fleet agent's `ready` child took over.

## How it ended

The `workspace` stage of the `ready` child failed 58 times (exit 75 or 1, or timeout) and then passed at attempt 58: 253 s, exit 0, outcome ok, peak 139 MB, "owners done 591 of 591", `readyMs` 1794993907570 (2026-11-18 about 09:25Z). I changed nothing; the coordinator's side must have changed what the stage needs, I did not find what. `repo preflight --lang en` then exited 0. See `round-31.md`.
