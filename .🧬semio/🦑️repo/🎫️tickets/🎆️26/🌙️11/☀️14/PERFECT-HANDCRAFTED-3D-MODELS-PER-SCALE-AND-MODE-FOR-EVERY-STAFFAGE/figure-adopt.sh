#!/bin/bash
# adoption of an authored human: figure-adopt.sh <slug> [scales...]  (restores nothing; run it on the source as it is; default scales 50 100 200 500)
source "$(dirname "$0")/author-33-env.sh"; cd "$(dirname "$0")"
S=${@:2}; S=${S:-50 100 200 500}
for sc in $S; do $BUN figure-build.ts humans/$1 --scale $sc --from-existing --write | grep -v "^  kept\|dropped"; done
./figure-fails.sh humans/$1 | cut -c1-330
