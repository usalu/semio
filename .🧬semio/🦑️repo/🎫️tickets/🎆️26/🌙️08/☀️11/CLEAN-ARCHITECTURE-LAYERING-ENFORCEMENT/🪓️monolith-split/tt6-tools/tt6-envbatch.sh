#!/bin/zsh
# usage: tt6-envbatch.sh <phase> <list-file> ; runs every listed test file inside the repository process envelope (tt6_envrun.ts), results in 🗑️generated/tt6-env-<phase>/<n>.txt
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P="$1"; LIST="$2"; OUT="$T/🗑️generated/tt6-env-$P"; mkdir -p "$OUT"; cd "$R" || exit 1; n=0
while IFS= read -r f; do
  n=$((n+1)); [ -z "$f" ] && continue
  case "$f" in /*) rel="${f#$R/}";; *) rel="$f";; esac
  res=$(SEMIO_NX_FIXED_STORAGE="$R/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m" NX_CACHE_DIRECTORY="$R/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts" SEMIO_TEST_ARTIFACT_DIR=$R/.cache/tt6-art perl -e 'alarm 290; exec @ARGV' -- nice -n 10 bun "$T/tt6-tools/tt6_envrun.ts" "./$rel" 2>&1)
  { echo "FILE $rel"; echo "$res" | /usr/bin/grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]//' | sort; echo "$res" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)|^error:' | head -5; } > "$OUT/$(printf %03d $n).txt"
done < "$LIST"
echo DONE > "$OUT/DONE"
