# L17c Puzzle 2D runtime tests

Status: board-host green (36 passed); remaining 30 steps paused, /home below 8G free (4.5G). No source changes.

## Findings
- After the disk was freed the runner picked up the board-host step and is rebuilding the closure (862 crates compiled so far, warnings only, no errors).
- /home free space fell from 15G to 8.2G during the build (shared with a 3D activation). Per the coordinator rule no further steps were queued; remaining lines of `l12-2d-steps.txt` (30) wait until /home is above 8G with the first step finished.

## Per-package table
| package | result |
|---|---|
| semio-s-artifact-puzzle-2d-board-host | ok, 36 passed, 0 failed |
| other 30 packages in l12-2d-steps.txt | not queued |
