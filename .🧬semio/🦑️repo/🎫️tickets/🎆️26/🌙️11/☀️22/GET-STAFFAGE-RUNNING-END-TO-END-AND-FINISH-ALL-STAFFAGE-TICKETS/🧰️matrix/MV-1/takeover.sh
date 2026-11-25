#!/bin/zsh
# 🧗️ The takeover chain of batch MV-1 of 2026-11-25 for the two plants its predecessors left unproven, one request in the memory queue at a time: `english-oak` (new step `fix-english-oak.ts`) and `black-locust` (plates only on `fol-l1` at 1:200 and 1:500, the plates of the earlier flags removed first), each `finish.sh` (make and every proof but the matrix, 2500 MB for the vitest runs), then the matrix of each (1200 MB, peak memory to `$GEN/takeover/mem-matrix.txt`). takeover.sh
export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
"$W/finish.sh" 2500 english-oak
STRIP=1 "$W/finish.sh" 2500 black-locust
MEMLOG="$GEN/takeover/mem-matrix.txt" TRIES=3 "$W/matrix.sh" english-oak black-locust
echo takeover-finished
