#!/bin/zsh
# usage: run it inside an empty scratch dir OUTSIDE the worktree: it writes nodes.jsonl (350 MB), changed14.tsv, blast pickles. Needs the persisted generation of .cache/b/g/m (read only) and `brotli`.
HERE=${0:A:h}; cp $HERE/*.py $HERE/*.sh . 2>/dev/null
python3 dump.py && ./changed14.sh > changed14.tsv
python3 reach.py && python3 reach3.py
for r in 0 1; do for a in 0 1; do [[ $r = 0 && $a = 1 ]] && continue; env REFRESHED=$r AFTER=$a python3 blast.py | sed -n 2,4p; done; done
python3 classes.py blast-stored.pkl blast-refreshed.pkl blast-refreshed-after.pkl
