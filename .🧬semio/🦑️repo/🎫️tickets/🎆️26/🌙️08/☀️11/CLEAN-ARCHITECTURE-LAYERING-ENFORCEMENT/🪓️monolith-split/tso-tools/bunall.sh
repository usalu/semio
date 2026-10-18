#!/bin/zsh
# usage: bunall.sh <label> <filelist> [start-index] -> G/tso-bunall-<label>.tsv (exit, pass, fail, file) + per-file logs in G/tso-bunlogs-<label>/
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tso-tools/env.sh
LABEL="$1"; LIST="$2"; START="${3:-1}"
OUT="$G/tso-bunall-$LABEL.tsv"; LOGS="$G/tso-bunlogs-$LABEL"
mkdir -p "$LOGS"; [[ "$START" == 1 ]] && : > "$OUT"
cd "$R"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
i=0
while IFS= read -r f; do
  i=$((i+1)); [[ $i -lt $START ]] && continue; log="$LOGS/$(printf %03d $i).log"
  perl -e 'use POSIX; POSIX::setsid(); alarm ($ENV{TSO_BUN_ALARM}||180); exec @ARGV' -- nice -n 10 bun test --timeout 120000 "$R/$f" > "$log" 2>&1
  ec=$?
  p=$(/usr/bin/grep -E '^ *[0-9]+ pass' "$log" | tail -1 | /usr/bin/awk '{print $1}')
  fl=$(/usr/bin/grep -E '^ *[0-9]+ fail' "$log" | tail -1 | /usr/bin/awk '{print $1}')
  seg=""; /usr/bin/grep -q -i 'segmentation fault\|panic' "$log" && seg="SEGV"
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$ec" "${p:-?}" "${fl:-?}" "$seg" "$(printf %03d $i)" "$f" >> "$OUT"
done < "$LIST"
echo done >> "$G/tso-bunall-$LABEL.done"
