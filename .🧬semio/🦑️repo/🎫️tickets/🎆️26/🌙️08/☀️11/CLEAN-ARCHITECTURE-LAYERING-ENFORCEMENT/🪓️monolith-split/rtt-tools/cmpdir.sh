#!/bin/zsh
out="$1"; real="$2"
for d in "$out"/*/; do n=$(basename "$d"); f=$(ls "$d" | head -1); if ! diff -q "$d$f" "$real/$n/$f" >/dev/null 2>&1; then echo "DIFF $n"; diff "$d$f" "$real/$n/$f" | head -8 | cut -c1-220; fi; done
